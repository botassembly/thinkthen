//! One ordered SQL collection enters one public find call.

use rusqlite::functions::Context;
use rusqlite::types::ValueRef;
use thinkthen::{Evidence, Question};

use super::deadline_at;
use crate::question::text;
use crate::{Failure, ffi, guard, worker};

const MAX_TEXT_BYTES: usize = 16 * 1024 * 1024;

/// Carry the original SQL position even when two units have equal text.
#[derive(Debug)]
struct Indexed {
    position: usize,
    text: String,
}

impl Evidence for Indexed {
    fn evidence(&self) -> &str {
        &self.text
    }
}

fn units(argument: &str) -> Result<Vec<Indexed>, Failure> {
    let source: serde_json::Value = serde_json::from_str(argument)
        .map_err(|_| Failure::usage("find units are a JSON array of text"))?;
    let array = source
        .as_array()
        .ok_or_else(|| Failure::usage("find units are a JSON array of text"))?;
    let mut bytes = 0_usize;
    array
        .iter()
        .enumerate()
        .map(|(position, value)| {
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
            Ok(Indexed {
                position,
                text: text.to_owned(),
            })
        })
        .collect()
}

fn offered(context: &Context<'_>) -> Result<bool, Failure> {
    if context.len() < 3 {
        return Ok(false);
    }
    match context.get_raw(2) {
        ValueRef::Integer(0) => Ok(false),
        ValueRef::Integer(1) => Ok(true),
        _ => Err(Failure::usage("find none is the integer 0 or 1")),
    }
}

/// `thinkthen_find(question, units_json[, none[, deadline_ms]])`.
pub(super) fn find(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_find", || {
        if (0..context.len()).any(|slot| matches!(context.get_raw(slot), ValueRef::Null)) {
            return Ok(None);
        }
        let argument = text(context.get_raw(0), "the question")?
            .ok_or_else(|| Failure::defect("a checked find question was NULL"))?;
        let source = text(context.get_raw(1), "the units")?
            .ok_or_else(|| Failure::defect("checked find units were NULL"))?;
        let none = offered(context)?;
        let deadline = deadline_at(context, 3)?;
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
        let question = if none {
            question.offering_none()?
        } else {
            question
        };
        let answer = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
            let call = engine.find_with(&question, units, options)?;
            let found = call.into_value();
            let selected = found.selected();
            let candidates = found.candidates();
            let winner = match selected {
                Some(unit) => candidates.get(unit.position),
                None => candidates.last().filter(|candidate| candidate.is_none()),
            }
            .ok_or_else(|| Failure::defect("a find answer selected no candidate"))?;
            let candidates: Vec<_> = candidates
                .iter()
                .map(|candidate| {
                    serde_json::json!({
                        "index": candidate.input().map(|unit| unit.position),
                        "probability": candidate.probability(),
                    })
                })
                .collect();
            Ok(serde_json::json!({
                "index": selected.map(|unit| unit.position),
                "value": selected.map(|unit| unit.text.as_str()),
                "probability": winner.probability(),
                "candidates": candidates,
            })
            .to_string())
        })?;
        Ok(Some(answer))
    })?)
}
