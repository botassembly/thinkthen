//! The thin `thinkthen` process entrypoint.

#![forbid(unsafe_code)]

use std::process::ExitCode;

fn main() -> ExitCode {
    thinkthen::entry()
}
