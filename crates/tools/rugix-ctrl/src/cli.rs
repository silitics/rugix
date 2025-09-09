//! Definition of the command line interface (CLI).

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Child;

use rugix_bundle::format::decode::decode_slice;
use rugix_bundle::manifest::ChunkerAlgorithm;
use rugix_bundle::reader::block_provider::StoredBlockProvider;
use rugix_bundle::reader::{DecodedPayloadInfo, PayloadTarget};
use rugix_bundle::source::{BundleSource, ReaderSource, SkipRead};
use rugix_bundle::xdelta::xdelta_decompress;
use rugix_bundle::{format, BUNDLE_MAGIC};
use rugix_common::pipe::{buffered_pipe, PipeWriter};
use rugix_common::slots::SlotState;
use rugix_hooks::{HooksLoader, RunOptions};
use si_crypto_hashes::{HashAlgorithm, HashDigest, Hasher};
use tracing::{debug, error, info, trace, warn};

use crate::system::boot_groups::{BootGroup, BootGroupIdx};
use crate::system::slots::SlotKind;
use crate::system::{System, SystemResult};
use clap::{Parser, ValueEnum};
use reportify::{bail, whatever, ErrorExt, ResultExt};
use rugix_common::disk::stream::ImgStream;
use rugix_common::maybe_compressed::{MaybeCompressed, PeekReader};
use rugix_common::stream_hasher::StreamHasher;
use xscript::{cmd_os, vars, ParentEnv, Run, Vars};

use crate::http_source::HttpSource;
use crate::overlay::overlay_dir;
use crate::slot_db::{self, BlockProvider};
use crate::system_state;
use crate::utils::{clear_flag, reboot, set_flag, DEFERRED_SPARE_REBOOT_FLAG};

fn create_rugix_state_directory() -> SystemResult<()> {
    fs::create_dir_all("/run/rugix/state/.rugix")
        .whatever("unable to create `/run/rugix/state/.rugix`")
}

fn set_rugix_state_flag(name: &str, value: Option<&str>) -> SystemResult<()> {
    fs::write(
        Path::new("/run/rugix/state/.rugix").join(name),
        value.unwrap_or_default(),
    )
    .whatever("unable to write state flag")
    .with_info(|_| format!("name: {name}"))
}

fn clear_rugix_state_flag(name: &str) -> SystemResult<()> {
    let path = Path::new("/run/rugix/state/.rugix").join(name);
    fs::remove_file(&path).or_else(|error| match error.kind() {
        io::ErrorKind::NotFound => Ok(()),
        _ => Err(error
            .whatever("unable to clear state flag")
            .with_info(format!("name: {name}"))),
    })?;
    if path.exists() {
        return Err(whatever!("unable to clear state flag").with_info(format!("name: {name}")));
    }
    Ok(())
}

pub fn main() -> SystemResult<()> {
    rugix_cli::CliBuilder::new().init();

    let args = Args::parse();
    let system = System::initialize()?;
    match &args.command {
        Command::State(state_cmd) => match state_cmd {
            StateCommand::Reset {
                backup,
                backup_name,
            } => {
                if backup_name.is_some() && !*backup {
                    warn!("ignoring `--backup-name` option because `--backup` is not set");
                }

                let reset_hooks = HooksLoader::default()
                    .load_hooks("state-reset")
                    .whatever("unable to load `state-reset` hooks")?;
                reset_hooks
                    .run_hooks("prepare", Vars::new(), &Default::default())
                    .whatever("unable to run `state-reset/prepare` hooks")?;
                create_rugix_state_directory()?;
                if *backup {
                    let backup_name = backup_name.clone().unwrap_or_else(|| {
                        jiff::Timestamp::now()
                            .strftime("default.%Y%m%d%H%M%S")
                            .to_string()
                    });
                    set_rugix_state_flag("reset-state", Some(&backup_name))?;
                } else {
                    set_rugix_state_flag("reset-state", None)?;
                };
                reboot()?;
            }
            StateCommand::Overlay(overlay_cmd) => match overlay_cmd {
                OverlayCommand::ForcePersist { persist } => match persist {
                    Boolean::True => {
                        create_rugix_state_directory()?;
                        set_rugix_state_flag("force-persist-overlay", None)?;
                    }
                    Boolean::False => {
                        clear_rugix_state_flag("force-persist-overlay")?;
                    }
                },
            },
        },
        Command::Update(update_cmd) => {
            match update_cmd {
                UpdateCommand::Install {
                    bundle: image,
                    reboot: reboot_type,
                    keep_overlay,
                    check_hash,
                    verify_bundle,
                    boot_group,
                    verify_signature,
                    root_cert,
                } => {
                    let check_hash = check_hash.as_deref()
                            .map(|encoded_hash| -> SystemResult<ImageHash> {
                                let (algorithm, hash) = encoded_hash
                                    .split_once(':')
                                    .ok_or_else(||
                                        whatever!("Invalid format of hash. Format must be `sha256:<HEX-ENCODED-HASH>`.")
                                    )?;
                                if algorithm != "sha256" {
                                    bail!("Algorithm must be SHA256.");
                                }
                                let decoded_hash = hex::decode(hash).whatever("unable to decode image hash")?;
                                Ok(ImageHash::Sha256(decoded_hash))
                        }).transpose()?;

                    if system.needs_commit()? {
                        bail!("System needs to be committed before installing an update.");
                    }

                    // Find the entry where we are going to install the update to.
                    let boot_group = match boot_group {
                        Some(entry_name) => {
                            let Some(entry) = system.boot_entries().find_by_name(entry_name) else {
                                bail!("unable to find entry {entry_name}")
                            };
                            Some(entry)
                        }
                        None => {
                            if system.boot_entries().iter().count() > 2 {
                                None
                            } else {
                                system
                                    .boot_entries()
                                    .iter()
                                    .find(|(_, entry)| !entry.active())
                            }
                        }
                    };
                    if let Some((_, boot_group)) = boot_group {
                        info!("installing update to boot group {:?}", boot_group.name());
                        if boot_group.active() {
                            bail!("selected entry {} is active", boot_group.name());
                        }
                    }

                    let hooks = HooksLoader::default()
                        .load_hooks("update-install")
                        .whatever("unable to load `update-install` hooks")?;

                    let hook_vars = vars! {
                        RUGIX_BOOT_GROUP = boot_group.map(|g| g.1.name()).unwrap_or(""),
                    };

                    hooks
                        .run_hooks("pre-update", hook_vars.clone(), &Default::default())
                        .whatever("error running `pre-update` hooks")?;

                    if !keep_overlay {
                        if let Some(boot_group) = &boot_group {
                            let spare_overlay_dir = overlay_dir(boot_group.1);
                            fs::remove_dir_all(spare_overlay_dir).ok();
                        }
                    }

                    let should_reboot = install_update_stream(
                        &system,
                        image,
                        check_hash,
                        verify_bundle,
                        boot_group.as_ref(),
                        *verify_signature,
                        root_cert,
                    )?;

                    hooks
                        .run_hooks("post-update", hook_vars.clone(), &Default::default())
                        .whatever("error running `post-update` hooks")?;

                    let reboot_type = reboot_type.clone().unwrap_or(should_reboot);

                    match reboot_type {
                        UpdateRebootType::Yes => {
                            let (entry_idx, boot_group) = boot_group.unwrap();
                            info!(
                                "instructing boot flow to try booting into {:?}",
                                boot_group.name()
                            );
                            system
                                .boot_flow()
                                .set_try_next(&system, entry_idx)
                                .whatever("unable to set next boot group")?;
                            reboot()?;
                        }
                        UpdateRebootType::No => { /* nothing to do */ }
                        UpdateRebootType::Deferred => {
                            set_flag(DEFERRED_SPARE_REBOOT_FLAG)?;
                        }
                    }
                }
            }
        }
        Command::System(sys_cmd) => match sys_cmd {
            SystemCommand::Info { json } => {
                let output = system_state::state_from_system(&system);
                if let Some(boot) = &output.boot {
                    eprintln!("Boot Flow: {}", boot.boot_flow);
                    eprintln!(
                        "Active Boot Group: {}",
                        boot.active_group.as_deref().unwrap_or("<unknown>")
                    );
                    eprintln!(
                        "Default Boot Group: {}",
                        boot.default_group.as_deref().unwrap_or("<unknown>")
                    );
                }
                for (name, info) in &output.slots {
                    eprintln!(
                        "Slot {name:?}: {}",
                        if let Some(active) = info.active {
                            if active {
                                "active"
                            } else {
                                "inactive"
                            }
                        } else {
                            "<unknown>"
                        }
                    );
                }
                if rugix_cli::stdout_is_piped() || *json {
                    serde_json::to_writer(std::io::stdout(), &output)
                        .whatever("unable to write system info to stdout")?;
                }
            }
            SystemCommand::Commit => {
                if system.needs_commit()? {
                    let hooks = HooksLoader::default()
                        .load_hooks("system-commit")
                        .whatever("unable to load `system-commit` hooks")?;
                    hooks
                        .run_hooks("pre-commit", Vars::new(), &Default::default())
                        .whatever("unable to run `pre-commit` hooks")?;
                    system.commit()?;
                    hooks
                        .run_hooks("post-commit", Vars::new(), &Default::default())
                        .whatever("unable to run `post-commit` hooks")?;
                } else {
                    println!("Active boot group is already the default!");
                }
            }
            SystemCommand::Reboot { spare } => {
                if *spare {
                    if let Some((spare, _)) = system.spare_entry()? {
                        system
                            .boot_flow()
                            .set_try_next(&system, spare)
                            .whatever("unable to set next boot group")?;
                    }
                }
                reboot()?;
            }
        },
        Command::Unstable(command) => match command {
            UnstableCommand::SetDeferredSpareReboot { value } => match value {
                Boolean::True => set_flag(DEFERRED_SPARE_REBOOT_FLAG)?,
                Boolean::False => clear_flag(DEFERRED_SPARE_REBOOT_FLAG)?,
            },
            UnstableCommand::PrintSystemInfo => {
                println!("Config:");
                println!("{:#?}", system.config());
                println!("Root:");
                println!("{:#?}", system.root());
                println!("Slots:");
                for (_, slot) in system.slots().iter() {
                    println!("{:#?}", slot)
                }
                println!("Boot Entries");
                println!("{:#?}", system.boot_entries());
            }
        },
        Command::Slots(slots_command) => match slots_command {
            SlotsCommand::Inspect { slot } => {
                let indices = slot_db::get_stored_indices(slot)?;
                if indices.is_empty() {
                    eprintln!("No indices for slot {slot}")
                } else {
                    for index in &indices {
                        eprintln!("Found index {:?}", &index.index_file);
                    }
                }
            }
            SlotsCommand::CreateIndex {
                slot,
                chunker: chunker_algorithm,
                hash_algorithm,
            } => {
                let Some((_, slot)) = system.slots().find_by_name(slot) else {
                    bail!("slot {slot} not found")
                };
                match slot.kind() {
                    SlotKind::Block(block_slot) => {
                        slot_db::add_index(
                            slot.name(),
                            block_slot.device().path(),
                            chunker_algorithm,
                            hash_algorithm,
                        )?;
                    }
                    SlotKind::File { path } => {
                        slot_db::add_index(slot.name(), path, chunker_algorithm, hash_algorithm)?;
                    }
                    SlotKind::Custom { .. } => {
                        bail!("cannot create indices on custom slots");
                    }
                }
            }
            SlotsCommand::Verify { slot } => {
                let Some((_, slot)) = system.slots().find_by_name(slot) else {
                    bail!("slot {slot} not found")
                };
                let Some(slot_state) = slot_db::get_stored_state(slot.name())? else {
                    bail!("no stored state for slot {}", slot.name());
                };
                if !slot.is_immutable() {
                    bail!("slot {} is not immutable, cannot verify", slot.name());
                }
                let Some((_, hash)) = &slot_state.hashes.iter().next() else {
                    bail!("no hashes stored for slot {}", slot.name());
                };
                let mut hasher = hash.algorithm().hasher();
                let mut file = match slot.kind() {
                    SlotKind::Block(block_slot) => {
                        File::open(block_slot.device()).whatever("error opening block device")?
                    }
                    SlotKind::File { path } => File::open(path).whatever("error opening file")?,
                    SlotKind::Custom { .. } => {
                        bail!("cannot create indices on custom slots");
                    }
                };
                info!(expected_hash = %hash, slot_name = slot.name(), size = slot_state.size.map(|s| s.raw), "verifying slot");
                let mut buffer = [0; 4096];
                let mut remaining = slot_state.size.map(|s| s.raw).unwrap_or(u64::MAX);
                let mut bytes_hashed = 0;
                while remaining > 0 {
                    let read = file.read(&mut buffer).whatever("error reading slot file")?;
                    if read == 0 {
                        break;
                    }
                    let chunk = &buffer[..(read as u64).min(remaining) as usize];
                    hasher.update(chunk);
                    bytes_hashed += chunk.len() as u64;
                    remaining = remaining.saturating_sub(read as u64);
                }
                debug!("hashed {} bytes from slot {}", bytes_hashed, slot.name());
                let found = hasher.finalize();
                if found != **hash {
                    bail!(
                        "hash mismatch for slot {}: expected {}, found {}",
                        slot.name(),
                        hash,
                        found
                    );
                }
                info!(slot_name = slot.name(), "slot verified successfully");
            }
        },
        Command::Boot(cmd) => match cmd {
            BootCommand::MarkGood { group } => {
                let boot_group = match group {
                    Some(entry_name) => {
                        let Some((group, _)) = system.boot_entries().find_by_name(entry_name)
                        else {
                            bail!("unable to find entry {entry_name}")
                        };
                        group
                    }
                    None => system.active_boot_entry().unwrap(),
                };
                info!(
                    "marking boot group {} as good",
                    system.boot_entries()[boot_group].name()
                );
                system
                    .boot_flow()
                    .mark_good(&system, boot_group)
                    .whatever("unable to mark boot group as good")?;
            }
            BootCommand::MarkBad { group } => {
                let Some((group, _)) = system.boot_entries().find_by_name(group) else {
                    bail!("unable to find entry {group}")
                };
                info!(
                    "marking boot group {} as good",
                    system.boot_entries()[group].name()
                );
                system
                    .boot_flow()
                    .mark_bad(&system, group)
                    .whatever("unable to mark boot group as bad")?;
            }
        },
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub enum ImageHash {
    Sha256(Vec<u8>),
}

pub enum MaybeStreamHasher<R> {
    NoHash {
        reader: R,
    },
    Sha256 {
        hasher: StreamHasher<R, sha2::Sha256>,
        expected: Vec<u8>,
    },
}

impl<R> MaybeStreamHasher<R> {
    pub fn verify(self) -> SystemResult<()> {
        match self {
            MaybeStreamHasher::NoHash { .. } => Ok(()),
            MaybeStreamHasher::Sha256 { hasher, expected } => {
                let found = hasher.finalize();
                if expected.as_slice() != found.as_slice() {
                    return Err(whatever(indoc::formatdoc! {
                        r#"
                            **Image Hash Mismatch:**
                            Expected: {}
                            Found: {}
                        "#,
                        hex::encode(expected),
                        hex::encode(found)
                    }));
                }
                Ok(())
            }
        }
    }
}

impl<R: Read> Read for MaybeStreamHasher<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            MaybeStreamHasher::NoHash { reader } => reader.read(buf),
            MaybeStreamHasher::Sha256 { hasher, .. } => hasher.read(buf),
        }
    }
}

fn install_update_stream(
    system: &System,
    image: &String,
    check_hash: Option<ImageHash>,
    verify_bundle: &Option<HashDigest>,
    boot_group: Option<&(BootGroupIdx, &BootGroup)>,
    verify_signature: bool,
    root_cert: &[PathBuf],
) -> SystemResult<UpdateRebootType> {
    if image.starts_with("http") {
        if check_hash.is_some() {
            bail!("--check-hash is not supported for update bundles, use --verify-bundle");
        }
        let mut bundle_source = HttpSource::new(image)?;
        let should_reboot = install_update_bundle(
            system,
            &mut bundle_source,
            verify_bundle,
            boot_group,
            verify_signature,
            root_cert,
        )?;
        let stats = bundle_source.get_download_stats();
        info!(
            "downloaded {:.1}% ({}/{}) of the full bundle",
            stats.download_ratio() * 100.0,
            stats.bytes_read,
            stats.total_bytes(),
        );
        return Ok(should_reboot);
    }
    let reader: &mut dyn io::Read = if image == "-" {
        &mut io::stdin()
    } else {
        &mut File::open(image).whatever("error opening image")?
    };
    let reader = match check_hash.clone() {
        Some(ImageHash::Sha256(expected)) => MaybeStreamHasher::Sha256 {
            hasher: StreamHasher::new(reader),
            expected,
        },
        None => MaybeStreamHasher::NoHash { reader },
    };
    let mut update_stream = PeekReader::new(reader);

    let magic = update_stream
        .peek(BUNDLE_MAGIC.len())
        .whatever("error reading bundle magic")?;

    if magic == BUNDLE_MAGIC {
        if check_hash.is_some() {
            bail!("--check-hash is not supported for update bundles, use --verify-bundle");
        }
        let bundle_source = ReaderSource::<_, SkipRead>::from_unbuffered(update_stream);
        return install_update_bundle(
            system,
            bundle_source,
            verify_bundle,
            boot_group,
            verify_signature,
            root_cert,
        );
    }
    if verify_bundle.is_some() {
        bail!("--verify-bundle is not supported on images, use --check-hash");
    }
    if verify_signature {
        bail!("--verify-signature is not supported on images");
    }

    let Some((entry_idx, entry)) = boot_group else {
        bail!("for image updates, you need to specify a boot group");
    };

    let update_stream =
        MaybeCompressed::new(update_stream).whatever("error decompressing stream")?;

    system
        .boot_flow()
        .pre_install(system, *entry_idx)
        .whatever("error executing pre-install step")?;

    let boot_slot = entry.get_slot("boot").unwrap();
    let system_slot = entry.get_slot("system").unwrap();

    let boot_slot = &system.slots()[boot_slot];
    let system_slot = &system.slots()[system_slot];

    slot_db::erase(boot_slot.name())?;
    slot_db::erase(system_slot.name())?;

    let SlotKind::Block(raw_boot_slot) = boot_slot.kind() else {
        bail!("boot slot must be a block device");
    };
    let SlotKind::Block(raw_system_slot) = system_slot.kind() else {
        bail!("system slot must be a block device");
    };

    let mut img_stream =
        ImgStream::new(update_stream).whatever("error reading image partitions")?;
    let mut partition_idx = 0;
    while let Some(mut partition) = img_stream
        .next_partition()
        .whatever("error reading next partition")?
    {
        let partition_name = match partition_idx {
            0 => "CONFIG",
            1 => "BOOT-A",
            2 => "BOOT-B",
            3 => "system-a",
            4 => "system-b",
            5 => "data",
            _ => "<unknown>",
        };
        println!(
            "Found {partition_idx},{partition_name} {}",
            partition.entry()
        );
        match partition_idx {
            1 => {
                io::copy(
                    &mut partition,
                    &mut fs::File::create(raw_boot_slot.device())
                        .whatever("error opening boot partition file")?,
                )
                .whatever("error copying boot partition")?;
            }
            3 => {
                io::copy(
                    &mut partition,
                    &mut fs::File::create(raw_system_slot.device())
                        .whatever("error opening system partition file")?,
                )
                .whatever("error copying system partition")?;
            }
            _ => { /* Nothing to do! */ }
        }

        partition_idx += 1;
    }

    let mut hashed_stream = img_stream.into_inner().into_inner().into_inner();
    // Make sure that the entire stream has been consumed. Otherwise, the hash
    // may not be match the file.
    loop {
        let mut buffer = vec![0; 4096];
        if hashed_stream
            .read_to_end(&mut buffer)
            .whatever("error reading image")?
            == 0
        {
            break;
        }
    }

    if let Err(error) = hashed_stream.verify() {
        error!("hash verification failed");
        if let Err(error) =
            rugix_fs::File::open_write(raw_boot_slot.device().path()).and_then(|mut device| {
                device.write_zeros(
                    byte_calc::NumBytes::new(0),
                    byte_calc::NumBytes::mebibytes(1),
                )
            })
        {
            error!("error overwriting boot partition: {error:?}");
        }
        if let Err(error) =
            rugix_fs::File::open_write(raw_system_slot.device().path()).and_then(|mut device| {
                device.write_zeros(
                    byte_calc::NumBytes::new(0),
                    byte_calc::NumBytes::mebibytes(1),
                )
            })
        {
            error!("error overwriting system partition: {error:?}");
        }
        return Err(error);
    }

    system
        .boot_flow()
        .post_install(system, *entry_idx)
        .whatever("error running post-install step")?;
    Ok(UpdateRebootType::Yes)
}

fn install_update_bundle<R: BundleSource>(
    system: &System,
    bundle_source: R,
    verify_bundle: &Option<HashDigest>,
    boot_group: Option<&(BootGroupIdx, &BootGroup)>,
    verify_signature: bool,
    root_certs: &[PathBuf],
) -> SystemResult<UpdateRebootType> {
    let mut bundle_reader =
        rugix_bundle::reader::BundleReader::start(bundle_source, verify_bundle.clone())
            .whatever("unable to read bundle")?;

    if verify_signature {
        let Some(signatures) = bundle_reader.signatures() else {
            bail!("no signatures found in bundle");
        };
        if root_certs.is_empty() {
            bail!("no root certificates provided for signature verification");
        }
        if root_certs.len() > 1 {
            bail!("multiple root certificates are not yet supported");
        }
        let mut found_valid_signature = false;
        info!("checking bundle signatures");
        for signature in signatures.cms_signatures.iter() {
            let tempdir = tempfile::tempdir().whatever("unable to create temporary directory")?;
            let tempdir_path = tempdir.path();
            let signed_metadata_raw = tempdir_path.join("signed-metadata.raw");
            let signed_metadata_cms = tempdir_path.join("signed-metadata.cms");
            std::fs::write(&signed_metadata_cms, &signature.raw)
                .whatever("unable to write CMS signature")?;
            let mut cmd = cmd_os!(
                "openssl",
                "cms",
                "-verify",
                "-in",
                &signed_metadata_cms,
                "-inform",
                "DER",
                "-out",
                &signed_metadata_raw,
                // Do not load OS default certificates.
                "-no-CAfile",
                "-no-CApath",
                "-no-CAstore",
                // Non-zero exit code on verification failure.
                "-verify_retcode",
            );
            for cert in root_certs {
                if cert.is_dir() {
                    cmd.add_arg("-CApath");
                    cmd.add_arg(cert);
                } else {
                    cmd.add_arg("-CAfile");
                    cmd.add_arg(cert);
                }
            }
            if let Err(error) = ParentEnv.run(cmd) {
                println!("{error}");
                continue;
            }
            let signed_metadata =
                std::fs::read(&signed_metadata_raw).whatever("unable to read signed metadata")?;
            let signed_metadata = decode_slice::<format::SignedMetadata>(&signed_metadata)
                .whatever("unable to decode signed metadata")?;
            if signed_metadata.header_hash
                == bundle_reader.header_hash(signed_metadata.header_hash.algorithm())
            {
                found_valid_signature = true;
                info!("found valid signature");
                break;
            }
        }
        if !found_valid_signature {
            bail!("no valid signature found");
        }
    }

    if !bundle_reader.header().is_incremental {
        let Some((entry_idx, _)) = boot_group else {
            bail!("full system updates require teh specification of a boot group");
        };
        system
            .boot_flow()
            .pre_install(system, *entry_idx)
            .whatever("error executing pre-install step")?;
    }

    let mut progress = {
        let hooks = HooksLoader::default()
            .load_hooks("update-install")
            .whatever("unable to load `update-install` hooks")?;

        let mut last_progress = 0.0;
        move |source: &R| {
            let Some(bytes_total) = source.bytes_total() else {
                return;
            };
            let Some(bytes_read) = source.bytes_read() else {
                return;
            };
            let current_progress = (bytes_read.raw as f64) / (bytes_total.raw as f64) * 100.0;
            if current_progress - last_progress > 0.9 {
                let hook_vars = vars! {
                    RUGIX_UPDATE_PROGRESS = format!("{current_progress:.2}")
                };
                if let Err(error) = hooks.run_hooks(
                    "progress",
                    hook_vars.clone(),
                    &RunOptions::default().with_silent(true),
                ) {
                    warn!("error running 'update-install/progress' hooks: {error:?}");
                }
                last_progress = current_progress;
            }
        }
    };

    while let Some(payload) = bundle_reader
        .next_payload()
        .whatever("unable to read payload")?
    {
        let payload_entry = payload.entry();
        if let Some(slot_type) = &payload_entry.type_slot {
            let slot = boot_group
                .and_then(|(_, entry)| entry.get_slot(&slot_type.slot))
                .or_else(|| system.slots().find_by_name(&slot_type.slot).map(|e| e.0));
            if let Some(slot) = slot {
                let slot = &system.slots()[slot];
                eprintln!(
                    "Installing bundle payload {} to slot {}",
                    payload.idx(),
                    slot.name()
                );
                slot_db::erase(slot.name())?;
                let mut block_provider = None;
                if let Some(block_encoding) = &payload.header().block_encoding {
                    let mut provider = BlockProvider::new(
                        block_encoding.chunker.clone(),
                        block_encoding.hash_algorithm,
                    );
                    for (_, slot) in system.slots().iter() {
                        // Since we erased all the indices of the target slot, it
                        // is fine to also add the target slot here.
                        match slot.kind() {
                            SlotKind::Block(block_slot) => {
                                provider.add_slot(
                                    slot.name(),
                                    block_slot.device().path().to_path_buf(),
                                )?;
                            }
                            SlotKind::File { path } => {
                                provider.add_slot(slot.name(), path.to_path_buf())?;
                            }
                            SlotKind::Custom { .. } => { /* nothing to do */ }
                        }
                    }
                    block_provider = Some(provider);
                }
                let decoded_payload_info = if let Some(delta_encoding) =
                    &payload_entry.delta_encoding
                {
                    let delta_encoding = delta_encoding.clone();
                    if delta_encoding.inputs.len() != 1 {
                        bail!("unsupported number of delta encoding inputs");
                    }
                    let input = &delta_encoding.inputs[0];
                    let mut source = None;
                    'slots: for (_, delta_slot) in system.slots().iter() {
                        let Ok(Some(slot_state)) = slot_db::get_stored_state(delta_slot.name())
                        else {
                            continue;
                        };
                        for input_hash in &input.hashes {
                            let Some(slot_hash) = slot_state.hashes.get(&input_hash.algorithm())
                            else {
                                trace!(slot_name = delta_slot.name(), algorithm = ?input_hash.algorithm(), "no hash found");
                                continue;
                            };
                            if slot_hash == input_hash {
                                // We found the slot to use as a source.
                                source = Some(delta_slot);
                                trace!(slot_name = delta_slot.name(), "delta source found");
                                break 'slots;
                            } else {
                                trace!(slot_name = delta_slot.name(), %slot_hash, %input_hash, "hash does not match");
                            }
                        }
                    }
                    let Some(source) = source else {
                        bail!("no slot suitable delta source found");
                    };
                    // This is here so that we get an error when introducing additional formats.
                    match delta_encoding.format {
                        rugix_bundle::manifest::DeltaEncodingFormat::Xdelta => { /* do nothing */ }
                    }
                    let source = match source.kind() {
                        SlotKind::Block(block_slot) => block_slot.device().path().to_owned(),
                        SlotKind::File { path } => path.to_owned(),
                        SlotKind::Custom { .. } => {
                            bail!("source slot must not be a custom slot");
                        }
                    };
                    let target = match slot.kind() {
                        SlotKind::Block(block_slot) => std::fs::OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open(block_slot.device())
                            .whatever("unable to open payload target")?,
                        SlotKind::File { path } => std::fs::OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(path)
                            .whatever("unable to open payload target")?,
                        SlotKind::Custom { .. } => {
                            bail!("custom slots do not support delta updates yet")
                        }
                    };
                    let mut target_writer =
                        HashWriter::new(delta_encoding.original_hash.algorithm(), target);
                    let (mut patch_reader, patch_writer) = buffered_pipe(8192);

                    let (decode_result, xdelta_result) = std::thread::scope(|scope| {
                        let target_writer = &mut target_writer;
                        // We must move the `patch_reader` here as we need it to be dropped when
                        // the decompression fails. Otherwise, we get a deadlock when waiting for
                        // the payload decoding in the following.
                        let handle = scope.spawn(move || {
                            trace!("starting xdelta");
                            let result =
                                xdelta_decompress(&source, &mut patch_reader, target_writer);
                            trace!(?result, "xdelta terminated");
                            result
                        });
                        let decode_result = payload.decode_into(
                            BufferedPipeTarget {
                                writer: patch_writer,
                            },
                            block_provider
                                .as_ref()
                                .map(|p| p as &dyn StoredBlockProvider),
                            &mut progress,
                        );
                        trace!("finished decoding payload into pipe");
                        (decode_result, handle.join().unwrap())
                    });
                    decode_result.whatever("unable to decode payload")?;
                    xdelta_result.whatever("unable to decode delta update")?;
                    let (target_hash, target_size) = target_writer.finalize();
                    if target_hash != delta_encoding.original_hash {
                        bail!("decoded slot data does not match hash");
                    }
                    DecodedPayloadInfo {
                        hash: target_hash,
                        size: target_size.into(),
                    }
                } else {
                    match slot.kind() {
                        SlotKind::Block(block_slot) => {
                            let target = std::fs::OpenOptions::new()
                                .read(true)
                                .write(true)
                                .open(block_slot.device())
                                .whatever("unable to open payload target")?;
                            payload
                                .decode_into(
                                    target,
                                    block_provider
                                        .as_ref()
                                        .map(|p| p as &dyn StoredBlockProvider),
                                    &mut progress,
                                )
                                .whatever("unable to decode payload")?
                        }
                        SlotKind::File { path } => {
                            let target = std::fs::OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(path)
                                .whatever("unable to open payload target")?;
                            payload
                                .decode_into(
                                    target,
                                    block_provider
                                        .as_ref()
                                        .map(|p| p as &dyn StoredBlockProvider),
                                    &mut progress,
                                )
                                .whatever("unable to decode payload")?
                        }
                        SlotKind::Custom { handler } => {
                            let target = CustomTarget::new(handler.iter().map(|arg| arg.as_str()))?;
                            payload
                                .decode_into(
                                    target,
                                    block_provider
                                        .as_ref()
                                        .map(|p| p as &dyn StoredBlockProvider),
                                    &mut progress,
                                )
                                .whatever("unable to decode payload")?
                        }
                    }
                };
                if let Err(error) = slot_db::save_slot_state(
                    slot.name(),
                    // Only save the hashes and size if the slot is immutable.
                    &SlotState {
                        hashes: if slot.is_immutable() {
                            [(
                                decoded_payload_info.hash.algorithm(),
                                decoded_payload_info.hash,
                            )]
                            .into_iter()
                            .collect()
                        } else {
                            Default::default()
                        },
                        size: if slot.is_immutable() {
                            Some(decoded_payload_info.size)
                        } else {
                            None
                        },
                        updated_at: Some(jiff::Timestamp::now()),
                    },
                ) {
                    error!("unable to save slot state: {error:?}");
                }
                continue;
            } else {
                error!(
                    "slot {:?} for bundle payload {} not found",
                    slot_type.slot,
                    payload.idx()
                );
            }
        } else if let Some(type_execute) = &payload_entry.type_execute {
            eprintln!("executing update payload {}", payload.idx(),);
            let target = CustomTarget::new(type_execute.handler.iter().map(|arg| arg.as_str()))?;
            payload
                .decode_into(target, None, &mut progress)
                .whatever("unable to decode payload")?;
            continue;
        }
        payload.skip().whatever("unable to skip payload")?;
    }

    if !bundle_reader.header().is_incremental {
        system
            .boot_flow()
            .post_install(system, boot_group.unwrap().0)
            .whatever("error executing post-install step")?;
        Ok(UpdateRebootType::Yes)
    } else {
        Ok(UpdateRebootType::No)
    }
}

#[derive(Debug)]
pub struct HashWriter<W> {
    writer: W,
    hasher: Hasher,
    size: u64,
}

impl<W> HashWriter<W> {
    pub fn new(algorithm: HashAlgorithm, writer: W) -> Self {
        Self {
            writer,
            hasher: algorithm.hasher(),
            size: 0,
        }
    }

    pub fn finalize(self) -> (HashDigest, u64) {
        (self.hasher.finalize(), self.size)
    }
}

impl<W: Write> Write for HashWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = self.writer.write(buf)?;
        self.hasher.update(&buf[..written]);
        self.size += buf.len() as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

#[derive(Debug)]
pub struct BufferedPipeTarget {
    writer: PipeWriter,
}

impl PayloadTarget for BufferedPipeTarget {
    fn write(&mut self, bytes: &[u8]) -> rugix_bundle::BundleResult<()> {
        self.writer.write_all(bytes).whatever("write failed")
    }

    fn finalize(mut self) -> rugix_bundle::BundleResult<()> {
        self.writer.flush().whatever("flush failed")
    }
}

#[derive(Debug)]
pub struct CustomTarget {
    child: Child,
}

impl CustomTarget {
    pub fn new<'arg>(mut command: impl Iterator<Item = &'arg str>) -> SystemResult<Self> {
        let Some(prog) = command.next() else {
            bail!("custom update handler cannot be an empty sequence");
        };
        let child = std::process::Command::new(prog)
            .args(command)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .whatever("unable to spawn custom update handler")?;
        Ok(Self { child })
    }
}

impl PayloadTarget for CustomTarget {
    fn write(&mut self, bytes: &[u8]) -> rugix_bundle::BundleResult<()> {
        self.child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(bytes)
            .whatever("unable to write payload to custom handler")
    }

    fn finalize(mut self) -> rugix_bundle::BundleResult<()> {
        info!("waiting on custom update handler to finalize");
        // Flush all bytes and close stdin.
        drop(self.child.stdin.take().unwrap());
        let status = self
            .child
            .wait()
            .whatever("error waiting for update handler")?;
        if !status.success() {
            bail!(
                "error running custom update handler, code {:?}",
                status.code()
            )
        }
        Ok(())
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Boolean {
    True,
    False,
}

#[derive(Debug, Parser)]
#[clap(author, version = rugix_version::RUGIX_GIT_VERSION, about)]
pub struct Args {
    /// The command.
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Manage the persistent state of the system.
    #[clap(subcommand)]
    State(StateCommand),
    /// Install and inspect over-the-air updates.
    #[clap(subcommand)]
    Update(UpdateCommand),
    /// Manage the system.
    #[clap(subcommand)]
    System(SystemCommand),
    /// Manage the update slots of the system.
    #[clap(subcommand)]
    Slots(SlotsCommand),
    /// Control the boot flow of the system.
    #[clap(subcommand)]
    Boot(BootCommand),
    /// Unstable experimental commands.
    #[clap(subcommand)]
    Unstable(UnstableCommand),
}

#[derive(Debug, Parser)]
pub enum StateCommand {
    /// Perform a factory reset of the system.
    Reset {
        /// Backup the old state by creating a new state profile.
        #[clap(long)]
        backup: bool,
        /// Name of the backup state profile.
        #[clap(long)]
        backup_name: Option<String>,
    },
    /// Configure the root filesystem overlay.
    #[clap(subcommand)]
    Overlay(OverlayCommand),
}

#[derive(Debug, Parser)]
pub enum SlotsCommand {
    /// Verify the integrity of a slot.
    Verify { slot: String },
    /// Query the state of a slot.
    Inspect { slot: String },
    /// Add an index to a slot.
    CreateIndex {
        slot: String,
        chunker: ChunkerAlgorithm,
        hash_algorithm: HashAlgorithm,
    },
}

#[derive(Debug, Parser)]
pub enum OverlayCommand {
    /// Set the persistency of the overlay.
    ForcePersist { persist: Boolean },
}

#[derive(Debug, Parser)]
pub enum UpdateCommand {
    /// Install an update.
    Install {
        /// Path to the update bundle.
        bundle: String,
        /// Check whether the (streamed) image matches the given hash.
        #[clap(long)]
        check_hash: Option<String>,
        /// Verify the signatures of the bundle.
        #[clap(long)]
        verify_signature: bool,
        /// Root certificate to use for signature verification.
        #[clap(long = "root-cert")]
        root_cert: Vec<PathBuf>,
        /// Verify a bundle based on the provided hash.
        #[clap(long)]
        verify_bundle: Option<HashDigest>,
        /// Do not delete an existing overlay.
        #[clap(long)]
        keep_overlay: bool,
        /// Control how to reboot the system.
        #[clap(long)]
        reboot: Option<UpdateRebootType>,
        /// Boot group to install the update to.
        #[clap(long)]
        boot_group: Option<String>,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum UpdateRebootType {
    Yes,
    No,
    Deferred,
}

#[derive(Debug, Parser)]
pub enum SystemCommand {
    Info {
        /// Output system information as JSON.
        #[clap(long)]
        json: bool,
    },
    /// Make the active system the default.
    Commit,
    /// Reboot the system.
    Reboot {
        /// Reboot into the spare system.
        #[clap(long)]
        spare: bool,
    },
}

#[derive(Debug, Parser)]
pub enum UnstableCommand {
    /// Set deferred spare reboot flag.
    SetDeferredSpareReboot {
        value: Boolean,
    },
    PrintSystemInfo,
}

#[derive(Debug, Parser)]
pub enum BootCommand {
    /// Mark a boot group as good.
    MarkGood { group: Option<String> },
    /// Mark a boot group as bad.
    MarkBad { group: String },
}
