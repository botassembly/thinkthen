//! The internal application library behind the `thinkthen` binary.

#![forbid(unsafe_code)]

mod annotate;
mod annotate_schedule;
mod args;
mod asked;
mod asking;
mod cache_lock;
mod edge;
mod failure;
mod find;
mod http;
mod judge;
mod normalize;
mod prepared_request;
mod recorder;
mod schedule;
mod table;

use std::io::{self, Write};
use std::process::ExitCode;

use crate::args::{Cli, Command};
use crate::edge::Environment;
use crate::failure::Failure;
use clap::Parser as _;

/// Parse the process inputs, run one command, and report its exit code.
#[must_use]
pub fn entry() -> ExitCode {
    let cli = Cli::parse_from(normalize::arguments(std::env::args_os()));
    let stdout = io::stdout();
    let stderr = io::stderr();
    match run(&cli, stdout.lock()) {
        Ok(code) => code,
        Err(failure) => failure::report(&failure, stderr.lock()),
    }
}

fn run(cli: &Cli, writer: impl Write) -> Result<ExitCode, Failure> {
    if cli.version {
        let line = thinkthen_core::version_line(env!("CARGO_PKG_VERSION"));
        edge::write_line(writer, &line)?;
        return Ok(ExitCode::SUCCESS);
    }
    let environment = Environment::read();
    if let Some(command) = cli.command.as_ref() {
        edge::waiting(command.input(), io::stderr().lock());
    }
    let input = io::stdin();
    match &cli.command {
        Some(Command::Decide(arguments)) => judge::decide(arguments, &environment, input, writer),
        Some(Command::Choose(arguments)) => judge::choose(arguments, &environment, input, writer),
        Some(Command::Tag(arguments)) => judge::tag(arguments, &environment, input, writer),
        Some(Command::Score(arguments)) => judge::score(arguments, &environment, input, writer),
        Some(Command::Filter(arguments)) => judge::filter(arguments, &environment, input, writer),
        Some(Command::Rank(arguments)) => judge::rank(arguments, &environment, input, writer),
        Some(Command::Find(arguments)) => find::run(arguments, &environment, input, writer),
        Some(Command::Annotate(arguments)) => annotate::run(arguments, &environment, input, writer),
        None => Err(Failure::Defect("no command and no version was parsed")),
    }
}
