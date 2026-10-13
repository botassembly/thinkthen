//! Explicit input batches reuse the same pull bridge, queue and asker as text.

use crate::public::engine::{DECISIONS, only};
use crate::public::{
    Answer, Batch, CallOptions, DecisionQuestion, Engine, Error, InputEvidence, Row,
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
}
