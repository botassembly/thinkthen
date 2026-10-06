//! Complete metadata retains legacy fields and explicitly reports observations.

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use super::ResultIdentity;
use crate::core::{Meta, Origin};

pub(crate) struct CompleteMeta<'a> {
    pub(crate) legacy: &'a Meta,
    pub(crate) identity: &'a ResultIdentity,
}

impl Serialize for CompleteMeta<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let legacy = self.legacy;
        let identity = self.identity;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("tool", &legacy.tool)?;
        map.serialize_entry("question_sha256", &legacy.question_sha256)?;
        map.serialize_entry("url", &legacy.url)?;
        map.serialize_entry("model", &legacy.model)?;
        if let Some(usage) = &legacy.reported_usage {
            map.serialize_entry("usage", usage)?;
        }
        map.serialize_entry("requests_sent", &legacy.requests_sent)?;
        let cached = !identity.question_sources().is_empty()
            && identity
                .question_sources()
                .iter()
                .all(|source| matches!(source.origin(), Origin::Cache | Origin::Replay));
        map.serialize_entry("cached", &cached)?;
        map.serialize_entry("requests", &legacy.requests)?;
        map.serialize_entry("failed_questions", &legacy.failed_questions)?;
        if let Some(warning) = &legacy.profile_warning {
            map.serialize_entry("profile_warning", warning)?;
        }
        if let Some(setting) = &legacy.batch_setting {
            map.serialize_entry("batch_setting", setting)?;
        }
        if let Some(warning) = &legacy.batch_warning {
            map.serialize_entry("batch_warning", warning)?;
        }
        if let Some(digest) = &legacy.context_sha256 {
            map.serialize_entry("context_sha256", digest)?;
        }
        if let Some(attempts) = &legacy.attempts {
            let complete: Vec<_> = attempts
                .iter()
                .map(crate::core::AttemptObservation::complete)
                .collect();
            map.serialize_entry("attempts", &complete)?;
        }
        map.serialize_entry("origin", &identity.origin())?;
        map.serialize_entry("question_sources", identity.question_sources())?;
        map.serialize_entry("observations", identity.observations())?;
        if let Some(model) = identity.answered_by() {
            map.serialize_entry("answered_by", model)?;
        }
        map.end()
    }
}
