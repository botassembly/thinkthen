//! Portable listed settings beside native DuckDB VARCHAR[] members.
#![allow(
    unsafe_code,
    reason = "the bridge copies caller byte ranges before work starts"
)]

use thinkthen::{LoadedQuestion, Settings};

use crate::engines;
use crate::errors::RowError;

use super::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, batch, copied_texts, probe,
    reply_boundary, run_detached, text,
};

fn prepare(
    argument: &str,
    from_file: bool,
    members: Option<&str>,
    settings: &str,
    kind: i32,
) -> Result<(LoadedQuestion, Settings), String> {
    let merged = if let Some(members) = members {
        let parsed =
            Settings::parse(settings).map_err(|error| RowError::usage(&error.to_string()).text)?;
        parsed
            .conflicts(&[], true)
            .map_err(|error| RowError::usage(&error.to_string()).text)?;
        let body = settings
            .trim()
            .strip_prefix('{')
            .and_then(|rest| rest.strip_suffix('}'))
            .ok_or_else(|| "thinkthen defect: checked settings lost their object".to_owned())?;
        let field = match kind {
            4 => "options",
            5 => "levels",
            6 => "labels",
            _ => return Err("thinkthen defect: unknown listed kind".into()),
        };
        format!(
            "{{{body}{}\"{field}\":{members}}}",
            if body.trim().is_empty() { "" } else { "," }
        )
    } else {
        settings.to_owned()
    };
    let (_, question, call) = super::portable_many::parse(argument, from_file, &merged, kind)?;
    Ok((question, call))
}

/// Check listed question, settings and member conversion before any send.
///
/// # Safety
/// All byte ranges remain live during the call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_portable_listed(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    members: *const u8,
    members_len: usize,
    settings: *const u8,
    settings_len: usize,
    kind: i32,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let members = (!members.is_null())
            .then(|| text(members, members_len))
            .transpose()?;
        let settings = text(settings, settings_len)?;
        prepare(question, from_file != 0, members, settings, kind).map(|_| Vec::new())
    })
}

/// One vector call with listed values under portable settings.
///
/// # Safety
/// All byte ranges and stop callback remain live until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_portable_listed_group(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    texts: *const BridgeText,
    count: usize,
    members: *const u8,
    members_len: usize,
    settings: *const u8,
    settings_len: usize,
    kind: i32,
    query_deadline_ms: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let members = (!members.is_null())
            .then(|| text(members, members_len))
            .transpose()?;
        let settings = text(settings, settings_len)?;
        let (question, call) = prepare(question, from_file != 0, members, settings, kind)?;
        let LoadedQuestion::Question(question) = question else {
            return Err(RowError::usage("the question has another kind").text);
        };
        let texts = copied_texts(texts, count)?;
        let session_batch = batch(&session)?;
        let batch = super::portable::batch_word(&call);
        let asked = asked(&session)?;
        let engine = engines::engine_for(&asked, |path| probe(&session, path))?;
        let total = asked.max_requests_total;
        let context = call.context().map(str::to_owned);
        run_detached(stop, move |token| {
            let options = engines::options_for(
                super::portable::due(query_deadline_ms, call.deadline_ms()),
                &token,
                total,
                batch.as_deref().or(session_batch.as_deref()),
                context.as_deref(),
            )?;
            super::complete_listed::run(&engine, &question, texts, options, total)
        })
    })
}
