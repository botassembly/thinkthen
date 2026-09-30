//! Owned controls at the Polars expression boundary. Each evaluation morsel
//! starts a fresh eager call, while its token and tally remain shared.

use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::Duration;

use polars::prelude::{Column, DataType, Expr, Field, IntoSeries, PolarsError};

use super::{PolarsCallOptions, PolarsEngine, column::kind_word};
use crate::public::{
    BatchSetting, CallOptions, CancelToken, Description, Engine, Error, Question, QuestionKind,
    Tally,
};

/// Controls captured by a lazy Polars expression. Clones of the token and
/// tally share their state; a deadline starts anew for every evaluated morsel.
#[derive(Clone, Default)]
pub struct PolarsExprOptions {
    probability: bool,
    threshold: Option<String>,
    yes: Option<Description>,
    no: Option<Description>,
    context: Option<String>,
    batch: Option<BatchSetting>,
    token: Option<CancelToken>,
    deadline: Option<Duration>,
    tally: Option<Tally>,
}

impl fmt::Debug for PolarsExprOptions {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolarsExprOptions")
            .field("probability", &self.probability)
            .field("threshold", &self.threshold.is_some())
            .field("meanings", &(self.yes.is_some() || self.no.is_some()))
            .field("context", &self.context.is_some())
            .field("batch", &self.batch)
            .field("token", &self.token.is_some())
            .field("deadline", &self.deadline)
            .field("tally", &self.tally.is_some())
            .finish()
    }
}

impl PolarsExprOptions {
    /// Start with no overrides; the engine's default Max packing remains.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return selected probability with decide or choose as a Struct.
    #[must_use]
    pub fn probability(mut self, value: bool) -> Self {
        self.probability = value;
        self
    }

    /// Apply the shared threshold grammar when the expression is built.
    #[must_use]
    pub fn threshold(mut self, value: impl Into<String>) -> Self {
        self.threshold = Some(value.into());
        self
    }

    /// Override a decide question's true meaning.
    #[must_use]
    pub fn true_meaning(mut self, value: Description) -> Self {
        self.yes = Some(value);
        self
    }

    /// Override a decide question's false meaning.
    #[must_use]
    pub fn false_meaning(mut self, value: Description) -> Self {
        self.no = Some(value);
        self
    }

    /// Include shared nonblank context in each morsel's packed requests.
    #[must_use]
    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Select a batch cut; absent means the engine's default Max.
    #[must_use]
    pub fn batch(mut self, value: BatchSetting) -> Self {
        self.batch = Some(value);
        self
    }

    /// Stop all later morsels and requests when this shared token fires.
    #[must_use]
    pub fn token(mut self, value: CancelToken) -> Self {
        self.token = Some(value);
        self
    }

    /// Bound each morsel from its own call start, not the whole query.
    ///
    /// # Errors
    /// Refuses a duration above the engine's accepted maximum.
    pub fn deadline_after(mut self, value: Duration) -> Result<Self, Error> {
        CallOptions::new().deadline_after(value)?;
        self.deadline = Some(value);
        Ok(self)
    }

    /// Add each completed or partially failed morsel to this shared tally.
    #[must_use]
    pub fn tally(mut self, value: Tally) -> Self {
        self.tally = Some(value);
        self
    }

    fn question(&self, source: &Question) -> Result<Question, Error> {
        let mut controls = PolarsCallOptions::new();
        if let Some(value) = &self.threshold {
            controls = controls.threshold(value);
        }
        if let Some(value) = &self.yes {
            controls = controls.true_meaning(value);
        }
        if let Some(value) = &self.no {
            controls = controls.false_meaning(value);
        }
        controls.apply(source).map(|(question, _, _)| question)
    }

    fn call(&self) -> Result<CallOptions<'_>, Error> {
        let mut call = CallOptions::new();
        if let Some(token) = &self.token {
            call = call.cancel(token);
        }
        if let Some(batch) = self.batch {
            call = call.batch(batch);
        }
        if let Some(context) = &self.context {
            call = call.context(context);
        }
        if let Some(deadline) = self.deadline {
            call = call.deadline_after(deadline)?;
        }
        Ok(call)
    }
}

/// Reject invalid controls before Polars can defer the failure until collect.
pub(super) fn expression(
    engine: &Engine,
    wanted: QuestionKind,
    question: &Question,
    input: Expr,
    controls: PolarsExprOptions,
) -> Result<Expr, Error> {
    if question.kind() != wanted {
        return Err(Error::usage(format!(
            "{}_expr needs a {} question, and this one is a {} question",
            kind_word(wanted),
            kind_word(wanted),
            kind_word(question.kind())
        )));
    }
    if controls.probability && !matches!(wanted, QuestionKind::Decide | QuestionKind::Choose) {
        return Err(Error::usage(format!(
            "probability belongs to decide and choose, not {}",
            kind_word(wanted)
        )));
    }
    let question = controls.question(question)?;
    if let Some(context) = &controls.context {
        crate::public::engine::evidence(context)?;
    }
    controls.call()?;
    let dtype = match (wanted, controls.probability) {
        (QuestionKind::Decide, false) => DataType::Boolean,
        (QuestionKind::Choose, false) => DataType::String,
        (QuestionKind::Score, false) => DataType::Float64,
        (QuestionKind::Tag, false) => DataType::List(Box::new(DataType::String)),
        (QuestionKind::Decide, true) => probability_dtype(DataType::Boolean),
        (QuestionKind::Choose, true) => probability_dtype(DataType::String),
        _ => return Err(Error::defect("an expression kind changed")),
    };
    let engine = engine.clone();
    Ok(input.map(
        move |column| caught(|| evaluate(&engine, &question, &controls, column)),
        move |_, field| Ok(Field::new(field.name().clone(), dtype.clone())),
    ))
}

fn probability_dtype(value: DataType) -> DataType {
    DataType::Struct(vec![
        Field::new("value".into(), value),
        Field::new("probability".into(), DataType::Float64),
    ])
}

fn evaluate(
    engine: &Engine,
    question: &Question,
    controls: &PolarsExprOptions,
    column: Column,
) -> Result<Column, PolarsError> {
    let series = column.as_materialized_series();
    let call = || {
        engine.column_with(
            question,
            series,
            PolarsCallOptions::new()
                .call(controls.call()?)
                .probability(controls.probability),
        )
    };
    let answered = match &controls.tally {
        Some(tally) => tally.run(call),
        None => call(),
    }
    .map_err(compute_error)?;
    let frame = answered.into_value();
    if controls.probability {
        Ok(frame
            .into_struct(column.name().clone())
            .into_series()
            .into())
    } else {
        frame
            .columns()
            .first()
            .cloned()
            .ok_or_else(|| PolarsError::ComputeError("a frame value was missing".into()))
    }
}

fn compute_error(error: Error) -> PolarsError {
    PolarsError::ComputeError(error.to_string().into())
}

fn caught<T>(call: impl FnOnce() -> Result<T, PolarsError>) -> Result<T, PolarsError> {
    catch_unwind(AssertUnwindSafe(call)).unwrap_or_else(|_| {
        Err(PolarsError::ComputeError(
            "thinkthen expression panicked".into(),
        ))
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_panic_stays_inside_the_polars_compute_boundary() {
        let stopped = super::caught::<()>(|| panic!("private callback content"))
            .expect_err("a compute error");
        assert_eq!(stopped.to_string(), "thinkthen expression panicked");
    }
}
