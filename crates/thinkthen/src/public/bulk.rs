//! The engine calls over many records: the lazy batches, `rank`, and `find`.

mod annotate_observation;
mod complete;
pub(in crate::public) use complete::count_message;
mod details;
pub(crate) mod observation;
pub(crate) use annotate_observation::{observe_annotated, observe_annotated_questions};
pub(crate) mod annotation;

use crate::core::{self, Value};
use crate::engine::pipeline::Failed;
use crate::public::asking::{self, Decisions, Miss};
use crate::public::batch::Batch;
use crate::public::engine::{DecisionQuestion, Engine, Evidence};
use crate::public::error::Error;
use crate::public::options::{CallOptions, Stop};
use crate::public::pull;
use crate::public::question::Question;
use crate::public::results::{
    self, Answer, ObservedQuestion, ObservedRow, QuestionDetail, RecordObservation, Row,
};

/// One record's named values and bounded per-member observations.
pub(crate) struct Values {
    pub(crate) values: Vec<(String, core::AnnotatedValue)>,
    pub(crate) observed: Vec<(String, ObservedQuestion)>,
}

/// One record's answer and its probability of yes.
type Decided = (Value, f64);
type Pair<I, T> = fn(I, Decided) -> Result<Option<T>, Error>;

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

impl Engine {
    /// Each record with its answer, lazily, in input order.
    pub fn decide_many<'a, I, Q: DecisionQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        records: I,
    ) -> Batch<'a, Row<I::Item, Answer>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.decide_many_with(question, records, CallOptions::new())
    }

    /// [`Engine::decide_many`] under these controls.
    pub fn decide_many_with<'a, I, Q: DecisionQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<I::Item, Answer>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.try_decide_input_many_with(question, records.into_iter().map(Ok), options)
    }

    pub(crate) fn try_decisions<'a, I, R: super::InputEvidence + 'a, T: 'a>(
        &self,
        function: crate::InputFunction,
        question: &Question,
        records: I,
        options: CallOptions<'a>,
        pair: Pair<R, T>,
    ) -> Result<Batch<'a, T>, Error>
    where
        I: IntoIterator<Item = Result<R, Error>> + 'a,
    {
        crate::public::request::pull::project(
            self,
            function,
            question.clone(),
            records
                .into_iter()
                .map(|row| row.map(crate::public::record_input::plain)),
            options,
            move |original, _, canonical| {
                pair(
                    original,
                    (
                        canonical.value().clone(),
                        canonical.answer().yes().unwrap_or_default(),
                    ),
                )
            },
        )
    }
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
