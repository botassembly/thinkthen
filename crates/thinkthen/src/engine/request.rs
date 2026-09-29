//! One prepared request through replay, cache locking, transport, and recording.

use std::time::Duration;

use crate::core::adapters::built_in;
use crate::core::{Backend, BackendProfile, Plan};
use crate::engine::error::Error;
use crate::engine::http::{Client, Exchange, HttpAnswer, Key};
use crate::engine::prepared_request::{Answered, PreparedRequest};
use crate::engine::recorder::{PreparedRecording, Recorder, WritePermit};
use crate::engine::usage::Counters;

#[cfg(test)]
pub(crate) enum Injection {
    Backend,
    Local,
    Cancelled,
    Deadline,
    Defect,
}

#[cfg(test)]
pub(crate) fn inject(injection: Injection) -> Error {
    match injection {
        Injection::Backend => Error::Status(422),
        Injection::Local => Error::RecordingStorage,
        Injection::Cancelled => Error::Cancelled,
        Injection::Deadline => Error::Deadline(crate::engine::error::Budget(Duration::ZERO)),
        Injection::Defect => Error::Defect("injected invariant failure"),
    }
}

/// The transport settings for one request.
pub(crate) struct Transport<'a> {
    pub(crate) client: &'a Client,
    pub(crate) max_retries: u32,
    pub(crate) retry_wait: Duration,
    pub(crate) usage: &'a Counters,
    pub(crate) send_budget: Option<(crate::public::SendBudget, Option<u64>)>,
}

#[expect(
    clippy::too_many_arguments,
    reason = "one request carries explicit cancellation, transport, storage, and key boundaries"
)]
pub(crate) fn ask_profile<E>(
    backend: &Backend,
    plan: &Plan,
    profile: Option<&BackendProfile>,
    recorder: &Recorder,
    cancel: &crate::engine::Cancel,
    transport: Transport<'_>,
    key: impl FnOnce() -> Result<Key, E>,
) -> Result<Answered, E>
where
    E: From<Error>,
{
    let prepared = PreparedRequest::with_profile(backend, plan, profile).map_err(E::from)?;
    ask_sent(backend, plan, prepared, recorder, cancel, transport, key)
}

/// Send one prepared request through replay, transport, and recording.
///
/// Every live attempt goes out on an engine worker, so a host signal on the
/// calling thread never lands in a socket read. A replay spawns nothing.
#[expect(
    clippy::too_many_arguments,
    reason = "one prepared request carries explicit cancellation, transport, storage, and key boundaries"
)]
pub(crate) fn ask_sent<E>(
    backend: &Backend,
    plan: &Plan,
    prepared: PreparedRequest,
    recorder: &Recorder,
    cancel: &crate::engine::Cancel,
    transport: Transport<'_>,
    key: impl FnOnce() -> Result<Key, E>,
) -> Result<Answered, E>
where
    E: From<Error>,
{
    ask_sent_observed(
        backend,
        plan,
        prepared,
        recorder,
        cancel,
        transport,
        key,
        || (),
    )
}

/// The same prepared request with one private after-mark attempt receipt.
#[expect(
    clippy::too_many_arguments,
    reason = "one prepared request keeps its existing boundary plus a private attempt receipt"
)]
pub(crate) fn ask_sent_observed<E>(
    backend: &Backend,
    plan: &Plan,
    prepared: PreparedRequest,
    recorder: &Recorder,
    cancel: &crate::engine::Cancel,
    transport: Transport<'_>,
    key: impl FnOnce() -> Result<Key, E>,
    marked: impl Fn() + Sync,
) -> Result<Answered, E>
where
    E: From<Error>,
{
    let cancel = cancel.with_process_budget(transport.send_budget.clone());
    ask_prepared(
        backend,
        plan,
        prepared,
        recorder,
        &cancel,
        transport.usage,
        key,
        |prepared, key| {
            let exchange = Exchange {
                url: backend.url().as_str(),
                body: &prepared.body,
                key,
                max_retries: transport.max_retries,
                retry_wait: transport.retry_wait,
            };
            crate::engine::workers::on_worker(&cancel, || {
                transport.client.post_marked_with_retry(
                    &exchange,
                    &cancel,
                    transport.usage,
                    |_| (),
                    &marked,
                )
            })
            .map_err(E::from)
        },
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "one prepared request carries explicit transport, storage, and counter boundaries"
)]
pub(crate) fn ask_prepared<E>(
    backend: &Backend,
    plan: &Plan,
    prepared: PreparedRequest,
    recorder: &Recorder,
    cancel: &crate::engine::Cancel,
    usage: &Counters,
    key: impl FnOnce() -> Result<Key, E>,
    send: impl FnOnce(&PreparedRequest, &Key) -> Result<HttpAnswer, E>,
) -> Result<Answered, E>
where
    E: From<Error>,
{
    let recorded = prepared.recorded(backend);
    let caches = recorder.caches();
    let complete = |response: &[u8]| {
        !caches || !built_in::decode(plan, response).is_ok_and(|reply| reply.failed_any())
    };
    let mut read_key = Some(key);
    let FirstUse {
        key: early_key,
        zero_limit: early_zero_limit,
    } = first_use_key(recorder, cancel, &mut read_key)?;
    let refresh = caches && (recorder.force_refresh() || built_in::is_mutable_alias(plan.model()));
    let operation = recorder
        .prepare_checked_refresh(&recorded, &prepared.digest, cancel, &complete, refresh)
        .map_err(E::from)?;
    let operation = observe_cancel(operation, cancel)?;
    let (reply, replayed, requests_sent) = match operation {
        PreparedRecording::Replay(response) => (
            {
                let reply = built_in::decode(plan, &response)
                    .map_err(Error::from)
                    .map_err(E::from)?;
                if recorder.counts_cache_answers() {
                    usage.cache_answer();
                    cancel.cache_answer();
                }
                reply
            },
            true,
            0,
        ),
        PreparedRecording::Live(permit) => {
            let found_key = match early_key {
                Some(result) => result,
                None => {
                    cancel.key_lookup();
                    let read = read_key
                        .take()
                        .ok_or_else(|| E::from(Error::Defect("key lookup was already used")))?;
                    read()
                }
            };
            let (permit, key) = finish_or_cancel(permit, found_key)?;
            if early_zero_limit {
                let stop = cancel.stop().unwrap_or(Error::SendBudgetFirst);
                permit.cancel().map_err(E::from)?;
                return Err(E::from(stop));
            }
            let (permit, answered) = finish_or_cancel(permit, send(&prepared, &key))?;
            let decoded = built_in::decode_observed(plan, &answered.body);
            usage.live_reply(decoded.usage);
            cancel.live_reply(decoded.usage);
            let reply = match decoded.reply {
                Ok(reply) => reply,
                Err(error) => {
                    permit.cancel().map_err(E::from)?;
                    return Err(E::from(Error::from(error)));
                }
            };
            if caches && reply.failed_any() {
                permit.cancel()
            } else {
                permit.finish(&recorded, &answered.body, &prepared.digest.file_name())
            }
            .map_err(E::from)?;
            (reply, false, answered.requests_sent)
        }
    };
    usage.answered_by(reply.model());
    cancel.answered_by(reply.model().as_str());
    Ok(Answered {
        reply,
        replayed,
        request: prepared.digest,
        requests_sent,
    })
}

struct FirstUse<E> {
    key: Option<Result<Key, E>>,
    zero_limit: bool,
}

/// Check only refusals certain before an unbound folder's writable admission.
fn first_use_key<E>(
    recorder: &Recorder,
    cancel: &crate::engine::Cancel,
    read_key: &mut Option<impl FnOnce() -> Result<Key, E>>,
) -> Result<FirstUse<E>, E>
where
    E: From<Error>,
{
    if !recorder.unbound_empty().map_err(E::from)? {
        return Ok(FirstUse {
            key: None,
            zero_limit: false,
        });
    }
    if let Some(stop) = cancel.stop() {
        return Err(E::from(stop));
    }
    cancel.key_lookup();
    let read = read_key
        .take()
        .ok_or_else(|| E::from(Error::Defect("key lookup was already used")))?;
    let key = match read() {
        Ok(key) => key,
        Err(error) => {
            if recorder.unbound_empty().map_err(E::from)? {
                return Err(error);
            }
            return Ok(FirstUse {
                key: Some(Err(error)),
                zero_limit: false,
            });
        }
    };
    if !cancel.has_zero_send_limit() {
        if let Err(error) = key.check_line_break()
            && recorder.unbound_empty().map_err(E::from)?
        {
            if let Some(stop) = cancel.stop() {
                return Err(E::from(stop));
            }
            return Err(E::from(error));
        }
        return Ok(FirstUse {
            key: Some(Ok(key)),
            zero_limit: false,
        });
    }
    if recorder.unbound_empty().map_err(E::from)? {
        if let Some(stop) = cancel.stop() {
            return Err(E::from(stop));
        }
        return Err(E::from(Error::SendBudgetFirst));
    }
    Ok(FirstUse {
        key: Some(Ok(key)),
        zero_limit: true,
    })
}

fn observe_cancel<E>(
    operation: PreparedRecording,
    cancel: &crate::engine::Cancel,
) -> Result<PreparedRecording, E>
where
    E: From<Error>,
{
    let Some(stop) = cancel.stop() else {
        return Ok(operation);
    };
    if let PreparedRecording::Live(permit) = operation {
        permit.cancel().map_err(E::from)?;
    }
    Err(E::from(stop))
}

fn finish_or_cancel<T, E>(permit: WritePermit, result: Result<T, E>) -> Result<(WritePermit, T), E>
where
    E: From<Error>,
{
    match result {
        Ok(value) => Ok((permit, value)),
        Err(error) => {
            permit.cancel().map_err(E::from)?;
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests;
