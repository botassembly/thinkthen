//! One no-send preview over the same keyed inputs as the many tables.

use std::num::NonZeroUsize;

use rusqlite::functions::Context;
use rusqlite::types::ValueRef;
use thinkthen::{BatchSetting, CallOptions, For, LoadedQuestion, QuestionKind};

use crate::many::keyed;
use crate::question::{call_settings, question, question_with_settings, text};
use crate::{Failure, guard, settings};

fn verb(argument: &str, source: &str) -> Result<For, Failure> {
    if argument.starts_with('{') || argument.starts_with('@') {
        return match &*question(argument)? {
            LoadedQuestion::Banded(_) => Ok(For::Decide),
            LoadedQuestion::Question(asked) => match asked.kind() {
                QuestionKind::Decide => Ok(For::Decide),
                QuestionKind::Choose => Ok(For::Choose),
                QuestionKind::Score => Ok(For::Score),
                QuestionKind::Tag => Ok(For::Tag),
                _ => Err(Failure::usage("plan takes a judgment question")),
            },
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
        let estimate = match &*asked {
            LoadedQuestion::Question(question) => engine.plan_with(question, records, options)?,
            LoadedQuestion::Banded(question) => engine.plan_with(question, records, options)?,
        };
        let (lower, upper) = estimate.estimated_input_tokens();
        let first_body = estimate
            .first_body()
            .map(std::str::from_utf8)
            .transpose()
            .map_err(|_| Failure::defect("the planned body was not UTF-8"))?;
        Ok(Some(
            serde_json::json!({
                "records": estimate.records(),
                "requests": estimate.requests(),
                "estimated_bytes": estimate.estimated_bytes(),
                "estimated_input_tokens": {"lower": lower, "upper": upper},
                "upper_bound": estimate.upper_bound(),
                "first_body_utf8": first_body,
            })
            .to_string(),
        ))
    })?)
}
