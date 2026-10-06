//! Native aggregate observers keep source occurrences and stage identities together.
use super::observation::{ObservedQuestion, QuestionDetail, RecordObservation};
use crate::core::{self, Backend, Threshold};
use crate::engine::{error::Error as EngineError, facade::Answered};
use crate::public::{InputFunction, QuestionInput, options::Stop};
use std::sync::Arc;

pub(crate) fn observe_question(
    stop: &Stop<'_>,
    backend: &Backend,
    reading: (
        InputFunction,
        &'static str,
        &core::Question,
        Option<Threshold>,
    ),
    answered: &Answered,
    positions: &mut [usize; 4],
) -> Result<(), EngineError> {
    observe_question_at(stop, backend, reading, answered, positions, (0, None))
}

fn stage_slot(stage: &str) -> Option<usize> {
    match stage {
        "boundary" => Some(0),
        "kind" => Some(1),
        "edge" => Some(2),
        "relation" => Some(3),
        _ => None,
    }
}

/// Name one answered logical question of a `recognize` or `relate` step.
pub(crate) fn observe_question_at(
    stop: &Stop<'_>,
    backend: &Backend,
    (function, stage, question, threshold): (
        crate::public::InputFunction,
        &'static str,
        &core::Question,
        Option<Threshold>,
    ),
    answered: &Answered,
    positions: &mut [usize; 4],
    (index, input): (usize, Option<&Arc<QuestionInput>>),
) -> Result<(), EngineError> {
    if !stop.observing() {
        return Ok(());
    }
    let [outcome] = answered.reply.outcomes() else {
        return Err(EngineError::Defect(
            "an observed question has more than one answer",
        ));
    };
    let place = stage_slot(stage).ok_or(EngineError::Defect("an observed stage is unknown"))?;
    let current = positions
        .get_mut(place)
        .ok_or(EngineError::Defect("an observed stage has no counter"))?;
    let position = *current;
    *current += 1;
    let detail = ObservedQuestion::from_reply(
        question,
        None,
        None,
        backend,
        outcome,
        &answered.reply,
        answered.request.as_str(),
        answered.requests_sent,
        answered.replayed,
        1,
        0,
    )
    .map(|detail| detail.with_receipt(answered).with_threshold(threshold))
    .and_then(|detail| detail.qualified(function, index, None, Some(stage), position))
    .map_err(|_| EngineError::Defect("an observed question identity could not be constructed"))?;
    let detail = match input {
        Some(input) => detail.with_input(Arc::clone(input)),
        None => detail,
    };
    stop.observe(RecordObservation::Question {
        index,
        member: None,
        stage: Some(stage),
        position,
        detail: QuestionDetail::of(&detail),
    });
    if stop.observer_panicked() {
        return Err(EngineError::Defect("the question observer panicked"));
    }
    Ok(())
}
