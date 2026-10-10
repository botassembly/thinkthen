//! One ordered SQL collection enters one public find call.

use rusqlite::functions::Context;
use rusqlite::types::ValueRef;
use thinkthen::{For, Question, Settings};

use crate::question::{call_settings, text};
use crate::{Failure, ffi, guard, worker};

const MAX_TEXT_BYTES: usize = 16 * 1024 * 1024;

fn units(argument: &str) -> Result<Vec<String>, Failure> {
    let source: serde_json::Value = serde_json::from_str(argument)
        .map_err(|_| Failure::usage("find units are a JSON array of text"))?;
    let array = source
        .as_array()
        .ok_or_else(|| Failure::usage("find units are a JSON array of text"))?;
    let mut bytes = 0_usize;
    array
        .iter()
        .map(|value| {
            let text = value.as_str().ok_or_else(|| {
                Failure::usage("each find unit is text, not NULL or another type")
            })?;
            if text.trim().is_empty() {
                return Err(Failure::usage("a find unit is text, not white space"));
            }
            bytes = bytes
                .checked_add(text.len())
                .ok_or_else(|| Failure::usage("find units exceed 16 MiB of text"))?;
            if bytes > MAX_TEXT_BYTES {
                return Err(Failure::usage("find units exceed 16 MiB of text"));
            }
            Ok(text.to_owned())
        })
        .collect()
}

/// `thinkthen_find(question, units_json[, settings])`.
pub(super) fn find(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_find", || {
        if (0..2).any(|slot| matches!(context.get_raw(slot), ValueRef::Null)) {
            return Ok(None);
        }
        let argument = text(context.get_raw(0), "the question")?
            .ok_or_else(|| Failure::defect("a checked find question was NULL"))?;
        let source = text(context.get_raw(1), "the units")?
            .ok_or_else(|| Failure::defect("checked find units were NULL"))?;
        let settings = if context.len() > 2 {
            if matches!(context.get_raw(2), ValueRef::Integer(_) | ValueRef::Real(_)) {
                return Err(Failure::plain_usage(
                    "find's none and deadline moved into the settings object",
                ));
            }
            call_settings(context.get_raw(2))?
        } else {
            Settings::default()
        };
        settings
            .check(For::Find)
            .map_err(|error| Failure::usage(error.to_string()))?;
        let none = settings.none().unwrap_or(false);
        let shared = settings.context().map(str::to_owned);
        let units = units(&source)?;
        if units.is_empty() {
            return Ok(None);
        }
        let most = if none { 254 } else { 255 };
        if !(2..=most).contains(&units.len()) {
            return Err(Failure::usage(if none {
                "a find question offering none takes 2 to 254 units"
            } else {
                "find takes 2 to 255 units"
            }));
        }
        let question = Question::find(&argument)?;
        let question = match settings.model() {
            Some(model) => question.with_model(model)?,
            None => question,
        };
        let question = if none {
            question.offering_none()?
        } else {
            question
        };
        let answer =
            worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
                let options = if let Some(shared) = &shared {
                    options.context(shared)
                } else {
                    options
                };
                let call = crate::request::run(
                    engine,
                    thinkthen::RequestFunction::Find,
                    question.into(),
                    units.into_iter().map(crate::request::text).collect(),
                    options,
                )?;
                let thinkthen::RequestValue::Found(found) = call.into_value() else {
                    return Err(crate::request::wrong_result().into());
                };
                project(&found)
            })?;
        Ok(Some(answer))
    })?)
}

fn project(found: &thinkthen::CompleteFound<thinkthen::QuestionInput>) -> Result<String, Failure> {
    let selected = match found.selection() {
        thinkthen::FindSelection::Unit(at) => Some(at),
        thinkthen::FindSelection::None => None,
    };
    let candidates = found.candidates();
    let winner = match selected {
        Some(at) => candidates.get(at),
        None => candidates.last().filter(|candidate| candidate.is_none()),
    }
    .ok_or_else(|| Failure::defect("a find answer selected no candidate"))?;
    let candidates: Vec<_> = candidates
        .iter()
        .enumerate()
        .map(|(at, candidate)| {
            serde_json::json!({
                "index": candidate.input().map(|_| at),
                "probability": candidate.probability(),
            })
        })
        .collect();
    Ok(serde_json::json!({
        "index": selected,
        "value": found.selected().map(|unit| match unit {
            thinkthen::QuestionInput::Text(text) => Ok(text.as_str()),
            thinkthen::QuestionInput::Record(record) => record.original().literal()
                .ok_or_else(|| Failure::defect("a find unit lost its text")),
            _ => Err(Failure::defect("a find unit held no text")),
        }).transpose()?,
        "probability": winner.probability(),
        "candidates": candidates,
    })
    .to_string())
}
