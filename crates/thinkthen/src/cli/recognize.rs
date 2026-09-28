//! The staged recognize command over the shared splitter and relation planner.

use std::io::{Read, Write};
use std::process::ExitCode;

use serde::Serialize;

use crate::args::{Common, RecognizeArguments};
use crate::asking::{self, Folders};
use crate::core::{
    Backend, Meta, ModelName, Outcome, Reading, RecognizeSpec, Record, RecordValue, RequestMeta,
    json_line, recognize_sha256,
};
use crate::edge::{self, Environment};
use crate::engine::facade::{Engine, MAX_TEXT_BYTES, Probabilities, Recognized};
use crate::failure::Failure;
use crate::profile;
use crate::schedule;
use crate::table::Rows as TableRows;

mod config;
mod dry_run;

#[derive(Debug, Serialize)]
struct Detailed<'a> {
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
    environment: &'a Environment,
    engine: Engine,
    mismatch: profile::Mismatch,
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
    let mut spec = config::settle(arguments)?;
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
    let reading = Reading::new(arguments.common.framing(), pointers)?;
    schedule::jobs_of(arguments.common.jobs, reading.streams())?;
    let source = edge::source(arguments.common.input.as_deref(), input)?;
    let configured = spec
        .model
        .as_ref()
        .map(ModelName::as_str)
        .or_else(|| environment.model());
    let request_size = environment.request_size(arguments.max_request_bytes.as_deref())?;
    let backend = Backend::resolve(
        arguments.common.url.as_deref(),
        environment.base_url(),
        configured.unwrap_or(crate::core::DEFAULT_MODEL),
    )?
    .with_request_size(request_size);
    environment.check_key(&backend)?;
    environment.warn_request_size(&backend)?;
    let selected_profile = profile::read(&arguments.common)?;
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
            (
                &spec,
                arguments
                    .kinds
                    .first()
                    .is_some_and(|kind| kind.starts_with('@')),
                max_text_bytes,
            ),
            &mut writer,
        );
    }

    let running = Running {
        common: &arguments.common,
        max_text_bytes,
        environment,
        engine: asking::engine(
            &arguments.common,
            environment,
            Folders::of(&arguments.common, environment)?,
            backend,
            selected_profile,
            arguments.common.jobs,
        )?,
        mismatch,
    };
    if let Some(kind) = config::table_kind(&arguments.common) {
        let rows = TableRows::new(source, kind)?;
        return schedule::over_records(
            &running.engine,
            &|record: &Record| judged_record(&running, &reading, &spec, record.clone(), true),
            rows.enumerate().map(|(place, row)| {
                row.map(|record| (place + 1, record))
                    .map_err(|error| schedule::Placed::at(error, place + 1))
            }),
            environment.cancel(),
            &mut schedule::Output::streaming(&mut writer, environment.usage()),
        );
    }
    let streams = reading.streams();
    let mut chunks = edge::numbered(edge::Chunks::new(source, streams), &reading);
    if !streams {
        let bytes = chunks
            .next()
            .map(|(_, row)| row)
            .transpose()?
            .unwrap_or_default();
        let record = reading
            .record(&bytes)
            .map_err(|error| Failure::record(error, streams))?;
        let judged = judged_record(&running, &reading, &spec, record, streams)?;
        schedule::Output::streaming(&mut writer, environment.usage()).take(judged)?;
        return Ok(ExitCode::SUCCESS);
    }
    schedule::over_records(
        &running.engine,
        &|bytes: &Vec<u8>| {
            let record = reading
                .record(bytes)
                .map_err(|error| Failure::record(error, streams))?;
            judged_record(&running, &reading, &spec, record, streams)
        },
        chunks.map(|(at, row)| {
            row.map(|bytes| (at, bytes))
                .map_err(|error| schedule::Placed::at(error, at))
        }),
        environment.cancel(),
        &mut schedule::Output::streaming(&mut writer, environment.usage()),
    )
}

fn judged_record(
    running: &Running<'_>,
    reading: &Reading,
    spec: &RecognizeSpec,
    record: Record,
    streams: bool,
) -> Result<schedule::Judged, Failure> {
    let evidence = reading.evidence(&record)?;
    let text = evidence.as_text()?.into_owned();
    let recognition = running.engine.recognize(
        spec,
        &text,
        running.max_text_bytes,
        running.environment.cancel(),
    )?;
    let (value, details, aggregate) = (recognition.value, recognition.details, recognition.meta);
    let line = if running.common.details {
        let model = aggregate
            .model
            .unwrap_or_else(|| running.engine.backend().model().clone());
        let meta = Meta::new(
            env!("CARGO_PKG_VERSION"),
            recognize_sha256(spec)?,
            running.engine.backend().url().clone(),
            model,
            aggregate.usage,
            RequestMeta::new(!aggregate.live, aggregate.requests_sent, aggregate.requests)
                .with_profile_warning(running.mismatch.warning()),
        );
        json_line(&Detailed {
            schema: crate::core::RESULT_SCHEMA,
            value: &value,
            input: streams.then_some(record),
            question: spec,
            answer: &details,
            meta,
        })?
    } else if streams {
        json_line(&RecordValue::new(record, value))?
    } else {
        json_line(&value)?
    };
    Ok(schedule::Judged {
        printed: Some(line),
        outcome: Outcome::Yes,
        replayed: !aggregate.live,
        probability: None,
        partial_failure: false,
        profile_mismatch: running.mismatch.notice(),
    })
}
