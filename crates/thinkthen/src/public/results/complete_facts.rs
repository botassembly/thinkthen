//! Complete invocation facts alongside the released count-only projection.

use super::Facts;
use crate::public::{AttemptObservation, CallId};
use serde::{Serialize, Serializer, ser::SerializeMap};

/// Borrowed final facts for exactly one admitted invocation.
#[derive(Debug)]
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

impl Serialize for CompleteFacts<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let facts = self.facts;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("call_id", self.call_id)?;
        map.serialize_entry("cache_answers", &facts.cache_answers)?;
        if let Some(value) = &facts.estimated_cost_usd {
            map.serialize_entry("estimated_cost_usd", value)?;
        }
        if let Some(value) = facts.input_tokens {
            map.serialize_entry("input_tokens", &value)?;
        }
        if let Some(value) = &facts.model {
            map.serialize_entry("model", value)?;
        }
        if let Some(value) = facts.output_tokens {
            map.serialize_entry("output_tokens", &value)?;
        }
        map.serialize_entry("records", &facts.records)?;
        map.serialize_entry("requests_sent", &facts.requests_sent)?;
        map.serialize_entry("seconds", &facts.seconds)?;
        if let Some(attempts) = self.attempts() {
            let complete: Vec<_> = attempts.iter().map(AttemptObservation::complete).collect();
            map.serialize_entry("attempts", &complete)?;
        }
        map.end()
    }
}
