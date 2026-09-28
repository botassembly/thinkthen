//! The warm aggregate's caller-session engine call.
#![allow(unsafe_code, reason = "the C ABI copies one complete aggregate group")]

use thinkthen::{LoadedQuestion, QuestionKind};

use crate::engines;
use crate::ffi::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, batch, probe, question_typed,
    reply_boundary, run_detached, text,
};

/// Judge every distinct aggregate text once with the caller's settings.
///
/// # Safety
/// The question and every entry of `texts` remain readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_warm(
    question: *const u8,
    question_len: usize,
    texts: *const BridgeText,
    count: usize,
    from_file: i32,
    deadline_ms: i64,
    context_bytes: *const u8,
    context_len: usize,
    settings: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let question = question_typed(question, from_file != 0).map_err(|error| error.text)?;
        if matches!(&question, LoadedQuestion::Question(held) if held.kind() != QuestionKind::Decide)
        {
            return Err("thinkthen usage: thinkthen_warm takes a decide question; ask others with thinkthen_decide".to_owned());
        }
        if texts.is_null() && count != 0 {
            return Err("thinkthen defect: the bridge got a null text array".to_owned());
        }
        let rows = if count == 0 {
            &[][..]
        } else {
            // SAFETY: C++ owns this fixed array until the synchronous call returns.
            unsafe { std::slice::from_raw_parts(texts, count) }
        };
        let copied = rows
            .iter()
            .map(|row| text(row.bytes, row.len).map(str::to_owned))
            .collect::<Result<Vec<_>, _>>()?;
        let context = (!context_bytes.is_null())
            .then(|| text(context_bytes, context_len).map(str::to_owned))
            .transpose()?;
        let batch = batch(&settings)?;
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
            let answered = match &question {
                LoadedQuestion::Question(held) => engine
                    .decide_many_with(held, copied, options)
                    .collect::<Result<Vec<_>, _>>(),
                LoadedQuestion::Banded(held) => engine
                    .decide_many_with(held, copied, options)
                    .collect::<Result<Vec<_>, _>>(),
            }
            .map_err(|error| engines::call_error(error, total).text)?;
            let count = i64::try_from(answered.len()).unwrap_or(i64::MAX);
            Ok(count.to_ne_bytes().to_vec())
        })
    })
}
