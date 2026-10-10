//! The Rust half of DuckDB's C++ extension.

#[allow(
    dead_code,
    reason = "the engine registry retains helpers used by the retired raw C API"
)]
mod engines;

#[path = "../../../sqlite/src/complete_native/mod.rs"]
mod complete_native;
mod errors;
mod ffi;

#[allow(
    dead_code,
    reason = "the C++ bridge reuses the shipped signal handler without relate's pipe"
)]
mod signal;

#[path = "usage/ffi.rs"]
mod usage_ffi;

#[path = "relate/ffi.rs"]
mod relate_ffi;
