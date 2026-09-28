//! Command output around the engine-owned ordinary-record scheduler.

use std::fmt;
use std::io::Write;
use std::process::ExitCode;
use std::sync::mpsc::Receiver;
use std::thread;

use crate::core::{Outcome, Withheld, ranking};
use crate::edge;
use crate::engine::error::Error as EngineError;
use crate::engine::facade::{Completed, Engine, Input, InputPort, RecordFlow, RunOutcome};
use crate::engine::usage::Counters;
use crate::engine::{Width, Widths};
use crate::failure::Failure;
use crate::profile::Mismatch;

type Asking<'a, T> = dyn Fn(&T) -> Result<Judged, Failure> + Sync + 'a;

/// A command failure and the input line it names, when one is known.
pub(crate) struct Placed {
    pub(crate) cause: Failure,
    pub(crate) at: Option<usize>,
}

impl Placed {
    pub(crate) fn at(cause: Failure, at: usize) -> Self {
        Self {
            cause,
            at: Some(at),
        }
    }
}

impl From<EngineError> for Placed {
    fn from(error: EngineError) -> Self {
        Self {
            cause: error.into(),
            at: None,
        }
    }
}

impl From<Failure> for Placed {
    fn from(cause: Failure) -> Self {
        Self { cause, at: None }
    }
}

/// One record's answer, as the line it prints and what the run counts.
pub(crate) struct Judged {
    pub(crate) printed: Option<String>,
    pub(crate) outcome: Outcome,
    pub(crate) replayed: bool,
    pub(crate) probability: Option<f64>,
    pub(crate) partial_failure: bool,
    pub(crate) profile_mismatch: Option<Mismatch>,
}

impl Judged {
    /// One record's row, as the scheduler counts it.
    pub(crate) fn completed<E>(self) -> Completed<Self, E> {
        let (replayed, partial) = (self.replayed, self.partial_failure);
        Completed::one(self, replayed, partial)
    }
}

impl fmt::Debug for Judged {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Judged")
            .field(
                "printed",
                &Withheld(self.printed.as_ref().map_or(0, String::len)),
            )
            .field("outcome", &self.outcome)
            .field("replayed", &self.replayed)
            .field("probability", &self.probability)
            .field("partial_failure", &self.partial_failure)
            .field(
                "profile_warning",
                &self
                    .profile_mismatch
                    .as_ref()
                    .is_some_and(|mismatch| mismatch.warning().is_some()),
            )
            .field(
                "batch_warning",
                &self
                    .profile_mismatch
                    .as_ref()
                    .is_some_and(|mismatch| mismatch.batch_warning().is_some()),
            )
            .finish()
    }
}

/// Where the command sends engine results in their ordered callback.
pub(crate) struct Output<'a> {
    mode: Mode<'a>,
    usage: &'a Counters,
}

enum Mode<'a> {
    Streaming(&'a mut dyn Write),
    Ordered {
        held: Vec<Judged>,
        top: Option<usize>,
        missing_probability: bool,
        writer: &'a mut dyn Write,
    },
}

fn keep_top(held: &mut Vec<Judged>, limit: usize, judged: Judged, missing: &mut bool) {
    let Some(probability) = judged.probability else {
        *missing = true;
        return;
    };
    let place = held.partition_point(|earlier| {
        earlier
            .probability
            .is_some_and(|score| score.total_cmp(&probability).is_ge())
    });
    if place >= limit {
        return;
    }
    if held.len() == limit {
        held.pop();
    }
    held.insert(place, judged);
}

impl fmt::Debug for Output<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.mode {
            Mode::Streaming(_) => formatter.write_str("Streaming"),
            Mode::Ordered { held, top, .. } => formatter
                .debug_struct("Ordered")
                .field("held", &format_args!("<{} rows withheld>", held.len()))
                .field("top", top)
                .finish_non_exhaustive(),
        }
    }
}

impl Output<'_> {
    pub(crate) fn streaming<'a>(writer: &'a mut dyn Write, usage: &'a Counters) -> Output<'a> {
        Output {
            mode: Mode::Streaming(writer),
            usage,
        }
    }

    pub(crate) fn ordered<'a>(
        writer: &'a mut dyn Write,
        top: Option<usize>,
        usage: &'a Counters,
    ) -> Output<'a> {
        Output {
            mode: Mode::Ordered {
                held: Vec::new(),
                top,
                missing_probability: false,
                writer,
            },
            usage,
        }
    }

    pub(crate) fn take(&mut self, judged: Judged) -> Result<bool, Failure> {
        let result = match &mut self.mode {
            Mode::Streaming(writer) => {
                if let Some(mismatch) = &judged.profile_mismatch {
                    mismatch.print_once()?;
                }
                match judged.printed.as_deref() {
                    Some(line) => edge::write_line(&mut **writer, line),
                    None => Ok(true),
                }
            }
            Mode::Ordered {
                held,
                top,
                missing_probability,
                ..
            } => {
                match top {
                    Some(limit) => keep_top(held, *limit, judged, missing_probability),
                    None => held.push(judged),
                }
                Ok(true)
            }
        };
        if result.is_ok() {
            self.usage.record_done();
        }
        result
    }

    pub(crate) fn ended(&mut self) -> Result<(), Failure> {
        let Mode::Ordered {
            held,
            top,
            missing_probability,
            writer,
        } = &mut self.mode
        else {
            return Ok(());
        };
        if *missing_probability {
            return Err(Failure::Defect("a ranked row carries no probability"));
        }
        let odds = held
            .iter()
            .map(|judged| {
                judged
                    .probability
                    .ok_or(Failure::Defect("a ranked row carries no probability"))
            })
            .collect::<Result<Vec<f64>, Failure>>()?;
        for place in ranking(&odds, *top) {
            let Some(line) = held.get(place).and_then(|judged| judged.printed.as_deref()) else {
                continue;
            };
            if let Some(mismatch) = held
                .get(place)
                .and_then(|judged| judged.profile_mismatch.as_ref())
            {
                mismatch.print_once()?;
            }
            if !edge::write_line(&mut **writer, line)? {
                return Ok(());
            }
        }
        Ok(())
    }

    pub(crate) fn writer(&mut self) -> &mut dyn Write {
        match &mut self.mode {
            Mode::Streaming(writer) | Mode::Ordered { writer, .. } => *writer,
        }
    }

    pub(crate) const fn flow(&self) -> RecordFlow {
        match self.mode {
            Mode::Streaming(_) => RecordFlow::Streaming,
            Mode::Ordered { top: None, .. } => RecordFlow::HeldAll,
            Mode::Ordered { top: Some(_), .. } => RecordFlow::HeldWindowed,
        }
    }
}

pub(crate) fn jobs_of(asked: Option<u8>, streams: bool) -> Result<usize, Failure> {
    match asked {
        Some(_) if !streams => Err(Failure::JobsOutsideRecords),
        asked => width(asked),
    }
}

/// Register this command's `--jobs` with the process and return the width
/// its calls follow.
pub(crate) fn width(asked: Option<u8>) -> Result<usize, Failure> {
    width_in(crate::engine::process_width(), asked)
}

/// `--jobs N` selects N. An omitted `--jobs` selects nothing and follows the
/// width the process runs at, which is 4 until something selects another.
pub(crate) fn width_in(widths: &Widths, asked: Option<u8>) -> Result<usize, Failure> {
    let asked = asked.map(|jobs| Width::new(u64::from(jobs))).transpose()?;
    widths
        .select(asked)
        .map(Width::get)
        .map_err(Failure::WidthActive)
}

pub(crate) fn over_records<T, I>(
    engine: &Engine,
    row: &Asking<'_, T>,
    chunks: I,
    cancel: &crate::engine::Cancel,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure>
where
    T: Send + 'static,
    I: Iterator<Item = Result<(usize, T), Placed>> + Send + 'static,
{
    let recording = engine.recording();
    let flow = output.flow();
    let outcome = engine.records(
        flow,
        cancel,
        |requests, events| {
            thread::spawn(move || read_records(chunks, &requests, &events));
        },
        &|(at, value)| {
            row(value)
                .map(Judged::completed)
                .map_err(|error| Placed::at(error, *at))
        },
        |judged| output.take(judged).map_err(Placed::from),
    )?;
    ended(outcome, recording, output)
}

/// The exit code of a run that completed, or the failure that stopped it.
pub(crate) fn ended(
    outcome: RunOutcome<Placed>,
    recording: bool,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    match outcome {
        RunOutcome::Complete => {
            output.ended()?;
            Ok(ExitCode::SUCCESS)
        }
        RunOutcome::Stopped {
            finished,
            replayed,
            held,
            cause,
        } => Err(Failure::Stopped {
            at: cause.at.unwrap_or(finished + 1),
            finished,
            replayed,
            recording,
            held,
            cause: Box::new(cause.cause),
        }),
    }
}

fn read_records<T, I>(
    mut chunks: I,
    requests: &Receiver<()>,
    events: &InputPort<(usize, T), Judged, Placed>,
) where
    I: Iterator<Item = Result<(usize, T), Placed>>,
{
    while requests.recv().is_ok() {
        let event = match chunks.next() {
            None => Input::End,
            Some(Ok(value)) => Input::Item(value),
            Some(Err(error)) => Input::Failed(error),
        };
        if events.send(event).is_err() {
            return;
        }
    }
}

#[cfg(test)]
mod top_tests;
#[cfg(test)]
pub(crate) mod width_tests;
