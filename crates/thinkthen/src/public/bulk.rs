//! The engine calls over many records: the lazy batches, `rank`, and `find`.

use std::sync::Arc;

mod observation;
use observation::observe_find;
mod annotate_observation;
mod details;
pub(crate) use annotate_observation::{observe_annotated, observe_annotated_questions};
mod annotation;
pub(crate) use annotation::{record as annotation_record, rendered as render_annotation};

use crate::core::{self, Find, Value, ranking};
use crate::public::annotated::AnnotatedRecord;
use crate::public::batch::{self, Batch};
use crate::public::choice::Choice;
use crate::public::engine::{DECISIONS, DecisionQuestion, Engine, Evidence, evidence, only};
use crate::public::error::{Error, ErrorKind};
use crate::public::options::{CallOptions, Stop};
use crate::public::question::{ChooseQuestion, Kind, Question, TagQuestion};
use crate::public::results::{Answer, Call, Found, ObservedQuestion, Ranked, Row, Written};
use crate::public::set::QuestionSet;

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

fn selected_set_batch(
    questions: &QuestionSet,
    options: &CallOptions<'_>,
    engine: Option<core::Setting>,
) -> Result<core::Setting, Error> {
    if let Some(typed) = options.batch_setting() {
        return Ok(typed.into());
    }
    if let Some(engine) = engine {
        return Ok(engine);
    }
    let Some(file) = questions.0.batch() else {
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
        Batch::of(only(question, &[Kind::Decide], "filter").and_then(|()| {
            self.decisions(
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

    /// Every record, most likely yes first; ties keep input order.
    ///
    /// # Errors
    ///
    /// The first record's [`Error`], and [`Error::Usage`] for another kind of
    /// question or more records than the engine's request limit.
    #[allow(
        clippy::type_complexity,
        reason = "the public return carries ranked rows and facts"
    )]
    pub fn rank<I>(
        &self,
        question: &Question,
        records: I,
    ) -> Result<Call<Vec<Ranked<I::Item>>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.rank_with(question, records, CallOptions::new())
    }

    /// [`Engine::rank`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::rank`].
    #[allow(
        clippy::type_complexity,
        reason = "the public return carries ranked rows and facts"
    )]
    pub fn rank_with<I>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<Ranked<I::Item>>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        only(question, &[Kind::Rank], "rank")?;
        let records = self.within_limit(records)?;
        let mut batch = self.decisions(question, records, options, |item, (_, yes)| {
            Ok(Some(Ranked::new(item, yes)))
        })?;
        let rows = batch.by_ref().collect::<Result<Vec<_>, _>>()?;
        let facts = batch
            .facts()
            .cloned()
            .ok_or_else(|| Error::defect("a completed rank has no facts"))?;
        let order = ranking(
            &rows.iter().map(Ranked::probability).collect::<Vec<_>>(),
            None,
        );
        let mut rows: Vec<Option<Ranked<I::Item>>> = rows.into_iter().map(Some).collect();
        Ok(Call::new(
            order
                .into_iter()
                .filter_map(|place| rows.get_mut(place).and_then(Option::take))
                .collect(),
            facts,
        ))
    }

    /// Select the one unit that best answers the question, from 2 to 255.
    /// A question from [`Question::offering_none`] takes 2 to 254 units and
    /// may select none.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`], and [`Error::Usage`] for another kind of
    /// question or a unit count outside its range.
    pub fn find<I>(&self, question: &Question, units: I) -> Result<Call<Found<I::Item>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.find_with(question, units, CallOptions::new())
    }

    /// [`Engine::find`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::find`].
    pub fn find_with<I>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Found<I::Item>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        options.without_context("find")?;
        only(question, &[Kind::Find, Kind::FindNone], "find")?;
        let none = question.kind == Kind::FindNone;
        let units: Vec<I::Item> = self.within_limit(units)?.collect();
        let texts = units
            .iter()
            .map(|unit| evidence(unit.evidence()))
            .collect::<Result<Vec<_>, _>>()?;
        let core::Question::Decide { text, .. } = &question.core else {
            return Err(Error::defect("a find question held no text"));
        };
        let engine = self.asking(question)?;
        let find = Find::new(text.clone(), &texts, engine.backend().model().clone(), none)
            .map_err(|_| {
                Error::usage(if none {
                    "a find question offering none takes 2 to 254 units"
                } else {
                    "find takes 2 to 255 units"
                })
            })?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        stop.run_call(1, |cancel| {
            let found = engine.find(&find, cancel).map_err(Error::from)?;
            observe_find(&stop, &engine, question, &find, &found)?;
            Ok(found)
        })?
        .try_map(|found| Found::new(units, none, &found))
    }

    /// Each record with every value of the set, lazily, in input order. A
    /// member with an `on` pointer reads that part of the record's JSON text,
    /// and a record missing a part is refused before its first request. A
    /// question the backend failed reads [`Annotated::Failed`](crate::Annotated::Failed).
    pub fn annotate<'a, I>(
        &'a self,
        questions: &'a QuestionSet,
        records: I,
    ) -> Batch<'a, AnnotatedRecord<I::Item>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.annotate_with(questions, records, CallOptions::new())
    }

    /// [`Engine::annotate`] under these controls.
    pub fn annotate_with<'a, I>(
        &'a self,
        questions: &'a QuestionSet,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, AnnotatedRecord<I::Item>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        Batch::of((|| {
            options.without_context("annotate")?;
            let setting = selected_set_batch(questions, &options, self.batch)?;
            let stop = Stop::begin(options)?.with_prices(self.prices);
            let (engine, set) = (Arc::clone(&self.inner), questions.0.clone());
            batch::start_annotation(engine, set, records.into_iter(), stop, self.most, setting)
        })())
    }

    /// Hold a finite input whole, and refuse it over the request limit.
    pub(super) fn within_limit<I: IntoIterator>(
        &self,
        records: I,
    ) -> Result<std::vec::IntoIter<I::Item>, Error> {
        let records: Vec<I::Item> = records.into_iter().collect();
        match self.most {
            Some(most) if records.len() > most => Err(Error::usage(format!(
                "this engine answers at most {most} records in one call"
            ))),
            _ => Ok(records.into_iter()),
        }
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
        let setting = selected_batch(question, &options, self.batch)?;
        let context = options.context_text().map(evidence).transpose()?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let engine = self.asking(question)?;
        batch::start_planned(
            engine,
            records.into_iter(),
            stop,
            self.most,
            question.core.clone(),
            question.threshold,
            question.profile.clone(),
            setting,
            context,
            pair,
        )
    }
}
