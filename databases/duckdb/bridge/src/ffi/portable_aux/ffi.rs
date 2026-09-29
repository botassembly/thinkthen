//! Portable call settings for the retained annotation codec.
#![allow(
    unsafe_code,
    reason = "the retained scalar boundary copies all borrowed bytes"
)]

use thinkthen::{For, LoadedQuestion, QuestionKind, Settings};

use crate::errors::RowError;

use super::{
    BridgeSettings, BridgeStop, BridgeText, CallScope, Reply, answered, asked, batch, copied_texts,
    probe, reply_boundary, run_detached, text,
};
use crate::engines;

fn prepare_try(argument: &str, from_file: bool, raw: &str) -> Result<(String, Settings), RowError> {
    let question = super::question_typed(argument, from_file)?;
    let kind = match &question {
        LoadedQuestion::Banded(_) => 0,
        LoadedQuestion::Question(value) => match value.kind() {
            QuestionKind::Decide => 0,
            QuestionKind::Choose => 4,
            QuestionKind::Score => 5,
            QuestionKind::Tag => 6,
            _ => return Err(RowError::usage("try_details needs an asking question")),
        },
    };
    super::portable_many::parse(argument, from_file, raw, kind)
        .map(|(written, _, call)| (written, call))
        .map_err(|error| RowError::usage(error.strip_prefix("thinkthen usage: ").unwrap_or(&error)))
}

fn failed_rows(error: RowError, count: usize) -> Result<Vec<u8>, String> {
    let Some(value) = error.value() else {
        return Err(error.text);
    };
    let mut bytes = Vec::new();
    for _ in 0..count {
        super::scalar::frame(&mut bytes, &value)?;
    }
    Ok(bytes)
}

pub(crate) fn call_settings(raw: &str) -> Result<(Settings, Option<String>), String> {
    let call = Settings::parse(raw)
        .and_then(|value| {
            value.check(For::Find)?;
            Ok(value)
        })
        .map_err(|error| RowError::usage(&error.to_string()).text)?;
    if call.none().is_some() {
        return Err(RowError::usage("the settings key `none` does not belong to this verb").text);
    }
    let model = serde_json::from_str::<serde_json::Value>(raw)
        .ok()
        .and_then(|value| value.get("model")?.as_str().map(str::to_owned));
    Ok((call, model))
}

fn nested_settings(
    argument: &str,
    from_file: bool,
    raw: &str,
    kind: i32,
) -> Result<(Settings, Option<String>), String> {
    if kind != 8 && kind != 9 {
        return Err("thinkthen defect: unknown nested kind".into());
    }
    let (call, model) = call_settings(raw)?;
    if kind == 9 && from_file {
        let source: serde_json::Value = serde_json::from_str(argument)
            .map_err(|error| RowError::usage(&error.to_string()).text)?;
        if source.get("model").is_some() {
            call.conflicts(&["model"], false)
                .map_err(|error| RowError::usage(&error.to_string()).text)?;
        }
    }
    Ok((call, model))
}

/// Check nested settings against the same shared schema before any send.
///
/// # Safety
/// The caller retains all byte ranges for this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_portable_nested(
    argument: *const u8,
    argument_len: usize,
    from_file: i32,
    members: *const BridgeText,
    member_count: usize,
    settings: *const u8,
    settings_len: usize,
    kind: i32,
) -> Reply {
    reply_boundary(|| {
        let argument = text(argument, argument_len)?;
        let members = copied_texts(members, member_count)?;
        nested_settings(
            argument,
            from_file != 0,
            text(settings, settings_len)?,
            kind,
        )?;
        super::nested::ask(kind, argument, &members, from_file != 0).map(|_| Vec::new())
    })
}

/// Run a nested judgment with shared settings and the retained result codec.
///
/// # Safety
/// The caller retains all byte ranges and the stop callback until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_portable_nested_group(
    argument: *const u8,
    argument_len: usize,
    from_file: i32,
    members: *const BridgeText,
    member_count: usize,
    texts: *const BridgeText,
    text_count: usize,
    settings: *const u8,
    settings_len: usize,
    kind: i32,
    query_deadline_ms: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let argument = text(argument, argument_len)?;
        let members = copied_texts(members, member_count)?;
        let texts = copied_texts(texts, text_count)?;
        let (call, model) = nested_settings(
            argument,
            from_file != 0,
            text(settings, settings_len)?,
            kind,
        )?;
        let ask = super::nested::ask(kind, argument, &members, from_file != 0)?;
        let mut held = asked(&session)?;
        if let Some(model) = model {
            held.model = Some(model);
        }
        let engine = engines::engine_for(&held, |path| probe(&session, path))?;
        let (texts, cut) = engines::within_total(&held, texts)?;
        let total = held.max_requests_total;
        let deadline = match call.deadline_ms() {
            None | Some(-1) => query_deadline_ms,
            Some(value) if query_deadline_ms < 0 => value,
            Some(value) => query_deadline_ms.min(value),
        };
        let session_batch = batch(&session)?;
        let chosen_batch = if call.batch_max() {
            Some("max".to_owned())
        } else {
            call.batch_records().map(|value| value.to_string())
        };
        let batch = chosen_batch.or(session_batch);
        let context = call.context().map(str::to_owned);
        run_detached(stop, move |token| {
            let values = super::nested::run(
                &engine,
                &ask,
                texts,
                kind,
                CallScope {
                    due: deadline,
                    token: &token,
                    total,
                    batch: batch.as_deref(),
                    context: context.as_deref(),
                },
            )?;
            if let Some(error) = cut {
                return Err(error);
            }
            Ok(values)
        })
    })
}

fn prepare(
    argument: &str,
    from_file: bool,
    raw: &str,
) -> Result<(Settings, Option<String>), String> {
    let (call, model) = call_settings(raw)?;
    super::set_typed(argument, from_file).map_err(|error| error.text)?;
    Ok((call, model))
}

/// Check one annotation set and its call settings before any chunk send.
///
/// # Safety
/// The caller retains both byte ranges until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_portable_annotate(
    argument: *const u8,
    argument_len: usize,
    from_file: i32,
    settings: *const u8,
    settings_len: usize,
) -> Reply {
    reply_boundary(|| {
        prepare(
            text(argument, argument_len)?,
            from_file != 0,
            text(settings, settings_len)?,
        )
        .map(|_| Vec::new())
    })
}

/// Run the existing owned annotation codec with validated call settings.
///
/// # Safety
/// The caller retains every byte range and the stop callback until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_portable_annotate_group(
    argument: *const u8,
    argument_len: usize,
    from_file: i32,
    texts: *const BridgeText,
    count: usize,
    settings: *const u8,
    settings_len: usize,
    query_deadline_ms: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    super::panic::caught(|| {
        let prepared = (|| -> Result<Reply, String> {
            let argument = text(argument, argument_len)?;
            let (call, model) = prepare(argument, from_file != 0, text(settings, settings_len)?)?;
            let mut session = session;
            if let Some(model) = &model {
                session.model_bytes = model.as_ptr();
                session.model_len = model.len();
            }
            let batch = if call.batch_max() {
                Some("max".to_owned())
            } else {
                call.batch_records().map(|value| value.to_string())
            };
            if let Some(batch) = &batch {
                session.batch_bytes = batch.as_ptr();
                session.batch_len = batch.len();
            }
            let context = call.context();
            let due = match call.deadline_ms() {
                None | Some(-1) => query_deadline_ms,
                Some(value) if query_deadline_ms < 0 => value,
                Some(value) => query_deadline_ms.min(value),
            };
            // The retained boundary copies every string before its worker detaches.
            Ok(unsafe {
                super::scalar::thinkthen_cpp_scalar_group(
                    argument.as_ptr(),
                    argument.len(),
                    texts,
                    count,
                    due,
                    7,
                    session,
                    from_file,
                    context.map_or(std::ptr::null(), str::as_ptr),
                    context.map_or(0, str::len),
                    stop,
                )
            })
        })();
        match prepared {
            Ok(reply) => reply,
            Err(error) => answered(Err(error)),
        }
    })
    .unwrap_or(Reply {
        status: 4,
        bytes: std::ptr::null_mut(),
        len: 0,
    })
}

/// Keep try-details' recoverable row envelope while applying portable settings.
///
/// # Safety
/// The caller retains byte ranges and the stop callback through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_portable_try_details_group(
    argument: *const u8,
    argument_len: usize,
    from_file: i32,
    texts: *const BridgeText,
    count: usize,
    settings: *const u8,
    settings_len: usize,
    query_deadline_ms: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    super::panic::caught(|| {
        let prepared = (|| -> Result<Reply, String> {
            let argument = text(argument, argument_len)?;
            let settings = text(settings, settings_len)?;
            let (written, call) = match prepare_try(argument, from_file != 0, settings) {
                Ok(value) => value,
                Err(error) => return Ok(answered(failed_rows(error, count))),
            };
            let batch = if call.batch_max() {
                Some("max".to_owned())
            } else {
                call.batch_records().map(|value| value.to_string())
            };
            let mut session = session;
            if let Some(batch) = &batch {
                session.batch_bytes = batch.as_ptr();
                session.batch_len = batch.len();
            }
            let context = call.context();
            let due = match call.deadline_ms() {
                None | Some(-1) => query_deadline_ms,
                Some(value) if query_deadline_ms < 0 => value,
                Some(value) => query_deadline_ms.min(value),
            };
            Ok(unsafe {
                super::scalar::thinkthen_cpp_try_details_group(
                    written.as_ptr(),
                    written.len(),
                    texts,
                    count,
                    due,
                    session,
                    0,
                    context.map_or(std::ptr::null(), str::as_ptr),
                    context.map_or(0, str::len),
                    stop,
                )
            })
        })();
        match prepared {
            Ok(reply) => reply,
            Err(error) => answered(Err(error)),
        }
    })
    .unwrap_or(Reply {
        status: 4,
        bytes: std::ptr::null_mut(),
        len: 0,
    })
}
