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
                let edge_result = edge_result(&result, edge);
                rows.push(
                    json!({"ordinal":rows.len(),"value":edge,"probability":edge.get("probability"),
                    "result":edge_result,"failure":null,"facts":facts,"observations":[]}),
                );
            }
        } else {
            let (value, probability) = project_value(function, &result);
            let value = observed_decision(function, &result, observations).unwrap_or(value);
            rows.push(
                json!({"ordinal":ordinal,"value":value,"probability":probability,
                "result":result,"failure":null,"facts":facts,"observations":[]}),
            );
        }
    }
    if function == F::Rank {
        rows.sort_by_key(|row| row.get("ordinal").and_then(Value::as_u64));
    }
    rows
}

fn edge_result(result: &Value, edge: &Value) -> Value {
    let mut result = result.clone();
    if let Some(value) = result.get_mut("value") {
        *value = json!([edge]);
    }
    result
}

fn observed_decision(function: F, result: &Value, observations: &[Value]) -> Option<Value> {
    if function != F::Decide {
        return None;
    }
    observations
        .iter()
        .find(|event| {
            event.get("kind").and_then(Value::as_str) == Some("row")
                && event.get("index") == result.get("index")
        })
        .and_then(|event| event.get("value"))
        .cloned()
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

fn failed_call(failure: Value, facts: &Value, observations: &[Value]) -> Value {
    let ordinal = failure
        .pointer("/stopped/at")
        .cloned()
        .unwrap_or(Value::Null);
    let mut row = failed(0, failure, facts, observations);
    if let Some(value) = row.get_mut("ordinal") {
        *value = ordinal;
    }
    row
}

/// Assign each native event once without comparing or deduplicating its contents.
pub(super) fn observe_rows(function: F, rows: &mut [Value], observations: &[Value]) {
    if rows.is_empty() {
        return;
    }
    let first = rows
        .iter()
        .position(|row| row.get("ordinal") == Some(&json!(0)))
        .unwrap_or(0);
    let terminal = rows
        .iter()
        .rposition(|row| row.get("failure").is_some_and(|failure| !failure.is_null()))
        .unwrap_or(first);
    let owners = observations
        .iter()
        .map(|event| {
            if matches!(function, F::Rank | F::Find | F::Relate) {
                first
            } else if let Some(index) = event.get("index") {
                rows.iter()
                    .position(|row| row.get("ordinal") == Some(index))
                    .unwrap_or(terminal)
            } else {
                first
            }
        })
        .collect::<Vec<_>>();
    for (index, row) in rows.iter_mut().enumerate() {
        let events = Value::Array(
            observations
                .iter()
                .zip(&owners)
                .filter(|(_, owner)| **owner == index)
                .map(|(event, _)| event.clone())
                .collect(),
        );
        if let Some(observations) = row.get_mut("observations") {
            *observations = events;
        }
    }
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
            crate::complete_native::put(
                row,
                "probability",
                json!(decision.result().probability()),
            )?;
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
        rows.push(failed_call(failure, &facts, &observations));
    }
    observe_rows(function, &mut rows, &observations);
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
    let mut rows = carrier
        .get("rows")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_else(|| complete(function, 0, result, &facts, &observations));
    if carrier.get("rows").is_none()
        && let Some(failure) = carrier.pointer("/native/error")
    {
        if matches!(function, F::Rank | F::Find | F::Relate) {
            rows.clear();
        }
        rows.push(failed_call(failure.clone(), &facts, &observations));
    }
    observe_rows(function, &mut rows, &observations);
    rows
}
