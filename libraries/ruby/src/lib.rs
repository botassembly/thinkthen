//! The Ruby surface over the `thinkthen` public API (ticket 0112).
//!
//! This crate holds no rule. It reads a call's inputs into owned Rust values
//! on the Ruby thread, runs the call on a worker thread that never touches
//! Ruby, and hands back plain values. `ffi.rs` is the only module that
//! touches Ruby. It waits for the worker in 50 ms slices with the
//! interpreter lock released, and between slices it reads Ruby's pending
//! interrupts, the caller's token, and the call's own token. On any stop it
//! fires the call's own token, detaches the worker, and raises at once. The
//! detached worker's sent requests finish, and its result is dropped.

mod call;
mod diagnostics;
mod ffi;

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use thinkthen::{CancelToken, Engine, EngineBuilder, ErrorKind};

use crate::call::{Ask, Output};

/// How long one wait holds before the Ruby thread reads its interrupts.
const SLICE: Duration = Duration::from_millis(50);

/// One failure: its kind, its safe message, and the retry signal.
#[derive(Debug, PartialEq)]
struct Fault {
    kind: ErrorKind,
    message: String,
    retryable: bool,
}

impl Fault {
    fn of(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            retryable: false,
        }
    }

    fn usage(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Usage, message)
    }

    fn cancelled() -> Self {
        Self::of(ErrorKind::Cancelled, "the call was cancelled")
    }
}

impl From<thinkthen::Error> for Fault {
    fn from(error: thinkthen::Error) -> Self {
        Self {
            kind: error.kind(),
            message: error.detail().message().to_owned(),
            retryable: error.retryable(),
        }
    }
}

/// The one error table: each kind's class under `ThinkThen`.
const fn class_name(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::Usage => "UsageError",
        ErrorKind::Backend => "BackendError",
        ErrorKind::Local => "LocalError",
        ErrorKind::Cancelled => "CancelledError",
        ErrorKind::Deadline => "DeadlineError",
        ErrorKind::Defect => "DefectError",
    }
}

/// The one panic guard. A panic in the binding's own code becomes the
/// defect kind. `thinkthen` already stops engine panics at its doors.
fn guarded<T>(body: impl FnOnce() -> Result<T, Fault>) -> Result<T, Fault> {
    diagnostics::owned(|| match catch_unwind(AssertUnwindSafe(body)) {
        Ok(value) => value,
        Err(payload) => {
            std::mem::forget(payload);
            Err(Fault::of(
                ErrorKind::Defect,
                "defect: the Ruby binding panicked",
            ))
        }
    })
}

/// What the waiting Ruby thread finds in the handoff.
#[derive(Debug)]
enum Taken {
    Waiting,
    Ready(Result<Output, Fault>),
    Closed,
}

/// The slot one worker fills and one Ruby thread reads, and the wake flag
/// Ruby's unblock function sets.
#[derive(Debug, Default)]
struct Handoff {
    state: Mutex<Slot>,
    changed: Condvar,
}

#[derive(Debug, Default)]
struct Slot {
    answer: Option<Result<Output, Fault>>,
    woken: bool,
    closed: bool,
}

impl Handoff {
    fn lock(&self) -> MutexGuard<'_, Slot> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Wait one slice, or less when the answer lands, the worker ends, or
    /// Ruby wakes the wait.
    fn wait(&self) {
        let mut state = self.lock();
        if state.answer.is_none() && !state.woken && !state.closed {
            state = match self.changed.wait_timeout(state, SLICE) {
                Ok((held, _)) => held,
                Err(poisoned) => poisoned.into_inner().0,
            };
        }
        state.woken = false;
    }

    fn wake(&self) {
        self.lock().woken = true;
        self.changed.notify_all();
    }

    fn take(&self) -> Taken {
        let mut state = self.lock();
        match state.answer.take() {
            Some(answer) => Taken::Ready(answer),
            None if state.closed => Taken::Closed,
            None => Taken::Waiting,
        }
    }
}

/// The worker's end of the handoff. Dropping it without an answer closes
/// the handoff, so the Ruby thread never waits on a worker that is gone.
#[derive(Debug)]
struct Feed(Arc<Handoff>);

impl Feed {
    fn put(self, answer: Result<Output, Fault>) {
        self.0.lock().answer = Some(answer);
        self.0.changed.notify_all();
    }
}

impl Drop for Feed {
    fn drop(&mut self) {
        self.0.lock().closed = true;
        self.0.changed.notify_all();
    }
}

/// Start one call on a worker thread that owns every input. `prepare` runs
/// first on the worker, and the FFI module passes the signal mask there.
fn start(
    engine: Engine,
    ask: Ask,
    own: CancelToken,
    deadline: Option<f64>,
    prepare: fn(),
) -> Result<Arc<Handoff>, Fault> {
    let handoff = Arc::new(Handoff::default());
    let feed = Feed(Arc::clone(&handoff));
    std::thread::Builder::new()
        .name("thinkthen-call".to_owned())
        .spawn(move || {
            feed.put(guarded(|| {
                prepare();
                call::run(&engine, ask, &own, deadline)
            }));
        })
        .map_err(|_| Fault::of(ErrorKind::Local, "the call's worker thread could not start"))?;
    Ok(handoff)
}

/// The settings `ThinkThen::Engine.new` takes, over the environment's.
#[derive(Debug, Default)]
struct Settings {
    base_url: Option<String>,
    model: Option<String>,
    throttle: Option<i64>,
    max_requests: Option<i64>,
    max_request_bytes: Option<i64>,
    cache_at: Option<String>,
    no_cache: bool,
    timeout: Option<i64>,
    max_retries: Option<i64>,
    record: Option<String>,
    replay: Option<String>,
    profile: Option<String>,
}

impl Settings {
    /// Start from `EngineBuilder::from_env()`, then apply each given setting.
    fn build(self) -> Result<Engine, Fault> {
        let mut builder = EngineBuilder::from_env()?;
        if let Some(value) = &self.base_url {
            builder = builder.base_url(value)?;
        }
        if let Some(value) = &self.model {
            builder = builder.model(value)?;
        }
        if let Some(value) = self.throttle {
            let value = u8::try_from(value)
                .map_err(|_| Fault::usage("a throttle is a whole number from 1 through 32"))?;
            builder = builder.throttle(value)?;
        }
        if let Some(value) = self.max_requests {
            let value = usize::try_from(value)
                .map_err(|_| Fault::usage("a request limit is a whole number of 1 or more"))?;
            builder = builder.max_requests(Some(value))?;
        }
        if let Some(value) = self.max_request_bytes {
            let value = usize::try_from(value)
                .map_err(|_| Fault::usage("max_request_bytes is a whole number of at least 1"))?;
            builder = builder.max_request_bytes(value)?;
        }
        if let Some(folder) = &self.cache_at {
            builder = builder.cache_at(folder)?;
        }
        if self.no_cache {
            builder = builder.no_cache();
        }
        if let Some(value) = self.timeout {
            let value = u64::try_from(value)
                .map_err(|_| Fault::usage("a timeout is a whole number of seconds above zero"))?;
            builder = builder.timeout(std::time::Duration::from_secs(value))?;
        }
        if let Some(value) = self.max_retries {
            let value =
                u32::try_from(value).map_err(|_| Fault::usage("max_retries is a whole number"))?;
            builder = builder.max_retries(value);
        }
        if let Some(folder) = self.record {
            builder = builder.record(folder)?;
        }
        if let Some(folder) = self.replay {
            builder = builder.replay(folder)?;
        }
        if let Some(path) = self.profile {
            builder = builder.profile(path)?;
        }
        Ok(builder.build()?)
    }
}

/// The worker design moves these values onto another thread.
const _: fn() = || {
    fn held<T: Send + 'static>() {}
    held::<thinkthen::Question>();
    held::<thinkthen::LoadedQuestion>();
    held::<thinkthen::QuestionSet>();
    held::<thinkthen::Recognize>();
    held::<thinkthen::Relate>();
    held::<thinkthen::Entity>();
    // Rust holds no Ruby object. Every Ruby value is !Send, so a Ruby field
    // in a wrapped struct fails this build.
    held::<ffi::CancelValue>();
    held::<ffi::QuestionValue>();
    held::<ffi::SetValue>();
    held::<ffi::EngineValue>();
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_guard_turns_a_panic_into_the_defect_class() {
        let held = guarded(|| -> Result<(), Fault> { panic!("a binding bug") });
        let fault = held.expect_err("a panic must not read as a value");
        assert_eq!(class_name(fault.kind), "DefectError");
        assert_eq!(fault.message, "defect: the Ruby binding panicked");
    }

    #[test]
    fn each_kind_maps_to_its_own_class() {
        let table = [
            (ErrorKind::Usage, "UsageError"),
            (ErrorKind::Backend, "BackendError"),
            (ErrorKind::Local, "LocalError"),
            (ErrorKind::Cancelled, "CancelledError"),
            (ErrorKind::Deadline, "DeadlineError"),
            (ErrorKind::Defect, "DefectError"),
        ];
        for (kind, name) in table {
            assert_eq!(class_name(kind), name, "{kind:?}");
        }
    }

    #[test]
    fn a_worker_that_ends_with_no_answer_closes_the_handoff() {
        let handoff = Arc::new(Handoff::default());
        drop(Feed(Arc::clone(&handoff)));
        handoff.wait();
        assert!(matches!(handoff.take(), Taken::Closed));
    }

    #[test]
    fn a_worker_outlives_its_caller_and_its_answer_is_dropped() {
        let handoff = Arc::new(Handoff::default());
        let feed = Feed(Arc::clone(&handoff));
        drop(handoff);
        let worker = std::thread::spawn(move || feed.put(Ok(Output::Score(0.5))));
        assert!(
            worker.join().is_ok(),
            "the worker's send on a gone caller must not panic"
        );
    }
}
