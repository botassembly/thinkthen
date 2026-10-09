//! Independent owned packets from a request worker.
use super::RequestValue;
use crate::{
    CompleteAnnotated, CompleteChoice, CompleteDecision, CompleteFilter, CompleteRecord,
    CompleteScore, CompleteTags, Error, Facts, OwnedRecordObservation, QuestionInput,
};

/// One actual completed row, aggregate, observation or settled terminal.
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "owned packets preserve the admitted native enum API without borrowed backing"
)]
pub enum RequestSessionResult {
    /// Selected streamed output in input order.
    Row(RequestSessionRow),
    /// Existing whole-function output after native admission.
    Aggregate(RequestValue),
    /// The actual synchronous native observer snapshot.
    Observation(OwnedRecordObservation),
    /// Final facts after native work has settled.
    Terminal(RequestSessionTerminal),
}

/// Complete native rows retain originals and nested observations.
#[derive(Debug)]
pub enum RequestSessionRow {
    /// A complete decision occurrence.
    Decision(CompleteRecord<QuestionInput, CompleteDecision>),
    /// A complete choice occurrence.
    Choice(CompleteRecord<QuestionInput, CompleteChoice>),
    /// A complete tag occurrence.
    Tags(CompleteRecord<QuestionInput, CompleteTags>),
    /// A complete score occurrence.
    Score(CompleteRecord<QuestionInput, CompleteScore>),
    /// A selected complete filter occurrence.
    Filter(CompleteRecord<QuestionInput, CompleteFilter>),
    /// A complete annotation occurrence.
    Annotation(CompleteRecord<QuestionInput, CompleteAnnotated>),
}

/// Immutable actual settlement, never a cancellation prediction.
#[derive(Debug)]
pub struct RequestSessionTerminal {
    /// Actual final native facts, absent before invocation started.
    pub facts: Option<Facts>,
    /// Joined native failure; success has none.
    pub error: Option<Error>,
}

impl RequestSessionTerminal {
    pub(super) fn failed(error: Error) -> Self {
        Self {
            facts: error.facts().cloned(),
            error: Some(error),
        }
    }
}
