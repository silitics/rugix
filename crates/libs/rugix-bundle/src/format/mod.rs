//! Low-level implementation of the bundle format and its structures.
//!
//! The bundle format is based on the *STLV encoding* specified and implemented in
//! [`stlv`].

use std::io::{self, Write};

use reportify::{bail, ResultExt};

use rugix_chunker::ChunkerAlgorithm;
use rugix_compression::CompressionFormat;
use si_crypto_hashes::{HashAlgorithm, HashDigest};

use crate::manifest::DeltaEncodingFormat;
use crate::source::BundleSource;
use crate::BundleResult;

use self::decode::{Decode, Decoder};
use self::encode::Encode;
use self::macros::define_struct;
use self::stlv::{write_atom_head, write_value, AtomHead, Tag};

mod macros;

pub mod decode;
pub mod encode;
pub mod stlv;
pub mod tags;

define_struct! {
    /// Bundle header.
    pub struct BundleHeader {
        /// Optional bundle manifest (JSON-encoded).
        pub manifest[BUNDLE_HEADER_MANIFEST]: Option<String>,
        /// Indicates whether the update is incremental.
        pub is_incremental[BUNDLE_HEADER_IS_INCREMENTAL]: bool,
        /// Hash algorithm to secure the bundle.
        pub hash_algorithm[BUNDLE_HEADER_HASH_ALGORITHM]: HashAlgorithm,
        /// Payload index.
        pub payload_index[BUNDLE_HEADER_PAYLOAD_INDEX]: Vec<PayloadEntry>,
    }
}

define_struct! {
    /// Entry in the payload index of a bundle.
    pub struct PayloadEntry {
        /// Slot where the payload should be installed to.
        pub type_slot[PAYLOAD_ENTRY_TYPE_SLOT]: Option<SlotPayloadType>,
        pub type_execute[PAYLOAD_ENTRY_TYPE_EXECUTE]: Option<ExecutePayloadType>,
        /// Hash of the payload header.
        pub header_hash[PAYLOAD_ENTRY_HEADER_HASH]: Bytes,
        /// Hash of the payload file.
        pub file_hash[PAYLOAD_ENTRY_FILE_HASH]: Bytes,
        /// Delta encoding.
        pub delta_encoding[PAYLOAD_ENTRY_DELTA_ENCODING]: Option<DeltaEncoding>,
    }
}

define_struct! {
    pub struct DeltaEncoding {
        pub format[DELTA_ENCODING_FORMAT]: DeltaEncodingFormat,
        pub inputs[DELTA_ENCODING_INPUT]: Vec<DeltaEncodingInput>,
        pub original_hash[DELTA_ENCODING_ORIGINAL_HASH]: HashDigest,
    }
}

define_struct! {
    pub struct DeltaEncodingInput {
        pub hashes[DELTA_ENCODING_INPUT_HASH]: Vec<HashDigest>,
    }
}

impl Decode for HashDigest {
    fn decode<S: BundleSource>(decoder: &mut Decoder<S>, atom: AtomHead) -> BundleResult<Self> {
        String::decode(decoder, atom)?
            .parse()
            .whatever("invalid hash digest")
    }
}

impl Encode for HashDigest {
    fn encode(&self, writer: &mut dyn Write, tag: Tag) -> io::Result<()> {
        write_value(writer, tag, self.to_string().as_bytes())
    }
}

define_struct! {
    /// Header of a payload.
    pub struct SlotPayloadType {
        pub slot[PAYLOAD_TYPE_SLOT_SLOT]: String,
    }
}

define_struct! {
    /// Header of a payload.
    pub struct ExecutePayloadType {
        pub handler[PAYLOAD_TYPE_EXECUTE_HANDLER]: Vec<String>,
    }
}

define_struct! {
    /// Header of a payload.
    pub struct PayloadHeader {
        /// Block encoding.
        pub block_encoding[PAYLOAD_HEADER_BLOCK_ENCODING]: Option<BlockEncoding>,
    }
}

define_struct! {
    /// Signatures.
    pub struct Signatures {
        /// Embedded CMS signatures.
        pub cms_signatures[SIGNATURES_CMS_SIGNATURE]: Vec<Bytes>,
    }
}

impl Default for Signatures {
    fn default() -> Self {
        Self {
            cms_signatures: Default::default(),
        }
    }
}

define_struct! {
    /// Payload block encoding.
    pub struct BlockEncoding {
        /// Chunker used for the encoding.
        pub chunker[BLOCK_ENCODING_CHUNKER]: ChunkerAlgorithm,
        /// Hash algorithm.
        pub hash_algorithm[BLOCK_ENCODING_HASH_ALGORITHM]: HashAlgorithm,
        /// Whether blocks have been deduplicated.
        pub deduplicated[BLOCK_ENCODING_DEDUPLICATED]: bool,
        pub compression[BLOCK_ENCODING_COMPRESSION]: Option<CompressionFormat>,
        /// Block index.
        pub block_hashes[BLOCK_ENCODING_BLOCK_HASHES]: Bytes,
        /// Block sizes.
        pub block_sizes[BLOCK_ENCODING_BLOCK_SIZES]: Option<Bytes>,
    }
}

define_struct! {
    pub struct BlockIndex {
        pub chunker[BLOCK_INDEX_CHUNKER]: ChunkerAlgorithm,
        pub hash_algorithm[BLOCK_INDEX_HASH_ALGORITHM]: HashAlgorithm,
        pub block_hashes[BLOCK_INDEX_BLOCK_HASHES]: Bytes,
        pub block_sizes[BLOCK_INDEX_BLOCK_SIZES]: Bytes,
    }
}

define_struct! {
    pub struct XzCompression {}
}

define_struct! {
    pub struct SignedMetadata {
        pub header_hash[SIGNED_METADATA_HEADER_HASH]: HashDigest,
    }
}

/// Encodable and decodable bytes.
#[derive(Debug, Clone)]
pub struct Bytes {
    /// Raw byte vector.
    pub raw: Vec<u8>,
}

impl Encode for Bytes {
    fn encode(&self, writer: &mut dyn Write, tag: Tag) -> io::Result<()> {
        write_value(writer, tag, &self.raw)
    }
}

impl Decode for Bytes {
    fn decode<S: BundleSource>(decoder: &mut Decoder<S>, atom: AtomHead) -> BundleResult<Self> {
        if !atom.is_value() {
            bail!("cannot decode `Bytes` from segment");
        }
        Ok(Self {
            raw: decoder.read_value()?,
        })
    }
}

impl Encode for HashAlgorithm {
    fn encode(&self, writer: &mut dyn Write, tag: Tag) -> io::Result<()> {
        write_value(
            writer,
            tag,
            match self {
                HashAlgorithm::Sha512_256 => "sha512-256".as_bytes(),
                _ => self.name().as_bytes(),
            },
        )
    }
}

impl Decode for HashAlgorithm {
    fn decode<S: BundleSource>(decoder: &mut Decoder<S>, atom: AtomHead) -> BundleResult<Self> {
        String::decode(decoder, atom)?
            .parse()
            .whatever("unknown hash algorithm")
    }
}

impl Encode for DeltaEncodingFormat {
    fn encode(&self, writer: &mut dyn Write, tag: Tag) -> io::Result<()> {
        write_value(
            writer,
            tag,
            match self {
                DeltaEncodingFormat::Xdelta => b"xdelta",
            },
        )
    }
}

impl Decode for DeltaEncodingFormat {
    fn decode<S: BundleSource>(decoder: &mut Decoder<S>, atom: AtomHead) -> BundleResult<Self> {
        match String::decode(decoder, atom)?.as_str() {
            "xdelta" => Ok(Self::Xdelta),
            format => bail!("unknown delta encoding format '{format}'"),
        }
    }
}

impl Encode for ChunkerAlgorithm {
    fn encode(&self, writer: &mut dyn Write, tag: Tag) -> io::Result<()> {
        write_value(writer, tag, self.to_string().as_bytes())
    }
}

impl Decode for ChunkerAlgorithm {
    fn decode<S: BundleSource>(decoder: &mut Decoder<S>, atom: AtomHead) -> BundleResult<Self> {
        String::decode(decoder, atom)?
            .parse()
            .whatever("unknown chunker algorithm")
    }
}

impl Encode for CompressionFormat {
    fn encode(&self, writer: &mut dyn Write, tag: Tag) -> io::Result<()> {
        write_value(writer, tag, self.as_str().as_bytes())
    }
}

impl Decode for CompressionFormat {
    fn decode<S: BundleSource>(decoder: &mut Decoder<S>, atom: AtomHead) -> BundleResult<Self> {
        String::decode(decoder, atom)?
            .parse()
            .whatever("unknown compression format")
    }
}
