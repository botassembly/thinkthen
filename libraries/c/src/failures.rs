//! The error surface: the six codes, one failure table per engine keyed by
//! the recording thread, and the panic guard every exported symbol runs
//! behind.

use crate::ffi::values as abi;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt;
use std::sync::{Arc, Mutex, PoisonError, Weak};
use std::thread::ThreadId;

use thinkthen::{Engine, ErrorKind, Facts, contained};

/// The header's success code.
pub(crate) const OK: i32 = abi::THINKTHEN_OK;
/// The header's usage code, which a null engine also returns.
pub(crate) const USAGE: i32 = abi::THINKTHEN_EUSAGE;
/// The header's defect code, the guard's answer when a panic reached it.
pub(crate) const DEFECT: i32 = abi::THINKTHEN_EDEFECT;

/// The header's code for a kind, in the header's own order.
const fn code_of(kind: ErrorKind) -> i32 {
    match kind {
        ErrorKind::Usage => USAGE,
        ErrorKind::Backend => abi::THINKTHEN_EBACKEND,
        ErrorKind::Deadline => abi::THINKTHEN_EDEADLINE,
        ErrorKind::Local => abi::THINKTHEN_ELOCAL,
        ErrorKind::Cancelled => abi::THINKTHEN_ECANCELLED,
        ErrorKind::Defect => DEFECT,
    }
}

/// One failure the door reports: its code, the retry signal, and a message
/// safe to log.
pub(crate) struct Failure {
    code: i32,
    retryable: bool,
    message: String,
    facts: Option<Box<Facts>>,
    stopped: Option<thinkthen::Stopped>,
}

impl fmt::Debug for Failure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Failure")
            .field("code", &self.code)
            .field("retryable", &self.retryable)
            .field("message", &self.message)
            .field("facts", &self.facts.is_some())
            .finish()
    }
}

/// The count-only report for one completed asking call. Its members are
/// numbers and strings, so writing it cannot fail.
pub(crate) fn facts_json(facts: &Facts) -> String {
    serde_json::to_string(facts).unwrap_or_default()
}

impl Failure {
    /// A refusal of the door's own argument rules.
    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self {
            code: USAGE,
            retryable: false,
            message: message.into(),
            facts: None,
            stopped: None,
        }
    }

    /// A named local question file could not supply a valid question.
    pub(crate) fn local(message: impl Into<String>) -> Self {
        Self {
            code: code_of(ErrorKind::Local),
            retryable: false,
            message: message.into(),
            facts: None,
            stopped: None,
        }
    }

    /// A broken promise of the door or the engine beneath it.
    pub(crate) fn defect(message: &str) -> Self {
        Self {
            code: DEFECT,
            retryable: false,
            message: format!("defect: {message}"),
            facts: None,
            stopped: None,
        }
    }

    pub(crate) fn completed(mut self, facts: Option<&Facts>) -> Self {
        if self.facts.is_none() {
            self.facts = facts.map(|facts| Box::new(facts.clone()));
        }
        self
    }
}

impl From<thinkthen::Error> for Failure {
    fn from(error: thinkthen::Error) -> Self {
        Self {
            code: code_of(error.kind()),
            retryable: error.retryable(),
            message: error.to_string(),
            facts: error.facts().map(|facts| Box::new(facts.clone())),
            stopped: Some(error.stopped()),
        }
    }
}

/// The stored failure behind `thinkthen_error_message`.
pub(crate) struct Last {
    code: i32,
    retryable: bool,
    message: CString,
    facts: Option<CString>,
    native_facts: Option<Box<Facts>>,
    stopped: Option<thinkthen::Stopped>,
}

impl Last {
    fn of(failure: Failure) -> Self {
        let message = CString::new(failure.message)
            .unwrap_or_else(|_| c"defect: the message held a NUL".to_owned());
        Self {
            code: failure.code,
            retryable: failure.retryable,
            message,
            facts: failure
                .facts
                .as_ref()
                .and_then(|facts| CString::new(facts_json(facts)).ok()),
            native_facts: failure.facts,
            stopped: failure.stopped,
        }
    }
}

type Table = Mutex<HashMap<ThreadId, Last>>;

/// The engine a host holds, with its own failure table. A failure on one
/// engine never answers for another, and the table dies with the engine, so
/// a new engine at a reused address starts clean.
pub(crate) struct Held {
    pub(crate) engine: Engine,
    failures: Arc<Table>,
}

impl fmt::Debug for Held {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Held").finish_non_exhaustive()
    }
}

/// The tables the calling thread holds an entry in, held weakly, so the
/// thread's entries leave when it exits and the table stays one entry per
/// live thread (R3-24).
struct Departures {
    thread: ThreadId,
    tables: Vec<Weak<Table>>,
}

impl Drop for Departures {
    fn drop(&mut self) {
        for table in self.tables.drain(..).filter_map(|table| table.upgrade()) {
            lock(&table).remove(&self.thread);
        }
    }
}

thread_local! {
    static DEPARTURES: RefCell<Departures> = RefCell::new(Departures {
        thread: std::thread::current().id(),
        tables: Vec::new(),
    });
    /// The calling thread's last failed `thinkthen_engine_new`, which the
    /// error functions report for a null engine. A built engine clears it.
    static UNBUILT: RefCell<Option<Last>> = const { RefCell::new(None) };
}

/// Build an engine and keep it, or record why none came for the calling
/// thread; a panic while building records the defect kind.
pub(crate) fn built<E: Into<Failure>>(build: impl FnOnce() -> Result<Engine, E>) -> Option<Engine> {
    let (engine, last) = match contained(build) {
        Some(Ok(engine)) => (Some(engine), None),
        Some(Err(error)) => (None, Some(error.into())),
        None => (None, Some(Failure::defect("a panic built no engine"))),
    };
    // After teardown the slot is gone, and a null engine reads no failure.
    let _ = UNBUILT.try_with(|slot| slot.replace(last.map(Last::of)));
    engine
}

/// Read the calling thread's last failed build, if one is held.
fn unbuilt<T>(read: impl Fn(&Last) -> T) -> Option<T> {
    UNBUILT
        .try_with(|slot| slot.borrow().as_ref().map(read))
        .ok()
        .flatten()
}

fn lock(table: &Table) -> std::sync::MutexGuard<'_, HashMap<ThreadId, Last>> {
    table.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Held {
    pub(crate) fn new(engine: Engine) -> Self {
        Self {
            engine,
            failures: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Record a failure for the calling thread and return its code. The
    /// message replaces this thread's previous one on this engine and lives
    /// until this thread's next failure here, or until the engine is freed.
    pub(crate) fn fail(&self, failure: Failure) -> i32 {
        let code = failure.code;
        let last = Last::of(failure);
        let first = lock(&self.failures)
            .insert(std::thread::current().id(), last)
            .is_none();
        if first {
            // A thread in its own teardown has no hook left; its one entry
            // then stays until the engine is freed.
            let _ = DEPARTURES.try_with(|departures| {
                let mut departures = departures.borrow_mut();
                departures.tables.retain(|table| table.strong_count() > 0);
                departures.tables.push(Arc::downgrade(&self.failures));
            });
        }
        code
    }

    /// Record a failure, or return the value.
    pub(crate) fn settle<T>(&self, result: Result<T, Failure>) -> Result<T, i32> {
        result.map_err(|failure| self.fail(failure))
    }

    fn read<T>(&self, read: impl Fn(&Last) -> T) -> Option<T> {
        lock(&self.failures)
            .get(&std::thread::current().id())
            .map(read)
    }
}

/// The calling thread's last failure on `held`, or its last failed build
/// when the engine is null.
fn last<T>(held: Option<&Held>, read: impl Fn(&Last) -> T) -> Option<T> {
    held.map_or_else(|| unbuilt(&read), |held| held.read(&read))
}

/// The last code: [`OK`] when an engine holds none, [`USAGE`] for a null
/// engine with no failed build.
pub(crate) fn code(held: Option<&Held>) -> i32 {
    last(held, |last| last.code).unwrap_or(if held.is_some() { OK } else { USAGE })
}

/// Whether the last failure could pass later; 0 when none is held.
pub(crate) fn retryable(held: Option<&Held>) -> i32 {
    last(held, |last| i32::from(last.retryable)).unwrap_or(0)
}

/// The last message. The pointer stays valid until the calling thread
/// records its next failure on this engine, the engine is freed, or the
/// thread exits. With a null engine, its distinct slot lasts until the
/// next constructor call on this thread or thread exit.
pub(crate) fn message(held: Option<&Held>) -> *const std::ffi::c_char {
    last(held, |last| last.message.as_ptr()).unwrap_or_else(|| {
        if held.is_some() {
            c"no failure yet"
        } else {
            NO_ENGINE
        }
        .as_ptr()
    })
}

/// Borrow the calling thread's last started-failure facts, if present.
pub(crate) fn facts(held: Option<&Held>) -> *const std::ffi::c_char {
    last(held, |last| last.facts.as_ref().map(|facts| facts.as_ptr()))
        .flatten()
        .unwrap_or(std::ptr::null())
}

/// Independent saved-error data; no successful-result provenance is synthesized.
#[derive(Clone)]
pub(crate) struct FailureSnapshot {
    pub(crate) code: i32,
    pub(crate) retryable: bool,
    pub(crate) message: String,
    pub(crate) facts: Option<Box<Facts>>,
    pub(crate) stopped: Option<thinkthen::Stopped>,
}
impl fmt::Debug for FailureSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FailureSnapshot")
            .field("code", &self.code)
            .field("retryable", &self.retryable)
            .field("facts", &self.facts.is_some())
            .finish_non_exhaustive()
    }
}
/// Clone under the existing thread/engine lock; never clears or replaces the saved error.
pub(crate) fn snapshot(held: Option<&Held>) -> Option<FailureSnapshot> {
    last(held, |last| FailureSnapshot {
        code: last.code,
        retryable: last.retryable,
        message: last.message.to_string_lossy().into_owned(),
        facts: last.native_facts.clone(),
        stopped: last.stopped,
    })
}

/// Run one door body so no panic unwinds into the host. A panic records the
/// defect kind on the engine at hand for the calling thread and returns
/// `fallback`. The error path reaches thread-local storage only through
/// `try_with`, so a call after thread-local teardown, such as a free from
/// an `atexit` handler, exits clean (R2-7).
pub(crate) fn guard<T>(held: Option<&Held>, fallback: T, body: impl FnOnce() -> T) -> T {
    contained(body).unwrap_or_else(|| {
        if let Some(held) = held {
            held.fail(Failure::defect("a panic crossed the C door"));
        }
        fallback
    })
}

/// The typed facts door retains a completed call through post-reply defects.
pub(crate) fn guard_completed<T>(
    held: &Held,
    completed: &mut Option<Facts>,
    fallback: T,
    body: impl FnOnce(&mut Option<Facts>) -> T,
) -> T {
    contained(|| body(completed)).unwrap_or_else(|| {
        held.fail(Failure::defect("a panic crossed the C door").completed(completed.as_ref()));
        fallback
    })
}

/// A message the door hands out when no engine table holds one.
pub(crate) const NO_ENGINE: &CStr = c"no engine came, so no failure is named";
/// The message after a panic in `thinkthen_error_message` itself.
pub(crate) const NO_MESSAGE: &CStr = c"the door panicked, so no message came";

#[cfg(test)]
mod tests;
