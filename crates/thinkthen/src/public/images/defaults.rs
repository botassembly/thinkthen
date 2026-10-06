//! Default-engine counterparts of the explicit input doors.

use crate::public::{
    Answer, Call, CallOptions, Choice, ChooseQuestion, DecisionQuestion, DetailQuestion, Details,
    Error, Question, QuestionInput, default_engine,
};

/// Explicit input on the default engine.
/// # Errors
/// As the default engine and its decide input method.
pub fn decide_input<Q: DecisionQuestion + ?Sized>(
    question: &Q,
    input: &QuestionInput,
) -> Result<Call<Answer>, Error> {
    default_engine()?.decide_input(question, input)
}
/// Explicit input and controls on the default engine.
/// # Errors
/// As the default engine and its decide input method.
pub fn decide_input_with<Q: DecisionQuestion + ?Sized>(
    question: &Q,
    input: &QuestionInput,
    options: CallOptions<'_>,
) -> Result<Call<Answer>, Error> {
    default_engine()?.decide_input_with(question, input, options)
}
/// Typed choice over explicit input on the default engine.
/// # Errors
/// As the default engine and its choose input method.
pub fn choose_input<C: Choice>(
    question: &ChooseQuestion<C>,
    input: &QuestionInput,
) -> Result<Call<Option<C>>, Error> {
    default_engine()?.choose_input(question, input)
}
/// Typed choice over explicit input with controls on the default engine.
/// # Errors
/// As the default engine and its choose input method.
pub fn choose_input_with<C: Choice>(
    question: &ChooseQuestion<C>,
    input: &QuestionInput,
    options: CallOptions<'_>,
) -> Result<Call<Option<C>>, Error> {
    default_engine()?.choose_input_with(question, input, options)
}
/// Rubric score over explicit input on the default engine.
/// # Errors
/// As the default engine and its score input method.
pub fn score_input(question: &Question, input: &QuestionInput) -> Result<Call<f64>, Error> {
    default_engine()?.score_input(question, input)
}
/// Rubric score over explicit input with controls on the default engine.
/// # Errors
/// As the default engine and its score input method.
pub fn score_input_with(
    question: &Question,
    input: &QuestionInput,
    options: CallOptions<'_>,
) -> Result<Call<f64>, Error> {
    default_engine()?.score_input_with(question, input, options)
}
/// Full typed details over explicit input on the default engine.
/// # Errors
/// As the default engine and its details input method.
pub fn details_input<Q: DetailQuestion + ?Sized>(
    question: &Q,
    input: &QuestionInput,
) -> Result<Call<Details>, Error> {
    default_engine()?.details_input(question, input)
}
/// Full typed details over explicit input with controls on the default engine.
/// # Errors
/// As the default engine and its details input method.
pub fn details_input_with<Q: DetailQuestion + ?Sized>(
    question: &Q,
    input: &QuestionInput,
    options: CallOptions<'_>,
) -> Result<Call<Details>, Error> {
    default_engine()?.details_input_with(question, input, options)
}
