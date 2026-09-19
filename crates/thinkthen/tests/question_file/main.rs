//! A question read from a file, and every refusal the two homes make.
//!
//! The four pages below share `harness`, and one binary compiles it once. A
//! split over several test binaries would leave part of the harness unused in
//! each one, and `rust-standards.md` lets no test file paste a suppression at
//! its top.

mod harness;

mod grammar;
mod overrides;
mod secrecy;
