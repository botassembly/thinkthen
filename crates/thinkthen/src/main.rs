//! The `thinkthen` command line.
//!
//! This crate owns every edge: arguments, the environment, standard input,
//! standard output, HTTP, and the exit code. It hands typed values to
//! `thinkthen-core` and prints what comes back.

#![forbid(unsafe_code)]

mod args;
mod asked;
mod asking;
mod edge;
mod failure;
mod http;
mod judge;
mod recorder;
mod schedule;

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
    let environment = Environment::read();
    // A person at a terminal is told what the command waits for, before the
    // command settles down to read it. The line goes nowhere in a pipe.
    if let Some(command) = cli.command.as_ref() {
        edge::waiting(command.common().input.as_deref(), io::stderr().lock());
    }
    let input = io::stdin();
    match &cli.command {
        Some(Command::Decide(arguments)) => judge::decide(arguments, &environment, input, writer),
        Some(Command::Choose(arguments)) => judge::choose(arguments, &environment, input, writer),
        Some(Command::Score(arguments)) => judge::score(arguments, &environment, input, writer),
        Some(Command::Filter(arguments)) => judge::filter(arguments, &environment, input, writer),
        Some(Command::Rank(arguments)) => judge::rank(arguments, &environment, input, writer),
        None => Err(Failure::Defect("no command and no version was parsed")),
    }
}
