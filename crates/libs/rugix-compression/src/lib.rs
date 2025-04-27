//! Functionality for streaming compression.

use std::error::Error;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompressionFormat {
    Xz,
    XzMt,
}

impl CompressionFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            CompressionFormat::Xz => "xz",
            CompressionFormat::XzMt => "xz-mt",
        }
    }
}

impl std::str::FromStr for CompressionFormat {
    type Err = InvalidCompressionFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "xz" => Ok(Self::Xz),
            "xz-mt" => Ok(Self::XzMt),
            _ => Err(InvalidCompressionFormatError {}),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct InvalidCompressionFormatError {}

impl std::fmt::Display for InvalidCompressionFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("compression format is not valid")
    }
}

impl Error for InvalidCompressionFormatError {}

pub trait ByteProcessor {
    type Output;

    fn process(&mut self, input: &[u8], output: &mut dyn Write) -> std::io::Result<()>;

    fn finalize(self, output: &mut dyn Write) -> std::io::Result<Self::Output>;
}

pub struct IdentityTransducer;

impl ByteProcessor for IdentityTransducer {
    type Output = ();

    fn process(&mut self, input: &[u8], output: &mut dyn Write) -> std::io::Result<()> {
        output.write_all(input)
    }

    fn finalize(self, _: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct XzEncoder {
    buffer: Vec<u8>,
    stream: xz2::stream::Stream,
}

impl XzEncoder {
    pub fn new(level: u8) -> Self {
        assert!(level <= 9, "compression level must be between 0 and 9");
        let stream = xz2::stream::Stream::new_easy_encoder(level as u32, xz2::stream::Check::Crc64)
            .expect("options should be valid");
        Self {
            buffer: Vec::with_capacity(32 * 1024),
            stream,
        }
    }

    fn flush_buffer(&mut self, output: &mut dyn Write) -> std::io::Result<()> {
        if !self.buffer.is_empty() {
            output.write_all(&self.buffer)?;
            self.buffer.clear();
        }
        Ok(())
    }

    fn feed_stream(
        &mut self,
        input: &[u8],
        action: xz2::stream::Action,
    ) -> std::io::Result<xz2::stream::Status> {
        self.stream
            .process_vec(input, &mut self.buffer, action)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
    }
}

impl ByteProcessor for XzEncoder {
    type Output = ();

    fn process(&mut self, mut input: &[u8], output: &mut dyn Write) -> std::io::Result<()> {
        while !input.is_empty() {
            self.flush_buffer(output)?;
            let total_in = self.stream.total_in();
            self.feed_stream(input, xz2::stream::Action::Run)?;
            let written = self.stream.total_in() - total_in;
            input = &input[written as usize..];
        }
        Ok(())
    }

    fn finalize(mut self, output: &mut dyn Write) -> std::io::Result<()> {
        loop {
            self.flush_buffer(output)?;
            match self.feed_stream(&[], xz2::stream::Action::Finish)? {
                xz2::stream::Status::StreamEnd => break,
                _ => { /* nothing to do */ }
            }
        }
        self.flush_buffer(output)?;
        Ok(())
    }
}

pub struct XzMtEncoder {
    buffer: Vec<u8>,
    stream: xz2::stream::Stream,
}

impl XzMtEncoder {
    pub fn new(level: u8) -> Self {
        assert!(level <= 9, "compression level must be between 0 and 9");
        let num_threads = num_cpus::get().min(4) as u32;
        let stream = xz2::stream::MtStreamBuilder::new()
            .threads(num_threads)
            .preset(level as u32)
            .check(xz2::stream::Check::Crc64)
            .encoder()
            .expect("options should be valid");
        
        Self {
            buffer: Vec::with_capacity(32 * 1024),
            stream,
        }
    }

    fn flush_buffer(&mut self, output: &mut dyn Write) -> std::io::Result<()> {
        if !self.buffer.is_empty() {
            output.write_all(&self.buffer)?;
            self.buffer.clear();
        }
        Ok(())
    }

    fn feed_stream(
        &mut self,
        input: &[u8],
        action: xz2::stream::Action,
    ) -> std::io::Result<xz2::stream::Status> {
        self.stream
            .process_vec(input, &mut self.buffer, action)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
    }
}

impl ByteProcessor for XzMtEncoder {
    type Output = ();

    fn process(&mut self, mut input: &[u8], output: &mut dyn Write) -> std::io::Result<()> {
        while !input.is_empty() {
            self.flush_buffer(output)?;
            let total_in = self.stream.total_in();
            self.feed_stream(input, xz2::stream::Action::Run)?;
            let written = self.stream.total_in() - total_in;
            input = &input[written as usize..];
        }
        Ok(())
    }

    fn finalize(mut self, output: &mut dyn Write) -> std::io::Result<()> {
        loop {
            self.flush_buffer(output)?;
            match self.feed_stream(&[], xz2::stream::Action::Finish)? {
                xz2::stream::Status::StreamEnd => break,
                _ => { /* nothing to do */ }
            }
        }
        self.flush_buffer(output)?;
        Ok(())
    }
}

pub struct XzDecoder {
    buffer: Vec<u8>,
    stream: xz2::stream::Stream,
}

impl XzDecoder {
    pub fn new() -> Self {
        let stream =
            xz2::stream::Stream::new_stream_decoder(u64::MAX, 0).expect("options should be valid");
        Self {
            buffer: Vec::with_capacity(32 * 1024),
            stream,
        }
    }

    fn flush_buffer(&mut self, output: &mut dyn Write) -> std::io::Result<()> {
        if !self.buffer.is_empty() {
            output.write_all(&self.buffer)?;
            self.buffer.clear();
        }
        Ok(())
    }

    fn feed_stream(
        &mut self,
        input: &[u8],
        action: xz2::stream::Action,
    ) -> std::io::Result<xz2::stream::Status> {
        self.stream
            .process_vec(input, &mut self.buffer, action)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
    }
}

impl ByteProcessor for XzDecoder {
    type Output = ();

    fn process(&mut self, mut input: &[u8], output: &mut dyn Write) -> std::io::Result<()> {
        while !input.is_empty() {
            self.flush_buffer(output)?;
            let total_in = self.stream.total_in();
            match self.feed_stream(input, xz2::stream::Action::Run)? {
                xz2::stream::Status::StreamEnd => break,
                _ => { /* do nothing */ }
            }
            let written = self.stream.total_in() - total_in;
            input = &input[written as usize..];
        }
        Ok(())
    }

    fn finalize(mut self, output: &mut dyn Write) -> std::io::Result<()> {
        loop {
            self.flush_buffer(output)?;
            match self.feed_stream(&[], xz2::stream::Action::Finish)? {
                xz2::stream::Status::StreamEnd => break,
                _ => { /* nothing to do */ }
            }
        }
        self.flush_buffer(output)?;
        Ok(())
    }
}
