//! Complete native calls and batches use the same R interrupt boundary.
use super::{Crossed, completion_of, deadline_of, interrupt_pending, text_of};
use crate::{calls, usage};
use extendr_api::prelude::*;
#[extendr]
fn tt_complete_native(request: Robj, deadline: Robj, completion: Robj) -> Crossed<List> {
    let request = text_of(&request, "complete request")?;
    let done = calls::call_owned(
        deadline_of(&deadline)?,
        &interrupt_pending,
        completion_of(&completion)?,
        None,
        move |engine, options, account| {
            let (value, facts) = crate::complete::execute(
                engine,
                crate::complete::parse(&request).map_err(|e| crate::carry(&e))?,
                options,
            )
            .map_err(|e| crate::carry(&e))?;
            account.include(account.start(), &facts)?;
            Ok(value)
        },
    )?;
    calls::render::envelope(done)
}

#[extendr]
fn tt_complete_batch_start(request: Robj, deadline: Robj) -> Crossed<Robj> {
    let deadline = deadline_of(&deadline)?
        .filter(|d| *d >= 0.0)
        .map(|d| d as i64);
    let session = crate::complete::stream::Session::start(
        crate::engine()?,
        text_of(&request, "complete request")?,
        deadline,
        None,
        None,
    )
    .map_err(|e| crate::carry(&e))?;
    Ok(ExternalPtr::new(session).into())
}
#[extendr]
fn tt_complete_batch_pull(batch: Robj) -> Crossed<String> {
    let session = ExternalPtr::<crate::complete::stream::Session>::try_from(batch)
        .map_err(|_| usage("a native complete batch is required"))?;
    session.advance().map_err(|e| crate::carry(&e))?;
    loop {
        if interrupt_pending() {
            session.cancel();
            return Err(crate::interrupted());
        }
        if let Some(event) = session.poll().map_err(|e| crate::carry(&e))? {
            return Ok(event);
        }
    }
}
#[extendr]
fn tt_complete_batch_poll(batch: Robj, advance: bool) -> Crossed<Robj> {
    let session = ExternalPtr::<crate::complete::stream::Session>::try_from(batch)
        .map_err(|_| usage("a native complete batch is required"))?;
    if advance {
        session.advance().map_err(|e| crate::carry(&e))?;
    }
    if interrupt_pending() {
        session.cancel();
        return Err(crate::interrupted());
    }
    Ok(match session.poll().map_err(|e| crate::carry(&e))? {
        Some(event) => event.into(),
        None => ().into(),
    })
}
#[extendr]
fn tt_complete_batch_close(batch: Robj) -> Crossed<()> {
    let session = ExternalPtr::<crate::complete::stream::Session>::try_from(batch)
        .map_err(|_| usage("a native complete batch is required"))?;
    session.close();
    Ok(())
}
#[extendr]
fn tt_complete_batch_cancel(batch: Robj) -> Crossed<()> {
    let session = ExternalPtr::<crate::complete::stream::Session>::try_from(batch)
        .map_err(|_| usage("a native complete batch is required"))?;
    session.cancel();
    Ok(())
}

#[extendr]
#[expect(
    clippy::too_many_arguments,
    reason = "R's engine constructor passes its public settings through this binding"
)]
fn tt_engine_set(
    base_url: Robj,
    model: Robj,
    throttle: Robj,
    max_requests: Robj,
    max_requests_total: Robj,
    max_request_bytes: Robj,
    cache: Robj,
    timeout: Robj,
    max_retries: Robj,
    record: Robj,
    replay: Robj,
    profile: Robj,
    batch: Robj,
    backend: Robj,
    refresh_cache: Robj,
) -> Crossed<()> {
    super::engine::configure([
        base_url,
        model,
        throttle,
        max_requests,
        max_requests_total,
        max_request_bytes,
        cache,
        timeout,
        max_retries,
        record,
        replay,
        profile,
        batch,
        backend,
        refresh_cache,
    ])
}

extendr_module! {
mod complete;
    fn tt_engine_set;
fn tt_complete_native;
fn tt_complete_batch_start;
fn tt_complete_batch_pull;
fn tt_complete_batch_poll;
fn tt_complete_batch_close;
fn tt_complete_batch_cancel;
}
