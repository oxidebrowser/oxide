//! # oxide-forge
//!
//! A Rust-based WebAssembly runtime for vibecoders.
//!
//! This crate exposes the building blocks used by the `oxide-forge` CLI
//! and can also be embedded directly into other Rust applications.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// The core runtime handle.
///
/// This is intentionally minimal for now — it will grow to host a
/// WebAssembly module loader, a linker, and an executor.
#[derive(Debug, Default, Clone)]
pub struct Runtime {
    _private: (),
}

impl Runtime {
    /// Create a new runtime with default configuration.
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Human-readable name of the runtime.
    pub const fn name() -> &'static str {
        "oxide-forge"
    }

    /// Version string, sourced from `Cargo.toml` at compile time.
    pub fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_reports_name() {
        assert_eq!(Runtime::name(), "oxide-forge");
    }

    #[test]
    fn runtime_reports_version() {
        let rt = Runtime::new();
        assert!(!rt.version().is_empty());
    }
}
