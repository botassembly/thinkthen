//! The plan one request is built from.

use thiserror::Error;

use crate::core::question::Question;
use crate::core::text::{Evidence, ModelName};

/// A plan that asks nothing, which no backend can answer.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("a plan asks at least one question")]
pub(crate) struct EmptyPlanError;

/// How a backend's descriptions travel (ADR 0115 section 3).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum Descriptions {
    /// Each description exactly as authored.
    #[default]
    Authored,
    /// Authored descriptions with an empty object for a missing yes-or-no side.
    BothSides,
    /// Each description as text: an object's `what`, and no empty or null
    /// description. A temporary Ollama-only workaround; the adapter's backend
    /// table names its debt issue.
    Text,
}

/// What one request asks: the questions, over the evidence, of the model, with
/// the descriptions in the form the model's backend reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Plan {
    evidence: Evidence,
    image_group: bool,
    image: Option<Vec<crate::core::image::ImageInput>>,
    model: ModelName,
    descriptions: Descriptions,
    questions: Vec<Question>,
}

impl Plan {
    /// Ask the questions, in the order given, of one model over one evidence.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyPlanError`] when the questions are empty.
    pub(crate) fn new(
        evidence: Evidence,
        model: ModelName,
        descriptions: Descriptions,
        questions: Vec<Question>,
    ) -> Result<Self, EmptyPlanError> {
        if questions.is_empty() {
            return Err(EmptyPlanError);
        }
        Ok(Self {
            evidence,
            image: None,
            image_group: false,
            model,
            descriptions,
            questions,
        })
    }

    /// A plan whose descriptions travel as authored, as every test plan does.
    #[cfg(test)]
    pub(crate) fn authored(
        evidence: Evidence,
        model: ModelName,
        questions: Vec<Question>,
    ) -> Result<Self, EmptyPlanError> {
        Self::new(evidence, model, Descriptions::Authored, questions)
    }

    pub(crate) fn with_image(mut self, image: Option<Vec<crate::core::image::ImageInput>>) -> Self {
        self.image = image;
        self
    }

    pub(crate) fn image(&self) -> Option<&[crate::core::image::ImageInput]> {
        self.image.as_deref()
    }

    pub(crate) fn with_image_group(
        mut self,
        images: Option<Vec<crate::core::image::ImageInput>>,
    ) -> Self {
        self.image = images;
        self.image_group = self.image.is_some();
        self
    }

    pub(crate) const fn image_group(&self) -> bool {
        self.image_group
    }

    /// Read the evidence back.
    pub(crate) const fn evidence(&self) -> &Evidence {
        &self.evidence
    }

    /// Read the model name back.
    pub(crate) const fn model(&self) -> &ModelName {
        &self.model
    }

    /// How this plan's descriptions travel.
    pub(crate) const fn descriptions(&self) -> Descriptions {
        self.descriptions
    }

    /// Read the questions back, in the order they were given.
    pub(crate) fn questions(&self) -> &[Question] {
        &self.questions
    }
}

#[cfg(test)]
mod tests {
    use super::{EmptyPlanError, Plan};
    use crate::core::question::Question;
    use crate::core::text::{Evidence, ModelName, QuestionText};

    fn evidence() -> Evidence {
        Evidence::new("Help! My payouts have been failing for 3 days.").expect("not blank")
    }

    fn model() -> ModelName {
        ModelName::new("jev-latest").expect("not blank")
    }

    fn question(text: &str) -> Question {
        Question::Decide {
            text: QuestionText::new(text).expect("not blank"),
            yes: None,
            no: None,
        }
    }

    #[test]
    fn a_plan_keeps_its_questions_in_the_order_they_were_given() {
        let questions = vec![question("is urgent"), question("asks for a refund")];
        let plan =
            Plan::authored(evidence(), model(), questions.clone()).expect("a question is asked");
        assert_eq!(plan.evidence(), &evidence());
        assert_eq!(plan.model(), &model());
        assert_eq!(plan.questions(), questions.as_slice());
    }

    #[test]
    fn a_plan_that_asks_nothing_is_refused() {
        assert_eq!(
            Plan::authored(evidence(), model(), Vec::new()),
            Err(EmptyPlanError)
        );
    }
}
