//! Shared resolved CLI construction for private maintenance and native judgments.
use crate::cli::{args::Common, asking::Folders, edge::Environment, failure::Failure};
use crate::core::{Backend, BackendProfile};
use crate::engine::{
    Width,
    facade::{Engine, Settings, Storage},
};
use std::io::{self, Write as _};
use std::time::Duration;

pub(crate) fn engine(
    common: &Common,
    environment: &Environment,
    folders: Folders,
    route: (Backend, Option<BackendProfile>),
    width: Option<u8>,
    legacy_invocation: bool,
) -> Result<Engine, Failure> {
    let (backend, profile) = route;
    let width = width.map(|jobs| Width::new(u64::from(jobs))).transpose()?;
    let roots = environment.roots()?;
    if legacy_invocation && !common.dry_run {
        environment.cancel().invocation()?;
    }
    if !common.dry_run && folders.writable_by_another() {
        writeln!(
            io::stderr().lock(),
            "thinkthen: warning: another user may change this named cache or recording folder; its writers decide the answers read from it"
        )
        .map_err(Failure::Output)?;
    }
    if folders.record.is_some()
        && folders.replay.is_some()
        && (folders.refresh_cache || backend.api_type().is_mutable_alias(backend.model()))
    {
        writeln!(
            io::stderr().lock(),
            "thinkthen: warning: a mutable model alias or --refresh-cache sends each planned cache request live and may incur a charge"
        )
        .map_err(Failure::Output)?;
    }
    Ok(Engine::with_roots(
        Settings {
            backend,
            profile,
            timeout: Duration::from_secs(common.timeout),
            max_retries: common.max_retries,
            retry_wait: environment.retry_wait(),
            per_minute: environment.per_minute,
            width,
            storage: Storage {
                record: folders.record,
                replay: folders.replay,
                private_default: folders.private_default,
                cache_answers: folders.cache_answers,
                refresh_cache: folders.refresh_cache,
            },
            key: environment.key_reader(),
            usage: environment.counters(),
        },
        roots,
    )?
    .with_process_budget(
        common.max_requests_total,
        common
            .max_estimated_input_tokens_total
            .or(environment.estimated_total),
    ))
}
