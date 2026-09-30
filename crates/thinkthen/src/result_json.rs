//! The detailed document one judgment prints. The command's `--details` row
//! and the library's `Details::to_json` both write it here, so their bytes agree.

use crate::core::{
    Backend, BatchSetting, BatchWarning, DecisionResult, Meta, ProfileName, ProfileWarning,
    Question, Record, RenderError, RequestMeta, Threshold, Value, json_line,
    question_sha256_with_profile,
};
use crate::engine::facade::Judgment;

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

/// One `thinkthen.result/1` line. `shown` is the value the row prints, and
/// `input` is the record a record row carries, and `requests` its question
/// keys.
///
/// # Errors
///
/// Returns [`RenderError`] when the question or the document cannot be written.
#[expect(
    clippy::too_many_arguments,
    reason = "the question keys belong to the same result document"
)]
pub(crate) fn decision(
    run: Run<'_>,
    judged: &Judgment,
    question: Question,
    threshold: Option<Threshold>,
    shown: Value,
    input: Option<Record>,
    requests: Vec<String>,
) -> Result<(String, String), RenderError> {
    let digest = question_sha256_with_profile(&question, threshold, run.tuned_for)?;
    let json = decision_with_digest(
        run,
        judged,
        question,
        threshold,
        shown,
        input,
        Some(requests),
        &digest,
        Vec::new(),
    )?;
    Ok((json, digest))
}

/// One command row, naming its question keys and its requests' attempts.
#[expect(
    clippy::too_many_arguments,
    reason = "the keys and attempts belong to the same result document"
)]
pub(crate) fn decision_row(
    run: Run<'_>,
    judged: &Judgment,
    question: Question,
    threshold: Option<Threshold>,
    shown: Value,
    input: Option<Record>,
    requests: Vec<String>,
    attempts: Vec<crate::public::AttemptObservation>,
) -> Result<String, RenderError> {
    let digest = question_sha256_with_profile(&question, threshold, run.tuned_for)?;
    decision_with_digest(
        run,
        judged,
        question,
        threshold,
        shown,
        input,
        Some(requests),
        &digest,
        attempts,
    )
}

/// One row that lists its question keys in `meta.requests`, by ADR 0111.
#[expect(
    clippy::too_many_arguments,
    reason = "the request list belongs to this member's result metadata"
)]
pub(crate) fn decision_with_requests(
    run: Run<'_>,
    judged: &Judgment,
    question: Question,
    threshold: Option<Threshold>,
    shown: Value,
    input: Option<Record>,
    requests: Vec<String>,
) -> Result<String, RenderError> {
    let digest = question_sha256_with_profile(&question, threshold, run.tuned_for)?;
    decision_with_digest(
        run,
        judged,
        question,
        threshold,
        shown,
        input,
        Some(requests),
        &digest,
        Vec::new(),
    )
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
) -> Result<String, RenderError> {
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
    .with_attempts(attempts);
    let row = DecisionResult::new(shown, question, judged.answer.clone(), threshold, meta);
    json_line(&match input {
        Some(record) => row.with_input(record),
        None => row,
    })
}
