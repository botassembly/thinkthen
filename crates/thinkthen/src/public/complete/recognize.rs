//! Complete recognition delegates all stages and offsets to the existing scheduler.
use crate::core;
use crate::public::options::Stop;
use crate::public::{CompleteRecognized, Error, InputFunction, Recognize, Recognized};

pub(super) fn rendered(
    engine: &crate::engine::facade::Engine,
    ask: &Recognize,
    found: crate::engine::facade::Recognition,
    value: Recognized,
    (ordinal, context, attempts): (usize, Option<&str>, Option<Vec<core::AttemptObservation>>),
) -> Result<CompleteRecognized, Error> {
    let canonical = crate::result_json::complete::recognition(
        engine,
        &ask.0,
        found,
        crate::result_json::complete::RecognitionRow {
            ordinal,
            input: None,
            context_sha256: context
                .filter(|text| !text.is_empty())
                .map(|text| core::bytes_sha256(text.as_bytes())),
            attempts,
        },
    )
    .map_err(|_| super::wrong())?;
    Ok(CompleteRecognized {
        canonical,
        value,
        source_value: None,
    })
}

pub(super) fn execute(
    engine: &crate::engine::facade::Engine,
    ask: &Recognize,
    evidence: &str,
    cancel: &crate::engine::Cancel<'_>,
    stop: &Stop<'_>,
    scope: (usize, Option<&std::sync::Arc<crate::public::QuestionInput>>),
) -> Result<crate::engine::facade::Recognition, crate::engine::error::Error> {
    let mut positions = [0; 4];
    engine.recognize_observed(
        &ask.0,
        evidence,
        crate::engine::facade::MAX_TEXT_BYTES,
        cancel,
        |stage, question, answered| {
            crate::public::results::observe_question_at(
                stop,
                engine.backend(),
                (
                    InputFunction::Recognize,
                    stage,
                    question,
                    match stage {
                        "boundary" => None,
                        "relation" => Some(ask.0.relation_threshold),
                        _ => Some(ask.0.threshold),
                    },
                ),
                answered,
                &mut positions,
                scope,
            )
        },
    )
}
pub(crate) mod ordered;
mod records;
mod streaming;

mod preview;
pub(crate) use preview::Preview as RecognitionPreview;
pub(in crate::public) use preview::prepare as preview_recognition;
