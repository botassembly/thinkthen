//! Command parsing, input framing, output, diagnostics, and exit codes.

pub(crate) mod annotate;
pub(crate) mod annotate_schedule;
pub(crate) mod args;
pub(crate) mod asked;
pub(crate) mod asking;
mod audit;
pub(crate) mod cache;
mod check;
mod diff;
pub(crate) mod edge;
pub(crate) mod failure;
mod file_size;
pub(crate) mod find;
mod hint;
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

use crate::cli::args::{Cli, Command, Common};
use crate::cli::asking::Folders;
use crate::cli::edge::Environment;
use crate::cli::failure::Failure;
use crate::core::{JsonError, RecordError, safe_key, version_line};
use clap::Parser as _;

/// Parse the process inputs, run one command, and report its exit code.
#[must_use]
pub fn entry() -> ExitCode {
    let cli = match Cli::try_parse_from(normalize::arguments(std::env::args_os())) {
        Ok(cli) => cli,
        Err(error) => return hint::refused(error),
    };
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
    if environment.config().shared() {
        let mut writer = stderr.lock();
        let _unwritten = writeln!(
            writer,
            "thinkthen: the configuration file is writable by another user; it decides where the key and evidence go"
        )
        .and_then(|()| writer.flush());
    }
    let activation = match interrupt::activate(&mut environment) {
        Ok(activation) => activation,
        Err(failure) => return failure::report(&failure, stderr.lock()),
    };
    let result = run(&cli, &environment, stdout.lock());
    let code = match result {
        Ok(code) => code,
        Err(failure) => failure::report(&told(failure, &cli, &environment), stderr.lock()),
    };
    if environment.usage().finish() {
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
    // A day bounds the timeout, because the HTTP client adds it to the clock
    // and a larger number can overflow there.
    if cli
        .command
        .as_ref()
        .is_some_and(|command| !(1..=86_400).contains(&command.timeout()))
    {
        return Err(Failure::Usage(
            "--timeout takes a whole number of seconds from 1 to 86400",
        ));
    }
    if cli.command.as_ref().is_some_and(Command::stray) {
        return Err(Failure::Usage(hint::ONE_QUESTION));
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
        Some(Command::Check(arguments)) => check::run(arguments, environment, writer),
        Some(Command::Transform(_) | Command::Audit(_) | Command::Diff(_)) => Err(Failure::Defect(
            "the catalog, audit, and diff return before setup",
        )),
        None => Err(Failure::Defect("no command and no version was parsed")),
    }
}

/// The default cache's own storage sentence (ticket 0138).
const DEFAULT_CACHE_STORAGE: &str = "the default cache folder could not be read or written; check its permissions and free space, use --no-cache, or set THINKTHEN_CACHE to another folder";

/// Say a failure in the words this run's own command line calls for: a
/// storage failure in the platform default cache names that cache, and a
/// refused pointer is echoed with JSON escapes.
fn told(mut failure: Failure, cli: &Cli, environment: &Environment) -> Failure {
    if environment.cancel().fired() {
        failure = failure.after_signal();
    }
    if let Failure::Stopped { at: 1, cause, .. } = &mut failure
        && matches!(
            cause.as_ref(),
            Failure::Record(RecordError::Json(JsonError::Syntax { .. }))
        )
        && cli.command.as_ref().is_some_and(Command::typed_jsonl)
    {
        **cause = Failure::Usage(hint::NOT_JSON_LINES);
    }
    let storage = match &failure {
        Failure::RecordingStorage => true,
        Failure::Stopped { cause, .. } => matches!(cause.as_ref(), Failure::RecordingStorage),
        Failure::Pointer(option, typed, error) => {
            return Failure::Pointer(option, safe_key(typed), *error);
        }
        _ => false,
    };
    let default = || {
        cli.command
            .as_ref()
            .is_some_and(|command| in_default_cache(command, environment))
    };
    if storage && default() {
        Failure::Configuration(DEFAULT_CACHE_STORAGE)
    } else {
        failure
    }
}

/// Whether a record command's folders are the platform default cache. A
/// command line `Folders` refuses keeps its own failure.
fn in_default_cache(command: &Command, environment: &Environment) -> bool {
    let default = |common: &Common| {
        Folders::of(common, environment).is_ok_and(|folders| folders.private_default)
    };
    match command {
        Command::Decide(arguments) => default(&arguments.common),
        Command::Choose(arguments) => default(&arguments.common),
        Command::Tag(arguments) => default(&arguments.common),
        Command::Score(arguments) => default(&arguments.common),
        Command::Filter(arguments) => default(&arguments.common),
        Command::Rank(arguments) => default(&arguments.common),
        Command::Find(arguments) => default(&arguments.common.as_common()),
        Command::Annotate(arguments) => default(&arguments.common),
        Command::Recognize(arguments) => default(&arguments.common),
        Command::Relate(arguments) => default(&arguments.common),
        Command::Cache(_)
        | Command::Status(_)
        | Command::Check(_)
        | Command::Transform(_)
        | Command::Audit(_)
        | Command::Diff(_) => false,
    }
}
