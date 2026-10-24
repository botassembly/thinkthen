//! One record's result.

// Schema derives generate sibling impls using JSON maps for metadata.
// This module holds result values; evidence parsing stays in function modules.
#![allow(
    clippy::disallowed_types,
    reason = "generated result schemas build metadata; values remain typed"
)]

use std::collections::BTreeMap;

use serde::Serialize;

use crate::core::error::Error;
use crate::core::probability::Probability;
use crate::core::record::Id;

/// One record's result: its id and what its question came to.
#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
pub struct Row {
    id: Id,
    #[serde(flatten)]
    outcome: Outcome,
}

impl Row {
    /// The JSON Schema of serialized result rows, including omitted fields.
    #[must_use]
    pub fn schema() -> schemars::Schema {
        schemars::generate::SchemaSettings::draft2020_12()
            .for_serialize()
            .into_generator()
            .into_root_schema_for::<Self>()
    }

    pub(crate) const fn new(id: Id, outcome: Outcome) -> Self {
        Self { id, outcome }
    }

    pub(crate) fn position(&mut self, position: usize) {
        if !matches!(self.outcome, Outcome::Failed { .. }) {
            self.outcome = Outcome::Answered {
                answer: Answer::Position(position),
                p: None,
                detail: match &self.outcome {
                    Outcome::Answered { detail, .. } | Outcome::Unsure { detail, .. } => {
                        detail.clone()
                    }
                    Outcome::Failed { .. } => None,
                },
            };
        }
    }

    /// Attach the exact question and route that served this result.
    #[must_use]
    pub fn with_source(mut self, source: Source) -> Self {
        match &mut self.outcome {
            Outcome::Answered { detail, .. } | Outcome::Unsure { detail, .. } => {
                *detail = Some(Detail::Sourced {
                    source,
                    probabilities: detail.take().map(Box::new),
                });
            }
            Outcome::Failed { .. } => {}
        }
        self
    }

    /// What the record's question came to.
    #[must_use]
    pub const fn outcome(&self) -> &Outcome {
        &self.outcome
    }
}

/// What a record's question came to.
#[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum Outcome {
    /// The model answered and the threshold accepted the answer.
    Answered {
        /// The answer.
        answer: Answer,
        /// The probability behind the answer.
        #[serde(skip_serializing_if = "Option::is_none")]
        p: Option<Probability>,
        /// The backend probabilities for each label or scale level.
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<Detail>,
    },
    /// The model answered inside the not sure band.
    #[serde(serialize_with = "serialize_unsure")]
    #[schemars(with = "Unsure<'static>")]
    Unsure {
        /// The probability behind the answer.
        p: Probability,
        /// The backend probabilities for each label or scale level.
        detail: Option<Detail>,
    },
    /// The question could not be answered.
    Failed {
        /// Why.
        error: Error,
    },
}

fn serialize_unsure<S: serde::Serializer>(
    p: &Probability,
    detail: &Option<Detail>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    Unsure {
        answer: (),
        p,
        detail,
    }
    .serialize(serializer)
}

#[derive(Serialize, schemars::JsonSchema)]
struct Unsure<'a> {
    answer: (),
    p: &'a Probability,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: &'a Option<Detail>,
}

/// An answer's value. Its shape depends on the function.
#[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Answer {
    /// Relations stated by the text.
    Relations(Vec<crate::core::Relation>),
    /// Names and their kinds in text order.
    Names(Vec<crate::core::Name>),
    /// A selected text unit, or no matching unit.
    Found(Option<crate::core::Found>),
    /// The record's rank, starting at one.
    Position(usize),
    /// Each named question and its own outcome.
    Annotations(BTreeMap<String, Outcome>),
    /// The chosen option.
    Label(String),
    /// The answer to a yes or no question.
    Bool(bool),
    /// Every label that applies.
    Labels(Vec<String>),
    /// A number on a scale from 0 to 1.
    Number(f64),
}

/// The backend probabilities behind a structured answer.
#[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Detail {
    /// The caller-visible question identity and route with optional backend detail.
    Sourced {
        /// The question and route that answered.
        source: Source,
        /// The function's complete backend detail, when present.
        #[serde(skip_serializing_if = "Option::is_none")]
        probabilities: Option<Box<Detail>>,
    },
    /// Every asked pair and its probability.
    Relate(Vec<crate::core::Relation>),
    /// Boundary and kind probabilities for recognized names.
    Recognize(crate::core::RecognizeDetail),
    /// A probability for each named label.
    Labels(BTreeMap<String, Probability>),
    /// A probability for each scale level, in the question's order.
    Levels(Vec<Probability>),
}

impl Outcome {
    /// The first failure in this outcome, including its named question entries.
    #[must_use]
    pub fn failure(&self) -> Option<&Error> {
        match self {
            Self::Failed { error } => Some(error),
            Self::Answered {
                answer: Answer::Annotations(entries),
                ..
            } => entries.values().find_map(Self::failure),
            _ => None,
        }
    }
}

/// The caller-visible question and actual route that answered it.
#[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
pub struct Source {
    /// SHA-256 of the caller-visible function and question content.
    pub question_identity: String,
    /// The backend that served the question.
    pub backend: String,
    /// The model that served the question.
    pub model: String,
}
