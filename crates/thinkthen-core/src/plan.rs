//! The plan one request is built from.

use thiserror::Error;

use crate::question::Question;
use crate::text::{Evidence, ModelName};

/// A plan that asks nothing, which no backend can answer.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("a plan asks at least one question")]
pub struct EmptyPlanError;

/// What one request asks: the questions, over the evidence, of the model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    evidence: Evidence,
    model: ModelName,
    questions: Vec<Question>,
}

impl Plan {
    /// Ask the questions, in the order given, of one model over one evidence.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyPlanError`] when the questions are empty.
    pub fn new(
        evidence: Evidence,
        model: ModelName,
        questions: Vec<Question>,
    ) -> Result<Self, EmptyPlanError> {
        if questions.is_empty() {
            return Err(EmptyPlanError);
        }
        Ok(Self {
            evidence,
            model,
            questions,
        })
    }

    /// Read the evidence back.
    pub(crate) const fn evidence(&self) -> &Evidence {
        &self.evidence
    }

    /// Read the model name back.
    pub(crate) const fn model(&self) -> &ModelName {
        &self.model
    }

    /// Read the questions back, in the order they were given.
    pub(crate) fn questions(&self) -> &[Question] {
        &self.questions
    }
}

#[cfg(test)]
mod tests {
    use super::{EmptyPlanError, Plan};
    use crate::question::Question;
    use crate::text::{Evidence, ModelName, QuestionText};

    fn evidence() -> Evidence {
        Evidence::new("Help! My payouts have been failing for 3 days.").expect("not blank")
    }

    fn model() -> ModelName {
        ModelName::new("jev-latest").expect("not blank")
    }

    fn question(text: &str) -> Question {
        Question::Decide {
            text: QuestionText::new(text).expect("not blank"),
        }
    }

    #[test]
    fn a_plan_keeps_its_questions_in_the_order_they_were_given() {
        let questions = vec![question("is urgent"), question("asks for a refund")];
        let plan = Plan::new(evidence(), model(), questions.clone()).expect("a question is asked");
        assert_eq!(plan.evidence(), &evidence());
        assert_eq!(plan.model(), &model());
        assert_eq!(plan.questions(), questions.as_slice());
    }

    #[test]
    fn a_plan_that_asks_nothing_is_refused() {
        assert_eq!(
            Plan::new(evidence(), model(), Vec::new()),
            Err(EmptyPlanError)
        );
    }
}
