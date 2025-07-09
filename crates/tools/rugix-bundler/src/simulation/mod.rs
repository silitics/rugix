//! Simulation of various delta update techniques and implementations.

use std::collections::HashSet;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use clap::Subcommand;
use rugix_bundle::block_encoding::block_index::BlockIndexConfig;
use rugix_chunker::{Chunker, ChunkerAlgorithm};
use serde::Serialize;
use si_crypto_hashes::{HashAlgorithm, HashDigest};
use tracing::info;

use crate::simulation::deltar::Instruction;
use crate::simulation::utils::compress_bytes;

mod utils;

pub mod deltar;

pub fn simulate(cmd: &Cmd) {
    match &cmd {
        Cmd::BlockBased(cmd) => match cmd {
            BlockBasedCmd::Benchmark {
                chunker,
                old,
                new,
                block_overhead,
                download_block_size,
            } => {
                let old = std::fs::read(old).unwrap();
                let mut table = HashSet::new();
                let mut old_blocks = 0;
                for chunk in chunker.chunker().unwrap().chunks(&old) {
                    let hash = HashAlgorithm::Sha256.hash::<Arc<[u8]>>(chunk);
                    table.insert(hash);
                    old_blocks += 1;
                }
                drop(old);
                let new = std::fs::read(new).unwrap();
                let mut new_blocks = 0;
                let mut downloaded_uncompressed = 0;
                let mut downloaded_compressed = 0;
                let mut downloaded_blocks = 0;
                let mut index_data = Vec::new();
                let mut sizes_data = Vec::new();
                let mut downloaded_blocks_set = HashSet::new();
                for (number, chunk) in chunker.chunker().unwrap().chunks(&new).enumerate() {
                    let hash = HashAlgorithm::Sha256.hash(chunk);
                    index_data.extend_from_slice(hash.as_ref());
                    if table.insert(hash) {
                        if let Some(download_block_size) = *download_block_size {
                            let number = number as u64;
                            let chunk_block_size = match chunker {
                                ChunkerAlgorithm::Casync { .. } => panic!("unsupported option"),
                                ChunkerAlgorithm::Fixed { block_size_kib } => {
                                    (*block_size_kib as u64) * 1024
                                }
                            };
                            assert!(download_block_size > chunk_block_size);
                            let chunk_block = (number * chunk_block_size) / download_block_size;
                            if downloaded_blocks_set.insert(chunk_block) {
                                let chunk_block_start =
                                    (chunk_block * download_block_size) as usize;
                                let chunk_block_end = ((chunk_block + 1) * download_block_size)
                                    .min(new.len() as u64)
                                    as usize;
                                let chunk_block = &new[chunk_block_start..chunk_block_end];
                                downloaded_uncompressed += chunk_block.len();
                                let compressed_len = compress_bytes(chunk_block).len();
                                downloaded_compressed += compressed_len;
                                sizes_data.extend_from_slice(
                                    &u32::try_from(compressed_len).unwrap().to_be_bytes(),
                                );
                                downloaded_blocks += 1;
                            }
                        } else {
                            downloaded_uncompressed += chunk.len();
                            let compressed_len = compress_bytes(chunk).len();
                            downloaded_compressed += compressed_len;
                            sizes_data.extend_from_slice(
                                &u32::try_from(compressed_len).unwrap().to_be_bytes(),
                            );
                            downloaded_blocks += 1;
                        }
                    }
                    new_blocks += 1;
                }
                let index_compressed = compress_bytes(&index_data).len();
                let sizes_compressed = compress_bytes(&sizes_data).len();
                let sizes_uncompressed = if chunker.is_fixed() {
                    0
                } else {
                    sizes_data.len()
                };
                let block_overhead = downloaded_blocks * block_overhead;
                eprintln!("Old Blocks: {}", old_blocks);
                eprintln!("New Blocks: {}", new_blocks);
                eprintln!("Downloaded Blocks: {}", downloaded_blocks);
                eprintln!("Block Overhead: {}", block_overhead);

                eprintln!("Index Uncompressed: {}", index_data.len());
                eprintln!("Sizes Uncompressed: {}", sizes_uncompressed);
                eprintln!("Data Uncompressed: {}", downloaded_uncompressed);
                let total_uncompressed = index_data.len()
                    + sizes_uncompressed
                    + downloaded_uncompressed
                    + block_overhead as usize;
                eprintln!("Total Uncompressed: {total_uncompressed}");

                eprintln!("Index Compressed: {}", index_compressed);
                eprintln!("Sizes Compressed: {}", sizes_compressed);
                eprintln!("Data Compressed: {}", downloaded_compressed);
                let total_compressed = index_compressed
                    + sizes_compressed
                    + downloaded_compressed
                    + block_overhead as usize;
                eprintln!("Total Compressed: {}", total_compressed);

                #[derive(Serialize)]
                struct Output {
                    total_uncompressed: usize,
                    total_compressed: usize,
                }

                serde_json::to_writer_pretty(
                    &std::io::stdout(),
                    &Output {
                        total_compressed,
                        total_uncompressed,
                    },
                )
                .unwrap();
            }
        },
        Cmd::Deltar(cmd) => match cmd {
            DeltarCmd::Plan {
                path,
                group_size_limit,
                chunker,
            } => {
                let archive =
                    tar::Archive::new(std::io::BufReader::new(std::fs::File::open(path).unwrap()));
                let (plan, _) = deltar::compute_plan(*group_size_limit, chunker, archive);
                for instr in plan {
                    match instr {
                        deltar::Instruction::Push { path } => {
                            println!("Push: {path:?}");
                        }
                        deltar::Instruction::Pop => {
                            println!("Pop");
                        }
                        deltar::Instruction::Owner { uid, gid } => {
                            println!("Owner: {uid}:{gid}");
                        }
                        deltar::Instruction::Mode { mode } => {
                            println!("Mode: {mode:#o}");
                        }
                        deltar::Instruction::File { chunks } => {
                            println!("File: {}", chunks.len());
                        }
                        deltar::Instruction::Directory => {
                            println!("Directory");
                        }
                        deltar::Instruction::CharacterDevice { major, minor } => {
                            println!("CharacterDevice: {major}:{minor}");
                        }
                        deltar::Instruction::BlockDevice { major, minor } => {
                            println!("BlockDevice: {major}:{minor}");
                        }
                        deltar::Instruction::Link { target } => {
                            println!("Link: {target:?}");
                        }
                    }
                }
            }
            DeltarCmd::Benchmark {
                old,
                new,
                group_size_limit,
                group_overhead,
                chunker,
            } => {
                let archive_new =
                    tar::Archive::new(std::io::BufReader::new(std::fs::File::open(new).unwrap()));
                let (plan, groups) = deltar::compute_plan(*group_size_limit, chunker, archive_new);
                let serialized_plan = deltar::serialize_plan(&plan);
                let compressed_plan = utils::compress_bytes(&serialized_plan);
                let mut local_blocks = HashSet::new();
                let mut archive_old =
                    tar::Archive::new(std::io::BufReader::new(std::fs::File::open(old).unwrap()));
                for entry in archive_old.entries().unwrap() {
                    let mut entry = entry.unwrap();
                    if !matches!(entry.header().entry_type(), tar::EntryType::Regular) {
                        continue;
                    }
                    let mut data = Vec::new();
                    entry.read_to_end(&mut data).unwrap();
                    for chunk in chunker.chunker().unwrap().chunks(&data) {
                        let hash: HashDigest = HashAlgorithm::Sha256.hash(chunk);
                        local_blocks.insert(hash);
                    }
                }
                let mut downloaded_groups = HashSet::new();
                let mut downloaded_size = 0;
                for instruction in &plan {
                    let Instruction::File { chunks } = instruction else {
                        continue;
                    };
                    for chunk in chunks {
                        let hash = &chunk.hash;
                        if local_blocks.contains(hash) {
                            // Nothing to download, block is locally available.
                            continue;
                        }
                        if !downloaded_groups.insert(chunk.address.group_number) {
                            // We already downloaded this group.
                            continue;
                        }
                        let group = &groups[chunk.address.group_number as usize];
                        downloaded_size += group.compressed.len() as u64;
                        downloaded_size += group_overhead;
                    }
                }
                info!(
                    "Plan Size: {:.2} MiB",
                    compressed_plan.len() as f64 / 1024.0 / 1024.0
                );
                info!(
                    "Data Size: {:.2} MiB",
                    downloaded_size as f64 / 1024.0 / 1024.0
                );

                #[derive(Serialize)]
                struct Output {
                    plan: u64,
                    data: u64,
                }

                serde_json::to_writer_pretty(
                    &std::io::stdout(),
                    &Output {
                        plan: compressed_plan.len() as u64,
                        data: downloaded_size,
                    },
                )
                .unwrap();
            }
        },
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeltaStats {
    entries_downloaded: usize,
    data_downloaded: u64,
}

pub fn compute_hash_set(
    config: &BlockIndexConfig,
    block: &[FsEntryBlock],
) -> HashSet<si_crypto_hashes::HashDigest> {
    let mut table = HashSet::new();
    for block in block {
        for entry in &block.entries {
            let FsEntryKind::File { data, .. } = &entry.kind else {
                continue;
            };
            let chunker = config.chunker.chunker().unwrap();
            for chunk in chunker.chunks(&data) {
                let hash = config.hash_algorithm.hash(chunk);
                table.insert(hash);
            }
        }
    }
    table
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    #[clap(subcommand)]
    BlockBased(BlockBasedCmd),
    /// Delta updates with the experimental deltar format.
    #[clap(subcommand)]
    Deltar(DeltarCmd),
}

#[derive(Debug, Subcommand)]
pub enum DeltarCmd {
    /// Compute and print a plan from the given input tar archive.
    Plan {
        /// Path to the tar archive.
        path: PathBuf,
        /// Maximum size of a chunk group.
        #[clap(long, default_value = "32768")]
        group_size_limit: usize,
        /// Chunker algorithm.
        #[clap(long, default_value = "casync-16")]
        chunker: ChunkerAlgorithm,
    },
    /// Benchmark a delta update.
    Benchmark {
        /// Old version.
        old: PathBuf,
        /// New version.
        new: PathBuf,
        /// Maximum size of a chunk group.
        #[clap(long, default_value = "32768")]
        group_size_limit: usize,
        /// Overhead for downloading a group.
        #[clap(long, default_value = "512")]
        group_overhead: u64,
        /// Chunker algorithm.
        #[clap(long, default_value = "casync-16")]
        chunker: ChunkerAlgorithm,
    },
}

#[derive(Debug, Subcommand)]
pub enum BlockBasedCmd {
    Benchmark {
        chunker: ChunkerAlgorithm,
        old: PathBuf,
        new: PathBuf,
        #[clap(long, default_value = "100")]
        block_overhead: u64,
        #[clap(long)]
        download_block_size: Option<u64>,
    },
}

#[derive(Debug)]
pub struct FsEntryBlock {
    pub first: PathBuf,
    pub last: PathBuf,
    pub hash: HashDigest,
    pub entries: Vec<FsEntry>,
}

#[derive(Debug)]
pub struct FsEntry {
    pub position: PathBuf,
    pub kind: FsEntryKind,
}

impl FsEntry {
    pub fn from_path(path: &Path, position: PathBuf) -> Option<Self> {
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) => {
                panic!("unable to read metadata of {path:?}: {error}");
            }
        };
        let file_type = metadata.file_type();
        let entry_kind = if file_type.is_file() {
            let data = std::fs::read(path).unwrap();
            let hash = HashAlgorithm::Sha256.hash(&data);
            FsEntryKind::File { hash, data }
        } else if file_type.is_dir() {
            FsEntryKind::Directory
        } else if file_type.is_symlink() {
            FsEntryKind::Symlink {
                target: std::fs::read_link(path).unwrap(),
            }
        } else {
            return None;
        };
        Some(FsEntry {
            position,
            kind: entry_kind,
        })
    }

    pub fn stable_hash(&self) -> HashDigest {
        let mut hasher = HashAlgorithm::Sha256.hasher();
        hasher.update(self.position.as_os_str().as_encoded_bytes());
        match &self.kind {
            FsEntryKind::Directory => hasher.update(b"directory"),
            FsEntryKind::File { hash, .. } => {
                hasher.update(b"file");
                hasher.update(hash.as_ref());
            }
            FsEntryKind::Symlink { target } => {
                hasher.update(b"symlink");
                hasher.update(target.as_os_str().as_encoded_bytes());
            }
        }
        hasher.finalize()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FsEntryKind {
    Directory,
    File { hash: HashDigest, data: Vec<u8> },
    Symlink { target: PathBuf },
}
