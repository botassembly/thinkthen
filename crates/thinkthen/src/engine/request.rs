//! One prepared request through replay, cache locking, transport, and recording.

use std::time::Duration;

use crate::core::adapters::built_in;
use crate::core::{Backend, BackendProfile, Plan};
use crate::engine::error::Error;
use crate::engine::http::{Client, Exchange, Key};
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
        Injection::Deadline => Error::Deadline,
        Injection::Defect => Error::Defect("injected invariant failure"),
    }
}

/// The transport settings for one request.
pub(crate) struct Transport<'a> {
    pub(crate) client: &'a Client,
    pub(crate) max_retries: u32,
    pub(crate) retry_wait: Duration,
    pub(crate) usage: &'a Counters,
}

pub(crate) fn ask_profile<E>(
    backend: &Backend,
    plan: &Plan,
    profile: Option<&BackendProfile>,
    recorder: &Recorder,
    transport: Transport<'_>,
    key: impl FnOnce() -> Result<Key, E>,
) -> Result<Answered, E>
where
    E: From<Error>,
{
    let prepared = PreparedRequest::with_profile(backend, plan, profile).map_err(E::from)?;
    ask_prepared(
        backend,
        plan,
        prepared,
        recorder,
        transport.usage,
        key,
        |prepared, key| {
            transport
                .client
                .post_observed(
                    &Exchange {
                        url: backend.url().as_str(),
                        body: &prepared.body,
                        key,
                        max_retries: transport.max_retries,
                        retry_wait: transport.retry_wait,
                    },
                    || transport.usage.request_sent(),
                )
                .map_err(E::from)
        },
    )
}

#[cfg(test)]
pub(crate) fn ask_with<E>(
    backend: &Backend,
    plan: &Plan,
    recorder: &Recorder,
    key: impl FnOnce() -> Result<Key, E>,
    send: impl FnOnce(&PreparedRequest, &Key) -> Result<Vec<u8>, E>,
) -> Result<Answered, E>
where
    E: From<Error>,
{
    let prepared = PreparedRequest::new(backend, plan).map_err(E::from)?;
    let usage = Counters::default();
    ask_prepared(backend, plan, prepared, recorder, &usage, key, send)
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
    usage: &Counters,
    key: impl FnOnce() -> Result<Key, E>,
    send: impl FnOnce(&PreparedRequest, &Key) -> Result<Vec<u8>, E>,
) -> Result<Answered, E>
where
    E: From<Error>,
{
    let recorded = prepared.recorded(backend);
    let operation = recorder
        .prepare(&recorded, &prepared.digest)
        .map_err(E::from)?;
    let (reply, replayed) = match operation {
        PreparedRecording::Replay(response) => (
            {
                let reply = built_in::decode(plan, &response)
                    .map_err(Error::from)
                    .map_err(E::from)?;
                if recorder.counts_cache_answers() {
                    usage.cache_answer();
                }
                reply
            },
            true,
        ),
        PreparedRecording::Live(permit) => {
            let (permit, key) = finish_or_cancel(permit, key())?;
            let (permit, answered) = finish_or_cancel(permit, send(&prepared, &key))?;
            let decoded = built_in::decode_observed(plan, &answered);
            if let Some(tokens) = decoded.usage {
                usage.tokens(tokens);
            }
            let reply = match decoded.reply {
                Ok(reply) => reply,
                Err(error) => {
                    permit.cancel().map_err(E::from)?;
                    return Err(E::from(Error::from(error)));
                }
            };
            permit
                .finish(&recorded, &answered, &prepared.digest.file_name())
                .map_err(E::from)?;
            (reply, false)
        }
    };
    Ok(Answered {
        reply,
        replayed,
        request: prepared.digest,
    })
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
