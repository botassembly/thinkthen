//! Serve the shared cases on loopback until standard input closes.
//!
//! The first line out is the port. A `count` line in prints the requests read
//! so far, a `release` line lets held replies go for good, a `round` line lets
//! go those held now, a `wait N` line later prints `wait K` once the count
//! reaches N or 5 s pass, and the final count prints on exit.

use std::io::{self, Write as _};
use std::process::ExitCode;

fn main() -> ExitCode {
    match conformance_backend::run(io::stdin().lock(), io::stdout()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = writeln!(io::stderr(), "conformance-backend: {error}");
            ExitCode::FAILURE
        }
    }
}
