//! The detailed document one judgment prints. The command's `--details` row
//! and the library's `Details::to_json` both write it here, so their bytes agree.

use crate::core::{
    Backend, BatchSetting, BatchWarning, DecisionResult, Meta, ProfileName, ProfileWarning,
    Question, Record, RenderError, RequestMeta, Threshold, Value,
};
use crate::engine::facade::Judgment;
pub(crate) mod bounded;
pub(crate) mod complete;

/// What a detailed row names beside the judgment: the backend's address and the
/// calibration profile the run compares.
#[derive(Clone)]
pub(crate) struct Run<'a> {
    pub(crate) backend: &'a Backend,
    pub(crate) tuned_for: Option<&'a ProfileName>,
    pub(crate) warning: Option<ProfileWarning>,
    pub(crate) batch_setting: Option<BatchSetting>,
    pub(crate) batch_warning: Option<BatchWarning>,
    pub(crate) context_sha256: Option<String>,
}

#[expect(
    clippy::too_many_arguments,
    reason = "the digest and optional batch belong to the same result document"
)]
fn decision_with_digest(
    run: Run<'_>,
    judged: &Judgment,
    question: Question,
    threshold: Option<Threshold>,
    shown: Value,
    input: Option<Record>,
    requests: Option<Vec<String>>,
    digest: &str,
    attempts: Vec<crate::public::AttemptObservation>,
) -> Result<DecisionResult, RenderError> {
    let answered = &judged.answered;
    let meta = Meta::new(
        env!("CARGO_PKG_VERSION"),
        digest.to_owned(),
        run.backend.url().clone(),
        answered.reply.model().clone(),
        answered.reply.usage(),
        RequestMeta::new(
            answered.replayed,
            answered.requests_sent,
            requests.unwrap_or_else(|| vec![answered.request.as_str().to_owned()]),
        )
        .with_profile_warning(run.warning)
        .with_batch_setting(run.batch_setting)
        .with_batch_warning(run.batch_warning)
        .with_context_sha256(run.context_sha256),
    )
    .with_reported_usage(answered.reply.reported_usage())
    .with_attempts(attempts);
    let row = DecisionResult::new(shown, question, judged.answer.clone(), threshold, meta);
    Ok(match input {
        Some(record) => row.with_input(record),
        None => row,
    })
}
