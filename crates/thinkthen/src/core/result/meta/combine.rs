//! Checked aggregate of actual per-member metadata, without process totals.
use super::Meta;
use crate::core::{ModelName, RenderError, ReportedSum};
impl Meta {
    pub(crate) fn combined(
        digest: String,
        requested_model: &ModelName,
        members: &[&Self],
    ) -> Result<Self, RenderError> {
        let mut total = (*members.first().ok_or(RenderError)?).clone();
        let mut usage = ReportedSum::default();
        total.question_sha256 = digest;
        total.requests.clear();
        total.requests_sent = 0;
        total.failed_questions = 0;
        total.cached = true;
        total.attempts = members
            .iter()
            .any(|member| member.attempts.is_some())
            .then(Vec::new);
        for member in members {
            usage.add(member.reported_usage);
            total.requests.extend(member.requests.iter().cloned());
            total.requests_sent = total
                .requests_sent
                .checked_add(member.requests_sent)
                .ok_or(RenderError)?;
            total.failed_questions = total
                .failed_questions
                .checked_add(member.failed_questions)
                .ok_or(RenderError)?;
            total.cached &= member.cached;
            if let Some(attempts) = &mut total.attempts {
                attempts.extend(member.attempts.iter().flatten().cloned());
            }
        }
        if members.iter().any(|member| member.model != total.model) {
            total.model = requested_model.clone();
        }
        total.reported_usage = usage.total().map_err(|()| RenderError)?;
        total.usage = total
            .reported_usage
            .and_then(crate::core::ReportedUsage::complete);
        Ok(total)
    }
}
