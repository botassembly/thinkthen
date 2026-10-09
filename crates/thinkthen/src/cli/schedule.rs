//! Command output, the command's width, and the ordered runner under the
//! many-line `recognize`.

use std::fmt;
use std::io::Write;

use crate::core::{ModelName, Outcome, Withheld};
use crate::engine::error::Error as EngineError;
use crate::engine::usage::Counters;
use crate::engine::{Width, Widths};
use crate::failure::Failure;
use crate::profile::Mismatch;

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
    writer: &'a mut dyn Write,
    display: crate::cli::display::Display,
    usage: &'a Counters,
    model_guard: bool,
    run_model: Option<ModelName>,
}

impl fmt::Debug for Output<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Streaming")
    }
}

impl Output<'_> {
    pub(crate) fn streaming<'a>(writer: &'a mut dyn Write, usage: &'a Counters) -> Output<'a> {
        Output {
            writer,
            usage,
            display: crate::cli::display::Display::default(),
            model_guard: false,
            run_model: None,
        }
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
        if let Some(mismatch) = &judged.profile_mismatch {
            mismatch.print_once()?;
        }
        let result = self.display.emit(self.writer, &judged);
        if result.is_ok() {
            self.usage.record_done();
        }
        result
    }
    pub(crate) fn writer(&mut self) -> &mut dyn Write {
        self.writer
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

#[cfg(test)]
pub(crate) mod width_tests;

#[cfg(test)]
pub(crate) mod ordered;
