//! Explicit input batches reuse the same pull bridge, queue and asker as text.

use crate::core::Value;
use crate::public::engine::{DECISIONS, only};
use crate::public::question::Kind;
use crate::public::{
    Answer, Batch, CallOptions, Choice, ChooseQuestion, DecisionQuestion, DetailQuestion, Details,
    Engine, Error, InputEvidence, Question, Row,
};

impl Engine {
    /// Ordered yes/no answers over explicit input items.
    pub fn decide_input_many<'a, I, Q: DecisionQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        inputs: I,
    ) -> Batch<'a, Row<I::Item, Answer>>
    where
        I: IntoIterator + 'a,
        I::Item: InputEvidence,
    {
        self.decide_input_many_with(question, inputs, CallOptions::new())
    }
    /// Ordered explicit input answers with controls.
    pub fn decide_input_many_with<'a, I, Q: DecisionQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        inputs: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<I::Item, Answer>>
    where
        I: IntoIterator + 'a,
        I::Item: InputEvidence,
    {
        self.try_decide_input_many_with(question, inputs.into_iter().map(Ok), options)
    }
    /// Fallible reader input with earlier completed rows retained.
    pub fn try_decide_input_many_with<'a, I, R: InputEvidence + 'a, Q: DecisionQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        inputs: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<R, Answer>>
    where
        I: IntoIterator<Item = Result<R, Error>> + 'a,
    {
        let question = question.question();
        Batch::of(only(question, DECISIONS, "decide").and_then(|()| {
            self.try_decisions(question, inputs, options, |input, (value, probability)| {
                Ok(Some(Row::new(
                    input,
                    crate::public::results::answer(&value),
                    probability,
                )))
            })
        }))
    }
    /// Typed choices over ordered explicit inputs.
    pub fn choose_input_many<'a, I, C: Choice>(
        &'a self,
        question: &'a ChooseQuestion<C>,
        inputs: I,
    ) -> Batch<'a, Row<I::Item, Option<C>>>
    where
        I: IntoIterator + 'a,
        I::Item: InputEvidence,
    {
        self.choose_input_many_with(question, inputs, CallOptions::new())
    }
    /// Typed choices over explicit inputs with controls.
    pub fn choose_input_many_with<'a, I, C: Choice>(
        &'a self,
        question: &'a ChooseQuestion<C>,
        inputs: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<I::Item, Option<C>>>
    where
        I: IntoIterator + 'a,
        I::Item: InputEvidence,
    {
        self.try_choose_input_many_with(question, inputs.into_iter().map(Ok), options)
    }
    /// Fallible explicit choice inputs.
    pub fn try_choose_input_many_with<'a, I, R: InputEvidence + 'a, C: Choice>(
        &'a self,
        question: &'a ChooseQuestion<C>,
        inputs: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<R, Option<C>>>
    where
        I: IntoIterator<Item = Result<R, Error>> + 'a,
    {
        Batch::of(self.try_decisions(
            &question.0,
            inputs,
            options,
            |input, (value, probability)| {
                let choice = match value {
                    Value::Choice(Some(label)) => Some(C::from_label(&label).ok_or_else(|| {
                        Error::defect("the backend answered a label the choice does not hold")
                    })?),
                    Value::Choice(None) => None,
                    _ => return Err(Error::defect("a choose answer held no choice")),
                };
                Ok(Some(Row::new(input, choice, probability)))
            },
        ))
    }
    /// Rubric scores over ordered explicit inputs.
    pub fn score_input_many<'a, I>(
        &'a self,
        question: &'a Question,
        inputs: I,
    ) -> Batch<'a, Row<I::Item, f64>>
    where
        I: IntoIterator + 'a,
        I::Item: InputEvidence,
    {
        self.score_input_many_with(question, inputs, CallOptions::new())
    }
    /// Rubric scores over explicit inputs with controls.
    pub fn score_input_many_with<'a, I>(
        &'a self,
        question: &'a Question,
        inputs: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<I::Item, f64>>
    where
        I: IntoIterator + 'a,
        I::Item: InputEvidence,
    {
        self.try_score_input_many_with(question, inputs.into_iter().map(Ok), options)
    }
    /// Fallible explicit score inputs.
    pub fn try_score_input_many_with<'a, I, R: InputEvidence + 'a>(
        &'a self,
        question: &'a Question,
        inputs: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<R, f64>>
    where
        I: IntoIterator<Item = Result<R, Error>> + 'a,
    {
        Batch::of(only(question, &[Kind::Score], "score").and_then(|()| {
            self.try_decisions(
                question,
                inputs,
                options,
                |input, (value, probability)| match value {
                    Value::Score(value) => Ok(Some(Row::new(input, value, probability))),
                    _ => Err(Error::defect("a score answer held no position")),
                },
            )
        }))
    }
    /// Complete probabilities and facts over explicit input items.
    pub fn details_input_many<'a, I, Q: DetailQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        inputs: I,
    ) -> Batch<'a, Row<I::Item, Details>>
    where
        I: IntoIterator + 'a,
        I::Item: InputEvidence + serde::Serialize,
    {
        self.details_input_many_with(question, inputs, CallOptions::new())
    }
    /// Complete details over explicit input items with controls.
    pub fn details_input_many_with<'a, I, Q: DetailQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        inputs: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<I::Item, Details>>
    where
        I: IntoIterator + 'a,
        I::Item: InputEvidence + serde::Serialize,
    {
        self.try_details_input_many_with(question, inputs.into_iter().map(Ok), options)
    }
}
