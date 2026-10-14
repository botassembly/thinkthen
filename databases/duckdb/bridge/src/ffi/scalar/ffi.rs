//! Owned scalar calls on the detachable DuckDB worker.
#![allow(
    unsafe_code,
    reason = "the scalar bridge copies C++ byte ranges before its worker runs"
)]

use thinkthen::{Engine, QuestionSet};

use super::{
    BridgeSettings, BridgeStop, BridgeText, CallScope, Reply, asked, batch, copied_texts, probe,
    reply_boundary, run_detached, set_typed, text,
};
use crate::engines;

pub(super) fn frame(bytes: &mut Vec<u8>, json: &str) -> Result<(), String> {
    let len = u32::try_from(json.len())
        .map_err(|_| "thinkthen defect: a JSON value is too large".to_owned())?;
    bytes.extend_from_slice(&len.to_ne_bytes());
    bytes.extend_from_slice(json.as_bytes());
    Ok(())
}

fn annotate(
    engine: &Engine,
    set: &QuestionSet,
    texts: Vec<String>,
    scope: CallScope<'_>,
) -> Result<Vec<u8>, String> {
    let result = super::request::run(
        engine,
        set.clone().into(),
        super::request::texts(texts),
        scope,
    )?
    .success(scope.total)?;
    let rows = result.rows;
    let mut bytes = Vec::new();
    for row in rows {
        let thinkthen::RequestSessionRow::Annotation(row) = row else {
            return Err("thinkthen defect: annotation returned another kind".to_owned());
        };
        frame(
            &mut bytes,
            &row.result()
                .value_json()
                .map_err(|e| engines::call_error(e, scope.total).text)?,
        )?;
    }
    Ok(bytes)
}

/// Evaluate one grouped annotation call.
///
/// # Safety
/// The question and every entry in `texts` stay readable through this call.
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
    context_bytes: *const u8,
    context_len: usize,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let argument = text(question_bytes, question_len)?;
        if kind != 7 {
            return Err("thinkthen defect: unknown annotation kind".to_owned());
        }
        let set = set_typed(argument, from_file != 0).map_err(|error| error.text)?;
        let copied = copied_texts(texts, count)?;
        let context = (!context_bytes.is_null())
            .then(|| text(context_bytes, context_len).map(str::to_owned))
            .transpose()?;
        let batch = batch(&settings)?;
        let asked = asked(&settings)?;
        let engine = engines::engine_for(&asked, |path| probe(&settings, path))?;
        let total = asked.max_requests_total;
        run_detached(stop, move |token| {
            let scope = CallScope {
                due: deadline_ms,
                token: &token,
                total,
                batch: batch.as_deref(),
                context: context.as_deref(),
            };
            let bytes = annotate(&engine, &set, copied, scope)?;
            Ok(bytes)
        })
    })
}
