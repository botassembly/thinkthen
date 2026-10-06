//! Command parsing, input framing, output, diagnostics, and exit codes.

pub(crate) mod annotate;
pub(crate) mod args;
pub(crate) mod asked;
pub(crate) mod asking;
mod audit;
pub(crate) mod cache;
mod check;
mod diff;
pub(crate) mod display;
pub(crate) mod edge;
mod facts;
pub(crate) mod failure;
mod file_size;
pub(crate) mod find;
mod hint;
pub(crate) mod intake;
mod interrupt;
pub(crate) mod judge;
mod measure;
pub(crate) mod normalize;
pub(crate) mod profile;
mod question_text;
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
use std::time::Instant;

use crate::cli::args::{Cli, Command, Common};
use crate::cli::asking::Folders;
use crate::cli::edge::Environment;
use crate::cli::failure::Failure;
use crate::core::{JsonError, RecordError, safe_key, version_line};
use clap::{CommandFactory as _, FromArgMatches as _};

/// Parse the process inputs, run one command, and report its exit code.
#[must_use]
pub fn entry() -> ExitCode {
    let started = Instant::now();
    let cli = match parsed_cli() {
        Ok(cli) => cli,
        Err(error) => return hint::refused(error),
    };
    let wants_facts = cli.command.as_ref().is_some_and(facts::enabled);
    let stdout = io::stdout();
    let stderr = io::stderr();
    if cli.version {
        return match edge::write_line(stdout.lock(), &version_line(env!("CARGO_PKG_VERSION"))) {
            Ok(_) => ExitCode::SUCCESS,
            Err(failure) => failure::report(&failure, stderr.lock()),
        };
    }
    let offline = match &cli.command {
        Some(Command::Transform(arguments)) => {
            Some(transform::run(&arguments.command, stdout.lock()))
        }
        Some(Command::Audit(arguments) | Command::Runs(args::RunsCommand::Audit(arguments))) => {
            Some(audit::run(arguments, stdout.lock()))
        }
        Some(Command::Diff(arguments) | Command::Runs(args::RunsCommand::Diff(arguments))) => {
            Some(diff::run(arguments, stdout.lock()))
        }
        _ => None,
    };
    if let Some(result) = offline {
        return match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(failure) => failure::report(&failure, stderr.lock()),
        };
    }
    // Every command that reads input may write a recording or a cache entry.
    if cli.command.as_ref().is_some_and(Command::reads_input)
        && let Err(failure) = file_size::claim()
    {
        return report_early(&failure, wants_facts, started, stderr.lock());
    }
    let mut environment = match Environment::read() {
        Ok(environment) => environment,
        Err(failure) => return report_early(&failure, wants_facts, started, stderr.lock()),
    };
    if environment.config().shared() {
        let mut writer = stderr.lock();
        #[cfg(not(windows))]
        let warning = "thinkthen: the configuration file is writable by another user; it decides where the key and evidence go";
        #[cfg(windows)]
        let warning = "thinkthen: another user owns the configuration file or its Windows access permissions allow another user to change it; it decides where the key and evidence go";
        let _unwritten = writeln!(writer, "{warning}").and_then(|()| writer.flush());
    }
    let activation = match interrupt::activate(&mut environment) {
        Ok(activation) => activation,
        Err(failure) => return report_early(&failure, wants_facts, started, stderr.lock()),
    };
    let result = run(&cli, &environment, stdout.lock());
    let (code, stopped) = match result {
        Ok(code) => (code, None),
        Err(failure) => {
            let failure = told(failure, &cli, &environment);
            let code = failure::facts::report(&failure, stderr.lock());
            let stopped = wants_facts.then(|| failure::facts::Stopped::of(&failure, code));
            (ExitCode::from(code), stopped)
        }
    };
    if environment.usage().finish() {
        let mut writer = stderr.lock();
        let advice = environment.usage().failed_file().map_or_else(
            || "check the usage folder permissions and free space".to_owned(),
            |(name, category)| format!("local usage file {name} has {category}"),
        );
        let _unwritten = writeln!(
            writer,
            "thinkthen: usage counters could not be updated; {advice}"
        )
        .and_then(|()| writer.flush());
    }
    if wants_facts {
        let snapshot = environment.usage().run_snapshot();
        let elapsed = started.elapsed();
        let writer = stderr.lock();
        let stopped = if activation.cancelled() {
            Some(failure::facts::Stopped::of(&Failure::Cancelled, 130))
        } else {
            stopped
        };
        facts::write(writer, snapshot, elapsed, stopped);
    }
    match activation.finish(code) {
        Ok(code) => code,
        Err(failure) => failure::report(&failure, stderr.lock()),
    }
}

/// Settle all positional routes once.
fn parsed_cli() -> Result<Cli, clap::Error> {
    let parser = Cli::command();
    let matches = parser.try_get_matches_from(normalize::arguments(std::env::args_os()))?;
    let mut cli = Cli::from_arg_matches(&matches)?;
    if let Some(command) = cli.command.as_mut() {
        command.route_inputs().map_err(|error| match error {
            Failure::Usage(message) => {
                clap::Error::raw(clap::error::ErrorKind::ValueValidation, message)
            }
            _ => clap::Error::raw(
                clap::error::ErrorKind::ValueValidation,
                "invalid input routes",
            ),
        })?;
    }
    Ok(cli)
}

fn report_early(
    failure: &Failure,
    wants_facts: bool,
    started: Instant,
    mut writer: impl Write,
) -> ExitCode {
    let code = failure::facts::report(failure, &mut writer);
    if wants_facts {
        facts::write(
            &mut writer,
            crate::engine::usage::Counters::default().run_snapshot(),
            started.elapsed(),
            Some(failure::facts::Stopped::of(failure, code)),
        );
    }
    ExitCode::from(code)
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
            args::CacheCommand::Unused(arguments) => cache::unused(arguments, writer),
            args::CacheCommand::Convert(arguments) => cache::convert(arguments),
        },
        Some(Command::Status(arguments)) => status::run(arguments, environment, writer),
        Some(
            Command::Check(arguments) | Command::Backends(args::BackendCommand::Check(arguments)),
        ) => check::run(arguments, environment, writer),
        Some(Command::Transform(_) | Command::Runs(_) | Command::Audit(_) | Command::Diff(_)) => {
            Err(Failure::Defect(
                "the catalog, audit, and diff return before setup",
            ))
        }
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
        Failure::Configuration(DEFAULT_CACHE_STORAGE.to_owned())
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
        | Command::Backends(_)
        | Command::Transform(_)
        | Command::Runs(_)
        | Command::Audit(_)
        | Command::Diff(_) => false,
    }
}
