//! A keyed SQL preview through the public, send-free engine planner.
#![allow(unsafe_code, reason = "the bridge copies caller-owned plan arguments")]

use thinkthen::{LoadedQuestion, QuestionKind};

use crate::engines;
use crate::errors::RowError;

use super::{BridgeSettings, Reply, asked, batch, probe, reply_boundary, text};

fn kind(argument: &str, from_file: bool) -> Result<i32, String> {
    if !from_file && !argument.trim_start().starts_with('{') {
        return Ok(0);
    }
    let parsed = super::question_typed(argument, from_file).map_err(|error| error.text)?;
    let kind = match parsed {
        LoadedQuestion::Banded(_) => 0,
        LoadedQuestion::Question(question) => match question.kind() {
            QuestionKind::Decide => 0,
            QuestionKind::Choose => 4,
            QuestionKind::Score => 5,
            QuestionKind::Tag => 6,
            _ => return Err(RowError::usage("plan takes an ordinary question").text),
        },
    };
    Ok(kind)
}

fn checked(
    question: &str,
    from_file: bool,
    keyed: &str,
    settings: &str,
) -> Result<(thinkthen::LoadedQuestion, thinkthen::Settings, Vec<String>), String> {
    let (_, question, settings) =
        super::portable_many::parse(question, from_file, settings, kind(question, from_file)?)?;
    let texts = super::portable_many::keyed_texts(keyed)?;
    Ok((question, settings, texts))
}

fn count(value: usize) -> Result<[u8; 8], String> {
    u64::try_from(value)
        .map(u64::to_ne_bytes)
        .map_err(|_| RowError::usage("the planned input is too large to count").text)
}

fn plan(
    question: &str,
    from_file: bool,
    keyed: &str,
    settings: &str,
    session: BridgeSettings,
) -> Result<Vec<u8>, String> {
    let (question, call, texts) = checked(question, from_file, keyed, settings)?;
    let asked = asked(&session)?;
    let engine = engines::engine_for(&asked, |path| probe(&session, path))?;
    let session_batch = batch(&session)?;
    let call_batch = super::portable::batch_word(&call);
    let context = call.context().map(str::to_owned);
    let token = thinkthen::CancelToken::new();
    let options = engines::options_for(
        super::portable::due(-1, call.deadline_ms()),
        &token,
        asked.max_requests_total,
        call_batch.as_deref().or(session_batch.as_deref()),
        context.as_deref(),
    )?;
    let estimate = match &question {
        LoadedQuestion::Question(value) => engine.plan_with(value, texts, options),
        LoadedQuestion::Banded(value) => engine.plan_with(value, texts, options),
    }
    .map_err(|error| RowError::from(error).text)?;
    let (lower, upper) = estimate.estimated_input_tokens();
    let body = estimate.first_body();
    let mut output = Vec::new();
    for value in [
        estimate.records(),
        estimate.requests(),
        estimate.estimated_bytes(),
        lower,
        upper,
        body.map_or(0, <[u8]>::len),
    ] {
        output.extend_from_slice(&count(value)?);
    }
    output.push(u8::from(estimate.upper_bound()));
    output.push(u8::from(body.is_some()));
    if let Some(body) = body {
        output.extend_from_slice(body);
    }
    Ok(output)
}

/// Validate the keyed object, question and settings before any other row runs.
///
/// # Safety
/// All byte ranges remain live through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_plan(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    keyed: *const u8,
    keyed_len: usize,
    settings: *const u8,
    settings_len: usize,
) -> Reply {
    reply_boundary(|| {
        checked(
            text(question, question_len)?,
            from_file != 0,
            text(keyed, keyed_len)?,
            text(settings, settings_len)?,
        )?;
        Ok(Vec::new())
    })
}

/// Return a native-boundary preview without sending or reading the key.
///
/// # Safety
/// All byte ranges remain live through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_plan(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    keyed: *const u8,
    keyed_len: usize,
    settings: *const u8,
    settings_len: usize,
    session: BridgeSettings,
) -> Reply {
    reply_boundary(|| {
        plan(
            text(question, question_len)?,
            from_file != 0,
            text(keyed, keyed_len)?,
            text(settings, settings_len)?,
            session,
        )
    })
}
