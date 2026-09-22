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
mod cache_configuration;
mod cache_identity;
mod cache_locking;
#[cfg(target_os = "linux")]
mod cache_prune_locking;
mod choosing;
mod default_cache;
mod distribution_total;
mod exchange;
mod find;
mod from_record;
mod json_syntax;
mod keeping;
mod limits;
mod parallel;
mod profile;
mod record_values;
mod recording_conflicts;
mod recording_durability;
mod recordings;
mod refusals;
mod refused;
mod result_assertions;
mod scheduling;
mod secrecy;
mod secrecy_find;
mod state;
mod streaming;
mod support;
mod table;
mod tag;
mod terminal;
mod threshold_args;
mod timeout;
mod wire;
