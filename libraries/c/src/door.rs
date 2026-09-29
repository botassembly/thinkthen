//! The typed doors' bodies on the Rust side: every argument arrives checked
//! and borrowed, and every answer leaves as a value the FFI edge writes.

use serde_json::Value;
use std::io::Read;
use thinkthen::{
    Answer, CallOptions, CancelToken, Engine, Entity, Facts, Judgment, LoadedQuestion,
    Probabilities, Question, Recognize, Relate,
};

use crate::Judgment as Reply;
use crate::failures::Failure;

/// The most records one `relate` takes.
const MOST_RELATED: usize = 255;

/// The header's outcome code for an answer.
pub(crate) const fn outcome(answer: Answer) -> i32 {
    match answer {
        Answer::Yes => 1,
        Answer::No => 0,
        Answer::Unsure => 2,
    }
}

/// The header's judgment for an answer and its probability of yes.
const fn reply(answer: Answer, probability: f64) -> Reply {
    Reply {
        outcome: outcome(answer),
        probability,
    }
}

/// The controls one `_opts` call carries: the budget under ADR 0041's rule
/// and the token, when the host passed one.
pub(crate) fn options(
    deadline_ms: i64,
    token: Option<&CancelToken>,
) -> Result<CallOptions<'_>, Failure> {
    let options = CallOptions::new().deadline_ms(deadline_ms)?;
    Ok(token.map_or(options, |token| options.cancel(token)))
}

/// A decide question: the question-file grammar when the text is a JSON
/// object, or the bare text of a decide question at the default cut.
pub(crate) fn question(text: &str) -> Result<LoadedQuestion, Failure> {
    let trimmed = text.trim();
    if trimmed.starts_with('{') {
        return Ok(Question::from_json(trimmed)?);
    }
    Ok(LoadedQuestion::Question(Question::decide(trimmed)?.cut()))
}

/// Validate one named question and retain its source JSON for the C caller.
pub(crate) fn question_file(path: &str) -> Result<String, Failure> {
    const LIMIT: u64 = 1_048_576;
    let file = std::fs::File::open(path)
        .map_err(|_| Failure::local("the question file could not be read"))?;
    let mut bytes = Vec::new();
    file.take(LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Failure::local("the question file could not be read"))?;
    if bytes.len() as u64 > LIMIT {
        return Err(Failure::local("the question file is too large"));
    }
    let source =
        String::from_utf8(bytes).map_err(|_| Failure::local("the question file is not UTF-8"))?;
    Question::from_json(&source)
        .map_err(|_| Failure::local("the question file has invalid question content"))?;
    Ok(source)
}

/// One judgment with its probability of yes.
pub(crate) fn decide(
    engine: &Engine,
    question: &LoadedQuestion,
    text: &str,
    options: CallOptions<'_>,
) -> Result<Reply, Failure> {
    decide_with_facts(engine, question, text, options, None)
}

pub(crate) fn decide_with_facts(
    engine: &Engine,
    question: &LoadedQuestion,
    text: &str,
    options: CallOptions<'_>,
    completed: Option<&mut Option<Facts>>,
) -> Result<Reply, Failure> {
    let details = match question {
        LoadedQuestion::Question(asked) if asked.kind() != thinkthen::QuestionKind::Decide => {
            return Err(Failure::usage("decide takes a decide question"));
        }
        LoadedQuestion::Question(asked) => engine.details_with(asked, text, options)?,
        LoadedQuestion::Banded(asked) => engine.details_with(asked, text, options)?,
    };
    if let Some(completed) = completed {
        *completed = Some(details.facts().clone());
    }
    let details = details.into_value();
    match (details.value(), details.probabilities()) {
        (Judgment::Decision(answer), Probabilities::YesNo { yes }) => Ok(reply(*answer, *yes)),
        _ => Err(Failure::defect(
            "a decide answer held no probability of yes",
        )),
    }
}

/// Every text's judgment, in input order, or the call's failure and no rows.
pub(crate) fn decide_many(
    engine: &Engine,
    question: &LoadedQuestion,
    texts: &[&str],
    options: CallOptions<'_>,
) -> Result<Vec<Reply>, Failure> {
    decide_many_with_facts(engine, question, texts, options, None)
}

pub(crate) fn decide_many_with_facts(
    engine: &Engine,
    question: &LoadedQuestion,
    texts: &[&str],
    options: CallOptions<'_>,
    completed: Option<&mut Option<Facts>>,
) -> Result<Vec<Reply>, Failure> {
    let rows: Result<Vec<_>, _> = match question {
        LoadedQuestion::Question(asked) => {
            let mut batch = engine.decide_many_with(asked, texts.iter().copied(), options);
            let rows = batch
                .by_ref()
                .map(|row| row.map(|row| reply(*row.value(), row.probability())))
                .collect();
            if let Some(completed) = completed {
                *completed = batch.facts().cloned();
            }
            rows
        }
        LoadedQuestion::Banded(asked) => {
            let mut batch = engine.decide_many_with(asked, texts.iter().copied(), options);
            let rows = batch
                .by_ref()
                .map(|row| row.map(|row| reply(*row.value(), row.probability())))
                .collect();
            if let Some(completed) = completed {
                *completed = batch.facts().cloned();
            }
            rows
        }
    };
    let rows = rows?;
    if rows.len() == texts.len() {
        Ok(rows)
    } else {
        Err(Failure::defect(
            "the engine answered another number of records",
        ))
    }
}

/// `{"entities": [...], "relations": [...]}` for one text.
pub(crate) fn recognize(
    engine: &Engine,
    spec: &str,
    text: &str,
    options: CallOptions<'_>,
) -> Result<(String, Facts), Failure> {
    let mut completed = None;
    let json = recognize_with_facts(engine, spec, text, options, &mut completed)?;
    Ok((
        json,
        completed.ok_or_else(|| Failure::defect("a completed recognition held no facts"))?,
    ))
}

pub(crate) fn recognize_with_facts(
    engine: &Engine,
    spec: &str,
    text: &str,
    options: CallOptions<'_>,
    completed: &mut Option<Facts>,
) -> Result<String, Failure> {
    let ask = Recognize::from_json(spec)?;
    let call = engine.recognize_with(&ask, text, options)?;
    *completed = Some(call.facts().clone());
    Ok(call.value().to_json())
}

/// Refuse a relate call past [`MOST_RELATED`] records.
pub(crate) fn capped(count: usize) -> Result<(), Failure> {
    if count > MOST_RELATED {
        return Err(Failure::usage("relate takes at most 255 records"));
    }
    Ok(())
}

/// Relate's records as entities, under the cap.
pub(crate) fn entities(records: &[&str]) -> Result<Vec<Entity>, Failure> {
    capped(records.len())?;
    records.iter().map(|record| entity(record)).collect()
}

/// One relate record: a JSON object with a string `name` and `kind`, read
/// at the default `/name` and `/kind` as the command reads JSONL. A name
/// `recognize` found carries `text` in place of `name`, and is read by it.
fn entity(record: &str) -> Result<Entity, Failure> {
    let value: Value =
        serde_json::from_str(record).map_err(|_| Failure::usage("a relate record is not JSON"))?;
    let name = value.get("name").or_else(|| value.get("text"));
    match (name, value.get("kind")) {
        (Some(Value::String(name)), Some(Value::String(kind))) => Ok(Entity::new(name, kind)?),
        _ => Err(Failure::usage(
            "a relate record is a JSON object with a string name and a string kind",
        )),
    }
}

/// `{"edges": [...]}`, one edge as the command prints it, in rule order.
pub(crate) fn relate(
    engine: &Engine,
    spec: &str,
    entities: Vec<Entity>,
    options: CallOptions<'_>,
) -> Result<(String, Facts), Failure> {
    let mut completed = None;
    let json = relate_with_facts(engine, spec, entities, options, &mut completed)?;
    Ok((
        json,
        completed.ok_or_else(|| Failure::defect("a completed relation held no facts"))?,
    ))
}

pub(crate) fn relate_with_facts(
    engine: &Engine,
    spec: &str,
    entities: Vec<Entity>,
    options: CallOptions<'_>,
    completed: &mut Option<Facts>,
) -> Result<String, Failure> {
    let ask = Relate::from_json(spec)?;
    let call = engine.relate_with(&ask, entities, options)?;
    *completed = Some(call.facts().clone());
    let edges: Vec<String> = call.value().iter().map(thinkthen::Edge::to_json).collect();
    Ok(format!("{{\"edges\":[{}]}}", edges.join(",")))
}

/// Refuse a null `out` or `out_len`, which success always writes.
pub(crate) fn outs(out: *mut *mut std::ffi::c_char, out_len: *mut usize) -> Result<(), Failure> {
    if out.is_null() {
        return Err(Failure::usage("a null out pointer"));
    }
    if out_len.is_null() {
        return Err(Failure::usage("a null out_len pointer"));
    }
    Ok(())
}
