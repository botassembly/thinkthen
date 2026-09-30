//! Results cross to Ruby as JSON, read by Ruby's own json library, and the
//! one-worker completion receipt.

use super::*;
use crate::call::Output;
use crate::result::Completed;
use serde::Serialize;
use serde_json::value::RawValue;
use thinkthen::Facts;

/// Write a Rust value as JSON and read it with `JSON.parse`, symbol keys.
pub(super) fn ruby_json(ruby: &Ruby, value: &impl Serialize) -> Result<Value, Error> {
    let text = serde_json::to_string(value).map_err(|_| {
        Error::new(
            ruby.exception_runtime_error(),
            "defect: a result could not be written as JSON",
        )
    })?;
    let json: RModule = ruby.class_object().const_get("JSON")?;
    let options = ruby.hash_new();
    options.aset(ruby.to_symbol("symbolize_names"), true)?;
    json.funcall("parse", (text, options))
}

/// Each question event's JSON text as one array.
fn raw(details: &[String]) -> Vec<&RawValue> {
    details
        .iter()
        .filter_map(|detail| serde_json::from_str(detail).ok())
        .collect()
}

pub(super) fn facts_value(ruby: &Ruby, facts: &Facts) -> Result<Value, Error> {
    ruby_json(ruby, facts)
}

pub(super) fn details_value(ruby: &Ruby, details: &[String]) -> Result<Value, Error> {
    ruby_json(ruby, &raw(details))
}

pub(super) fn output(ruby: &Ruby, answer: &Output) -> Value {
    match answer {
        Output::Answer(answer) => ruby.into_value(*answer),
        Output::Score(position) => ruby.into_value(*position),
        Output::Rows(rows) => ruby.into_value(rows.clone()),
        Output::Places(places) => ruby.into_value(places.clone()),
        Output::Ranked(ranked) => ruby.into_value(ranked.clone()),
        Output::Found(found) => ruby.into_value(*found),
        Output::Json(json) => ruby.into_value(json.clone()),
        Output::JsonRows(rows) => ruby.into_value(rows.clone()),
    }
}

pub(super) fn attach_completion(ruby: &Ruby, error: &Error, handoff: &Arc<Handoff>) {
    if let Some(value) = error.value() {
        let receipt = CompletionValue(Arc::clone(handoff));
        let _: Result<Value, Error> = value.funcall(
            "instance_variable_set",
            ("@thinkthen_completion", ruby.into_value(receipt)),
        );
    }
}

/// A completion's final account as Ruby reads it.
#[derive(Serialize)]
pub(super) struct Receipt<'a> {
    outcome: &'static str,
    facts: Option<&'a Facts>,
    details: Option<Vec<&'a RawValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retryable: Option<bool>,
}

/// The receipt of a worker that closed with no answer.
pub(super) const PANICKED: Receipt<'static> = Receipt {
    outcome: "panicked",
    facts: None,
    details: None,
    kind: None,
    message: None,
    retryable: None,
};

pub(super) fn protected_completion(
    ruby: &Ruby,
    terminal: &Result<Completed, Fault>,
) -> Result<Value, Error> {
    let receipt = match terminal {
        Ok(done) => Receipt {
            outcome: "succeeded",
            facts: Some(&done.facts),
            details: Some(raw(&done.details)),
            kind: None,
            message: None,
            retryable: None,
        },
        Err(fault) => Receipt {
            outcome: if fault.message == "defect: the Ruby binding panicked" {
                "panicked"
            } else {
                "failed"
            },
            facts: fault.facts.as_deref(),
            details: fault.facts.as_ref().map(|_| raw(&fault.details)),
            kind: Some(fault.kind.name()),
            message: Some(&fault.message),
            retryable: Some(fault.retryable),
        },
    };
    ruby_json(ruby, &receipt)
}
