//! File transfer module with compression and multi-threaded transfer
//! Uses LZ4 for fast compression and parallel chunk transfer

use anyhow::{anyhow, Result};
use crossbeam_channel::{bounded, Receiver, Sender};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use xxhash_rust::xxh3::xxh3_64;

/// Default chunk size for transfer (1MB)
pub const CHUNK_SIZE: usize = 1024 * 1024;

/// Number of parallel compression threads
pub const COMPRESSION_THREADS: usize = 4;

/// File metadata for transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub name: String,
    pub relative_path: String,
    pub size: u64,
    pub is_dir: bool,
}

/// Transfer progress tracking
#[derive(Debug, Clone)]
pub struct TransferProgress {
    pub total_bytes: u64,
    pub transferred_bytes: Arc<AtomicU64>,
    pub current_file: Arc<std::sync::RwLock<String>>,
    pub file_count: usize,
    pub completed_files: Arc<AtomicU64>,
    pub cancelled: Arc<AtomicBool>,
    pub start_time: std::time::Instant,
}

impl TransferProgress {
    pub fn new(total_bytes: u64, file_count: usize) -> Self {
        Self {
            total_bytes,
            transferred_bytes: Arc::new(AtomicU64::new(0)),
            current_file: Arc::new(std::sync::RwLock::new(String::new())),
            file_count,
            completed_files: Arc::new(AtomicU64::new(0)),
            cancelled: Arc::new(AtomicBool::new(false)),
            start_time: std::time::Instant::now(),
        }
    }

    pub fn progress_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            return 100.0;
        }
        (self.transferred_bytes.load(Ordering::Relaxed) as f32 / self.total_bytes as f32) * 100.0
    }

    pub fn bytes_per_second(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed == 0.0 {
            return 0.0;
        }
        self.transferred_bytes.load(Ordering::Relaxed) as f64 / elapsed
    }

    pub fn estimated_remaining_seconds(&self) -> u64 {
        let speed = self.bytes_per_second();
        if speed == 0.0 {
            return 0;
        }
        let remaining_bytes = self.total_bytes - self.transferred_bytes.load(Ordering::Relaxed);
        (remaining_bytes as f64 / speed) as u64
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    pub fn add_transferred(&self, bytes: u64) {
        self.transferred_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn set_current_file(&self, name: &str) {
        if let Ok(mut current) = self.current_file.write() {
            *current = name.to_string();
        }
    }

    pub fn increment_completed_files(&self) {
        self.completed_files.fetch_add(1, Ordering::Relaxed);
    }
}

/// Compressed chunk for transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedChunk {
    pub index: u64,
    pub original_size: usize,
    pub compressed_data: Vec<u8>,
    pub hash: u64,
}

/// Compress data using LZ4 (fast compression)
pub fn compress_data(data: &[u8]) -> Vec<u8> {
    compress_prepend_size(data)
}

/// Decompress data using LZ4
pub fn decompress_data(compressed: &[u8]) -> Result<Vec<u8>> {
    decompress_size_prepended(compressed).map_err(|e| anyhow!("Decompression failed: {}", e))
}

/// Calculate XXH3 hash of data
pub fn calculate_hash(data: &[u8]) -> u64 {
    xxh3_64(data)
}

/// Compress chunks in parallel
pub fn compress_chunks_parallel(chunks: Vec<Vec<u8>>) -> Vec<CompressedChunk> {
    chunks
        .into_par_iter()
        .enumerate()
        .map(|(index, data)| {
            let hash = calculate_hash(&data);
            let original_size = data.len();
            let compressed_data = compress_data(&data);
            CompressedChunk {
                index: index as u64,
                original_size,
                compressed_data,
                hash,
            }
        })
        .collect()
}

/// Read file and split into chunks
pub fn read_file_chunks(path: &Path, chunk_size: usize) -> Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let file_size = file.metadata()?.len() as usize;
    let mut reader = BufReader::with_capacity(chunk_size, file);
    let mut chunks = Vec::with_capacity((file_size / chunk_size) + 1);

    loop {
        let mut chunk = vec![0u8; chunk_size];
        let bytes_read = reader.read(&mut chunk)?;
        if bytes_read == 0 {
            break;
        }
        chunk.truncate(bytes_read);
        chunks.push(chunk);
    }

    Ok(chunks)
}

/// Write chunks to file
pub fn write_chunks_to_file(path: &Path, chunks: &[Vec<u8>]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(CHUNK_SIZE, file);

    for chunk in chunks {
        writer.write_all(chunk)?;
    }
    writer.flush()?;

    Ok(())
}

/// File entry for transfer
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub relative_path: String,
    pub size: u64,
    pub is_dir: bool,
}

/// Collect all files from a list of paths (handles directories recursively)
pub fn collect_files(paths: &[PathBuf]) -> Result<Vec<FileEntry>> {
    let mut entries = Vec::new();

    for path in paths {
        if path.is_dir() {
            collect_directory_files(path, path, &mut entries)?;
        } else if path.is_file() {
            let size = path.metadata()?.len();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            entries.push(FileEntry {
                path: path.clone(),
                relative_path: name,
                size,
                is_dir: false,
            });
        }
    }

    Ok(entries)
}

/// Recursively collect files from a directory
fn collect_directory_files(
    base_path: &Path,
    current_path: &Path,
    entries: &mut Vec<FileEntry>,
) -> Result<()> {
    let _base_name = base_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        let relative = path
            .strip_prefix(base_path.parent().unwrap_or(base_path))
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        if path.is_dir() {
            entries.push(FileEntry {
                path: path.clone(),
                relative_path: relative.clone(),
                size: 0,
                is_dir: true,
            });
            collect_directory_files(base_path, &path, entries)?;
        } else {
            let size = path.metadata()?.len();
            entries.push(FileEntry {
                path: path.clone(),
                relative_path: relative,
                size,
                is_dir: false,
            });
        }
    }

    Ok(())
}

/// Calculate total size of files
pub fn calculate_total_size(files: &[FileEntry]) -> u64 {
    files.iter().filter(|f| !f.is_dir).map(|f| f.size).sum()
}

/// Producer-consumer pattern for parallel file reading and compression
pub struct FileProcessor {
    chunk_sender: Sender<CompressedChunk>,
    chunk_receiver: Receiver<CompressedChunk>,
    cancelled: Arc<AtomicBool>,
}

impl FileProcessor {
    pub fn new(buffer_size: usize) -> Self {
        let (chunk_sender, chunk_receiver) = bounded(buffer_size);
        Self {
            chunk_sender,
            chunk_receiver,
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Process a file: read, chunk, compress in parallel
    pub fn process_file(
        &self,
        path: &Path,
        progress: &TransferProgress,
    ) -> Result<Vec<CompressedChunk>> {
        let chunks = read_file_chunks(path, CHUNK_SIZE)?;
        let _total_chunks = chunks.len();

        // Compress chunks in parallel
        let compressed: Vec<CompressedChunk> = chunks
            .into_par_iter()
            .enumerate()
            .map(|(index, data)| {
                if self.cancelled.load(Ordering::Relaxed) {
                    return CompressedChunk {
                        index: index as u64,
                        original_size: 0,
                        compressed_data: Vec::new(),
                        hash: 0,
                    };
                }

                let hash = calculate_hash(&data);
                let original_size = data.len();
                let compressed_data = compress_data(&data);

                progress.add_transferred(original_size as u64);

                CompressedChunk {
                    index: index as u64,
                    original_size,
                    compressed_data,
                    hash,
                }
            })
            .collect();

        Ok(compressed)
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
}

/// Decompressor for received chunks
pub struct ChunkDecompressor;

impl ChunkDecompressor {
    /// Decompress chunks in parallel and reassemble
    pub fn decompress_and_write(
        chunks: Vec<CompressedChunk>,
        output_path: &Path,
        progress: &TransferProgress,
    ) -> Result<()> {
        // Sort chunks by index
        let mut sorted_chunks = chunks;
        sorted_chunks.sort_by_key(|c| c.index);

        // Decompress in parallel
        let decompressed: Result<Vec<(u64, Vec<u8>)>> = sorted_chunks
            .into_par_iter()
            .map(|chunk| {
                if progress.is_cancelled() {
                    return Err(anyhow!("Transfer cancelled"));
                }

                let data = decompress_data(&chunk.compressed_data)?;

                // Verify hash
                let hash = calculate_hash(&data);
                if hash != chunk.hash {
                    return Err(anyhow!("Hash mismatch for chunk {}", chunk.index));
                }

                progress.add_transferred(data.len() as u64);

                Ok((chunk.index, data))
            })
            .collect();

        let mut decompressed = decompressed?;
        decompressed.sort_by_key(|(idx, _)| *idx);

        // Write to file
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = File::create(output_path)?;
        let mut writer = BufWriter::with_capacity(CHUNK_SIZE, file);

        for (_, data) in decompressed {
            writer.write_all(&data)?;
        }
        writer.flush()?;

        Ok(())
    }
}

/// Streaming compressor for large files
pub struct StreamingCompressor {
    chunk_size: usize,
}

impl StreamingCompressor {
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    /// Compress a file and return an iterator over compressed chunks
    pub fn compress_file<'a>(
        &'a self,
        path: &Path,
        progress: &'a TransferProgress,
    ) -> Result<impl Iterator<Item = Result<CompressedChunk>> + 'a> {
        let file = File::open(path)?;
        let reader = BufReader::with_capacity(self.chunk_size, file);

        Ok(ChunkIterator {
            reader,
            chunk_size: self.chunk_size,
            index: 0,
            progress,
            done: false,
        })
    }
}

struct ChunkIterator<'a, R: Read> {
    reader: BufReader<R>,
    chunk_size: usize,
    index: u64,
    progress: &'a TransferProgress,
    done: bool,
}

impl<'a, R: Read> Iterator for ChunkIterator<'a, R> {
    type Item = Result<CompressedChunk>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done || self.progress.is_cancelled() {
            return None;
        }

        let mut chunk = vec![0u8; self.chunk_size];
        match self.reader.read(&mut chunk) {
            Ok(0) => {
                self.done = true;
                None
            }
            Ok(bytes_read) => {
                chunk.truncate(bytes_read);
                let hash = calculate_hash(&chunk);
                let original_size = chunk.len();
                let compressed_data = compress_data(&chunk);

                let result = CompressedChunk {
                    index: self.index,
                    original_size,
                    compressed_data,
                    hash,
                };
                self.index += 1;
                Some(Ok(result))
            }
            Err(e) => {
                self.done = true;
                Some(Err(e.into()))
            }
        }
    }
}

/// Batch processor for multiple files
pub struct BatchTransfer {
    files: Vec<FileEntry>,
    progress: TransferProgress,
}

impl BatchTransfer {
    pub fn new(files: Vec<FileEntry>) -> Self {
        let total_size = calculate_total_size(&files);
        let file_count = files.iter().filter(|f| !f.is_dir).count();
        let progress = TransferProgress::new(total_size, file_count);

        Self { files, progress }
    }

    pub fn progress(&self) -> &TransferProgress {
        &self.progress
    }

    pub fn files(&self) -> &[FileEntry] {
        &self.files
    }

    pub fn total_size(&self) -> u64 {
        self.progress.total_bytes
    }

    pub fn file_count(&self) -> usize {
        self.progress.file_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_compress_decompress() {
        let data = b"Hello, World! This is a test of compression.";
        let compressed = compress_data(data);
        let decompressed = decompress_data(&compressed).unwrap();
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[test]
    fn test_hash() {
        let data = b"Test data for hashing";
        let hash1 = calculate_hash(data);
        let hash2 = calculate_hash(data);
        assert_eq!(hash1, hash2);

        let different_data = b"Different data";
        let hash3 = calculate_hash(different_data);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_parallel_compression() {
        let chunks: Vec<Vec<u8>> = (0..10)
            .map(|i| format!("Chunk number {}", i).into_bytes())
            .collect();

        let compressed = compress_chunks_parallel(chunks.clone());
        assert_eq!(compressed.len(), 10);

        // Verify each chunk
        for (i, comp) in compressed.iter().enumerate() {
            assert_eq!(comp.index, i as u64);
            let decompressed = decompress_data(&comp.compressed_data).unwrap();
            assert_eq!(decompressed, chunks[i]);
        }
    }

    #[test]
    fn test_file_chunks() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        // Create a test file
        let mut file = File::create(&file_path).unwrap();
        let data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
        file.write_all(&data).unwrap();

        // Read chunks
        let chunks = read_file_chunks(&file_path, 1000).unwrap();
        assert_eq!(chunks.len(), 10);

        // Verify content
        let reassembled: Vec<u8> = chunks.into_iter().flatten().collect();
        assert_eq!(reassembled, data);
    }
}
