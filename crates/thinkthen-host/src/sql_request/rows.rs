//! SQL output rows preserve canonical records and native aggregate membership.
use crate::complete_native::defect;
use serde_json::{Value, json};
use thinkthen::{Error, RequestFunction as F, RequestOutcome};

pub(super) fn complete(
    function: F,
    ordinal: usize,
    encoded: Value,
    facts: &Value,
    observations: &[Value],
) -> Vec<Value> {
    let records = encoded.as_array().cloned().unwrap_or_else(|| vec![encoded]);
    let mut rows = Vec::new();
    for result in records {
        let ordinal = result
            .get("index")
            .and_then(Value::as_u64)
            .unwrap_or(ordinal as u64);
        if function == F::Relate {
            for edge in result
                .get("value")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                rows.push(
                    json!({"ordinal":rows.len(),"value":edge,"probability":edge.get("probability"),
                    "result":result,"failure":null,"facts":facts,"observations":observations}),
                );
            }
        } else {
            let (value, probability) = project_value(function, &result);
            rows.push(
                json!({"ordinal":ordinal,"value":value,"probability":probability,
                "result":result,"failure":null,"facts":facts,"observations":observations}),
            );
        }
    }
    rows
}

pub(super) fn failed(
    ordinal: usize,
    failure: Value,
    facts: &Value,
    observations: &[Value],
) -> Value {
    json!({"ordinal":ordinal,"value":null,"probability":null,"result":null,
        "failure":failure,"facts":facts,"observations":observations})
}

/// Project a canonical SQL record into its ordinary value and probability.
pub fn project_value(function: F, row: &Value) -> (Value, Value) {
    let answer = row.get("value").cloned().unwrap_or(Value::Null);
    let probability = match function {
        F::Decide | F::Filter | F::Rank => row
            .pointer("/answer/probability")
            .cloned()
            .unwrap_or(Value::Null),
        F::Choose | F::Find => row
            .pointer("/answer/pick")
            .and_then(Value::as_str)
            .and_then(|pick| {
                row.pointer("/answer/probabilities")
                    .and_then(|values| values.get(pick))
            })
            .cloned()
            .unwrap_or(Value::Null),
        F::Recognize => answer
            .get("entities")
            .and_then(Value::as_array)
            .map(|entities| {
                Value::Array(
                    entities
                        .iter()
                        .map(|entity| entity.get("strength").cloned().unwrap_or(Value::Null))
                        .collect(),
                )
            })
            .unwrap_or(Value::Null),
        _ => Value::Null,
    };
    let value = match function {
        F::Filter => {
            if answer == Value::Bool(true) {
                row.get("input").cloned().unwrap_or(Value::Null)
            } else {
                Value::Null
            }
        }
        F::Recognize => answer.get("entities").cloned().unwrap_or(Value::Null),
        _ => answer,
    };
    (value, probability)
}

/// Project an ordinary SQL call without changing its aggregate input semantics.
pub fn project_outcome(
    function: F,
    outcome: RequestOutcome,
    observations: Vec<Value>,
) -> Result<Vec<Value>, Error> {
    let (result, facts, failure) = match outcome {
        RequestOutcome::Complete(call) => {
            let facts = serde_json::to_value(call.facts()).map_err(|_| defect())?;
            (call.into_value(), facts, None)
        }
        RequestOutcome::Failed { completed, error } => {
            if matches!(
                error.kind(),
                thinkthen::ErrorKind::Cancelled
                    | thinkthen::ErrorKind::Deadline
                    | thinkthen::ErrorKind::Defect
            ) {
                return Err(error);
            }
            let failed = crate::complete_native::failure(&error);
            (
                completed,
                failed.get("facts").cloned().unwrap_or(Value::Null),
                failed.get("error").cloned(),
            )
        }
    };
    let mut rows = complete(
        function,
        0,
        serde_json::to_value(&result).map_err(|_| defect())?,
        &facts,
        &observations,
    );
    if let Some(failure) = failure {
        rows.push(failed(rows.len(), failure, &facts, &observations));
    }
    Ok(rows)
}
