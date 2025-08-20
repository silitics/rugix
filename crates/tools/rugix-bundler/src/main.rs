use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use cms::cert::x509::der::oid::db::rfc5911::ID_SIGNED_DATA;
use cms::cert::x509::der::Decode;
use reportify::{bail, ResultExt};
use rugix_bundle::format::decode::decode_slice;
use rugix_bundle::format::tags::TagNameResolver;
use rugix_bundle::manifest::{
    BlockEncoding, BundleManifest, Compression, DeliveryConfig, DeltaEncoding, DeltaEncodingFormat,
    DeltaEncodingInput, HashAlgorithm, XzCompression,
};
use rugix_bundle::reader::BundleReader;
use rugix_bundle::source::FileSource;
use rugix_bundle::xdelta::xdelta_compress;
use rugix_bundle::{add_bundle_signature, bundle_hash, format, signed_metadata, BundleResult};
use rugix_chunker::ChunkerAlgorithm;
use si_crypto_hashes::HashDigest;
use tracing::{info, Level};
use xscript::{cmd_os, run, ParentEnv, Run};

mod simulation;

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
    /// Manipulate and inspect signatures.
    #[clap(subcommand)]
    Signatures(SignaturesCmd),
    /// Simulate an update.
    #[clap(subcommand)]
    Simulator(simulation::SimulationCmd),
    /// Print the low-level structure of a bundle.
    #[clap(hide(true))]
    PrintStructure(PrintCmd),
}

#[derive(Debug, Subcommand)]
pub enum SignaturesCmd {
    /// Add a signature to a bundle.
    Add {
        /// Bundle to add the signature to.
        bundle: PathBuf,
        /// Signature in CMS format.
        signature: PathBuf,
        /// Output bundle.
        out: PathBuf,
    },
    /// Extract bundle metadata for signing.
    Prepare {
        /// Bundle to extract metadata from.
        bundle: PathBuf,
        /// Output path.
        out: PathBuf,
    },
    /// List the signatures in a bundle.
    List {
        /// Bundle to inspect.
        bundle: PathBuf,
    },
    /// Sign a bundle.
    Sign {
        /// Additional intermediate certificates to include.
        #[clap(long = "intermediate-cert")]
        certs: Vec<PathBuf>,
        /// Bundle to sign.
        bundle: PathBuf,
        /// Signer certificate.
        cert: PathBuf,
        /// Signer private key.
        key: PathBuf,
        /// Output path.
        out: PathBuf,
    },
    /// Verify that the bundle has been signed using the given certificate.
    Verify {
        /// Bundle to verify.
        bundle: PathBuf,
        /// Root certificate.
        cert: PathBuf,
    },
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
    /// Disable compression of individual patch blocks.
    #[clap(long)]
    without_compression: bool,
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
                &["system".to_owned(), "boot:system".to_owned()]
            } else {
                cmd.slots.as_slice()
            };
            for slot in slots {
                let (new_slot, old_slot) = slot
                    .split_once(':')
                    .unwrap_or((slot.as_str(), slot.as_str()));
                let Some(new_payload_idx) =
                    new_manifest
                        .payloads
                        .iter()
                        .position(|p| match &p.delivery {
                            DeliveryConfig::Slot(config) => &config.slot == new_slot,
                            _ => false,
                        })
                else {
                    panic!("unable to find slot {new_slot} in new bundle");
                };
                let Some(old_payload_idx) =
                    new_manifest
                        .payloads
                        .iter()
                        .position(|p| match &p.delivery {
                            DeliveryConfig::Slot(config) => &config.slot == old_slot,
                            _ => false,
                        })
                else {
                    panic!("unable to find slot {old_slot} in old bundle");
                };
                info!(%old_slot, %new_slot, "computing delta");
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
                new_payload.block_encoding = Some(
                    BlockEncoding::new(ChunkerAlgorithm::Fixed {
                        block_size_kib: 256,
                    })
                    .with_compression(if cmd.without_compression {
                        None
                    } else {
                        Some(Compression::Xz(XzCompression::new()))
                    }),
                );
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
        Cmd::Simulator(cmd) => {
            simulation::run(&cmd);
        }
        Cmd::Signatures(cmd) => match cmd {
            SignaturesCmd::Add {
                bundle,
                signature,
                out,
            } => {
                let signature = std::fs::read(&signature).whatever("unable to read signature")?;
                let content_info = cms::content_info::ContentInfo::from_der(&signature)
                    .expect("invalid signature");
                if content_info.content_type != ID_SIGNED_DATA {
                    bail!("invalid signature content type");
                }
                let signed_data = content_info
                    .content
                    .decode_as::<cms::signed_data::SignedData>()
                    .expect("invalid signature");
                println!("CMS Version: {:?}", signed_data.version);
                println!(
                    "Embedded Certificates: {}",
                    signed_data.certificates.map(|c| c.0.len()).unwrap_or(0)
                );
                let bundle_hash = bundle_hash(&bundle)?;
                if let Some(content) = signed_data.encap_content_info.econtent {
                    let signed_metadata = decode_slice::<format::SignedMetadata>(content.value())?;
                    if bundle_hash != signed_metadata.header_hash {
                        bail!("bundle hash does not match signature");
                    }
                } else {
                    bail!("no encapsulated content");
                }
                add_bundle_signature(&bundle, signature, &out)?;
            }
            SignaturesCmd::List { bundle } => {
                let source = FileSource::from_unbuffered(File::open(&bundle).unwrap());
                let reader = BundleReader::start(source, None)?;
                if let Some(signatures) = reader.signatures() {
                    for (idx, signature) in signatures.cms_signatures.iter().enumerate() {
                        println!("CMS Signature {} (length={})", idx, signature.raw.len());
                    }
                } else {
                    println!("No signatures found");
                }
            }
            SignaturesCmd::Prepare { bundle, out } => {
                let metadata = signed_metadata(&bundle)?;
                std::fs::write(out, metadata).whatever("unable to write metadata")?;
            }
            SignaturesCmd::Sign {
                certs,
                bundle,
                cert,
                key,
                out,
            } => {
                let tempdir =
                    tempfile::tempdir().whatever("unable to create temporary directory")?;
                let tempdir_path = tempdir.path();
                let signed_metadata_raw = tempdir_path.join("signed-metadata.raw");
                let signed_metadata_cms = tempdir_path.join("signed-metadata.cms");
                let metadata = signed_metadata(&bundle)?;
                std::fs::write(&signed_metadata_raw, metadata)
                    .whatever("unable to write metadata")?;
                let mut cmd = cmd_os!(
                    "openssl",
                    "cms",
                    "-sign",
                    "-in",
                    &signed_metadata_raw,
                    "-signer",
                    &cert,
                    "-inkey",
                    &key,
                    "-out",
                    &signed_metadata_cms,
                    "-outform",
                    "DER",
                    "-nosmimecap",
                    "-nodetach",
                    "-binary"
                );
                for cert in certs {
                    cmd.add_arg("-certfile");
                    cmd.add_arg(cert);
                }
                ParentEnv.run(cmd).whatever("unable to sign bundle")?;
                let signature =
                    std::fs::read(&signed_metadata_cms).whatever("unable to read signature")?;
                add_bundle_signature(&bundle, signature, &out)?;
            }
            SignaturesCmd::Verify { bundle, cert } => {
                let source = FileSource::from_unbuffered(File::open(&bundle).unwrap());
                let reader = BundleReader::start(source, None)?;
                let Some(signatures) = reader.signatures() else {
                    bail!("no signatures found");
                };
                let mut found_valid_signature = false;
                for signature in signatures.cms_signatures.iter() {
                    let tempdir =
                        tempfile::tempdir().whatever("unable to create temporary directory")?;
                    let tempdir_path = tempdir.path();
                    let signed_metadata_raw = tempdir_path.join("signed-metadata.raw");
                    let signed_metadata_cms = tempdir_path.join("signed-metadata.cms");
                    std::fs::write(&signed_metadata_cms, &signature.raw)
                        .whatever("unable to write CMS signature")?;
                    if let Err(error) = run!([
                        "openssl",
                        "cms",
                        "-verify",
                        "-in",
                        &signed_metadata_cms,
                        "-inform",
                        "DER",
                        "-CAfile",
                        &cert,
                        "-out",
                        &signed_metadata_raw,
                    ]) {
                        println!("{error}");
                        continue;
                    }
                    let signed_metadata = std::fs::read(&signed_metadata_raw)
                        .whatever("unable to read signed metadata")?;
                    let signed_metadata = decode_slice::<format::SignedMetadata>(&signed_metadata)
                        .whatever("unable to decode signed metadata")?;
                    if signed_metadata.header_hash
                        == reader.header_hash(signed_metadata.header_hash.algorithm())
                    {
                        found_valid_signature = true;
                        println!("Found valid signature!");
                        break;
                    }
                }
                if !found_valid_signature {
                    bail!("no valid signature found");
                }
            }
        },
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
