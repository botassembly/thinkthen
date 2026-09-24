//! What a backend said, once an adapter has read its response.

use serde::Serialize;

use crate::core::answer::Answer;
use crate::core::result::Usage;
use crate::core::text::ModelName;

/// The model that answered, one answer per planned question, and the usage.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Reply {
    model: ModelName,
    outcomes: Vec<AnswerOutcome>,
    usage: Option<Usage>,
}

impl Reply {
    /// Gather what one adapter read from one response body.
    pub(crate) fn new(
        model: ModelName,
        outcomes: Vec<AnswerOutcome>,
        usage: Option<Usage>,
    ) -> Self {
        Self {
            model,
            outcomes,
            usage,
        }
    }

    /// Read the model the backend reported.
    #[must_use]
    pub(crate) const fn model(&self) -> &ModelName {
        &self.model
    }

    /// Read every logical outcome in plan order, including partial failures.
    #[must_use]
    pub(crate) fn outcomes(&self) -> &[AnswerOutcome] {
        &self.outcomes
    }

    /// Read the usage the backend reported, or `None` when it reported none.
    #[must_use]
    pub(crate) const fn usage(&self) -> Option<Usage> {
        self.usage
    }
}

/// One logical question decoded from a backend reply.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AnswerOutcome {
    /// The backend returned a usable answer.
    Answered(Answer),
    /// The backend failed this question while answering another one.
    Failed(BackendFailure),
}

/// The backend failure carried by one failed logical question.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct BackendFailure {
    kind: BackendFailureKind,
    cause: BackendFailureCause,
}

impl BackendFailure {
    pub(crate) const fn new(cause: BackendFailureCause) -> Self {
        Self {
            kind: BackendFailureKind::Backend,
            cause,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum BackendFailureKind {
    Backend,
}

/// The closed causes for a failed logical question.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BackendFailureCause {
    /// The response omitted the wire answer.
    MissingAnswer,
    /// The response used a shape the question did not ask for.
    WrongKind,
    /// An option or level has no probability.
    MissingProbability,
    /// A reported probability falls outside zero to one.
    InvalidProbability,
    /// A distribution does not total one within the adapter tolerance.
    InvalidDistribution,
    /// A distribution contains an option or level that was not sent.
    UnexpectedProbability,
}

/// A failed value in bare `annotate` output.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct FailedValue {
    failed: BackendFailure,
}

impl FailedValue {
    /// Mark one named value as failed.
    #[must_use]
    pub(crate) const fn new(failed: BackendFailure) -> Self {
        Self { failed }
    }

    /// The closed cause of the failed answer.
    #[must_use]
    pub(crate) const fn cause(self) -> BackendFailureCause {
        self.failed.cause
    }
}
