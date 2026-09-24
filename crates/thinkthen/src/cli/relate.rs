use std::io::{Read, Write};
use std::process::ExitCode;
use std::time::Duration;

use crate::args::{Common, RelateArguments};
use crate::asking::{Folders, ask_prepared};
use crate::core::{AnswerOutcome, Backend, ModelName, RelationEntity, assemble_edges};
use crate::edge::Environment;
use crate::failure::Failure;
use crate::http::Client;
use crate::prepared_request::Answered;
use crate::profile;
use crate::recorder::Recorder;

mod config;
mod dry_run;
mod input;
mod plan;
mod result;

/// The boundaries every relation request passes through.
struct Asking<'a> {
    backend: &'a Backend,
    common: &'a Common,
    environment: &'a Environment,
    recorder: Recorder,
    client: Client,
}

pub(crate) fn run(
    arguments: &RelateArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let settled = config::settle(arguments)?;
    let configured = settled.spec.model.as_ref().map(ModelName::as_str);
    let backend = Backend::resolve(
        arguments.common.url.as_deref(),
        environment.base_url(),
        configured
            .or_else(|| environment.model())
            .unwrap_or(crate::core::DEFAULT_MODEL),
    )?;
    let selected_profile = profile::read(&arguments.common)?;
    let source = crate::edge::source(arguments.common.input.as_deref(), input)?;
    let entities = input::read(source, settled.framing, &settled.spec)?;
    if entities.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    let prepared = plan::prepare(
        &entities,
        &settled.spec,
        &backend,
        selected_profile.as_ref(),
    )?;
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
            entity_count: entities.len(),
        };
        dry_run::write(&mut writer, context, &prepared)?;
        return Ok(ExitCode::SUCCESS);
    }
    let asking = Asking {
        backend: &backend,
        common: &arguments.common,
        environment,
        recorder: Recorder::of_private(
            folders.record.as_deref(),
            folders.replay.as_deref(),
            folders.private_default,
            folders.cache_answers,
        )?,
        client: Client::new(
            Duration::from_secs(arguments.common.timeout),
            backend.is_secure(),
        ),
    };
    let threshold = settled.spec.threshold.cut_value().unwrap_or(0.5);
    let execution = execute(&asking, prepared, &entities, threshold)?;
    if execution.answered == 0 && execution.failed > 0 {
        return Err(Failure::Relate(crate::failure::relate::Error::Logical));
    }
    let partial = execution.failed > 0;
    let mismatch = profile::Mismatch::new(settled.spec.profile.as_ref(), selected_profile.as_ref());
    let output = result::Output {
        details: arguments.common.details,
        framing: settled.framing,
        spec: &settled.spec,
        entities: &entities,
        backend: &backend,
        warning: mismatch.warning(),
    };
    result::write(&mut writer, &output, &execution)?;
    mismatch.print_once()?;
    Ok(if partial {
        ExitCode::from(6)
    } else {
        ExitCode::SUCCESS
    })
}

fn execute(
    asking: &Asking<'_>,
    prepared: Vec<plan::PreparedRelation>,
    entities: &[RelationEntity],
    threshold: f64,
) -> Result<result::Execution, Failure> {
    let mut execution = result::Execution {
        replayed: true,
        ..result::Execution::default()
    };
    for relation in prepared {
        let mut mappings = relation.mappings.into_iter();
        for chunk in relation.chunks {
            let answered = ask_prepared(
                asking.backend,
                &chunk.plan,
                chunk.request,
                asking.common,
                asking.environment,
                &asking.recorder,
                &asking.client,
            )?;
            add_meta(&mut execution, &answered)?;
            for outcome in answered.reply.outcomes() {
                let mapping = mappings
                    .next()
                    .ok_or(Failure::Defect("a relation reply exceeds its question map"))?;
                if let AnswerOutcome::Answered(answer) = outcome {
                    execution.answered += 1;
                    execution.edges.extend(assemble_edges(
                        entities,
                        &relation.relation,
                        std::slice::from_ref(&mapping),
                        std::slice::from_ref(answer),
                        threshold,
                    ));
                } else {
                    execution.failed += 1;
                }
                execution.logical.push(result::Logical {
                    relation: relation.relation.clone(),
                    mapping,
                    outcome: outcome.clone(),
                    request: answered.request.as_str().to_owned(),
                });
            }
        }
        if mappings.next().is_some() {
            return Err(Failure::Defect(
                "a relation reply did not cover its question map",
            ));
        }
    }
    Ok(execution)
}

fn add_meta(execution: &mut result::Execution, answered: &Answered) -> Result<(), Failure> {
    let model = answered.reply.model();
    if execution.model.as_ref().is_some_and(|held| held != model) {
        return Err(Failure::ModelsDiffer(None));
    }
    execution.model.get_or_insert_with(|| model.clone());
    execution.usage = match (execution.usage, answered.reply.usage()) {
        (Some(left), Some(right)) => Some(left.checked_plus(right).ok_or(Failure::UsageOverflow)?),
        (None, held) | (held, None) => held,
    };
    execution.replayed &= answered.replayed;
    execution.requests_sent = execution
        .requests_sent
        .checked_add(answered.requests_sent)
        .ok_or(Failure::UsageOverflow)?;
    execution
        .requests
        .push(answered.request.as_str().to_owned());
    Ok(())
}
