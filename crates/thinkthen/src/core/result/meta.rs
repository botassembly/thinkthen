//! Detailed judgment metadata.

use serde::Serialize;

use super::{BatchSetting, BatchWarning, ProfileWarning, RequestMeta, Usage};
use crate::core::text::{ModelName, Url};

/// Who answered, how, at what cost, from a backend or from a recording.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "meta"))]
pub(crate) struct Meta {
    pub(super) tool: String,
    pub(super) question_sha256: String,
    pub(super) url: Url,
    pub(super) model: ModelName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) usage: Option<Usage>,
    #[serde(skip)]
    #[cfg_attr(test, schemars(skip))]
    pub(super) reported_usage: Option<super::ReportedUsage>,
    pub(super) requests_sent: u64,
    pub(super) cached: bool,
    pub(super) requests: Vec<String>,
    pub(super) failed_questions: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) profile_warning: Option<ProfileWarning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) batch_setting: Option<BatchSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) batch_warning: Option<BatchWarning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) context_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) attempts: Option<Vec<crate::core::AttemptObservation>>,
}

impl Meta {
    pub(crate) fn with_captured_attempts(
        mut self,
        attempts: Option<Vec<crate::core::AttemptObservation>>,
    ) -> Self {
        self.attempts = attempts;
        self
    }

    /// Name the tool, who answered, at what cost, and whether a recording did.
    ///
    /// The result keeps the binary, resolved question, backend, cost, send
    /// count, replay state, and ordered logical request identities.
    #[must_use]
    pub(crate) fn new(
        version: &str,
        question_sha256: String,
        url: Url,
        model: ModelName,
        usage: Option<Usage>,
        request_meta: RequestMeta,
    ) -> Self {
        let RequestMeta {
            replayed,
            requests_sent,
            requests,
            failed_questions,
            profile_warning,
            batch_setting,
            batch_warning,
            context_sha256,
        } = request_meta;
        Self {
            tool: crate::core::version_line(version),
            question_sha256,
            url,
            model,
            usage,
            reported_usage: usage.map(super::ReportedUsage::from_complete),
            requests_sent,
            cached: replayed,
            requests,
            failed_questions,
            profile_warning,
            batch_setting,
            batch_warning,
            context_sha256,
            attempts: None,
        }
    }

    pub(crate) fn with_reported_usage(mut self, usage: Option<super::ReportedUsage>) -> Self {
        self.reported_usage = usage;
        self
    }

    pub(crate) fn with_attempts(mut self, attempts: Vec<crate::core::AttemptObservation>) -> Self {
        if !attempts.is_empty() {
            self.attempts = Some(attempts);
        }
        self
    }
}
