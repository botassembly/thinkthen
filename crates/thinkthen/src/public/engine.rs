//! The public engine: every call checks its question, fixes its controls,
//! and hands typed values to the one private facade.

use std::fmt;
use std::sync::Arc;

use crate::core::{self, BackendProfile, Prices};
use crate::engine::facade::Roots;
use crate::engine::facade::{self, Settings};
use crate::public::choice::Choice;
use crate::public::error::Error;
use crate::public::options::{CallOptions, guarded};
use crate::public::question::{ChooseQuestion, Kind, LoadedQuestion, Question, TagQuestion};
use crate::public::results::Counters;
use crate::public::settings::EngineBuilder;

/// One judgment and the question keys behind it.
pub(super) type Keyed = (facade::Judgment, Vec<String>);

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
    pub(crate) prices: Option<Prices>,
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

/// A question `decide` accepts.
#[expect(
    private_bounds,
    reason = "the sealed bound keeps the question kinds closed"
)]
pub trait DecisionQuestion: Sealed {}

/// A primitive question the planning methods accept.
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
impl Sealed for LoadedQuestion {
    fn question(&self) -> &Question {
        match self {
            LoadedQuestion::Question(question) => question,
            LoadedQuestion::Banded(banded) => &banded.0,
        }
    }
}
impl DecisionQuestion for Question {}
impl DecisionQuestion for crate::public::question::BandedQuestion {}
impl DecisionQuestion for LoadedQuestion {}
impl DetailQuestion for Question {}
impl DetailQuestion for crate::public::question::BandedQuestion {}
impl DetailQuestion for LoadedQuestion {}
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
    core::Evidence::new(text).map_err(evidence_error)
}

pub(crate) fn evidence_error(_: core::BlankTextError) -> Error {
    Error::usage("evidence is text, not white space")
}

impl Engine {
    /// The actual selected adapter's canonical provider label, without inference from its URL.
    #[must_use]
    pub fn provider_name(&self) -> &'static str {
        self.inner.backend().api_type().provider_name()
    }
    /// Whether the resolved posting endpoint names the default built-in service.
    ///
    /// This uses the native backend's exact endpoint comparison after URL resolution.
    /// It reads nothing and sends nothing; alternate paths and custom services return false.
    #[must_use]
    pub fn is_built_in(&self) -> bool {
        self.inner.backend().is_built_in()
    }
    /// Whether the actual adapter treats this engine's selected model as a mutable alias.
    #[must_use]
    pub fn model_is_mutable_alias(&self) -> bool {
        self.inner
            .backend()
            .api_type()
            .is_mutable_alias(self.inner.backend().model())
    }
    /// The actual built engine's selected local profile name, including configured defaults.
    /// This reads no file or environment variable and exposes no profile content.
    #[must_use]
    pub fn profile_name(&self) -> Option<&str> {
        self.inner.profile().map(|profile| profile.name().as_str())
    }

    /// Check a zero-based record ordinal against this engine's call admission cap.
    /// Composed source calls use the same `max_requests` limit as native batches.
    ///
    /// # Errors
    /// Returns [`Error::Usage`] for the first excess record or any later ordinal.
    pub fn check_record_limit(&self, at: usize) -> Result<(), Error> {
        if let Some(most) = self.most.filter(|most| at >= *most) {
            return Err(Error::usage(format!(
                "this engine answers at most {most} records in one call"
            )));
        }
        Ok(())
    }

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
        totals: (Option<u64>, Option<u64>),
        source: (Option<BackendProfile>, Option<Roots>),
        batch: Option<core::Setting>,
        prices: Option<Prices>,
    ) -> Result<Self, Error> {
        let (profile, roots) = source;
        let inner = guarded(|| facade::Engine::with_roots(settings, roots).map_err(Error::from))?
            .with_process_budget(totals.0, totals.1);
        Ok(Self {
            inner: Arc::new(inner),
            most,
            profile,
            batch,
            prices,
        })
    }

    /// Estimate the cost of complete reported token totals under this engine's
    /// caller-selected prices. The caller establishes usage completeness.
    #[must_use]
    pub fn estimate_reported_cost(&self, input_tokens: u64, output_tokens: u64) -> Option<String> {
        self.prices
            .and_then(|prices| prices.estimate(input_tokens, output_tokens))
    }

    /// Observe count persistence without waiting for a write or touching inherited
    /// state in a forked child. Written covers only this engine's current deltas.
    #[must_use]
    pub fn usage_persistence(&self) -> crate::public::UsagePersistence {
        self.inner.usage_persistence()
    }

    /// Drain current deltas and report persistence. Only usage-lock acquisition
    /// has the existing one-second deadline; other filesystem work can take longer.
    #[must_use]
    pub fn finish_usage_status(&self) -> crate::public::UsagePersistence {
        self.inner.finish_usage_status()
    }

    /// Monotonic union of this engine's HTTP and response-body read intervals.
    /// Overlapping attempts count once. Missing timing or invalid clock arithmetic
    /// returns None; an engine without sends has zero elapsed HTTP time.
    #[must_use]
    pub fn http_time(&self) -> Option<std::time::Duration> {
        guarded(|| self.inner.http_time().map_err(Error::from))
            .ok()
            .flatten()
    }

    /// The first retained local usage failure, with no path or OS error text.
    /// None also covers failures without a retained file classification. Observe
    /// `usage_persistence` to distinguish those failures from working persistence.
    #[must_use]
    pub fn usage_failure(&self) -> Option<crate::UsageFailure> {
        guarded(|| self.inner.usage_failure().map_err(Error::from))
            .ok()
            .flatten()
            .map(crate::UsageFailure::of)
    }

    /// This engine's totals: the sends, cache answers and tokens of calls
    /// made through it and its clones. Another engine in the same process
    /// counts its own, from zero; add several with `Counters`' `Sum`. A
    /// forked child starts from zero. The durable usage totals of ADR 0113
    /// give the view across engines and processes.
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
    ) -> Result<crate::DecisionCall, Error> {
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
    ) -> Result<crate::DecisionCall, Error> {
        self.decide_input_complete_with(
            question,
            &super::QuestionInput::Text(evidence.to_owned()),
            options,
        )?
        .project(|result| {
            let probability = result.probability();
            let value = match result.value() {
                crate::Answer::Yes => Some(true),
                crate::Answer::No => Some(false),
                crate::Answer::Unsure => None,
            };
            Ok((value, vec![result], probability))
        })
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
    ) -> Result<crate::ChoiceCall<C>, Error> {
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
    ) -> Result<crate::ChoiceCall<C>, Error> {
        self.atomic_complete(
            super::InputFunction::Choose,
            question.question(),
            &super::QuestionInput::Text(evidence.to_owned()),
            options,
        )?
        .try_map(super::complete::choice)?
        .project(|result| {
            let probability = result.probability();
            let value = result
                .value()
                .map(|label| {
                    C::from_label(label).ok_or(Error::defect(
                        "the backend answered a label the choice does not hold",
                    ))
                })
                .transpose()?;
            Ok((value, vec![result], probability))
        })
    }

    /// Place the evidence on the question's levels: 0 at the lowest, 1 at the highest.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`], and [`Error::Usage`] for another kind of question.
    pub fn score(&self, question: &Question, evidence: &str) -> Result<crate::ScoreCall, Error> {
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
    ) -> Result<crate::ScoreCall, Error> {
        self.atomic_complete(
            super::InputFunction::Score,
            question,
            &super::QuestionInput::Text(evidence.to_owned()),
            options,
        )?
        .try_map(super::complete::score)?
        .project(|result| Ok((result.value(), vec![result], None::<f64>)))
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
    ) -> Result<crate::TagCall<C>, Error> {
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
    ) -> Result<crate::TagCall<C>, Error> {
        self.atomic_complete(
            super::InputFunction::Tag,
            question.question(),
            &super::QuestionInput::Text(evidence.to_owned()),
            options,
        )?
        .try_map(super::complete::tags)?
        .project(|result| {
            let value = result
                .value()
                .iter()
                .map(|label| {
                    C::from_label(label).ok_or(Error::defect(
                        "the backend answered a label the choice does not hold",
                    ))
                })
                .collect::<Result<Vec<_>, Error>>()?;
            Ok((value, vec![result], None::<f64>))
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
}
