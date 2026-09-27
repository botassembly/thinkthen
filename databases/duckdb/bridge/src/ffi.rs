//! The private C++ extension bridge. DuckDB values never cross this ABI.
#![allow(unsafe_code, reason = "the bridge copies caller-owned byte ranges")]

use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Once;

use crate::{engines, errors};
use thinkthen::{Answer, CallOptions, LoadedQuestion, Question};

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

fn reply_boundary(call: impl FnOnce() -> Result<Vec<u8>, String>) -> Reply {
    catch_unwind(AssertUnwindSafe(|| in_bridge(|| answered(call())))).unwrap_or(Reply {
        status: 4,
        bytes: std::ptr::null_mut(),
        len: 0,
    })
}

/// Initialize the direct bridge hook at extension load, before bind.
#[unsafe(no_mangle)]
pub(crate) extern "C" fn thinkthen_cpp_init() -> i32 {
    catch_unwind(AssertUnwindSafe(install_hook)).map_or(4, |_| 0)
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
///
/// # Safety
/// `bytes` must name `len` readable bytes through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_question(
    bytes: *const u8,
    len: usize,
) -> Reply {
    reply_boundary(|| {
        let argument = text(bytes, len)?;
        question(argument).map(|_| Vec::new())
    })
}

/// One copied text in a C++ chunk group.
#[repr(C)]
#[derive(Debug)]
pub(crate) struct BridgeText {
    bytes: *const u8,
    len: usize,
}

/// Unset numeric SQL settings use `i64::MIN`; every valid setting is larger.
#[repr(C)]
#[derive(Debug)]
pub(crate) struct BridgeSettings {
    throttle: i64,
    max_requests: i64,
    max_requests_total: i64,
}

fn asked(settings: BridgeSettings) -> engines::Asked {
    let present = |value| (value != i64::MIN).then_some(value);
    engines::Asked {
        throttle: present(settings.throttle),
        max_requests: present(settings.max_requests),
        max_requests_total: present(settings.max_requests_total),
        cache: None,
    }
}

/// One real grouped decision, probability, or details call.
///
/// # Safety
/// The question and every entry in `texts` must remain readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_scalar_group(
    question_bytes: *const u8,
    question_len: usize,
    texts: *const BridgeText,
    count: usize,
    deadline_ms: i64,
    kind: i32,
    settings: BridgeSettings,
) -> Reply {
    reply_boundary(|| {
        let question = question(text(question_bytes, question_len)?)?;
        if count > 2048 || (texts.is_null() && count != 0) {
            return Err("thinkthen defect: the bridge got an invalid chunk size".to_owned());
        }
        let copied = if count == 0 {
            Vec::new()
        } else {
            // SAFETY: C++ owns this fixed array until the call returns.
            let rows = unsafe { std::slice::from_raw_parts(texts, count) };
            rows.iter()
                .map(|row| text(row.bytes, row.len).map(str::to_owned))
                .collect::<Result<Vec<_>, _>>()?
        };
        let asked = asked(settings);
        let engine = engines::engine_for(&asked, |_| engines::Probe::Allowed)?;
        let (copied, cut) = engines::within_total(&asked, copied)?;
        let options = || {
            if deadline_ms == -1 {
                Ok(CallOptions::new())
            } else {
                CallOptions::new()
                    .deadline_millis(deadline_ms)
                    .map_err(|error| errors::RowError::from(error).text)
            }
        };
        if kind == 2 {
            let mut bytes = Vec::new();
            for text in copied {
                let result = match &question {
                    LoadedQuestion::Question(held) => engine.details_with(held, &text, options()?),
                    LoadedQuestion::Banded(held) => engine.details_with(held, &text, options()?),
                }
                .map_err(|error| errors::RowError::from(error).text)?;
                let json = result.to_json();
                let len = u32::try_from(json.len())
                    .map_err(|_| "thinkthen defect: a details value is too large".to_owned())?;
                bytes.extend_from_slice(&len.to_ne_bytes());
                bytes.extend_from_slice(json.as_bytes());
            }
            if let Some(error) = cut {
                return Err(error);
            }
            return Ok(bytes);
        }
        if kind != 0 && kind != 1 {
            return Err("thinkthen defect: the bridge got an unknown scalar kind".to_owned());
        }
        let answers: Vec<(Answer, f64)> = match &question {
            LoadedQuestion::Question(held) => engine
                .decide_many_with(held, copied, options()?)
                .map(|row| row.map(|row| (*row.value(), row.probability())))
                .collect::<Result<Vec<_>, _>>(),
            LoadedQuestion::Banded(held) => engine
                .decide_many_with(held, copied, options()?)
                .map(|row| row.map(|row| (*row.value(), row.probability())))
                .collect::<Result<Vec<_>, _>>(),
        }
        .map_err(|error| errors::RowError::from(error).text)?;
        if let Some(error) = cut {
            return Err(error);
        }
        if kind == 1 {
            Ok(answers
                .into_iter()
                .flat_map(|(_, value)| value.to_ne_bytes())
                .collect())
        } else {
            Ok(answers
                .into_iter()
                .map(|(answer, _)| match answer {
                    Answer::No => 0,
                    Answer::Yes => 1,
                    Answer::Unsure => 2,
                })
                .collect())
        }
    })
}
