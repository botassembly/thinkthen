//! Translate parser failures using the grammar's own accepted names.
use super::{grammar, options};
use clap::ArgMatches;
use clap::error::{ContextKind, ContextValue, ErrorKind};

pub(super) fn parse() -> ArgMatches {
    let color = options::diagnostic_color();
    let mut command = grammar::command().color(color);
    if std::env::args_os().len() == 1 {
        use std::io::Write;
        let _ = command.print_help();
        let _ = writeln!(std::io::stdout().lock());
        std::process::exit(0);
    }
    match command.try_get_matches_from_mut(std::env::args_os()) {
        Ok(matches) => matches,
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            error.exit()
        }
        Err(mut error)
            if matches!(
                error.kind(),
                ErrorKind::MissingRequiredArgument
                    | ErrorKind::TooFewValues
                    | ErrorKind::WrongNumberOfValues
            ) || (error.kind() == ErrorKind::InvalidValue
                && context(&error, ContextKind::InvalidValue)
                    .is_some_and(|value| value.is_empty())) =>
        {
            let usage = active(&command).clone().render_usage();
            error.insert(ContextKind::Usage, ContextValue::StyledStr(usage));
            error.exit()
        }
        Err(error) => {
            let active = active(&command);
            let field = context(&error, ContextKind::InvalidArg)
                .or_else(|| context(&error, ContextKind::InvalidSubcommand))
                .unwrap_or_else(|| "command".into());
            let failure = if error.kind() == ErrorKind::UnknownArgument && field.starts_with('-') {
                let names: Vec<_> = active
                    .get_arguments()
                    .filter_map(|arg| arg.get_long().map(|name| format!("--{name}")))
                    .collect();
                options::unknown(
                    "option",
                    field.split('=').next().unwrap_or(&field),
                    &names.iter().map(String::as_str).collect::<Vec<_>>(),
                )
            } else if error.kind() == ErrorKind::InvalidSubcommand {
                let names: Vec<_> = command
                    .get_subcommands()
                    .map(clap::Command::get_name)
                    .collect();
                options::unknown("command", &field, &names)
            } else if let Some(ContextValue::Strings(values)) = error.get(ContextKind::ValidValue) {
                let name = context(&error, ContextKind::InvalidValue).unwrap_or_default();
                options::unknown_value(
                    &field,
                    &name,
                    &values.iter().map(String::as_str).collect::<Vec<_>>(),
                )
            } else {
                let (expected, received) = match error.kind() {
                    ErrorKind::MissingRequiredArgument => ("required arguments", "absent argument"),
                    ErrorKind::ArgumentConflict => {
                        ("compatible arguments", "conflicting arguments")
                    }
                    ErrorKind::TooManyValues => {
                        ("the declared number of values", "too many values")
                    }
                    ErrorKind::TooFewValues | ErrorKind::WrongNumberOfValues => {
                        ("the declared number of values", "too few values")
                    }
                    _ => (
                        "arguments matching the command grammar",
                        "invalid arguments (contents redacted)",
                    ),
                };
                options::usage(
                    if error.kind() == ErrorKind::UnknownArgument {
                        "command"
                    } else {
                        &field
                    },
                    expected,
                    received,
                )
            };
            if color == clap::ColorChoice::Always {
                use std::io::Write;
                let _ = writeln!(
                    std::io::stderr().lock(),
                    "thinkthen: \x1b[31m{}\x1b[0m: {failure}",
                    failure.code().as_str()
                );
            } else {
                super::report(&failure);
            }
            std::process::exit(2)
        }
    }
}

fn context(error: &clap::Error, kind: ContextKind) -> Option<String> {
    error.get(kind).map(ToString::to_string)
}

fn active(command: &clap::Command) -> &clap::Command {
    let partial = command
        .clone()
        .ignore_errors(true)
        .try_get_matches_from(std::env::args_os())
        .ok();
    partial
        .as_ref()
        .and_then(ArgMatches::subcommand_name)
        .and_then(|name| command.find_subcommand(name))
        .unwrap_or(command)
}
