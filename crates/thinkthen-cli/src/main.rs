//! The `thinkthen` command and local stdio server.

#![cfg_attr(not(windows), forbid(unsafe_code))]
#![cfg_attr(windows, deny(unsafe_code))]

use std::process::ExitCode;
use thinkthen::*;

#[cfg(test)]
mod schema_tests;
#[cfg(test)]
mod test_deadline;

mod cli;
mod mcp;
mod plan_document;
mod probes;
mod render;
mod saved_json;
mod statistics;
#[cfg(windows)]
mod windows;

use cli::{args, asking, edge, failure, judge, profile, schedule};

fn main() -> ExitCode {
    cli::entry()
}
