//! Canonical result/2 constituent facts; no retrieval facts are inferred.

use serde::Serialize;

use crate::core::{AnswerId, FailureId, ModelName, ObservationId};

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
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QuestionSource {
    origin: Origin,
    answered_by: ModelName,
}

impl QuestionSource {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ResultIdentity {
    answer_id: AnswerId,
    origin: Option<Origin>,
    question_sources: Vec<QuestionSource>,
    observations: Vec<Observation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    answered_by: Option<ModelName>,
}

impl ResultIdentity {
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
