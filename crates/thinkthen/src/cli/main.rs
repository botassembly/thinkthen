//! The command translates shell conventions to the typed library.
mod execute;
mod grammar;
mod input;
mod invocation;
mod options;
mod render;
mod saved_run;
mod syntax;
mod table;

use crate::core::local;
use crate::{Cancel, Code, Error, Family};
use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::{Arc, Mutex, PoisonError};

pub(crate) fn main() -> ExitCode {
    let matches = syntax::parse();
    let Some((function, args)) = matches.subcommand() else {
        return ExitCode::from(2);
    };
    if function == "completions" {
        let mut bytes = Vec::new();
        if let Some(shell) = args.get_one::<clap_complete::Shell>("shell") {
            clap_complete::generate(*shell, &mut grammar::command(), "thinkthen", &mut bytes);
        }
        return match io::stdout().write_all(&bytes) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::from(141),
            Err(_) => ExitCode::from(5),
        };
    }
    if function == "man" {
        return match clap_mangen::Man::new(grammar::command()).render(&mut io::stdout()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::from(141),
            Err(_) => ExitCode::from(5),
        };
    }
    let cancel = Cancel::default();
    let signal = cancel.clone();
    let interrupt_notice = Arc::new(Mutex::new(None));
    let handler_notice = interrupt_notice.clone();
    let result = ctrlc::set_handler(move || {
        if signal.is_cancelled() {
            for cleanup in [crate::runs::force_cleanup(), crate::files::force_cleanup()] {
                if let Err(error) = cleanup {
                    report(&error);
                }
            }
            std::process::exit(130);
        }
        // Publish the notice writer before cancellation can finish main. Keep
        // this callback free to handle another interrupt if stderr blocks.
        let mut notice = handler_notice
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        signal.cancel();
        let (finished, completion) = std::sync::mpsc::channel();
        *notice = Some(completion);
        std::thread::spawn(move || {
            let _ = writeln!(
                io::stderr().lock(),
                "thinkthen: cancelling; press Ctrl-C again to exit"
            );
            let _ = finished.send(());
        });
    })
    .map_err(|_| local("the interrupt handler could not be installed"))
    .and_then(|()| execute::invocation(function, args, &cancel));
    match result {
        _ if cancel.is_cancelled() => {
            let notice = interrupt_notice
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();
            if let Some(notice) = notice {
                let _ = notice.recv_timeout(std::time::Duration::from_secs(1));
            }
            ExitCode::from(130)
        }
        Err(error) if broken_pipe(&error) => ExitCode::from(141),
        Err(error) if error.code() == Code::LocalCancelled => ExitCode::from(130),
        Err(error) => {
            report(&error);
            ExitCode::from(family_code(error.code().family()))
        }
        Ok(code) => ExitCode::from(code),
    }
}

fn output_error(error: io::Error, cancel: &Cancel) -> Error {
    if cancel.is_cancelled() {
        return crate::retry::cancelled();
    }
    let mut failure = local("standard output could not be written");
    failure.source = Some(std::sync::Arc::new(error));
    failure
}

fn broken_pipe(error: &Error) -> bool {
    std::error::Error::source(error)
        .and_then(|source| source.downcast_ref::<io::Error>())
        .is_some_and(|source| source.kind() == io::ErrorKind::BrokenPipe)
}

/// A text line's evidence excludes its line ending, while its original keeps it.
fn evidence(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

fn report(error: &Error) {
    let _ = writeln!(
        io::stderr().lock(),
        "thinkthen: {}: {error}",
        error.code().as_str()
    );
}
const fn family_code(family: Family) -> u8 {
    match family {
        Family::Usage => 2,
        Family::Record => 6,
        Family::Backend => 4,
        Family::Local => 5,
        Family::Internal => 5,
    }
}
