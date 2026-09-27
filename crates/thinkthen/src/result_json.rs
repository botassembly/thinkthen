//! The detailed document one judgment prints. The command's `--details` row
//! and the library's `Details::to_json` both write it here, so their bytes agree.

use crate::core::{
    Backend, BatchMeta, DecisionResult, Meta, ProfileName, ProfileWarning, Question, Record,
    RenderError, RequestMeta, Threshold, Value, json_line, question_sha256_with_profile,
};
use crate::engine::facade::Judgment;

/// What a detailed row names beside the judgment: the backend's address and the
/// calibration profile the command compares. The library compares none.
pub(crate) struct Run<'a> {
    pub(crate) backend: &'a Backend,
    pub(crate) tuned_for: Option<&'a ProfileName>,
    pub(crate) warning: Option<ProfileWarning>,
}

/// One `thinkthen.result/1` line. `shown` is the value the row prints, and
/// `input` is the record a record row carries.
///
/// # Errors
///
/// Returns [`RenderError`] when the question or the document cannot be written.
pub(crate) fn decision(
    run: Run<'_>,
    judged: &Judgment,
    question: Question,
    threshold: Option<Threshold>,
    shown: Value,
    input: Option<Record>,
) -> Result<String, RenderError> {
    decision_with_batch(run, judged, question, threshold, shown, input, None)
}

#[expect(
    clippy::too_many_arguments,
    reason = "the optional batch belongs to the same result document"
)]
pub(crate) fn decision_with_batch(
    run: Run<'_>,
    judged: &Judgment,
    question: Question,
    threshold: Option<Threshold>,
    shown: Value,
    input: Option<Record>,
    batch: Option<BatchMeta>,
) -> Result<String, RenderError> {
    let answered = &judged.answered;
    let meta = Meta::new(
        env!("CARGO_PKG_VERSION"),
        question_sha256_with_profile(&question, threshold, run.tuned_for)?,
        run.backend.url().clone(),
        answered.reply.model().clone(),
        answered.reply.usage(),
        RequestMeta::new(
            answered.replayed,
            answered.requests_sent,
            vec![answered.request.as_str().to_owned()],
        )
        .with_profile_warning(run.warning)
        .with_batch(batch),
    );
    let row = DecisionResult::new(shown, question, judged.answer.clone(), threshold, meta);
    json_line(&match input {
        Some(record) => row.with_input(record),
        None => row,
    })
}
