//! Application use cases — orchestration layer (implemented in later phases).

/// Crate version for health/diagnostic stubs.
pub fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
