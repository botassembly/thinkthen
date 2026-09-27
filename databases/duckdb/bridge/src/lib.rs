//! The Rust half of DuckDB's C++ extension.

#[allow(
    dead_code,
    reason = "the bridge shares the registry while the other verbs move"
)]
#[path = "../../src/engines.rs"]
mod engines;

mod errors;
mod ffi;

#[allow(
    dead_code,
    reason = "the C++ bridge reuses the shipped signal handler without relate's pipe"
)]
#[path = "../../src/signal.rs"]
mod signal;

#[path = "usage/ffi.rs"]
mod usage_ffi;

#[path = "warm/ffi.rs"]
mod warm_ffi;

#[path = "relate/ffi.rs"]
mod relate_ffi;
