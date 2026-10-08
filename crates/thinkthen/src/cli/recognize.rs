//! The staged recognize command over the shared splitter and relation planner.

use std::io::{Read, Write};
use std::process::ExitCode;

#[cfg(test)]
use serde::Serialize;

use crate::args::{Common, RecognizeArguments};
use crate::asking::{self, Folders};
use crate::cli::intake::{Data, Intake, Item};
#[cfg(test)]
use crate::core::Meta;
use crate::core::{ModelName, Outcome, Reading, RecognizeSpec, Record, RecordValue, json_line};
use crate::edge::{self, Environment};
use crate::engine::facade::{Engine, MAX_TEXT_BYTES};
#[cfg(test)]
use crate::engine::facade::{Probabilities, Recognized};
use crate::failure::{Failure, ReplayContext};
use crate::profile;
use crate::schedule;

mod config;
mod dry_run;
mod source;

#[cfg(test)]
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename = "recognizeDetails"))]
pub(crate) struct Detailed<'a> {
    schema: &'static str,
    value: &'a Recognized,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Record>,
    question: &'a RecognizeSpec,
    answer: &'a Probabilities,
    meta: Meta,
}

struct Running<'a> {
    common: &'a Common,
    max_text_bytes: usize,
    engine: Engine,
    mismatch: profile::Mismatch,
    context: Option<String>,
    context_field: Option<String>,
    cancel: crate::engine::Cancel<'static>,
}

#[expect(
    clippy::too_many_lines,
    reason = "the command edge keeps validation and mode selection in their observable order"
)]
pub(crate) fn run(
    arguments: &RecognizeArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    arguments.common.check_plan_name()?;
    let context = asking::context::shared(arguments.context.as_deref())?;
    let mut spec = config::settle(arguments)?;
    crate::public::RecordReading::new(&[], arguments.context_field.as_deref(), None)
        .map_err(|_| Failure::Usage("--context-field needs a valid JSON Pointer"))?;
    let max_text_bytes = arguments.max_text_bytes.unwrap_or(MAX_TEXT_BYTES);
    let pointers = if arguments.common.field.is_empty() {
        spec.on.clone()
    } else {
        let fields = arguments
            .common
            .field
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        crate::core::pointers(&fields, crate::core::Source::CommandLine, "field")?
    };
    if let Some(model) = arguments.common.model.as_deref() {
        spec.model = Some(edge::model_flag(model)?);
    }
    let framing = if arguments.common.input.len() > 1
        && arguments.common.unit.is_none()
        && arguments.common.framing() == crate::core::Framing::Document
    {
        crate::core::Framing::Lines
    } else {
        arguments.common.framing()
    };
    let reading =
        Reading::new(framing, pointers)?.with_item_schema(spec.metadata.item_schema.clone());
    schedule::jobs_of(arguments.common.jobs, reading.streams())?;
    if (arguments.common.located() || arguments.common.input.len() > 1)
        && (!spec.on.is_empty()
            || arguments.common.jsonl
            || arguments.common.csv
            || arguments.common.tsv
            || !arguments.common.field.is_empty())
    {
        return Err(Failure::Usage(
            "located recognize needs text units without JSON fields or table framing",
        ));
    }
    let source = Intake::new(&arguments.common, &reading, input, !spec.on.is_empty())?;
    let request_size = environment.request_size(arguments.max_request_bytes.as_deref())?;
    let backend = environment
        .resolve(
            arguments.common.backend.as_deref(),
            arguments.common.url.as_deref(),
            spec.model.as_ref().map(ModelName::as_str),
        )?
        .with_request_size(request_size);
    environment.warn_request_size(&backend)?;
    let selected_profile = profile::read(&arguments.common, environment, &backend)?;
    let mismatch = profile::Mismatch::new(spec.profile.as_ref(), selected_profile.as_ref());

    if arguments.common.dry_run {
        let folders = Folders::of(&arguments.common, environment)?;
        if folders.named() {
            return Err(Failure::DryRunWithRecording);
        }
        return dry_run::run(
            &reading,
            source,
            &backend,
            selected_profile.as_ref(),
            dry_run::Question {
                spec: &spec,
                from_file: arguments
                    .kinds
                    .first()
                    .is_some_and(|kind| kind.starts_with('@')),
                limit: max_text_bytes,
                key_env: environment.key_variable(),
                backend_name: environment.named(),
                context: context.as_deref(),
                context_field: arguments.context_field.as_deref(),
            },
            &mut writer,
        );
    }

    let source: Box<dyn Iterator<Item = Result<Item, schedule::Placed>> + Send> =
        if arguments.context_field.is_some() {
            let mut held = Vec::new();
            for item in source {
                let item = item.map_err(|placed| placed.cause)?;
                let record = match &item.data {
                    Data::Bytes(bytes) => reading
                        .record(bytes)
                        .map_err(|error| Failure::record(error, reading.streams()))?,
                    Data::Record(record) => record.clone(),
                    Data::Images(_) => {
                        return Err(Failure::Usage(
                            "recognize accepts text only; images are unsupported",
                        ));
                    }
                };
                selected_context(
                    &record,
                    arguments.context_field.as_deref(),
                    &spec,
                    context.as_deref(),
                )?;
                held.push(Ok(item));
            }
            Box::new(held.into_iter())
        } else {
            Box::new(source)
        };
    let running = Running {
        common: &arguments.common,
        max_text_bytes,
        engine: asking::engine(
            &arguments.common,
            environment,
            Folders::of(&arguments.common, environment)?,
            backend,
            selected_profile,
            arguments.common.jobs,
        )?
        .with_aggregate_context(context.clone()),
        mismatch,
        context,
        context_field: arguments.context_field.clone(),
        cancel: environment.cancel().with_storage_scope(),
    };
    let streams = reading.streams();
    if !streams && !arguments.common.located() && arguments.common.input.len() <= 1 {
        let item = source
            .into_iter()
            .next()
            .transpose()
            .map_err(|placed| placed.cause)?
            .ok_or(Failure::Defect("document recognize has no input"))?;
        let judged = judged_item(&running, &reading, &spec, &item, 0)?;
        schedule::Output::streaming(&mut writer, environment.usage()).take(judged)?;
        return Ok(ExitCode::SUCCESS);
    }
    schedule::over_records(
        &running.engine,
        &|(ordinal, item): &(usize, Item)| judged_item(&running, &reading, &spec, item, *ordinal),
        source
            .enumerate()
            .map(|(ordinal, item)| item.map(|item| (item.at, (ordinal, item)))),
        &running.cancel,
        &mut schedule::Output::streaming(&mut writer, environment.usage()),
    )
}

fn judged_item(
    running: &Running<'_>,
    reading: &Reading,
    spec: &RecognizeSpec,
    item: &Item,
    ordinal: usize,
) -> Result<schedule::Judged, Failure> {
    let streams = reading.streams();
    let record = match &item.data {
        Data::Bytes(bytes) => reading
            .record(bytes)
            .map_err(|error| Failure::record(error, streams))?,
        Data::Record(record) => record.clone(),
        Data::Images(_) => {
            return Err(Failure::Usage(
                "recognize accepts text only; images are unsupported",
            ));
        }
    };
    let location = item
        .position
        .as_ref()
        .filter(|p| p.located || running.common.input.len() > 1)
        .map(|position| {
            let Data::Bytes(bytes) = &item.data else {
                return Err(Failure::Usage("located recognize needs text units"));
            };
            let mut position = position.clone();
            position.located = true;
            Ok((position, reading.as_it_arrived(bytes)?.to_owned()))
        })
        .transpose()?;
    let mut judged = judged_record(
        running,
        reading,
        spec,
        record,
        Render {
            streams,
            ordinal,
            location: location
                .as_ref()
                .map(|(position, text)| (position, text.as_str())),
        },
    )?;
    if let Some((position, _)) = location {
        judged.position = Some(position);
    }
    Ok(judged)
}

fn execute(
    running: &Running<'_>,
    spec: &RecognizeSpec,
    text: &str,
    engine: &Engine,
) -> Result<
    (
        crate::engine::facade::Recognition,
        Vec<crate::core::AttemptObservation>,
    ),
    Failure,
> {
    let cancel = running
        .cancel
        .with_captured_attempts(running.common.details);
    let mut events = std::collections::BTreeMap::new();
    let recognition = engine
        .recognize_observed(
            spec,
            text,
            running.max_text_bytes,
            &cancel,
            |_, _, answered| {
                for event in &answered.attempts {
                    events.insert(event.ordinal(), event.clone());
                }
                Ok(())
            },
        )
        .map_err(|error| Failure::from(error).with_replay_context(ReplayContext::Recognize))?;
    Ok((recognition, events.into_values().collect()))
}

struct Render<'a> {
    streams: bool,
    ordinal: usize,
    location: Option<(&'a crate::cli::intake::Position, &'a str)>,
}

fn judged_record(
    running: &Running<'_>,
    reading: &Reading,
    spec: &RecognizeSpec,
    record: Record,
    row: Render<'_>,
) -> Result<schedule::Judged, Failure> {
    let evidence = reading.evidence(&record)?;
    let text = evidence.as_text()?.into_owned();
    let context = selected_context(
        &record,
        running.context_field.as_deref(),
        spec,
        running.context.as_deref(),
    )?;
    let engine = running
        .engine
        .clone()
        .with_aggregate_context_value(context.clone());
    let (recognition, events) = execute(running, spec, &text, &engine)?;
    let replayed = !recognition.meta.live;
    let line = if running.common.details {
        let canonical = crate::result_json::complete::recognition(
            &engine,
            spec,
            recognition,
            crate::result_json::complete::RecognitionRow {
                ordinal: row.ordinal,
                input: row.streams.then_some(record),
                context_sha256: context
                    .as_ref()
                    .map(|value| {
                        value
                            .as_text()
                            .map(|text| asking::context::digest(Some(&text)))
                    })
                    .transpose()?
                    .flatten(),
                attempts: Some(events),
            },
        )
        .map_err(|_| Failure::Defect("a complete recognition could not be constructed"))?;
        match row.location {
            Some((position, text)) => json_line(&source::Complete {
                canonical: &canonical,
                value: source::located(&canonical.value, position, text)?,
            })?,
            None => json_line(&canonical)?,
        }
    } else {
        match row.location {
            Some((position, text)) => {
                let value = source::located(&recognition.value, position, text)?;
                if row.streams {
                    json_line(&RecordValue::new(record, value))?
                } else {
                    crate::cli::intake::source_value(&text, &json_line(&value)?, position)?
                }
            }
            None if row.streams => json_line(&RecordValue::new(record, recognition.value))?,
            None => json_line(&recognition.value)?,
        }
    };
    let mut printed = Some(line);
    if let Some((position, _)) = row.location
        && (row.streams || running.common.details)
    {
        crate::cli::intake::source_members(&mut printed, Some(position))?;
    }
    Ok(schedule::Judged {
        model: None,
        printed,
        position: None,
        outcome: Outcome::Yes,
        replayed,
        order_value: None,
        rank: None,
        partial_failure: false,
        profile_mismatch: running.mismatch.notice(),
    })
}

fn selected_context(
    record: &Record,
    pointer: Option<&str>,
    spec: &RecognizeSpec,
    fallback: Option<&str>,
) -> Result<Option<crate::core::Evidence>, Failure> {
    let selected = asking::context::record(record, pointer, spec.metadata.context_schema.as_ref())?;
    crate::public::RecordContext::resolved(selected.as_ref(), fallback)
        .map_err(|_| Failure::Usage("the per-item context does not match context_schema"))
}
