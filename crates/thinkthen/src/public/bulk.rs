//! The engine calls over many records: the lazy batches, `rank`, and `find`.

use std::sync::Arc;

use crate::core::{self, Find, Plan, ranking};
use crate::engine::facade::{self, Completed};
use crate::public::annotated::AnnotatedRecord;
use crate::public::batch::{self, Batch};
use crate::public::engine::{DECISIONS, DecisionQuestion, Engine, Evidence, evidence, only};
use crate::public::error::Error;
use crate::public::options::{CallOptions, Stop};
use crate::public::question::{Kind, Question};
use crate::public::results::{self, Answer, Found, Ranked, Row, Written};
use crate::public::set::QuestionSet;

/// One record's named values, in set order, and their bare JSON line.
type Values = (Vec<(String, core::AnnotatedValue)>, Written);

/// One record's answer and its probability of yes.
type Decided = (Answer, f64);

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
            self.decisions(question, records, options, |item, (answer, _)| {
                (answer == Answer::Yes).then_some(item)
            })
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
            self.decisions(question, records, options, |item, (answer, yes)| {
                Some(Row::new(item, answer, yes))
            })
        }))
    }

    /// Every record, most likely yes first; ties keep input order.
    ///
    /// # Errors
    ///
    /// The first record's [`Error`], and [`Error::Usage`] for another kind of
    /// question or more records than the engine's request limit.
    pub fn rank<I>(&self, question: &Question, records: I) -> Result<Vec<Ranked<I::Item>>, Error>
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
    pub fn rank_with<I>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Vec<Ranked<I::Item>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        only(question, &[Kind::Rank], "rank")?;
        let records = self.within_limit(records)?;
        let rows = self
            .decisions(question, records, options, |item, (_, yes)| {
                Some(Ranked::new(item, yes))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        let order = ranking(
            &rows.iter().map(Ranked::probability).collect::<Vec<_>>(),
            None,
        );
        let mut rows: Vec<Option<Ranked<I::Item>>> = rows.into_iter().map(Some).collect();
        Ok(order
            .into_iter()
            .filter_map(|place| rows.get_mut(place).and_then(Option::take))
            .collect())
    }

    /// Select the one unit that best answers the question, from 2 to 255.
    /// A question from [`Question::offering_none`] takes 2 to 254 units and
    /// may select none.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`], and [`Error::Usage`] for another kind of
    /// question or a unit count outside its range.
    pub fn find<I>(&self, question: &Question, units: I) -> Result<Found<I::Item>, Error>
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
    ) -> Result<Found<I::Item>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
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
        let stop = Stop::begin(options)?;
        let found = stop.run(|cancel| engine.find(&find, cancel).map_err(Error::from))?;
        Found::new(units, none, &found)
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
        Batch::of(Stop::begin(options).map(|stop| {
            let (engine, set, cancel) =
                (Arc::clone(&self.inner), questions.0.clone(), stop.shared());
            let worker = Arc::clone(&engine);
            let answer: Arc<batch::Answer<_>> =
                Arc::new(move |text: &str| annotated(&worker, &set, text, &cancel));
            batch::start(
                engine,
                records.into_iter(),
                stop,
                self.most,
                answer,
                |item, (values, json)| Some(AnnotatedRecord::new(item, values, json)),
            )
        }))
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
        pair: fn(I::Item, Decided) -> Option<T>,
    ) -> Result<Batch<'a, T>, Error>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        let stop = Stop::begin(options)?;
        let engine = self.asking(question)?;
        let (worker, core, threshold, cancel) = (
            Arc::clone(&engine),
            question.core.clone(),
            question.threshold,
            stop.shared(),
        );
        let answer: Arc<batch::Answer<Decided>> = Arc::new(move |text: &str| {
            let judged = worker.judge(&core, threshold, evidence(text)?, &cancel)?;
            Ok(Completed::one(
                (
                    results::answer(&judged.value),
                    judged.answer.yes().unwrap_or_default(),
                ),
                judged.answered.replayed,
                false,
            ))
        });
        Ok(batch::start(
            engine,
            records.into_iter(),
            stop,
            self.most,
            answer,
            pair,
        ))
    }
}

/// Answer every question of the set for one record.
fn annotated(
    engine: &facade::Engine,
    set: &core::QuestionSet,
    text: &str,
    cancel: &crate::engine::Cancel<'_>,
) -> Result<Completed<Values, Error>, Error> {
    let record = record(set, text)?;
    let parts = set
        .groups()
        .into_iter()
        .map(|places| Ok((set.group_evidence(&places, &record)?, places)))
        .collect::<Result<Vec<_>, core::PartError>>()
        .map_err(|error| match error {
            core::PartError::Record(error) => Error::usage(error.to_string()),
            core::PartError::Reading(_) => {
                Error::defect("a checked question set could not read its parts")
            }
        })?;
    let model = engine.backend().model();
    let plan = |places: &[usize]| {
        let part = parts
            .iter()
            .find(|(_, held)| held == places)
            .map(|(part, _)| part.clone())
            .ok_or(crate::engine::error::Error::Defect(
                "an annotate group has no part",
            ))?;
        let questions = places
            .iter()
            .filter_map(|place| set.questions().get(*place));
        Plan::new(
            part,
            model.clone(),
            questions.map(|named| named.question().clone()).collect(),
        )
        .map_err(|_| crate::engine::error::Error::Defect("an annotate group asks nothing"))
    };
    let annotation = engine.annotate(set, plan, cancel)?;
    let json = Written::of(&core::NamedValues::new(annotation.values.clone()))?;
    Ok(Completed::one(
        (annotation.values, json),
        annotation.replayed,
        annotation.failed_questions > 0,
    ))
}

/// One record as its groups read it. Only a part group parses the text, once,
/// as the command reads a whole document, so a root-only set sends it as given.
fn record(set: &core::QuestionSet, text: &str) -> Result<core::BatchRecord, Error> {
    let evidence = evidence(text)?;
    if set.first_part().is_none() {
        let value = core::Json::String(text.to_owned());
        return Ok(core::BatchRecord { evidence, value });
    }
    let usage = |error: core::RecordError| Error::usage(error.to_string());
    let reading = core::Reading::new(core::Framing::Document, Vec::new())
        .map_err(|_| Error::defect("a document reading takes no pointer"))?;
    let held = reading.annotation_record(text.as_bytes()).map_err(usage)?;
    let value = reading.batch_record(&held).map_err(usage)?.value;
    Ok(core::BatchRecord { evidence, value })
}
