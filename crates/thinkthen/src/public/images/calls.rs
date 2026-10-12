//! Additive scalar input interfaces on the existing scheduler and result carriers.

use crate::public::engine::only;
use crate::public::question::Kind;
use crate::public::{
    Answer, Call, CallOptions, Choice, ChooseQuestion, DecisionQuestion, DetailQuestion, Details,
    Engine, Error, Question, QuestionInput,
};

impl Engine {
    /// Execute an explicit input with call controls.
    /// # Errors
    /// Refuses unsupported images before transport.
    pub(crate) fn decide_input_with<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<Answer>, Error> {
        Ok(self
            .decide_input_complete_with(question, input, options)?
            .map(|result| result.value()))
    }
    /// Choose over explicit input with controls.
    /// # Errors
    /// Refuses unsupported images before transport.
    pub(crate) fn choose_input_with<C: Choice>(
        &self,
        question: &ChooseQuestion<C>,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<Option<C>>, Error> {
        self.choose_input_complete_with(question, input, options)?
            .try_map(|result| {
                result
                    .value()
                    .map(|label| {
                        C::from_label(label).ok_or(Error::defect(
                            "the backend answered a label the choice does not hold",
                        ))
                    })
                    .transpose()
            })
    }
    /// Score an explicit input with controls.
    /// # Errors
    /// Refuses unsupported images before transport.
    pub(crate) fn score_input_with(
        &self,
        question: &Question,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<f64>, Error> {
        Ok(self
            .score_input_complete_with(question, input, options)?
            .map(|result| result.value()))
    }
    /// Full details for explicit input with controls.
    /// # Errors
    /// Refuses unsupported images before transport.
    pub(crate) fn details_input_with<Q: DetailQuestion + ?Sized>(
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
        let function =
            crate::RequestFunction::for_definition(&question.clone().into()).input_function();
        self.atomic_complete(function, question, input, options)?
            .try_map(|canonical| Details::of_complete::<()>(&canonical, None))
    }
}
