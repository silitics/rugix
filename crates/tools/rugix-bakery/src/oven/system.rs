use std::fs::{self, File};
use std::io::Seek;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use tracing::info;

use reportify::{bail, whatever, ResultExt};
use xscript::{run, Run};

use rugix_common::disk::gpt::gpt_types;
use rugix_common::disk::mbr::mbr_types;
use rugix_common::disk::{
    parse_size, DiskId, NumBlocks, Partition, PartitionTable, PartitionTableType,
};
use rugix_common::fsutils::allocate_file;
use rugix_common::utils::ascii_numbers;
use rugix_common::utils::units::NumBytes;
use rugix_common::{grub_patch_env, rpi_patch_boot, rpi_patch_config};

use crate::config::images::{Filesystem, ImageLayout};
use crate::config::systems::{SystemConfig, Target};
use crate::oven::targets;
use crate::oven::targets::generic_grub_efi::initialize_grub;
use crate::oven::targets::rpi_tryboot::initialize_tryboot;
use crate::oven::targets::rpi_uboot::initialize_uboot;
use crate::utils::caching::mtime;
use crate::BakeryResult;

use super::layer::FrozenLayer;

pub fn make_system(config: &SystemConfig, frozen: &FrozenLayer, out: &Path) -> BakeryResult<()> {
    let system_info = out.join("system-info.json");
    if system_info.exists() {
        let system_mtime = mtime(&system_info).whatever("unable to get system mtime")?;
        let layer_mtime = frozen.last_modified()?;
        if layer_mtime < system_mtime {
            info!("System is newer than layer.");
            return Ok(());
        } else {
            info!("Layer is newer than system (system: {system_mtime:?}, layer: {layer_mtime:?}).");
        }
    }

    std::fs::remove_dir_all(out).ok();
    std::fs::create_dir_all(out).ok();

    let layer = frozen.unfreeze()?;
    let layer_path = layer.path();

    let system_dir = layer_path.join("roots/system");
    fs::create_dir_all(&system_dir).whatever("unable to create system directory")?;

    // Create directories for config and boot partitions.
    info!("Creating config and boot directories.");
    let config_dir = layer_path.join("roots/config");
    fs::create_dir_all(&config_dir).whatever("unable to create config directory")?;
    let boot_dir = layer_path.join("roots/boot");
    fs::create_dir_all(&boot_dir).whatever("unable to create boot directory")?;

    // Initialize config partition.
    info!("Initialize boot flow.");
    if let Some(target) = &config.target {
        match target {
            Target::RpiTryboot => {
                initialize_tryboot(&config_dir)?;
            }
            Target::RpiUboot => {
                initialize_uboot(config, &config_dir)?;
            }
            Target::GenericGrubEfi => {
                initialize_grub(&config, &config_dir)?;
            }
            Target::Unknown => { /* nothing to do */ }
        }

        if !matches!(target, Target::Unknown) {
            std::fs::create_dir_all(config_dir.join(".rugix")).ok();
            std::fs::File::create(config_dir.join(".rugix/bootstrap"))
                .whatever("unable to create file `.rugix/bootstrap`")?;
        }
    }

    // At this point, everything is initialized and we can compute the partition table.
    let layout = config
        .image
        .as_ref()
        .and_then(|image| image.layout.clone())
        .or_else(|| config.target.as_ref().and_then(targets::get_default_layout))
        .ok_or_else(|| whatever!("image layout needs to be specified"))?;

    let image_file = out.join("system.img");

    info!("Computing partition table.");
    let table = compute_partition_table(&layout, &layer_path.join("roots"))?;

    let size_bytes = table.blocks_to_bytes(table.disk_size);

    info!("Allocating image file.");
    if let Some(size) = &config.image.as_ref().and_then(|image| image.size) {
        allocate_file(&image_file, size.raw)
    } else {
        allocate_file(&image_file, size_bytes.into_raw())
    }
    .whatever("error allocating image file")?;

    info!("Writing image partition table.");
    table
        .write(&image_file)
        .whatever("error writing image partition table")?;

    let table =
        PartitionTable::read(&image_file).whatever("error reading image partition table")?;

    if let Some(target) = &config.target {
        if matches!(target, Target::RpiTryboot | Target::RpiUboot) {
            let disk_id = match table.disk_id {
                DiskId::Mbr(mbr_id) => mbr_id.into_raw(),
                _ => bail!("unsupported GPT partition layout"),
            };
            info!("Patching boot configuration.");
            rpi_patch_boot(&boot_dir, format!("PARTUUID={disk_id:08X}-05"))
                .whatever("unable to patch boot configuration")?;
            info!("Patching `config.txt`.");
            rpi_patch_config(boot_dir.join("config.txt"))
                .whatever("unable to patch `config.txt`")?;
        }
        if matches!(target, Target::GenericGrubEfi) {
            let root_part = &table.partitions[3];
            let part_uuid = root_part
                .gpt_id
                .unwrap()
                .to_hex_str(ascii_numbers::Case::Lower);
            grub_patch_env(boot_dir, part_uuid)
                .whatever("unable to patch Grub boot environment")?;
        }
    }

    let filesystems_dir = out.join("filesystems");

    std::fs::create_dir_all(&filesystems_dir).ok();

    // Create filesystems.
    if let Some(partitions) = &layout.partitions {
        for (partition, (layout_partition, image_partition)) in
            partitions.iter().zip(table.partitions.iter()).enumerate()
        {
            let Some(filesystem) = &layout_partition.filesystem else {
                continue;
            };
            info!(
                "Creating {} filesystem on partition {} (size: {}).",
                filesystem.name(),
                image_partition.number,
                image_partition.size.into_raw()
            );
            let fs_image = filesystems_dir.join(format!("partition-{}.img", partition + 1));
            match filesystem {
                Filesystem::Ext4 => {
                    let size = table.blocks_to_bytes(image_partition.size);
                    allocate_file(&fs_image, size.into_raw())
                        .whatever("unable to allocate filesystem file")?;
                    if let Some(path) = &layout_partition.root {
                        run!([
                            "mkfs.ext4",
                            "-d",
                            layer_path.join("roots").join(path),
                            &fs_image
                        ])
                    } else {
                        run!(["mkfs.ext4", &fs_image])
                    }
                    .whatever("unable to create EXT4 filesystem")?;
                    let mut src =
                        File::open(&fs_image).whatever("unable to open filesystem image file")?;
                    let mut dst = File::options()
                        .write(true)
                        .open(&image_file)
                        .whatever("unable to open image file")?;
                    dst.seek(std::io::SeekFrom::Start(
                        table.blocks_to_bytes(image_partition.start).into_raw(),
                    ))
                    .whatever("unable to seek in image file")?;
                    std::io::copy(&mut src, &mut dst)
                        .whatever("error copying filesystem into image")?;
                }
                Filesystem::Fat32 => {
                    let size = table.blocks_to_bytes(image_partition.size);
                    allocate_file(&fs_image, size.into_raw())
                        .whatever("error allocating filesystem image")?;
                    run!(["mkfs.vfat", &fs_image]).whatever("error creating FAT32 filesystem")?;
                    if let Some(path) = &layout_partition.root {
                        let fs_path = layer_path.join("roots").join(path);
                        for entry in
                            fs::read_dir(&fs_path).whatever("error reading filesystem content")?
                        {
                            let entry = entry.whatever("error reading filesystem entry")?;
                            run!([
                                "/usr/bin/mcopy",
                                "-i",
                                &fs_image,
                                "-snop",
                                entry.path(),
                                "::"
                            ])
                            .whatever("error copying files into image")?;
                        }
                    }
                    let mut src =
                        File::open(&fs_image).whatever("unable to open filesystem image file")?;
                    let mut dst = File::options()
                        .write(true)
                        .open(&image_file)
                        .whatever("unable to open image file")?;
                    dst.seek(std::io::SeekFrom::Start(
                        table.blocks_to_bytes(image_partition.start).into_raw(),
                    ))
                    .whatever("unable to seek in image file")?;
                    std::io::copy(&mut src, &mut dst)
                        .whatever("error copying filesystem into image")?;
                }
            }
        }
    }

    std::fs::write(&system_info, "{}").whatever("unable to write system info")?;

    Ok(())
}

/// We are calculating everything with a portable block size of 512 bytes.
const BLOCK_SIZE: NumBytes = NumBytes::from_raw(512);

/// We align everything to 2048 blocks, i.e., 1MiB.
const ALIGNMENT: NumBlocks = NumBlocks::from_raw(2048);

/// Convert number of bytes to number of blocks.
fn bytes_to_blocks(bytes: NumBytes) -> NumBlocks {
    NumBlocks::from_raw(bytes.into_raw().div_ceil(BLOCK_SIZE.into_raw()))
}

/// Compute the partition table for an image based on the provided layout.
fn compute_partition_table(layout: &ImageLayout, roots_dir: &Path) -> BakeryResult<PartitionTable> {
    let table_type = layout
        .ty
        .map(|ty| match ty {
            crate::config::images::PartitionTableType::Mbr => PartitionTableType::Mbr,
            crate::config::images::PartitionTableType::Gpt => PartitionTableType::Gpt,
        })
        .unwrap_or(PartitionTableType::Mbr);
    let mut partitions = Vec::new();
    let mut next_usable = ALIGNMENT;
    let mut next_number = 1;
    let mut in_extended = false;
    if let Some(layout_partitions) = &layout.partitions {
        for partition in layout_partitions {
            // Partitions are numbered based on their appearance in the layout.
            let number = next_number;
            next_number += 1;
            if table_type.is_mbr() && number > 4 && !in_extended {
                bail!("invalid number of primary partitions in MBR");
            }
            // Leave space for the EBR, if we are creating a logical MBR partition.
            if in_extended {
                next_usable = (next_usable + NumBlocks::ONE).ceil_align_to(ALIGNMENT);
            }
            // By default, we create `LINUX` partitions.
            let partition_type = partition.ty.unwrap_or(match table_type {
                PartitionTableType::Mbr => mbr_types::LINUX,
                PartitionTableType::Gpt => gpt_types::LINUX,
            });
            if layout.ty.unwrap() != partition_type.table_type() {
                bail!("partition type `{partition_type}` does not match table type `{table_type}`",)
            }
            // The start of the partition is the next usable block.
            let start = next_usable;
            if partition_type.is_extended() {
                if in_extended {
                    bail!("nested extended partitions are not allowed")
                }
                partitions.push(Partition {
                    number,
                    start,
                    // We fix this later once we know the size of the extended part.
                    size: 0.into(),
                    ty: partition_type,
                    name: None,
                    gpt_id: None,
                });
                in_extended = true;
                next_number = 5;
                // Space for the EBR is automatically added prior to the next partition.
            } else {
                let size = match &partition.size {
                    Some(size) => bytes_to_blocks(size.raw.into()),
                    None => {
                        let Some(path) = &partition.root else {
                            bail!("partitions without a fixed size must have a root path");
                        };
                        compute_fs_size(roots_dir.join(path))?
                    }
                };
                partitions.push(Partition {
                    number,
                    start,
                    size,
                    ty: partition_type,
                    name: None,
                    gpt_id: None,
                });
                next_usable = (start + size).ceil_align_to(ALIGNMENT);
            }
        }
    }
    // Fix the size of the extended partition, if there is one.
    for partition in partitions.iter_mut() {
        if !partition.ty.is_extended() {
            continue;
        }
        partition.size = (next_usable - partition.start + NumBlocks::ONE).ceil_align_to(ALIGNMENT);
        break;
    }
    // Create and validate the partition table.
    let image_size = match partitions.last() {
        Some(last_partition) => {
            (last_partition.start + last_partition.size).ceil_align_to(ALIGNMENT) + ALIGNMENT
        }
        None => ALIGNMENT * 32,
    };
    let table_id = match table_type {
        PartitionTableType::Mbr => DiskId::random_mbr(),
        PartitionTableType::Gpt => DiskId::random_gpt(),
    };
    let mut table = PartitionTable::new(table_id, image_size);
    table.partitions = partitions;
    table
        .validate()
        .whatever("unable to validate image partitions")?;
    Ok(table)
}

/// Compute the required size for a filesystem based on the given root path.
fn compute_fs_size(root: PathBuf) -> BakeryResult<NumBlocks> {
    let mut size = NumBytes::from_raw(0);
    let mut stack = vec![root];
    while let Some(top) = stack.pop() {
        // We do not want to follow symlinks here as we are interested in the size of
        // the symlink and not the size of the symlink's target.
        let metadata = fs::symlink_metadata(&top).whatever("unable to get file metadata")?;
        size += NumBytes::from_raw(metadata.size());
        if metadata.is_dir() {
            for entry in fs::read_dir(&top).whatever("unable to read directory")? {
                stack.push(entry.whatever("unable to read directory entry")?.path());
            }
        }
    }
    size = size.max(parse_size("64M").unwrap());
    // Add an overhead of 20% for filesystem metadata.
    size += NumBytes::from_raw(size.into_raw().div_ceil(5));
    Ok(bytes_to_blocks(size))
}
