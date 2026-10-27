//! Translate saved entries through the same framing and rendering as commands.
use super::{
    input::Input,
    options::{self, Output, usage},
};
use crate::runs::{RunCompletion, execute, load};
use crate::{Cancel, Error};
use clap::ArgMatches;
use std::io::{self, Write};
#[cfg(not(windows))]
use std::process::ExitStatus;

pub(super) fn run(
    args: &ArgMatches,
    mut input: Input,
    output: &Output,
    cancel: &Cancel,
) -> Result<u8, Error> {
    let directory = std::env::current_dir()
        .map_err(|_| super::local("the current directory could not be read"))?;
    let toolbox = args
        .get_one::<String>("question")
        .ok_or_else(|| usage("toolbox", "a toolbox", "absent argument"))?;
    let entry = load(
        &directory,
        toolbox,
        options::value(args, "entry").as_deref(),
    )?
    .map_requests(|request| super::invocation::overrides(request, args))?;
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
    let engine = options::client(args)?;
    if let Some(request) = entry.request() {
        let request = request.clone();
        return if input.table() {
            super::table::run(&input, args, &request, &engine, cancel)
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
            let Some((_, bytes)) = input.next()? else {
                return Ok(None);
            };
            let text = String::from_utf8(bytes)
                .map_err(|_| super::local("input could not be read as UTF-8"))?;
            let value = if input.format == super::input::Format::Jsonl {
                text
            } else {
                serde_json::to_string(super::evidence(&text))
                    .map_err(|_| super::local("input could not be encoded"))?
            };
            Ok(Some(value))
        },
        cancel.clone(),
    )?;
    let result = execute(&entry, &engine, cancel.clone(), receiver, |line| {
        // Shell outputs remain arbitrary JSON; final typed rows use command rendering.
        let mut value: serde_json::Value = serde_json::from_str(line)
            .map_err(|_| super::local("the run output could not be decoded"))?;
        if let Some(fields) = value.as_object_mut() {
            fields.remove("item");
        }
        let row = serde_json::from_value::<crate::results::AnyRow>(value);
        if output.json {
            if let Ok(row @ crate::results::AnyRow::Filter(_)) = row {
                let fields: std::collections::BTreeMap<&str, &serde_json::value::RawValue> =
                    serde_json::from_str(line)
                        .map_err(|_| super::local("the run output could not be decoded"))?;
                if let Some(item) = fields.get("item") {
                    return super::render::filter_row(&row, item, cancel);
                }
            }
            return writeln!(io::stdout().lock(), "{line}")
                .map_err(|error| super::output_error(error, cancel));
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
