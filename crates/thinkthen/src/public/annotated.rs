//! One record's named `annotate` values, and why a failed member failed.

use std::fmt;

use crate::core::{AnnotatedValue, BackendFailureCause, Value};
use crate::public::error::ErrorKind;
use crate::public::results::{Answer, Written, answer, withheld_debug};

/// One named value of an annotated record.
#[derive(Clone, Debug, PartialEq)]
pub enum Annotated {
    /// A `decide` answer.
    Decision(Answer),
    /// A `choose` pick, or `None` when the answer is not sure.
    Choice(Option<String>),
    /// A `score` position.
    Score(f64),
    /// The `tag` labels that reached the cut.
    Tags(Vec<String>),
    /// The backend failed this one question.
    Failed(Failed),
}

/// One question of a set and its value.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedAnnotation {
    name: String,
    value: Annotated,
}

impl NamedAnnotation {
    /// The member name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Its value.
    #[must_use]
    pub fn value(&self) -> &Annotated {
        &self.value
    }
}

/// One record and every value the set gave it, in set order.
#[derive(Clone, PartialEq)]
pub struct AnnotatedRecord<T> {
    input: T,
    values: Vec<NamedAnnotation>,
    json: Written,
}

withheld_debug!(AnnotatedRecord<T> { values });

impl<T> AnnotatedRecord<T> {
    pub(crate) fn new(input: T, values: Vec<(String, AnnotatedValue)>, json: Written) -> Self {
        let values = values
            .into_iter()
            .map(|(name, value)| NamedAnnotation {
                name,
                value: match value {
                    AnnotatedValue::Answered(Value::YesNo(held)) => {
                        Annotated::Decision(answer(&Value::YesNo(held)))
                    }
                    AnnotatedValue::Answered(Value::Choice(label)) => Annotated::Choice(label),
                    AnnotatedValue::Answered(Value::Score(position)) => Annotated::Score(position),
                    AnnotatedValue::Answered(Value::Tag(labels)) => Annotated::Tags(labels),
                    AnnotatedValue::Failed(failed) => {
                        Annotated::Failed(Failed(cause(failed.cause())))
                    }
                },
            })
            .collect();
        Self {
            input,
            values,
            json,
        }
    }

    /// The record as given.
    #[must_use]
    pub fn input(&self) -> &T {
        &self.input
    }

    /// Each member's value, in set order.
    #[must_use]
    pub fn values(&self) -> &[NamedAnnotation] {
        &self.values
    }

    /// The record.
    #[must_use]
    pub fn into_input(self) -> T {
        self.input
    }

    /// The bare `annotate` value the command prints for this one text.
    #[must_use]
    pub fn value_json(&self) -> String {
        self.json.text()
    }
}

/// One question the backend failed inside an otherwise answered record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Failed(FailureCause);

impl Failed {
    /// Always [`ErrorKind::Backend`].
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        ErrorKind::Backend
    }

    /// What the backend's answer broke.
    #[must_use]
    pub fn cause(&self) -> FailureCause {
        self.0
    }
}

/// Why the backend's answer to one question could not be read.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureCause {
    /// The reply omitted the answer.
    MissingAnswer,
    /// The reply answered another kind of question.
    WrongKind,
    /// An option or level had no probability.
    MissingProbability,
    /// A probability fell outside zero to one.
    InvalidProbability,
    /// A distribution did not total one.
    InvalidDistribution,
    /// A distribution named an option that was not sent.
    UnexpectedProbability,
}

pub(crate) const fn cause(cause: BackendFailureCause) -> FailureCause {
    match cause {
        BackendFailureCause::MissingAnswer => FailureCause::MissingAnswer,
        BackendFailureCause::WrongKind => FailureCause::WrongKind,
        BackendFailureCause::MissingProbability => FailureCause::MissingProbability,
        BackendFailureCause::InvalidProbability => FailureCause::InvalidProbability,
        BackendFailureCause::InvalidDistribution => FailureCause::InvalidDistribution,
        BackendFailureCause::UnexpectedProbability => FailureCause::UnexpectedProbability,
    }
}
