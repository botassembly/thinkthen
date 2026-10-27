//! Bounded stage execution and owned child groups.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender, TrySendError};
use std::time::Duration;

use crate::core::local;
use crate::{Cancel, Engine, Error, OffsetUnit};

use crate::contract_driver::driver::{Driver, Event};
use crate::records::Record;
use crate::runs::RunEntry;
use crate::runs::entry::Step;

const POLL: Duration = Duration::from_millis(20);
type Input = Receiver<Result<String, Error>>;
type Output = SyncSender<Result<String, Error>>;

/// The completion of a saved entry, preserving an explicit command's exit status.
#[derive(Debug)]
pub enum RunCompletion {
    /// Every stage completed successfully.
    Success,
    /// A library, input, output or launch operation failed.
    Failure(Error),
    /// A function step returned a failed row and stopped the run.
    RecordFailure(Error),
    /// One declared command exited unsuccessfully.
    ChildExit {
        /// The step's ordinal, starting at one.
        step: usize,
        /// The platform's original process exit status.
        status: ExitStatus,
    },
    /// The caller cancelled the invocation.
    Cancelled,
}

struct State {
    cancel: Cancel,
    caller: Cancel,
    failure: Mutex<Option<RunCompletion>>,
    record_failure: Mutex<Option<Error>>,
}
impl State {
    fn fail(&self, failure: RunCompletion) {
        if let Ok(mut first) = self.failure.lock()
            && first.is_none()
        {
            *first = Some(failure);
        }
        self.cancel.cancel();
    }
}

/// Run an admitted entry with JSON-line records supplied through a bounded channel.
///
/// The caller owns input production and must stop it when `cancel` is set or the
/// receiver closes. The sink receives complete JSON values without line endings.
/// All stage workers and declared child groups finish before this returns.
pub fn execute(
    entry: &RunEntry,
    client: &Engine,
    cancel: Cancel,
    input: Receiver<Result<String, Error>>,
    mut sink: impl FnMut(&str) -> Result<(), Error>,
) -> RunCompletion {
    if cancel.is_cancelled() {
        return RunCompletion::Cancelled;
    }
    let state = State {
        cancel: Cancel::default(),
        caller: cancel,
        failure: Mutex::new(None),
        record_failure: Mutex::new(None),
    };
    std::thread::scope(|scope| {
        let (send, mut receiver) = mpsc::sync_channel(1);
        let shared = &state;
        if std::thread::Builder::new()
            .spawn_scoped(scope, move || forward(input, send, shared))
            .is_err()
        {
            state.fail(RunCompletion::Failure(local(
                "the run input worker could not be started",
            )));
            return;
        }
        let mut previous_filter = false;
        for (index, step) in entry.0.iter().enumerate() {
            let consume_filter = previous_filter;
            previous_filter = matches!(step, Step::Function(request) if matches!(request.question(), crate::Question::Filter(_)));
            let (send, next) = mpsc::sync_channel(1);
            let prior = std::mem::replace(&mut receiver, next);
            let state = &state;
            if std::thread::Builder::new()
                .spawn_scoped(scope, move || match step {
                    Step::Function(request) => function(
                        request,
                        client,
                        prior,
                        send,
                        state,
                        index + 1 == entry.0.len(),
                        consume_filter,
                    ),
                    Step::Shell { arguments, env } => {
                        shell(arguments, env, index + 1, prior, send, state)
                    }
                })
                .is_err()
            {
                state.fail(RunCompletion::Failure(local(
                    "a run stage could not be started",
                )));
                break;
            }
        }
        while !state.cancel.is_cancelled() {
            if state.caller.is_cancelled() {
                state.cancel.cancel();
                break;
            }
            match receiver.recv_timeout(POLL) {
                Ok(Ok(line)) => {
                    if let Err(error) = sink(&line) {
                        state.fail(RunCompletion::Failure(error));
                    }
                }
                Ok(Err(error)) => state.fail(RunCompletion::Failure(error)),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => {
                    state.cancel.cancel();
                    break;
                }
            }
        }
    });
    match state.failure.into_inner() {
        Ok(Some(failure)) => failure,
        _ if state.caller.is_cancelled() => RunCompletion::Cancelled,
        _ => state
            .record_failure
            .into_inner()
            .ok()
            .flatten()
            .map_or(RunCompletion::Success, RunCompletion::RecordFailure),
    }
}

fn forward(input: Input, output: Output, state: &State) {
    while !state.cancel.is_cancelled() {
        match input.recv_timeout(POLL) {
            Ok(Ok(line)) if line.trim().is_empty() => {}
            Ok(Ok(line)) => {
                if !send(&output, line, state) {
                    return;
                }
            }
            Ok(Err(error)) => {
                state.fail(RunCompletion::Failure(error));
                return;
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return,
        }
    }
}

fn function(
    request: &crate::Request,
    client: &Engine,
    input: Input,
    output: Output,
    state: &State,
    final_step: bool,
    consume_filter: bool,
) {
    let result = if matches!(request.question(), crate::Question::Rank(_)) {
        finite_records(&input, state, consume_filter).and_then(|records| {
            Driver::open_records(client, request, records, OffsetUnit::CodePoints)
        })
    } else {
        Driver::open(client, request, OffsetUnit::CodePoints)
    };
    let mut call = match result {
        Ok(call) => call,
        Err(error) => {
            state.fail(RunCompletion::Failure(error));
            return;
        }
    };
    let mut originals = std::collections::BTreeMap::new();
    let mut position = 0_u64;
    let mut rows = 0_u64;
    while !state.cancel.is_cancelled() {
        let event = match call.poll(POLL) {
            Ok(event) => event,
            Err(error) => {
                state.fail(RunCompletion::Failure(error));
                return;
            }
        };
        match event {
            Some(Event::NeedsInput) => match input.recv_timeout(POLL) {
                Ok(Ok(line)) => {
                    let line = if consume_filter {
                        filtered_item(&line).unwrap_or(line)
                    } else {
                        line
                    };
                    position += 1;
                    if matches!(request.question(), crate::Question::Filter(_)) {
                        originals.insert(position, line.clone());
                    }
                    if let Err(error) = call.send(Record::from_json(&line)) {
                        state.fail(RunCompletion::Failure(error));
                    }
                }
                Ok(Err(error)) => {
                    state.fail(RunCompletion::Failure(error));
                    return;
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => call.end_input(),
            },
            Some(Event::Row(row)) => {
                let failed = row.failure();
                if rows == 0
                    && let Some(error) = failed
                    && error.code() == crate::Code::BackendTransport
                {
                    state.fail(RunCompletion::Failure(error.clone()));
                    return;
                }
                rows += 1;
                if let Some(error) = failed {
                    if !final_step {
                        state.fail(RunCompletion::RecordFailure(error.clone()));
                        return;
                    }
                    if let Ok(mut first) = state.record_failure.lock()
                        && first.is_none()
                    {
                        *first = Some(error.clone());
                    }
                }
                let original = originals.remove(&row.position());
                if !final_step
                    && matches!(&row, crate::results::AnyRow::Filter(filter) if !matches!(filter.outcome(), crate::results::Outcome::Answered { answer: true, .. }))
                {
                    continue;
                }
                let line = match serde_json::to_string(&row) {
                    Ok(line) => line,
                    Err(error) => {
                        state.fail(RunCompletion::Failure(local(&error.to_string())));
                        return;
                    }
                };
                let line = if let Some(item) = original {
                    format!(
                        "{},\"item\":{item}}}",
                        line.strip_suffix('}').unwrap_or(&line)
                    )
                } else {
                    line
                };
                if !send(&output, line, state) {
                    return;
                }
            }
            Some(Event::Finished) => {
                return;
            }
            None => {}
        }
    }
    call.cancel();
}

fn finite_records(
    input: &Input,
    state: &State,
    consume_filter: bool,
) -> Result<Vec<Record>, Error> {
    let mut records = Vec::new();
    while !state.cancel.is_cancelled() {
        match input.recv_timeout(POLL) {
            Ok(Ok(line)) => {
                let line = if consume_filter {
                    filtered_item(&line).unwrap_or(line)
                } else {
                    line
                };
                records.push(Record::from_json(&line));
            }
            Ok(Err(error)) => return Err(error),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return Ok(records),
        }
    }
    Err(crate::retry::cancelled())
}

fn shell(
    args: &[String],
    env: &[String],
    ordinal: usize,
    input: Input,
    output: Output,
    state: &State,
) {
    let Some(program) = args.first() else {
        return;
    };
    let mut command = Command::new(program);
    command
        .args(args.iter().skip(1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    for key in crate::backend::BackendTable::default().key_variables() {
        if !env.iter().any(|name| name == key) {
            command.env_remove(key);
        }
    }
    let child = match crate::runs::children::OwnedChild::spawn(&mut command) {
        Ok(child) => child,
        Err(_) => {
            state.fail(RunCompletion::Failure(local(
                "the declared command could not be started",
            )));
            return;
        }
    };
    let (Some(mut stdin), Some(stdout)) = child.pipes() else {
        state.fail(RunCompletion::Failure(local(
            "the declared command pipes could not be opened",
        )));
        let _ = child.kill();
        let _ = child.wait();
        return;
    };
    let finished = AtomicBool::new(false);
    std::thread::scope(|scope| {
        let finished = &finished;
        let writing = std::thread::Builder::new().spawn_scoped(scope, move || {
            while !state.cancel.is_cancelled() && !finished.load(Ordering::Acquire) {
                match input.recv_timeout(POLL) {
                    Ok(Ok(line)) => {
                        if writeln!(stdin, "{line}").is_err() {
                            break;
                        }
                    }
                    Ok(Err(error)) => {
                        state.fail(RunCompletion::Failure(error));
                        break;
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            }
            drop(stdin);
        });
        if writing.is_err() {
            state.fail(RunCompletion::Failure(local(
                "the command input worker could not be started",
            )));
            let _ = child.kill();
            let _ = child.wait();
            return;
        }
        let reading = std::thread::Builder::new().spawn_scoped(scope, || {
            for line in BufReader::new(stdout).lines() {
                if state.cancel.is_cancelled() {
                    break;
                }
                match line {
                    Ok(line) if line.trim().is_empty() => {}
                    Ok(line) => {
                        if serde_json::from_str::<serde_json::Value>(&line).is_err() {
                            state.fail(RunCompletion::Failure(local(
                                "a declared command wrote an invalid JSON line",
                            )));
                            break;
                        }
                        if !send(&output, line, state) {
                            finished.store(true, Ordering::Release);
                            break;
                        }
                    }
                    Err(_) => {
                        state.fail(RunCompletion::Failure(local(
                            "the declared command output could not be read as UTF-8",
                        )));
                        break;
                    }
                }
            }
        });
        if reading.is_err() {
            state.fail(RunCompletion::Failure(local(
                "the command output worker could not be started",
            )));
            let _ = child.kill();
            let _ = child.wait();
            return;
        }
        loop {
            if state.cancel.is_cancelled() || finished.load(Ordering::Acquire) {
                let _ = child.kill();
                let _ = child.wait();
                break;
            }
            match child.try_wait() {
                Ok(Some(status)) => {
                    if !status.success() && !finished.load(Ordering::Acquire) {
                        state.fail(RunCompletion::ChildExit {
                            step: ordinal,
                            status,
                        });
                    }
                    finished.store(true, Ordering::Release);
                    // A command owns its descendants even when its leader has exited.
                    let _ = child.kill();
                    let _ = child.wait();
                    break;
                }
                Ok(None) => std::thread::park_timeout(POLL),
                Err(_) => {
                    state.fail(RunCompletion::Failure(local(
                        "the declared command could not be waited for",
                    )));
                    finished.store(true, Ordering::Release);
                    let _ = child.kill();
                    let _ = child.wait();
                    break;
                }
            }
        }
    });
}

fn send(output: &Output, mut line: String, state: &State) -> bool {
    while !state.cancel.is_cancelled() {
        match output.try_send(Ok(line)) {
            Ok(()) => return true,
            Err(TrySendError::Full(Ok(value))) => {
                line = value;
                std::thread::park_timeout(POLL);
            }
            Err(TrySendError::Disconnected(_)) => return false,
            Err(TrySendError::Full(Err(_))) => return false,
        }
    }
    false
}

fn filtered_item(line: &str) -> Option<String> {
    let fields = serde_json::from_str::<
        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    >(line)
    .ok()?;
    (fields.get("function")?.get() == "\"filter\"")
        .then(|| fields.get("item").map(|item| item.get().to_owned()))
        .flatten()
}
