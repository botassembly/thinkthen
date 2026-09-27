//! The public API from outside the crate, as one test binary.
//!
//! The case runner reads shared cases whose missing members read as null and
//! fail their comparison, so it indexes JSON freely.

#[allow(
    clippy::indexing_slicing,
    clippy::expect_used,
    reason = "a missing member of a shared case reads as null and fails its comparison"
)]
mod cases;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a loopback fixture that fails should stop this proof"
)]
mod parts;
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a loopback fixture that fails should stop this proof"
)]
mod paths;
#[path = "../../../../../crates/thinkthen/src/test_deadline/run.rs"]
mod run;
#[allow(
    clippy::expect_used,
    reason = "public settings regression fixtures must load"
)]
mod settings;
#[path = "../../../../../crates/thinkthen/src/test_deadline/wait.rs"]
mod wait;
