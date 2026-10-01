//! Owned scalar calls on the detachable DuckDB worker.
#![allow(
    unsafe_code,
    reason = "the scalar bridge copies C++ byte ranges before its worker runs"
)]

use std::sync::Arc;
use thinkthen::{CancelToken, Engine, ErrorKind, LoadedQuestion, QuestionSet, RecoverableDetails};

use super::{
    BridgeSettings, BridgeStop, BridgeText, CallScope, Reply, asked, batch, copied_texts, probe,
    question_typed, reply_boundary, run_detached, set_typed, text,
};
use crate::{engines, errors};

pub(super) fn frame(bytes: &mut Vec<u8>, json: &str) -> Result<(), String> {
    let len = u32::try_from(json.len())
        .map_err(|_| "thinkthen defect: a JSON value is too large".to_owned())?;
    bytes.extend_from_slice(&len.to_ne_bytes());
    bytes.extend_from_slice(json.as_bytes());
    Ok(())
}

fn safe_failure(kind: ErrorKind, retryable: bool) -> Result<String, String> {
    let message = match kind {
        ErrorKind::Usage => {
            "check the row's question and arguments, or raise the process request total when it is spent"
        }
        ErrorKind::Local => "check the named file and its permissions",
        ErrorKind::Backend => "the backend did not answer; retry if allowed",
        ErrorKind::Cancelled | ErrorKind::Deadline | ErrorKind::Defect => {
            return Err("thinkthen defect: a fatal error entered a recoverable row".to_owned());
        }
    };
    Ok(format!(
        "{{\"status\":\"failed\",\"error\":{{\"kind\":\"{}\",\"message\":\"{message}\",\"retryable\":{retryable}}}}}",
        kind.name()
    ))
}

fn put(values: &mut [Option<String>], place: usize, value: String) -> Result<(), String> {
    let slot = values
        .get_mut(place)
        .ok_or_else(|| "thinkthen defect: a try-details slot was lost".to_owned())?;
    *slot = Some(value);
    Ok(())
}

struct TryGroup {
    copied: Vec<String>,
    question: LoadedQuestion,
    context: Option<String>,
    batch: Option<String>,
    total: Option<i64>,
    engine: Arc<Engine>,
    due: i64,
}

impl TryGroup {
    fn run(self, token: CancelToken) -> Result<Vec<u8>, String> {
        let mut places = Vec::new();
        let mut valid = Vec::new();
        let mut values = vec![None; self.copied.len()];
        for (place, text) in self.copied.iter().enumerate() {
            if text.trim().is_empty() {
                put(&mut values, place, safe_failure(ErrorKind::Usage, false)?)?;
                continue;
            }
            places.push(place);
            valid.push(text.clone());
        }
        if !valid.is_empty() {
            self.answer(&token, &valid, places, &mut values)?;
        }
        let mut bytes = Vec::new();
        for value in values {
            let value = value
                .ok_or_else(|| "thinkthen defect: a try-details row lost its value".to_owned())?;
            frame(&mut bytes, &value)?;
        }
        Ok(bytes)
    }

    fn answer(
        &self,
        token: &CancelToken,
        valid: &[String],
        places: Vec<usize>,
        values: &mut [Option<String>],
    ) -> Result<(), String> {
        let Ok(options) = engines::options_for(
            self.due,
            token,
            self.total,
            self.batch.as_deref(),
            self.context.as_deref(),
        ) else {
            for place in places {
                put(values, place, safe_failure(ErrorKind::Usage, false)?)?;
            }
            return Ok(());
        };
        let result = self
            .engine
            .details_many_recoverable_with(&self.question, valid, options);
        let answered = match result {
            Ok(answered) => answered,
            Err(error) if error.send_budget_denial().is_some() => {
                let failed = safe_failure(ErrorKind::Usage, false)?;
                for place in places {
                    put(values, place, failed.clone())?;
                }
                return Ok(());
            }
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::Usage | ErrorKind::Local | ErrorKind::Backend
                ) =>
            {
                let failed = safe_failure(error.kind(), error.retryable())?;
                for place in places {
                    put(values, place, failed.clone())?;
                }
                return Ok(());
            }
            Err(error) => return Err(engines::call_error(error, self.total).text),
        };
        if answered.value().len() != places.len() {
            return Err("thinkthen defect: a try-details group lost its values".to_owned());
        }
        for (place, row) in places.into_iter().zip(answered.into_value()) {
            let value = match row {
                RecoverableDetails::Answered(details) => format!(
                    "{{\"status\":\"answered\",\"details\":{}}}",
                    details.to_json()
                ),
                RecoverableDetails::Failed {
                    kind, retryable, ..
                } => safe_failure(kind, retryable)?,
                _ => return Err("thinkthen defect: unknown recoverable detail".to_owned()),
            };
            put(values, place, value)?;
        }
        Ok(())
    }
}

/// Evaluate one bounded group with recoverable member outcomes.
///
/// # Safety
/// The question, context and text array remain readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_try_details_group(
    question_bytes: *const u8,
    question_len: usize,
    texts: *const BridgeText,
    count: usize,
    deadline_ms: i64,
    settings: BridgeSettings,
    from_file: i32,
    context_bytes: *const u8,
    context_len: usize,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let copied = copied_texts(texts, count)?;
        let prepared = (|| -> Result<_, errors::RowError> {
            let argument = text(question_bytes, question_len)
                .map_err(|_| errors::RowError::usage("a question is not UTF-8 text"))?;
            let question = question_typed(argument, from_file != 0)?;
            let context = (!context_bytes.is_null())
                .then(|| text(context_bytes, context_len).map(str::to_owned))
                .transpose()
                .map_err(|_| errors::RowError::usage("context is not UTF-8 text"))?;
            let batch =
                batch(&settings).map_err(|_| errors::RowError::usage("batch is not UTF-8 text"))?;
            let asked = asked(&settings)
                .map_err(|_| errors::RowError::usage("a setting is not UTF-8 text"))?;
            let engine = engines::engine_for_typed(&asked, |path| probe(&settings, path))?;
            Ok((question, context, batch, asked.max_requests_total, engine))
        })();
        let (question, context, batch, total, engine) = match prepared {
            Ok(ready) => ready,
            Err(error) => {
                let Some(value) = error.value() else {
                    return Err(error.text);
                };
                let mut bytes = Vec::new();
                for _ in 0..count {
                    frame(&mut bytes, &value)?;
                }
                return Ok(bytes);
            }
        };
        run_detached(stop, move |token| {
            TryGroup {
                copied,
                question,
                context,
                batch,
                total,
                engine,
                due: deadline_ms,
            }
            .run(token)
        })
    })
}

fn annotate(
    engine: &Engine,
    set: &QuestionSet,
    texts: Vec<String>,
    scope: CallScope<'_>,
) -> Result<Vec<u8>, String> {
    let options = engines::options_for(
        scope.due,
        scope.token,
        scope.total,
        scope.batch,
        scope.context,
    )?;
    // A row refused before its request, such as one missing an `on` part,
    // fails alone: its neighbours are still asked and stored, and the vector
    // then raises that row's error. A rerun that leaves the bad row out
    // answers its neighbours from the store.
    let rows = engine
        .annotate_each_with(set, &texts, options)
        .map_err(|error| engines::call_error(error, scope.total).text)?
        .into_value();
    let mut bytes = Vec::new();
    for row in rows {
        let row = row.map_err(|error| engines::call_error(error, scope.total).text)?;
        frame(&mut bytes, &row.value_json())?;
    }
    Ok(bytes)
}

fn details(
    engine: &Engine,
    question: &LoadedQuestion,
    texts: Vec<String>,
    scope: CallScope<'_>,
) -> Result<Vec<u8>, String> {
    let options = engines::options_for(
        scope.due,
        scope.token,
        scope.total,
        scope.batch,
        scope.context,
    )?;
    let rows = engine
        .details_many_with(question, texts, options)
        .map(|row| row.map(|row| row.value().to_scalar_json()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| engines::call_error(error, scope.total).text)?;
    let mut bytes = Vec::new();
    for json in rows {
        frame(&mut bytes, &json)?;
    }
    Ok(bytes)
}

enum ScalarAsk {
    Question(LoadedQuestion),
    Set(QuestionSet),
}

fn scalar_bytes(
    engine: &Engine,
    ask: ScalarAsk,
    texts: Vec<String>,
    kind: i32,
    scope: CallScope<'_>,
) -> Result<Vec<u8>, String> {
    match (kind, ask) {
        (7, ScalarAsk::Set(set)) => annotate(engine, &set, texts, scope),
        (2, ScalarAsk::Question(question)) => details(engine, &question, texts, scope),
        _ => Err("thinkthen defect: the bridge got an unknown scalar kind".to_owned()),
    }
}

/// Evaluate one grouped details or annotation call.
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
        let ask = if kind == 7 {
            ScalarAsk::Set(set_typed(argument, from_file != 0).map_err(|error| error.text)?)
        } else {
            ScalarAsk::Question(
                question_typed(argument, from_file != 0).map_err(|error| error.text)?,
            )
        };
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
            let bytes = scalar_bytes(&engine, ask, copied, kind, scope)?;
            Ok(bytes)
        })
    })
}

fn try_answer(
    engine: &Engine,
    question: &LoadedQuestion,
    evidence: &str,
    cut: Option<errors::RowError>,
    scope: CallScope<'_>,
) -> Result<String, errors::RowError> {
    let options = engines::options(scope.due, scope.token, scope.total)
        .map_err(|_| errors::RowError::usage("the deadline is outside the supported range"))?;
    let details = engine
        .details_with(question, evidence, options)
        .map_err(|error| engines::call_error(error, scope.total))?;
    if let Some(error) = cut {
        return Err(error);
    }
    Ok(format!(
        "{{\"status\":\"answered\",\"details\":{}}}",
        details.into_value().to_json()
    ))
}

/// Evaluate one recoverable details row.
///
/// # Safety
/// Both byte ranges stay readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_try_details_row(
    question_bytes: *const u8,
    question_len: usize,
    evidence_bytes: *const u8,
    evidence_len: usize,
    deadline_ms: i64,
    settings: BridgeSettings,
    from_file: i32,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let prepared = (|| -> Result<_, errors::RowError> {
            let argument = text(question_bytes, question_len)
                .map_err(|_| errors::RowError::usage("a question is not UTF-8 text"))?;
            let evidence = text(evidence_bytes, evidence_len)
                .map_err(|_| errors::RowError::usage("evidence is not UTF-8 text"))?;
            let question = question_typed(argument, from_file != 0)?;
            let asked = asked(&settings)
                .map_err(|_| errors::RowError::usage("a cache folder is not UTF-8 text"))?;
            let engine = engines::engine_for_typed(&asked, |path| probe(&settings, path))?;
            let (_, cut) = engines::within_total_typed(&asked, vec![evidence.to_owned()])?;
            Ok((
                question,
                engine,
                evidence.to_owned(),
                cut,
                asked.max_requests_total,
            ))
        })();
        let result = match prepared {
            Err(error) => Err(error),
            Ok((question, engine, evidence, cut, total)) => run_detached(stop, move |token| {
                let scope = CallScope {
                    due: deadline_ms,
                    token: &token,
                    total,
                    batch: None,
                    context: None,
                };
                Ok(try_answer(&engine, &question, &evidence, cut, scope))
            })?,
        };
        match result {
            Ok(value) => Ok(value.into_bytes()),
            Err(error) => error.value().map(String::into_bytes).ok_or(error.text),
        }
    })
}
