//! The compiled binary against a loopback backend, as one test binary.
//!
//! The pages below share `harness`, and one binary compiles it once. A
//! module split over several test binaries would leave part of the harness
//! unused in each one, and `rust-standards.md` lets no test file paste a
//! suppression at its top. Every other command-line page joins this binary
//! too, each with its own helpers (ticket 0338).
#![cfg(feature = "cli")]

#[path = "../../src/test_deadline/child.rs"]
mod child;
mod harness;
#[allow(
    clippy::expect_used,
    reason = "a measurement fixture that cannot run stops the proof"
)]
#[path = "../support/measure.rs"]
mod measure_support;
use child::{run, wait};

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
// Its cases name the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
mod cache_configuration;
mod cache_identity;
// Its cases name the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
mod cache_partial;
#[cfg(unix)]
mod cache_trust;
#[allow(
    clippy::expect_used,
    reason = "a helper that cannot run the check or read its files should stop the test"
)]
mod check;
mod choosing;
#[allow(
    clippy::expect_used,
    reason = "fixture setup must run the compiled CLI"
)]
mod command_namespace;
// Windows does not tell the command that its output reader closed (sdlc/planning/windows.md).
#[cfg(unix)]
mod closed_pipe;
// Its cases name the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
mod default_cache;
// Its cases set Unix folder modes (sdlc/planning/windows.md).
#[cfg(unix)]
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
mod find_display;
mod from_record;
mod images;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "fixture setup and checked JSON rows stop the intake boundary proof"
)]
mod input_sources;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "fixture setup and measured requests stop the compatibility proof"
)]
mod input_sources_compat;
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
#[allow(
    clippy::expect_used,
    reason = "fixture failures stop the CLI byte proof"
)]
mod record_display;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "fixture failures and checked stored keys stop the identity proof"
)]
mod record_display_identity;
#[allow(
    clippy::expect_used,
    reason = "fixture failures stop the CLI safety proof"
)]
mod record_display_safety;
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
mod row_usage;
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
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::unwrap_used,
    reason = "fixture failures stop the located source boundary proof"
)]
mod source_files;
mod source_limits;
mod state;
mod status_reason;
mod streaming;
mod support;
mod table;
mod tag;
// It gives the binary a pseudo-terminal through the Unix `script` tool.
#[cfg(unix)]
mod terminal;
mod threshold_args;
mod timeout;
mod wire;

// The command-line pages that were their own binaries until ticket 0338.
mod audit;
mod audit_cases;
mod audit_model;
mod audit_output;
mod audit_refusals;
mod audit_sets;
mod audit_verbs;
mod audit_write;
mod cache_convert;
mod choose_and_score_edge;
mod decide_edge;
// The demo runner is a shell script that needs `sh`, `mustmatch` and a `:` PATH.
#[cfg(unix)]
mod demo_runner;
mod diff;
mod dry_run_terminal;
mod find_edge;
mod hints;
mod named_backends;
mod question_file;
mod relate_edge;
mod settings_cases;
// The speed probe starts its children with only PATH and HOME, which a Windows child cannot run under.
#[cfg(unix)]
mod speed;
mod status;
mod transform;
mod version;

#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "checked loopback fixtures and result rows stop the rank-set contract proof"
)]
mod rank_set;

#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "checked result objects and recorded keys stop the identity proof"
)]
mod rank_set_identity;
