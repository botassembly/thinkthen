//! Additive scalar input doors on the existing scheduler and result carriers.

use crate::core::{self, Value};
use crate::engine::pipeline::{self, Flow};
use crate::public::asking::{Decisions, Text};
use crate::public::engine::{DECISIONS, Keyed, only};
use crate::public::options::Stop;
use crate::public::question::Kind;
use crate::public::{
    Answer, Call, CallOptions, Choice, ChooseQuestion, DecisionQuestion, DetailQuestion, Details,
    Engine, Error, InputFunction, Question, QuestionInput,
};
use crate::public::{pull, results};

impl Engine {
    /// Execute an explicit text or image yes/no input.
    /// # Errors
    /// Returns the existing six errors; image admission refuses before transport.
    pub fn decide_input<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        input: &QuestionInput,
    ) -> Result<Call<Answer>, Error> {
        self.decide_input_with(question, input, CallOptions::new())
    }
    /// Execute an explicit input with call controls.
    /// # Errors
    /// As [`Self::decide_input`].
    pub fn decide_input_with<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<Answer>, Error> {
        let question = question.question();
        only(question, DECISIONS, "decide")?;
        Ok(self
            .keyed_input(question, input, options)?
            .map(|(judged, _)| results::answer(&judged.value)))
    }
    /// Choose from a typed option set over explicit input.
    /// # Errors
    /// As [`Self::decide_input`].
    pub fn choose_input<C: Choice>(
        &self,
        question: &ChooseQuestion<C>,
        input: &QuestionInput,
    ) -> Result<Call<Option<C>>, Error> {
        self.choose_input_with(question, input, CallOptions::new())
    }
    /// Choose over explicit input with controls.
    /// # Errors
    /// As [`Self::decide_input`].
    pub fn choose_input_with<C: Choice>(
        &self,
        question: &ChooseQuestion<C>,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<Option<C>>, Error> {
        self.keyed_input(&question.0, input, options)?
            .try_map(|(judged, _)| match judged.value {
                Value::Choice(Some(label)) => C::from_label(&label).map(Some).ok_or_else(|| {
                    Error::defect("the backend answered a label the choice does not hold")
                }),
                Value::Choice(None) => Ok(None),
                _ => Err(Error::defect("a choose answer held no choice")),
            })
    }
    /// Score an explicit input on the question's rubric.
    /// # Errors
    /// As [`Self::decide_input`].
    pub fn score_input(
        &self,
        question: &Question,
        input: &QuestionInput,
    ) -> Result<Call<f64>, Error> {
        self.score_input_with(question, input, CallOptions::new())
    }
    /// Score an explicit input with controls.
    /// # Errors
    /// As [`Self::decide_input`].
    pub fn score_input_with(
        &self,
        question: &Question,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<f64>, Error> {
        only(question, &[Kind::Score], "score")?;
        self.keyed_input(question, input, options)?
            .try_map(|(judged, _)| match judged.value {
                Value::Score(value) => Ok(value),
                _ => Err(Error::defect("a score answer held no position")),
            })
    }
    /// Full typed probabilities and facts for explicit input.
    /// # Errors
    /// Unsupported image functions or routes return Usage before transport.
    pub fn details_input<Q: DetailQuestion + ?Sized>(
        &self,
        question: &Q,
        input: &QuestionInput,
    ) -> Result<Call<Details>, Error> {
        self.details_input_with(question, input, CallOptions::new())
    }
    /// Full details for explicit input with controls.
    /// # Errors
    /// As [`Self::details_input`].
    pub fn details_input_with<Q: DetailQuestion + ?Sized>(
        &self,
        question: &Q,
        input: &QuestionInput,
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
        self.keyed_input(question, input, options)?
            .try_map(|(judged, keys)| {
                Details::of(
                    &judged,
                    question,
                    self.inner.backend(),
                    self.profile.as_ref(),
                    keys,
                )
            })
    }
    /// Generic typed native dispatch. The seven text-only image cells refuse
    /// with their own function name before selecting a plan or transport.
    /// # Errors
    /// Returns Usage for unsupported function/input combinations.
    pub fn input_details<Q: DetailQuestion + ?Sized>(
        &self,
        function: InputFunction,
        question: &Q,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<Details>, Error> {
        super::guard(function, input)?;
        if matches!(input, QuestionInput::Images(_)) {
            let kinds = match function {
                InputFunction::Decide => DECISIONS,
                InputFunction::Choose => &[Kind::Choose],
                InputFunction::Score => &[Kind::Score],
                _ => &[],
            };
            only(question.question(), kinds, function.name())?;
        }
        self.details_input_with(question, input, options)
    }

    pub(crate) fn keyed_input(
        &self,
        question: &Question,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<Keyed>, Error> {
        options.without_context("a single-document call")?;
        question.metadata.validate_item(input)?;
        if let QuestionInput::Text(text) = input {
            crate::public::engine::evidence(text)?;
        }
        let engine = self.asking(question)?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let asker = Decisions::new(&engine, question, None);
        let mut packing = pull::packing(core::Setting::Max, false, false);
        packing.detailed = stop.facts().attempts().is_some();
        let input = Text {
            at: 0,
            input: input.clone(),
        };
        stop.run_call(1, |cancel| {
            let mut taken = None;
            let host = pipeline::eager(vec![input], |row| {
                taken = Some(row);
                Flow::Stop
            });
            engine
                .ask_all(&asker, packing, host, cancel)
                .map_err(Error::from)?;
            let row = taken.ok_or_else(|| Error::defect("a single call returned no row"))?;
            let keys = row
                .as_ref()
                .map_or_else(|_| Vec::new(), |decided| decided.keys.clone());
            crate::public::bulk::judged(&stop, question, engine.backend(), 0, row)
                .map(|judged| (judged, keys))
        })
    }
}
