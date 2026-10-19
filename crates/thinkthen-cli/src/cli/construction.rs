//! Build the command's engine from its one captured public builder.
use crate::cli::{args::Common, asking::Folders, edge::Environment, failure::Failure};
use crate::Engine;
use std::io::{self, Write as _};
use std::time::Duration;

pub(crate) fn engine(
    common: &Common,
    environment: &Environment,
    folders: Folders,
    width: Option<u8>,
) -> Result<Engine, Failure> {
    environment.roots()?;
    if !common.dry_run && folders.writable_by_another() {
        writeln!(io::stderr().lock(),
            "thinkthen: warning: another user may change this named cache or recording folder; its writers decide the answers read from it")
            .map_err(Failure::Output)?;
    }
    let route = environment.inspect_builder(crate::EngineBuilder::resolved_settings)?;
    if folders.record.is_some() && folders.replay.is_some()
        && (folders.refresh_cache || route.model_is_mutable_alias) {
        writeln!(io::stderr().lock(),
            "thinkthen: warning: a mutable model alias or --refresh-cache sends each planned cache request live and may incur a charge")
            .map_err(Failure::Output)?;
    }
    let mut builder = environment.take_builder()?
        .timeout(Duration::from_secs(common.timeout))?
        .max_retries(common.max_retries)
        .retry_wait(environment.retry_wait())
        .max_requests_total(common.max_requests_total);
    if common.max_estimated_input_tokens_total.is_some() {
        builder = builder.max_estimated_input_tokens_total(common.max_estimated_input_tokens_total);
    }
    if let Some(width) = width { builder = builder.throttle(width)?; }
    if folders.cache_answers {
        if folders.private_default {
            builder = builder.default_cache();
        } else {
            let path = folders.record.as_deref().ok_or(Failure::Defect("an answer cache has no folder"))?;
            builder = builder.cache_at(path)?;
        }
    } else {
        builder = builder.no_cache();
        if let Some(path) = &folders.record { builder = builder.record(path)?; }
        if let Some(path) = &folders.replay { builder = builder.replay(path)?; }
    }
    let engine = builder.refresh_cache(folders.refresh_cache).build()?;
    environment.observe_engine(&engine);
    Ok(engine)
}
