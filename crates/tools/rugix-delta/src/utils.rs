//! Utility functions.

use rugix_compression::ByteProcessor;

/// Compress the given bytes.
pub fn compress_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut compressor = rugix_compression::XzEncoder::new(6);
    let mut compressed = Vec::new();
    compressor.process(&bytes, &mut compressed).unwrap();
    compressor.finalize(&mut compressed).unwrap();
    compressed
}
