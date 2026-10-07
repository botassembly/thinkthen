//! Digest rewriting and JSON comparison for shared cases.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use serde_json::{Value, json};

use thinkthen::{Details, Probabilities};

use crate::cases::Checked;

/// Read one optional absolute ID list, and refuse duplicate or unknown IDs.
pub(crate) fn selected_ids(cases: &[Value]) -> Checked<Option<BTreeSet<String>>> {
    let mut available = BTreeSet::new();
    for case in cases {
        let id = case["id"].as_str().ok_or("a shared case has no ID")?;
        if !available.insert(id) {
            return Err(format!("duplicate shared case `{id}`"));
        }
    }
    let Some(path) = std::env::var_os("THINKTHEN_CONFORMANCE_IDS") else {
        return Ok(None);
    };
    let path = PathBuf::from(path);
    if !path.is_absolute() {
        return Err("THINKTHEN_CONFORMANCE_IDS takes an absolute path".to_owned());
    }
    let text = fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut selected = BTreeSet::new();
    for id in text
        .lines()
        .map(str::trim)
        .filter(|id| !id.is_empty() && !id.starts_with('#'))
    {
        if !selected.insert(id.to_owned()) {
            return Err(format!("duplicate selected case `{id}`"));
        }
    }
    if selected.is_empty() {
        return Err("the selected case list is empty".to_owned());
    }
    for id in &selected {
        if !available.contains(id.as_str()) {
            return Err(format!(
                "selected case `{id}` is absent from the shared corpus"
            ));
        }
    }
    Ok(Some(selected))
}

#[path = "keys.rs"]
mod question_keys;
pub(crate) use question_keys::fixture_keys;

pub(crate) fn swap(value: &Value, renamed: &BTreeMap<String, Value>) -> Value {
    match value {
        Value::String(held) => renamed.get(held).cloned().unwrap_or_else(|| json!(held)),
        Value::Array(items) => items.iter().map(|item| swap(item, renamed)).collect(),
        Value::Object(fields) => fields
            .iter()
            .map(|(name, field)| (name.clone(), swap(field, renamed)))
            .collect(),
        other => other.clone(),
    }
}

/// Compare two values, holding numbers to a rounding tolerance.
pub(crate) fn same(what: &str, actual: &Value, expected: &Value) -> Checked {
    fn close(one: &Value, other: &Value) -> bool {
        match (one, other) {
            (Value::Number(one), Value::Number(other)) => {
                (one.as_f64().unwrap_or(f64::NAN) - other.as_f64().unwrap_or(f64::NAN)).abs() < 1e-9
            }
            (Value::Array(one), Value::Array(other)) => {
                one.len() == other.len()
                    && one.iter().zip(other).all(|(one, other)| close(one, other))
            }
            (Value::Object(one), Value::Object(other)) => {
                one.len() == other.len()
                    && one
                        .iter()
                        .all(|(name, value)| other.get(name).is_some_and(|held| close(value, held)))
            }
            (one, other) => one == other,
        }
    }
    if close(actual, expected) {
        Ok(())
    } else {
        Err(format!("{what}: got {actual}, expected {expected}"))
    }
}

pub(crate) fn detailed(details: &Details, expected: &Value, base: &str) -> Checked {
    let wanted = &expected["details"];
    let answer = &wanted["answer"];
    let probabilities = match details.probabilities() {
        Probabilities::YesNo { yes } => ("probability", json!(yes)),
        Probabilities::Named(named) => (
            "probabilities",
            named
                .iter()
                .map(|one| (one.name().to_owned(), json!(one.probability())))
                .collect(),
        ),
    };
    same(probabilities.0, &probabilities.1, &answer[probabilities.0])?;
    if let Some(level) = answer.get("level") {
        same("level", &json!(details.nearest()), level)?;
    }
    same("model", &json!(details.model()), &wanted["model"])?;
    same(
        "question_sha256",
        &json!(details.question_sha256()),
        &wanted["question_sha256"],
    )?;
    keyed_requests(details.requests(), &wanted["requests"])?;
    same(
        "confidence",
        &json!(details.confidence()),
        &answer["confidence"],
    )?;
    let usage = details.usage().map(|usage| {
        json!({"input_tokens": usage.input_tokens(), "output_tokens": usage.output_tokens()})
    });
    same("usage", &json!(usage), &wanted["usage"])?;
    same(
        "requests_sent",
        &json!(details.requests_sent()),
        &wanted["requests_sent"],
    )?;
    same("cached", &json!(details.cached()), &wanted["cached"])?;
    let served = json!(format!("{base}/systemone"));
    same("url", &json!(details.url()), &served)?;
    let line: Value =
        serde_json::from_str(&details.to_json()).map_err(|error| error.to_string())?;
    let url = line.get("meta").and_then(|meta| meta.get("url"));
    same("line url", url.unwrap_or(&Value::Null), &served)
}

/// A row lists its own question keys, in order, out of the keys of the
/// requests the case recorded. A row of digests matches them exactly.
fn keyed_requests(printed: &[String], wanted: &Value) -> Checked {
    let recorded = wanted.as_array().map_or(&[][..], Vec::as_slice);
    if !recorded.iter().any(Value::is_array) {
        return same("requests", &json!(printed), wanted);
    }
    let mut rest = recorded
        .iter()
        .flat_map(|keys| keys.as_array().into_iter().flatten());
    if !printed.is_empty() && printed.iter().all(|key| rest.any(|held| held == key)) {
        Ok(())
    } else {
        Err(format!(
            "requests: printed {printed:?}, expected keys out of {wanted}"
        ))
    }
}
