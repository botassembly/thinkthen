//! The staged recognize command over the shared splitter and relation planner.

use std::io::{Read, Write};
use std::process::ExitCode;

use serde::Serialize;

use crate::args::{Common, RecognizeArguments};
use crate::asking::{self, Folders};
use crate::cli::intake::{Data, Intake, Item};
use crate::core::{
    Meta, ModelName, Outcome, Reading, RecognizeSpec, Record, RecordValue, RequestMeta, json_line,
    recognize_sha256,
};
use crate::edge::{self, Environment};
use crate::engine::facade::{Engine, MAX_TEXT_BYTES, Probabilities, Recognized};
use crate::failure::{Failure, ReplayContext};
use crate::profile;
use crate::schedule;

mod config;
mod dry_run;
mod source;

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
    arguments.common.check_plan_name()?;
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
    let framing = if arguments.common.input.len() > 1
        && arguments.common.unit.is_none()
        && arguments.common.framing() == crate::core::Framing::Document
    {
        crate::core::Framing::Lines
    } else {
        arguments.common.framing()
    };
    let reading = Reading::new(framing, pointers)?;
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
            },
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
    let streams = reading.streams();
    if !streams && !arguments.common.located() && arguments.common.input.len() <= 1 {
        let item = source
            .into_iter()
            .next()
            .transpose()
            .map_err(|placed| placed.cause)?
            .ok_or(Failure::Defect("document recognize has no input"))?;
        let judged = judged_item(&running, &reading, &spec, &item)?;
        schedule::Output::streaming(&mut writer, environment.usage()).take(judged)?;
        return Ok(ExitCode::SUCCESS);
    }
    schedule::over_records(
        &running.engine,
        &|item: &Item| judged_item(&running, &reading, &spec, item),
        source.map(|item| item.map(|item| (item.at, item))),
        environment.cancel(),
        &mut schedule::Output::streaming(&mut writer, environment.usage()),
    )
}

fn judged_item(
    running: &Running<'_>,
    reading: &Reading,
    spec: &RecognizeSpec,
    item: &Item,
) -> Result<schedule::Judged, Failure> {
    let streams = reading.streams();
    let record = match &item.data {
        Data::Bytes(bytes) => reading
            .record(bytes)
            .map_err(|error| Failure::record(error, streams))?,
        Data::Record(record) => record.clone(),
    };
    let mut judged = judged_record(running, reading, spec, record, streams)?;
    if let Some(position) = item
        .position
        .as_ref()
        .filter(|p| p.located || running.common.input.len() > 1)
    {
        let mut position = position.clone();
        position.located = true;
        let Data::Bytes(bytes) = &item.data else {
            return Err(Failure::Usage("located recognize needs text units"));
        };
        source::locate(
            &mut judged.printed,
            &position,
            reading.as_it_arrived(bytes)?,
            streams,
            running.common.details,
        )?;
        judged.position = Some(position);
    }
    Ok(judged)
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
    let recognition = running
        .engine
        .recognize(
            spec,
            &text,
            running.max_text_bytes,
            running.environment.cancel(),
        )
        .map_err(|error| Failure::from(error).with_replay_context(ReplayContext::Recognize))?;
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
        model: None,
        printed: Some(line),
        position: None,
        outcome: Outcome::Yes,
        replayed: !aggregate.live,
        order_value: None,
        partial_failure: false,
        profile_mismatch: running.mismatch.notice(),
    })
}
