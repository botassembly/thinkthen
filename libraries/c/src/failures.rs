//! The error surface: the six codes, one failure table per engine keyed by
//! the recording thread, and the panic guard every exported symbol runs
//! behind.

use crate::ffi::values as abi;
use crate::session::views::{Node, Storage, thinkthen_complete_utf8_v1, thinkthen_error_view_v1};
use crate::session::views_generated::read_thinkthen_complete_immediate_facts_v1;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString, c_char};
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

use crate::ffi::session::{failure_code as code_of, failure_message};

/// One failure the interface reports: its code, the retry signal, and a message
/// safe to log.
pub(crate) struct Failure {
    code: i32,
    retryable: bool,
    message: String,
    facts: Option<Box<Facts>>,
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
    pub(crate) fn carrier_overflow() -> Self {
        Self {
            code: crate::ffi::values::THINKTHEN_COBOL_OVERFLOW,
            ..Self::usage("a counted COBOL scalar exceeds its representation capacity")
        }
    }
    /// A refusal of the interface's own argument rules.
    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self {
            code: USAGE,
            retryable: false,
            message: message.into(),
            facts: None,
        }
    }

    /// A broken promise of the interface or the engine beneath it.
    pub(crate) fn defect(message: &str) -> Self {
        Self {
            code: DEFECT,
            retryable: false,
            message: format!("defect: {message}"),
            facts: None,
        }
    }
}

impl From<ErrorKind> for Failure {
    fn from(kind: ErrorKind) -> Self {
        thinkthen::Error::new(kind, failure_message(kind)).into()
    }
}

impl From<thinkthen::Error> for Failure {
    fn from(error: thinkthen::Error) -> Self {
        Self {
            code: code_of(error.kind()),
            retryable: error.retryable(),
            message: error.to_string(),
            facts: error.facts().map(|facts| Box::new(facts.clone())),
        }
    }
}

/// The stored failure behind `thinkthen_error_message`.
pub(crate) struct Last {
    code: i32,
    retryable: bool,
    message: CString,
    facts: Option<CString>,
    view: Box<thinkthen_error_view_v1>,
    _storage: Storage,
}

impl Last {
    pub(crate) fn of(failure: Failure) -> Self {
        let message = CString::new(failure.message)
            .unwrap_or_else(|_| c"defect: the message held a NUL".to_owned());
        let mut storage = Storage::default();
        let facts = failure
            .facts
            .as_ref()
            .and_then(|facts| {
                let text = facts
                    .complete()
                    .map_or_else(
                        || serde_json::to_string(facts),
                        |facts| serde_json::to_string(&facts),
                    )
                    .ok()?;
                let node = Node::parse(&text).ok()?;
                let facts = read_thinkthen_complete_immediate_facts_v1(&node, &mut storage).ok()?;
                Some(storage.hold(facts))
            })
            .unwrap_or(std::ptr::null());
        let view = Box::new(thinkthen_error_view_v1 {
            code: failure.code,
            retryable: i32::from(failure.retryable),
            message: thinkthen_complete_utf8_v1 {
                data: message.as_ptr(),
                len: message.as_bytes().len(),
            },
            facts,
        });
        Self {
            view,
            _storage: storage,
            code: failure.code,
            retryable: failure.retryable,
            message,
            facts: failure
                .facts
                .as_ref()
                .and_then(|facts| CString::new(facts_json(facts)).ok()),
        }
    }
    pub(crate) fn view(&self) -> *const thinkthen_error_view_v1 {
        std::ptr::from_ref(self.view.as_ref())
    }
    pub(crate) fn message(&self) -> *const c_char {
        self.message.as_ptr()
    }
    pub(crate) fn retryable(&self) -> i32 {
        i32::from(self.retryable)
    }
    pub(crate) fn facts(&self) -> *const c_char {
        self.facts
            .as_ref()
            .map_or(std::ptr::null(), |facts| facts.as_ptr())
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
    /// Immediate session failures remain independent from constructors and engines.
    static SESSION: RefCell<Option<Last>> = const { RefCell::new(None) };
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
    last(held, Last::retryable).unwrap_or(0)
}

/// The last message. The pointer stays valid until the calling thread
/// records its next failure on this engine, the engine is freed, or the
/// thread exits. With a null engine, its distinct slot lasts until the
/// next constructor call on this thread or thread exit.
pub(crate) fn message(held: Option<&Held>) -> *const std::ffi::c_char {
    last(held, Last::message).unwrap_or_else(|| {
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
    last(held, Last::facts).unwrap_or(std::ptr::null())
}

/// Run one interface body so no panic unwinds into the host. A panic records the
/// defect kind on the engine at hand for the calling thread and returns
/// `fallback`. The error path reaches thread-local storage only through
/// `try_with`, so a call after thread-local teardown, such as a free from
/// an `atexit` handler, exits clean (R2-7).
pub(crate) fn guard<T>(held: Option<&Held>, fallback: T, body: impl FnOnce() -> T) -> T {
    protect(fallback, body, || {
        if let Some(held) = held {
            held.fail(Failure::defect("a panic crossed the C interface"));
        }
    })
}

fn protect<T>(fallback: T, body: impl FnOnce() -> T, failed: impl FnOnce()) -> T {
    contained(body).unwrap_or_else(|| {
        failed();
        fallback
    })
}

/// Record an immediate session failure without changing either engine slot.
fn session_fail(failure: Failure) -> i32 {
    let code = failure.code;
    let _ = SESSION.try_with(|slot| {
        if let Ok(mut slot) = slot.try_borrow_mut() {
            *slot = Some(Last::of(failure));
        }
    });
    code
}

/// Run a session operation with fixed diagnostics for its error kind.
pub(crate) fn session_call(body: impl FnOnce() -> Result<(), ErrorKind>) -> i32 {
    session_native_call(|| body().map_err(Failure::from))
}

/// Run a session operation with its owned native diagnostics.
pub(crate) fn session_native_call(body: impl FnOnce() -> Result<(), Failure>) -> i32 {
    session_guard(DEFECT, || match body() {
        Ok(()) => OK,
        Err(failure) => session_fail(failure),
    })
}

/// Contain a session panic and record the fixed session defect message.
pub(crate) fn session_guard<T>(fallback: T, body: impl FnOnce() -> T) -> T {
    protect(fallback, body, || {
        session_fail(ErrorKind::Defect.into());
    })
}

fn session_last<T>(fallback: T, inspect: impl Fn(&Last) -> T) -> T {
    SESSION
        .try_with(|slot| {
            slot.try_borrow()
                .ok()
                .and_then(|held| held.as_ref().map(inspect))
        })
        .ok()
        .flatten()
        .unwrap_or(fallback)
}

/// Borrow the immediate session message until the calling thread's next failure.
pub(crate) fn session_message() -> *const c_char {
    session_last(c"no session failure yet".as_ptr(), Last::message)
}

/// Whether the immediate session failure could pass later.
pub(crate) fn session_retryable() -> i32 {
    session_last(0, Last::retryable)
}

/// Borrow the immediate session failure's count-only facts, if present.
pub(crate) fn session_facts() -> *const c_char {
    session_last(std::ptr::null(), Last::facts)
}

/// A message the interface hands out when no engine table holds one.
pub(crate) const NO_ENGINE: &CStr = c"no engine came, so no failure is named";
/// The message after a panic in `thinkthen_error_message` itself.
pub(crate) const NO_MESSAGE: &CStr = c"the interface panicked, so no message came";

#[cfg(test)]
mod tests;

/// Borrow the immutable last failure; NULL means this slot holds no failure.
pub(crate) fn view(held: Option<&Held>) -> *const thinkthen_error_view_v1 {
    last(held, Last::view).unwrap_or(std::ptr::null())
}
/// Borrow the independent immediate session failure, or NULL before a failure.
pub(crate) fn session_view() -> *const thinkthen_error_view_v1 {
    session_last(std::ptr::null(), Last::view)
}
