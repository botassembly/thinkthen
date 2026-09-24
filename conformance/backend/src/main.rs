//! Serve the shared cases on loopback until standard input closes.
//!
//! The first line out is the port. A `count` line in prints the requests read
//! so far, a `release` line lets held replies go, and the final count prints
//! on exit.

use std::io::{self, Write as _};
use std::process::ExitCode;

fn main() -> ExitCode {
    match conformance_backend::run(io::stdin().lock(), io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = writeln!(io::stderr(), "conformance-backend: {error}");
            ExitCode::FAILURE
        }
    }
}
