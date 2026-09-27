//! The private C++ extension bridge. DuckDB values never cross this ABI.
#![allow(unsafe_code, reason = "the bridge copies caller-owned byte ranges")]

use std::cell::Cell;
use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Once;

use thinkthen::{Answer, CallOptions, LoadedQuestion, Question};

#[allow(
    dead_code,
    reason = "the bridge shares the registry while the other verbs move"
)]
#[path = "../../src/engines.rs"]
mod engines;

mod errors {
    use thinkthen::{Error, ErrorKind};

    #[derive(Debug)]
    pub(super) struct RowError {
        pub(super) text: String,
    }

    impl RowError {
        pub(super) fn usage(message: &str) -> Self {
            Self {
                text: usage(message),
            }
        }
    }

    impl From<Error> for RowError {
        fn from(error: Error) -> Self {
            Self {
                text: format!("{}{}", prefix(error.kind()), error.detail().message()),
            }
        }
    }

    pub(super) fn usage(message: &str) -> String {
        format!("thinkthen usage: {message}")
    }

    fn prefix(kind: ErrorKind) -> &'static str {
        match kind {
            ErrorKind::Usage => "thinkthen usage: ",
            ErrorKind::Backend => "thinkthen backend: ",
            ErrorKind::Local => "thinkthen local: ",
            ErrorKind::Cancelled => "thinkthen cancelled: ",
            ErrorKind::Deadline => "thinkthen deadline: ",
            ErrorKind::Defect => "thinkthen defect: ",
        }
    }
}

thread_local! {
    static BRIDGE_DEPTH: Cell<usize> = const { Cell::new(0) };
}

static HOOK: Once = Once::new();

fn install_hook() {
    HOOK.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if !BRIDGE_DEPTH
                .try_with(|depth| depth.get() > 0)
                .unwrap_or(false)
            {
                previous(info);
            }
        }));
    });
}

struct BridgeDepth(usize);

impl Drop for BridgeDepth {
    fn drop(&mut self) {
        BRIDGE_DEPTH.with(|depth| depth.set(self.0));
    }
}

fn in_bridge<T>(call: impl FnOnce() -> T) -> T {
    install_hook();
    let prior = BRIDGE_DEPTH.with(|depth| {
        let prior = depth.get();
        depth.set(prior.saturating_add(1));
        prior
    });
    let _restore = BridgeDepth(prior);
    call()
}

fn guarded<T>(call: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    catch_unwind(AssertUnwindSafe(|| in_bridge(call)))
        .unwrap_or_else(|_| Err("thinkthen defect: the bridge panicked".to_owned()))
}

/// A Rust-owned reply; C++ copies it before calling `thinkthen_cpp_free`.
#[repr(C)]
#[derive(Debug)]
pub struct Reply {
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

fn answered(result: Result<u8, String>) -> Reply {
    match result {
        Ok(value) => reply(0, &[value]),
        Err(error) => reply(1, error.as_bytes()),
    }
}

fn reply_boundary(call: impl FnOnce() -> Result<u8, String>) -> Reply {
    catch_unwind(AssertUnwindSafe(|| in_bridge(|| answered(call())))).unwrap_or(Reply {
        status: 4,
        bytes: std::ptr::null_mut(),
        len: 0,
    })
}

/// Initialize the direct bridge hook at extension load, before bind.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_cpp_init() -> i32 {
    catch_unwind(AssertUnwindSafe(install_hook)).map_or(4, |_| 0)
}

/// Free one returned byte buffer, including an error reply.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_cpp_free(bytes: *mut u8, len: usize) {
    let _ = guarded(|| {
        if !bytes.is_null() {
            // SAFETY: `reply` leaks exactly this boxed slice to the caller.
            drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(bytes, len)) });
        }
        Ok(())
    });
}

fn text<'a>(bytes: *const u8, len: usize) -> Result<&'a str, String> {
    if bytes.is_null() {
        return Err("thinkthen defect: the bridge got a null text pointer".to_owned());
    }
    // SAFETY: C++ owns this byte range until the call returns.
    let slice = unsafe { std::slice::from_raw_parts(bytes, len) };
    std::str::from_utf8(slice)
        .map_err(|_| "thinkthen usage: a text argument is not UTF-8".to_owned())
}

fn question(argument: &str) -> Result<LoadedQuestion, String> {
    if argument.starts_with('@') {
        Err("thinkthen local: question files are not yet enabled in this C++ candidate".to_owned())
    } else if argument.starts_with('{') {
        Question::from_json(argument).map_err(|error| errors::RowError::from(error).text)
    } else {
        Question::decide(argument)
            .map(|built| LoadedQuestion::Question(built.cut()))
            .map_err(|error| errors::RowError::from(error).text)
    }
}

/// Validate a foldable question without reading a key or sending a request.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_cpp_validate_question(bytes: *const u8, len: usize) -> Reply {
    reply_boundary(|| {
        let argument = text(bytes, len)?;
        question(argument).map(|_| 0)
    })
}

/// One real decision call. The caller currently supplies no SQL settings.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_cpp_decide(
    question_bytes: *const u8,
    question_len: usize,
    evidence_bytes: *const u8,
    evidence_len: usize,
    deadline_ms: i64,
    _caller: *mut c_void,
) -> Reply {
    reply_boundary(|| {
        let question = question(text(question_bytes, question_len)?)?;
        let evidence = text(evidence_bytes, evidence_len)?;
        let engine = engines::engine_for(&engines::Asked::default(), |_| engines::Probe::Allowed)?;
        let options = if deadline_ms == -1 {
            CallOptions::new()
        } else {
            CallOptions::new()
                .deadline_millis(deadline_ms)
                .map_err(|error| errors::RowError::from(error).text)?
        };
        let answered = match &question {
            LoadedQuestion::Question(held) => engine.decide_with(held, evidence, options),
            LoadedQuestion::Banded(held) => engine.decide_with(held, evidence, options),
        }
        .map_err(|error| errors::RowError::from(error).text)?;
        Ok(match answered {
            Answer::No => 0,
            Answer::Yes => 1,
            Answer::Unsure => 2,
        })
    })
}
