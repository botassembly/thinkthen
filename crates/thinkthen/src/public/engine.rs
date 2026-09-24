//! The public engine: every call checks its question, fixes its controls,
//! and hands typed values to the one private facade.

use std::fmt;
use std::sync::Arc;

use crate::core::{self, Find, Plan, Value, ranking};
use crate::engine::facade::{self, Completed, Settings};
use crate::public::batch::{self, Batch};
use crate::public::error::Error;
use crate::public::options::{CallOptions, Stop, guarded};
use crate::public::question::{Choice, ChooseQuestion, Kind, Question, TagQuestion};
use crate::public::results::{
    self, AnnotatedRecord, Answer, Counters, Details, Found, Ranked, Row,
};
use crate::public::set::QuestionSet;
use crate::public::settings::EngineBuilder;

/// One engine: its settings, its connection pool, its cache, and its counters.
///
/// Clones share everything, and one engine serves many threads at once.
/// Building sends nothing. `Debug` shows no setting.
#[derive(Clone)]
pub struct Engine {
    pub(crate) inner: Arc<facade::Engine>,
    most: Option<usize>,
}

impl fmt::Debug for Engine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Engine").finish_non_exhaustive()
    }
}

/// Text a call reads as evidence.
pub trait Evidence {
    /// The text sent as evidence.
    fn evidence(&self) -> &str;
}

impl Evidence for String {
    fn evidence(&self) -> &str {
        self
    }
}

impl Evidence for &str {
    fn evidence(&self) -> &str {
        self
    }
}

/// The question every typed call reads.
pub(crate) trait Sealed {
    fn question(&self) -> &Question;
}

/// A question `decide` and `decide_many` accept.
#[expect(
    private_bounds,
    reason = "the sealed bound keeps the question kinds closed"
)]
pub trait DecisionQuestion: Sealed {}

/// A question `details` accepts.
#[expect(
    private_bounds,
    reason = "the sealed bound keeps the question kinds closed"
)]
pub trait DetailQuestion: Sealed {}

impl Sealed for Question {
    fn question(&self) -> &Question {
        self
    }
}
impl Sealed for crate::public::question::BandedQuestion {
    fn question(&self) -> &Question {
        &self.0
    }
}
impl<C: Choice> Sealed for ChooseQuestion<C> {
    fn question(&self) -> &Question {
        &self.0
    }
}
impl<C: Choice> Sealed for TagQuestion<C> {
    fn question(&self) -> &Question {
        &self.0
    }
}
impl DecisionQuestion for Question {}
impl DecisionQuestion for crate::public::question::BandedQuestion {}
impl DetailQuestion for Question {}
impl DetailQuestion for crate::public::question::BandedQuestion {}
impl<C: Choice> DetailQuestion for ChooseQuestion<C> {}
impl<C: Choice> DetailQuestion for TagQuestion<C> {}

fn only(question: &Question, kinds: &[Kind], call: &str) -> Result<(), Error> {
    if kinds.contains(&question.kind) {
        Ok(())
    } else {
        Err(Error::usage(
            format!("{call} does not take a {:?} question", question.kind).to_lowercase(),
        ))
    }
}

pub(crate) fn evidence(text: &str) -> Result<core::Evidence, Error> {
    core::Evidence::new(text).map_err(|_| Error::usage("evidence is text, not white space"))
}

/// One record's named values, in set order.
type Values = Vec<(String, core::AnnotatedValue)>;

/// One record's answer and its probability of yes.
type Decided = (Answer, f64);

const DECISIONS: &[Kind] = &[Kind::Decide, Kind::Banded];

impl Engine {
    /// Build from what the command reads: see [`EngineBuilder::from_env`].
    ///
    /// # Errors
    ///
    /// As [`EngineBuilder::from_env`] and [`EngineBuilder::build`].
    pub fn from_env() -> Result<Self, Error> {
        EngineBuilder::from_env()?.build()
    }

    /// Start from the library defaults, which read no environment.
    #[must_use]
    pub fn builder() -> EngineBuilder {
        EngineBuilder::new()
    }

    pub(crate) fn from_settings(settings: Settings, most: Option<usize>) -> Result<Self, Error> {
        let inner = guarded(|| facade::Engine::new(settings).map_err(Error::from))?;
        Ok(Self {
            inner: Arc::new(inner),
            most,
        })
    }

    /// This process's totals, which start at zero in a forked child.
    #[must_use]
    pub fn usage(&self) -> Counters {
        let counts = guarded(|| self.inner.usage().map_err(Error::from));
        counts.map_or(Counters::ZERO, |counts| Counters::of(&counts))
    }

    /// Answer a yes or no question.
    ///
    /// # Errors
    ///
    /// Returns the call's [`Error`]; a failed answer is [`Error::Backend`].
    pub fn decide<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        evidence: &str,
    ) -> Result<Answer, Error> {
        self.decide_with(question, evidence, CallOptions::new())
    }

    /// [`Engine::decide`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`].
    pub fn decide_with<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        evidence: &str,
        options: CallOptions<'_>,
    ) -> Result<Answer, Error> {
        let question = question.question();
        only(question, DECISIONS, "decide")?;
        Ok(results::answer(
            &self.judge(question, evidence, options)?.value,
        ))
    }

    /// Pick one option of `C`, or `None` when the pick is unresolved.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`].
    pub fn choose<C: Choice>(
        &self,
        question: &ChooseQuestion<C>,
        evidence: &str,
    ) -> Result<Option<C>, Error> {
        self.choose_with(question, evidence, CallOptions::new())
    }

    /// [`Engine::choose`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`].
    pub fn choose_with<C: Choice>(
        &self,
        question: &ChooseQuestion<C>,
        evidence: &str,
        options: CallOptions<'_>,
    ) -> Result<Option<C>, Error> {
        match self.judge(&question.0, evidence, options)?.value {
            Value::Choice(Some(label)) => C::from_label(&label).map(Some).ok_or_else(unbound),
            _ => Ok(None),
        }
    }

    /// Place the evidence on the question's levels: 0 at the lowest, 1 at the highest.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`], and [`Error::Usage`] for another kind of question.
    pub fn score(&self, question: &Question, evidence: &str) -> Result<f64, Error> {
        self.score_with(question, evidence, CallOptions::new())
    }

    /// [`Engine::score`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::score`].
    pub fn score_with(
        &self,
        question: &Question,
        evidence: &str,
        options: CallOptions<'_>,
    ) -> Result<f64, Error> {
        only(question, &[Kind::Score], "score")?;
        match self.judge(question, evidence, options)?.value {
            Value::Score(position) => Ok(position),
            _ => Err(Error::defect("a score answer held no position")),
        }
    }

    /// Every label of `C` that reached the cut, in declared order.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`].
    pub fn tag<C: Choice>(
        &self,
        question: &TagQuestion<C>,
        evidence: &str,
    ) -> Result<Vec<C>, Error> {
        self.tag_with(question, evidence, CallOptions::new())
    }

    /// [`Engine::tag`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`].
    pub fn tag_with<C: Choice>(
        &self,
        question: &TagQuestion<C>,
        evidence: &str,
        options: CallOptions<'_>,
    ) -> Result<Vec<C>, Error> {
        match self.judge(&question.0, evidence, options)?.value {
            Value::Tag(labels) => labels
                .iter()
                .map(|label| C::from_label(label).ok_or_else(unbound))
                .collect(),
            _ => Err(Error::defect("a tag answer held no labels")),
        }
    }

    /// One judgment with its probabilities and request facts.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`], and [`Error::Usage`] for a `rank` or `find` question.
    pub fn details<Q: DetailQuestion + ?Sized>(
        &self,
        question: &Q,
        evidence: &str,
    ) -> Result<Details, Error> {
        self.details_with(question, evidence, CallOptions::new())
    }

    /// [`Engine::details`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::details`].
    pub fn details_with<Q: DetailQuestion + ?Sized>(
        &self,
        question: &Q,
        evidence: &str,
        options: CallOptions<'_>,
    ) -> Result<Details, Error> {
        let question = question.question();
        only(
            question,
            &[
                Kind::Decide,
                Kind::Banded,
                Kind::Choose,
                Kind::Tag,
                Kind::Score,
            ],
            "details",
        )?;
        let judged = self.judge(question, evidence, options)?;
        Details::of(&judged, &question.core, question.threshold)
    }

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
            self.decisions(question, records, options, |item, (answer, _)| {
                Some(Row::new(item, answer))
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
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`], and [`Error::Usage`] for another kind of
    /// question or a unit count outside 2 to 255.
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
        only(question, &[Kind::Find], "find")?;
        let units: Vec<I::Item> = self.within_limit(units)?.collect();
        let texts = units
            .iter()
            .map(|unit| evidence(unit.evidence()))
            .collect::<Result<Vec<_>, _>>()?;
        let core::Question::Decide { text, .. } = &question.core else {
            return Err(Error::defect("a find question held no text"));
        };
        let engine = self.asking(question)?;
        let find = Find::new(
            text.clone(),
            &texts,
            engine.backend().model().clone(),
            false,
        )
        .map_err(|_| Error::usage("find takes 2 to 255 units"))?;
        let stop = Stop::begin(options)?;
        let found = stop.run(|cancel| engine.find(&find, cancel).map_err(Error::from))?;
        Found::new(units, &found)
    }

    /// Each record with every value of the set, lazily, in input order. A
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
                |item, values| Some(AnnotatedRecord::new(item, values)),
            )
        }))
    }

    /// The facade engine that asks this question's model.
    fn asking(&self, question: &Question) -> Result<Arc<facade::Engine>, Error> {
        self.for_model(question.model.as_ref())
    }

    /// The facade engine that asks this model, or the engine's own.
    pub(crate) fn for_model(
        &self,
        model: Option<&core::ModelName>,
    ) -> Result<Arc<facade::Engine>, Error> {
        match model {
            Some(model) if model != self.inner.backend().model() => {
                Ok(Arc::new(self.inner.with_model(model.clone())?))
            }
            _ => Ok(Arc::clone(&self.inner)),
        }
    }

    fn judge(
        &self,
        question: &Question,
        text: &str,
        options: CallOptions<'_>,
    ) -> Result<facade::Judgment, Error> {
        let evidence = evidence(text)?;
        let engine = self.asking(question)?;
        let stop = Stop::begin(options)?;
        stop.run(|cancel| {
            engine
                .judge(&question.core, question.threshold, evidence, cancel)
                .map_err(Error::from)
        })
    }

    /// Hold a finite input whole, and refuse it over the request limit.
    fn within_limit<I: IntoIterator>(
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
            Ok(Completed {
                value: (
                    results::answer(&judged.value),
                    judged.answer.yes().unwrap_or_default(),
                ),
                replayed: judged.answered.replayed,
                partial_failure: false,
            })
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
) -> Result<Completed<Values>, Error> {
    let evidence = evidence(text)?;
    let model = engine.backend().model();
    let plan = |places: &[usize]| {
        let questions = places
            .iter()
            .filter_map(|place| set.questions().get(*place));
        Plan::new(
            evidence.clone(),
            model.clone(),
            questions.map(|named| named.question().clone()).collect(),
        )
        .map_err(|_| crate::engine::error::Error::Defect("an annotate group asks nothing"))
    };
    let annotation = engine.annotate(set, plan, cancel)?;
    Ok(Completed {
        replayed: annotation.replayed,
        partial_failure: annotation.failed_questions > 0,
        value: annotation.values,
    })
}

fn unbound() -> Error {
    Error::defect("the backend answered a label the choice does not hold")
}
