use std::io::{Read, Write};
use std::process::ExitCode;

use crate::args::RelateArguments;
use crate::asking::{self, Folders};
use crate::edge::Environment;
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
    let request_size = environment.request_size(arguments.max_request_bytes.as_deref())?;
    let backend = environment.resolve(
        arguments.common.backend.as_deref(),
        arguments.common.url.as_deref(),
        settled.model.as_deref(),
    )?;
    environment.set_request_size(request_size)?;
    environment.warn_request_size(&backend, request_size)?;
    profile::read(&arguments.common, environment)?;
    let (sources, originals) =
        source::selection(&arguments.common, input, settled.framing, &settled.fields)?;
    if originals.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    let folders = Folders::of(&arguments.common, environment)?;
    if arguments.common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    let engine = crate::cli::construction::engine(
        &arguments.common,
        environment,
        folders,
        arguments.common.jobs,
    )?;
    let mut arguments_native = admitted.request().call.arguments().clone();
    arguments_native.question = crate::RequestQuestion::Definition {
        value: settled.definition.clone(),
    };
    arguments_native.input = crate::RequestInput::Feed {
        name: "cli-relate".to_owned(),
        framing: crate::RequestFraming::Document,
        reading: crate::ReaderOptions::default(),
        images: Vec::new(),
    };
    arguments_native.options.field = None;
    arguments_native.options.attempts = arguments.common.details;
    arguments_native.options.context = shared_context.clone();
    arguments_native.options.threshold = settled.threshold;
    let request = crate::Request::for_function(crate::RequestFunction::Relate, arguments_native);
    if arguments.common.dry_run {
        dry_run::write(
            &mut writer,
            dry_run::Context {
                backend: &backend,
                profile: engine.profile_name(),
                framing: settled.framing,
                fields: &settled.fields,
                from: settled.from,
                key_env: &backend.key_variable,
                shared_context: shared_context.as_deref(),
            },
            request.admit()?,
            (&originals, sources.as_ref()),
            &engine,
        )?;
        return Ok(ExitCode::SUCCESS);
    }
    let mismatch = profile::Mismatch::new(settled.profile.as_deref(), engine.profile_name());
    let result = execute(
        request,
        (&originals, sources.as_ref()),
        &engine,
        environment,
    )?;
    let partial = result.members().any(|member| member.failure_id().is_some());
    if let Some(sources) = &sources {
        source::write(&mut writer, arguments.common.details, &result, sources)?;
    } else {
        result::write(&mut writer, arguments.common.details, &result, &originals)?;
    }
    mismatch.print_once()?;
    Ok(if partial {
        ExitCode::from(6)
    } else {
        ExitCode::SUCCESS
    })
}

fn descriptor(
    original: &crate::RawRecord,
    location: Option<crate::SourceLocation>,
) -> Result<crate::RequestSessionDescriptor, crate::Error> {
    let original = Some(match original.literal() {
        Some(text) => crate::RequestOriginal::Text {
            text: text.to_owned(),
        },
        None => crate::RequestOriginal::Json {
            value: original.clone(),
        },
    });
    Ok(crate::RequestSessionDescriptor {
        item: crate::RequestItem {
            original,
            images: Vec::new(),
            context: None,
            options: None,
            examples: None,
            seed_spans: None,
        },
        location,
    })
}

fn execute(
    request: crate::Request,
    (originals, sources): (&[crate::RawRecord], Option<&source::Sources>),
    engine: &crate::Engine,
    environment: &Environment,
) -> Result<crate::CompleteRelated, Failure> {
    let tally = environment.tally().start();
    let session = engine.request_session_with_feed_options(
        request,
        crate::Surface::Cli,
        crate::RequestSessionFeedOptions {
            retain_success_on_cancel: true,
            eager: true,
            ..Default::default()
        },
    )?;
    let mut descriptors = originals.iter().enumerate().map(|(ordinal, original)| {
        descriptor(
            original,
            sources
                .map(|sources| sources.location(ordinal))
                .transpose()?,
        )
    });
    let mut input_done = false;
    let mut cancelled = false;
    let mut tally = Some(tally);
    let mut terminal = None;
    let mut result = None;
    loop {
        if !cancelled && environment.cancel().fired() {
            session.cancel();
            cancelled = true;
            input_done = true;
        }
        let packet = session.try_read();
        let received = matches!(&packet, crate::RequestSessionRead::Result(_));
        match packet {
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Aggregate(
                crate::RequestValue::Related(row),
            )) => {
                result = Some(row.into_parts().1);
            }
            crate::RequestSessionRead::Result(
                crate::RequestSessionResult::Aggregate(_) | crate::RequestSessionResult::Row(_),
            ) => {
                session.cancel();
                cancelled = true;
                input_done = true;
            }
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Observation {
                ..
            }) => {}
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Terminal(value)) => {
                environment.settle_session(&value, &mut tally)?;
                terminal = Some(value.error.map_or(Ok(()), Err));
            }

            crate::RequestSessionRead::End => break,
            crate::RequestSessionRead::Pending => {}
        }
        if received {
            continue;
        }
        if !input_done {
            match session.try_push_with(|| {
                descriptors
                    .next()
                    .ok_or(FeedRead::End)?
                    .map_err(FeedRead::Failed)
            }) {
                Ok(crate::RequestSessionPushStatus::Accepted) => continue,
                Ok(crate::RequestSessionPushStatus::Full) => {}
                Ok(crate::RequestSessionPushStatus::Closed) => input_done = true,
                Err(FeedRead::End) => {
                    session.finish(None)?;
                    input_done = true;
                }
                Err(FeedRead::Failed(error)) => {
                    session.finish_native_reader_error(error)?;
                    input_done = true;
                }
            }
        }
        let interest = if cancelled {
            crate::RequestSessionWaitInterest::Settlement
        } else if input_done {
            crate::RequestSessionWaitInterest::Result
        } else {
            crate::RequestSessionWaitInterest::Input
        };
        if session.wait(interest, std::time::Duration::from_millis(50))
            == crate::RequestSessionReady::End
        {
            break;
        }
    }
    terminal
        .ok_or(Failure::Defect("native session ended without settlement"))?
        .map_err(|error| {
            Failure::from(error)
                .with_replay_context(ReplayContext::Relate)
                .with_size_advice(crate::failure::SizeAdvice::Batches)
        })?;
    result.ok_or(Failure::Defect("relate returned a different function"))
}

enum FeedRead {
    End,
    Failed(crate::Error),
}
impl From<crate::Error> for FeedRead {
    fn from(error: crate::Error) -> Self {
        Self::Failed(error)
    }
}

fn records<'a>(
    originals: &'a [crate::RawRecord],
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
                && let Some(text) = original.literal()
            {
                return Ok(crate::RecordInput {
                    original: crate::QuestionInput::Text(text.to_owned()),
                    context: None,
                    options: None,
                    examples: None,
                    seed_spans: None,
                });
            }
            let mut row = composition.compose(original.clone())?;
            if let Some(sources) = sources {
                row.original = row.original.with_location(sources.location(ordinal)?);
            }
            Ok(row.map_original(crate::QuestionInput::Record))
        }))
}
