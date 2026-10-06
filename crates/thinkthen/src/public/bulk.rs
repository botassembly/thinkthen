//! The engine calls over many records: the lazy batches, `rank`, and `find`.

mod annotate_observation;
mod complete;
mod details;
pub(crate) mod functions;
mod observation;
pub(crate) use annotate_observation::{observe_annotated, observe_annotated_questions};
mod annotation;

use crate::core::{self, Value};
use crate::engine::pipeline::Failed;
use crate::public::asking::{self, Decisions, Miss};
use crate::public::batch::Batch;
use crate::public::choice::Choice;
use crate::public::engine::{DECISIONS, DecisionQuestion, Engine, Evidence, evidence, only};
use crate::public::error::{Error, ErrorKind};
use crate::public::options::{CallOptions, Stop};
use crate::public::pull;
use crate::public::question::{ChooseQuestion, Kind, Question, TagQuestion};
use crate::public::results::{
    self, Answer, ObservedQuestion, ObservedRow, QuestionDetail, RecordObservation, Row, Written,
};

/// One record's named values and bounded per-member observations.
pub(crate) struct Values {
    pub(crate) values: Vec<(String, core::AnnotatedValue)>,
    pub(crate) json: Written,
    pub(crate) observed: Vec<(String, ObservedQuestion)>,
}

/// One record's answer and its probability of yes.
type Decided = (Value, f64);
type Pair<I, T> = fn(I, Decided) -> Result<Option<T>, Error>;

fn invalid_answer() -> Error {
    Error::of(ErrorKind::Backend, "a backend question failed in a batch")
}

pub(super) fn selected_batch(
    question: &Question,
    options: &CallOptions<'_>,
    engine: Option<core::Setting>,
) -> Result<core::Setting, Error> {
    if let Some(typed) = options.batch_setting() {
        return Ok(typed.into());
    }
    if let Some(engine) = engine {
        return Ok(engine);
    }
    let Some(file) = question.batch.as_ref() else {
        return Ok(core::Setting::Max);
    };
    core::Setting::of_json(file).ok_or_else(|| {
        Error::refused(core::QuestionFileError::Shape {
            key: "batch",
            wanted: "takes max or a whole number of at least 1",
        })
    })
}

pub(super) fn selected_set_batch(
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
    /// The records whose answer is yes, lazily, in input order.
    pub fn filter<'a, I>(&'a self, question: &'a Question, records: I) -> Batch<'a, I::Item>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.filter_with(question, records, CallOptions::new())
    }

    /// [`Engine::filter`] under these controls.
    pub fn filter_with<'a, I>(
        &'a self,
        question: &'a Question,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, I::Item>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.try_filter_with(question, records.into_iter().map(Ok), options)
    }

    /// Fallible input for [`Engine::filter_with`], preserving ordered matches.
    pub fn try_filter_with<'a, I, T>(
        &'a self,
        question: &'a Question,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, T>
    where
        I: IntoIterator<Item = Result<T, Error>> + 'a,
        T: Evidence + 'a,
    {
        Batch::of(only(question, &[Kind::Decide], "filter").and_then(|()| {
            self.try_decisions(
                question,
                records,
                options,
                |item, (answer, _)| match answer {
                    Value::YesNo(value) => Ok((value == Some(true)).then_some(item)),
                    _ => Err(invalid_answer()),
                },
            )
        }))
    }

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
        let question = question.question();
        Batch::of(only(question, DECISIONS, "decide_many").and_then(|()| {
            self.decisions(
                question,
                records,
                options,
                |item, (answer, yes)| match answer {
                    Value::YesNo(value) => Ok(Some(Row::new(
                        item,
                        match value {
                            Some(true) => Answer::Yes,
                            Some(false) => Answer::No,
                            None => Answer::Unsure,
                        },
                        yes,
                    ))),
                    _ => Err(invalid_answer()),
                },
            )
        }))
    }

    /// Each record's typed choice, including an unresolved `None`, in input order.
    pub fn choose_many<'a, I, C: Choice>(
        &'a self,
        question: &'a ChooseQuestion<C>,
        records: I,
    ) -> Batch<'a, Row<I::Item, Option<C>>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.choose_many_with(question, records, CallOptions::new())
    }

    /// [`Engine::choose_many`] under these controls.
    pub fn choose_many_with<'a, I, C: Choice>(
        &'a self,
        question: &'a ChooseQuestion<C>,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<I::Item, Option<C>>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        Batch::of(
            self.decisions(&question.0, records, options, |item, (answer, _)| {
                let value = match answer {
                    Value::Choice(Some(label)) => {
                        Some(C::from_label(&label).ok_or_else(invalid_answer)?)
                    }
                    Value::Choice(None) => None,
                    _ => return Err(invalid_answer()),
                };
                Ok(Some(Row::new(item, value, 0.0)))
            }),
        )
    }

    /// Each record's position on the declared scale, in input order.
    pub fn score_many<'a, I>(
        &'a self,
        question: &'a Question,
        records: I,
    ) -> Batch<'a, Row<I::Item, f64>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.score_many_with(question, records, CallOptions::new())
    }

    /// [`Engine::score_many`] under these controls.
    pub fn score_many_with<'a, I>(
        &'a self,
        question: &'a Question,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<I::Item, f64>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        Batch::of(only(question, &[Kind::Score], "score_many").and_then(|()| {
            self.decisions(
                question,
                records,
                options,
                |item, (answer, _)| match answer {
                    Value::Score(value) => Ok(Some(Row::new(item, value, 0.0))),
                    _ => Err(invalid_answer()),
                },
            )
        }))
    }

    /// Each record's labels in declared order, in input order.
    pub fn tag_many<'a, I, C: Choice>(
        &'a self,
        question: &'a TagQuestion<C>,
        records: I,
    ) -> Batch<'a, Row<I::Item, Vec<C>>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.tag_many_with(question, records, CallOptions::new())
    }

    /// [`Engine::tag_many`] under these controls.
    pub fn tag_many_with<'a, I, C: Choice>(
        &'a self,
        question: &'a TagQuestion<C>,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<I::Item, Vec<C>>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        Batch::of(
            self.decisions(&question.0, records, options, |item, (answer, _)| {
                let Value::Tag(labels) = answer else {
                    return Err(invalid_answer());
                };
                let values = labels
                    .iter()
                    .map(|label| C::from_label(label).ok_or_else(invalid_answer))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Some(Row::new(item, values, 0.0)))
            }),
        )
    }

    fn decisions<'a, I, T: 'a>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'a>,
        pair: Pair<I::Item, T>,
    ) -> Result<Batch<'a, T>, Error>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.try_decisions(question, records.into_iter().map(Ok), options, pair)
    }

    pub(crate) fn try_decisions<'a, I, R: super::InputEvidence + 'a, T: 'a>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'a>,
        pair: Pair<R, T>,
    ) -> Result<Batch<'a, T>, Error>
    where
        I: IntoIterator<Item = Result<R, Error>> + 'a,
    {
        let setting = selected_batch(question, &options, self.batch)?;
        let context = options.context_text().map(evidence).transpose()?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let engine = self.asking(question)?;
        let asker = Decisions::new(&engine, question, context.clone());
        let backend = engine.backend().clone();
        let question = question.clone();
        let call = pull::Call {
            packing: pull::packing(setting, context.is_some(), false),
            engine,
            stop,
            most: self.most,
        };
        Ok(pull::try_start(
            call,
            asker,
            records.into_iter(),
            Box::new(move |stop, index, item, row| {
                let judged = judged(stop, &question, &backend, index, row)?;
                let item = item.ok_or_else(|| Error::defect("a row arrived with no record"))?;
                pair(
                    item,
                    (judged.value, judged.answer.yes().unwrap_or_default()),
                )
            }),
        ))
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
        Ok(decided) | Err(Failed::Asker(Miss::Failed(decided))) => Some(decided),
        Err(_) => None,
    };
    if let Some(decided) = decided.filter(|_| stop.observing()) {
        let observed = decided.observed(question, backend)?;
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
