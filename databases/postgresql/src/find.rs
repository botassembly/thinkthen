//! One ordered PostgreSQL array enters one public find call.

use pgrx::datum::{Array, JsonB};
use pgrx::prelude::*;
use thinkthen::{Error, For, Question};

use crate::call::{self, OrRaise as _};
use crate::ffi::RawJson;
use crate::forms::{self, Named};

type Units = Option<(Question, Vec<String>)>;

fn indexed(array: Array<'_, &str>, question: &str, none: bool) -> Result<Units, Error> {
    // Keep host NULL refusal ahead of question construction without copying text.
    if array.iter().any(|member| member.is_none()) {
        return Err(call::usage("a find unit is text, not NULL"));
    }
    if array.is_empty() {
        return Ok(None);
    }
    let question = Question::find(question)?;
    let question = if none {
        question.offering_none()?
    } else {
        question
    };
    question.admit_find_units(array.iter().flatten().map(Ok))?;
    Ok(Some((
        question,
        array.iter().flatten().map(str::to_owned).collect(),
    )))
}

fn found(
    question: Option<&str>,
    units: Option<Array<'_, &str>>,
    raw: Option<RawJson>,
) -> Option<JsonB> {
    let (Some(question), Some(units)) = (question, units) else {
        return None;
    };
    let (settings, mut call) = forms::controls(raw.as_ref(), Named::default());
    settings
        .check(For::Find)
        .map_err(|error| call::usage(error.to_string()))
        .or_raise();
    let none = settings.none().unwrap_or(false);
    if let Some(raw) = &raw {
        // The shared parser has already checked the closed grammar and type.
        // Find has no question-file model setter, so select its engine here.
        let value: serde_json::Value = serde_json::from_str(&raw.0)
            .unwrap_or_else(|_| call::raise(call::usage("settings is one JSON object")));
        if let Some(model) = value.get("model").and_then(serde_json::Value::as_str) {
            call = call.with_model(model);
        }
    }
    let (question, units) = indexed(units, question, none).or_raise()?;
    let answer = call::run(call, move |engine, options| {
        let call = crate::request::run(
            engine,
            thinkthen::RequestFunction::Find,
            question.into(),
            units.into_iter().map(crate::request::text).collect(),
            options,
        )?;
        match call.into_value() {
            thinkthen::RequestValue::Found(found) => Ok(found),
            _ => Err(crate::request::wrong_result()),
        }
    });
    let selected = match answer.selection() {
        thinkthen::FindSelection::Unit(at) => Some(at),
        thinkthen::FindSelection::None => None,
    };
    let candidates = answer.candidates();
    let winner = match selected {
        Some(at) => candidates.get(at),
        None => candidates.last().filter(|candidate| candidate.is_none()),
    }
    .ok_or_else(|| call::defect("a find answer selected no candidate"))
    .or_raise();
    let probabilities: Vec<_> = candidates
        .iter()
        .enumerate()
        .map(|(at, candidate)| {
            serde_json::json!({
                "index": candidate.input().map(|_| at),
                "probability": candidate.probability(),
            })
        })
        .collect();
    Some(JsonB(serde_json::json!({
        "index": selected,
        "value": answer.selected().map(|unit| match unit {
            thinkthen::QuestionInput::Text(text) => text.as_str(),
            thinkthen::QuestionInput::Record(record) => record.original().literal()
                .unwrap_or_else(|| call::raise(crate::request::wrong_result())),
            _ => call::raise(crate::request::wrong_result()),
        }),
        "probability": winner.probability(),
        "candidates": probabilities,
    })))
}

crate::descriptions::describe! {
"Find one original unit in an ordered text[]; return zero-based index, value, probability and candidates as jsonb. Settings may admit explicit none. NULL question or units returns NULL before checking partners; NULL settings uses defaults. Failures raise SQL errors; evidence files must be read by the client.";
[name = "thinkthen_find", parallel_restricted];
/// Find one original unit, or explicit none, in an ordered text array.
fn thinkthen_find(
    question: Option<&str>,
    units: Option<Array<'_, &str>>,
    settings: default!(Option<RawJson>, "NULL"),
) -> Option<JsonB> {
    call::guarded(|| found(question, units, settings))
}
}

crate::descriptions::describe! {
"Removed positional none overload: always raise Usage, including NULL arguments. Put none and deadline in the settings object; sends nothing.";
[name = "thinkthen_find", parallel_restricted];
/// The removed positional none form fails with its replacement spelling.
fn thinkthen_find_none(
    question: Option<&str>,
    units: Option<Array<'_, &str>>,
    _none: Option<bool>,
) -> Option<JsonB> {
    call::guarded(|| {
        let _ = (question, units);
        call::raise(call::usage(
            "find's none and deadline moved into the settings object",
        ))
    })
}
}
