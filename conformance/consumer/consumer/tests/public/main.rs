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
mod paths;
