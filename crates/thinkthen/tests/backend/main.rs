//! The compiled binary against a loopback backend, as one test binary.
//!
//! The three pages below share `harness`, and one binary compiles it once. A
//! module split over several test binaries would leave part of the harness
//! unused in each one, and `rust-standards.md` lets no test file paste a
//! suppression at its top.

mod harness;

mod address;
mod choosing;
mod exchange;
mod from_record;
mod parallel;
mod recordings;
mod streaming;
