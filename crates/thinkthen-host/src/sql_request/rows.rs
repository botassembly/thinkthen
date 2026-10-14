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
        let ordinal = if function == F::Find {
            result.get("index").cloned().unwrap_or(Value::Null)
        } else {
            json!(ordinal)
        };
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
            let (mut value, probability) = project_value(function, &result);
            if function == F::Decide {
                value = observations
                    .iter()
                    .find(|event| {
                        event.get("kind").and_then(Value::as_str) == Some("row")
                            && event.get("index") == result.get("index")
                    })
                    .and_then(|event| event.get("value"))
                    .cloned()
                    .unwrap_or(value);
            }
            rows.push(
                json!({"ordinal":ordinal,"value":value,"probability":probability,
                "result":result,"failure":null,"facts":facts,"observations":observations}),
            );
        }
    }
    if function == F::Rank {
        rows.sort_by_key(|row| row.get("ordinal").and_then(Value::as_u64));
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

/// Restore decisions and aligned probabilities from authoritative native results.
pub fn typed_values(result: &thinkthen::RequestValue, rows: &mut [Value]) -> Result<(), Error> {
    if let thinkthen::RequestValue::Choices(choices) = result {
        for (row, choice) in rows.iter_mut().zip(choices) {
            crate::complete_native::put(row, "probability", json!(choice.result().probability()))?;
        }
    }
    if let thinkthen::RequestValue::Recognized(recognized) = result {
        for (row, recognized) in rows.iter_mut().zip(recognized) {
            crate::complete_native::put(
                row,
                "probability",
                json!(recognized.result().probability()),
            )?;
        }
    }
    if let thinkthen::RequestValue::Found(found) = result {
        for row in rows.iter_mut() {
            crate::complete_native::put(row, "probability", json!(found.probability()))?;
        }
    }
    if let thinkthen::RequestValue::Decisions(decisions) = result {
        for (row, decision) in rows.iter_mut().zip(decisions) {
            let value = match decision.result().value() {
                thinkthen::Answer::Yes => Value::Bool(true),
                thinkthen::Answer::No => Value::Bool(false),
                thinkthen::Answer::Unsure => Value::Null,
            };
            crate::complete_native::put(row, "value", value)?;
        }
    }
    Ok(())
}

/// Project a canonical SQL record into its ordinary value and probability.
pub fn project_value(function: F, row: &Value) -> (Value, Value) {
    let answer = row.get("value").cloned().unwrap_or(Value::Null);
    let probability = match function {
        F::Decide | F::Filter | F::Rank => row
            .pointer("/answer/probability")
            .cloned()
            .unwrap_or(Value::Null),
        F::Find if row.get("index").is_none_or(Value::is_null) => Value::Null,
        F::Choose if answer.is_null() => Value::Null,
        F::Choose | F::Find => row
            .pointer("/answer/pick")
            .and_then(Value::as_str)
            .and_then(|pick| {
                row.pointer("/answer/probabilities")
                    .and_then(|values| values.get(pick))
            })
            .cloned()
            .unwrap_or(Value::Null),
        F::Recognize => recognition_probability(row),
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
        F::Recognize => answer
            .get("entities")
            .or_else(|| answer.get("proposals"))
            .cloned()
            .unwrap_or(Value::Null),
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
            let facts = serde_json::to_value(call.facts().complete()).map_err(|_| defect())?;
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
    typed_values(&result, &mut rows)?;
    if let Some(failure) = failure {
        if matches!(function, F::Rank | F::Find | F::Relate) {
            rows.clear();
        }
        rows.push(failed(rows.len(), failure, &facts, &observations));
    }
    Ok(rows)
}

fn recognition_probability(row: &Value) -> Value {
    if let Some(proposals) = row.pointer("/value/proposals").and_then(Value::as_array) {
        return Value::Array(
            proposals
                .iter()
                .map(|proposal| proposal.get("probability").cloned().unwrap_or(Value::Null))
                .collect(),
        );
    }
    let entities = row.pointer("/value/entities").and_then(Value::as_array);
    let proposals = row.pointer("/answer/proposals").and_then(Value::as_array);
    Value::Array(
        entities
            .into_iter()
            .flatten()
            .map(|entity| {
                proposals
                    .into_iter()
                    .flatten()
                    .find(|proposal| {
                        proposal.get("kept") == Some(&Value::Bool(true))
                            && proposal.pointer("/selected/start") == entity.get("start")
                            && proposal.pointer("/selected/end") == entity.get("end")
                            && proposal.get("kind") == entity.get("kind")
                    })
                    .and_then(|proposal| proposal.get("span_probability"))
                    .cloned()
                    .unwrap_or(Value::Null)
            })
            .collect(),
    )
}

/// Project an already rendered ordinary host carrier into SQL details rows.
pub fn project_carrier(function: F, carrier: &Value) -> Vec<Value> {
    let facts = carrier
        .pointer("/native/facts")
        .cloned()
        .unwrap_or(Value::Null);
    let observations = carrier
        .get("observations")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let result = carrier
        .pointer("/native/value")
        .or_else(|| carrier.get("completed"))
        .cloned()
        .unwrap_or(json!([]));
    let mut rows = complete(function, 0, result, &facts, &observations);
    if let Some(failure) = carrier.pointer("/native/error") {
        if matches!(function, F::Rank | F::Find | F::Relate) {
            rows.clear();
        }
        rows.push(failed(rows.len(), failure.clone(), &facts, &observations));
    }
    rows
}
