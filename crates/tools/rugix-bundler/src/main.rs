use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap::Parser;

use reportify::{bail, ResultExt};
use rugix_bundle::format::tags::TagNameResolver;
use rugix_bundle::manifest::{
    BundleManifest, DeliveryConfig, DeltaEncoding, DeltaEncodingFormat, DeltaEncodingInput,
    HashAlgorithm,
};
use rugix_bundle::reader::BundleReader;
use rugix_bundle::source::FileSource;
use rugix_bundle::xdelta::xdelta_compress;
use rugix_bundle::BundleResult;
use si_crypto_hashes::HashDigest;
use tracing::{info, Level};

#[derive(Debug, Parser)]
#[clap(version = rugix_version::RUGIX_GIT_VERSION)]
pub struct Args {
    #[clap(subcommand)]
    cmd: Cmd,
    #[clap(flatten)]
    logging: si_observability::clap4::LoggingArgs,
}

#[derive(Debug, Parser)]
pub enum Cmd {
    /// Create a bundle from a bundle directory.
    Bundle(BundleCmd),
    /// Unpack a bundle into a bundle directory.
    Unpack(UnpackCmd),
    /// Hash the header of a bundle.
    Hash(HashCmd),
    /// Extract a payload from a bundle.
    Extract(ExtractCmd),
    /// Compute a static delta update.
    Delta(DeltaCmd),
    /// Inspect an update bundle.
    Inspect(InspectCmd),
    /// Print the low-level structure of a bundle.
    #[clap(hide(true))]
    PrintStructure(PrintCmd),
}

#[derive(Debug, Parser)]
pub struct PrintCmd {
    bundle: PathBuf,
}

#[derive(Debug, Parser)]
pub struct BundleCmd {
    /// Source bundle directory.
    src: PathBuf,
    /// Output bundle file.
    dst: PathBuf,
}

#[derive(Debug, Parser)]
pub struct ListCmd {
    bundle: PathBuf,
}

#[derive(Debug, Parser)]
pub struct ExtractCmd {
    #[clap(long)]
    verify_bundle: Option<HashDigest>,
    bundle: PathBuf,
    payload: usize,
    dst: PathBuf,
}

#[derive(Debug, Parser)]
pub struct DeltaCmd {
    /// Slots to compute patches for.
    #[clap(long = "slot")]
    slots: Vec<String>,
    /// Path to the old bundle.
    old: PathBuf,
    /// Path to the new bundle.
    new: PathBuf,
    /// Path to the output patch bundle.
    out: PathBuf,
}

#[derive(Debug, Parser)]
pub struct UnpackCmd {
    /// Path to the bundle.
    src: PathBuf,
    /// Output directory.
    out: PathBuf,
}

#[derive(Debug, Parser)]
pub struct InspectCmd {
    #[clap(long)]
    verify_bundle: Option<HashDigest>,
    bundle: PathBuf,
}

#[derive(Debug, Parser)]
pub struct HashCmd {
    bundle: PathBuf,
}

fn main() -> BundleResult<()> {
    let args = Args::parse();
    let _guard = si_observability::Initializer::new("RUGIX")
        .apply(&args.logging)
        .init();
    match args.cmd {
        Cmd::Bundle(create_cmd) => {
            rugix_bundle::builder::pack(&create_cmd.src, &create_cmd.dst)?;
        }
        Cmd::Unpack(cmd) => {
            unpack(&cmd.src, &cmd.out)?;
        }
        Cmd::Extract(unpack_cmd) => {
            let source = FileSource::from_unbuffered(File::open(&unpack_cmd.bundle).unwrap());
            let mut reader = BundleReader::start(source, unpack_cmd.verify_bundle)?;
            let mut did_read = false;
            while let Some(payload_reader) = reader.next_payload()? {
                if payload_reader.idx() != unpack_cmd.payload {
                    payload_reader.skip()?;
                } else {
                    println!("unpacking payload...");
                    let target = std::fs::OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .read(true)
                        .write(true)
                        .open(&unpack_cmd.dst)
                        .whatever("unable to open payload target")?;
                    payload_reader.decode_into(target, None, &mut |_| {})?;
                    did_read = true;
                    break;
                }
            }
            if !did_read {
                bail!("not enough payloads");
            }
        }
        Cmd::PrintStructure(print_cmd) => {
            let mut source = FileSource::from_unbuffered(File::open(&print_cmd.bundle).unwrap());
            rugix_bundle::format::stlv::pretty_print(&mut source, Some(&TagNameResolver)).unwrap();
        }
        Cmd::Hash(hash_cmd) => {
            let hash = rugix_bundle::bundle_hash(&hash_cmd.bundle).unwrap();
            println!("{hash}");
        }
        Cmd::Inspect(inspect_cmd) => {
            let source = FileSource::from_unbuffered(File::open(&inspect_cmd.bundle).unwrap());
            let reader = BundleReader::start(source, inspect_cmd.verify_bundle)?;
            println!("Payloads:");
            for (idx, entry) in reader.header().payload_index.iter().enumerate() {
                if let Some(slot_type) = &entry.type_slot {
                    println!(
                        "  {idx}: slot={:?} file={}",
                        slot_type.slot,
                        HashDigest::new_unchecked(
                            reader.header().hash_algorithm,
                            &entry.file_hash.raw
                        )
                    );
                }
                if let Some(type_execute) = &entry.type_execute {
                    let command = type_execute.handler.join(" ");
                    println!(
                        "  {idx}: execute({command}) file={}",
                        HashDigest::new_unchecked(
                            reader.header().hash_algorithm,
                            &entry.file_hash.raw
                        )
                    );
                }
            }
        }
        Cmd::Delta(cmd) => {
            let old_dir = tempfile::TempDir::new().unwrap();
            info!(directory = ?old_dir.path(), "unpacking old update bundle");
            unpack(&cmd.old, old_dir.path())?;
            let new_dir = tempfile::TempDir::new().unwrap();
            info!(direction = ?new_dir.path(), "unpacking new update bundle");
            unpack(&cmd.new, new_dir.path())?;
            let old_manifest = toml::from_str::<BundleManifest>(
                &std::fs::read_to_string(old_dir.path().join("rugix-bundle.toml")).unwrap(),
            )
            .unwrap();
            let mut new_manifest = toml::from_str::<BundleManifest>(
                &std::fs::read_to_string(old_dir.path().join("rugix-bundle.toml")).unwrap(),
            )
            .unwrap();
            let slots = if cmd.slots.is_empty() {
                &["system".to_owned()]
            } else {
                cmd.slots.as_slice()
            };
            for slot in slots {
                let Some(new_payload_idx) =
                    new_manifest
                        .payloads
                        .iter()
                        .position(|p| match &p.delivery {
                            DeliveryConfig::Slot(config) => &config.slot == slot,
                            _ => false,
                        })
                else {
                    panic!("unable to find slot {slot} in new bundle");
                };
                let Some(old_payload_idx) =
                    new_manifest
                        .payloads
                        .iter()
                        .position(|p| match &p.delivery {
                            DeliveryConfig::Slot(config) => &config.slot == slot,
                            _ => false,
                        })
                else {
                    panic!("unable to find slot {slot} in old bundle");
                };
                info!(%slot, "computing delta");
                let old_filename = &old_manifest.payloads[old_payload_idx].filename;
                let new_filename = &old_manifest.payloads[new_payload_idx].filename;
                let new_filename_patched = format!("{new_filename}.xdelta");
                let old_path = old_dir.path().join("payloads").join(old_filename);
                let new_path = new_dir.path().join("payloads").join(new_filename);
                let hash_algorithm = new_manifest
                    .hash_algorithm
                    .unwrap_or(si_crypto_hashes::HashAlgorithm::Sha512_256);
                let old_hash = hash_file(hash_algorithm, &old_path);
                let new_hash = hash_file(hash_algorithm, &new_path);
                let patch_path = new_dir.path().join("payloads").join(&new_filename_patched);
                xdelta_compress(&old_path, &new_path, &patch_path)?;
                std::fs::remove_file(&new_path).unwrap();
                assert!(patch_path.exists());
                let new_payload = &mut new_manifest.payloads[new_payload_idx];
                new_payload.filename = new_filename_patched;
                new_payload.delta_encoding = Some(DeltaEncoding::new(
                    vec![DeltaEncodingInput {
                        hashes: vec![old_hash],
                    }],
                    DeltaEncodingFormat::Xdelta,
                    new_hash,
                ));
            }
            std::fs::write(
                new_dir.path().join("rugix-bundle.toml"),
                toml::to_string(&new_manifest).unwrap(),
            )
            .unwrap();
            rugix_bundle::builder::pack(new_dir.path(), &cmd.out)?;
        }
    }
    Ok(())
}

pub fn unpack(src: &Path, dst: &Path) -> BundleResult<()> {
    std::fs::create_dir_all(dst).unwrap();
    let source = FileSource::from_unbuffered(File::open(&src).unwrap());
    let mut reader = BundleReader::start(source, None)?;
    let Some(manifest) = &reader.header().manifest else {
        panic!("unpacking requires a manifest");
    };
    let manifest = serde_json::from_str::<BundleManifest>(&manifest).unwrap();
    std::fs::write(
        dst.join("rugix-bundle.toml"),
        toml::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();
    let payload_dir = dst.join("payloads");
    std::fs::create_dir_all(&payload_dir).unwrap();
    while let Some(payload_reader) = reader.next_payload()? {
        let filename = &manifest.payloads[payload_reader.idx()].filename;
        info!(%filename, "unpacking bundle payload");
        let target = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(payload_dir.join(filename))
            .whatever("unable to open payload target")?;
        payload_reader.decode_into(target, None, &mut |_| {})?;
    }
    Ok(())
}

#[tracing::instrument(level = Level::DEBUG)]
pub fn hash_file(algorithm: HashAlgorithm, path: &Path) -> HashDigest {
    let mut file = std::fs::File::open(&path).unwrap();
    let mut buffer = vec![0u8; 8096];
    let mut hasher = algorithm.hasher();
    loop {
        let chunk_size = file.read(&mut buffer).unwrap();
        if chunk_size > 0 {
            hasher.update(&buffer[..chunk_size]);
        } else {
            break;
        }
    }
    hasher.finalize()
}
