//! Canonical choice results retain the complete label distribution.
use super::Choose;
use crate::core::{self, Ask, Detail, Error, Function, FunctionAnswers, Outcome};
impl Function for Choose {
    fn try_asks_record(
        &self,
        record: &core::AdmittedRecord,
        limits: core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        self.inner.try_asks_record(record, limits)
    }
    fn read_record(context: FunctionAnswers<'_, Self>, limits: core::Limits) -> Outcome {
        complete_choice(context.for_function(&context.question().inner), limits)
    }
}

pub(crate) fn complete_choice(
    context: FunctionAnswers<'_, core::Choose>,
    limits: core::Limits,
) -> Outcome {
    let question = context.question();
    let mut outcome = core::Choose::read_record(context.for_function(question), limits);
    if let Some(Ok(answer)) = context.answers().first() {
        let labels = question
            .options
            .iter()
            .zip(answer.probabilities())
            .map(|(option, p)| (option.label().to_owned(), *p))
            .collect();
        match &mut outcome {
            Outcome::Answered { detail, .. } | Outcome::Unsure { detail, .. } => {
                *detail = Some(Detail::Labels(labels))
            }
            Outcome::Failed { .. } => {}
        }
    }
    outcome
}
