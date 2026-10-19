//! The `thinkthen` command and local stdio server.

#![cfg_attr(not(windows), forbid(unsafe_code))]
#![cfg_attr(windows, deny(unsafe_code))]

use std::process::ExitCode;
use thinkthen::*;

#[cfg(test)]
mod schema_tests;

mod cli;
mod mcp;
mod statistics;
mod saved_json;
mod probes;
mod plan_document;
mod render;
#[cfg(windows)]
mod windows;

use cli::{args, asking, edge, failure, judge, profile, schedule};

fn main() -> ExitCode {
    cli::entry()
}
