//! Simulation of various delta update techniques and implementations.

use std::collections::HashSet;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Subcommand;
use rugix_bundle::xdelta::xdelta_compress;
use rugix_chunker::{Chunker, ChunkerAlgorithm};
use serde::Serialize;
use si_crypto_hashes::{HashAlgorithm, HashDigest};
use tracing::info;

use crate::simulation::deltar::Instruction;
use crate::simulation::utils::compress_bytes;

mod utils;

pub mod deltar;

/// Run a simulation command.
pub fn run(cmd: &SimulationCmd) {
    match &cmd {
        SimulationCmd::BlockBased(cmd) => match cmd {
            BlockBasedCmd::Simulate {
                chunker,
                old,
                new,
                group_overhead,
                group_size,
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
                        if let Some(download_block_size) = *group_size {
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
                let block_overhead = downloaded_blocks * group_overhead;
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
                    uncompressed: usize,
                    compressed: usize,
                }

                serde_json::to_writer_pretty(
                    &std::io::stdout(),
                    &Output {
                        compressed: total_compressed,
                        uncompressed: total_uncompressed,
                    },
                )
                .unwrap();
            }
        },
        SimulationCmd::Deltar(cmd) => match cmd {
            DeltarCmd::Plan {
                path,
                group_size: group_size_limit,
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
            DeltarCmd::Simulate {
                old,
                new,
                group_size,
                group_overhead,
                chunker,
            } => {
                let archive_new =
                    tar::Archive::new(std::io::BufReader::new(std::fs::File::open(new).unwrap()));
                let (plan, groups) = deltar::compute_plan(*group_size, chunker, archive_new);
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
        SimulationCmd::Xdelta(cmd) => match cmd {
            XdeltaCmd::Simulate { old, new } => {
                let tempdir = tempfile::tempdir().unwrap();
                let patch = tempdir.path().join("patch.xdelta");
                xdelta_compress(old, new, &patch).unwrap();
                let size = patch.metadata().unwrap().len();
                println!("Xdelta Size: {size}");

                #[derive(Serialize)]
                struct Output {
                    patch: u64,
                }

                serde_json::to_writer_pretty(&std::io::stdout(), &Output { patch: size }).unwrap();
            }
        },
    }
}

/// Simulation command.
#[derive(Debug, Subcommand)]
pub enum SimulationCmd {
    /// Simulation for block-based updates.
    #[clap(subcommand)]
    BlockBased(BlockBasedCmd),
    /// Simulation for delta updates with the experimental Deltar format.
    #[clap(subcommand)]
    Deltar(DeltarCmd),
    /// Computation for static delta updates through Xdelta.
    #[clap(subcommand)]
    Xdelta(XdeltaCmd),
}

/// Simulation for delta updates using the experimental Deltar format.
#[derive(Debug, Subcommand)]
pub enum DeltarCmd {
    /// Compute and print a plan from the given input tar archive.
    Plan {
        /// Tar archive.
        path: PathBuf,
        /// Chunker algorithm to use for chunking files into blocks.
        #[clap(long, default_value = "casync-16")]
        chunker: ChunkerAlgorithm,
        /// Size to group blocks into for compression.
        #[clap(long, default_value = "32768")]
        group_size: usize,
    },
    /// Simulate delta updates using the experimental Deltar format.
    Simulate {
        /// Old rootfs tar archive.
        old: PathBuf,
        /// New rootfs tar archive.
        new: PathBuf,
        /// Chunker algorithm to use for chunking files into blocks.
        #[clap(long, default_value = "casync-16")]
        chunker: ChunkerAlgorithm,
        /// Size to group blocks into for compression.
        #[clap(long, default_value = "32768")]
        group_size: usize,
        /// Overhead for downloading a group of blocks.
        #[clap(long, default_value = "768")]
        group_overhead: u64,
    },
}

/// Simulation for delta updates using block-based diffing.
#[derive(Debug, Subcommand)]
pub enum BlockBasedCmd {
    /// Simulate delta updates using block-based diffing.
    Simulate {
        /// Old filesystem image (do NOT provide an update bundle).
        old: PathBuf,
        /// New filesystem image (do NOT provide an update bundle).
        new: PathBuf,
        /// Chunking algorithm to use for determining block boundaries.
        #[clap(long, default_value = "casync-64")]
        chunker: ChunkerAlgorithm,
        /// Overhead for downloading a block or group of blocks.
        #[clap(long, default_value = "768")]
        group_overhead: u64,
        /// Optional size to group blocks into for compression.
        ///
        /// Only works for `fixed` chunkers at the moment.
        #[clap(long)]
        group_size: Option<u64>,
    },
}

/// Simulation for delta updates using Xdelta.
#[derive(Debug, Subcommand)]
pub enum XdeltaCmd {
    /// Simulate a delta update using Xdelta.
    Simulate {
        /// Old filesystem image (do NOT provide an update bundle).
        old: PathBuf,
        /// New filesystem image (do NOT provide an update bundle).
        new: PathBuf,
    },
}
