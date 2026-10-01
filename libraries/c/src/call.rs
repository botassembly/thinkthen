//! The JSON door's grammar and its call-value writer.
//!
//! A request names one verb of ten and up to six envelope keys. Every other
//! key forms the question object, which the public `from_json` readers
//! validate, so the door holds no question grammar of its own. The wrapper's
//! value is the command's bare answer, and `tests/door/` compares its bytes.

use std::collections::BTreeMap;
use std::sync::Mutex;

use serde_json::value::RawValue;
use thinkthen::{
    AttemptObservation, BatchSetting, CallOptions, CancelToken, DoorReply, Engine, Facts, Judgment,
    LoadedQuestion, Question, QuestionSet, Ranked,
};

use crate::door;
use crate::failures::Failure;

mod records;

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
const ENVELOPE: [(&str, &[&str]); 7] = [
    (
        "evidence",
        &["decide", "choose", "score", "tag", "recognize"],
    ),
    (
        "records",
        &[
            "decide", "choose", "score", "tag", "filter", "rank", "annotate", "relate",
        ],
    ),
    ("units", &["find"]),
    ("details", &["decide", "choose", "score", "tag"]),
    ("usage", &[]),
    ("call", &VERBS),
    ("attempts", &VERBS),
];

type Members = BTreeMap<String, Box<RawValue>>;

/// One parsed request: its verb, its envelope keys, and its question object.
struct Request {
    verb: String,
    envelope: Members,
    question: Members,
    call: Option<Controls>,
    attempts: bool,
}

struct Controls {
    batch: Option<BatchSetting>,
    context: Option<String>,
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
        return written(serde_json::to_string(&engine.usage()));
    }
    let request = split(members)?;
    let options = controls(&request, door::options(deadline_ms, token)?)?;
    let attempts = Mutex::new(Vec::<AttemptObservation>::new());
    let collect = |event| {
        if let Ok(mut held) = attempts.lock() {
            held.push(event);
        }
    };
    let options = if request.attempts {
        options.observe_attempt(&collect)
    } else {
        options
    };
    let (value, facts) = answer(engine, &request, options)?;
    let attempts = if request.attempts {
        let mut events = attempts
            .into_inner()
            .map_err(|_| Failure::defect("attempt collection failed"))?;
        events.sort_by_key(AttemptObservation::ordinal);
        Some(events)
    } else {
        None
    };
    written(serde_json::to_string(&DoorReply::new(
        value, facts, attempts,
    )?))
}

fn controls<'a>(
    request: &'a Request,
    mut options: CallOptions<'a>,
) -> Result<CallOptions<'a>, Failure> {
    if let Some(call) = request.call.as_ref() {
        if let Some(batch) = call.batch {
            options = options.batch(batch);
        }
        if let Some(context) = call.context.as_deref() {
            options = options.context(context);
        }
    }
    Ok(options)
}

fn split(members: Members) -> Result<Request, Failure> {
    let attempts = match members.get("attempts") {
        None => false,
        Some(raw) => match serde_json::from_str::<bool>(raw.get()) {
            Ok(true) => true,
            _ => return Err(Failure::usage("attempts takes true")),
        },
    };
    if attempts && members.contains_key("usage") {
        return Err(Failure::usage("an attempts request takes no usage key"));
    }
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
    let call = envelope
        .get("call")
        .map(|raw| {
            let value: serde_json::Value = serde_json::from_str(raw.get())
                .map_err(|_| Failure::usage("call is one JSON object"))?;
            let object = value
                .as_object()
                .ok_or_else(|| Failure::usage("call is one JSON object"))?;
            let many = envelope.contains_key("records")
                && matches!(
                    verb.as_str(),
                    "decide" | "choose" | "score" | "tag" | "filter" | "rank" | "annotate"
                );
            if !many {
                return Err(Failure::usage(format!("{verb} takes no call key")));
            }
            let mut call = Controls {
                batch: None,
                context: None,
            };
            for (key, value) in object {
                match key.as_str() {
                    "batch" if many => call.batch = Some(crate::settings::batch(value)?),
                    "context" if many && verb != "annotate" => {
                        call.context = Some(
                            value
                                .as_str()
                                .ok_or_else(|| Failure::usage("call context is text"))?
                                .to_owned(),
                        );
                    }
                    "batch" | "context" => {
                        return Err(Failure::usage(format!("{verb} takes no call {key}")));
                    }
                    _ => return Err(Failure::usage(format!("call has unknown key {key}"))),
                }
            }
            Ok(call)
        })
        .transpose()?;
    Ok(Request {
        verb,
        envelope,
        question,
        call,
        attempts,
    })
}

fn answer(
    engine: &Engine,
    request: &Request,
    options: CallOptions<'_>,
) -> Result<(String, Facts), Failure> {
    let verb = request.verb.as_str();
    match verb {
        "decide" | "choose" | "score" | "tag" => {
            let detailed = flag(&request.envelope, "details")?;
            if request.envelope.contains_key("records") {
                return records::judgments(engine, request, options, detailed);
            }
            let evidence = member(request, "evidence", |raw| {
                serde_json::from_str::<String>(raw)
            })?;
            let call = match Question::from_json(&object(&request.question)?)? {
                LoadedQuestion::Question(asked) => engine.details_with(&asked, &evidence, options),
                LoadedQuestion::Banded(asked) => engine.details_with(&asked, &evidence, options),
            }?;
            if detailed {
                return Ok((call.value().to_json(), call.facts().clone()));
            }
            Ok((bare(call.value().value())?, call.facts().clone()))
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
            let mut batch = engine.filter_with(&asked, records.iter().map(String::as_str), options);
            let kept = batch.by_ref().collect::<Result<Vec<_>, _>>()?;
            let facts = batch
                .facts()
                .cloned()
                .ok_or_else(|| Failure::defect("completed filter has no facts"))?;
            Ok((written(serde_json::to_string(&kept))?, facts))
        }
        "rank" => {
            let asked = Question::rank(&alone(&request.verb, &request.question)?)?;
            let records = member(request, "records", |raw| {
                serde_json::from_str::<Vec<String>>(raw)
            })?;
            let ranked = engine.rank_with(&asked, records.iter().map(String::as_str), options)?;
            Ok((
                written(serde_json::to_string(
                    &ranked.value().iter().map(Ranked::row).collect::<Vec<_>>(),
                ))?,
                ranked.facts().clone(),
            ))
        }
        "find" => {
            let mut question = request.question.clone();
            let none = question.remove("none").map_or(Ok(false), |raw| {
                serde_json::from_str::<bool>(raw.get())
                    .map_err(|_| Failure::usage("find takes `none` as true or false"))
            })?;
            let asked = Question::find(&alone(&request.verb, &question)?)?;
            let asked = if none { asked.offering_none()? } else { asked };
            let units = member(request, "units", |raw| {
                serde_json::from_str::<Vec<String>>(raw)
            })?;
            let found = engine.find_with(&asked, units.iter().map(String::as_str), options)?;
            Ok((
                written(serde_json::to_string(&found.value().picked()))?,
                found.facts().clone(),
            ))
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
) -> Result<(String, Facts), Failure> {
    let set = match request.question.get("annotate") {
        Some(set) if request.question.len() == 1 => QuestionSet::from_json(set.get())?,
        _ => return Err(Failure::usage("annotate takes its question set alone")),
    };
    let records = member(request, "records", |raw| {
        serde_json::from_str::<Vec<String>>(raw)
    })?;
    let mut batch = engine.annotate_with(&set, records.iter().map(String::as_str), options);
    let rows = batch
        .by_ref()
        .map(|row| raw(row?.value_json()))
        .collect::<Result<Vec<_>, _>>()?;
    let facts = batch
        .facts()
        .cloned()
        .ok_or_else(|| Failure::defect("completed annotate has no facts"))?;
    Ok((written(serde_json::to_string(&rows))?, facts))
}

/// The question text of `rank` or `find`, which read no other key.
fn alone(verb: &str, question: &Members) -> Result<String, Failure> {
    match question.get(verb) {
        Some(text) if question.len() == 1 => serde_json::from_str::<String>(text.get())
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
        Judgment::Decision(answer) => written(serde_json::to_string(&match answer {
            thinkthen::Answer::Yes => Some(true),
            thinkthen::Answer::No => Some(false),
            thinkthen::Answer::Unsure => None,
        })),
        Judgment::Choice(pick) => written(serde_json::to_string(pick)),
        Judgment::Score(position) => written(serde_json::to_string(position)),
        Judgment::Tags(labels) => written(serde_json::to_string(labels)),
    }
}

/// One value's JSON text as the crate wrote it, to serialize inside a reply.
pub(crate) fn raw(text: String) -> Result<Box<RawValue>, Failure> {
    RawValue::from_string(text).map_err(|_| Failure::defect("an answer could not be written"))
}

/// One typed value's JSON text, or the door's defect.
pub(crate) fn written(text: serde_json::Result<String>) -> Result<String, Failure> {
    text.map_err(|_| Failure::defect("an answer could not be written"))
}
