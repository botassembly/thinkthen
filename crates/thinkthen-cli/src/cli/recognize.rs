//! The staged recognize command over the shared splitter and relation planner.

use std::io::{Read, Write};
use std::process::ExitCode;

use crate::Engine;
use crate::RequestFraming as Framing;
use crate::args::{Common, RecognizeArguments};
use crate::asking::{self, Folders};
use crate::cli::intake::Intake;
use crate::cli::reading::Reading;
use crate::edge::Environment;
use crate::failure::Failure;
use crate::profile;
use crate::schedule;

mod config;
mod dry_run;
mod examples;
mod native;
mod source;

struct Running<'a> {
    common: &'a Common,
    environment: &'a Environment,
    engine: Engine,
    mismatch: profile::Mismatch,
    context: Option<String>,
    context_field: Option<String>,
    examples_field: Option<String>,
    seed_spans_field: Option<String>,
    recording: bool,
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
        crate::RecordReading::new(&[pointer], None, None)
            .map_err(|_| Failure::Usage("--examples-field needs a valid JSON Pointer"))?;
        if !arguments.common.jsonl && !arguments.common.csv && !arguments.common.tsv {
            return Err(Failure::Usage(
                "--examples-field needs JSON or table records",
            ));
        }
    }
    if let Some(pointer) = arguments.seed_spans_field.as_deref() {
        crate::RecordReading::new(&[pointer], None, None)
            .map_err(|_| Failure::Usage("--seed-spans-field needs a valid JSON Pointer"))?;
        if !arguments.common.jsonl && !arguments.common.csv && !arguments.common.tsv {
            return Err(Failure::Usage(
                "--seed-spans-field needs JSON or table records",
            ));
        }
    }
    let (mut question, admitted) = config::settle(arguments, admitted)?;
    let context = asking::context::shared(arguments.context.as_deref())?;
    examples::shared(arguments.examples.as_deref(), &mut question)?;
    let spec = question.reading();
    crate::RecordReading::new(&[], arguments.context_field.as_deref(), None)
        .map_err(|_| Failure::Usage("--context-field needs a valid JSON Pointer"))?;
    let pointers = if arguments.common.field.is_empty() {
        spec.on().map(str::to_owned).collect::<Vec<_>>()
    } else {
        let fields = arguments
            .common
            .field
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        crate::RecordReading::new(&fields, None, None)?;
        arguments.common.field.clone()
    };
    let has_on = spec.on().next().is_some();
    let mut request_arguments = admitted.request().call.arguments().clone();
    request_arguments.question = crate::RequestQuestion::Definition {
        value: question.clone().into(),
    };
    request_arguments.options.max_text_bytes = arguments.max_text_bytes;
    let admitted =
        crate::Request::for_function(crate::RequestFunction::Recognize, request_arguments)
            .admit()?;
    let framing = if arguments.common.input.len() > 1
        && arguments.common.unit.is_none()
        && matches!(arguments.common.framing(), Framing::Document)
    {
        Framing::Lines
    } else {
        arguments.common.framing()
    };
    let reading = Reading::new(framing, pointers)?;
    schedule::jobs_of(arguments.common.jobs, reading.streams(), environment)?;
    if (arguments.common.located() || arguments.common.input.len() > 1)
        && (has_on
            || arguments.common.jsonl
            || arguments.common.csv
            || arguments.common.tsv
            || !arguments.common.field.is_empty())
    {
        return Err(Failure::Usage(
            "located recognize needs text units without JSON fields or table framing",
        ));
    }
    let source = Intake::new(&arguments.common, &reading, input, has_on)?;
    let request_size = environment.request_size(arguments.max_request_bytes.as_deref())?;
    let backend = environment.resolve(
        arguments.common.backend.as_deref(),
        arguments.common.url.as_deref(),
        arguments.common.model.as_deref().or(spec.model()),
    )?;
    environment.set_request_size(request_size)?;
    environment.warn_request_size(&backend, request_size)?;
    profile::read(&arguments.common, environment)?;
    if arguments.common.dry_run {
        let folders = Folders::of(&arguments.common, environment)?;
        if folders.named() {
            return Err(Failure::DryRunWithRecording);
        }
        return dry_run::run(
            &reading,
            admitted,
            source,
            (&backend, environment),
            dry_run::Question {
                spec: &question,
                from_file: arguments
                    .kinds
                    .first()
                    .is_some_and(|kind| kind.starts_with('@')),
                key_env: &backend.key_variable,
                backend_name: environment.named(),
                context: context.as_deref(),
                context_field: arguments.context_field.as_deref(),
                examples_field: arguments.examples_field.as_deref(),
                seed_spans_field: arguments.seed_spans_field.as_deref(),
            },
            &mut writer,
        );
    }

    let folders = Folders::of(&arguments.common, environment)?;
    let recording = folders.reported();
    let engine = crate::cli::construction::engine(
        &arguments.common,
        environment,
        folders,
        arguments.common.jobs,
    )?;
    let mismatch = profile::Mismatch::new(spec.profile(), engine.profile_name());
    let running = Running {
        common: &arguments.common,
        environment,
        engine,
        mismatch,
        context,
        context_field: arguments.context_field.clone(),
        examples_field: arguments.examples_field.clone(),
        seed_spans_field: arguments.seed_spans_field.clone(),
        recording,
    };
    native::run(
        &running,
        admitted,
        &reading,
        &question,
        source,
        &mut schedule::Output::streaming(&mut writer),
    )
}
