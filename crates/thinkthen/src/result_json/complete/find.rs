//! Whole-set find identity and metadata shared by native and command callers.
use crate::core::{self, Meta, RenderError, RequestMeta, ResultIdentity};
use crate::engine::facade;

pub(crate) struct FindRow {
    pub(crate) declarations: core::declaration::QuestionMetadata,
    pub(crate) question: core::Question,
    pub(crate) candidates: Vec<String>,
    pub(crate) input: Option<core::Record>,
    pub(crate) context_sha256: Option<String>,
    pub(crate) attempts: Option<Vec<core::AttemptObservation>>,
}
#[derive(serde::Serialize)]
struct Reading<'a> {
    question: &'a core::Question,
    candidates: &'a [String],
    profile: Option<&'a core::ProfileName>,
}
pub(crate) fn find(
    engine: &facade::Engine,
    find: &core::Find,
    found: facade::Found,
    row: FindRow,
) -> Result<core::CompleteFind, RenderError> {
    let identity = ResultIdentity::of(
        core::image::InputFunction::Find,
        &core::RecordScope { record: 0 },
        found.answered.sources.clone(),
        found.answered.observations.clone(),
        &Reading {
            question: &row.question,
            candidates: &row.candidates,
            profile: find.profile(),
        },
        &[],
    )?;
    let answered = &found.answered;
    let meta = Meta::new(
        env!("CARGO_PKG_VERSION"),
        find.question_sha256().map_err(|_| RenderError)?,
        engine.backend().url().clone(),
        answered.reply.model().clone(),
        answered.reply.usage(),
        RequestMeta::new(
            answered.replayed,
            answered.requests_sent,
            vec![answered.request.as_str().to_owned()],
        )
        .with_profile_warning(core::ProfileWarning::between(
            find.profile(),
            engine.profile().map(core::BackendProfile::name),
        ))
        .with_context_sha256(row.context_sha256),
    )
    .with_reported_usage(answered.reply.reported_usage())
    .with_captured_attempts(row.attempts);
    Ok(core::CompleteFind {
        declarations: row.declarations,
        identity,
        legacy: find.result(row.input, found.selection, meta),
    })
}
