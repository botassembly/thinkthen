//! The typed doors' bodies on the Rust side: every argument arrives checked
//! and borrowed, and every answer leaves as a value the FFI edge writes.

use serde_json::Value;
use thinkthen::{
    Answer, CallOptions, CancelToken, Engine, Entity, Judgment, LoadedQuestion, Probabilities,
    Question, Recognize, Relate,
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
    let options = CallOptions::new().deadline_millis(deadline_ms)?;
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

/// One judgment with its probability of yes.
pub(crate) fn decide(
    engine: &Engine,
    question: &LoadedQuestion,
    text: &str,
    options: CallOptions<'_>,
) -> Result<Reply, Failure> {
    let details = match question {
        LoadedQuestion::Question(asked) if asked.kind() != thinkthen::QuestionKind::Decide => {
            return Err(Failure::usage("decide takes a decide question"));
        }
        LoadedQuestion::Question(asked) => engine.details_with(asked, text, options)?,
        LoadedQuestion::Banded(asked) => engine.details_with(asked, text, options)?,
    };
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
    let rows: Result<Vec<_>, _> = match question {
        LoadedQuestion::Question(asked) => engine
            .decide_many_with(asked, texts.iter().copied(), options)
            .map(|row| row.map(|row| reply(*row.value(), row.probability())))
            .collect(),
        LoadedQuestion::Banded(asked) => engine
            .decide_many_with(asked, texts.iter().copied(), options)
            .map(|row| row.map(|row| reply(*row.value(), row.probability())))
            .collect(),
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
) -> Result<String, Failure> {
    let ask = Recognize::from_json(spec)?;
    Ok(engine.recognize_with(&ask, text, options)?.to_json())
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
/// at the default `/name` and `/kind` as the command reads JSONL.
fn entity(record: &str) -> Result<Entity, Failure> {
    let value: Value =
        serde_json::from_str(record).map_err(|_| Failure::usage("a relate record is not JSON"))?;
    match (value.get("name"), value.get("kind")) {
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
) -> Result<String, Failure> {
    let ask = Relate::from_json(spec)?;
    let edges = engine.relate_with(&ask, entities, options)?;
    let edges: Vec<String> = edges.iter().map(thinkthen::Edge::to_json).collect();
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
