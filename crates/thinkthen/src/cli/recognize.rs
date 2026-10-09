//! The staged recognize command over the shared splitter and relation planner.

use std::io::{Read, Write};
use std::process::ExitCode;

#[cfg(test)]
use serde::Serialize;

use crate::args::{Common, RecognizeArguments};
use crate::asking::{self, Folders};
use crate::cli::intake::Intake;
#[cfg(test)]
use crate::core::Meta;
use crate::core::{ModelName, Reading, RecognizeSpec, Record};
use crate::edge::{self, Environment};
use crate::engine::facade::{Engine, MAX_TEXT_BYTES};
#[cfg(test)]
use crate::engine::facade::{Probabilities, Recognized};
use crate::failure::Failure;
use crate::profile;
use crate::schedule;

mod config;
mod dry_run;
mod examples;
#[cfg(test)]
mod legacy_schema;
mod native;
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
    #[schemars(with = "legacy_schema::RecognitionOdds")]
    answer: &'a Probabilities,
    meta: Meta,
}

struct Running<'a> {
    common: &'a Common,
    environment: &'a Environment,
    max_text_bytes: usize,
    engine: Engine,
    mismatch: profile::Mismatch,
    context: Option<String>,
    context_field: Option<String>,
    examples_field: Option<String>,
    seed_spans_field: Option<String>,
    cancel: crate::engine::Cancel<'static>,
}

/// Prepare inline recognition declarations for shared admission without evidence reads.
pub(crate) fn request_definition(
    arguments: &crate::args::RecognizeArguments,
) -> Result<crate::Recognize, Failure> {
    config::settle(arguments).map(crate::Recognize)
}

#[expect(
    clippy::too_many_lines,
    reason = "the command edge keeps validation and mode selection in their observable order"
)]
pub(crate) fn run(
    arguments: &RecognizeArguments,
    environment: &Environment,
    admitted: crate::AdmittedRequest,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    arguments.common.check_plan_name()?;
    if let Some(pointer) = arguments.examples_field.as_deref() {
        crate::core::Pointer::new(pointer)
            .map_err(|_| Failure::Usage("--examples-field needs a valid JSON Pointer"))?;
        if !arguments.common.jsonl && !arguments.common.csv && !arguments.common.tsv {
            return Err(Failure::Usage(
                "--examples-field needs JSON or table records",
            ));
        }
    }
    if let Some(pointer) = arguments.seed_spans_field.as_deref() {
        crate::core::Pointer::new(pointer)
            .map_err(|_| Failure::Usage("--seed-spans-field needs a valid JSON Pointer"))?;
        if !arguments.common.jsonl && !arguments.common.csv && !arguments.common.tsv {
            return Err(Failure::Usage(
                "--seed-spans-field needs JSON or table records",
            ));
        }
    }
    let mut spec = config::settle(arguments)?;
    let context = asking::context::shared(arguments.context.as_deref())?;
    examples::shared(arguments.examples.as_deref(), &mut spec)?;
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
                examples_field: arguments.examples_field.as_deref(),
                seed_spans_field: arguments.seed_spans_field.as_deref(),
            },
            &mut writer,
        );
    }

    let running = Running {
        common: &arguments.common,
        environment,
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
        examples_field: arguments.examples_field.clone(),
        seed_spans_field: arguments.seed_spans_field.clone(),
        cancel: environment.cancel().with_storage_scope(),
    };
    native::run(
        &running,
        admitted,
        &reading,
        &spec,
        source,
        &mut schedule::Output::streaming(&mut writer, environment.usage()),
    )
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
