//! Command parsing, input framing, output, diagnostics, and exit codes.

pub(crate) mod annotate;
pub(crate) mod args;
pub(crate) mod asked;
pub(crate) mod asking;
mod audit;
pub(crate) mod cache;
mod check;
mod construction;
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
mod request;
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
    let accepted = Instant::now();
    let wants_facts = cli.command.as_ref().is_some_and(facts::enabled);
    let stdout = io::stdout();
    let stderr = io::stderr();
    if cli.version {
        return match edge::write_line(stdout.lock(), &version_line(env!("CARGO_PKG_VERSION"))) {
            Ok(_) => ExitCode::SUCCESS,
            Err(failure) => failure::report(&failure, stderr.lock()),
        };
    }
    if let Some(Command::Mcp(arguments)) = &cli.command {
        return mcp_entry(arguments);
    }
    if let Some(Command::Transform(arguments)) = &cli.command {
        return report_offline(
            transform::run(&arguments.command, stdout.lock()),
            stderr.lock(),
        );
    }
    if let Some(Command::Audit(arguments) | Command::Runs(args::RunsCommand::Audit(arguments))) =
        &cli.command
    {
        return report_offline(audit::run(arguments, stdout.lock()), stderr.lock());
    }
    if let Some(Command::Diff(arguments) | Command::Runs(args::RunsCommand::Diff(arguments))) =
        &cli.command
    {
        return report_offline(diff::run(arguments, stdout.lock()), stderr.lock());
    }
    let admitted = match request_admission(&cli, wants_facts, started, accepted, stderr.lock()) {
        Ok(admitted) => admitted,
        Err(code) => return code,
    };
    // Every command that reads input may write a recording or a cache entry.
    if cli.command.as_ref().is_some_and(Command::reads_input)
        && let Err(failure) = file_size::claim()
    {
        return report_early(&failure, wants_facts, (started, accepted), stderr.lock());
    }
    let mut environment = match Environment::read() {
        Ok(environment) => environment,
        Err(failure) => {
            return report_early(&failure, wants_facts, (started, accepted), stderr.lock());
        }
    };
    warn_configuration(&environment, stderr.lock());
    let activation = match interrupt::activate(&mut environment) {
        Ok(activation) => activation,
        Err(failure) => {
            return report_early(&failure, wants_facts, (started, accepted), stderr.lock());
        }
    };
    let result = run(&cli, &environment, admitted, stdout.lock());
    let (code, stopped) = match result {
        Ok(code) => (code, None),
        Err(failure) => {
            let failure = told(failure, &cli, &environment);
            let code = failure::facts::report(&failure, stderr.lock());
            let stopped = wants_facts.then(|| failure::facts::Stopped::of(&failure, code));
            (ExitCode::from(code), stopped)
        }
    };
    warn_held_model_mismatch(&environment, stderr.lock());
    finish_usage(&environment);
    if wants_facts {
        write_run_facts(
            &environment,
            started,
            accepted,
            stopped,
            activation.cancelled(),
        );
    }
    match activation.finish(code) {
        Ok(code) => code,
        Err(failure) => failure::report(&failure, stderr.lock()),
    }
}

fn write_run_facts(
    environment: &Environment,
    started: Instant,
    accepted: Instant,
    stopped: Option<failure::facts::Stopped>,
    cancelled: bool,
) {
    let stderr = io::stderr();
    let snapshot = environment.usage().run_snapshot();
    let elapsed = started.elapsed();
    let writer = stderr.lock();
    let stopped = if cancelled {
        Some(failure::facts::Stopped::of(&Failure::Cancelled, 130))
    } else {
        stopped
    };
    facts::write(
        writer,
        snapshot,
        elapsed,
        accepted.elapsed(),
        stopped,
        environment
            .native_call_id()
            .or_else(|| environment.cancel().call_id()),
    );
}

fn mcp_entry(arguments: &crate::mcp::startup::Arguments) -> ExitCode {
    match file_size::claim() {
        Ok(()) => crate::mcp::startup::entry(arguments),
        Err(failure) => failure::report(&failure, io::stderr().lock()),
    }
}

fn warn_configuration(environment: &Environment, mut writer: impl Write) {
    warn_configuration_shared(environment.config().shared(), &mut writer);
}

pub(crate) fn warn_configuration_shared(shared: bool, mut writer: impl Write) {
    if shared {
        #[cfg(not(windows))]
        let warning = "thinkthen: the configuration file is writable by another user; it decides where the key and evidence go";
        #[cfg(windows)]
        let warning = "thinkthen: another user owns the configuration file or its Windows access permissions allow another user to change it; it decides where the key and evidence go";
        let _unwritten = writeln!(writer, "{warning}").and_then(|()| writer.flush());
    }
}

fn warn_held_model_mismatch(environment: &Environment, mut writer: impl Write) {
    if environment.usage().has_held_model_mismatch() {
        let _unwritten = writeln!(
            writer,
            "thinkthen: warning: a held answer names a different model and cannot be reused online"
        );
    }
}

fn finish_usage(environment: &Environment) {
    if environment.usage().finish() {
        let mut writer = io::stderr().lock();
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
    (started, accepted): (Instant, Instant),
    mut writer: impl Write,
) -> ExitCode {
    let code = failure::facts::report(failure, &mut writer);
    if wants_facts {
        facts::write(
            &mut writer,
            crate::engine::usage::Counters::default().run_snapshot(),
            started.elapsed(),
            accepted.elapsed(),
            Some(failure::facts::Stopped::of(failure, code)),
            None,
        );
    }
    ExitCode::from(code)
}

fn run(
    cli: &Cli,
    environment: &Environment,
    admitted: Option<crate::AdmittedRequest>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
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
        Some(Command::Mcp(_)) => Err(Failure::Defect("MCP command bypassed startup")),
        Some(Command::Decide(arguments)) => judge::decide(arguments, environment, admitted.ok_or(Failure::Defect("atomic call has no admitted request"))?, input, writer),
        Some(Command::Choose(arguments)) => judge::choose(arguments, environment, admitted.ok_or(Failure::Defect("atomic call has no admitted request"))?, input, writer),
        Some(Command::Tag(arguments)) => judge::tag(arguments, environment, admitted.ok_or(Failure::Defect("atomic call has no admitted request"))?, input, writer),
        Some(Command::Score(arguments)) => judge::score(arguments, environment, admitted.ok_or(Failure::Defect("atomic call has no admitted request"))?, input, writer),
        Some(Command::Filter(arguments)) => judge::filter(arguments, environment, admitted.ok_or(Failure::Defect("atomic call has no admitted request"))?, input, writer),
        Some(Command::Rank(arguments)) => judge::rank(arguments, environment, input, writer),
        Some(Command::Find(arguments)) => find::run(
            arguments,
            environment,
            admitted.ok_or(Failure::Defect("find has no admitted request"))?,
            input,
            writer,
        ),
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
        | Command::Mcp(_)
        | Command::Check(_)
        | Command::Backends(_)
        | Command::Transform(_)
        | Command::Runs(_)
        | Command::Audit(_)
        | Command::Diff(_) => false,
    }
}

fn report_offline(result: Result<(), Failure>, writer: impl Write) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => failure::report(&failure, writer),
    }
}

fn request_admission(
    cli: &Cli,
    wants_facts: bool,
    started: Instant,
    accepted: Instant,
    writer: impl Write,
) -> Result<Option<crate::AdmittedRequest>, ExitCode> {
    cli.command
        .as_ref()
        .map(request::admit)
        .transpose()
        .map(Option::flatten)
        .map_err(|failure| report_early(&failure, wants_facts, (started, accepted), writer))
}
