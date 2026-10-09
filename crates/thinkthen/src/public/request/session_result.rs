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
    Observation {
        /// Declared request function, retained independently of event payload.
        function: super::RequestFunction,
        /// The actual native observer snapshot.
        value: OwnedRecordObservation,
    },
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

use crate::public::results::SessionObservationDocument;
use serde::Serialize;

impl RequestSessionResult {
    /// Serialize the canonical owned packet, preserving complete native payloads.
    /// # Errors
    /// Returns a defect if the native document cannot be written.
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&SessionPacketDocument::of(self))
            .map_err(|_| Error::defect("a session packet could not be written as JSON"))
    }
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "sessionPacket"))]
#[serde(tag = "kind", rename_all = "lowercase")]
#[expect(
    clippy::large_enum_variant,
    reason = "one short-lived borrowed document keeps serialization stack-local without another allocation"
)]
pub(crate) enum SessionPacketDocument<'a> {
    Row(SessionRowDocument<'a>),
    Aggregate(SessionAggregateDocument<'a>),
    Observation {
        function: super::RequestFunction,
        value: SessionObservationDocument<'a>,
    },
    Terminal {
        #[serde(skip_serializing_if = "Option::is_none")]
        facts: Option<crate::CompleteFacts<'a>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure: Option<crate::CompleteError<'a>>,
    },
}
impl<'a> SessionPacketDocument<'a> {
    fn of(value: &'a RequestSessionResult) -> Self {
        match value {
            RequestSessionResult::Row(value) => Self::Row(SessionRowDocument::of(value)),
            RequestSessionResult::Aggregate(value) => {
                Self::Aggregate(SessionAggregateDocument::of(value))
            }
            RequestSessionResult::Observation { function, value } => Self::Observation {
                function: *function,
                value: SessionObservationDocument::of(value),
            },
            RequestSessionResult::Terminal(value) => Self::Terminal {
                facts: value.facts.as_ref().and_then(Facts::complete),
                failure: value.error.as_ref().map(Error::complete),
            },
        }
    }
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "sessionRow"))]
#[serde(tag = "function", content = "value", rename_all = "lowercase")]
pub(crate) enum SessionRowDocument<'a> {
    #[cfg_attr(
        test,
        schemars(
            with = "crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value>"
        )
    )]
    Decide(&'a CompleteRecord<QuestionInput, CompleteDecision>),
    #[cfg_attr(
        test,
        schemars(
            with = "crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value, Option<&'a str>>"
        )
    )]
    Choose(&'a CompleteRecord<QuestionInput, CompleteChoice>),
    #[cfg_attr(
        test,
        schemars(
            with = "crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value, &'a [String]>"
        )
    )]
    Tag(&'a CompleteRecord<QuestionInput, CompleteTags>),
    #[cfg_attr(
        test,
        schemars(
            with = "crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value, f64>"
        )
    )]
    Score(&'a CompleteRecord<QuestionInput, CompleteScore>),
    #[cfg_attr(
        test,
        schemars(
            with = "crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value, bool>"
        )
    )]
    Filter(&'a CompleteRecord<QuestionInput, CompleteFilter>),
    #[cfg_attr(
        test,
        schemars(
            with = "crate::core::complete_documents::annotation::Document<'a, serde_json::Value>"
        )
    )]
    Annotate(&'a CompleteRecord<QuestionInput, CompleteAnnotated>),
}
impl<'a> SessionRowDocument<'a> {
    fn of(value: &'a RequestSessionRow) -> Self {
        match value {
            RequestSessionRow::Decision(v) => Self::Decide(v),
            RequestSessionRow::Choice(v) => Self::Choose(v),
            RequestSessionRow::Tags(v) => Self::Tag(v),
            RequestSessionRow::Score(v) => Self::Score(v),
            RequestSessionRow::Filter(v) => Self::Filter(v),
            RequestSessionRow::Annotation(v) => Self::Annotate(v),
        }
    }
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sessionAggregate")
)]
#[serde(tag = "function", content = "value", rename_all = "lowercase")]
pub(crate) enum SessionAggregateDocument<'a> {
    #[cfg_attr(
        test,
        schemars(
            with = "Vec<crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value>>"
        )
    )]
    Decide(&'a [CompleteRecord<QuestionInput, CompleteDecision>]),
    #[cfg_attr(
        test,
        schemars(
            with = "Vec<crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value, Option<&'a str>>>"
        )
    )]
    Choose(&'a [CompleteRecord<QuestionInput, CompleteChoice>]),
    #[cfg_attr(
        test,
        schemars(
            with = "Vec<crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value, &'a [String]>>"
        )
    )]
    Tag(&'a [CompleteRecord<QuestionInput, CompleteTags>]),
    #[cfg_attr(
        test,
        schemars(
            with = "Vec<crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value, f64>>"
        )
    )]
    Score(&'a [CompleteRecord<QuestionInput, CompleteScore>]),
    #[cfg_attr(
        test,
        schemars(
            with = "Vec<crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value, bool>>"
        )
    )]
    Filter(&'a [CompleteRecord<QuestionInput, CompleteFilter>]),
    #[cfg_attr(
        test,
        schemars(
            with = "Vec<crate::core::complete_documents::wire::AtomicDocument<'a, serde_json::Value, std::num::NonZeroUsize>>"
        )
    )]
    Rank(SessionRankDocument<'a>),
    #[cfg_attr(
        test,
        schemars(with = "crate::core::complete_documents::find::Document<'a, serde_json::Value>")
    )]
    Find(&'a crate::CompleteFound<QuestionInput>),
    #[cfg_attr(
        test,
        schemars(
            with = "Vec<crate::core::complete_documents::annotation::Document<'a, serde_json::Value>>"
        )
    )]
    Annotate(&'a [CompleteRecord<QuestionInput, CompleteAnnotated>]),
    #[cfg_attr(
        test,
        schemars(
            with = "Vec<crate::core::complete_documents::recognize::Document<'a, serde_json::Value>>"
        )
    )]
    Recognize(&'a [CompleteRecord<QuestionInput, crate::CompleteRecognized>]),
    #[cfg_attr(
        test,
        schemars(
            with = "crate::core::complete_documents::relate::Document<'a, serde_json::Value>"
        )
    )]
    Relate(&'a CompleteRecord<Vec<QuestionInput>, crate::CompleteRelated>),
}
impl<'a> SessionAggregateDocument<'a> {
    fn of(value: &'a RequestValue) -> Self {
        match value {
            RequestValue::Decisions(v) => Self::Decide(v),
            RequestValue::Choices(v) => Self::Choose(v),
            RequestValue::Tags(v) => Self::Tag(v),
            RequestValue::Scores(v) => Self::Score(v),
            RequestValue::Filtered(v) => Self::Filter(v),
            RequestValue::Ranked(v) => Self::Rank(SessionRankDocument::Single(v)),
            RequestValue::SetRanked(v) => Self::Rank(SessionRankDocument::Set(v)),
            RequestValue::Found(v) => Self::Find(v),
            RequestValue::Annotations(v) => Self::Annotate(v),
            RequestValue::Recognized(v) => Self::Recognize(v),
            RequestValue::Related(v) => Self::Relate(v),
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum SessionRankDocument<'a> {
    Single(&'a [CompleteRecord<QuestionInput, crate::CompleteRank>]),
    Set(&'a [CompleteRecord<QuestionInput, crate::CompleteSetRank>]),
}
