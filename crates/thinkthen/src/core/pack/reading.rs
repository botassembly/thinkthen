//! Assemble logical answers from the selected adapter's persistent members.
use super::{Stored, wire_count};
use crate::core::adapters::{ApiType, built_in::DecodeError};
use crate::core::{Answer, AnswerOutcome, Question};
pub(super) fn read(
    api: ApiType,
    questions: &[Question],
    answers: &[Stored<'_>],
) -> Result<Vec<AnswerOutcome>, DecodeError> {
    let mut rest = answers;
    let mut outcomes = Vec::new();
    for question in questions {
        let (own, after) = rest
            .split_at_checked(wire_count(question))
            .ok_or(DecodeError::MissingAnswer(answers.len()))?;
        rest = after;
        if let Some(cause) = own.iter().find_map(|answer| answer.err()) {
            outcomes.push(AnswerOutcome::Failed(
                crate::core::reply::BackendFailure::new(cause),
            ));
            continue;
        }
        let answer = match question {
            Question::Tag { text, labels } => {
                let decoder = Question::Decide {
                    text: text.clone(),
                    yes: None,
                    no: None,
                };
                let mut probabilities = Vec::new();
                for (label, member) in labels.names().zip(own) {
                    let decoded = api.stored(
                        &decoder,
                        member.as_ref().map_err(|_| DecodeError::UnexpectedAnswer)?,
                    )?;
                    probabilities.push((
                        label.clone(),
                        crate::core::probability::Probability::new(
                            decoded.yes().ok_or(DecodeError::WrongKind(0))?,
                        )
                        .map_err(|_| DecodeError::ProbabilityOutOfRange(0))?,
                    ));
                }
                Answer::new_tag(probabilities)
            }
            _ => api.stored(
                question,
                own.first()
                    .and_then(|v| v.as_ref().ok())
                    .ok_or(DecodeError::MissingAnswer(0))?,
            )?,
        };
        outcomes.push(AnswerOutcome::Answered(answer));
    }
    if !rest.is_empty() {
        return Err(DecodeError::UnexpectedAnswer);
    }
    Ok(outcomes)
}
