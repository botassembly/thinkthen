//! A question read from a file, and every refusal the two homes make.
//!
//! The six pages below share `harness`, and one binary compiles it once. A
//! split over several test binaries would leave part of the harness unused in
//! each one, and `rust-standards.md` lets no test file paste a suppression at
//! its top.
#![cfg(feature = "cli")]

mod harness;
#[path = "../../src/test_deadline/wait.rs"]
mod wait;

mod corpus;
mod grammar;
mod overrides;
mod relate;
mod secrecy;
mod structured;
