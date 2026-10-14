//! Named typed fields compose native collection data before shared admission.
use super::Arguments;
use crate::{Failure, complete_native, named, question};
use rusqlite::types::ValueRef;

pub(super) fn identity(verb: &str, args: &Arguments) -> Result<String, Failure> {
    let mut parts = vec![verb.to_owned()];
    for (name, value) in args.names.iter().zip(&args.values) {
        if !matches!(name.as_str(), "projection" | "lookup_key") {
            parts.push(format!("{name}:{value:?}"));
        }
    }
    if inputs(args).is_ok()
        && let Ok(source) = source(args)
    {
        if let Some(stamp) = question::stamp(source.as_bytes()) {
            parts.push(format!("{stamp:?}"));
        }
        if let Some(name) = source.strip_prefix("@@")
            && let Ok(reference) = thinkthen::QuestionFileReference::named(name)
            && let Ok(metadata) = std::fs::metadata(reference.path())
        {
            parts.push(format!(
                "{:?}:{:?}:{}",
                reference.path(),
                metadata.modified(),
                metadata.len()
            ));
        }
    }
    for field in ["request_paths", "paths"] {
        let Some(paths) = args.text(field)? else {
            continue;
        };
        let Ok(paths) = serde_json::from_str::<Vec<String>>(&paths) else {
            continue;
        };
        for path in paths {
            path_stamp(
                std::path::Path::new(&path),
                &mut parts,
                &mut std::collections::HashSet::new(),
            );
        }
    }
    Ok(parts.join("\0"))
}

pub(super) fn source(args: &Arguments) -> Result<String, Failure> {
    for name in ["request_question", "question"] {
        if let Some(source) = args.text(name)? {
            return Ok(source);
        }
    }
    if args.names.iter().any(|name| name == "body") {
        return Ok(args.text("kinds")?.unwrap_or_else(|| "{}".into()));
    }
    if let Some(rules) = args.text("rules")? {
        return Ok(rules);
    }
    Err(Failure::usage("the native table needs a question"))
}

pub(super) fn settings(args: &Arguments) -> Result<thinkthen::Settings, Failure> {
    let source = args.text("settings")?.unwrap_or_else(|| "{}".into());
    let values = [
        "model",
        "context",
        "batch",
        "deadline_ms",
        "threshold",
        "true_reading",
        "false_reading",
        "options",
        "levels",
        "labels",
        "none",
    ]
    .map(|name| args.value(name));
    let settings = named::settings(source.as_bytes(), &values)?;
    question::call_settings(ValueRef::Text(&settings))
}

pub(super) fn inputs(args: &Arguments) -> Result<complete_native::Inputs, thinkthen::Error> {
    let image = match args.value("images") {
        ValueRef::Null => args.value("text"),
        value => value,
    };
    if let ValueRef::Blob(bytes) = image {
        let mut inputs = complete_native::Inputs::from_record(crate::images::complete_record(
            bytes,
            args.text("image_text")?,
        )?);
        inputs.attempts = args.flag("attempts")?;
        inputs.incremental = args.flag("incremental")?;
        inputs.cancelled = args.flag("cancelled")?;
        return Ok(inputs);
    }
    let records = args.text("request_records")?;
    let paths = args.text("request_paths")?.or(args.text("paths")?);
    let mut fields = Vec::new();
    if let Some(records) = records {
        fields.push(format!("\"records\":{records}"));
    }
    if let Some(paths) = paths {
        let mut file = vec![format!("\"paths\":{paths}")];
        let mut reading = Vec::new();
        for name in ["request_unit", "unit"] {
            text_field(args, name, "unit", &mut reading)?;
        }
        for name in ["request_window", "window"] {
            integer_field(args, name, "window", &mut reading)?;
        }
        let mut options = vec![format!("\"reading\":{{{}}}", reading.join(","))];
        for name in ["request_media", "media"] {
            text_field(args, name, "media", &mut options)?;
        }
        file.push(format!("\"options\":{{{}}}", options.join(",")));
        for name in ["request_format", "format"] {
            text_field(args, name, "format", &mut file)?;
        }
        fields.push(format!("\"files\":{{{}}}", file.join(",")));
    }
    if fields.is_empty() {
        let text = args.text("text")?.or(args.text("body")?);
        let records = if let Some(text) = text {
            serde_json::json!([{"text":text}]).to_string()
        } else if let Some(keyed) = args.text("keyed_json")? {
            let rows = crate::many::keyed(&keyed)?
                .into_iter()
                .map(|(_, text)| serde_json::json!({"text":text}))
                .collect::<Vec<_>>();
            serde_json::Value::Array(rows).to_string()
        } else if let Some(paths) = args.text("paths")? {
            fields.push(format!("\"files\":{{\"paths\":{paths}}}"));
            String::new()
        } else {
            return Err(Failure::usage("supply native records, text or file paths").into());
        };
        if fields.is_empty() {
            fields.push(format!("\"records\":{records}"));
        }
    }
    let mut reading = Vec::new();
    for (name, field) in [
        ("record_fields", "fields"),
        ("record_context_field", "context"),
        ("record_context_schema", "context_schema"),
        ("record_options_field", "options"),
        ("record_seed_spans_field", "seed_spans"),
        ("record_examples_field", "examples"),
    ] {
        if matches!(name, "record_fields" | "record_context_schema") {
            if let Some(value) = args.text(name)? {
                reading.push(format!("\"{field}\":{value}"));
            } else if args.supplied(name) {
                reading.push(format!("\"{field}\":null"));
            }
        } else {
            text_field(args, name, field, &mut reading)?;
        }
    }
    if !reading.is_empty() {
        fields.push(format!("\"reading\":{{{}}}", reading.join(",")));
    }
    for name in ["incremental", "attempts", "cancelled"] {
        fields.push(format!("\"{name}\":{}", args.flag(name)?));
    }
    complete_native::Inputs::parse_request(&format!("{{{}}}", fields.join(",")))
}

fn text_field(
    args: &Arguments,
    name: &str,
    field: &str,
    output: &mut Vec<String>,
) -> Result<(), Failure> {
    if let Some(value) = args.text(name)? {
        output.push(format!("\"{field}\":{}", serde_json::json!(value)));
    } else if args.supplied(name) && (name.starts_with("request_") || name.starts_with("record_")) {
        output.push(format!("\"{field}\":null"));
    }
    Ok(())
}
fn integer_field(
    args: &Arguments,
    name: &str,
    field: &str,
    output: &mut Vec<String>,
) -> Result<(), Failure> {
    if let Some(value) = args.number(name)? {
        output.push(format!("\"{field}\":{value}"));
    }
    Ok(())
}

pub(super) fn options(args: &Arguments) -> Result<thinkthen::RequestOptions, Failure> {
    let mut fields = Vec::new();
    text_field(args, "mode", "mode", &mut fields)?;
    for name in ["snippet_pieces", "top", "max_requests_total"] {
        integer_field(args, name, name, &mut fields)?;
    }
    for name in ["seed_spans", "examples", "stage_context"] {
        if let Some(value) = args.text(name)? {
            fields.push(format!("\"{name}\":{value}"));
        }
    }
    match args.value("relation_threshold") {
        ValueRef::Null => (),
        ValueRef::Real(value) => fields.push(format!("\"relation_threshold\":{value}")),
        ValueRef::Integer(value) => fields.push(format!("\"relation_threshold\":{value}")),
        value => {
            if let Some(value) = question::text(value, "relation_threshold")? {
                fields.push(format!(
                    "\"relation_threshold\":{}",
                    serde_json::json!(value)
                ));
            }
        }
    }
    fields.push(format!(
        "\"files_only\":{}",
        args.flag("request_files_only")? || args.flag("files_only")?
    ));
    if let Some(fields_data) = args.text("fields")? {
        fields.push(format!("\"field\":{fields_data}"));
    }
    text_field(args, "context_field", "context_field", &mut fields)?;
    fields.push(format!("\"attempts\":{}", args.flag("attempts")?));
    serde_json::from_str(&format!("{{{}}}", fields.join(",")))
        .map_err(|error| Failure::usage(format!("invalid named request options: {error}")))
}

pub(super) fn keys(args: &Arguments) -> Result<Vec<String>, Failure> {
    args.text("keyed_json")?
        .map(|source| {
            crate::many::keyed(&source).map(|rows| rows.into_iter().map(|(key, _)| key).collect())
        })
        .transpose()
        .map(Option::unwrap_or_default)
}

pub(super) fn query_arguments(args: &Arguments, entities: &[thinkthen::Entity]) -> Arguments {
    let records = entities
        .iter()
        .map(|entity| serde_json::json!({"json":{"name":entity.name(),"kind":entity.kind()}}))
        .collect::<Vec<_>>();
    args.with_value(
        "request_records",
        rusqlite::types::Value::Text(serde_json::Value::Array(records).to_string()),
    )
}

fn path_stamp(
    path: &std::path::Path,
    parts: &mut Vec<String>,
    seen: &mut std::collections::HashSet<std::path::PathBuf>,
) {
    let Ok(metadata) = std::fs::metadata(path) else {
        parts.push(format!("{path:?}:missing"));
        return;
    };
    parts.push(format!(
        "{path:?}:{:?}:{}",
        metadata.modified(),
        metadata.len()
    ));
    if metadata.is_dir()
        && seen.insert(std::fs::canonicalize(path).unwrap_or_else(|_| path.to_owned()))
        && let Ok(children) = std::fs::read_dir(path)
    {
        let mut children = children
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        children.sort();
        for child in children {
            path_stamp(&child, parts, seen);
        }
    }
}
