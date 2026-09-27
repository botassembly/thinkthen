//! Command output around the engine-owned ordinary-record scheduler.

use std::fmt;
use std::io::Write;
use std::process::ExitCode;
use std::sync::mpsc::Receiver;
use std::thread;

use crate::core::{Outcome, Withheld, ranking};
use crate::edge;
use crate::engine::facade::{Completed, Engine, Input, InputPort, RunOutcome};
use crate::engine::{Width, Widths};
use crate::failure::Failure;
use crate::profile::Mismatch;

type Asking<'a, T> = dyn Fn(&T) -> Result<Judged, Failure> + Sync + 'a;

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
    pub(crate) fn completed(self) -> Completed<Self, Failure> {
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
            .field("profile_warning", &self.profile_mismatch.is_some())
            .finish()
    }
}

/// Where the command sends engine results in their ordered callback.
pub(crate) enum Output<'a> {
    Streaming(&'a mut dyn Write),
    Ordered {
        held: Vec<Judged>,
        top: Option<usize>,
        writer: &'a mut dyn Write,
    },
}

impl fmt::Debug for Output<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Streaming(_) => formatter.write_str("Streaming"),
            Self::Ordered { held, top, .. } => formatter
                .debug_struct("Ordered")
                .field("held", &format_args!("<{} rows withheld>", held.len()))
                .field("top", top)
                .finish_non_exhaustive(),
        }
    }
}

impl Output<'_> {
    pub(crate) fn take(&mut self, judged: Judged) -> Result<bool, Failure> {
        match self {
            Self::Streaming(writer) => {
                if let Some(mismatch) = &judged.profile_mismatch {
                    mismatch.print_once()?;
                }
                match judged.printed.as_deref() {
                    Some(line) => edge::write_line(&mut **writer, line),
                    None => Ok(true),
                }
            }
            Self::Ordered { held, .. } => {
                held.push(judged);
                Ok(true)
            }
        }
    }

    pub(crate) fn ended(&mut self) -> Result<(), Failure> {
        let Self::Ordered { held, top, writer } = self else {
            return Ok(());
        };
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
        match self {
            Self::Streaming(writer) | Self::Ordered { writer, .. } => *writer,
        }
    }

    pub(crate) const fn holds(&self) -> bool {
        matches!(*self, Self::Ordered { .. })
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
    I: Iterator<Item = Result<T, Failure>> + Send + 'static,
{
    let recording = engine.recording();
    let held = output.holds();
    let outcome = engine.records(
        held,
        cancel,
        |requests, events| {
            thread::spawn(move || read_records(chunks, &requests, &events));
        },
        &|value| {
            row(value).map(Judged::completed)
        },
        |judged| output.take(judged),
    )?;
    ended(outcome, recording, output)
}

/// The exit code of a run that completed, or the failure that stopped it.
pub(crate) fn ended(
    outcome: RunOutcome<Failure>,
    recording: bool,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    match outcome {
        RunOutcome::Complete => {
            output.ended()?;
            Ok(ExitCode::SUCCESS)
        }
        RunOutcome::Stopped {
            at,
            finished,
            replayed,
            held,
            cause,
        } => Err(Failure::Stopped {
            at,
            finished,
            replayed,
            recording,
            held,
            cause: Box::new(cause),
        }),
    }
}

fn read_records<T, I>(
    mut chunks: I,
    requests: &Receiver<()>,
    events: &InputPort<T, Judged, Failure>,
) where
    I: Iterator<Item = Result<T, Failure>>,
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
pub(crate) mod width_tests;
