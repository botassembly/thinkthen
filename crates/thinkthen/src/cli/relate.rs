use std::io::{Read, Write};
use std::process::ExitCode;

use crate::args::RelateArguments;
use crate::asking::{self, Folders};
use crate::core::{Backend, ModelName};
use crate::edge::Environment;
use crate::engine::facade;
use crate::failure::Failure;
use crate::profile;

mod config;
mod dry_run;
mod input;
mod result;

pub(crate) fn run(
    arguments: &RelateArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let settled = config::settle(arguments)?;
    let configured = settled.spec.model.as_ref().map(ModelName::as_str);
    let request_size = environment.request_size(arguments.max_request_bytes.as_deref())?;
    let backend = Backend::resolve(
        arguments.common.url.as_deref(),
        environment.base_url(),
        configured
            .or_else(|| environment.model())
            .unwrap_or(crate::core::DEFAULT_MODEL),
    )?
    .with_request_size(request_size);
    environment.warn_request_size(&backend)?;
    let selected_profile = profile::read(&arguments.common)?;
    let source = crate::edge::source(arguments.common.input.as_deref(), input)?;
    let entities = input::read(source, settled.framing, &settled.spec)?;
    if entities.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    let prepared = facade::relations(
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
    let mismatch = profile::Mismatch::new(settled.spec.profile.as_ref(), selected_profile.as_ref());
    let engine = asking::engine(
        &arguments.common,
        environment,
        folders,
        backend.clone(),
        selected_profile,
        arguments.common.jobs,
    )?;
    let threshold = settled.spec.threshold.cut_value().unwrap_or(0.5);
    let execution = engine.relate(prepared, &entities, threshold, environment.cancel())?;
    let partial = execution.failed > 0;
    let output = result::Output {
        details: arguments.common.details,
        framing: settled.framing,
        spec: &settled.spec,
        entities: &entities,
        backend: &backend,
        warning: mismatch.warning(),
    };
    result::write(&mut writer, &output, &execution)?;
    environment.usage().record_done();
    mismatch.print_once()?;
    Ok(if partial {
        ExitCode::from(6)
    } else {
        ExitCode::SUCCESS
    })
}
