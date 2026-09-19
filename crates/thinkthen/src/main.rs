//! The `thinkthen` command line.
//!
//! This crate owns every edge: arguments, the environment, standard input,
//! standard output, HTTP, and the exit code. It hands typed values to
//! `thinkthen-core` and prints what comes back.

#![forbid(unsafe_code)]

mod args;
mod decide;
mod edge;
mod failure;
mod http;
mod recorder;

use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

use crate::args::{Cli, Command};
use crate::edge::Environment;
use crate::failure::Failure;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let stdout = io::stdout();
    let stderr = io::stderr();
    match run(&cli, stdout.lock()) {
        Ok(code) => code,
        Err(failure) => failure::report(&failure, stderr.lock()),
    }
}

/// Run what was asked and give back the exit code it earned.
fn run(cli: &Cli, writer: impl Write) -> Result<ExitCode, Failure> {
    if cli.version {
        let line = thinkthen_core::version_line(env!("CARGO_PKG_VERSION"));
        edge::write_line(writer, &line)?;
        return Ok(ExitCode::SUCCESS);
    }
    match &cli.command {
        Some(Command::Decide(arguments)) => {
            decide::decide(arguments, &Environment::read(), io::stdin().lock(), writer)
        }
        None => Err(Failure::Defect("no command and no version was parsed")),
    }
}
