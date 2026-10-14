//! Native SQL record collections retain canonical request admission and receipts.
use super::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, batch, probe, reply_boundary,
    run_detached, text,
};
use crate::{complete_native, engines};
use serde_json::{Value, value::RawValue};
use thinkthen::{Error, Request, RequestArguments, RequestInput, RequestOptions, RequestQuestion};

pub(super) mod observations;
mod records;
use records::{Record, compose, fields, read_record};

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

pub(super) fn decision_value(answer: thinkthen::Answer) -> Value {
    match answer {
        thinkthen::Answer::Yes => Value::Bool(true),
        thinkthen::Answer::No => Value::Bool(false),
        thinkthen::Answer::Unsure => Value::Null,
    }
}

pub(super) fn filter_value<T: serde::Serialize>(
    row: &thinkthen::CompleteRecord<T, thinkthen::CompleteFilter>,
) -> Result<Value, Error> {
    if row.result().value() {
        Ok(Value::Array(vec![
            serde_json::to_value(row.original()).map_err(|_| complete_native::defect())?,
        ]))
    } else {
        Ok(Value::Array(Vec::new()))
    }
}

pub(super) fn plain_values(value: &thinkthen::RequestValue) -> Result<Value, Error> {
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
    Ok(match value {
        RequestValue::Decisions(rows) => Value::Array(
            rows.iter()
                .map(|row| decision_value(row.result().value()))
                .collect(),
        ),
        RequestValue::Choices(rows) => rows!(rows),
        RequestValue::Tags(rows) => rows!(rows),
        RequestValue::Scores(rows) => rows!(rows),
        RequestValue::Filtered(rows) => Value::Array(
            rows.iter()
                .map(filter_value)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        RequestValue::Ranked(rows) => rows!(rows),
        RequestValue::SetRanked(rows) => rows!(rows),
        _ => Value::Null,
    })
}

#[derive(Debug)]
struct PreparedNative {
    call: thinkthen::Settings,
    prepared: complete_native::Prepared,
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
    selection: (Option<&thinkthen::QuestionFileReference>, bool),
) -> Result<PreparedNative, Error> {
    let (reference, ordinary) = selection;
    let call =
        thinkthen::Settings::parse(settings).map_err(|e| complete_native::usage(&e.to_string()))?;
    // SAFETY: the host retains the selected question through synchronous return.
    let saved = reference
        .map(|held| complete_native::settings::parse_file(verb, question, held))
        .transpose()?;
    let prepared = if let Some(saved) = saved {
        complete_native::settings::prepare_file(verb, question, &call, saved)?
    } else if ordinary && verb == "rank" {
        complete_native::Prepared::Rank(thinkthen::Question::rank(question)?)
            .configured(verb, &call)?
    } else if ordinary && verb == "find" && !question.trim_start().starts_with('{') {
        complete_native::Prepared::Find(
            thinkthen::Question::find(question)?,
            thinkthen::RecordReading::new(&[], None, None)?,
        )
        .configured(verb, &call)?
    } else if ordinary && verb == "recognize" && question.trim_start().starts_with('[') {
        let kinds: Vec<String> = serde_json::from_str(question)
            .map_err(|_| complete_native::usage("kinds is a text list"))?;
        let asking = super::nested::ask(8, "", &kinds, false)
            .map_err(|error| complete_native::usage(&error))?;
        complete_native::Prepared::Recognize(
            asking,
            thinkthen::RecordReading::new(&[], None, None)?,
        )
        .configured(verb, &call)?
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
        prepared,
        request,
        projection,
        records,
    })
}

fn execute_native(
    engine: &thinkthen::Engine,
    prepared: PreparedNative,
    controls: thinkthen::CallOptions<'_>,
    _verb: &str,
    incremental: bool,
) -> Result<Vec<u8>, String> {
    let images = prepared.records.iter().any(|row| {
        row.as_ref()
            .is_ok_and(|record| !record.item.images.is_empty())
    });
    let rows = prepared
        .records
        .into_iter()
        .map(|record| compose(record?, &prepared.projection));
    let feed = thinkthen::RequestFeed::from_records("duckdb", rows);
    let feed = if incremental { feed } else { feed.eager() };
    let feed = if images {
        feed.with_image_inputs()
    } else {
        feed
    };
    let value = thinkthen_host::sql_request::run_feed_recovering(
        engine,
        &prepared.prepared,
        &prepared.request,
        feed,
        controls,
        thinkthen::Surface::Duckdb,
    )
    .map_err(|error| crate::errors::RowError::from(error).text)?;
    serde_json::to_vec(&value)
        .map_err(|_| "thinkthen defect: details could not be projected".to_owned())
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
    cancelled: bool,
    ordinary: bool,
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
            (selected, ordinary),
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
            if cancelled {
                token.cancel();
            }
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

/// Normalize the existing ordinary keyed input grammar through its sole parser.
/// # Safety
/// The caller retains the counted range until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_details_keyed_inputs(input: BridgeText) -> Reply {
    reply_boundary(|| {
        let records = super::portable_many::keyed_texts(text(input.bytes, input.len)?)?
            .into_iter()
            .map(|text| serde_json::json!({"text":text}))
            .collect::<Vec<_>>();
        serde_json::to_vec(&records)
            .map_err(|_| "thinkthen defect: keyed inputs could not be encoded".to_owned())
    })
}
