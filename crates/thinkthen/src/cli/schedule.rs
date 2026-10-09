//! Command output, the command's width, and the ordered runner under the
//! many-line `recognize`.

use std::fmt;
use std::io::Write;
use std::process::ExitCode;
use std::sync::mpsc::Receiver;
use std::thread;

use crate::core::{ModelName, Outcome, Threshold, Withheld, ranking_under};
use crate::engine::error::Error as EngineError;
use crate::engine::facade::Engine;
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
    pub(crate) rank: Option<rank::RankRow>,
    pub(crate) model: Option<ModelName>,
    pub(crate) printed: Option<String>,
    pub(crate) position: Option<crate::cli::intake::Position>,
    pub(crate) outcome: Outcome,
    pub(crate) replayed: bool,
    pub(crate) order_value: Option<f64>,
    pub(crate) partial_failure: bool,
    pub(crate) profile_mismatch: Option<Mismatch>,
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
            .field("order_value", &self.order_value)
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
    display: crate::cli::display::Display,
    usage: &'a Counters,
    model_guard: bool,
    run_model: Option<ModelName>,
    members: Vec<Vec<(usize, Judged)>>,
    member_scores: Vec<Vec<(usize, f64)>>,
    rank_threshold: Option<Threshold>,
}

enum Mode<'a> {
    Streaming(&'a mut dyn Write),
    Ordered {
        held: Vec<Judged>,
        top: Option<usize>,
        missing_order_value: bool,
        writer: &'a mut dyn Write,
    },
}

fn keep_top(held: &mut Vec<Judged>, limit: usize, judged: Judged, missing: &mut bool) {
    let Some(value) = judged.order_value else {
        *missing = true;
        return;
    };
    let place = held.partition_point(|earlier| {
        earlier
            .order_value
            .is_some_and(|score| score.total_cmp(&value).is_ge())
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
            display: crate::cli::display::Display::default(),
            model_guard: false,
            run_model: None,
            members: Vec::new(),
            member_scores: Vec::new(),
            rank_threshold: None,
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
                missing_order_value: false,
                writer,
            },
            usage,
            display: crate::cli::display::Display::default(),
            model_guard: false,
            run_model: None,
            members: Vec::new(),
            member_scores: Vec::new(),
            rank_threshold: None,
        }
    }

    pub(crate) const fn rank_threshold(&mut self, threshold: Option<Threshold>) {
        self.rank_threshold = threshold;
    }

    pub(crate) fn display(&mut self, arguments: crate::cli::display::Arguments) {
        self.display.arguments = arguments;
    }

    pub(crate) fn validate_display(&mut self, common: &crate::args::Common) -> Result<(), Failure> {
        self.display.validate(common)
    }

    pub(crate) const fn neighbors(&self) -> bool {
        self.display.around.is_some()
    }

    pub(crate) fn text_view(&self) -> bool {
        self.display.arguments.line_number
            || self.display.arguments.scores
            || self.display.around.is_some()
    }

    pub(crate) fn snapshot(&mut self, snapshot: Option<crate::cli::intake::Snapshot>) {
        self.display.snapshot = snapshot;
    }

    pub(crate) fn guard_models(&mut self) {
        self.model_guard = true;
    }

    fn check_model(&mut self, judged: &Judged) -> Result<(), Failure> {
        // A row the store answered wholly names no live model, so it takes
        // no part in the check, by ADR 0111 section 4.
        if let (true, Some(model)) = (self.model_guard, judged.model.as_ref()) {
            match &self.run_model {
                Some(first) if first != model => return Err(Failure::RunModelsDiffer),
                None => self.run_model = Some(model.clone()),
                Some(_) => {}
            }
        }
        Ok(())
    }

    pub(crate) fn take(&mut self, judged: Judged) -> Result<bool, Failure> {
        self.check_model(&judged)?;
        let result = match &mut self.mode {
            Mode::Streaming(writer) => {
                if let Some(mismatch) = &judged.profile_mismatch {
                    mismatch.print_once()?;
                }
                self.display.emit(&mut **writer, &judged)
            }
            Mode::Ordered {
                held,
                top,
                missing_order_value,
                ..
            } => {
                match top {
                    Some(limit) => keep_top(held, *limit, judged, missing_order_value),
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
        if !self.members.is_empty() {
            return self.end_members();
        }
        let Mode::Ordered {
            held,
            top,
            missing_order_value,
            writer,
        } = &mut self.mode
        else {
            return Ok(());
        };
        if *missing_order_value {
            return Err(Failure::Defect("a ranked row carries no probability"));
        }
        let odds = held
            .iter()
            .map(|judged| {
                judged
                    .order_value
                    .ok_or(Failure::Defect("a ranked row carries no probability"))
            })
            .collect::<Result<Vec<f64>, Failure>>()?;
        for (at, place) in ranking_under(&odds, *top, self.rank_threshold)
            .into_iter()
            .enumerate()
        {
            let Some(judged) = held.get_mut(place) else {
                continue;
            };
            judged.finish_rank(at, None)?;
            if let Some(mismatch) = judged.profile_mismatch.as_ref() {
                mismatch.print_once()?;
            }
            if !self.display.emit(&mut **writer, judged)? {
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
    let cancel = crate::engine::Cancel::default();
    let limits = crate::engine::limits::of(std::process::id(), &cancel)?;
    width_in(&limits.widths, asked)
}

/// `--jobs N` selects N. An omitted `--jobs` selects nothing and follows the
/// width the process runs at, which is 8 until something selects another.
pub(crate) fn width_in(widths: &Widths, asked: Option<u8>) -> Result<usize, Failure> {
    let asked = asked.map(|jobs| Width::new(u64::from(jobs))).transpose()?;
    widths
        .select(asked)
        .map(Width::get)
        .map_err(Failure::WidthActive)
}

/// Answer each record of `chunks` with `row` over the engine's width and
/// print the rows in input order. Only the many-line `recognize` runs this
/// way; every record function asks through the question pipeline.
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
    let outcome = ordered::run(
        engine.width(cancel)?,
        cancel,
        |requests, events| {
            thread::spawn(move || read_records(chunks, &requests, &events));
        },
        &|(at, value): (usize, T)| {
            row(&value)
                .map(|judged| ordered::Row {
                    replayed: judged.replayed,
                    value: judged,
                })
                .map_err(|error| Placed::at(error, at))
        },
        |judged| output.take(judged).map_err(Placed::from),
        &|error: EngineError| Placed::from(error),
        || Placed::from(Failure::Defect("the record reader ended early")),
    )?;
    match outcome {
        ordered::Outcome::Complete => {
            output.ended()?;
            Ok(ExitCode::SUCCESS)
        }
        ordered::Outcome::Stopped {
            finished,
            replayed,
            cause,
        } => Err(Failure::Stopped {
            at: cause.at.unwrap_or(finished + 1),
            finished,
            replayed,
            recording: engine.recording(),
            held: false,
            cause: Box::new(cause.cause),
        }),
    }
}

fn read_records<T, I>(
    mut chunks: I,
    requests: &Receiver<()>,
    events: &ordered::Port<(usize, T), Judged, Placed>,
) where
    I: Iterator<Item = Result<(usize, T), Placed>>,
{
    while requests.recv().is_ok() {
        let event = match chunks.next() {
            None => ordered::Input::End,
            Some(Ok(value)) => ordered::Input::Item(value),
            Some(Err(error)) => ordered::Input::Failed(error),
        };
        if events.send(event).is_err() {
            return;
        }
    }
}

pub(super) mod ordered;
#[cfg(test)]
mod top_tests;
#[cfg(test)]
pub(crate) mod width_tests;

mod rank_set;

#[cfg(test)]
mod rank_set_tests;

pub(crate) mod rank;
