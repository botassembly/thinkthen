//! The driver's replies, the case file's members, and the wire digests.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use super::{Checked, Members, Reply};

/// Parse the driver's `CODE LENGTH` framed replies.
pub(crate) fn replies(output: &[u8]) -> Checked<Vec<Reply>> {
    let mut rest = output;
    let mut all = Vec::new();
    while !rest.is_empty() {
        let line = rest
            .iter()
            .position(|byte| *byte == b'\n')
            .ok_or("a reply has no header")?;
        let header = String::from_utf8_lossy(&rest[..line]).into_owned();
        let (code, length) = header
            .split_once(' ')
            .ok_or("a reply header is malformed")?;
        let code: i32 = code.parse().map_err(|_| "a reply code is not a number")?;
        let length: usize = length
            .parse()
            .map_err(|_| "a reply length is not a number")?;
        let body = rest
            .get(line + 1..line + 1 + length)
            .ok_or("a reply is short")?;
        all.push((code, String::from_utf8_lossy(body).into_owned()));
        rest = rest.get(line + 2 + length..).unwrap_or_default();
    }
    Ok(all)
}

pub(super) fn parsed((code, body): &Reply) -> Checked<Value> {
    if *code != 0 {
        return Err(format!("code {code}: {body}"));
    }
    let value: Value = serde_json::from_str(body).map_err(|error| format!("{error}: {body}"))?;
    if value.get("facts").is_some() {
        return value
            .get("value")
            .cloned()
            .ok_or("a call wrapper has no value".to_owned());
    }
    Ok(value)
}

/// A judgment reply's `OUTCOME PROBABILITY` pairs.
pub(super) fn judged(reply: &Reply) -> Checked<Vec<(i32, f64)>> {
    let (code, body) = reply;
    if *code != 0 {
        return Err(format!("code {code}: {body}"));
    }
    let words: Vec<&str> = body.split(' ').collect();
    words
        .chunks(2)
        .map(|pair| match pair {
            [outcome, probability] => Ok((
                outcome.parse().map_err(|_| "an outcome is not a number")?,
                probability
                    .parse()
                    .map_err(|_| "a probability is not a number")?,
            )),
            _ => Err("a judgment is half written".to_owned()),
        })
        .collect()
}

/// A question object's written bytes with one more member at the end.
pub(crate) fn with(object: &str, key: &str, value: &str) -> String {
    let open = object.trim_end().strip_suffix('}').unwrap_or(object);
    format!("{open},\"{key}\":{value}}}")
}

/// A `decide` question file asked under another verb's key.
pub(crate) fn renamed_verb(question: &str, verb: &str) -> Checked<String> {
    question
        .trim_start()
        .strip_prefix('{')
        .and_then(|rest| rest.trim_start().strip_prefix(r#""decide""#))
        .map(|rest| format!(r#"{{"{verb}"{rest}"#))
        .ok_or_else(|| format!("the question does not lead with decide: {question}"))
}

/// One member of a case; a missing member reads as null.
pub(crate) fn member(case: &Members, key: &str) -> Value {
    case.get(key)
        .and_then(|raw| serde_json::from_str(raw.get()).ok())
        .unwrap_or_default()
}

pub(crate) fn string(case: &Members, key: &str) -> String {
    member(case, key).as_str().unwrap_or_default().to_owned()
}

pub(super) fn digest(url: &str, request: &[u8]) -> String {
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
    let parts: Members = serde_json::from_slice(body).map_err(|error| error.to_string())?;
    let part = |name: &str| {
        parts
            .get(name)
            .map(|raw| raw.get())
            .ok_or(format!("no {name}"))
    };
    let (model, state) = (part("model")?, part("state")?);
    let questions: Members =
        serde_json::from_str(part("questions")?).map_err(|error| error.to_string())?;
    let mut placed = Vec::new();
    for (name, question) in &questions {
        let place: usize = name[1..]
            .parse()
            .map_err(|_| format!("no qN name: {name}"))?;
        placed.push((place, question.get()));
    }
    placed.sort_by_key(|(place, _)| *place);
    Ok(placed
        .into_iter()
        .map(|(_, question)| {
            let joined = ["systemone", url, model, state, question].join("\n");
            Sha256::digest(joined.as_bytes())
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect()
        })
        .collect())
}

pub(super) fn swap(value: &Value, renamed: &BTreeMap<String, Value>) -> Value {
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
pub(super) fn same(what: &str, actual: &Value, expected: &Value) -> Checked {
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
