use std::collections::HashMap;

use byte_calc::NumBytes;
use si_crypto_hashes::{HashAlgorithm, HashDigest};

/// Slot state.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SlotState {
    /// Hashes of the slot's contents.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub hashes: HashMap<HashAlgorithm, HashDigest>,
    /// Size of the slot's contents (well-defined part of the block device).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<NumBytes>,
    /// Timestamp when the slot was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<jiff::Timestamp>,
}
