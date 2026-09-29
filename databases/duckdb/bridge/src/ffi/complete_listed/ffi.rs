//! Complete choose, score and tag questions from the caller's SQL session.
#![allow(
    unsafe_code,
    reason = "the complete-question bridge copies caller-owned byte ranges"
)]

use thinkthen::{Engine, Judgment, LoadedQuestion, Question, QuestionKind};

use super::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, batch, copied_texts, probe,
    reply_boundary, run_detached, text,
};
use crate::{engines, errors};

fn question(argument: &str, kind: i32, from_file: bool) -> Result<Question, String> {
    if !from_file && !argument.starts_with('{') {
        return Err(errors::RowError::usage(
            "a complete listed question takes JSON or a named file",
        )
        .text);
    }
    let parsed = Question::from_json(argument).map_err(|error| {
        if from_file {
            errors::RowError::local("the question file has invalid content").text
        } else {
            errors::RowError::from(error).text
        }
    })?;
    let LoadedQuestion::Question(question) = parsed else {
        return Err(errors::RowError::usage("the question has another kind").text);
    };
    let expected = match kind {
        4 => QuestionKind::Choose,
        5 => QuestionKind::Score,
        6 => QuestionKind::Tag,
        _ => return Err("thinkthen defect: the bridge got an unknown listed kind".to_owned()),
    };
    if question.kind() != expected {
        return Err(errors::RowError::usage("the question has another kind").text);
    }
    Ok(question)
}

fn string(bytes: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let len = u32::try_from(value.len())
        .map_err(|_| "thinkthen defect: a listed value is too large".to_owned())?;
    bytes.extend_from_slice(&len.to_ne_bytes());
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

pub(super) fn run(
    engine: &Engine,
    question: &Question,
    texts: Vec<String>,
    options: thinkthen::CallOptions<'_>,
    total: Option<i64>,
) -> Result<Vec<u8>, String> {
    let rows = engine
        .details_many_with(question, texts, options)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| engines::call_error(error, total).text)?;
    let mut bytes = Vec::new();
    for row in rows {
        match row.value().value() {
            Judgment::Choice(Some(value)) => {
                bytes.push(1);
                string(&mut bytes, value)?;
            }
            Judgment::Choice(None) => bytes.push(0),
            Judgment::Score(value) => {
                bytes.push(2);
                bytes.extend_from_slice(&value.to_ne_bytes());
            }
            Judgment::Tags(values) => {
                bytes.push(3);
                let count = u32::try_from(values.len())
                    .map_err(|_| "thinkthen defect: too many listed values".to_owned())?;
                bytes.extend_from_slice(&count.to_ne_bytes());
                for value in values {
                    string(&mut bytes, value)?;
                }
            }
            Judgment::Decision(_) => {
                return Err("thinkthen defect: a listed question returned another kind".to_owned());
            }
        }
    }
    Ok(bytes)
}

/// Validate one resolved complete question without sending.
///
/// # Safety
/// The question bytes remain readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_complete_listed(
    bytes: *const u8,
    len: usize,
    kind: i32,
    from_file: i32,
) -> Reply {
    reply_boundary(|| {
        let argument = text(bytes, len)?;
        question(argument, kind, from_file != 0).map(|_| Vec::new())
    })
}

/// Ask one group of complete questions under the calling statement's stop scope.
///
/// # Safety
/// The question and text array remain readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_listed_group(
    question_bytes: *const u8,
    question_len: usize,
    texts: *const BridgeText,
    text_count: usize,
    deadline_ms: i64,
    kind: i32,
    settings: BridgeSettings,
    from_file: i32,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let argument = text(question_bytes, question_len)?;
        let texts = copied_texts(texts, text_count)?;
        let question = question(argument, kind, from_file != 0)?;
        let batch = batch(&settings)?;
        let asked = asked(&settings)?;
        let engine = engines::engine_for(&asked, |path| probe(&settings, path))?;
        let total = asked.max_requests_total;
        run_detached(stop, move |token| {
            let options = engines::options_for(deadline_ms, &token, total, batch.as_deref(), None)?;
            run(&engine, &question, texts, options, total)
        })
    })
}
