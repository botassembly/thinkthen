//! Canonical session observation values retain native detail and actual ancillary inputs.
use crate::complete_native;
use serde_json::Value;
use thinkthen::Error;

pub(crate) fn supplement(value: &mut Value, actual: Value) -> Result<(), Error> {
    if actual.get("kind").and_then(Value::as_str) == Some("question") {
        for field in [
            "answer_id",
            "failure_id",
            "question_sources",
            "observations",
            "inputs",
            "raw_pick",
            "author",
        ] {
            if let Some(member) = actual.get(field) {
                complete_native::put(value, field, member.clone())?;
            }
        }
    }
    Ok(())
}
