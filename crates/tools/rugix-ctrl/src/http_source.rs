use std::io::Read;

use crate::system::SystemResult;
use byte_calc::NumBytes;
use reportify::{bail, ResultExt};
use rugix_bundle::source::BundleSource;
use tracing::error;
use ureq::http::Response;
use ureq::Body;

pub struct HttpSource {
    url: String,
    use_range_queries: bool,
    current_response: Option<Response<Body>>,
    content_length: Option<u64>,
    current_end: Option<u64>,
    next_chunk_end: Option<u64>,
    current_position: u64,
    current_skipped: u64,
    skip_buffer: Vec<u8>,
    bytes_read: u64,
    bytes_skipped: u64,
    total_bytes: Option<NumBytes>,
}

#[derive(Debug, Clone)]
pub struct DownloadStats {
    pub bytes_read: NumBytes,
    pub bytes_skipped: NumBytes,
}

impl DownloadStats {
    pub fn total_bytes(&self) -> NumBytes {
        self.bytes_read + self.bytes_skipped
    }

    pub fn download_ratio(&self) -> f64 {
        self.bytes_read.raw as f64 / self.total_bytes().raw as f64
    }
}

/// Minimal size of chunks to be fetched individually via range queries.
const MIN_CHUNK_SIZE: NumBytes = NumBytes::kibibytes(64);

impl HttpSource {
    pub fn new(url: &str, use_range_queries: bool) -> SystemResult<Self> {
        // Try a request with a `Range` header and look at the response to determine the
        // content length and whether range requests are supported by the server.
        let (mut content_length, use_range_queries) = if use_range_queries {
            ureq::get(url)
                .header("Range", "bytes=0-0")
                .call()
                .ok()
                .and_then(|response| {
                    response.headers().get("Content-Range").and_then(|range| {
                        range
                            .to_str()
                            .ok()?
                            .rsplit_once("/")?
                            .1
                            .trim()
                            .parse::<u64>()
                            .ok()
                    })
                })
                .map(|length| (Some(length), true))
                .unwrap_or_default()
        } else {
            (None, false)
        };

        let (current_response, current_end) = if !use_range_queries {
            // Fetch the whole bundle all at once.
            let response = ureq::get(url)
                .call()
                .whatever("unable to get bundle from URL")?;
            content_length = response
                .headers()
                .get("Content-Length")
                .and_then(|length| length.to_str().ok()?.trim().parse::<u64>().ok());
            (response, None)
        } else {
            // Fetch a first chunk of the bundle.
            let first_chunk_size = MIN_CHUNK_SIZE.raw.min(content_length.unwrap());
            (
                ureq::get(url)
                    .header("Range", format!("bytes=0-{}", first_chunk_size - 1))
                    .call()
                    .whatever("unable to get bundle from URL")?,
                Some(first_chunk_size),
            )
        };
        Ok(Self {
            url: url.to_owned(),
            use_range_queries,
            content_length,
            current_response: Some(current_response),
            current_end,
            next_chunk_end: None,
            current_skipped: 0,
            current_position: 0,
            skip_buffer: Vec::new(),
            bytes_read: 0,
            bytes_skipped: 0,
            total_bytes: content_length.map(|l| l.into()),
        })
    }
}

impl HttpSource {
    pub fn get_download_stats(&self) -> DownloadStats {
        DownloadStats {
            bytes_read: NumBytes::new(self.bytes_read),
            bytes_skipped: NumBytes::new(self.bytes_skipped),
        }
    }
}

impl BundleSource for HttpSource {
    fn read(&mut self, slice: &mut [u8]) -> rugix_bundle::BundleResult<usize> {
        assert_ne!(slice.len(), 0, "slice must not be empty");
        if self.current_skipped > 0 {
            self.current_position += self.current_skipped;
            // Check whether we exceeded the chunk that we are currently reading. Note
            // that the end position is itself not included in the chunk.
            let chunk_exceeded = self
                .current_end
                .map(|end| self.current_position >= end)
                .unwrap_or(false);
            if chunk_exceeded {
                // We have exceeded the chunk and need to fetch a new one.
                self.current_response = None;
                let actually_skipped = self.current_position - self.current_end.unwrap();
                // Count the bytes that were still in the pending request as read.
                self.bytes_skipped += actually_skipped;
                self.bytes_read += self.current_skipped - actually_skipped;
            } else {
                // We are still within the chunk and need to skip the bytes by reading.
                if let Some(current_response) = self.current_response.as_mut() {
                    // Read the bytes that we skip from the current response.
                    let mut remaining = self.current_skipped;
                    while remaining > 0 {
                        self.skip_buffer.resize(remaining.min(8192) as usize, 0);
                        let read = current_response
                            .body_mut()
                            .as_reader()
                            .read(&mut self.skip_buffer)
                            .whatever("unable to read from HTTP source")?;
                        if read == 0 {
                            error!("unexpected end of HTTP stream");
                            self.current_response = None;
                            break;
                        }
                        remaining -= read as u64;
                    }
                    self.bytes_read += self.current_skipped;
                } else {
                    self.bytes_skipped += self.current_skipped;
                }
            }
            self.current_skipped = 0;
        }
        loop {
            if self.current_response.is_none() {
                // We need to issue a new request for a new chunk.
                if !self.use_range_queries {
                    bail!("response is not available but range queries are not supported");
                }
                // Compute the end of the next chunk using the provided hint, if any.
                let next_end = (self.current_position + MIN_CHUNK_SIZE.raw.max(slice.len() as u64))
                    .max(self.next_chunk_end.unwrap_or(0))
                    .min(self.content_length.unwrap());
                self.next_chunk_end = None;
                if self.current_position == next_end {
                    assert_eq!(self.current_position, self.content_length.unwrap());
                    // We reached the end of the update bundle, return `0`.
                    return Ok(0);
                }
                assert!(self.current_position < next_end);
                self.current_response = Some(
                    // HTTP range headers are inclusive, hence, we need to subtract one.
                    ureq::get(&self.url)
                        .header(
                            "Range",
                            format!("bytes={}-{}", self.current_position, next_end - 1),
                        )
                        .call()
                        .whatever("unable to get bundle from URL")?,
                );
                self.current_end = Some(next_end);
            }
            let current_response = self.current_response.as_mut().unwrap();
            // We should now be able to read some bytes from the current response.
            let read = current_response
                .body_mut()
                .as_reader()
                .read(slice)
                .whatever("unable to read from HTTP source")?;
            if read == 0 {
                // We reached the end of the response. Do a followup request.
                self.current_response = None;
                continue;
            }
            self.bytes_read += read as u64;
            self.current_position += read as u64;
            break Ok(read);
        }
    }

    fn hint_next_chunk(&mut self, length: NumBytes) {
        if length.raw != 0 {
            self.next_chunk_end = Some(self.current_position + length.raw);
        }
    }

    fn skip(&mut self, length: byte_calc::NumBytes) -> rugix_bundle::BundleResult<()> {
        self.current_skipped += length.raw;
        Ok(())
    }

    fn bytes_read(&self) -> Option<NumBytes> {
        Some(NumBytes::new(self.bytes_read + self.bytes_skipped))
    }

    fn bytes_total(&self) -> Option<NumBytes> {
        self.total_bytes
    }
}
