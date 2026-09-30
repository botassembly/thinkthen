//! Digest rewriting and JSON comparison for shared cases.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use serde_json::value::RawValue;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

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

pub(crate) fn digest(url: &str, request: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"systemone\n");
    hasher.update(url.as_bytes());
    hasher.update(b"\n");
    hasher.update(request);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Every question key of one request body, in wire order, by ADR 0111
/// section 2: the SHA-256 of the adapter, the URL, the model, the state and
/// one question as the body carries them, joined by line feeds.
pub(crate) fn keys(url: &str, body: &[u8]) -> Checked<Vec<String>> {
    #[derive(serde::Deserialize)]
    struct Parts<'a> {
        #[serde(borrow)]
        state: &'a RawValue,
        #[serde(borrow)]
        model: &'a RawValue,
        #[serde(borrow)]
        questions: BTreeMap<String, &'a RawValue>,
    }
    let parts: Parts<'_> = serde_json::from_slice(body).map_err(|error| error.to_string())?;
    let mut questions = Vec::new();
    for (name, question) in parts.questions {
        let place: usize = name[1..].parse().map_err(|_| format!("no qN name: {name}"))?;
        questions.push((place, question));
    }
    questions.sort_by_key(|(place, _)| *place);
    Ok(questions
        .into_iter()
        .map(|(_, question)| {
            let joined = [
                "systemone",
                url,
                parts.model.get(),
                parts.state.get(),
                question.get(),
            ]
            .join("\n");
            Sha256::digest(joined.as_bytes())
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect()
        })
        .collect())
}

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
