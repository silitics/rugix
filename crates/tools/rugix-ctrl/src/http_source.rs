use std::io::Read;

use crate::system::SystemResult;
use byte_calc::NumBytes;
use reportify::{bail, ResultExt};
use rugix_bundle::source::BundleSource;
use ureq::http::Response;
use ureq::Body;

pub struct HttpSource {
    url: String,
    supports_range: bool,
    current_response: Response<Body>,
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

impl HttpSource {
    pub fn new(url: &str) -> SystemResult<Self> {
        let response = ureq::get(url)
            .call()
            .whatever("unable to get bundle from URL")?;
        let content_length = response.headers().get("Content-Length").and_then(|length| {
            length
                .to_str()
                .ok()?
                .trim()
                .parse::<u64>()
                .ok()
                .map(NumBytes::new)
        });
        Ok(Self {
            url: url.to_owned(),
            supports_range: response
                .headers()
                .get("Accept-Ranges")
                .map(|value| value.as_bytes() == b"bytes")
                .unwrap_or(false),
            current_response: response,
            current_skipped: 0,
            current_position: 0,
            skip_buffer: Vec::new(),
            bytes_read: 0,
            bytes_skipped: 0,
            total_bytes: content_length,
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
        if self.current_skipped > 0 {
            self.current_position += self.current_skipped;
            if self.current_skipped > NumBytes::kibibytes(32) && self.supports_range {
                self.current_response = ureq::get(&self.url)
                    .header("Range", format!("bytes={}-", self.current_position))
                    .call()
                    .whatever("unable to get bundle from URL")?;
                self.bytes_skipped += self.current_skipped;
            } else {
                self.bytes_read += self.current_skipped;
                let mut remaining = self.current_skipped;
                while remaining > 0 {
                    self.skip_buffer.resize(remaining.min(8192) as usize, 0);
                    let read = self
                        .current_response
                        .body_mut()
                        .as_reader()
                        .read(&mut self.skip_buffer)
                        .whatever("unable to read from HTTP source")?;
                    if read == 0 {
                        bail!("unexpected end of HTTP stream")
                    }
                    remaining -= read as u64;
                }
            }
            self.current_skipped = 0;
        }
        let read = self
            .current_response
            .body_mut()
            .as_reader()
            .read(slice)
            .whatever("unable to read from HTTP source")?;
        self.bytes_read += read as u64;
        self.current_position += read as u64;
        Ok(read)
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
