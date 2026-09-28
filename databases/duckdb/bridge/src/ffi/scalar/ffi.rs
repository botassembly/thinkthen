//! Owned scalar calls on the detachable DuckDB worker.
#![allow(
    unsafe_code,
    reason = "the scalar bridge copies C++ byte ranges before its worker runs"
)]

use thinkthen::{Answer, CallOptions, CancelToken, Engine, LoadedQuestion, QuestionSet};

use super::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, copied_texts, probe, question_typed,
    reply_boundary, run_detached, set_typed, text,
};
use crate::{engines, errors};

fn options<'a>(deadline_ms: i64, token: &'a CancelToken) -> Result<CallOptions<'a>, String> {
    if deadline_ms == -1 {
        Ok(CallOptions::new().cancel(token))
    } else {
        CallOptions::new()
            .deadline_millis(deadline_ms)
            .map(|options| options.cancel(token))
            .map_err(|error| errors::RowError::from(error).text)
    }
}

fn frame(bytes: &mut Vec<u8>, json: &str) -> Result<(), String> {
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
    due: i64,
    token: &CancelToken,
) -> Result<Vec<u8>, String> {
    let rows = engine
        .annotate_with(set, texts, options(due, token)?)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| errors::RowError::from(error).text)?;
    let mut bytes = Vec::new();
    for row in rows {
        frame(&mut bytes, &row.value_json())?;
    }
    Ok(bytes)
}

fn details(
    engine: &Engine,
    question: &LoadedQuestion,
    texts: Vec<String>,
    due: i64,
    token: &CancelToken,
) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    for text in texts {
        let result = match question {
            LoadedQuestion::Question(held) => {
                engine.details_with(held, &text, options(due, token)?)
            }
            LoadedQuestion::Banded(held) => engine.details_with(held, &text, options(due, token)?),
        }
        .map_err(|error| errors::RowError::from(error).text)?;
        frame(&mut bytes, &result.to_json())?;
    }
    Ok(bytes)
}

fn decisions(
    engine: &Engine,
    question: &LoadedQuestion,
    texts: Vec<String>,
    due: i64,
    kind: i32,
    token: &CancelToken,
) -> Result<Vec<u8>, String> {
    let answers: Vec<(Answer, f64)> = match question {
        LoadedQuestion::Question(held) => engine
            .decide_many_with(held, texts, options(due, token)?)
            .map(|row| row.map(|row| (*row.value(), row.probability())))
            .collect::<Result<Vec<_>, _>>(),
        LoadedQuestion::Banded(held) => engine
            .decide_many_with(held, texts, options(due, token)?)
            .map(|row| row.map(|row| (*row.value(), row.probability())))
            .collect::<Result<Vec<_>, _>>(),
    }
    .map_err(|error| errors::RowError::from(error).text)?;
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
}

enum ScalarAsk {
    Question(LoadedQuestion),
    Set(QuestionSet),
}

fn scalar_bytes(
    engine: &Engine,
    ask: ScalarAsk,
    texts: Vec<String>,
    due: i64,
    kind: i32,
    token: &CancelToken,
) -> Result<Vec<u8>, String> {
    match (kind, ask) {
        (7, ScalarAsk::Set(set)) => annotate(engine, &set, texts, due, token),
        (2, ScalarAsk::Question(question)) => details(engine, &question, texts, due, token),
        (0 | 1, ScalarAsk::Question(question)) => {
            decisions(engine, &question, texts, due, kind, token)
        }
        _ => Err("thinkthen defect: the bridge got an unknown scalar kind".to_owned()),
    }
}

/// Evaluate one grouped decision, probability, details, or annotation call.
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
        let asked = asked(&settings)?;
        let engine = engines::engine_for(&asked, |_| probe(&settings))?;
        let (copied, cut) = engines::within_total(&asked, copied)?;
        run_detached(stop, move |token| {
            let bytes = scalar_bytes(&engine, ask, copied, deadline_ms, kind, &token)?;
            if let Some(error) = cut {
                return Err(error);
            }
            Ok(bytes)
        })
    })
}

fn try_answer(
    engine: &Engine,
    question: &LoadedQuestion,
    evidence: &str,
    due: i64,
    token: &CancelToken,
    cut: Option<errors::RowError>,
) -> Result<String, errors::RowError> {
    let options = options(due, token)
        .map_err(|_| errors::RowError::usage("the deadline is outside the supported range"))?;
    let details = match question {
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
            let engine = engines::engine_for_typed(&asked, |_| probe(&settings))?;
            let (_, cut) = engines::within_total_typed(&asked, vec![evidence.to_owned()])?;
            Ok((question, engine, evidence.to_owned(), cut))
        })();
        let result = match prepared {
            Err(error) => Err(error),
            Ok((question, engine, evidence, cut)) => run_detached(stop, move |token| {
                Ok(try_answer(
                    &engine,
                    &question,
                    &evidence,
                    deadline_ms,
                    &token,
                    cut,
                ))
            })?,
        };
        match result {
            Ok(value) => Ok(value.into_bytes()),
            Err(error) => error.value().map(String::into_bytes).ok_or(error.text),
        }
    })
}
