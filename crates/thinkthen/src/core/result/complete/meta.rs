//! One actual serialized metadata type supplies native documents and their schema.
use super::ResultIdentity;
use crate::core::{Meta, Origin};
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "completeMeta"))]
pub(crate) struct CompleteMeta<'a> {
    tool: &'a str,
    #[cfg_attr(test, schemars(regex(pattern = "^[0-9a-f]{64}$")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    question_sha256: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(regex(pattern = "^[0-9a-f]{64}$")))]
    questions_sha256: Option<&'a str>,
    url: &'a str,
    model: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<crate::core::ReportedUsage>,
    requests_sent: u64,
    cached: bool,
    requests: &'a [String],
    failed_questions: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_warning: Option<&'a crate::core::ProfileWarning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_setting: Option<crate::core::BatchSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_warning: Option<&'a crate::core::BatchWarning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context_sha256: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attempts: Option<Vec<crate::core::CompleteAttempt<'a>>>,
    origin: Option<Origin>,
    question_sources: &'a [crate::core::QuestionSource],
    observations: &'a [crate::core::Observation],
    #[serde(skip_serializing_if = "Option::is_none")]
    answered_by: Option<&'a str>,
}
impl<'a> CompleteMeta<'a> {
    pub(crate) fn of(legacy: &'a Meta, identity: &'a ResultIdentity) -> Self {
        Self::from_fields(legacy.fields(), identity)
    }
    pub(crate) fn from_fields(
        fields: crate::core::MetadataFields<'a>,
        identity: &'a ResultIdentity,
    ) -> Self {
        Self {
            tool: fields.tool,
            question_sha256: (!fields.plural).then_some(fields.digest),
            questions_sha256: fields.plural.then_some(fields.digest),
            url: fields.url,
            model: fields.model,
            usage: fields.usage,
            requests_sent: fields.sent,
            cached: !identity.question_sources().is_empty()
                && identity
                    .question_sources()
                    .iter()
                    .all(|source| matches!(source.origin(), Origin::Cache | Origin::Replay)),
            requests: fields.requests,
            failed_questions: fields.failed,
            profile_warning: fields.profile,
            batch_setting: fields.batch,
            batch_warning: fields.batch_warning,
            context_sha256: fields.context,
            attempts: fields.attempts.map(|attempts| {
                attempts
                    .iter()
                    .map(crate::core::AttemptObservation::complete)
                    .collect()
            }),
            origin: identity.origin(),
            question_sources: identity.question_sources(),
            observations: identity.observations(),
            answered_by: identity.answered_by(),
        }
    }
}
