//! Complete annotation entries distinguish successful null from failed occurrences.

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use super::ResultIdentity;
use crate::core::{AnnotateResult, AnnotatedEntry, AnswerId, FailureId, Origin};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Annotation {
    pub(crate) identity: ResultIdentity,
    pub(crate) legacy: AnnotateResult,
    pub(crate) members: Vec<(String, AnnotationMember)>,
}

/// The mutually exclusive identity of a successful reading or failed member.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemberIdentity {
    /// A successful logical answer, including successful null.
    Answered(AnswerId),
    /// A failed logical occurrence; no answer is fabricated.
    Failed(FailureId),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AnnotationMember {
    pub(crate) identity: MemberIdentity,
    pub(crate) legacy: AnnotatedEntry,
}

impl AnnotationMember {
    pub(crate) fn request(&self) -> &str {
        match &self.legacy {
            AnnotatedEntry::Answered(entry) => &entry.request,
            AnnotatedEntry::Failed(entry) => &entry.request,
        }
    }

    pub(crate) const fn value(&self) -> Option<&crate::core::Value> {
        match &self.legacy {
            AnnotatedEntry::Answered(entry) => Some(&entry.value),
            AnnotatedEntry::Failed(_) => None,
        }
    }
}

impl Serialize for AnnotationMember {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        match (&self.identity, &self.legacy) {
            (MemberIdentity::Answered(id), AnnotatedEntry::Answered(entry)) => {
                map.serialize_entry("answer_id", id)?;
                map.serialize_entry("value", &entry.value)?;
                map.serialize_entry("question", &entry.question)?;
                map.serialize_entry("answer", &entry.answer)?;
                map.serialize_entry("threshold", &entry.threshold)?;
                map.serialize_entry("request", &entry.request)?;
            }
            (MemberIdentity::Failed(id), AnnotatedEntry::Failed(entry)) => {
                map.serialize_entry("failure_id", id)?;
                map.serialize_entry("question", &entry.question)?;
                map.serialize_entry("failure", &entry.failure)?;
                map.serialize_entry("request", &entry.request)?;
            }
            _ => {
                return Err(serde::ser::Error::custom(
                    "an annotation identity does not match its outcome",
                ));
            }
        }
        map.end()
    }
}

struct Members<'a>(&'a [(String, AnnotationMember)]);

impl Serialize for Members<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(name, member)| (name, member)))
    }
}

impl Serialize for Annotation {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let row = &self.legacy;
        let meta = &row.meta;
        let identity = &self.identity;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("schema", "thinkthen.result/2")?;
        map.serialize_entry("answer_id", identity.answer_id())?;
        map.serialize_entry("input", &row.input)?;
        map.serialize_entry("value", &row.value)?;
        map.serialize_entry("answers", &Members(&self.members))?;
        map.serialize_entry(
            "meta",
            &AnnotationMeta {
                row: meta,
                identity,
            },
        )?;
        map.end()
    }
}

struct AnnotationMeta<'a> {
    row: &'a super::super::AnnotateMeta,
    identity: &'a ResultIdentity,
}

impl Serialize for AnnotationMeta<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let row = self.row;
        let identity = self.identity;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("tool", &row.tool)?;
        map.serialize_entry("questions_sha256", &row.questions_sha256)?;
        map.serialize_entry("url", &row.url)?;
        map.serialize_entry("model", &row.model)?;
        if let Some(usage) = &row.usage {
            map.serialize_entry("usage", usage)?;
        }
        map.serialize_entry("requests_sent", &row.requests_sent)?;
        let cached = !identity.question_sources().is_empty()
            && identity
                .question_sources()
                .iter()
                .all(|source| matches!(source.origin(), Origin::Cache | Origin::Replay));
        map.serialize_entry("cached", &cached)?;
        map.serialize_entry("requests", &row.requests)?;
        map.serialize_entry("failed_questions", &row.failed_questions)?;
        if let Some(warning) = &row.profile_warning {
            map.serialize_entry("profile_warning", warning)?;
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
