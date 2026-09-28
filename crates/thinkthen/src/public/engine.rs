//! The public engine: every call checks its question, fixes its controls,
//! and hands typed values to the one private facade.

use std::fmt;
use std::sync::Arc;

use crate::core::{self, BackendProfile, Value};
use crate::engine::facade::Roots;
use crate::engine::facade::{self, Settings};
use crate::public::choice::Choice;
use crate::public::error::Error;
use crate::public::options::{CallOptions, Stop, guarded};
use crate::public::question::{ChooseQuestion, Kind, Question, TagQuestion};
use crate::public::results::{
    self, Answer, Call, Counters, Details, ObservedQuestion, ObservedRow, QuestionDetail,
    RecordObservation,
};
use crate::public::settings::EngineBuilder;

/// One engine: its settings, its connection pool, its cache, and its counters.
///
/// Clones share everything, and one engine serves many threads at once.
/// Building sends nothing. `Debug` shows no setting.
#[derive(Clone)]
pub struct Engine {
    pub(crate) inner: Arc<facade::Engine>,
    pub(super) most: Option<usize>,
    pub(crate) profile: Option<BackendProfile>,
    pub(crate) batch: Option<core::Setting>,
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

pub(super) fn only(question: &Question, kinds: &[Kind], call: &str) -> Result<(), Error> {
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

pub(super) const DECISIONS: &[Kind] = &[Kind::Decide, Kind::Banded];

impl Engine {
    /// Build from what the command reads: see [`EngineBuilder::from_env`].
    ///
    /// # Errors
    ///
    /// As [`EngineBuilder::from_env`] and [`EngineBuilder::build`].
    pub fn from_env() -> Result<Self, Error> {
        EngineBuilder::from_env()?.build()
    }

    /// Start from the library defaults. The builder reads no environment; with
    /// the default cache still selected, `build` resolves the platform cache
    /// folder from `HOME` and `XDG_CACHE_HOME`.
    #[must_use]
    pub fn builder() -> EngineBuilder {
        EngineBuilder::new()
    }

    pub(crate) fn from_settings(
        settings: Settings,
        most: Option<usize>,
        profile: Option<BackendProfile>,
        roots: Option<Roots>,
        batch: Option<core::Setting>,
    ) -> Result<Self, Error> {
        let inner = guarded(|| facade::Engine::with_roots(settings, roots).map_err(Error::from))?;
        Ok(Self {
            inner: Arc::new(inner),
            most,
            profile,
            batch,
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
    ) -> Result<Call<Answer>, Error> {
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
    ) -> Result<Call<Answer>, Error> {
        let question = question.question();
        only(question, DECISIONS, "decide")?;
        Ok(self
            .judge(question, evidence, options)?
            .map(|judged| results::answer(&judged.value)))
    }

    /// Pick one option of `C`, or `None` when the answer is not sure.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`].
    pub fn choose<C: Choice>(
        &self,
        question: &ChooseQuestion<C>,
        evidence: &str,
    ) -> Result<Call<Option<C>>, Error> {
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
    ) -> Result<Call<Option<C>>, Error> {
        self.judge(&question.0, evidence, options)?
            .try_map(|judged| match judged.value {
                Value::Choice(Some(label)) => C::from_label(&label).map(Some).ok_or_else(unbound),
                Value::Choice(None) => Ok(None),
                _ => Err(Error::defect("a choose answer held no choice")),
            })
    }

    /// Place the evidence on the question's levels: 0 at the lowest, 1 at the highest.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`], and [`Error::Usage`] for another kind of question.
    pub fn score(&self, question: &Question, evidence: &str) -> Result<Call<f64>, Error> {
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
    ) -> Result<Call<f64>, Error> {
        only(question, &[Kind::Score], "score")?;
        self.judge(question, evidence, options)?
            .try_map(|judged| match judged.value {
                Value::Score(position) => Ok(position),
                _ => Err(Error::defect("a score answer held no position")),
            })
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
    ) -> Result<Call<Vec<C>>, Error> {
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
    ) -> Result<Call<Vec<C>>, Error> {
        self.judge(&question.0, evidence, options)?
            .try_map(|judged| match judged.value {
                Value::Tag(labels) => labels
                    .iter()
                    .map(|label| C::from_label(label).ok_or_else(unbound))
                    .collect(),
                _ => Err(Error::defect("a tag answer held no labels")),
            })
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
    ) -> Result<Call<Details>, Error> {
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
    ) -> Result<Call<Details>, Error> {
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
        self.judge(question, evidence, options)?.try_map(|judged| {
            Details::of(
                &judged,
                question,
                self.inner.backend(),
                self.profile.as_ref(),
            )
        })
    }

    /// The facade engine that asks this question's model.
    pub(super) fn asking(&self, question: &Question) -> Result<Arc<facade::Engine>, Error> {
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
    ) -> Result<Call<facade::Judgment>, Error> {
        let evidence = evidence(text)?;
        let engine = self.asking(question)?;
        let stop = Stop::begin(options)?;
        stop.run_call(1, |cancel| {
            let judged = engine
                .judge(&question.core, question.threshold, evidence, cancel)
                .map_err(Error::from)?;
            if stop.observing() {
                let details = Details::of(&judged, question, engine.backend(), engine.profile())?;
                let observed = ObservedQuestion::from_details(&details);
                stop.observe(RecordObservation::Question {
                    index: 0,
                    member: None,
                    stage: None,
                    position: 0,
                    detail: QuestionDetail::of(&observed),
                });
                stop.observe(RecordObservation::Row {
                    index: 0,
                    value: ObservedRow::Judgment(details.value()),
                });
            }
            Ok(judged)
        })
    }
}

fn unbound() -> Error {
    Error::defect("the backend answered a label the choice does not hold")
}
