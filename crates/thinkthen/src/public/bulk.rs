//! The engine calls over many records: the lazy batches, `rank`, and `find`.

mod annotate_observation;
mod complete;
pub(in crate::public) use complete::count_message;
#[cfg(test)]
mod details;
pub(crate) mod observation;
pub(crate) use annotate_observation::{observe_annotated, observe_annotated_questions};
pub(crate) mod annotation;

use crate::core;
use crate::engine::pipeline::Failed;
use crate::public::asking::{self, Decisions, Miss};
use crate::public::error::Error;
use crate::public::options::{CallOptions, Stop};
use crate::public::pull;
use crate::public::question::Question;
use crate::public::results::{
    self, ObservedQuestion, ObservedRow, QuestionDetail, RecordObservation,
};

/// One record's named values and bounded per-member observations.
pub(crate) struct Values {
    pub(crate) values: Vec<(String, core::AnnotatedValue)>,
    pub(crate) observed: Vec<(String, ObservedQuestion)>,
}

pub(super) fn selected_batch(
    question: &Question,
    options: &CallOptions<'_>,
    engine: Option<core::Setting>,
) -> Result<core::Setting, Error> {
    selected_batch_file(question.batch.as_ref(), options, engine)
}

pub(crate) fn selected_batch_file(
    file: Option<&core::Json>,
    options: &CallOptions<'_>,
    engine: Option<core::Setting>,
) -> Result<core::Setting, Error> {
    if let Some(typed) = options.batch_setting() {
        return Ok(typed.into());
    }
    if let Some(engine) = engine {
        return Ok(engine);
    }
    let Some(file) = file else {
        return Ok(core::Setting::Max);
    };
    core::Setting::of_json(file).ok_or_else(|| {
        Error::refused(core::QuestionFileError::Shape {
            key: "batch",
            wanted: "takes max or a whole number of at least 1",
        })
    })
}

pub(crate) fn selected_set_batch(
    questions: &core::QuestionSet,
    options: &CallOptions<'_>,
    engine: Option<core::Setting>,
) -> Result<core::Setting, Error> {
    if let Some(typed) = options.batch_setting() {
        return Ok(typed.into());
    }
    if let Some(engine) = engine {
        return Ok(engine);
    }
    let Some(file) = questions.batch() else {
        return Ok(core::Setting::Max);
    };
    core::Setting::of_json(file).ok_or_else(|| {
        Error::refused(core::QuestionSetError::Shape {
            path: "batch".to_owned(),
            wanted: "takes max or a whole number of at least 1",
        })
    })
}

/// One pipeline row as its judgment, after the observer sees its question
/// and its row. A failed row is the batch's error.
pub(crate) fn judged(
    stop: &Stop<'_>,
    question: &Question,
    backend: &core::Backend,
    index: usize,
    row: pull::Row<Decisions>,
) -> Result<crate::engine::facade::Judgment, Error> {
    let decided = match &row {
        Ok(decided) => Some(decided),
        Err(Failed::Asker(Miss::Failed(decided))) => Some(decided.as_ref()),
        Err(_) => None,
    };
    if let Some(decided) = decided.filter(|_| stop.observing()) {
        let observed = decided.observed(question, backend)?.qualified(
            crate::public::asking::observed_function(question),
            index,
            None,
            None,
            0,
        )?;
        stop.observe(RecordObservation::Question {
            index,
            member: None,
            stage: None,
            position: 0,
            detail: QuestionDetail::of(&observed),
        });
        if stop.observer_panicked() {
            return Err(Error::cancelled());
        }
    }
    let decided = row.map_err(asking::failure)?;
    let judged = decided
        .judgment(question)
        .ok_or_else(|| Error::defect("an answered row held no answer"))?;
    if stop.observing() {
        stop.observe(RecordObservation::Row {
            index,
            value: ObservedRow::Judgment(&results::judgment(&judged.value)),
        });
        if stop.observer_panicked() {
            return Err(Error::cancelled());
        }
    }
    Ok(judged)
}
