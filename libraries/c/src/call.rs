//! The JSON door's grammar and its bare-value writer.
//!
//! A request names one verb of ten and up to five envelope keys. Every other
//! key forms the question object, which the public `from_json` readers
//! validate, so the door holds no question grammar of its own. Each answer
//! is the bare value the command prints, and `tests/door/` holds the
//! writer to the command's bytes on the shared cases.

use std::collections::BTreeMap;

use serde_json::json;
use serde_json::value::RawValue;
use thinkthen::{
    CallOptions, CancelToken, Engine, Judgment, LoadedQuestion, Question, QuestionSet,
};

use crate::door;
use crate::failures::Failure;

const VERBS: [&str; 10] = [
    "decide",
    "choose",
    "score",
    "tag",
    "filter",
    "rank",
    "find",
    "annotate",
    "recognize",
    "relate",
];

/// Each envelope key and the verbs it goes with.
const ENVELOPE: [(&str, &[&str]); 5] = [
    (
        "evidence",
        &["decide", "choose", "score", "tag", "recognize"],
    ),
    ("records", &["filter", "rank", "annotate", "relate"]),
    ("units", &["find"]),
    ("details", &["decide", "choose", "score", "tag"]),
    ("usage", &[]),
];

type Members = BTreeMap<String, Box<RawValue>>;

/// One parsed request: its verb, its envelope keys, and its question object.
struct Request {
    verb: String,
    envelope: Members,
    question: Members,
}

/// Answer one JSON request.
pub(crate) fn call(
    engine: &Engine,
    text: &str,
    deadline_ms: i64,
    token: Option<&CancelToken>,
) -> Result<String, Failure> {
    let members: Members = serde_json::from_str(text)
        .map_err(|_| Failure::usage("the request is not one JSON object"))?;
    if flag(&members, "usage")? {
        if members.len() > 1 {
            return Err(Failure::usage("a usage request holds no other key"));
        }
        let counts = engine.usage();
        return Ok(format!(
            "{{\"requests_sent\":{},\"input_tokens\":{},\"output_tokens\":{},\"cache_answers\":{}}}",
            counts.requests_sent(),
            counts.input_tokens(),
            counts.output_tokens(),
            counts.cache_answers()
        ));
    }
    let request = split(members)?;
    let options = door::options(deadline_ms, token)?;
    answer(engine, &request, options)
}

fn split(members: Members) -> Result<Request, Failure> {
    let named: Vec<&str> = VERBS
        .into_iter()
        .filter(|verb| members.contains_key(*verb))
        .collect();
    let verb = match named.as_slice() {
        [verb] => (*verb).to_owned(),
        [] => {
            return Err(Failure::usage(format!(
                "the request names no verb: {}",
                VERBS.join(", ")
            )));
        }
        _ => return Err(Failure::usage("the request names more than one verb")),
    };
    let (envelope, question): (Members, Members) = members
        .into_iter()
        .partition(|(key, _)| ENVELOPE.iter().any(|(held, _)| held == key));
    for key in envelope.keys() {
        let fits = ENVELOPE
            .iter()
            .any(|(held, verbs)| held == key && verbs.contains(&verb.as_str()));
        if !fits {
            return Err(Failure::usage(format!("{verb} takes no {key} key")));
        }
    }
    Ok(Request {
        verb,
        envelope,
        question,
    })
}

fn answer(engine: &Engine, request: &Request, options: CallOptions<'_>) -> Result<String, Failure> {
    let verb = request.verb.as_str();
    match verb {
        "decide" | "choose" | "score" | "tag" => {
            let evidence = member(request, "evidence", |raw| {
                serde_json::from_str::<String>(raw)
            })?;
            let details = match Question::from_json(&object(&request.question)?)? {
                LoadedQuestion::Question(asked) => engine.details_with(&asked, &evidence, options),
                LoadedQuestion::Banded(asked) => engine.details_with(&asked, &evidence, options),
            }?;
            if flag(&request.envelope, "details")? {
                return Ok(details.to_json());
            }
            bare(details.value())
        }
        "filter" => {
            let mut question = request.question.clone();
            if let Some(text) = question.remove("filter") {
                question.insert("decide".to_owned(), text);
            }
            let LoadedQuestion::Question(asked) = Question::from_json(&object(&question)?)? else {
                return Err(Failure::usage("filter keeps a record at a cut, not a band"));
            };
            let records = member(request, "records", |raw| {
                serde_json::from_str::<Vec<String>>(raw)
            })?;
            let kept: Result<Vec<&str>, _> = engine
                .filter_with(&asked, records.iter().map(String::as_str), options)
                .collect();
            write(&json!(kept?))
        }
        "rank" => {
            let asked = Question::rank(&alone(request)?)?;
            let records = member(request, "records", |raw| {
                serde_json::from_str::<Vec<String>>(raw)
            })?;
            let ranked = engine.rank_with(&asked, records.iter().map(String::as_str), options)?;
            write(&json!(
                ranked.iter().map(|row| *row.input()).collect::<Vec<_>>()
            ))
        }
        "find" => {
            let asked = Question::find(&alone(request)?)?;
            let units = member(request, "units", |raw| {
                serde_json::from_str::<Vec<String>>(raw)
            })?;
            let found = engine.find_with(&asked, units.iter().map(String::as_str), options)?;
            write(&json!(found.selected()))
        }
        "annotate" => annotate(engine, request, options),
        "recognize" => {
            let evidence = member(request, "evidence", |raw| {
                serde_json::from_str::<String>(raw)
            })?;
            door::recognize(engine, &object(&request.question)?, &evidence, options)
        }
        _ => {
            let records = member(request, "records", |raw| {
                serde_json::from_str::<Vec<Box<RawValue>>>(raw)
            })?;
            let records: Vec<&str> = records.iter().map(|record| record.get()).collect();
            let entities = door::entities(&records)?;
            door::relate(engine, &object(&request.question)?, entities, options)
        }
    }
}

fn annotate(
    engine: &Engine,
    request: &Request,
    options: CallOptions<'_>,
) -> Result<String, Failure> {
    let set = match request.question.get("annotate") {
        Some(set) if request.question.len() == 1 => QuestionSet::from_json(set.get())?,
        _ => return Err(Failure::usage("annotate takes its question set alone")),
    };
    let records = member(request, "records", |raw| {
        serde_json::from_str::<Vec<String>>(raw)
    })?;
    let rows: Result<Vec<String>, _> = engine
        .annotate_with(&set, records.iter().map(String::as_str), options)
        .map(|row| row.map(|row| row.value_json()))
        .collect();
    Ok(format!("[{}]", rows?.join(",")))
}

/// The question text of `rank` or `find`, which read no other key.
fn alone(request: &Request) -> Result<String, Failure> {
    let verb = &request.verb;
    match request.question.get(verb) {
        Some(text) if request.question.len() == 1 => serde_json::from_str::<String>(text.get())
            .map_err(|_| Failure::usage(format!("{verb} takes its question as a string"))),
        _ => Err(Failure::usage(format!(
            "{verb} takes its question text alone"
        ))),
    }
}

/// One envelope member the verb needs, read as `T`.
fn member<T>(
    request: &Request,
    key: &str,
    read: impl FnOnce(&str) -> serde_json::Result<T>,
) -> Result<T, Failure> {
    let wanted = || {
        let shape = match key {
            "evidence" => "one string",
            _ if request.verb == "relate" => "an array of JSON records",
            _ => "an array of strings",
        };
        Failure::usage(format!("{} takes {key} as {shape}", request.verb))
    };
    let raw = request.envelope.get(key).ok_or_else(wanted)?;
    read(raw.get()).map_err(|_| wanted())
}

/// A door flag read by value: `true` or `false`, absent as `false`.
fn flag(members: &Members, key: &str) -> Result<bool, Failure> {
    members.get(key).map_or(Ok(false), |raw| {
        serde_json::from_str::<bool>(raw.get())
            .map_err(|_| Failure::usage(format!("the {key} key takes true or false")))
    })
}

/// The question object as JSON text for a `from_json` reader.
fn object(question: &Members) -> Result<String, Failure> {
    serde_json::to_string(question).map_err(|_| Failure::defect("a question could not be written"))
}

/// One judgment's bare value, as the command prints it.
fn bare(value: &Judgment) -> Result<String, Failure> {
    match value {
        Judgment::Decision(answer) => write(&json!(match answer {
            thinkthen::Answer::Yes => Some(true),
            thinkthen::Answer::No => Some(false),
            thinkthen::Answer::Unsure => None,
        })),
        Judgment::Choice(pick) => write(&json!(pick)),
        Judgment::Score(position) => write(&json!(position)),
        Judgment::Tags(labels) => write(&json!(labels)),
    }
}

fn write(value: &serde_json::Value) -> Result<String, Failure> {
    serde_json::to_string(value).map_err(|_| Failure::defect("an answer could not be written"))
}
