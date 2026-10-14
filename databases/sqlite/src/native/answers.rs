//! Ordinary answer columns project the same retained native receipt.
use crate::Failure;
use crate::receipt::{Cells, Receipt};
use rusqlite::types::Value;
use serde_json::Value as Json;

pub(super) fn rows(verb: &str, receipt: &Receipt, rows: &mut Vec<Cells>) -> Result<(), Failure> {
    let native = receipt.carrier.get("native");
    let values = native
        .and_then(|value| value.get("value"))
        .or_else(|| receipt.carrier.get("completed"));
    let values = values.map_or(Vec::new(), |value| {
        value
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![value.clone()])
    });
    if verb == "recognize" {
        recognition_rows(rows, &values);
        return Ok(());
    }
    if verb == "relate" {
        relation_rows(rows, &values, receipt);
        return Ok(());
    }
    for (row, value) in rows.iter_mut().zip(values.iter()) {
        if matches!(verb, "rank" | "rank_set") {
            rank_columns(row, value, receipt, verb);
            continue;
        }
        if verb == "filter" {
            filter_columns(row, value, receipt);
            continue;
        }
        let answer = value.get("value").unwrap_or(&Json::Null);
        let value = match answer {
            Json::Bool(value) => Value::Integer(i64::from(*value)),
            Json::String(value) => Value::Text(value.clone()),
            Json::Number(value) => {
                if verb == "score" {
                    value.as_f64().map_or(Value::Null, Value::Real)
                } else {
                    value.as_i64().map_or(Value::Null, Value::Integer)
                }
            }
            Json::Null => Value::Null,
            Json::Array(_) | Json::Object(_) => Value::Text(answer.to_string()),
        };
        row.insert(0, value.clone());
        if matches!(verb, "decide" | "choose") {
            row.insert(1, value_probability(value, &receipt.carrier));
        }
        if matches!(verb, "details" | "details_images") {
            detail_columns(row, &receipt.carrier);
        }
    }
    Ok(())
}

fn value_probability(_: Value, carrier: &Json) -> Value {
    carrier
        .pointer("/native/value/0/answer/probability")
        .and_then(Json::as_f64)
        .map_or(Value::Null, Value::Real)
}

pub(super) fn detail_root(row: &Json) -> &str {
    match row.pointer("/question/verb").and_then(Json::as_str) {
        Some("choose") => "choice",
        Some("tag") => "tags",
        Some("score") => "score",
        _ => "decision",
    }
}

fn detail_columns(row: &mut Cells, carrier: &Json) {
    let detail = carrier
        .get("observations")
        .and_then(Json::as_array)
        .into_iter()
        .flatten()
        .find_map(|event| event.get("detail"));
    let Some(detail) = detail else {
        return;
    };
    let columns = [
        detail
            .get("confidence")
            .and_then(Json::as_f64)
            .map_or(Value::Null, Value::Real),
        detail
            .get("answer_id")
            .and_then(Json::as_str)
            .map_or(Value::Null, |value| Value::Text(value.to_owned())),
        detail
            .get("model")
            .and_then(Json::as_str)
            .map_or(Value::Null, |value| Value::Text(value.to_owned())),
        detail
            .get("requests_sent")
            .and_then(Json::as_i64)
            .map_or(Value::Null, Value::Integer),
    ];
    for (at, value) in columns.into_iter().enumerate() {
        row.insert(at + 1, value);
    }
}

fn rank_columns(row: &mut Cells, value: &Json, receipt: &Receipt, verb: &str) {
    let index = value.get("index").and_then(Json::as_u64).unwrap_or(0) as usize;
    let key = receipt
        .keys
        .get(index)
        .cloned()
        .unwrap_or_else(|| index.to_string());
    let columns = [
        Value::Text(key),
        value
            .get("value")
            .and_then(Json::as_i64)
            .map_or(Value::Null, Value::Integer),
        value
            .pointer("/answer/probability")
            .and_then(Json::as_f64)
            .map_or(Value::Null, Value::Real),
        value
            .get("question_name")
            .and_then(Json::as_str)
            .map_or(Value::Null, |name| Value::Text(name.to_owned())),
    ];
    for (at, value) in columns
        .into_iter()
        .take(if verb == "rank_set" { 4 } else { 3 })
        .enumerate()
    {
        row.insert(at, value);
    }
}

fn filter_columns(row: &mut Cells, value: &Json, receipt: &Receipt) {
    let source = value.get("source").unwrap_or(value);
    let facts = receipt
        .carrier
        .pointer("/native/facts")
        .unwrap_or(&Json::Null);
    let values = [
        source.get("file"),
        source.get("first_line"),
        source.get("last_line"),
        value.get("index"),
        value.get("value"),
        value.pointer("/answer/probability"),
        value.get("answer_id"),
        facts.get("call_id"),
        facts.get("requests_sent"),
        facts.get("cache_answers"),
        facts.get("records"),
        facts.get("input_tokens"),
        facts.get("output_tokens"),
        facts.get("seconds"),
    ];
    for (at, value) in values.into_iter().enumerate() {
        row.insert(at, cell(value));
    }
}
fn cell(value: Option<&Json>) -> Value {
    match value {
        Some(Json::Bool(value)) => Value::Integer(i64::from(*value)),
        Some(Json::String(value)) => Value::Text(value.clone()),
        Some(Json::Number(value)) => value
            .as_i64()
            .map(Value::Integer)
            .or_else(|| value.as_f64().map(Value::Real))
            .unwrap_or(Value::Null),
        _ => Value::Null,
    }
}
fn recognition_rows(rows: &mut Vec<Cells>, values: &[Json]) {
    let mut expanded = Vec::new();
    for (row, value) in rows.iter().zip(values) {
        for entity in value
            .pointer("/value/entities")
            .and_then(Json::as_array)
            .into_iter()
            .flatten()
        {
            let mut row = row.clone();
            let fields = [
                entity.get("text"),
                entity.get("start"),
                entity.get("end"),
                None,
                entity.get("kind"),
                entity.get("strength"),
            ];
            for (at, value) in fields.into_iter().enumerate() {
                row.insert(at, cell(value));
            }
            let length = entity.get("end").and_then(Json::as_i64).unwrap_or(0)
                - entity.get("start").and_then(Json::as_i64).unwrap_or(0);
            row.insert(3, Value::Integer(length));
            expanded.push(row);
        }
    }
    *rows = expanded;
}
fn relation_rows(rows: &mut Vec<Cells>, values: &[Json], receipt: &Receipt) {
    let mut expanded = Vec::new();
    for (row, value) in rows.iter().zip(values) {
        for edge in value
            .get("value")
            .and_then(Json::as_array)
            .into_iter()
            .flatten()
        {
            expand_edge(&mut expanded, row, edge, receipt);
        }
    }
    *rows = expanded;
}
fn expand_edge(rows: &mut Vec<Cells>, parent: &Cells, edge: &Json, receipt: &Receipt) {
    let source = endpoint(edge.get("source"), receipt);
    let target = endpoint(edge.get("target"), receipt);
    for left in &source {
        for right in &target {
            let mut row = parent.clone();
            let values = [
                cell(edge.get("relation")),
                left.clone(),
                right.clone(),
                cell(edge.get("probability")),
                cell(edge.get("either")),
            ];
            for (at, value) in values.into_iter().enumerate() {
                row.insert(at, value);
            }
            rows.push(row);
        }
    }
}
fn endpoint(value: Option<&Json>, receipt: &Receipt) -> Vec<Value> {
    let name = value
        .and_then(|value| value.get("name"))
        .and_then(Json::as_str)
        .unwrap_or_default();
    let kind = value
        .and_then(|value| value.get("kind"))
        .and_then(Json::as_str)
        .unwrap_or_default();
    receipt
        .ids
        .get(&(name.to_owned(), kind.to_owned()))
        .cloned()
        .unwrap_or_else(|| vec![Value::Text(name.to_owned())])
}
