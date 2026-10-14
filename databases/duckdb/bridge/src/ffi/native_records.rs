//! Native SQL record collections retain canonical request admission and receipts.
use super::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, batch, probe, reply_boundary,
    run_detached, text,
};
use crate::{complete_native, engines};
use serde_json::{Value, value::RawValue};
use std::collections::BTreeMap;
use thinkthen::{
    Error, InputEvidence, Request, RequestArguments, RequestEnvironment, RequestFeed, RequestInput,
    RequestOptions, RequestOriginal, RequestOutcome, RequestQuestion,
};

type Fields = BTreeMap<String, Box<RawValue>>;

#[derive(Debug)]
struct Record {
    item: thinkthen::RequestItem,
    location: Option<thinkthen::SourceLocation>,
}

fn fields(raw: &str) -> Result<Fields, Error> {
    match thinkthen::Settings::parse(&format!("{{\"sql_record\":{raw}}}")) {
        Ok(_) | Err(thinkthen::SettingsError::UnknownKey(_)) => {}
        Err(error) => return Err(complete_native::usage(&error.to_string())),
    }
    serde_json::from_str(raw).map_err(|_| complete_native::usage("native SQL record is one object"))
}

fn read_record(raw: &str) -> Result<Record, Error> {
    let mut given = fields(raw)?;
    if given.contains_key("read_error") {
        let input = complete_native::Inputs::parse_request(&format!("{{\"records\":[{raw}]}}"))?;
        let mut records = input.records(None)?;
        return match records.next() {
            Some(Err(error)) => Err(error),
            _ => Err(complete_native::defect()),
        };
    }
    let json_present = given
        .remove("json_present")
        .map(|value| value.get() == "true")
        .unwrap_or(true);
    given.retain(|key, value| value.get() != "null" || key == "json" && json_present);
    let source = given.remove("source");
    let mut originals = Vec::new();
    for name in ["text", "json", "document"] {
        if let Some(value) = given.remove(name) {
            let original = match name {
                "json" => format!("{{\"kind\":\"json\",\"value\":{}}}", value.get()),
                "text" => format!("{{\"kind\":\"text\",\"text\":{}}}", value.get()),
                _ => {
                    let document: String = serde_json::from_str(value.get())
                        .map_err(|_| complete_native::usage("document is text"))?;
                    let value = thinkthen::RawRecord::json(&document)
                        .or_else(|_| thinkthen::RawRecord::text(&document))?;
                    serde_json::to_string(&match value.literal() {
                        Some(text) => RequestOriginal::Text {
                            text: text.to_owned(),
                        },
                        None => RequestOriginal::Json { value },
                    })
                    .map_err(|_| complete_native::defect())?
                }
            };
            originals.push(original);
        }
    }
    if originals.len() > 1 {
        return Err(complete_native::usage("one original per record"));
    }
    if let Some(images) = given.get_mut("images") {
        *images = image_descriptors(images.get())?;
    }
    let mut descriptor = String::from("{");
    if let Some(original) = originals.first() {
        descriptor.push_str("\"original\":");
        descriptor.push_str(original);
    }
    for (name, value) in given {
        if descriptor.len() > 1 {
            descriptor.push(',');
        }
        descriptor.push_str(&serde_json::to_string(&name).map_err(|_| complete_native::defect())?);
        descriptor.push(':');
        descriptor.push_str(value.get());
    }
    descriptor.push('}');
    let item = serde_json::from_str(&descriptor).map_err(|_| {
        complete_native::usage("native record fields, options, context or images are invalid")
    })?;
    let location = source
        .map(|value| source_location(value.get()))
        .transpose()?;
    Ok(Record { item, location })
}

fn image_descriptors(raw: &str) -> Result<Box<RawValue>, Error> {
    let values: Vec<Value> = serde_json::from_str(raw)
        .map_err(|_| complete_native::usage("images is an ordered native collection"))?;
    let images = values
        .into_iter()
        .map(|image| {
            let media = image
                .get("media")
                .and_then(Value::as_str)
                .ok_or_else(|| complete_native::usage("image requires a media type"))?;
            let bytes = image
                .get("data")
                .and_then(Value::as_str)
                .ok_or_else(|| complete_native::usage("images require native BLOB content"))?;
            Ok(serde_json::json!({"kind":"bytes","media":media,"bytes":bytes}))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    RawValue::from_string(serde_json::to_string(&images).map_err(|_| complete_native::defect())?)
        .map_err(|_| complete_native::defect())
}

fn coordinate(fields: &mut Fields, name: &str) -> Result<Option<usize>, Error> {
    fields
        .remove(name)
        .filter(|value| value.get() != "null")
        .map(|value| {
            serde_json::from_str(value.get())
                .map_err(|_| complete_native::usage("source coordinate is a positive whole number"))
        })
        .transpose()
}

fn source_location(raw: &str) -> Result<thinkthen::SourceLocation, Error> {
    let mut fields = fields(raw)?;
    let file = fields
        .remove("file")
        .ok_or_else(|| complete_native::usage("source requires a file identity"))?;
    let file = serde_json::from_str(file.get())
        .map_err(|_| complete_native::usage("source file is text"))?;
    let location = thinkthen::SourceLocation::new(
        file,
        coordinate(&mut fields, "first_line")?,
        coordinate(&mut fields, "last_line")?,
    )?;
    if !fields.is_empty() {
        return Err(complete_native::usage("unknown source field"));
    }
    Ok(location)
}

fn options(raw: &str) -> Result<RequestOptions, Error> {
    let mut fields = fields(raw)?;
    fields.retain(|_, value| value.get() != "null");
    fields.remove("context_schema");
    if let Some(stage) = fields.get_mut("stage_context") {
        let mut members = self::fields(stage.get())?;
        members.retain(|_, value| value.get() != "null");
        *stage = RawValue::from_string(
            serde_json::to_string(&members).map_err(|_| complete_native::defect())?,
        )
        .map_err(|_| complete_native::defect())?;
    }
    serde_json::from_str(&serde_json::to_string(&fields).map_err(|_| complete_native::defect())?)
        .map_err(|_| complete_native::usage("reading uses named native request controls"))
}

fn composed_request(
    verb: &str,
    admitted: &thinkthen::AdmittedRequest,
) -> Result<thinkthen::AdmittedRequest, Error> {
    let mut args = admitted.request().call.arguments().clone();
    args.options.field = None;
    args.options.context_field = None;
    args.options.options_field = None;
    args.options.examples_field = None;
    args.options.seed_spans_field = None;
    Request::named(verb, args)?.admit()
}

fn compose(
    record: Record,
    reading: &thinkthen::RecordReading,
) -> Result<thinkthen::RecordInput<thinkthen::QuestionInput>, Error> {
    let mut row = record.item.compose_record(reading)?;
    if let Some(location) = record.location {
        row.original = match row.original {
            thinkthen::QuestionInput::Record(record) => {
                thinkthen::QuestionInput::Record(record.with_location(location))
            }
            thinkthen::QuestionInput::Images(images)
                if location.first_line().is_none() && images.images().len() == 1 =>
            {
                thinkthen::ImageSourceRecord {
                    record: images
                        .images()
                        .first()
                        .cloned()
                        .ok_or_else(complete_native::defect)?,
                    file: location.file().to_owned(),
                }
                .question_input()
            }
            _ => {
                return Err(complete_native::usage(
                    "source coordinates require a text record",
                ));
            }
        };
    }
    Ok(row)
}

pub(super) fn decision_value(answer: thinkthen::Answer) -> Value {
    match answer {
        thinkthen::Answer::Yes => Value::Bool(true),
        thinkthen::Answer::No => Value::Bool(false),
        thinkthen::Answer::Unsure => Value::Null,
    }
}

pub(super) fn plain_values(value: &thinkthen::RequestValue) -> Value {
    use thinkthen::RequestValue;
    macro_rules! rows {
        ($rows:expr) => {
            serde_json::json!(
                $rows
                    .iter()
                    .map(|row| row.result().value())
                    .collect::<Vec<_>>()
            )
        };
    }
    match value {
        RequestValue::Decisions(rows) => Value::Array(
            rows.iter()
                .map(|row| decision_value(row.result().value()))
                .collect(),
        ),
        RequestValue::Choices(rows) => rows!(rows),
        RequestValue::Tags(rows) => rows!(rows),
        RequestValue::Scores(rows) => rows!(rows),
        RequestValue::Filtered(rows) => rows!(rows),
        RequestValue::Ranked(rows) => rows!(rows),
        RequestValue::SetRanked(rows) => rows!(rows),
        _ => Value::Null,
    }
}

fn render(outcome: RequestOutcome, observations: Value) -> Result<Value, Error> {
    let answers = match &outcome {
        RequestOutcome::Complete(call) => plain_values(call.value()),
        RequestOutcome::Failed { completed, .. } => plain_values(completed),
    };
    let mut native = match outcome {
        RequestOutcome::Complete(call) => {
            let mut value = serde_json::to_value(&call).map_err(|_| complete_native::defect())?;
            super::complete::supplement(&mut value, call.value())?;
            value
        }
        RequestOutcome::Failed { completed, error } => {
            let mut value = complete_native::failure(&error);
            super::complete::supplement(&mut value, &completed)?;
            complete_native::put(
                &mut value,
                "completed",
                serde_json::to_value(completed).map_err(|_| complete_native::defect())?,
            )?;
            value
        }
    };
    // The strict native envelope and its supplements come from the same call.
    let mut carrier = complete_native::carrier(std::mem::take(&mut native), observations);
    complete_native::put(&mut carrier, "answers", answers)?;
    Ok(carrier)
}

#[derive(Debug)]
struct PreparedNative {
    call: thinkthen::Settings,
    request: thinkthen::AdmittedRequest,
    projection: thinkthen::RecordReading,
    records: Vec<Result<Record, Error>>,
}

pub(super) fn header(
    verb: &str,
    prepared: &complete_native::Prepared,
    reading: &str,
) -> Result<(thinkthen::AdmittedRequest, thinkthen::RecordReading), Error> {
    let options = options(reading)?;
    let admitted = Request::named(
        verb,
        RequestArguments {
            question: RequestQuestion::Definition {
                value: super::complete::request::definition(prepared),
            },
            input: RequestInput::Feed {
                name: "duckdb".to_owned(),
                framing: Default::default(),
                reading: Default::default(),
                images: Vec::new(),
            },
            options,
        },
    )?
    .admit()?;
    let mut projection = admitted.record_reading()?;
    if let Some(schema) = fields(reading)?
        .get("context_schema")
        .filter(|raw| raw.get() != "null")
    {
        let question = thinkthen::Question::from_json(&format!(
            "{{\"decide\":\"Context declaration\",\"context_schema\":{}}}",
            schema.get()
        ))?;
        let thinkthen::LoadedQuestion::Question(question) = question else {
            return Err(complete_native::defect());
        };
        projection = projection.with_context_schema(
            question
                .context_schema()
                .cloned()
                .ok_or_else(complete_native::defect)?,
        );
    }
    Ok((composed_request(verb, &admitted)?, projection))
}

fn prepare_native(
    verb: &str,
    question: &str,
    records: &str,
    reading: &str,
    settings: &str,
    reference: Option<&thinkthen::QuestionFileReference>,
) -> Result<PreparedNative, Error> {
    let call =
        thinkthen::Settings::parse(settings).map_err(|e| complete_native::usage(&e.to_string()))?;
    // SAFETY: the host retains the selected question through synchronous return.
    let saved = reference
        .map(|held| complete_native::settings::parse_file(verb, question, held))
        .transpose()?;
    let prepared = if let Some(saved) = saved {
        complete_native::settings::prepare_file(verb, question, &call, saved)?
    } else {
        complete_native::prepare(verb, question, &call)?
    };
    let (request, projection) = header(verb, &prepared, reading)?;
    let raw: Vec<Box<RawValue>> = serde_json::from_str(records)
        .map_err(|_| complete_native::usage("records is an ordered native collection"))?;
    let records = raw
        .into_iter()
        .map(|raw| read_record(raw.get()))
        .collect::<Vec<_>>();
    Ok(PreparedNative {
        call,
        request,
        projection,
        records,
    })
}

fn execute_native(
    engine: &thinkthen::Engine,
    prepared: PreparedNative,
    controls: thinkthen::CallOptions<'_>,
    verb: &str,
    incremental: bool,
) -> Result<Vec<u8>, String> {
    let images = prepared.records.iter().any(|record| {
        record
            .as_ref()
            .is_ok_and(|record| !record.item.images.is_empty())
    });
    let events = std::sync::Mutex::new(Vec::new());
    let observer = |event: thinkthen::RecordObservation<'_>| {
        events
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(complete_native::observations::event(&event));
    };
    let rows = prepared
        .records
        .into_iter()
        .map(|record| compose(record?, &prepared.projection));
    let mut feed = RequestFeed::from_records("duckdb", rows);
    if !incremental {
        feed = feed.eager();
    }
    if images {
        feed = feed.with_image_inputs();
    }
    if verb == "filter" {
        feed = feed.with_all_filter_results();
    }
    let result = engine.execute_request(
        &prepared.request,
        RequestEnvironment {
            controls: controls
                .surface(thinkthen::Surface::Duckdb)
                .observe(&observer),
            feed: Some(feed),
        },
    );
    let observations = events
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "thinkthen defect: native observations could not be projected".to_owned())?;
    let value = match result {
        Ok(result) => render(result, Value::Array(observations)),
        Err(error) => Ok(complete_native::carrier(
            complete_native::failure(&error),
            Value::Array(observations),
        )),
    };
    Ok(value
        .map_err(|_| "thinkthen defect: native receipt could not be projected".to_owned())?
        .to_string()
        .into_bytes())
}

/// Execute a native SQL collection through the canonical admitted request.
/// # Safety
/// Counted host ranges and the retained question selection remain live through return.
#[allow(
    clippy::too_many_arguments,
    reason = "the private counted SQL ABI retains ranges and stop ownership"
)]
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_native_records(
    verb: BridgeText,
    question: BridgeText,
    records: BridgeText,
    reading: BridgeText,
    settings: BridgeText,
    reference: *const thinkthen::QuestionFileReference,
    incremental: bool,
    deadline: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let verb = text(verb.bytes, verb.len)?.to_owned();
        let question = text(question.bytes, question.len)?;
        // SAFETY: the host retains the selected question through synchronous return.
        let selected = unsafe { reference.as_ref() };
        let parsed = prepare_native(
            &verb,
            question,
            text(records.bytes, records.len)?,
            text(reading.bytes, reading.len)?,
            text(settings.bytes, settings.len)?,
            selected,
        );
        let prepared = match parsed {
            Ok(value) => value,
            Err(error) => return Ok(complete_native::admission(&error).to_string().into_bytes()),
        };
        let held = asked(&session)?;
        let engine = engines::engine_for(&held, |path| probe(&session, path))?;
        let batch = super::portable::batch_word(&prepared.call).or(batch(&session)?);
        let due = super::portable::due(deadline, prepared.call.deadline_ms());
        let context = prepared.call.context().map(str::to_owned);
        run_detached(stop, move |token| {
            let controls = engines::options_for(
                due,
                &token,
                held.max_requests_total,
                batch.as_deref(),
                context.as_deref(),
            )?;
            execute_native(&engine, prepared, controls, &verb, incremental)
        })
    })
}
