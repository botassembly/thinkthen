//! The compiled binary against a loopback backend, as one test binary.
//!
//! The three pages below share `harness`, and one binary compiles it once. A
//! module split over several test binaries would leave part of the harness
//! unused in each one, and `rust-standards.md` lets no test file paste a
//! suppression at its top.
#![cfg(feature = "cli")]

#[path = "../../src/test_deadline/child.rs"]
mod child;
mod harness;

mod address;
mod annotate;
mod annotate_on;
mod asked;
#[allow(
    clippy::expect_used,
    reason = "a failed loopback fixture stops the proof"
)]
mod backoff;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a helper that cannot start the command stops the test; rows and requests are counted before they are read"
)]
mod batching;
mod blank_lines;
mod cache_configuration;
mod cache_identity;
mod cache_partial;
#[cfg(unix)]
mod cache_trust;
#[allow(
    clippy::expect_used,
    reason = "a helper that cannot run the check or read its files should stop the test"
)]
mod check;
mod choosing;
mod closed_pipe;
mod default_cache;
#[cfg(target_os = "linux")]
mod default_cache_storage;
mod distribution_total;
mod exchange;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a missing compiled-command fact or loopback fixture stops the boundary proof"
)]
mod facts;
mod find;
mod from_record;
#[cfg(unix)]
mod interrupt;
mod json_syntax;
mod keeping;
mod limits;
mod loopback_arms;
#[allow(
    clippy::indexing_slicing,
    reason = "a missing member of a shared case reads as null and fails its comparison"
)]
mod loopback_cases;
mod parallel;
mod pointer_echo;
mod profile;
mod public_json;
mod question_cache;
mod question_cache_steps;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::unwrap_used,
    reason = "fixture failures should stop this compiled command-boundary proof"
)]
mod recognize;
mod recognize_refusals;
mod record_values;
mod recording_conflicts;
mod recording_durability;
mod recordings;
mod refusals;
mod refused;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "fixture failures should stop this compiled relation boundary proof"
)]
mod relate;
mod resend;
mod result_assertions;
mod scheduling;
mod secrecy;
mod secrecy_find;
#[allow(
    clippy::expect_used,
    reason = "fixture failures should stop this recognize secrecy proof"
)]
mod secrecy_recognize;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "fixture failures should stop this relation secrecy proof"
)]
mod secrecy_relate;
mod state;
mod status_reason;
mod streaming;
mod support;
mod table;
mod tag;
mod terminal;
mod threshold_args;
mod timeout;
mod wire;
