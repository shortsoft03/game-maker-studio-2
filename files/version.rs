//! Static build info for Game Maker Studio 2.

pub const BUILD_VERSION: &str  = "v3.2.6";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "6a3e06465ee4";
pub const BUILD_TIMESTAMP: u64 = 1730372040;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Game Maker Studio 2", BUILD_VERSION, BUILD_CHANNEL)
}
