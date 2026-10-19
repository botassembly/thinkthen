//! Command output, the command's width, and the ordered runner under the
//! many-line `recognize`.

use std::fmt;
use std::io::Write;

use crate::cli::edge::Environment;
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

impl From<crate::Error> for Placed {
    fn from(error: crate::Error) -> Self {
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

/// The command's three exit/read states, independent of native implementation types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Outcome {
    Yes,
    No,
    Unresolved,
}
impl From<crate::Answer> for Outcome {
    fn from(answer: crate::Answer) -> Self {
        match answer {
            crate::Answer::Yes => Self::Yes,
            crate::Answer::No => Self::No,
            crate::Answer::Unsure => Self::Unresolved,
        }
    }
}

/// One record's answer, as the line it prints and what the run counts.
pub(crate) struct Judged {
    pub(crate) model: Option<String>,
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
                &self.printed.as_ref().map(String::len),
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
    model_guard: bool,
    run_model: ModelGuard,
}

/// Live reply comparison shared by row output and rank observations.
#[derive(Default)]
pub(crate) struct ModelGuard(Option<String>);

impl ModelGuard {
    pub(crate) fn check_sources<'a>(
        &mut self,
        sources: impl Iterator<Item = &'a crate::QuestionSource>,
    ) -> Result<(), Failure> {
        for source in sources {
            if source.origin() == crate::Origin::Live {
                self.check_model(Some(source.answered_by()))?;
            }
        }
        Ok(())
    }

    fn check_model(&mut self, model: Option<&str>) -> Result<(), Failure> {
        // Stored answers take no part in this comparison (ADR 0111 section 4).
        if let Some(model) = model {
            match &self.0 {
                Some(first) if first.as_str() != model => return Err(Failure::RunModelsDiffer),
                None => self.0 = Some(model.to_owned()),
                Some(_) => {}
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Output<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Streaming")
    }
}

impl Output<'_> {
    pub(crate) fn streaming<'a>(writer: &'a mut dyn Write) -> Output<'a> {
        Output {
            writer,
            display: crate::cli::display::Display::default(),
            model_guard: false,
            run_model: ModelGuard::default(),
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

    fn check_model(&mut self, model: Option<&str>) -> Result<(), Failure> {
        if self.model_guard {
            self.run_model.check_model(model)?;
        }
        Ok(())
    }

    pub(crate) fn take(&mut self, judged: Judged) -> Result<bool, Failure> {
        self.print(judged)
    }

    pub(crate) fn print(&mut self, judged: Judged) -> Result<bool, Failure> {
        self.check_model(judged.model.as_deref())?;
        if let Some(mismatch) = &judged.profile_mismatch {
            mismatch.print_once()?;
        }
        self.display.emit(self.writer, &judged)
    }
    pub(crate) fn writer(&mut self) -> &mut dyn Write {
        self.writer
    }
}

pub(crate) fn jobs_of(asked: Option<u8>, streams: bool, environment: &Environment) -> Result<(), Failure> {
    if asked.is_some() && !streams {
        return Err(Failure::JobsOutsideRecords);
    }
    width(asked, environment)
}

/// Select an explicit command width at this command's original admission point.
/// Omission follows the process without reserving its fallback width.
pub(crate) fn width(asked: Option<u8>, environment: &Environment) -> Result<(), Failure> {
    if let Some(width) = asked {
        environment.update_builder(|builder| builder.select_throttle(width))?;
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod width_tests;
