use std::io::{Read, Write};
use std::process::ExitCode;

use crate::args::RelateArguments;
use crate::asking::{self, Folders};
use crate::core::ModelName;
use crate::edge::Environment;
use crate::engine::facade;
use crate::failure::{Failure, ReplayContext};
use crate::profile;

mod config;
mod dry_run;
mod input;
pub(crate) mod result;
pub(crate) mod source;

pub(crate) fn run(
    arguments: &RelateArguments,
    environment: &Environment,
    mut admitted: crate::AdmittedRequest,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    arguments.common.check_plan_name()?;
    let shared_context = asking::context::shared(arguments.context.as_deref())?;
    let settled = config::settle(arguments, &mut admitted)?;
    *admitted.cli_definition().map_err(Failure::from)? = crate::Relate(settled.spec.clone()).into();
    let configured = settled.spec.model.as_ref().map(ModelName::as_str);
    let request_size = environment.request_size(arguments.max_request_bytes.as_deref())?;
    let backend = environment
        .resolve(
            arguments.common.backend.as_deref(),
            arguments.common.url.as_deref(),
            configured,
        )?
        .with_request_size(request_size);
    environment.warn_request_size(&backend)?;
    let selected_profile = profile::read(&arguments.common, environment, &backend)?;
    let (sources, originals) =
        source::selection(&arguments.common, input, settled.framing, &settled.spec)?;
    if originals.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    let folders = Folders::of(&arguments.common, environment)?;
    if arguments.common.dry_run {
        if folders.named() {
            return Err(Failure::DryRunWithRecording);
        }
        let context = dry_run::Context {
            backend: &backend,
            profile: selected_profile.as_ref(),
            framing: settled.framing,
            spec: &settled.spec,
            from: settled.from,
            key_env: environment.key_variable(),
            shared_context: shared_context.as_deref(),
        };
        dry_run::write(
            &mut writer,
            context,
            admitted,
            (&originals, sources.as_ref()),
        )?;
        return Ok(ExitCode::SUCCESS);
    }
    let mismatch = profile::Mismatch::new(settled.spec.profile.as_ref(), selected_profile.as_ref());
    let engine = asking::engine(
        &arguments.common,
        environment,
        folders,
        backend.clone(),
        selected_profile,
        arguments.common.jobs,
    )?
    .with_aggregate_context(shared_context.clone());
    let result = execute(
        admitted,
        shared_context.as_deref(),
        (&originals, sources.as_ref()),
        &engine,
        environment,
        arguments.common.details,
    )?;
    let canonical = &result.canonical;
    let partial = canonical
        .members
        .iter()
        .any(|member| matches!(member.identity, crate::core::MemberIdentity::Failed(_)));
    if let Some(sources) = &sources {
        source::write(&mut writer, arguments.common.details, &result, sources)?;
    } else {
        result::write(&mut writer, arguments.common.details, canonical, &originals)?;
    }
    mismatch.print_once()?;
    Ok(if partial {
        ExitCode::from(6)
    } else {
        ExitCode::SUCCESS
    })
}

fn execute(
    admitted: crate::AdmittedRequest,
    context: Option<&str>,
    (originals, sources): (&[crate::core::Record], Option<&source::Sources>),
    engine: &facade::Engine,
    environment: &Environment,
    details: bool,
) -> Result<crate::CompleteRelated, Failure> {
    let records = records(originals, sources)?;
    let request = admitted.with_composed_feed("cli-relate");
    let native = crate::Engine::from_cli(engine.clone(), environment.config().prices());
    let token = crate::CancelToken::from_flag(environment.cancel().flag());
    let signal = || environment.cancel().fired();
    let mut controls = crate::CallOptions::new()
        .cli_cancel(&token, environment.cancel().deadline())
        .interrupt(&signal)
        .surface(crate::Surface::Cli)
        .attempts(details);
    if let Some(context) = context {
        controls = controls.context(context);
    }
    let failure = |error: crate::Error| {
        if let Some(facts) = error.facts() {
            environment.settle_native(facts);
        }
        Failure::from(error).with_replay_context(ReplayContext::Relate)
    };
    let outcome = native
        .execute_request(
            &request,
            crate::RequestEnvironment {
                controls,
                feed: Some(crate::RequestFeed::from_records("cli-relate", records).eager()),
            },
        )
        .map_err(failure)?;
    let call = match outcome {
        crate::RequestOutcome::Complete(call) => call,
        crate::RequestOutcome::Failed { error, .. } => {
            return Err(failure(error));
        }
    };
    environment.settle_native(call.facts());
    let crate::RequestValue::Related(row) = call.into_value() else {
        return Err(Failure::Defect("relate returned a different function"));
    };
    Ok(row.result)
}

fn records<'a>(
    originals: &'a [crate::core::Record],
    sources: Option<&'a source::Sources>,
) -> Result<
    impl Iterator<Item = Result<crate::RecordInput<crate::QuestionInput>, crate::Error>> + 'a,
    Failure,
> {
    let composition = crate::RecordReading::new(&[], None, None).map_err(Failure::from)?;
    Ok(originals
        .iter()
        .enumerate()
        .map(move |(ordinal, original)| {
            if sources.is_none()
                && let Some(text) = original.text()
            {
                return Ok(crate::RecordInput {
                    original: crate::QuestionInput::Text(text.to_owned()),
                    context: None,
                    options: None,
                    examples: None,
                    seed_spans: None,
                });
            }
            let mut row =
                composition.compose(crate::RawRecord(std::sync::Arc::new(original.clone())))?;
            if let Some(sources) = sources {
                row.original = row.original.with_location(sources.location(ordinal)?);
            }
            Ok(row.map_original(crate::QuestionInput::Record))
        }))
}
