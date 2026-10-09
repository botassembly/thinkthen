//! Complete relation members retain a typed success/failure union and full answers.

use super::{CompleteMeta, MemberIdentity, ResultIdentity};
use crate::core::{Answer, BackendFailure, Meta, RelateSpec, RelationEdge, RelationEntity, Usage};
use serde::{Serialize, Serializer};

/// The existing wire question method used by one relation member.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum RelationMethod {
    /// A yes/no question about one source and target pair.
    YesNo,
    /// An ordered target menu, optionally selecting none.
    Choice,
}

/// The existing semantic direction of a relation rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum RelationDirection {
    /// Directed source to target.
    SourceToTarget,
    /// The relation holds in either direction.
    Either,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "EntryDocument<'static>")
)]
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

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
enum Outcome<'a> {
    Answered {
        answer_id: &'a crate::core::AnswerId,
        probability: f64,
        accepted: bool,
        answer: &'a Answer,
    },
    Failed {
        failure_id: &'a crate::core::FailureId,
        failure: &'a BackendFailure,
    },
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeRelationMember")
)]
struct EntryDocument<'a> {
    relation: &'a str,
    reads: &'a str,
    method: RelationMethod,
    direction: RelationDirection,
    source: &'a RelationEntity,
    target: Option<&'a RelationEntity>,
    #[serde(flatten)]
    outcome: Outcome<'a>,
    request: &'a str,
    question: crate::core::declaration::ReadableQuestion<'a, crate::core::Question>,
    threshold: crate::core::Threshold,
    question_sources: &'a [crate::core::QuestionSource],
    observations: &'a [crate::core::Observation],
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<crate::core::ReportedUsage>,
}
impl Serialize for RelationEntry {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let outcome = match (
            &self.identity,
            &self.answer,
            self.probability,
            self.accepted,
            &self.failure,
        ) {
            (
                MemberIdentity::Answered(id),
                Some(answer),
                Some(probability),
                Some(accepted),
                None,
            ) => Outcome::Answered {
                answer_id: id,
                probability,
                accepted,
                answer,
            },
            (MemberIdentity::Failed(id), None, None, None, Some(failure)) => Outcome::Failed {
                failure_id: id,
                failure,
            },
            _ => {
                return Err(serde::ser::Error::custom(
                    "a relation identity does not match its outcome",
                ));
            }
        };
        EntryDocument {
            relation: &self.relation,
            reads: &self.reads,
            method: self.method,
            direction: self.direction,
            source: &self.source,
            target: self.target.as_ref(),
            outcome,
            request: &self.request,
            question: crate::core::declaration::ReadableQuestion::atomic(
                &self.question,
                &crate::core::declaration::QuestionMetadata::default(),
            ),
            threshold: self.threshold,
            question_sources: &self.sources,
            observations: &self.observations,
            usage: self.reported_usage,
        }
        .serialize(serializer)
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
#[cfg_attr(test, derive(schemars::JsonSchema))]
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

#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeRelation")
)]
pub(crate) struct Document<'a, T: Serialize, V: Serialize = Vec<RelationEdge<RelationEntity>>> {
    schema: super::wire::Version,
    answer_id: &'a crate::core::AnswerId,
    value: &'a V,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<&'a T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<usize>,
    question: crate::core::declaration::SemanticReadableQuestion<
        'a,
        crate::core::relate_file::RelateQuestion<'a>,
    >,
    answer: Answers<'a>,
    meta: CompleteMeta<'a>,
}
impl Relation {
    pub(crate) fn serialize_with_input<S: Serializer, T: Serialize>(
        &self,
        input: Option<&T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        self.serialize_with_value(input, &self.value, serializer)
    }
    /// Physical occurrence expansion presents typed values without changing identity.
    pub(crate) fn serialize_with_value<S: Serializer, T: Serialize, V: Serialize>(
        &self,
        input: Option<&T>,
        value: &V,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        self.serialize_occurrence(input, value, None, serializer)
    }
    pub(crate) fn serialize_occurrence<S: Serializer, T: Serialize, V: Serialize>(
        &self,
        input: Option<&T>,
        value: &V,
        index: Option<usize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Document {
            index,
            schema: super::wire::Version::V2,
            answer_id: self.identity.answer_id(),
            value,
            input,
            question: crate::core::declaration::SemanticReadableQuestion::semantic(
                &self.question.question(self.lines),
                &self.question.metadata,
            ),
            answer: Answers {
                questions: &self.members,
            },
            meta: CompleteMeta::of(&self.meta, &self.identity),
        }
        .serialize(serializer)
    }
}

impl Serialize for Relation {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_with_input::<S, ()>(None, serializer)
    }
}
