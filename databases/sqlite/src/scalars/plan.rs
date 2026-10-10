//! One no-send preview over the same keyed inputs as the many tables.

use std::num::NonZeroUsize;

use rusqlite::functions::Context;
use rusqlite::types::ValueRef;
use thinkthen::{BatchSetting, CallOptions, For, QuestionKind};

use crate::many::keyed;
use crate::question::{call_settings, question, question_with_settings, text};
use crate::{Failure, guard, settings};

fn verb(argument: &str, source: &str) -> Result<For, Failure> {
    if argument.starts_with('{') || argument.starts_with('@') {
        return match question(argument)?.kind() {
            QuestionKind::Decide => Ok(For::Decide),
            QuestionKind::Choose => Ok(For::Choose),
            QuestionKind::Score => Ok(For::Score),
            QuestionKind::Tag => Ok(For::Tag),
            _ => Err(Failure::usage("plan takes a judgment question")),
        };
    }
    let object: serde_json::Value = serde_json::from_str(source)
        .map_err(|_| Failure::usage("the settings are one JSON object"))?;
    let fields = object
        .as_object()
        .ok_or_else(|| Failure::usage("the settings are one JSON object"))?;
    Ok(if fields.contains_key("options") {
        For::Choose
    } else if fields.contains_key("levels") {
        For::Score
    } else if fields.contains_key("labels") {
        For::Tag
    } else {
        For::Decide
    })
}

pub(super) fn plan(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_plan", || {
        if (0..2).any(|slot| matches!(context.get_raw(slot), ValueRef::Null)) {
            return Ok(None);
        }
        let Some(argument) = text(context.get_raw(0), "the question")? else {
            return Ok(None);
        };
        let Some(packed) = text(context.get_raw(1), "the keyed records")? else {
            return Ok(None);
        };
        let source = match context.len() {
            3 => match context.get_raw(2) {
                ValueRef::Null => "{}".to_owned(),
                value => text(value, "the settings")?.unwrap_or_else(|| "{}".to_owned()),
            },
            _ => "{}".to_owned(),
        };
        let controls = call_settings(ValueRef::Text(source.as_bytes()))?;
        let verb = verb(&argument, &source)?;
        let asked = question_with_settings(&argument, &controls, verb)?;
        let records: Vec<_> = keyed(&packed)?.into_iter().map(|(_, text)| text).collect();
        let mut options = CallOptions::new();
        if controls.batch_max() {
            options = options.batch(BatchSetting::Max);
        } else if let Some(records) = controls.batch_records() {
            let count = NonZeroUsize::new(records)
                .ok_or_else(|| Failure::defect("validated batch had zero records"))?;
            options = options.batch(BatchSetting::Records(count));
        }
        if let Some(shared) = controls.context() {
            options = options.context(shared);
        }
        let engine = settings::engine()?;
        let estimate = engine.plan_with(&*asked, records, options)?;
        // A `Value` keeps its members in alphabetical order, as this text always had.
        let written = serde_json::to_value(&estimate)
            .map_err(|_| Failure::defect("the plan did not serialize"))?;
        Ok(Some(written.to_string()))
    })?)
}
