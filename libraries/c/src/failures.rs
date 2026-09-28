//! The error surface: the six codes, one failure table per engine keyed by
//! the recording thread, and the panic guard every exported symbol runs
//! behind.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Mutex, PoisonError, Weak};
use std::thread::ThreadId;

use thinkthen::{Engine, ErrorKind};

/// The header's success code.
pub(crate) const OK: i32 = 0;
/// The header's usage code, which a null engine also returns.
pub(crate) const USAGE: i32 = 1;
/// The header's defect code, the guard's answer when a panic reached it.
pub(crate) const DEFECT: i32 = 6;

/// The header's code for a kind, in the header's own order.
const fn code_of(kind: ErrorKind) -> i32 {
    match kind {
        ErrorKind::Usage => USAGE,
        ErrorKind::Backend => 2,
        ErrorKind::Deadline => 3,
        ErrorKind::Local => 4,
        ErrorKind::Cancelled => 5,
        ErrorKind::Defect => DEFECT,
    }
}

/// One failure the door reports: its code, the retry signal, and a message
/// safe to log.
#[derive(Debug)]
pub(crate) struct Failure {
    code: i32,
    retryable: bool,
    message: String,
}

impl Failure {
    /// A refusal of the door's own argument rules.
    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self {
            code: USAGE,
            retryable: false,
            message: message.into(),
        }
    }

    /// A broken promise of the door or the engine beneath it.
    pub(crate) fn defect(message: &str) -> Self {
        Self {
            code: DEFECT,
            retryable: false,
            message: format!("defect: {message}"),
        }
    }
}

impl From<thinkthen::Error> for Failure {
    fn from(error: thinkthen::Error) -> Self {
        Self {
            code: code_of(error.kind()),
            retryable: error.retryable(),
            message: error.to_string(),
        }
    }
}

/// The stored failure behind `thinkthen_error_message`.
pub(crate) struct Last {
    code: i32,
    retryable: bool,
    message: CString,
}

impl Last {
    fn of(failure: Failure) -> Self {
        let message = CString::new(failure.message)
            .unwrap_or_else(|_| c"defect: the message held a NUL".to_owned());
        Self {
            code: failure.code,
            retryable: failure.retryable,
            message,
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
    let (engine, last) = match catch_unwind(AssertUnwindSafe(build)) {
        Ok(Ok(engine)) => (Some(engine), None),
        Ok(Err(error)) => (None, Some(error.into())),
        Err(_) => (None, Some(Failure::defect("a panic built no engine"))),
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
/// records its next failure on this engine, or with a null engine until its
/// next `thinkthen_engine_new`: only this thread replaces its own entry.
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

/// Run one door body so no panic unwinds into the host. A panic records the
/// defect kind on the engine at hand for the calling thread and returns
/// `fallback`. The error path reaches thread-local storage only through
/// `try_with`, so a call after thread-local teardown, such as a free from
/// an `atexit` handler, exits clean (R2-7).
pub(crate) fn guard<T>(held: Option<&Held>, fallback: T, body: impl FnOnce() -> T) -> T {
    catch_unwind(AssertUnwindSafe(body)).unwrap_or_else(|payload| {
        if let Some(held) = held {
            let said = payload
                .downcast_ref::<&str>()
                .map(|text| (*text).to_owned())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_default();
            held.fail(Failure::defect(&format!(
                "a panic crossed the C door: {said}"
            )));
        }
        fallback
    })
}

/// A message the door hands out when no engine table holds one.
pub(crate) const NO_ENGINE: &CStr = c"no engine came, so no failure is named";
/// The message after a panic in `thinkthen_error_message` itself.
pub(crate) const NO_MESSAGE: &CStr = c"the door panicked, so no message came";

#[cfg(test)]
mod tests {
    use super::{DEFECT, Failure, Held, USAGE, guard, lock};

    fn held() -> Held {
        let engine = thinkthen::Engine::builder()
            .base_url("http://127.0.0.1:9/v1")
            .map(thinkthen::EngineBuilder::no_cache)
            .and_then(thinkthen::EngineBuilder::build)
            .expect("an engine that sends nothing");
        Held::new(engine)
    }

    /// R3-24: 200,000 failing threads once grew a table to 36 MB. Each
    /// thread's entry leaves with the thread.
    #[test]
    fn a_threads_failure_leaves_the_table_when_the_thread_exits() {
        let engine = std::sync::Arc::new(held());
        for _ in 0..200 {
            let shared = std::sync::Arc::clone(&engine);
            // A plain join waits for the thread's exit hooks.
            let _ =
                std::thread::spawn(move || shared.fail(Failure::usage("a short thread"))).join();
        }
        assert_eq!(
            lock(&engine.failures).len(),
            0,
            "every exited thread's entry left"
        );
        engine.fail(Failure::usage("this thread"));
        assert_eq!(
            lock(&engine.failures).len(),
            1,
            "a live thread keeps its entry"
        );
    }

    /// A panic below the door is the defect code with its own text, and it
    /// is never retryable.
    #[test]
    fn a_panic_behind_the_door_is_the_defect_kind() {
        let engine = held();
        let code = guard(Some(&engine), DEFECT, || -> i32 {
            panic!("the probe panic")
        });
        assert_eq!(
            (
                code,
                super::code(Some(&engine)),
                super::retryable(Some(&engine))
            ),
            (DEFECT, DEFECT, 0)
        );
        let message = message_of(&engine);
        assert_eq!(
            message,
            "defect: a panic crossed the C door: the probe panic"
        );
        assert_eq!(guard(Some(&engine), DEFECT, || USAGE), USAGE);
    }

    fn message_of(engine: &Held) -> String {
        engine
            .read(|last| last.message.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}
