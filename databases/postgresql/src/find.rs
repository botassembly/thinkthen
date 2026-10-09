//! One ordered PostgreSQL array enters one public find call.

use pgrx::datum::{Array, JsonB};
use pgrx::prelude::*;
use thinkthen::{Error, For, Question};

use crate::call::{self, OrRaise as _};
use crate::ffi::RawJson;
use crate::forms::{self, Named};

const MAX_TEXT_BYTES: usize = 16 * 1024 * 1024;

fn indexed(array: Array<'_, &str>, none: bool) -> Result<Vec<String>, Error> {
    let mut bytes = 0_usize;
    let units = array
        .iter()
        .map(|member| {
            let text = member.ok_or_else(|| call::usage("a find unit is text, not NULL"))?;
            if text.trim().is_empty() {
                return Err(call::usage("a find unit is text, not white space"));
            }
            bytes = bytes
                .checked_add(text.len())
                .ok_or_else(|| call::usage("find units exceed 16 MiB of text"))?;
            if bytes > MAX_TEXT_BYTES {
                return Err(call::usage("find units exceed 16 MiB of text"));
            }
            Ok(text.to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    if units.is_empty() {
        return Ok(units);
    }
    let most = if none { 254 } else { 255 };
    if !(2..=most).contains(&units.len()) {
        return Err(call::usage(if none {
            "a find question offering none takes 2 to 254 units"
        } else {
            "find takes 2 to 255 units"
        }));
    }
    Ok(units)
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
    let units = indexed(units, none).or_raise();
    if units.is_empty() {
        return None;
    }
    let question = Question::find(question).or_raise();
    let question = if none {
        question.offering_none().or_raise()
    } else {
        question
    };
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

/// Find one original unit, or explicit none, in an ordered text array.
#[pg_extern(name = "thinkthen_find", parallel_restricted)]
fn thinkthen_find(
    question: Option<&str>,
    units: Option<Array<'_, &str>>,
    settings: default!(Option<RawJson>, "NULL"),
) -> Option<JsonB> {
    call::guarded(|| found(question, units, settings))
}

/// The removed positional none form fails with its replacement spelling.
#[pg_extern(name = "thinkthen_find", parallel_restricted)]
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
