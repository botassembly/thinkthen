//! Complete serialization uses typed judgments and actual traces, never JSON reconstruction.
use super::{Run, decision_with_digest};
use crate::core::image::InputFunction;
use crate::core::{self, Question, RenderError, Threshold, Value};
use crate::engine::facade::Judgment;

pub(crate) struct AtomicSpec {
    pub(crate) function: InputFunction,
    pub(crate) record: usize,
    pub(crate) question: Question,
    pub(crate) threshold: Option<Threshold>,
    pub(crate) shown: Value,
    pub(crate) rank_position: Option<std::num::NonZeroUsize>,
}

pub(crate) fn atomic(
    run: Run<'_>,
    judged: &Judgment,
    spec: AtomicSpec,
    requests: Vec<String>,
    input: Option<core::Record>,
    attempts: Option<Vec<core::AttemptObservation>>,
) -> Result<core::CompleteAtomic, RenderError> {
    if requests.len() != judged.answered.sources.len() {
        return Err(RenderError);
    }
    let reading = core::AtomicReading {
        question: &spec.question,
        threshold: spec.threshold,
        rank_position: spec.rank_position,
    };
    let identity = core::ResultIdentity::of(
        spec.function,
        &core::RecordScope {
            record: spec.record,
        },
        judged.answered.sources.clone(),
        judged.answered.observations.clone(),
        &reading,
        &[],
    )?;
    let digest = core::question_sha256_with_profile(&spec.question, spec.threshold, run.tuned_for)?;
    let legacy = decision_with_digest(
        run,
        judged,
        spec.question,
        spec.threshold,
        spec.shown,
        input,
        Some(requests),
        &digest,
        Vec::new(),
    )?
    .with_captured_attempts(attempts);
    Ok(core::CompleteAtomic {
        identity,
        legacy,
        rank_position: spec.rank_position,
    })
}
