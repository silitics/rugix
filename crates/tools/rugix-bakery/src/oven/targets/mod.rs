use byte_calc::NumBytes;

use rugix_common::disk::gpt::gpt_types;
use rugix_common::disk::mbr::mbr_types;

use crate::config::images::{
    Ext4Options, Filesystem, ImageLayout, ImagePartition, PartitionTableType, SquashfsOptions,
};
use crate::config::systems::Target;

pub mod generic_grub_efi;
pub mod rpi_tryboot;
pub mod rpi_uboot;

/// Get the default image layout for the provided target.
pub fn get_default_layout(
    target: &Target,
    squashfs_options: Option<&SquashfsOptions>,
) -> Option<ImageLayout> {
    match target {
        Target::GenericGrubEfi => Some(default_gpt_layout(squashfs_options)),
        Target::RpiTryboot => Some(default_mbr_layout(squashfs_options)),
        Target::RpiUboot => Some(default_mbr_layout(squashfs_options)),
        Target::Unknown => None,
    }
}

fn default_mbr_layout(squashfs_options: Option<&SquashfsOptions>) -> ImageLayout {
    ImageLayout::new()
        .with_ty(Some(PartitionTableType::Mbr))
        .with_partitions(Some(vec![
            // Config partition.
            ImagePartition::new()
                .with_size(Some(NumBytes::mebibytes(256)))
                .with_ty(Some(mbr_types::FAT32_LBA))
                .with_filesystem(Some(Filesystem::Fat32))
                .with_root(Some("config".to_owned())),
            // `A` boot partition.
            ImagePartition::new()
                .with_size(Some(NumBytes::mebibytes(128)))
                .with_ty(Some(mbr_types::FAT32_LBA))
                .with_filesystem(Some(Filesystem::Fat32))
                .with_root(Some("boot".to_owned())),
            // `B` boot partition.
            ImagePartition::new()
                .with_size(Some(NumBytes::mebibytes(128)))
                .with_ty(Some(mbr_types::FAT32_LBA)),
            // MBR extended partition.
            ImagePartition::new().with_ty(Some(mbr_types::EXTENDED)),
            // `A` system partition.
            ImagePartition::new()
                .with_ty(Some(mbr_types::LINUX))
                .with_filesystem(Some(
                    squashfs_options
                        .cloned()
                        .map(Filesystem::Squashfs)
                        .unwrap_or(Filesystem::Ext4(
                            Ext4Options::new().with_additional_options(Some(
                                [
                                    "-O",
                                    "^has_journal",
                                    "-E",
                                    "hash_seed=035cb65d-0a86-404a-bad7-19c88d05e400",
                                    "-U",
                                    "12341234-a4ec-4304-a70f-c549ea829da9",
                                ]
                                .map(str::to_owned)
                                .into(),
                            )),
                        )),
                ))
                .with_root(Some("system".to_owned())),
        ]))
}

fn default_gpt_layout(squashfs_options: Option<&SquashfsOptions>) -> ImageLayout {
    ImageLayout::new()
        .with_ty(Some(PartitionTableType::Gpt))
        .with_partitions(Some(vec![
            // Config partition.
            ImagePartition::new()
                .with_size(Some(NumBytes::mebibytes(256)))
                .with_ty(Some(gpt_types::EFI))
                .with_filesystem(Some(Filesystem::Fat32))
                .with_root(Some("config".to_owned())),
            // `A` boot partition.
            ImagePartition::new()
                .with_size(Some(NumBytes::mebibytes(256)))
                .with_ty(Some(gpt_types::LINUX))
                .with_filesystem(Some(Filesystem::Ext4(Ext4Options::new())))
                .with_root(Some("boot".to_owned())),
            // `B` boot partition.
            ImagePartition::new()
                .with_size(Some(NumBytes::mebibytes(256)))
                .with_ty(Some(gpt_types::LINUX)),
            // `A` system partition.
            ImagePartition::new()
                .with_ty(Some(gpt_types::LINUX))
                .with_filesystem(Some(
                    squashfs_options
                        .cloned()
                        .map(Filesystem::Squashfs)
                        .unwrap_or(Filesystem::Ext4(
                            Ext4Options::new().with_additional_options(Some(
                                [
                                    "-O",
                                    "^has_journal",
                                    "-E",
                                    "hash_seed=035cb65d-0a86-404a-bad7-19c88d05e400",
                                    "-U",
                                    "12341234-a4ec-4304-a70f-c549ea829da9",
                                ]
                                .map(str::to_owned)
                                .into(),
                            )),
                        )),
                ))
                .with_root(Some("system".to_owned())),
        ]))
}
