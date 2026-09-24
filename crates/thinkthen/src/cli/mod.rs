//! Command parsing, input framing, output, diagnostics, and exit codes.

pub(crate) mod annotate;
pub(crate) mod annotate_schedule;
pub(crate) mod args;
pub(crate) mod asked;
pub(crate) mod asking;
mod audit;
pub(crate) mod cache;
mod diff;
pub(crate) mod edge;
pub(crate) mod failure;
mod file_size;
pub(crate) mod find;
mod interrupt;
pub(crate) mod judge;
mod measure;
pub(crate) mod normalize;
pub(crate) mod profile;
pub(crate) mod recognize;
pub(crate) mod relate;
pub(crate) mod schedule;
pub(crate) mod status;
pub(crate) mod table;
mod transform;

#[cfg(test)]
mod conformance_tests;

use std::io::{self, Write};
use std::process::ExitCode;

use crate::cli::args::{Cli, Command};
use crate::cli::edge::Environment;
use crate::cli::failure::Failure;
use crate::core::version_line;
use clap::Parser as _;

/// Parse the process inputs, run one command, and report its exit code.
#[must_use]
pub fn entry() -> ExitCode {
    let cli = Cli::parse_from(normalize::arguments(std::env::args_os()));
    let stdout = io::stdout();
    let stderr = io::stderr();
    if cli.version {
        return match edge::write_line(stdout.lock(), &version_line(env!("CARGO_PKG_VERSION"))) {
            Ok(_) => ExitCode::SUCCESS,
            Err(failure) => failure::report(&failure, stderr.lock()),
        };
    }
    if let Some(Command::Transform(arguments)) = &cli.command {
        return match transform::run(&arguments.command, stdout.lock()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(failure) => failure::report(&failure, stderr.lock()),
        };
    }
    if let Some(Command::Audit(arguments)) = &cli.command {
        return match audit::run(arguments, stdout.lock()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(failure) => failure::report(&failure, stderr.lock()),
        };
    }
    if let Some(Command::Diff(arguments)) = &cli.command {
        return match diff::run(arguments, stdout.lock()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(failure) => failure::report(&failure, stderr.lock()),
        };
    }
    // Every command that reads input may write a recording or a cache entry.
    if cli.command.as_ref().is_some_and(Command::reads_input)
        && let Err(failure) = file_size::claim()
    {
        return failure::report(&failure, stderr.lock());
    }
    let mut environment = match Environment::read() {
        Ok(environment) => environment,
        Err(failure) => return failure::report(&failure, stderr.lock()),
    };
    let activation = match interrupt::activate(&mut environment) {
        Ok(activation) => activation,
        Err(failure) => return failure::report(&failure, stderr.lock()),
    };
    let result = run(&cli, &environment, stdout.lock());
    let code = match result {
        Ok(code) => code,
        Err(failure) => failure::report(&failure, stderr.lock()),
    };
    if environment.usage().warning() {
        let mut writer = stderr.lock();
        let _unwritten = writeln!(writer, "thinkthen: usage counters could not be updated; check the usage folder permissions and free space")
            .and_then(|()| writer.flush());
    }
    match activation.finish(code) {
        Ok(code) => code,
        Err(failure) => failure::report(&failure, stderr.lock()),
    }
}

fn run(cli: &Cli, environment: &Environment, writer: impl Write) -> Result<ExitCode, Failure> {
    if cli
        .command
        .as_ref()
        .is_some_and(|command| command.timeout() == 0)
    {
        return Err(Failure::Usage(
            "--timeout takes a whole number of seconds greater than zero",
        ));
    }
    if let Some(command) = cli.command.as_ref().filter(|command| command.reads_input()) {
        edge::waiting(command.input(), io::stderr().lock());
    }
    let input = io::stdin();
    match &cli.command {
        Some(Command::Decide(arguments)) => judge::decide(arguments, environment, input, writer),
        Some(Command::Choose(arguments)) => judge::choose(arguments, environment, input, writer),
        Some(Command::Tag(arguments)) => judge::tag(arguments, environment, input, writer),
        Some(Command::Score(arguments)) => judge::score(arguments, environment, input, writer),
        Some(Command::Filter(arguments)) => judge::filter(arguments, environment, input, writer),
        Some(Command::Rank(arguments)) => judge::rank(arguments, environment, input, writer),
        Some(Command::Find(arguments)) => find::run(arguments, environment, input, writer),
        Some(Command::Annotate(arguments)) => annotate::run(arguments, environment, input, writer),
        Some(Command::Recognize(arguments)) => {
            recognize::run(arguments, environment, input, writer)
        }
        Some(Command::Relate(arguments)) => relate::run(arguments, environment, input, writer),
        Some(Command::Cache(arguments)) => match &arguments.command {
            args::CacheCommand::Prune(arguments) => cache::prune(arguments, environment, writer),
        },
        Some(Command::Status(arguments)) => status::run(arguments, environment, writer),
        Some(Command::Transform(_) | Command::Audit(_) | Command::Diff(_)) => Err(Failure::Defect(
            "the catalog, audit, and diff return before setup",
        )),
        None => Err(Failure::Defect("no command and no version was parsed")),
    }
}
