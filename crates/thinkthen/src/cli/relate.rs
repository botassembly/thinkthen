use std::io::{Read, Write};
use std::process::ExitCode;
use std::time::Duration;

use crate::args::RelateArguments;
use crate::asking::{Folders, ask_prepared};
use crate::core::{AnswerOutcome, Backend, ModelName, RelationEntity, assemble_edges};
use crate::edge::Environment;
use crate::failure::Failure;
use crate::http::Client;
use crate::profile;
use crate::recorder::Recorder;

mod config;
mod dry_run;
mod input;
mod plan;
mod result;

#[rustfmt::skip]
pub(crate) fn run(
    arguments: &RelateArguments, environment: &Environment,
    input: impl Read + Send + 'static, mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let settled = config::settle(arguments)?;
    let source = crate::edge::source(arguments.common.input.as_deref(), input)?;
    let entities = input::read(source, settled.framing, &settled.spec)?;
    let configured = settled
        .spec
        .model
        .as_ref()
        .map(ModelName::as_str)
        .or_else(|| environment.model());
    let backend = Backend::resolve(
        arguments.common.url.as_deref(),
        environment.base_url(),
        configured.unwrap_or(crate::core::DEFAULT_MODEL),
    )?;
    let selected_profile = profile::read(&arguments.common)?;
    if entities.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    let prepared = plan::prepare(
        &entities,
        &settled.spec,
        &backend,
        selected_profile.as_ref(),
    )?;
    if arguments.common.dry_run {
        let folders = Folders::of(&arguments.common, environment)?;
        if folders.named() {
            return Err(Failure::DryRunWithRecording);
        }
        dry_run::write(&mut writer, dry_run::Context {
            backend: &backend, profile: selected_profile.as_ref(), framing: settled.framing,
            spec: &settled.spec, from: settled.from, entity_count: entities.len(), prepared: &prepared,
        })?;
        return Ok(ExitCode::SUCCESS);
    }
    let folders = Folders::of(&arguments.common, environment)?;
    let recorder = Recorder::of_private(
        folders.record.as_deref(),
        folders.replay.as_deref(),
        folders.private_default,
        folders.cache_answers,
    )?;
    let client = Client::new(
        Duration::from_secs(arguments.common.timeout),
        backend.is_secure(),
    );
    let mismatch = profile::Mismatch::new(settled.spec.profile.as_ref(), selected_profile.as_ref());
    let execution = execute(
        prepared,
        &entities,
        &backend,
        arguments,
        environment,
        &recorder,
        &client,
        settled.spec.threshold.cut_value().unwrap_or(0.5),
    )?;
    if execution.answered == 0 && execution.failed > 0 {
        return Err(Failure::Relate(crate::failure::relate::Error::Logical));
    }
    let partial = execution.failed > 0;
    result::write(&mut writer, result::Output {
        details: arguments.common.details, framing: settled.framing, spec: &settled.spec,
        entities: &entities, backend: &backend, warning: mismatch.warning(), execution,
    })?;
    mismatch.print_once()?;
    Ok(if partial {
        ExitCode::from(6)
    } else {
        ExitCode::SUCCESS
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "the command passes existing request, storage, and cancellation boundaries explicitly"
)]
#[rustfmt::skip]
fn execute(
    prepared: Vec<plan::PreparedRelation>, entities: &[RelationEntity], backend: &Backend,
    arguments: &RelateArguments, environment: &Environment, recorder: &Recorder,
    client: &Client, threshold: f64,
) -> Result<result::Execution, Failure> {
    let mut execution = result::Execution { replayed: true, ..result::Execution::default() };
    for relation in prepared {
        let mut offset = 0;
        for chunk in relation.chunks {
            let answered = ask_prepared(
                backend,
                &chunk.plan,
                chunk.request,
                &arguments.common,
                environment,
                recorder,
                client,
            )?;
            add_meta(&mut execution, &answered)?;
            let request = answered.request.as_str().to_owned();
            for outcome in answered.reply.outcomes() {
                let mapping = relation
                    .mappings
                    .get(offset)
                    .cloned()
                    .ok_or(Failure::Defect("a relation reply exceeds its question map"))?;
                let logical = result::Logical {
                    relation: relation.relation.clone(),
                    mapping,
                    outcome: outcome.clone(),
                    request: request.clone(),
                };
                add_logical(&mut execution, entities, logical, threshold);
                offset += 1;
            }
        }
        if offset != relation.mappings.len() {
            return Err(Failure::Defect(
                "a relation reply did not cover its question map",
            ));
        }
    }
    Ok(execution)
}

fn add_logical(
    execution: &mut result::Execution,
    entities: &[RelationEntity],
    logical: result::Logical,
    threshold: f64,
) {
    if let AnswerOutcome::Answered(answer) = &logical.outcome {
        execution.answered += 1;
        execution.edges.extend(assemble_edges(
            entities,
            &logical.relation,
            std::slice::from_ref(&logical.mapping),
            std::slice::from_ref(answer),
            threshold,
        ));
    } else {
        execution.failed += 1;
    }
    execution.logical.push(logical);
}

#[rustfmt::skip]
fn add_meta(execution: &mut result::Execution, answered: &crate::prepared_request::Answered) -> Result<(), Failure> {
    if execution
        .model
        .as_ref()
        .is_some_and(|model| model != answered.reply.model())
    { return Err(Failure::ModelsDiffer(None)); }
    execution.model.get_or_insert_with(|| answered.reply.model().clone());
    execution.usage = match (execution.usage, answered.reply.usage()) {
        (Some(left), Some(right)) => left.checked_plus(right).ok_or(Failure::UsageOverflow).map(Some)?,
        (None, held) | (held, None) => held,
    };
    execution.replayed &= answered.replayed;
    execution.requests_sent = execution.requests_sent.checked_add(answered.requests_sent).ok_or(Failure::UsageOverflow)?;
    execution.requests.push(answered.request.as_str().to_owned());
    Ok(())
}
