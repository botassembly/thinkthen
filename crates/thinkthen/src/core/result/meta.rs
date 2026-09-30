//! Detailed judgment metadata.

use serde::Serialize;

use super::{BatchMeta, BatchWarning, ProfileWarning, RequestMeta, Usage};
use crate::core::text::{ModelName, Url};

/// Who answered, how, at what cost, from a backend or from a recording.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct Meta {
    tool: String,
    question_sha256: String,
    url: Url,
    model: ModelName,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<Usage>,
    requests_sent: u64,
    cached: bool,
    requests: Vec<String>,
    failed_questions: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_warning: Option<ProfileWarning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch: Option<BatchMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_warning: Option<BatchWarning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attempts: Option<Vec<crate::public::AttemptObservation>>,
}

impl Meta {
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
            batch,
            batch_warning,
            context_sha256,
        } = request_meta;
        Self {
            tool: crate::core::version_line(version),
            question_sha256,
            url,
            model,
            usage,
            requests_sent,
            cached: replayed,
            requests,
            failed_questions,
            profile_warning,
            batch,
            batch_warning,
            context_sha256,
            attempts: None,
        }
    }

    pub(crate) fn with_attempts(
        mut self,
        attempts: Vec<crate::public::AttemptObservation>,
    ) -> Self {
        if !attempts.is_empty() {
            self.attempts = Some(attempts);
        }
        self
    }
}
