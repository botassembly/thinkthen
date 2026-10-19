use super::conformance_support::ExpectedAnswer;
use super::same_json;
use crate::core::{AnswerOutcome, Threshold, Value};

pub(super) fn check(
    case_id: &str,
    name: &str,
    outcome: &AnswerOutcome,
    held: &ExpectedAnswer,
    threshold: Option<Threshold>,
) -> Result<Option<(Value, f64)>, String> {
    match outcome {
        AnswerOutcome::Answered(answer) => {
            let expected = held
                .details
                .answer
                .as_ref()
                .ok_or_else(|| format!("{name} expects no successful answer"))?;
            if held.details.failure.is_some() {
                return Err(format!("{name} expects both an answer and a failure"));
            }
            let (value, _) = answer.read(threshold);
            if !same_json(
                &serde_json::to_string(&value).map_err(|error| error.to_string())?,
                held.bare.get(),
            )? || !same_json(
                &serde_json::to_string(answer).map_err(|error| error.to_string())?,
                expected.get(),
            )? {
                return Err(format!("{case_id} answer `{name}` has wrong details"));
            }
            Ok(Some((value, answer.yes().unwrap_or_default())))
        }
        AnswerOutcome::Failed(failure) => {
            let expected = held
                .details
                .failure
                .as_ref()
                .ok_or_else(|| format!("{name} expects no failure"))?;
            let rendered = serde_json::to_string(failure).map_err(|error| error.to_string())?;
            if held.details.answer.is_some()
                || !same_json(&rendered, expected.get())?
                || !same_json(&format!(r#"{{"failed":{rendered}}}"#), held.bare.get())?
            {
                return Err(format!("{case_id} answer `{name}` has wrong failure"));
            }
            Ok(None)
        }
    }
}
