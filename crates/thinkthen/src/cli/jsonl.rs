//! Append typed answers to admitted JSON objects, preserving their original members.
mod store;
use super::execute::Completion;
use super::input::{self, Input};
use super::options::{self, Output, usage};
use crate::contract_driver::driver::{Driver, Event};
use crate::{Cancel, Engine, Error, Record, Request};
use clap::ArgMatches;
use serde_json::{Value, value::RawValue};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use store::Store;

pub(super) fn run(
    input: Input,
    args: &ArgMatches,
    request: &Request,
    output: &Output,
    cancel: &Cancel,
) -> Result<u8, Error> {
    let name = options::explicit(args, "into").unwrap_or_default();
    if name.is_empty() || output.check {
        return Err(usage(
            "into",
            "a nonempty answer name without --check",
            "empty name or --check",
        ));
    }
    let selection = input.selection.clone();
    let (mut records, count, origins) = admit(input, &name, cancel)?;
    let engine = options::client(args)?;
    let mut answers = Store::new()?;
    let mut completion = Completion::default();
    evaluate(
        &mut records,
        &mut answers,
        (count, &selection, &origins),
        request,
        &engine,
        &mut completion,
        cancel,
    )?;
    if !output.quiet {
        publish(
            &mut records,
            &mut answers,
            count,
            &name,
            matches!(request.question(), crate::Question::Filter(_)),
            cancel,
        )?;
    }
    if matches!(request.question(), crate::Question::Filter(_)) {
        completion.summarize(output.quiet);
    }
    Ok(completion.code(false))
}

type Source = (String, u64, Box<dyn BufRead + Send>);
fn admit(input: Input, name: &str, cancel: &Cancel) -> Result<(Store, u64, input::Origins), Error> {
    let mut sources = input.sources.into_iter();
    let mut current: Option<Source> = None;
    let receiver = input::spawn(
        move || loop {
            if current.is_none() {
                let Some((path, _)) = sources.next() else {
                    return Ok(None);
                };
                let reader: Box<dyn BufRead + Send> = if path == "-" {
                    Box::new(BufReader::new(io::stdin()))
                } else {
                    Box::new(BufReader::new(File::open(&path).map_err(|_| {
                        super::local(&format!("input {path} could not be opened"))
                    })?))
                };
                current = Some((path, 0, reader));
            }
            let Some((path, line, reader)) = current.as_mut() else {
                continue;
            };
            let bytes = input::read_record(reader.as_mut(), input::Format::Jsonl, *line == 0)
                .map_err(|_| super::local("JSON Lines input could not be read"))?;
            if input::source_body(&bytes, *line == 0).is_empty() {
                current = None;
                continue;
            }
            *line += 1;
            return Ok(Some((path.clone(), *line, bytes)));
        },
        cancel.clone(),
    )?;
    let mut records = Store::new()?;
    let mut count = 0;
    let mut origins = input::Origins::default();
    while let Some((path, line, bytes)) = input::receive(&receiver, cancel)? {
        let invalid = |why: &str| {
            usage(
                "into",
                "JSON objects without the answer field",
                &format!("{path} line {line}: {why}"),
            )
        };
        let body = input::source_body(&bytes, line == 1);
        if body.len() > crate::files::MAX_RECORD_BYTES {
            return Err(invalid("record exceeds the byte limit"));
        }
        let members = serde_json::from_slice::<BTreeMap<String, Box<RawValue>>>(body)
            .map_err(|_| invalid("expected a JSON object"))?;
        if members.contains_key(name) {
            return Err(invalid("answer field already exists"));
        }
        count += 1;
        records.put(count, body)?;
        origins.record(count, path);
    }
    Ok((records, count, origins))
}

fn evaluate(
    records: &mut Store,
    answers: &mut Store,
    input: (u64, &[crate::Pointer], &input::Origins),
    request: &Request,
    engine: &Engine,
    completion: &mut Completion,
    cancel: &Cancel,
) -> Result<(), Error> {
    let (count, selection, origins) = input;
    let mut next = 1;
    let mut call = Driver::open_finite(
        &engine.clone().cancel(cancel.clone()),
        request,
        crate::OffsetUnit::CodePoints,
    )?;
    loop {
        if cancel.is_cancelled() {
            return Err(crate::retry::cancelled());
        }
        match call.next_event()? {
            Event::NeedsInput if next <= count => {
                let bytes = records.get(next)?;
                let text = std::str::from_utf8(&bytes)
                    .map_err(|_| super::local("admitted JSON could not be read"))?;
                call.send(Record::from_json(text).select_field(selection.to_vec()))?;
                next += 1;
            }
            Event::NeedsInput => call.end_input(),
            Event::Row(row) => {
                if completion.rows() == 0
                    && let Some(error) = row.failure()
                    && error.code() == crate::Code::BackendTransport
                {
                    return Err(error.clone());
                }
                completion.row(&row);
                super::execute::report_failure(&row, origins.at(row.position()));
                let row_value = serde_json::to_value(&row)
                    .map_err(|_| super::local("answer could not be encoded"))?;
                let value = answer(&row_value);
                let bytes = serde_json::to_vec(&value)
                    .map_err(|_| super::local("answer could not be encoded"))?;
                answers.put(row.position(), &bytes)?;
            }
            Event::Finished => return Ok(()),
        }
    }
}

fn answer(row: &Value) -> Value {
    match row["status"].as_str() {
        Some("unsure") => Value::String("unsure".into()),
        Some("answered") if row["function"] == "annotate" => {
            row["answer"].as_object().map_or(Value::Null, |children| {
                Value::Object(
                    children
                        .iter()
                        .map(|(name, child)| (name.clone(), answer(child)))
                        .collect(),
                )
            })
        }
        Some("answered") => row["answer"].clone(),
        _ => Value::Null,
    }
}

fn publish(
    records: &mut Store,
    answers: &mut Store,
    count: u64,
    name: &str,
    filter: bool,
    cancel: &Cancel,
) -> Result<(), Error> {
    let name = serde_json::to_string(name)
        .map_err(|_| super::local("answer name could not be encoded"))?;
    let mut output = io::stdout().lock();
    for position in 1..=count {
        if cancel.is_cancelled() {
            return Err(crate::retry::cancelled());
        }
        let value = answers.get(position)?;
        if filter && value != b"true" {
            continue;
        }
        let raw = records.get(position)?;
        let raw = std::str::from_utf8(&raw)
            .map_err(|_| super::local("admitted JSON could not be read"))?
            .trim();
        let body = &raw[..raw.len() - 1];
        let separator = if body[1..].trim().is_empty() { "" } else { "," };
        let value = if value.is_empty() {
            b"null".as_slice()
        } else {
            &value
        };
        output
            .write_all(body.as_bytes())
            .and_then(|()| write!(output, "{separator}{name}:"))
            .and_then(|()| output.write_all(value))
            .and_then(|()| writeln!(output, "}}"))
            .map_err(|error| super::output_error(error, cancel))?;
    }
    Ok(())
}
