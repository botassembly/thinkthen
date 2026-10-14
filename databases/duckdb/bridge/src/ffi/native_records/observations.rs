//! Canonical session observation values retain native detail and actual ancillary inputs.
use crate::complete_native;
use serde_json::Value;
use thinkthen::{Error, RecordObservation, RequestFunction, RequestSessionResult};

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

pub(super) fn event(
    event: &RecordObservation<'_>,
    function: RequestFunction,
) -> Result<Value, Error> {
    let packet = RequestSessionResult::Observation {
        function,
        value: event.to_owned(),
    };
    let mut native: Value =
        serde_json::from_str(&packet.to_json()?).map_err(|_| complete_native::defect())?;
    let mut value = native
        .get_mut("value")
        .ok_or_else(complete_native::defect)?
        .take();
    supplement(&mut value, complete_native::observations::event(event)?)?;
    Ok(value)
}
