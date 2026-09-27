//! The private C++ extension bridge. DuckDB values never cross this ABI.
#![allow(unsafe_code, reason = "the bridge copies caller-owned byte ranges")]

use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Once;

use crate::{engines, errors};
use thinkthen::{Answer, CallOptions, LoadedQuestion, Question, QuestionSet};

mod listed;
mod nested;

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

pub(crate) fn reply_boundary(call: impl FnOnce() -> Result<Vec<u8>, String>) -> Reply {
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

fn question_typed(argument: &str, from_file: bool) -> Result<LoadedQuestion, errors::RowError> {
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
    cache_bytes: *const u8,
    cache_len: usize,
    cache_allowed: i32,
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

fn copied_texts(rows: *const BridgeText, count: usize) -> Result<Vec<String>, String> {
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

fn frame(bytes: &mut Vec<u8>, json: &str) -> Result<(), String> {
    let len = u32::try_from(json.len())
        .map_err(|_| "thinkthen defect: a JSON value is too large".to_owned())?;
    bytes.extend_from_slice(&len.to_ne_bytes());
    bytes.extend_from_slice(json.as_bytes());
    Ok(())
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
) -> Reply {
    reply_boundary(|| {
        let argument = text(argument, argument_len)?;
        let members = copied_texts(members, member_count)?;
        let texts = copied_texts(texts, text_count)?;
        let ask = nested::ask(kind, argument, &members, from_file != 0)?;
        let asked = asked(&settings)?;
        let engine = engines::engine_for(&asked, |_| probe(&settings))?;
        let (texts, cut) = engines::within_total(&asked, texts)?;
        let values = nested::run(&engine, &ask, texts, deadline_ms, kind)?;
        if let Some(error) = cut {
            return Err(error);
        }
        Ok(values)
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
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let members = copied_texts(members, member_count)?;
        let texts = copied_texts(texts, text_count)?;
        let set = listed::set(kind, question, &members)?;
        let asked = asked(&settings)?;
        let engine = engines::engine_for(&asked, |_| probe(&settings))?;
        let (texts, cut) = engines::within_total(&asked, texts)?;
        let options = if deadline_ms == -1 {
            CallOptions::new()
        } else {
            CallOptions::new()
                .deadline_millis(deadline_ms)
                .map_err(|error| errors::RowError::from(error).text)?
        };
        let values = listed::run(&engine, &set, texts, options)?;
        if let Some(error) = cut {
            return Err(error);
        }
        Ok(values)
    })
}

fn asked(settings: &BridgeSettings) -> Result<engines::Asked, String> {
    let present = |value| (value != i64::MIN).then_some(value);
    let cache = (!settings.cache_bytes.is_null())
        .then(|| text(settings.cache_bytes, settings.cache_len).map(str::to_owned))
        .transpose()?;
    Ok(engines::Asked {
        throttle: present(settings.throttle),
        max_requests: present(settings.max_requests),
        max_requests_total: present(settings.max_requests_total),
        cache,
    })
}

fn probe(settings: &BridgeSettings) -> engines::Probe {
    if settings.cache_allowed != 0 {
        engines::Probe::Allowed
    } else {
        engines::Probe::Refused
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
    from_file: i32,
) -> Reply {
    reply_boundary(|| {
        let argument = text(question_bytes, question_len)?;
        let question = if kind == 7 {
            None
        } else {
            Some(question_typed(argument, from_file != 0).map_err(|error| error.text)?)
        };
        let copied = copied_texts(texts, count)?;
        let asked = asked(&settings)?;
        let engine = engines::engine_for(&asked, |_| probe(&settings))?;
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
        if kind == 7 {
            let set = set_typed(argument, from_file != 0).map_err(|error| error.text)?;
            let rows = engine
                .annotate_with(&set, copied, options()?)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| errors::RowError::from(error).text)?;
            let mut bytes = Vec::new();
            for row in rows {
                frame(&mut bytes, &row.value_json())?;
            }
            if let Some(error) = cut {
                return Err(error);
            }
            return Ok(bytes);
        }
        let question = question.ok_or_else(|| "thinkthen defect: no scalar question".to_owned())?;
        if kind == 2 {
            let mut bytes = Vec::new();
            for text in copied {
                let result = match &question {
                    LoadedQuestion::Question(held) => engine.details_with(held, &text, options()?),
                    LoadedQuestion::Banded(held) => engine.details_with(held, &text, options()?),
                }
                .map_err(|error| errors::RowError::from(error).text)?;
                frame(&mut bytes, &result.to_json())?;
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

/// One try-details row; only usage, local, and backend errors become JSON values.
///
/// # Safety
/// Both byte ranges must remain readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_try_details_row(
    question_bytes: *const u8,
    question_len: usize,
    evidence_bytes: *const u8,
    evidence_len: usize,
    deadline_ms: i64,
    settings: BridgeSettings,
    from_file: i32,
) -> Reply {
    reply_boundary(|| {
        let result = (|| -> Result<String, errors::RowError> {
            let argument = text(question_bytes, question_len)
                .map_err(|_| errors::RowError::usage("a question is not UTF-8 text"))?;
            let evidence = text(evidence_bytes, evidence_len)
                .map_err(|_| errors::RowError::usage("evidence is not UTF-8 text"))?;
            let question = question_typed(argument, from_file != 0)?;
            let asked = asked(&settings)
                .map_err(|_| errors::RowError::usage("a cache folder is not UTF-8 text"))?;
            let engine = engines::engine_for_typed(&asked, |_| probe(&settings))?;
            let (_, cut) = engines::within_total_typed(&asked, vec![evidence.to_owned()])?;
            let options = if deadline_ms == -1 {
                CallOptions::new()
            } else {
                CallOptions::new()
                    .deadline_millis(deadline_ms)
                    .map_err(errors::RowError::from)?
            };
            let details = match &question {
                LoadedQuestion::Question(held) => engine.details_with(held, evidence, options),
                LoadedQuestion::Banded(held) => engine.details_with(held, evidence, options),
            }
            .map_err(errors::RowError::from)?;
            if let Some(error) = cut {
                return Err(error);
            }
            Ok(format!(
                "{{\"status\":\"answered\",\"details\":{}}}",
                details.to_json()
            ))
        })();
        match result {
            Ok(value) => Ok(value.into_bytes()),
            Err(error) => error.value().map(String::into_bytes).ok_or(error.text),
        }
    })
}
