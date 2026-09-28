//! The warm aggregate's private, environment-only engine call.
#![allow(unsafe_code, reason = "the C ABI copies one complete aggregate group")]

use thinkthen::{LoadedQuestion, QuestionKind};

use crate::engines;
use crate::errors::RowError;
use crate::ffi::{
    BridgeStop, BridgeText, Reply, question_typed, reply_boundary, run_detached, text,
};

/// Judge every distinct aggregate text once, ignoring SQL session settings.
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
        let engine = engines::from_env()?;
        run_detached(stop, move |token| {
            let answered = match &question {
                LoadedQuestion::Question(held) => engine
                    .decide_many_with(held, copied, thinkthen::CallOptions::new().cancel(&token))
                    .collect::<Result<Vec<_>, _>>(),
                LoadedQuestion::Banded(held) => engine
                    .decide_many_with(held, copied, thinkthen::CallOptions::new().cancel(&token))
                    .collect::<Result<Vec<_>, _>>(),
            }
            .map_err(|error| RowError::from(error).text)?;
            let count = i64::try_from(answered.len()).unwrap_or(i64::MAX);
            Ok(count.to_ne_bytes().to_vec())
        })
    })
}
