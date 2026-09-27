//! The Rust half of DuckDB's C++ extension.

#[allow(
    dead_code,
    reason = "the bridge shares the registry while the other verbs move"
)]
#[path = "../../src/engines.rs"]
mod engines;

mod errors;
mod ffi;
