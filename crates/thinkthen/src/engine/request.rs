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
    ask_prepared(
        backend,
        plan,
        prepared,
        recorder,
        cancel,
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
            crate::engine::workers::on_worker(cancel, || {
                transport
                    .client
                    .post_observed(&exchange, cancel, || transport.usage.request_sent())
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
    let operation = recorder
        .prepare_checked(&recorded, &prepared.digest, cancel, &complete)
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
                }
                reply
            },
            true,
            0,
        ),
        PreparedRecording::Live(permit) => {
            cancel.key_lookup();
            let (permit, key) = finish_or_cancel(permit, key())?;
            let (permit, answered) = finish_or_cancel(permit, send(&prepared, &key))?;
            let decoded = built_in::decode_observed(plan, &answered.body);
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
            if caches && reply.failed_any() {
                permit.cancel()
            } else {
                permit.finish(&recorded, &answered.body, &prepared.digest.file_name())
            }
            .map_err(E::from)?;
            (reply, false, answered.requests_sent)
        }
    };
    Ok(Answered {
        reply,
        replayed,
        request: prepared.digest,
        requests_sent,
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
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, atomic::AtomicUsize, atomic::Ordering, mpsc};
    use std::thread;
    use std::time::Duration;

    use crate::core::{Backend, Evidence, ModelName, Plan, Question, QuestionText};
    use crate::engine::cache_lock;
    use crate::engine::error::Error;
    use crate::engine::http::Key;
    use crate::engine::prepared_request::{Answered, PreparedRequest};
    use crate::engine::recorder::Recorder;
    use crate::engine::usage::Counters;

    fn request() -> (Backend, Plan, PreparedRequest) {
        let backend = Backend::resolve(Some("http://127.0.0.1:1/v1/systemone"), None, "jev-latest")
            .expect("backend");
        let plan = Plan::new(
            Evidence::new("evidence").expect("evidence"),
            ModelName::new("jev-latest").expect("model"),
            vec![Question::Decide {
                text: QuestionText::new("Is this relevant?").expect("question"),
                yes: None,
                no: None,
            }],
        )
        .expect("plan");
        let prepared = PreparedRequest::new(&backend, &plan).expect("request");
        (backend, plan, prepared)
    }

    type Counts = (Arc<AtomicUsize>, Arc<AtomicUsize>);

    fn folder(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!("thinkthen-cancel-{label}-{}", std::process::id()))
    }

    fn has_partial(folder: &Path) -> bool {
        let prefix = format!(".{}.", std::process::id());
        fs::read_dir(folder)
            .expect("recording folder")
            .filter_map(Result::ok)
            .any(|entry| entry.file_name().to_string_lossy().starts_with(&prefix))
    }

    fn wait_on_request(
        request: (Backend, Plan, PreparedRequest),
        recorder: Recorder,
        observed: (crate::engine::Cancel<'static>, Counts),
    ) -> thread::JoinHandle<Result<Answered, Error>> {
        let (backend, plan, prepared) = request;
        let (cancel, counts) = observed;
        thread::spawn(move || {
            super::ask_prepared(
                &backend,
                &plan,
                prepared,
                &recorder,
                &cancel,
                &Counters::default(),
                || {
                    counts.0.fetch_add(1, Ordering::SeqCst);
                    Ok(Key::of("unused"))
                },
                |_, _| {
                    counts.1.fetch_add(1, Ordering::SeqCst);
                    Err(Error::Defect("send unexpectedly reached"))
                },
            )
        })
    }

    #[test]
    fn a_held_folder_cancels_through_the_prepared_request_path() {
        let path = folder("folder");
        let _absent = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("recording folder");
        let owner = cache_lock::exclusive_folder(&path).expect("exclusive owner");
        let (blocked_send, blocked) = mpsc::channel();
        let cancel = crate::engine::Cancel::observed(blocked_send);
        let counts = Counts::default();
        let waiter = wait_on_request(
            request(),
            Recorder::of(Some(&path), None).expect("recorder"),
            (cancel.clone(), counts.clone()),
        );

        blocked
            .recv_timeout(Duration::from_secs(2))
            .expect("shared lock received WouldBlock");
        cancel.fire();
        assert!(matches!(
            waiter.join().expect("waiter"),
            Err(Error::Cancelled)
        ));
        assert_eq!(counts.0.load(Ordering::SeqCst), 0);
        assert_eq!(counts.1.load(Ordering::SeqCst), 0);
        drop(owner);
        assert!(!has_partial(&path));
        fs::remove_dir_all(path).expect("fixture removed");
    }

    #[test]
    fn a_held_digest_cancels_through_the_prepared_request_path() {
        let path = folder("digest");
        let _absent = fs::remove_dir_all(&path);
        let (owner_backend, _, owner_prepared) = request();
        let recorder = Recorder::of(Some(&path), None).expect("recorder");
        let owner_recorded = owner_prepared.recorded(&owner_backend);
        let crate::engine::recorder::PreparedRecording::Live(owner) = recorder
            .prepare_cancelled(
                &owner_recorded,
                &owner_prepared.digest,
                &crate::engine::Cancel::default(),
            )
            .expect("owner prepared")
        else {
            unreachable!("empty recording cannot replay");
        };
        let (blocked_send, blocked) = mpsc::channel();
        let cancel = crate::engine::Cancel::observed(blocked_send);
        let counts = Counts::default();
        let (backend, plan, prepared) = request();
        let digest = prepared.digest.as_str().to_owned();
        let waiter = wait_on_request(
            (backend, plan, prepared),
            recorder,
            (cancel.clone(), counts.clone()),
        );

        blocked
            .recv_timeout(Duration::from_secs(2))
            .expect("digest lock received WouldBlock");
        cancel.fire();
        assert!(matches!(
            waiter.join().expect("waiter"),
            Err(Error::Cancelled)
        ));
        assert_eq!(counts.0.load(Ordering::SeqCst), 0);
        assert_eq!(counts.1.load(Ordering::SeqCst), 0);
        owner.cancel().expect("owner cleanup");
        assert!(!has_partial(&path));
        assert!(path.join(".locks").join(digest).exists());
        fs::remove_dir_all(path).expect("fixture removed");
    }

    #[test]
    fn cleanup_failure_at_the_cancel_boundary_wins_and_keeps_owned_state() {
        let (backend, _plan, prepared) = request();
        let folder = folder("prepared");
        let _absent = fs::remove_dir_all(&folder);
        let recorder = Recorder::of(Some(&folder), None).expect("recorder");
        let cancel = crate::engine::Cancel::default();
        let operation = recorder
            .prepare_cancelled(&prepared.recorded(&backend), &prepared.digest, &cancel)
            .expect("prepared recording");
        let lock = folder.join(".locks").join(prepared.digest.as_str());
        assert!(has_partial(&folder));

        crate::engine::recorder::fail_cleanup();
        cancel.fire();
        let result = super::observe_cancel::<Error>(operation, &cancel);

        assert!(matches!(result, Err(Error::RecordingStorage)));
        assert!(has_partial(&folder));
        assert!(lock.exists());
        fs::remove_dir_all(folder).expect("fixture removed");
    }
}
