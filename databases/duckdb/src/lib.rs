//! The DuckDB extension: SQL functions over the `thinkthen` engine
//! (tickets 0110 and 0118, ADR 0047).
//!
//! `ffi` holds every call into DuckDB's C API. The other modules are safe
//! Rust over `thinkthen`'s public API.

mod engines;
mod errors;
mod ffi;
mod questions;
mod scalars;
mod signal;
mod tables;
mod worker;
