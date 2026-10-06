//! Complete relation members retain a typed success/failure union and full answers.

use super::{CompleteMeta, MemberIdentity, ResultIdentity};
use crate::core::{Answer, BackendFailure, Meta, RelateSpec, RelationEdge, RelationEntity, Usage};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

/// The existing wire question method used by one relation member.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationMethod {
    /// A yes/no question about one source and target pair.
    YesNo,
    /// An ordered target menu, optionally selecting none.
    Choice,
}

/// The existing semantic direction of a relation rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationDirection {
    /// Directed source to target.
    SourceToTarget,
    /// The relation holds in either direction.
    Either,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RelationEntry {
    pub(crate) identity: MemberIdentity,
    pub(crate) question: crate::core::Question,
    pub(crate) threshold: crate::core::Threshold,
    pub(crate) sources: Vec<crate::core::QuestionSource>,
    pub(crate) observations: Vec<crate::core::Observation>,
    pub(crate) reported_usage: Option<crate::core::ReportedUsage>,
    pub(crate) relation: String,
    pub(crate) reads: String,
    pub(crate) method: RelationMethod,
    pub(crate) direction: RelationDirection,
    pub(crate) source: RelationEntity,
    pub(crate) target: Option<RelationEntity>,
    pub(crate) answer: Option<Answer>,
    pub(crate) probability: Option<f64>,
    pub(crate) accepted: Option<bool>,
    pub(crate) failure: Option<BackendFailure>,
    pub(crate) request: String,
}

impl Serialize for RelationEntry {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("relation", &self.relation)?;
        map.serialize_entry("reads", &self.reads)?;
        map.serialize_entry("method", &self.method)?;
        map.serialize_entry("direction", &self.direction)?;
        map.serialize_entry("source", &self.source)?;
        map.serialize_entry("target", &self.target)?;
        match (
            &self.identity,
            &self.answer,
            self.probability,
            self.accepted,
            &self.failure,
        ) {
            (MemberIdentity::Answered(id), Some(_), Some(p), Some(accepted), None) => {
                map.serialize_entry("answer_id", id)?;
                map.serialize_entry("probability", &p)?;
                map.serialize_entry("accepted", &accepted)?;
                map.serialize_entry("answer", &self.answer)?;
            }
            (MemberIdentity::Failed(id), None, None, None, Some(failure)) => {
                map.serialize_entry("failure_id", id)?;
                map.serialize_entry("failure", failure)?;
            }
            _ => {
                return Err(serde::ser::Error::custom(
                    "a relation identity does not match its outcome",
                ));
            }
        }
        map.serialize_entry("request", &self.request)?;
        map.end()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Relation {
    pub(crate) identity: ResultIdentity,
    pub(crate) value: Vec<RelationEdge<RelationEntity>>,
    pub(crate) question: RelateSpec,
    pub(crate) lines: bool,
    pub(crate) members: Vec<RelationEntry>,
    pub(crate) meta: Meta,
}

#[derive(Serialize)]
struct Answers<'a> {
    questions: &'a [RelationEntry],
}

impl Relation {
    pub(crate) fn metadata(&self) -> crate::core::MetadataFields<'_> {
        self.meta.fields()
    }

    pub(crate) const fn reported_usage(&self) -> Option<crate::core::ReportedUsage> {
        self.meta.reported_usage
    }

    pub(crate) const fn usage(&self) -> Option<Usage> {
        self.meta.usage
    }
}

impl Serialize for Relation {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("schema", "thinkthen.result/2")?;
        map.serialize_entry("answer_id", self.identity.answer_id())?;
        map.serialize_entry("value", &self.value)?;
        map.serialize_entry("question", &self.question.question(self.lines))?;
        map.serialize_entry(
            "answer",
            &Answers {
                questions: &self.members,
            },
        )?;
        map.serialize_entry(
            "meta",
            &CompleteMeta {
                legacy: &self.meta,
                identity: &self.identity,
            },
        )?;
        map.end()
    }
}
