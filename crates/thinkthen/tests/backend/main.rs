//! The compiled binary against a loopback backend, as one test binary.
//!
//! The three pages below share `harness`, and one binary compiles it once. A
//! module split over several test binaries would leave part of the harness
//! unused in each one, and `rust-standards.md` lets no test file paste a
//! suppression at its top.

mod harness;

mod address;
mod annotate;
mod asked;
mod cache_locking;
mod choosing;
mod distribution_total;
mod exchange;
mod from_record;
mod json_syntax;
mod keeping;
mod limits;
mod parallel;
mod recording_conflicts;
mod recordings;
mod refusals;
mod refused;
mod scheduling;
mod secrecy;
mod streaming;
mod table;
mod tag;
mod terminal;
mod threshold_args;
mod wire;
