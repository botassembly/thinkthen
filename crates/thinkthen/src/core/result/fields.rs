//! Borrowed metadata fields shared by concrete complete consumer views.
use super::{AnnotateMeta, Meta};
use crate::core::{AttemptObservation, BatchSetting, BatchWarning, ProfileWarning, ReportedUsage};

#[derive(Clone, Copy)]
pub(crate) struct Fields<'a> {
    pub(crate) tool: &'a str,
    pub(crate) digest: &'a str,
    pub(crate) plural: bool,
    pub(crate) url: &'a str,
    pub(crate) model: &'a str,
    pub(crate) usage: Option<ReportedUsage>,
    pub(crate) sent: u64,
    pub(crate) requests: &'a [String],
    pub(crate) failed: usize,
    pub(crate) profile: Option<&'a ProfileWarning>,
    pub(crate) batch: Option<BatchSetting>,
    pub(crate) batch_warning: Option<&'a BatchWarning>,
    pub(crate) context: Option<&'a str>,
    pub(crate) attempts: Option<&'a [AttemptObservation]>,
}
impl Meta {
    pub(crate) fn fields(&self) -> Fields<'_> {
        Fields {
            tool: &self.tool,
            digest: &self.question_sha256,
            plural: false,
            url: self.url.as_str(),
            model: self.model.as_str(),
            usage: self.reported_usage,
            sent: self.requests_sent,
            requests: &self.requests,
            failed: self.failed_questions,
            profile: self.profile_warning.as_ref(),
            batch: self.batch_setting,
            batch_warning: self.batch_warning.as_ref(),
            context: self.context_sha256.as_deref(),
            attempts: self.attempts.as_deref(),
        }
    }
}
impl AnnotateMeta {
    pub(crate) fn fields(&self) -> Fields<'_> {
        Fields {
            tool: &self.tool,
            digest: &self.questions_sha256,
            plural: true,
            url: self.url.as_str(),
            model: self.model.as_str(),
            usage: self.reported_usage,
            sent: self.requests_sent,
            requests: &self.requests,
            failed: self.failed_questions,
            profile: self.profile_warning.as_ref(),
            batch: None,
            batch_warning: None,
            context: None,
            attempts: None,
        }
    }
}
