//! Translate command streams to the shared saved-entry executor.

use std::io::{self, BufRead, Write};
#[cfg(not(windows))]
use std::process::ExitStatus;
use std::sync::mpsc::{self, TrySendError};
use std::time::Duration;

use clap::Args as ClapArgs;
use thinkthen::runs::{RunCompletion, execute, load};
use thinkthen::{Cancel, Client, Error};

/// Saved entry selection and the existing client configuration flags.
#[derive(Debug, ClapArgs)]
pub(super) struct Args {
    #[command(flatten)]
    table: super::table::TableArgs,
    /// A toolbox path with NAME, a single-entry .json file, or a local entry name.
    toolbox: String,
    /// An entry in the explicitly named toolbox file or folder.
    name: Option<String>,
    /// The backend that answers function steps.
    #[arg(long, default_value = "systemone")]
    adapter: String,
    /// The model to ask.
    #[arg(long)]
    model: Option<String>,
    /// The backend's base address.
    #[arg(long)]
    url: Option<String>,
    /// How many records share one request.
    #[arg(long)]
    batch: Option<serde_json::Number>,
    /// Convert each nonblank input text line to a JSON string record.
    #[arg(long)]
    lines: bool,
}

pub(super) fn run(args: Args, cancel: Cancel) -> Result<u8, Error> {
    let directory = std::env::current_dir()
        .map_err(|_| super::local("the current directory could not be read"))?;
    let entry = load(&directory, &args.toolbox, args.name.as_deref())?;
    let mut settings = serde_json::Map::from_iter([("adapter".to_owned(), args.adapter.into())]);
    for (key, value) in [("model", args.model), ("url", args.url)] {
        if let Some(value) = value {
            settings.insert(key.to_owned(), value.into());
        }
    }
    if let Some(batch) = args.batch {
        settings.insert("batch".to_owned(), batch.into());
    }
    let client = Client::from_json(&serde_json::Value::Object(settings).to_string(), |name| {
        std::env::var(name).ok()
    })?;
    if args.table.enabled() {
        return args.table.run_entry(&entry, &client, &cancel);
    }
    let (send, input) = mpsc::sync_channel(1);
    let stop = cancel.clone();
    // The command owns standard input. It can remain open when Ctrl-C arrives;
    // stage cancellation never waits for this blocking reader to receive EOF.
    std::thread::spawn(move || {
        for line in io::stdin().lock().lines() {
            let mut value = match line {
                Ok(line) if line.trim().is_empty() => continue,
                Ok(line) if args.lines => Ok(serde_json::Value::String(line).to_string()),
                Ok(line) => Ok(line),
                Err(_) => Err(super::local("standard input could not be read as UTF-8")),
            };
            let failed = value.is_err();
            while !stop.is_cancelled() {
                match send.try_send(value) {
                    Ok(()) => break,
                    Err(TrySendError::Full(item)) => {
                        value = item;
                        std::thread::park_timeout(Duration::from_millis(20));
                    }
                    Err(TrySendError::Disconnected(_)) => return,
                }
            }
            if failed || stop.is_cancelled() {
                break;
            }
        }
    });
    match execute(&entry, &client, cancel, input, |line| {
        writeln!(io::stdout().lock(), "{line}")
            .map_err(|_| super::local("standard output could not be written"))
    }) {
        RunCompletion::Success => Ok(0),
        RunCompletion::Failure(error) => Err(error),
        RunCompletion::Cancelled => Ok(130),
        RunCompletion::ChildExit { step, status } => {
            let _ = writeln!(
                io::stderr().lock(),
                "thinkthen: run step {step} exited with {status}"
            );
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
