//! Canonical result/2 constituent facts; no retrieval facts are inferred.

use serde::Serialize;

use crate::core::{AnswerId, FailureId, ModelName, ObservationId};

mod annotation;
mod atomic;
mod find;
pub use annotation::MemberIdentity;
pub(crate) use annotation::{Annotation, AnnotationMember};
pub(crate) use find::Find;
mod recognize;
pub(crate) use recognize::Recognition;
mod relate;
pub(crate) use relate::{Relation, RelationEntry};
pub use relate::{RelationDirection, RelationMethod};
mod meta;
pub(crate) use atomic::Atomic;
use meta::CompleteMeta;

/// The actual source of one logical question's accepted reply.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    /// A current direct provider reply.
    Live,
    /// An answer retained in the working cache.
    Cache,
    /// An answer obtained from explicitly requested offline replay.
    Replay,
    /// Reserved for an admitted future proxy protocol.
    Proxy,
    /// Reserved for an admitted future in-memory source.
    Memory,
}

/// The identity of exactly one logical observation or failed occurrence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Observation {
    /// A successful accepted answer, including successful null.
    Answered {
        /// The identity retained by cache and replay.
        observation_id: ObservationId,
    },
    /// A failed occurrence, never persisted as an answer.
    Failed {
        /// The identity of this occurrence's failure.
        failure_id: FailureId,
    },
}

/// A logical question's retrieval origin and validated reported model.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct QuestionSource {
    origin: Origin,
    answered_by: ModelName,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<std::num::NonZeroU32>,
}

impl QuestionSource {
    pub(crate) const fn new(
        origin: Origin,
        answered_by: ModelName,
        batch_size: Option<std::num::NonZeroU32>,
    ) -> Self {
        Self {
            origin,
            answered_by,
            batch_size,
        }
    }

    pub(crate) const fn model(&self) -> &ModelName {
        &self.answered_by
    }

    /// Actual wire-question count that produced this observation; unknown history stays absent.
    #[must_use]
    pub fn batch_size(&self) -> Option<u32> {
        self.batch_size.map(std::num::NonZeroU32::get)
    }

    /// Where this occurrence's response was retrieved.
    #[must_use]
    pub const fn origin(&self) -> Origin {
        self.origin
    }

    /// The actual model named by this occurrence's response.
    #[must_use]
    pub fn answered_by(&self) -> &str {
        self.answered_by.as_str()
    }
}

/// One complete result's aligned constituent trace and stable logical ID.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ResultIdentity {
    answer_id: AnswerId,
    origin: Option<Origin>,
    question_sources: Vec<QuestionSource>,
    observations: Vec<Observation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    answered_by: Option<ModelName>,
}

impl ResultIdentity {
    pub(crate) const fn resolved(
        answer_id: AnswerId,
        origin: Option<Origin>,
        question_sources: Vec<QuestionSource>,
        observations: Vec<Observation>,
        answered_by: Option<ModelName>,
    ) -> Self {
        Self {
            answer_id,
            origin,
            question_sources,
            observations,
            answered_by,
        }
    }

    /// The logical answer ID, available without probability opt-in.
    #[must_use]
    pub const fn answer_id(&self) -> &AnswerId {
        &self.answer_id
    }

    /// Aggregate retrieval origin; absent when no observations exist.
    #[must_use]
    pub const fn origin(&self) -> Option<Origin> {
        self.origin
    }

    /// Sources in logical question order, including repeated occurrences.
    #[must_use]
    pub fn question_sources(&self) -> &[QuestionSource] {
        &self.question_sources
    }

    /// Identities aligned with the logical questions and their sources.
    #[must_use]
    pub fn observations(&self) -> &[Observation] {
        &self.observations
    }

    /// The agreed actual model, absent for empty or mixed-model traces.
    #[must_use]
    pub fn answered_by(&self) -> Option<&str> {
        self.answered_by.as_ref().map(ModelName::as_str)
    }
}

#[cfg(test)]
mod tests;

impl std::fmt::Debug for QuestionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuestionSource")
            .field("origin", &self.origin)
            .field("answered_by", &"<withheld>")
            .field("batch_size", &self.batch_size)
            .finish()
    }
}
impl std::fmt::Debug for ResultIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResultIdentity")
            .field("answer_id", &self.answer_id)
            .field("origin", &self.origin)
            .field("question_sources", &self.question_sources)
            .field("observations", &self.observations)
            .finish_non_exhaustive()
    }
}
