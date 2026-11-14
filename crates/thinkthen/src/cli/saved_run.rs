//! Translate saved entries through the same framing and rendering as commands.
use super::{
    input::Input,
    options::{self, Output, usage},
};
use crate::runs::{RunCompletion, execute, load, names_entry};
use crate::{Cancel, Error};
use clap::ArgMatches;
use std::io::{self, Write};
#[cfg(not(windows))]
use std::process::ExitStatus;

/// The toolbox and entry a run names.
pub(super) struct Target {
    directory: std::path::PathBuf,
    toolbox: String,
    entry: Option<String>,
}

/// Read `run TOOLBOX [NAME] [FILE...]`, returning the target and its input files.
///
/// After a toolbox file or folder, the next word is the entry name when
/// `names_entry` says so; `--entry NAME` is the same and leaves every word as input.
pub(super) fn target(args: &ArgMatches) -> Result<(Target, Vec<String>), Error> {
    let directory = std::env::current_dir()
        .map_err(|_| super::local("the current directory could not be read"))?;
    let toolbox = args
        .get_one::<String>("question")
        .ok_or_else(|| usage("toolbox", "a toolbox", "absent argument"))?
        .clone();
    let mut files = super::input::files(args);
    let mut entry = options::explicit(args, "entry");
    if entry.is_none()
        && files
            .first()
            .is_some_and(|word| names_entry(&directory, &toolbox, word))
    {
        entry = Some(files.remove(0));
    }
    Ok((
        Target {
            directory,
            toolbox,
            entry,
        },
        files,
    ))
}

pub(super) fn run(
    args: &ArgMatches,
    target: &Target,
    mut input: Input,
    output: &Output,
    cancel: &Cancel,
    account: &crate::usage::Account,
) -> Result<u8, Error> {
    let entry = load(&target.directory, &target.toolbox, target.entry.as_deref())?
        .map_requests(|request| super::invocation::overrides(request, args))?;
    if let Some(function) = entry.final_function() {
        output.validate_function(function)?;
    }
    let shells: Vec<_> = entry.shell_steps().collect();
    if !shells.is_empty() && !args.get_flag("allow-shell") {
        let commands = shells
            .iter()
            .map(|(args, _)| format!("{args:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(Error::new(
            crate::ErrorKind::Toolbox,
            format!("shell steps require --allow-shell: {commands}"),
        ));
    }
    for (_, env) in shells {
        let keys = crate::backend::BackendTable::default();
        let passed: Vec<_> = keys
            .key_variables()
            .filter(|key| env.iter().any(|name| name == key) && std::env::var_os(key).is_some())
            .collect();
        if !passed.is_empty() {
            writeln!(
                io::stderr().lock(),
                "thinkthen: shell passes backend key variables: {}",
                passed.join(", ")
            )
            .map_err(|_| super::local("the shell notice could not be written"))?;
        }
    }
    if input.table() && entry.request().is_none() {
        return Err(usage(
            "entry",
            "a single-function entry for file framing",
            "a mixed-step run",
        ));
    }
    let engine = options::client(args, account)?;
    if let Some(request) = entry.request() {
        let request = request.clone();
        return if input.table() {
            super::table::run(&input, args, &request, &engine, cancel, output.quiet)
        } else {
            super::execute::stream(input, output, &request, &engine, cancel)
        };
    }
    if input.table() {
        return Err(usage(
            "entry",
            "a single-function entry for file framing",
            "a mixed-step run",
        ));
    }
    let receiver = super::input::spawn(
        move || {
            let Some((_, bytes, start)) = input.next()? else {
                return Ok(None);
            };
            let text = std::str::from_utf8(super::input::source_body(&bytes, start))
                .map_err(|_| super::local("input could not be read as UTF-8"))?;
            // Each step reads exactly one JSON value per line.
            let value = if input.format == super::input::Format::Jsonl {
                super::field(text).to_owned()
            } else {
                serde_json::to_string(super::field(text))
                    .map_err(|_| super::local("input could not be encoded"))?
            };
            Ok(Some(value))
        },
        cancel.clone(),
    )?;
    let final_shell = entry.final_shell();
    let result = execute(&entry, &engine, cancel.clone(), receiver, |line| {
        if output.quiet {
            return Ok(());
        }
        // The last shell step's output belongs to the caller, unchanged.
        if final_shell {
            return io::stdout()
                .lock()
                .write_all(line.as_bytes())
                .map_err(|error| super::output_error(error, cancel));
        }
        // An item is any JSON the pipeline admitted, so only the row's own fields are typed.
        let undecoded = |_| super::local("the run output could not be decoded");
        let mut fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(line).map_err(undecoded)?;
        let item = fields.remove("item");
        let row = serde_json::to_string(&fields).map_err(undecoded)?;
        let row = serde_json::from_str::<crate::results::AnyRow>(&row);
        if output.json {
            if let (Ok(row @ crate::results::AnyRow::Filter(_)), Some(item)) = (&row, &item) {
                return super::render::item_row(row, item, cancel);
            }
            return writeln!(io::stdout().lock(), "{line}")
                .map_err(|error| super::output_error(error, cancel));
        }
        if let Ok(row @ crate::results::AnyRow::Filter(_)) = &row {
            if super::render::keeps(row) {
                let item = item
                    .as_ref()
                    .ok_or_else(|| super::local("a kept record has no original item"))?;
                let text = serde_json::from_str::<String>(item.get())
                    .unwrap_or_else(|_| item.get().to_owned());
                writeln!(io::stdout().lock(), "{text}")
                    .map_err(|error| super::output_error(error, cancel))?;
            }
            return Ok(());
        }
        if let Ok(row) = row {
            super::render::row(&row, output.json, cancel)
        } else {
            writeln!(io::stdout().lock(), "{line}")
                .map_err(|error| super::output_error(error, cancel))
        }
    });
    match result {
        RunCompletion::Success => Ok(0),
        RunCompletion::Failure(error) => Err(error),
        RunCompletion::RecordFailure(error) => {
            super::report(&error);
            Ok(6)
        }
        RunCompletion::Cancelled => Ok(130),
        RunCompletion::ChildExit { step, status } => {
            super::report(&super::local(&format!(
                "run step {step} exited with {status}"
            )));
            #[cfg(windows)]
            std::process::exit(status.code().unwrap_or(5));
            #[cfg(not(windows))]
            Ok(exit_code(status))
        }
    }
}

#[cfg(not(windows))]
fn exit_code(status: ExitStatus) -> u8 {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        status
            .code()
            .and_then(|code| u8::try_from(code).ok())
            .unwrap_or_else(|| {
                status
                    .signal()
                    .and_then(|signal| u8::try_from(128 + signal).ok())
                    .unwrap_or(5)
            })
    }
    #[cfg(not(unix))]
    {
        status
            .code()
            .and_then(|code| u8::try_from(code).ok())
            .unwrap_or(5)
    }
}
