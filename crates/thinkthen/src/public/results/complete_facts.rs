//! Complete invocation facts alongside the released count-only projection.

use super::Facts;
use crate::public::{AttemptObservation, CallId};
use serde::{Serialize, Serializer};

/// Borrowed final facts for exactly one admitted invocation.
#[derive(Debug)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "Document<'static>")
)]
pub struct CompleteFacts<'a> {
    facts: &'a Facts,
    call_id: &'a CallId,
}

impl Facts {
    /// Complete facts for one invocation; aggregate tallies have no invented ID.
    #[must_use]
    pub fn complete(&self) -> Option<CompleteFacts<'_>> {
        Some(CompleteFacts {
            facts: self,
            call_id: self.call_id()?,
        })
    }
}

impl CompleteFacts<'_> {
    /// The actual admitted invocation, shared by its prepared sends.
    #[must_use]
    pub const fn call_id(&self) -> &CallId {
        self.call_id
    }

    /// Final counts and optional usage using the released typed accessors.
    #[must_use]
    pub const fn counts(&self) -> &Facts {
        self.facts
    }

    /// Requested ordered attempts, including an empty list after zero sends.
    #[must_use]
    pub fn attempts(&self) -> Option<&[AttemptObservation]> {
        self.facts.attempts()
    }
}

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "completeFacts"))]
struct Document<'a> {
    call_id: &'a CallId,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_persistence: Option<PersistenceObservation>,
    cache_answers: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    held_model_mismatch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_cost_usd: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_tokens: Option<u64>,
    records: u64,
    requests_sent: u64,
    largest_request_bytes: usize,
    largest_request_estimated_input_tokens: Option<u64>,
    token_estimate_method: &'static str,
    seconds: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    attempts: Option<Vec<crate::public::CompleteAttempt<'a>>>,
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completePersistenceObservation")
)]
struct PersistenceObservation {
    state: super::UsagePersistence,
    observed_at: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    advice: Option<&'static str>,
}

impl PersistenceObservation {
    const fn of(state: super::UsagePersistence) -> Self {
        Self {
            state,
            observed_at: "facts_snapshot",
            advice: state.advice(),
        }
    }
}

impl Serialize for CompleteFacts<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let facts = self.facts;
        Document {
            call_id: self.call_id,
            usage_persistence: facts.usage_persistence().map(PersistenceObservation::of),
            cache_answers: facts.cache_answers,
            held_model_mismatch: facts.held_model_mismatch.then_some(true),
            estimated_cost_usd: facts.estimated_cost_usd.as_deref(),
            input_tokens: facts.input_tokens,
            model: facts.model.as_deref(),
            output_tokens: facts.output_tokens,
            records: facts.records,
            requests_sent: facts.requests_sent,
            largest_request_bytes: facts.largest_request_bytes,
            largest_request_estimated_input_tokens: facts.largest_request_estimated_input_tokens,
            token_estimate_method: facts.token_estimate_method,
            seconds: facts.seconds,
            attempts: self
                .attempts()
                .map(|attempts| attempts.iter().map(AttemptObservation::complete).collect()),
        }
        .serialize(serializer)
    }
}
