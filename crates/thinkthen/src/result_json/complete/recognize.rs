//! Complete recognition uses actual stage trace and checked aggregate usage.
use super::aggregate::{Totals, meta};
use crate::core::{self, RenderError};
use crate::engine::facade;
pub(crate) struct RecognitionRow {
    pub(crate) ordinal: usize,
    pub(crate) input: Option<core::Record>,
    pub(crate) context_sha256: Option<String>,
    pub(crate) attempts: Option<Vec<core::AttemptObservation>>,
}
pub(crate) fn recognition(
    engine: &facade::Engine,
    spec: &core::RecognizeSpec,
    found: facade::Recognition,
    row: RecognitionRow,
) -> Result<core::CompleteRecognition, RenderError> {
    let identity = found.meta.trace.identity(
        core::image::InputFunction::Recognize,
        &core::RecordScope {
            record: row.ordinal,
        },
        spec,
        &[],
    )?;
    let aggregate = found.meta;
    let meta = meta(
        engine,
        core::recognize_sha256(spec).map_err(|_| RenderError)?,
        Totals {
            model: aggregate.model,
            usage: aggregate.usage,
            reported: aggregate.reported_usage,
            cached: !aggregate.live,
            sent: aggregate.requests_sent,
            requests: aggregate.requests,
            failed: 0,
        },
        row.context_sha256,
        row.attempts,
        spec.profile.as_ref(),
    );
    Ok(core::CompleteRecognition {
        source: None,
        identity,
        value: found.value,
        input: row.input,
        question: spec.clone(),
        answer: found.details,
        meta,
    })
}
