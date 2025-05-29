//! Rugix version information.

/// Rugix Git version string.
pub const RUGIX_GIT_VERSION: &str = match option_env!("RUGIX_GIT_VERSION") {
    Some(version) => version,
    None => "unknown",
};
