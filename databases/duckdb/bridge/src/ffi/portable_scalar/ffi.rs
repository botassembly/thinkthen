//! Portable settings for the details result codec.
#![allow(
    unsafe_code,
    reason = "the existing scalar boundary copies every borrowed range"
)]

use thinkthen::Settings;

use super::{BridgeSettings, BridgeStop, BridgeText, Reply, answered, reply_boundary, text};

/// Details reads any asking question (kind 2).
fn prepare(
    argument: &str,
    from_file: bool,
    settings: &str,
    kind: i32,
) -> Result<(String, Settings), String> {
    if kind != 2 {
        return Err("thinkthen defect: unknown portable scalar kind".into());
    }
    let asked = super::portable_aux::asking_kind(argument, from_file, "details")
        .map_err(|error| error.text)?;
    let (written, _, call) = super::portable_many::parse(argument, from_file, settings, asked)?;
    Ok((written, call))
}

/// Check one details (kind 2) question and settings before any chunk member sends.
///
/// # Safety
/// All input byte ranges remain live during this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_portable_scalar(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    settings: *const u8,
    settings_len: usize,
    kind: i32,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let settings = text(settings, settings_len)?;
        prepare(question, from_file != 0, settings, kind).map(|_| Vec::new())
    })
}

/// Forward one portable group to the retained scalar codec and engine route.
///
/// # Safety
/// All byte ranges and the stop callback remain live until this call returns.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_portable_scalar_group(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    texts: *const BridgeText,
    count: usize,
    settings: *const u8,
    settings_len: usize,
    kind: i32,
    query_deadline_ms: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    super::panic::caught(|| {
        let prepared = (|| -> Result<Reply, String> {
            let question = text(question, question_len)?;
            let settings = text(settings, settings_len)?;
            let (written, call) = prepare(question, from_file != 0, settings, kind)?;
            let batch = super::portable::batch_word(&call);
            let mut session = session;
            if let Some(word) = &batch {
                session.batch_bytes = word.as_ptr();
                session.batch_len = word.len();
            }
            let context = call.context();
            // The existing boundary copies all strings before its detached worker starts.
            Ok(unsafe {
                super::scalar::thinkthen_cpp_scalar_group(
                    written.as_ptr(),
                    written.len(),
                    texts,
                    count,
                    super::portable::due(query_deadline_ms, call.deadline_ms()),
                    kind,
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
