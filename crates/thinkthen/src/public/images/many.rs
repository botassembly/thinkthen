//! Explicit input batches reuse the same pull bridge, queue and asker as text.

use crate::core::Value;
use crate::public::engine::{DECISIONS, only};
use crate::public::question::Kind;
use crate::public::{
    Answer, Batch, CallOptions, Choice, ChooseQuestion, DecisionQuestion, Engine, Error,
    InputEvidence, Question, Row,
};

impl Engine {
    /// Fallible reader input with earlier completed rows retained.
    pub(crate) fn try_decide_input_many_with<
        'a,
        I,
        R: InputEvidence + 'a,
        Q: DecisionQuestion + ?Sized,
    >(
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
            self.try_decisions(
                crate::InputFunction::Decide,
                question,
                inputs,
                options,
                |input, (value, probability)| {
                    Ok(Some(Row::new(
                        input,
                        crate::public::results::answer(&value),
                        probability,
                    )))
                },
            )
        }))
    }
    /// Fallible explicit choice inputs.
    pub(crate) fn try_choose_input_many_with<'a, I, R: InputEvidence + 'a, C: Choice>(
        &'a self,
        question: &'a ChooseQuestion<C>,
        inputs: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<R, Option<C>>>
    where
        I: IntoIterator<Item = Result<R, Error>> + 'a,
    {
        Batch::of(self.try_decisions(
            crate::InputFunction::Choose,
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
    /// Fallible explicit score inputs.
    pub(crate) fn try_score_input_many_with<'a, I, R: InputEvidence + 'a>(
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
                crate::InputFunction::Score,
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
}
