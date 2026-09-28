//! The private C++ extension bridge. DuckDB values never cross this ABI.
#![allow(unsafe_code, reason = "the bridge copies caller-owned byte ranges")]

use std::ffi::c_void;
use std::sync::mpsc::{RecvTimeoutError, channel};
use std::time::Duration;

use crate::{engines, errors, signal};
use thinkthen::{CancelToken, LoadedQuestion, Question, QuestionSet};

#[path = "ffi/complete_listed/ffi.rs"]
mod complete_listed;
mod find;
mod listed;
mod nested;
mod panic;
#[path = "ffi/scalar/ffi.rs"]
mod scalar;
mod settings;
#[cfg(test)]
mod tests;

pub(crate) use settings::{BridgeSettings, asked, batch, probe};

unsafe extern "C" {
    fn thinkthen_cpp_interrupt_busy();
}

fn interrupt_relate() {
    // SAFETY: the C++ callback is no-throw and touches only live registry entries.
    unsafe { thinkthen_cpp_interrupt_busy() };
}

fn guarded<T>(call: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    panic::caught(call).unwrap_or_else(|_| Err("thinkthen defect: the bridge panicked".to_owned()))
}

/// A Rust-owned reply; C++ copies it before calling `thinkthen_cpp_free`.
#[repr(C)]
#[derive(Debug)]
pub(crate) struct Reply {
    status: i32,
    bytes: *mut u8,
    len: usize,
}

fn reply(status: i32, bytes: &[u8]) -> Reply {
    let mut owned = bytes.to_vec().into_boxed_slice();
    let value = Reply {
        status,
        bytes: owned.as_mut_ptr(),
        len: owned.len(),
    };
    std::mem::forget(owned);
    value
}

fn answered(result: Result<Vec<u8>, String>) -> Reply {
    match result {
        Ok(value) => reply(0, &value),
        Err(error) => reply(1, error.as_bytes()),
    }
}

pub(crate) fn reply_boundary(call: impl FnOnce() -> Result<Vec<u8>, String>) -> Reply {
    panic::caught(|| answered(call())).unwrap_or(Reply {
        status: 4,
        bytes: std::ptr::null_mut(),
        len: 0,
    })
}

/// Validate one whole find row before any row in its chunk can send.
///
/// # Safety
/// The caller retains the question and unit byte ranges through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_find(
    question: *const u8,
    question_len: usize,
    units: *const BridgeText,
    count: usize,
    none: i32,
) -> Reply {
    reply_boundary(|| find::validate(question, question_len, units, count, none))
}

/// Evaluate one owned find set, returning its original-index result frame.
///
/// # Safety
/// The C++ caller retains the question, units, settings and stop predicate through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_find(
    question: *const u8,
    question_len: usize,
    units: *const BridgeText,
    count: usize,
    none: i32,
    deadline_ms: i64,
    settings: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        find::run(find::Input {
            question,
            question_len,
            units,
            count,
            none,
            deadline_ms,
            settings,
            stop,
        })
    })
}

/// Initialize the direct bridge hook at extension load, before bind.
#[unsafe(no_mangle)]
pub(crate) extern "C" fn thinkthen_cpp_init() -> i32 {
    panic::caught(|| {
        signal::install();
        signal::start_bridge(interrupt_relate);
    })
    .map_or(4, |_| 0)
}

/// Start the host-signal scope for one SQL statement.
#[unsafe(no_mangle)]
pub(crate) extern "C" fn thinkthen_cpp_query_begin() -> *mut signal::Invoke {
    panic::caught(|| Box::into_raw(Box::new(signal::Invoke::begin())))
        .unwrap_or(std::ptr::null_mut())
}

/// Read one still-owned statement's host-signal scope.
///
/// # Safety
/// `scope` is a live value returned by `thinkthen_cpp_query_begin`.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_query_stopped(scope: *const signal::Invoke) -> i32 {
    panic::caught(|| {
        // SAFETY: the C++ statement owner retains this value through the call.
        unsafe { scope.as_ref() }.is_none_or(signal::Invoke::stopped)
    })
    .map_or(1, i32::from)
}

/// End one SQL statement's host-signal scope.
///
/// # Safety
/// `scope` is a live value returned by `thinkthen_cpp_query_begin`, freed once.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_query_end(scope: *mut signal::Invoke) {
    let _ = panic::caught(|| {
        if !scope.is_null() {
            // SAFETY: the C++ statement owner calls end once on this allocation.
            drop(unsafe { Box::from_raw(scope) });
        }
    });
}

/// Free one returned byte buffer, including an error reply.
///
/// # Safety
/// `bytes` and `len` must be the live buffer from one reply, freed once.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_free(bytes: *mut u8, len: usize) {
    let _ = guarded(|| {
        if !bytes.is_null() {
            // SAFETY: `reply` leaks exactly this boxed slice to the caller.
            drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(bytes, len)) });
        }
        Ok(())
    });
}

pub(crate) fn text<'a>(bytes: *const u8, len: usize) -> Result<&'a str, String> {
    if bytes.is_null() {
        return Err("thinkthen defect: the bridge got a null text pointer".to_owned());
    }
    // SAFETY: C++ owns this byte range until the call returns.
    let slice = unsafe { std::slice::from_raw_parts(bytes, len) };
    std::str::from_utf8(slice)
        .map_err(|_| "thinkthen usage: a text argument is not UTF-8".to_owned())
}

pub(crate) fn question_typed(
    argument: &str,
    from_file: bool,
) -> Result<LoadedQuestion, errors::RowError> {
    if from_file {
        Question::from_json(argument).map_err(|error| {
            if error.kind() == thinkthen::ErrorKind::Usage {
                errors::RowError::local(error.detail().message())
            } else {
                error.into()
            }
        })
    } else if argument.starts_with('@') {
        Err(errors::RowError::local(
            "the question file was not read by this database",
        ))
    } else if argument.starts_with('{') {
        Question::from_json(argument).map_err(Into::into)
    } else {
        Question::decide(argument)
            .map(|built| LoadedQuestion::Question(built.cut()))
            .map_err(Into::into)
    }
}

fn set_typed(argument: &str, from_file: bool) -> Result<QuestionSet, errors::RowError> {
    if from_file {
        QuestionSet::from_json(argument).map_err(|error| {
            if error.kind() == thinkthen::ErrorKind::Usage {
                errors::RowError::local(error.detail().message())
            } else {
                error.into()
            }
        })
    } else if argument.starts_with('@') {
        Err(errors::RowError::local(
            "the question file was not read by this database",
        ))
    } else if argument.starts_with('{') {
        QuestionSet::from_json(argument).map_err(Into::into)
    } else {
        Err(errors::RowError::usage(
            "annotate names a question set as '@form.json' or JSON",
        ))
    }
}

/// Validate a foldable question without reading a key or sending a request.
///
/// # Safety
/// `bytes` must name `len` readable bytes through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_question(
    bytes: *const u8,
    len: usize,
    from_file: i32,
) -> Reply {
    reply_boundary(|| {
        let argument = text(bytes, len)?;
        question_typed(argument, from_file != 0)
            .map(|_| Vec::new())
            .map_err(|error| error.text)
    })
}

/// Validate a foldable question set before any backend request.
///
/// # Safety
/// `bytes` must name `len` readable bytes through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_set(
    bytes: *const u8,
    len: usize,
    from_file: i32,
) -> Reply {
    reply_boundary(|| {
        let argument = text(bytes, len)?;
        set_typed(argument, from_file != 0)
            .map(|_| Vec::new())
            .map_err(|error| error.text)
    })
}

/// One copied text in a C++ chunk group.
#[repr(C)]
#[derive(Debug)]
pub(crate) struct BridgeText {
    pub(crate) bytes: *const u8,
    pub(crate) len: usize,
}

/// A no-throw query interrupt predicate, valid for the synchronous bridge call.
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct BridgeStop {
    context: *mut c_void,
    interrupted: Option<extern "C" fn(*mut c_void) -> i32>,
}

#[derive(Clone, Copy)]
pub(crate) struct CallScope<'a> {
    pub(crate) due: i64,
    pub(crate) token: &'a CancelToken,
    pub(crate) total: Option<i64>,
    pub(crate) batch: Option<&'a str>,
    pub(crate) context: Option<&'a str>,
}

// SAFETY: the public engine invokes the host check only on the FFI calling
// thread. The C++ context outlives that synchronous call; the callback catches
// its own exceptions before returning across the ABI.
unsafe impl Sync for BridgeStop {}

impl BridgeStop {
    pub(crate) fn stopped(self) -> bool {
        self.interrupted
            .is_some_and(|check| check(self.context) != 0)
    }
}

/// Keep DuckDB's callback on its calling thread while a blocking model send
/// runs on an owned worker. A stopped query detaches only that worker; its
/// cancel token prevents new sends or retries after the held attempt ends.
pub(crate) fn run_detached<T: Send + 'static>(
    stop: BridgeStop,
    work: impl FnOnce(CancelToken) -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    let cancelled = || "thinkthen cancelled: the call was cancelled".to_owned();
    if stop.stopped() {
        return Err(cancelled());
    }
    let token = CancelToken::new();
    let owned = token.clone();
    let (sender, receiver) = channel();
    std::thread::Builder::new()
        .name("thinkthen-duckdb-call".to_owned())
        .spawn(move || {
            let result = panic::caught(|| work(owned))
                .unwrap_or_else(|_| Err("thinkthen defect: the engine worker panicked".to_owned()));
            let _ = sender.send(result);
        })
        .map_err(|_| "thinkthen defect: the engine worker could not start".to_owned())?;
    loop {
        if stop.stopped() {
            token.cancel();
            return Err(cancelled());
        }
        match receiver.recv_timeout(Duration::from_millis(10)) {
            Ok(result) => {
                if stop.stopped() {
                    token.cancel();
                    return Err(cancelled());
                }
                return result;
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                return Err("thinkthen defect: the engine worker ended with no answer".to_owned());
            }
        }
    }
}

/// Validate one listed question and its members before any group sends.
///
/// # Safety
/// The question and each member must stay readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_listed(
    question: *const u8,
    question_len: usize,
    members: *const BridgeText,
    member_count: usize,
    kind: i32,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let members = copied_texts(members, member_count)?;
        listed::set(kind, question, &members).map(|_| Vec::new())
    })
}

pub(crate) fn copied_texts(rows: *const BridgeText, count: usize) -> Result<Vec<String>, String> {
    if count > 2048 || (rows.is_null() && count != 0) {
        return Err("thinkthen defect: the bridge got an invalid chunk size".to_owned());
    }
    if count == 0 {
        return Ok(Vec::new());
    }
    // SAFETY: C++ owns the fixed array through the synchronous call.
    let rows = unsafe { std::slice::from_raw_parts(rows, count) };
    rows.iter()
        .map(|row| text(row.bytes, row.len).map(str::to_owned))
        .collect()
}

/// Validate one recognize kind list or relation question before any send.
///
/// # Safety
/// The argument and members must stay readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_nested(
    argument: *const u8,
    argument_len: usize,
    members: *const BridgeText,
    member_count: usize,
    kind: i32,
    from_file: i32,
) -> Reply {
    reply_boundary(|| {
        let argument = text(argument, argument_len)?;
        let members = copied_texts(members, member_count)?;
        nested::ask(kind, argument, &members, from_file != 0).map(|_| Vec::new())
    })
}

/// Evaluate one grouped recognize or relations call through the public engine.
///
/// # Safety
/// The argument, members, and texts must stay readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_nested_group(
    argument: *const u8,
    argument_len: usize,
    members: *const BridgeText,
    member_count: usize,
    texts: *const BridgeText,
    text_count: usize,
    deadline_ms: i64,
    kind: i32,
    settings: BridgeSettings,
    from_file: i32,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let argument = text(argument, argument_len)?;
        let members = copied_texts(members, member_count)?;
        let texts = copied_texts(texts, text_count)?;
        let ask = nested::ask(kind, argument, &members, from_file != 0)?;
        let asked = asked(&settings)?;
        let engine = engines::engine_for(&asked, |path| probe(&settings, path))?;
        let (texts, cut) = engines::within_total(&asked, texts)?;
        let total = asked.max_requests_total;
        run_detached(stop, move |token| {
            let scope = CallScope {
                due: deadline_ms,
                token: &token,
                total,
                batch: None,
                context: None,
            };
            let values = nested::run(&engine, &ask, texts, kind, scope)?;
            if let Some(error) = cut {
                return Err(error);
            }
            Ok(values)
        })
    })
}

/// Evaluate one listed group through the public engine.
///
/// # Safety
/// The question, members, and texts must stay readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_listed_group(
    question: *const u8,
    question_len: usize,
    members: *const BridgeText,
    member_count: usize,
    texts: *const BridgeText,
    text_count: usize,
    deadline_ms: i64,
    kind: i32,
    settings: BridgeSettings,
    context_bytes: *const u8,
    context_len: usize,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let members = copied_texts(members, member_count)?;
        let texts = copied_texts(texts, text_count)?;
        let context = (!context_bytes.is_null())
            .then(|| text(context_bytes, context_len).map(str::to_owned))
            .transpose()?;
        let batch = batch(&settings)?;
        let set = listed::set(kind, question, &members)?;
        let asked = asked(&settings)?;
        let engine = engines::engine_for(&asked, |path| probe(&settings, path))?;
        let total = asked.max_requests_total;
        run_detached(stop, move |token| {
            let options = engines::options_for(
                deadline_ms,
                &token,
                total,
                batch.as_deref(),
                context.as_deref(),
            )?;
            let values = listed::run(&engine, &set, texts, options, total)?;
            Ok(values)
        })
    })
}
