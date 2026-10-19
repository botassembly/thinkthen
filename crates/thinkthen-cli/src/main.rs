//! The `thinkthen` command and local stdio server.

#![cfg_attr(not(windows), forbid(unsafe_code))]
#![cfg_attr(windows, deny(unsafe_code))]

use std::process::ExitCode;
use thinkthen::*;

mod cli;
mod mcp;
mod statistics;
mod probes;
mod plan_document;
#[cfg(windows)]
mod windows;

use cli::{args, asking, edge, failure, judge, profile, schedule};

fn main() -> ExitCode {
    cli::entry()
}
