#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use clap::{Parser, Subcommand};
use dir_stream::DirStream;
use rugix_bundle::block_encoding::block_index::{BlockIndexConfig, compute_block_index};
use rugix_bundle::block_encoding::block_table::BlockTable;
use rugix_chunker::{Chunker, ChunkerAlgorithm};
use rugix_compression::ByteProcessor;
use serde::Serialize;
use si_crypto_hashes::{HashAlgorithm, HashDigest, Hasher};
use walkdir::WalkDir;

mod dir_stream;

pub enum FsCommand {
    Push {
        path: OsString,
    },
    Pop,
    File {
        filename: OsString,
        chunks: Vec<FileChunk>,
        data: Vec<u8>,
    },
    Link {
        filename: OsString,
        target: PathBuf,
    },
    Owner {
        uid: u32,
        gid: u32,
    },
    Mode {
        mode: u32,
    },
}

pub struct FileChunk {
    hash: HashDigest,
    block_number: u32,
    chunk_number: u16,
}

pub fn compute_plan_size(plan: &[FsCommand]) -> u64 {
    let mut size = 0;
    for cmd in plan {
        size += 1;
        match cmd {
            FsCommand::Push { path } => {
                size += 1;
                size += path.as_encoded_bytes().len() as u64;
            }
            FsCommand::Pop => { /* no args */ }
            FsCommand::File {
                filename, chunks, ..
            } => {
                size += 1;
                size += filename.as_encoded_bytes().len() as u64;
                size += 2;
                size += 32 * chunks.len() as u64;
                size += 4 * chunks.len() as u64;
                size += 2 * chunks.len() as u64;
            }
            FsCommand::Link { filename, target } => {
                size += 1;
                size += filename.as_encoded_bytes().len() as u64;
                size += 2;
                size += target.as_os_str().as_encoded_bytes().len() as u64;
            }
            FsCommand::Owner { .. } => {
                size += 8;
            }
            FsCommand::Mode { .. } => {
                size += 4;
            }
        }
    }
    size
}

pub fn serialize_plan(root: &PathBuf, plan: &[FsCommand]) -> Vec<u8> {
    let mut current = root.to_path_buf();
    let mut serialized = Vec::new();
    for cmd in plan {
        match cmd {
            FsCommand::Push { path } => {
                serialized.push(1);
                current.push(path);
                let path = path.as_encoded_bytes();
                serialized.push(u8::try_from(path.len()).unwrap());
                serialized.extend_from_slice(path);
            }
            FsCommand::Pop => {
                current.pop();
                serialized.push(2);
            }
            FsCommand::File {
                filename, chunks, ..
            } => {
                serialized.push(3);
                current.push(filename);
                let filename = filename.as_encoded_bytes();
                serialized.push(u8::try_from(filename.len()).unwrap());
                serialized.extend_from_slice(filename);
                serialized.extend_from_slice(&u16::try_from(chunks.len()).unwrap().to_be_bytes());
                for chunk in chunks {
                    serialized.extend_from_slice(chunk.hash.as_ref());
                    serialized.extend_from_slice(&chunk.block_number.to_be_bytes());
                    serialized.extend_from_slice(&chunk.chunk_number.to_be_bytes());
                }
                current.pop();
            }
            FsCommand::Link { filename, target } => {
                serialized.push(3);
                let filename = filename.as_encoded_bytes();
                serialized.push(u8::try_from(filename.len()).unwrap());
                serialized.extend_from_slice(filename);
                let target = target.as_os_str().as_encoded_bytes();
                serialized.extend_from_slice(&u16::try_from(target.len()).unwrap().to_be_bytes());
                serialized.extend_from_slice(target);
            }
            FsCommand::Owner { uid, gid } => {
                serialized.push(4);
                serialized.extend_from_slice(&uid.to_be_bytes());
                serialized.extend_from_slice(&gid.to_be_bytes());
            }
            FsCommand::Mode { mode } => {
                serialized.push(5);
                serialized.extend_from_slice(&mode.to_be_bytes());
            }
        }
    }
    serialized
}

pub struct ChunkCollector {
    table: HashMap<HashDigest, (u32, u16)>,
    chunk_blocks: Vec<ChunkBlock>,
    pending_data: Vec<u8>,
    current_chunks: u16,
    block_size_limit: usize,
}

pub struct ChunkBlock {
    data: Vec<u8>,
    compressed: Vec<u8>,
}

impl ChunkCollector {
    pub fn new(block_size_limit: usize) -> Self {
        Self {
            table: HashMap::new(),
            chunk_blocks: Vec::new(),
            pending_data: Vec::new(),
            current_chunks: 0,
            block_size_limit,
        }
    }

    pub fn flush(&mut self) {
        if self.pending_data.len() > 0 {
            let data = std::mem::take(&mut self.pending_data);
            let mut compressed = Vec::new();
            let mut compressor = rugix_compression::XzEncoder::new(6);
            compressor.process(&data, &mut compressed).unwrap();
            compressor.finalize(&mut compressed).unwrap();
            self.chunk_blocks.push(ChunkBlock { data, compressed });
            self.current_chunks = 0;
        }
    }

    pub fn add_chunk(&mut self, hash: &HashDigest, chunk: &[u8]) -> (u32, u16) {
        if let Some(addr) = self.table.get(hash) {
            return *addr;
        }
        let chunk_idx = u32::try_from(self.chunk_blocks.len()).unwrap();
        let number = self.current_chunks;
        self.pending_data
            .extend_from_slice(&u32::try_from(chunk.len()).unwrap().to_be_bytes());
        self.pending_data.extend_from_slice(chunk);
        self.current_chunks += 1;
        if self.pending_data.len() > self.block_size_limit {
            self.flush();
        }
        let addr = (chunk_idx, number);
        self.table.insert(hash.clone(), addr);
        addr
    }

    pub fn finalize(mut self) -> Vec<ChunkBlock> {
        self.flush();
        self.chunk_blocks
    }
}

pub fn compute_plan(
    chunker_algorithm: &ChunkerAlgorithm,
    path: &mut PathBuf,
    plan: &mut Vec<FsCommand>,
    owner: &mut (u32, u32),
    mode: &mut u32,
    chunk_collector: &mut ChunkCollector,
) {
    let metadata = std::fs::symlink_metadata(&path).unwrap();
    let uid = metadata.uid();
    let gid = metadata.gid();
    if owner.0 != uid || owner.1 != gid {
        *owner = (uid, gid);
        plan.push(FsCommand::Owner { uid, gid })
    }
    let entry_mode = metadata.permissions().mode();
    if entry_mode != *mode {
        *mode = entry_mode;
        plan.push(FsCommand::Mode { mode: entry_mode })
    }
    let file_type = metadata.file_type();
    if file_type.is_file() {
        let data = std::fs::read(&path).unwrap();
        let mut chunks = Vec::new();
        for chunk in chunker_algorithm.chunker().unwrap().chunks(&data) {
            let hash = HashAlgorithm::Sha256.hash::<Arc<[u8]>>(chunk);
            let (block_number, chunk_number) = chunk_collector.add_chunk(&hash, chunk);
            chunks.push(FileChunk {
                hash,
                block_number,
                chunk_number,
            })
        }
        plan.push(FsCommand::File {
            filename: path.file_name().unwrap().into(),
            data,
            chunks,
        })
    } else if file_type.is_dir() {
        let mut entries = std::fs::read_dir(&path)
            .unwrap()
            .into_iter()
            .map(|e| {
                let e = e.unwrap();
                (e.file_name(), e.file_type().unwrap())
            })
            .collect::<Vec<_>>();
        entries.sort_by(|(a, _), (b, _)| a.cmp(b));
        for (file_name, file_type) in entries {
            path.push(&file_name);
            if file_type.is_dir() {
                plan.push(FsCommand::Push { path: file_name })
            }
            compute_plan(chunker_algorithm, path, plan, owner, mode, chunk_collector);
            path.pop();
            if file_type.is_dir() {
                plan.push(FsCommand::Pop)
            }
        }
    } else if file_type.is_symlink() {
        plan.push(FsCommand::Link {
            filename: path.file_name().unwrap().into(),
            target: std::fs::read_link(path).unwrap(),
        })
    } else {
        println!("WARNING: Ignored path {path:?}. Unsupported file type.");
    }
}

fn main() {
    let args = Args::parse();

    let _guard = si_observability::Initializer::new("DELTAR")
        .apply(&args.logging_args)
        .init();

    match &args.cmd {
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
        Cmd::ComputePlan { path } => {
            let mut plan = Vec::new();
            let chunker_algorithm = ChunkerAlgorithm::Casync {
                avg_block_size_kib: 64,
            };
            let mut chunk_collector = ChunkCollector::new(64 * 1024);
            compute_plan(
                &chunker_algorithm,
                &mut path.clone(),
                &mut plan,
                &mut (0, 0),
                &mut 0,
                &mut chunk_collector,
            );
            let serialized = serialize_plan(path, &plan);
            println!("Plan Length: {}", plan.len());
            println!(
                "Plan Size: {} ({})",
                compute_plan_size(&plan),
                serialized.len()
            );
            let mut compressor = rugix_compression::XzEncoder::new(6);
            let mut output = Vec::new();
            compressor.process(&serialized, &mut output).unwrap();
            compressor.finalize(&mut output).unwrap();
            println!("Compressed Plan: {}", output.len());
        }
        Cmd::CopyPlanned {
            original,
            source,
            casync_block_size,
            compressor_block_size,
        } => {
            let mut plan = Vec::new();
            let chunker_algorithm = ChunkerAlgorithm::Casync {
                avg_block_size_kib: *casync_block_size,
            };
            let mut chunk_collector = ChunkCollector::new(*compressor_block_size * 1024);
            compute_plan(
                &chunker_algorithm,
                &mut original.clone(),
                &mut plan,
                &mut (0, 0),
                &mut 0,
                &mut chunk_collector,
            );
            let chunk_blocks = chunk_collector.finalize();
            let serialized = serialize_plan(source, &plan);
            println!("Plan Length: {}", plan.len());
            println!(
                "Plan Size: {} ({})",
                compute_plan_size(&plan),
                serialized.len()
            );
            let mut compressor = rugix_compression::XzEncoder::new(6);
            let mut output = Vec::new();
            compressor.process(&serialized, &mut output).unwrap();
            compressor.finalize(&mut output).unwrap();
            let compressed_plan = output.len();
            println!("Compressed Plan: {}", compressed_plan);
            let mut current_path = PathBuf::new();
            let mut local_chunks = HashSet::new();
            for entry in WalkDir::new(source) {
                let entry = entry.unwrap();
                if entry.file_type().is_file() {
                    let data = std::fs::read(entry.path()).unwrap();
                    for chunk in chunker_algorithm.chunker().unwrap().chunks(&data) {
                        local_chunks.insert(HashAlgorithm::Sha256.hash::<Arc<[u8]>>(chunk));
                    }
                }
            }
            let mut downloaded_data = 0;
            let mut downloaded_compressed = 0;
            let mut downloaded_blocks = HashSet::new();
            for cmd in &plan {
                match cmd {
                    FsCommand::Push { path } => {
                        current_path.push(path);
                    }
                    FsCommand::Pop => {
                        current_path.pop();
                    }
                    FsCommand::File {
                        filename, chunks, ..
                    } => {
                        current_path.push(filename);
                        for chunk in chunks {
                            if local_chunks.insert(chunk.hash.clone()) {
                                if downloaded_blocks.insert(chunk.block_number) {
                                    let block = &chunk_blocks[chunk.block_number as usize];
                                    downloaded_data += block.data.len() as u64;
                                    downloaded_compressed += block.compressed.len() as u64;
                                }
                            }
                        }
                    }
                    FsCommand::Link { .. } => { /* nothing to do */ }
                    FsCommand::Owner { .. } => { /* nothing to do */ }
                    FsCommand::Mode { .. } => { /* nothing to do */ }
                }
            }
            println!("Total Blocks: {}", chunk_blocks.len());
            println!("Downloaded Blocks: {}", downloaded_blocks.len());
            println!("Downloaded Data: {downloaded_data}");
            println!("Downloaded Compressed: {}", downloaded_compressed);
            let total_downloaded = downloaded_compressed + compressed_plan as u64;
            println!("Total: {}", total_downloaded);
            println!(
                "Plan %: {:.2}",
                compressed_plan as f64 / total_downloaded as f64 * 100.0
            );
        }
        Cmd::ComputeBlocks { path } => {
            let blocks = compute_blocks(path);
            for block in &blocks {
                println!("Block:");
                println!("  First: {:?}", block.first);
                println!("  Last: {:?}", block.last);
                println!("  Hash: {}", block.hash);
                println!("  Entries: {:?}", block.entries.len());
            }
            println!("Blocks: {}", blocks.len());
            println!(
                "Avg. Block Size: {:.2} entries",
                (blocks.iter().map(|b| b.entries.len()).sum::<usize>() as f64)
                    / (blocks.len() as f64)
            )
        }
        Cmd::DeltaCopy {
            original,
            source,
            target,
        } => {
            let config = BlockIndexConfig {
                hash_algorithm: si_crypto_hashes::HashAlgorithm::Sha256,
                chunker: rugix_chunker::ChunkerAlgorithm::Casync {
                    avg_block_size_kib: 4,
                },
            };
            let mut stats = DeltaStats::default();
            let blocks = compute_blocks(&original);
            println!("Target Blocks: {}", blocks.len());
            let source_blocks = compute_blocks(&source);
            println!("Source Blocks: {}", blocks.len());
            let mut source_table = compute_hash_set(&config, &source_blocks);
            drop(source_blocks);
            for block in &blocks {
                copy_block(
                    &block,
                    source,
                    target,
                    &mut stats,
                    &config,
                    &mut source_table,
                );
            }
            println!("Blocks Downloaded: {}", blocks.len());
            println!("Entries Downloaded: {}", stats.entries_downloaded);
            println!("Data Downloaded: {}", stats.data_downloaded);
        }
        Cmd::Fingerprint { path } => {
            let blocks = compute_blocks(path);
            let mut hasher = HashAlgorithm::Sha256.hasher();
            for block in blocks {
                hasher.update(block.hash.as_ref());
            }
            println!("Hash: {}", hasher.finalize::<Arc<[u8]>>());
        }
        Cmd::CasyncDelta {
            source,
            target,
            block_size_kib,
        } => {
            let config = BlockIndexConfig {
                hash_algorithm: si_crypto_hashes::HashAlgorithm::Sha256,
                chunker: rugix_chunker::ChunkerAlgorithm::Casync {
                    avg_block_size_kib: *block_size_kib,
                },
            };
            let mut table = HashMap::new();
            let source_index = compute_block_index(config.clone(), source).unwrap();
            for block in source_index.iter() {
                table.insert(source_index.block_hash(block), block);
            }
            let target_index = compute_block_index(config.clone(), target).unwrap();
            let mut download_size = 0;
            for block in target_index.iter() {
                let hash = target_index.block_hash(block);
                if !table.contains_key(hash) {
                    download_size += target_index.block_size(block).raw;
                    table.insert(hash, block);
                }
            }
            println!("Download Size: {download_size}");
        }
        Cmd::FixedDelta {
            source,
            target,
            block_size_kib,
        } => {
            let config = BlockIndexConfig {
                hash_algorithm: si_crypto_hashes::HashAlgorithm::Sha256,
                chunker: rugix_chunker::ChunkerAlgorithm::Fixed {
                    block_size_kib: *block_size_kib,
                },
            };
            let mut table = HashMap::new();
            let source_index = compute_block_index(config.clone(), source).unwrap();
            for block in source_index.iter() {
                table.insert(source_index.block_hash(block), block);
            }
            let target_index = compute_block_index(config.clone(), target).unwrap();
            let mut download_size = 0;
            for block in target_index.iter() {
                let hash = target_index.block_hash(block);
                if !table.contains_key(hash) {
                    download_size += target_index.block_size(block).raw;
                    table.insert(hash, block);
                }
            }
            println!("Download Size: {download_size}");
        }
    }
}

fn compress_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut compressor = rugix_compression::XzEncoder::new(6);
    let mut compressed = Vec::new();
    compressor.process(&bytes, &mut compressed).unwrap();
    compressor.finalize(&mut compressed).unwrap();
    compressed
}

#[derive(Debug, Clone, Default)]
pub struct DeltaStats {
    entries_downloaded: usize,
    data_downloaded: u64,
}

pub fn copy_block(
    expected_block: &FsEntryBlock,
    source: &Path,
    target: &Path,
    stats: &mut DeltaStats,
    config: &BlockIndexConfig,
    source_table: &mut HashSet<si_crypto_hashes::HashDigest>,
) {
    let mut stream = DirStream::new(source);
    if !stream.seek_to(&expected_block.first) {
        todo!("first is beyond the end of the source, trigger full reconstruction");
    }
    let mut chunker = FsChunker::new();
    let copied_block = loop {
        if stream.current_position() > expected_block.last {
            break chunker.flush();
        }
        let path = stream.current_path();
        let Some(entry) = FsEntry::from_path(path, stream.current_position().to_path_buf()) else {
            if stream.next() {
                continue;
            } else {
                break chunker.flush();
            }
        };
        let entry_target = target.join(&entry.position);
        if let Some(parent) = entry_target.parent() {
            std::fs::create_dir_all(parent).unwrap()
        }
        match &entry.kind {
            FsEntryKind::Directory => {
                std::fs::create_dir_all(&entry_target).unwrap();
            }
            FsEntryKind::File { data, .. } => {
                std::fs::write(&entry_target, &data).unwrap();
            }
            FsEntryKind::Symlink { target } => {
                std::os::unix::fs::symlink(target, &entry_target).unwrap();
            }
        }
        if let Some(read_block) = chunker.push(entry) {
            break Some(read_block);
        }
        if !stream.next() {
            break chunker.flush();
        }
    };
    let Some(copied_block) = copied_block else {
        todo!("no block has been copied, reconstruct full block")
    };
    if copied_block.hash != expected_block.hash {
        // We need to download the entries of the expected block.
        stats.entries_downloaded += expected_block.entries.len();
        let expected_entries = &expected_block.entries;
        let copied_entries = &copied_block.entries;
        let mut expected_idx = 0;
        let mut copied_idx = 0;
        while expected_idx < expected_entries.len() || copied_idx < copied_entries.len() {
            if expected_idx < expected_entries.len() && copied_idx < copied_entries.len() {
                let expected_entry = &expected_entries[expected_idx];
                let copied_entry = &copied_entries[copied_idx];
                // Expected entry is missing, materialize.
                let target_path = target.join(&expected_entry.position);
                let copied_path = target.join(&copied_entry.position);
                if let Some(parent) = target_path.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                }
                if expected_entry.position == copied_entry.position {
                    if expected_entry.kind != copied_entry.kind {
                        if target_path.exists() {
                            match &copied_entry.kind {
                                FsEntryKind::Directory => {
                                    std::fs::remove_dir_all(&target_path).unwrap();
                                }
                                FsEntryKind::File { .. } => {
                                    std::fs::remove_file(&target_path).unwrap();
                                }
                                FsEntryKind::Symlink { .. } => {
                                    std::fs::remove_file(&target_path).unwrap();
                                }
                            }
                        }
                        match &expected_entry.kind {
                            FsEntryKind::Directory => {
                                std::fs::create_dir_all(target_path).unwrap();
                            }
                            FsEntryKind::File { data, .. } => {
                                // We need to download the data of the file.
                                for chunk in config.chunker.chunker().unwrap().chunks(data) {
                                    let hash = config.hash_algorithm.hash(chunk);
                                    if source_table.insert(hash) {
                                        stats.data_downloaded += chunk.len() as u64;
                                    }
                                }
                                std::fs::write(&target_path, data).unwrap();
                            }
                            FsEntryKind::Symlink { target } => {
                                std::os::unix::fs::symlink(target, &target_path).unwrap();
                            }
                        }
                    }
                    expected_idx += 1;
                    copied_idx += 1;
                } else if expected_entry.position < copied_entry.position {
                    match &expected_entry.kind {
                        FsEntryKind::Directory => {
                            std::fs::create_dir_all(target_path).unwrap();
                        }
                        FsEntryKind::File { data, .. } => {
                            // We need to download the data of the file.
                            for chunk in config.chunker.chunker().unwrap().chunks(data) {
                                let hash = config.hash_algorithm.hash(chunk);
                                if source_table.insert(hash) {
                                    stats.data_downloaded += chunk.len() as u64;
                                }
                            }
                            std::fs::write(&target_path, data).unwrap();
                        }
                        FsEntryKind::Symlink { target } => {
                            std::os::unix::fs::symlink(target, &target_path).unwrap();
                        }
                    }
                    expected_idx += 1;
                } else {
                    if copied_path.exists() {
                        match &copied_entry.kind {
                            FsEntryKind::Directory => {
                                std::fs::remove_dir_all(&copied_path).unwrap();
                            }
                            FsEntryKind::File { .. } => {
                                std::fs::remove_file(&copied_path).unwrap();
                            }
                            FsEntryKind::Symlink { .. } => {
                                std::fs::remove_file(&copied_path).unwrap();
                            }
                        }
                    }
                    copied_idx += 1;
                }
            } else if expected_idx < expected_entries.len() {
                let expected_entry = &expected_entries[expected_idx];
                // Expected entry is missing, materialize.
                let target_path = target.join(&expected_entry.position);
                if let Some(parent) = target_path.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                }
                match &expected_entry.kind {
                    FsEntryKind::Directory => {
                        std::fs::create_dir_all(target_path).unwrap();
                    }
                    FsEntryKind::File { data, .. } => {
                        // We need to download the data of the file.
                        for chunk in config.chunker.chunker().unwrap().chunks(data) {
                            let hash = config.hash_algorithm.hash(chunk);
                            if source_table.insert(hash) {
                                stats.data_downloaded += chunk.len() as u64;
                            }
                        }
                        std::fs::write(&target_path, data).unwrap();
                    }
                    FsEntryKind::Symlink { target } => {
                        std::os::unix::fs::symlink(target, &target_path).unwrap();
                    }
                }
                expected_idx += 1;
            } else {
                let copied_entry = &copied_entries[copied_idx];
                let copied_path = target.join(&copied_entry.position);
                if copied_path.exists() {
                    match &copied_entry.kind {
                        FsEntryKind::Directory => {
                            std::fs::remove_dir_all(&copied_path).unwrap();
                        }
                        FsEntryKind::File { .. } => {
                            std::fs::remove_file(&copied_path).unwrap();
                        }
                        FsEntryKind::Symlink { .. } => {
                            std::fs::remove_file(&copied_path).unwrap();
                        }
                    }
                }
                copied_idx += 1;
            }
        }
    }
}

pub fn compute_blocks(path: &Path) -> Vec<FsEntryBlock> {
    let mut stream = DirStream::new(path);
    let mut chunker = FsChunker::new();
    loop {
        let path = stream.current_path();
        let Some(entry) = FsEntry::from_path(path, stream.current_position().to_path_buf()) else {
            if stream.next() { continue } else { break }
        };
        chunker.push(entry);
        if !stream.next() {
            break;
        }
    }
    chunker.finalize()
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

pub struct FsChunker {
    divider_threshold: u64,
    max_block_size: usize,
    min_block_size: usize,
    blocks: Vec<FsEntryBlock>,
    pending_entries: Vec<FsEntry>,
    current_hasher: Hasher,
}

impl FsChunker {
    pub fn new() -> Self {
        Self {
            divider_threshold: u64::MAX / 64,
            max_block_size: 128,
            min_block_size: 32,
            blocks: Vec::new(),
            pending_entries: Vec::new(),
            current_hasher: HashAlgorithm::Sha256.hasher(),
        }
    }

    pub fn flush(&mut self) -> Option<&FsEntryBlock> {
        if self.pending_entries.is_empty() {
            return None;
        }
        let first = self.pending_entries.first().unwrap().position.clone();
        let last = self.pending_entries.last().unwrap().position.clone();
        let entries = std::mem::take(&mut self.pending_entries);
        let hash =
            std::mem::replace(&mut self.current_hasher, HashAlgorithm::Sha256.hasher()).finalize();
        self.blocks.push(FsEntryBlock {
            first,
            last,
            hash,
            entries,
        });
        Some(self.blocks.last().unwrap())
    }

    pub fn push(&mut self, entry: FsEntry) -> Option<&FsEntryBlock> {
        let entry_hash = entry.stable_hash();
        self.current_hasher.update(entry_hash.as_ref());
        self.pending_entries.push(entry);
        if self.pending_entries.len() >= self.max_block_size {
            return self.flush();
        } else if self.pending_entries.len() >= self.min_block_size {
            let divider_value = u64::from_be_bytes(entry_hash.as_ref()[..8].try_into().unwrap());
            if divider_value < self.divider_threshold {
                return self.flush();
            }
        }
        None
    }

    pub fn finalize(mut self) -> Vec<FsEntryBlock> {
        self.flush();
        self.blocks
    }
}

#[derive(Debug, Parser)]
#[clap(version = rugix_version::RUGIX_GIT_VERSION)]
pub struct Args {
    #[clap(flatten)]
    logging_args: si_observability::clap4::LoggingArgs,
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    #[clap(subcommand)]
    BlockBased(BlockBasedCmd),
    ComputePlan {
        path: PathBuf,
    },
    CopyPlanned {
        #[clap(long, default_value = "16")]
        casync_block_size: u16,
        #[clap(long, default_value = "32")]
        compressor_block_size: usize,
        original: PathBuf,
        source: PathBuf,
    },
    ComputeBlocks {
        path: PathBuf,
    },
    DeltaCopy {
        original: PathBuf,
        source: PathBuf,
        target: PathBuf,
    },
    Fingerprint {
        path: PathBuf,
    },
    CasyncDelta {
        source: PathBuf,
        target: PathBuf,
        #[clap(default_value = "64")]
        block_size_kib: u16,
    },
    FixedDelta {
        source: PathBuf,
        target: PathBuf,
        #[clap(default_value = "4")]
        block_size_kib: u16,
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
