//! Complete annotation entries distinguish successful null from failed occurrences.

use serde::{Serialize, Serializer};

use super::ResultIdentity;
use crate::core::{AnnotateResult, AnnotatedEntry, AnswerId, FailureId};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Annotation {
    pub(crate) identity: ResultIdentity,
    pub(crate) source: Option<super::wire::PhysicalSource>,
    pub(crate) legacy: AnnotateResult,
    pub(crate) members: Vec<(String, AnnotationMember)>,
    pub(crate) context_sha256: Option<String>,
    pub(crate) attempts: Option<Vec<crate::core::AttemptObservation>>,
}

impl Annotation {
    pub(crate) fn metadata(&self) -> crate::core::MetadataFields<'_> {
        let mut fields = self.legacy.meta.fields();
        fields.context = self.context_sha256.as_deref();
        fields.attempts = self.attempts.as_deref();
        fields
    }
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
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "MemberDocument<'static>")
)]
pub(crate) struct AnnotationMember {
    pub(crate) declarations: crate::core::declaration::QuestionMetadata,
    pub(crate) identity: MemberIdentity,
    pub(crate) legacy: AnnotatedEntry,
    pub(crate) threshold: Option<crate::core::Threshold>,
    pub(crate) sources: Vec<crate::core::QuestionSource>,
    pub(crate) observations: Vec<crate::core::Observation>,
    pub(crate) reported_usage: Option<crate::core::ReportedUsage>,
}

impl AnnotationMember {
    pub(crate) fn question(&self) -> &crate::core::Question {
        match &self.legacy {
            AnnotatedEntry::Answered(entry) => &entry.question,
            AnnotatedEntry::Failed(entry) => &entry.question,
        }
    }

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

#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeAnnotationMember")
)]
#[serde(untagged)]
enum MemberDocument<'a> {
    Answered {
        answer_id: &'a AnswerId,
        value: &'a crate::core::Value,
        question: crate::core::declaration::ReadableQuestion<'a, crate::core::Question>,
        answer: &'a crate::core::Answer,
        threshold: Option<crate::core::Threshold>,
        request: &'a str,
        question_sources: &'a [crate::core::QuestionSource],
        observations: &'a [crate::core::Observation],
        #[serde(skip_serializing_if = "Option::is_none")]
        usage: Option<crate::core::ReportedUsage>,
    },
    Failed {
        failure_id: &'a FailureId,
        question: crate::core::declaration::ReadableQuestion<'a, crate::core::Question>,
        failure: &'a crate::core::BackendFailure,
        threshold: Option<crate::core::Threshold>,
        request: &'a str,
        question_sources: &'a [crate::core::QuestionSource],
        observations: &'a [crate::core::Observation],
        #[serde(skip_serializing_if = "Option::is_none")]
        usage: Option<crate::core::ReportedUsage>,
    },
}

impl Serialize for AnnotationMember {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let question =
            crate::core::declaration::ReadableQuestion::atomic(self.question(), &self.declarations);
        let document = match (&self.identity, &self.legacy) {
            (MemberIdentity::Answered(id), AnnotatedEntry::Answered(entry)) => {
                MemberDocument::Answered {
                    answer_id: id,
                    value: &entry.value,
                    question,
                    answer: &entry.answer,
                    threshold: self.threshold,
                    request: &entry.request,
                    question_sources: &self.sources,
                    observations: &self.observations,
                    usage: self.reported_usage,
                }
            }
            (MemberIdentity::Failed(id), AnnotatedEntry::Failed(entry)) => MemberDocument::Failed {
                failure_id: id,
                question,
                failure: &entry.failure,
                threshold: self.threshold,
                request: &entry.request,
                question_sources: &self.sources,
                observations: &self.observations,
                usage: self.reported_usage,
            },
            _ => {
                return Err(serde::ser::Error::custom(
                    "an annotation identity does not match its outcome",
                ));
            }
        };
        document.serialize(serializer)
    }
}

#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "std::collections::BTreeMap<String, AnnotationMember>")
)]
struct Members<'a>(&'a [(String, AnnotationMember)]);
impl Serialize for Members<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(name, member)| (name, member)))
    }
}

#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeAnnotation")
)]
pub(crate) struct Document<'a, T: Serialize> {
    schema: super::wire::Version,
    answer_id: &'a AnswerId,
    input: &'a T,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<&'a super::wire::PhysicalSource>,
    value: &'a crate::core::NamedValues,
    answers: Members<'a>,
    meta: super::CompleteMeta<'a>,
}
impl Annotation {
    pub(crate) fn serialize_with_input<S: Serializer, T: Serialize>(
        &self,
        input: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        self.serialize_occurrence(input, None, serializer)
    }
    pub(crate) fn serialize_occurrence<S: Serializer, T: Serialize>(
        &self,
        input: &T,
        index: Option<usize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Document {
            index,
            source: self.source.as_ref(),
            schema: super::wire::Version::V2,
            answer_id: self.identity.answer_id(),
            input,
            value: &self.legacy.value,
            answers: Members(&self.members),
            meta: super::CompleteMeta::from_fields(self.metadata(), &self.identity),
        }
        .serialize(serializer)
    }
}
impl Serialize for Annotation {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_with_input(&self.legacy.input, serializer)
    }
}
