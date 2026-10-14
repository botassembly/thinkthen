//! All projections belong to one canonical admitted Request and one call.
use super::{Arguments, input};
use crate::receipt::Receipt;
use crate::{Failure, complete_native, worker};
use rusqlite::ffi::sqlite3;
use rusqlite::types::Value;
use serde_json::{Value as Json, json};
use thinkthen::{Error, ErrorKind, Surface};
use thinkthen_host::sql_request;

pub(super) fn always_native(verb: &str) -> bool {
    matches!(
        verb,
        "decide"
            | "choose"
            | "tag"
            | "score"
            | "find"
            | "annotate"
            | "details"
            | "details_images"
            | "rank_set"
    )
}
pub(super) fn root(verb: &str) -> &str {
    match verb {
        "decide" => "decision",
        "choose" => "choice",
        "tag" => "tags",
        "score" => "score",
        "filter" => "filtered",
        "rank" | "rank_set" => "ranked",
        "find" => "found",
        "annotate" => "annotation",
        "recognize" => "recognized",
        "relate" => "related",
        "details" | "details_images" => "question_detail",
        _ => "error",
    }
}

pub(super) fn execute(
    db: *mut sqlite3,
    verb: &'static str,
    args: &Arguments,
) -> Result<Receipt, Failure> {
    let result = invoke(db, verb, args);
    let carrier = match result {
        Ok(value) => value,
        Err(error) if matches!(error.kind, ErrorKind::Cancelled | ErrorKind::Deadline) => {
            return Err(error);
        }
        Err(error) => complete_native::admission(&Error::new(error.kind, error.message)),
    };
    let mut receipt = Receipt::default();
    let native = carrier
        .get("native")
        .ok_or_else(|| Failure::defect("a native call lost its receipt"))?;
    if let Some(facts) = native.get("facts") {
        receipt.add("facts", facts)?;
    }
    if let Some(error) = native.get("error") {
        receipt.add("error", error)?;
    }
    let completed = carrier.get("completed");
    let value = native.get("value").or(completed);
    let values = value.map_or(Vec::new(), |value| {
        value
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![value.clone()])
    });
    let ordinals = carrier.get("ordinals").and_then(Json::as_array);
    for (at, value) in values.iter().enumerate() {
        let actual = if matches!(verb, "details" | "details_images") {
            detail_root(value)
        } else {
            root(verb)
        };
        let index = receipt.add(actual, value)?;
        receipt.add(
            "record_order",
            &json!({"ordinal":ordinals.and_then(|v|v.get(at)).cloned().unwrap_or(json!(at)),
            "result_index":index,"completed":completed.is_some()}),
        )?;
    }
    if let Some(selection) = carrier.get("selection") {
        receipt.add("find_selection", &json!({"selected":selection}))?;
    }
    for event in carrier
        .get("observations")
        .and_then(Json::as_array)
        .into_iter()
        .flatten()
    {
        receipt.add("event", event)?;
    }
    receipt.keys = input::keys(args).unwrap_or_default();
    receipt.carrier = carrier;
    Ok(receipt)
}

fn invoke(db: *mut sqlite3, verb: &'static str, args: &Arguments) -> Result<Json, Failure> {
    let inputs = input::inputs(args)?;
    let source = input::source(args)?;
    let settings = input::settings(args)?;
    let actual = if matches!(verb, "details" | "details_images") {
        match crate::question::question(&source)?.kind() {
            thinkthen::QuestionKind::Decide => "decide",
            thinkthen::QuestionKind::Choose => "choose",
            thinkthen::QuestionKind::Tag => "tag",
            thinkthen::QuestionKind::Score => "score",
            _ => return Err(Failure::usage("details takes a judgment question")),
        }
    } else if verb == "rank_set" {
        "rank"
    } else {
        verb
    };
    let selector = crate::complete::selector::admit(actual, &source)?;
    let prepared = crate::complete::prepare(actual, &source, &settings)?;
    let options = input::options(args)?;
    let request = match selector {
        Some(selector) => {
            let resolved = selector.with_resolved_definition(sql_request::definition(&prepared))?;
            let mut arguments = resolved.request().call.arguments().clone();
            arguments.options = options;
            thinkthen::Request::for_function(resolved.request().call.function(), arguments)
                .admit()?
        }
        None => sql_request::request(&prepared, options)?.admit()?,
    };
    let shared = settings.context().map(str::to_owned);
    worker::run_settings(db, settings, move |engine, controls| {
        let controls = shared
            .as_deref()
            .map_or(controls, |context| controls.context(context));
        Ok(sql_request::run(
            engine,
            &prepared,
            &request,
            inputs,
            controls,
            Surface::Sqlite,
        ))
    })
}

pub(super) fn answers(
    verb: &str,
    receipt: &Receipt,
    rows: &mut [Vec<Value>],
) -> Result<(), Failure> {
    let native = receipt.carrier.get("native");
    let values = native
        .and_then(|value| value.get("value"))
        .or_else(|| receipt.carrier.get("completed"));
    let values = values.map_or(Vec::new(), |value| {
        value
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![value.clone()])
    });
    for (row, value) in rows.iter_mut().zip(values.iter()) {
        if matches!(verb, "rank" | "rank_set") {
            rank_columns(row, value, receipt);
            continue;
        }
        if !always_native(verb) {
            continue;
        }
        let answer = value.get("value").unwrap_or(&Json::Null);
        let value = match answer {
            Json::Bool(value) => Value::Integer(i64::from(*value)),
            Json::String(value) => Value::Text(value.clone()),
            Json::Number(value) => {
                if verb == "score" {
                    value.as_f64().map_or(Value::Null, Value::Real)
                } else {
                    value.as_i64().map_or(Value::Null, Value::Integer)
                }
            }
            Json::Null => Value::Null,
            Json::Array(_) | Json::Object(_) => Value::Text(answer.to_string()),
        };
        if let Some(slot) = row.first_mut() {
            *slot = value.clone();
        }
        if matches!(verb, "decide" | "choose")
            && let Some(slot) = row.get_mut(1)
        {
            *slot = value_probability(value, &receipt.carrier);
        }
        if matches!(verb, "details" | "details_images") {
            detail_columns(row, &receipt.carrier);
        }
    }
    Ok(())
}

fn value_probability(_: Value, carrier: &Json) -> Value {
    carrier
        .pointer("/native/value/0/answer/probability")
        .and_then(Json::as_f64)
        .map_or(Value::Null, Value::Real)
}

fn detail_root(row: &Json) -> &str {
    let question = row.get("question").unwrap_or(&Json::Null);
    if question.get("choose").is_some() {
        "choice"
    } else if question.get("tag").is_some() {
        "tags"
    } else if question.get("score").is_some() {
        "score"
    } else {
        "decision"
    }
}

fn detail_columns(row: &mut [Value], carrier: &Json) {
    let detail = carrier
        .get("observations")
        .and_then(Json::as_array)
        .into_iter()
        .flatten()
        .find_map(|event| event.get("detail"));
    let Some(detail) = detail else {
        return;
    };
    let columns = [
        detail
            .get("confidence")
            .and_then(Json::as_f64)
            .map_or(Value::Null, Value::Real),
        detail
            .get("answer_id")
            .and_then(Json::as_str)
            .map_or(Value::Null, |value| Value::Text(value.to_owned())),
        detail
            .get("model")
            .and_then(Json::as_str)
            .map_or(Value::Null, |value| Value::Text(value.to_owned())),
        detail
            .get("requests_sent")
            .and_then(Json::as_i64)
            .map_or(Value::Null, Value::Integer),
    ];
    for (slot, value) in row.iter_mut().skip(1).zip(columns) {
        *slot = value;
    }
}

pub(super) fn legacy_error(receipt: &Receipt) -> Result<(), Failure> {
    let Some(error) = receipt.carrier.pointer("/native/error") else {
        return Ok(());
    };
    let kind = match error.get("kind").and_then(Json::as_str) {
        Some("usage") => ErrorKind::Usage,
        Some("local") => ErrorKind::Local,
        Some("cancelled") => ErrorKind::Cancelled,
        Some("deadline") => ErrorKind::Deadline,
        Some("defect") => ErrorKind::Defect,
        _ => ErrorKind::Backend,
    };
    let mut failure = Failure::of(
        kind,
        error
            .get("message")
            .and_then(Json::as_str)
            .unwrap_or("the native call failed"),
    );
    failure.retryable = error
        .get("retryable")
        .and_then(Json::as_bool)
        .unwrap_or(false);
    Err(failure)
}

fn rank_columns(row: &mut [Value], value: &Json, receipt: &Receipt) {
    let index = value.get("index").and_then(Json::as_u64).unwrap_or(0) as usize;
    let key = receipt
        .keys
        .get(index)
        .cloned()
        .unwrap_or_else(|| index.to_string());
    let columns = [
        Value::Text(key),
        value
            .get("value")
            .and_then(Json::as_i64)
            .map_or(Value::Null, Value::Integer),
        value
            .pointer("/answer/probability")
            .and_then(Json::as_f64)
            .map_or(Value::Null, Value::Real),
        value
            .get("question_name")
            .and_then(Json::as_str)
            .map_or(Value::Null, |name| Value::Text(name.to_owned())),
    ];
    for (slot, value) in row.iter_mut().take(4).zip(columns) {
        *slot = value;
    }
}
