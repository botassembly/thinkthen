//! One prepared request through replay, cache locking, transport, and recording.

use std::time::Duration;

use crate::core::adapters::built_in;
use crate::core::{Backend, Plan};
use crate::engine::cache_lock;
use crate::engine::error::Error;
use crate::engine::http::{Client, Exchange, Key};
use crate::engine::prepared_request::{Answered, PreparedRequest};
use crate::engine::recorder::Recorder;

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
        Injection::Local => {
            Error::Recording(std::io::Error::other("injected recording read failure"))
        }
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
}

pub(crate) fn ask<E>(
    backend: &Backend,
    plan: &Plan,
    recorder: &Recorder,
    transport: Transport<'_>,
    key: impl FnOnce() -> Result<Key, E>,
) -> Result<Answered, E>
where
    E: From<Error>,
{
    ask_with(backend, plan, recorder, key, |prepared, key| {
        transport
            .client
            .post(&Exchange {
                url: backend.url().as_str(),
                body: &prepared.body,
                key,
                max_retries: transport.max_retries,
                retry_wait: transport.retry_wait,
            })
            .map_err(E::from)
    })
}

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
    let recorded = prepared.recorded(backend);
    let (reply, replayed) = cache_lock::coalesce(
        || {
            let replayed = recorder
                .replayed(&recorded, &prepared.digest)
                .map_err(E::from)?;
            replayed
                .map(|response| {
                    built_in::decode(plan, &response)
                        .map_err(Error::from)
                        .map_err(E::from)
                })
                .transpose()
        },
        || recorder.lock(&prepared.digest).map_err(E::from),
        || {
            let key = key()?;
            let answered = send(&prepared, &key)?;
            let reply = built_in::decode(plan, &answered)
                .map_err(Error::from)
                .map_err(E::from)?;
            recorder
                .record(&recorded, &prepared.digest, &answered)
                .map_err(E::from)?;
            Ok(reply)
        },
    )?;
    Ok(Answered {
        reply,
        replayed,
        request: prepared.digest,
    })
}
