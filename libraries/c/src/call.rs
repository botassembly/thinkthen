//! The JSON interface's grammar and its call-value writer.
//!
//! A request names one verb of ten and up to six envelope keys. Every other
//! key forms the question object, which the public `from_json` readers
//! validate, so the interface holds no question grammar of its own. The wrapper's
//! value is the command's bare answer, and `tests/interface/` compares its bytes.

use std::collections::BTreeMap;
use std::sync::Mutex;

use serde_json::value::RawValue;
use thinkthen::{
    AttemptObservation, BatchSetting, CallOptions, CallReply, CancelToken, Engine,
    RequestEnvironment, RequestOutcome,
};

use crate::failures::Failure;
use crate::typed;

mod canonical;
mod frozen;
mod projection;
pub(crate) use thinkthen_host::source;

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
const ENVELOPE: [(&str, &[&str]); 8] = [
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
    ("source", &VERBS),
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
    let mut members: Members = serde_json::from_str(text)
        .map_err(|_| Failure::usage("the request is not one JSON object"))?;
    // The frozen map grammar routes by the effective last top-level value.
    // Canonical decoding must receive the original bytes, including duplicates.
    if members
        .get("schema")
        .and_then(|raw| serde_json::from_str::<String>(raw.get()).ok())
        .is_some_and(|schema| schema.starts_with("thinkthen.request/"))
    {
        return canonical::call(engine, text, typed::options(deadline_ms, token)?);
    }
    members.remove("schema");
    if flag(&members, "usage")? {
        if members.len() > 1 {
            return Err(Failure::usage("a usage request holds no other key"));
        }
        return written(serde_json::to_string(&engine.usage()));
    }
    let request = split(members)?;
    let admitted = frozen::translate(&request)?;
    let options = controls(&request, typed::options(deadline_ms, token)?)?;
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
    let call = match engine
        .execute_request(
            &admitted,
            RequestEnvironment {
                controls: options,
                feed: None,
            },
        )
        .map_err(|error| frozen_failure(error, &request))?
    {
        RequestOutcome::Complete(call) => call,
        RequestOutcome::Failed { error, .. } => return Err(frozen_failure(error, &request)),
    };
    let value = projection::value(call.value(), &request)
        .map_err(|failure| failure.completed(Some(call.facts())))?;
    let facts = call.facts().clone();
    let attempts = if request.attempts {
        let mut events = attempts
            .into_inner()
            .map_err(|_| Failure::defect("attempt collection failed"))?;
        events.sort_by_key(AttemptObservation::ordinal);
        Some(events)
    } else {
        None
    };
    written(serde_json::to_string(&CallReply::new(
        value, facts, attempts,
    )?))
}

fn frozen_failure(error: thinkthen::Error, request: &Request) -> Failure {
    let message = if request.envelope.contains_key("source") {
        match error.source_budget_exceeded() {
            Some(thinkthen::SourceBudgetExceeded::Rank) => Some("source rank input exceeds 16 MiB"),
            Some(thinkthen::SourceBudgetExceeded::Relate) => {
                Some("source relate input exceeds 16 MiB")
            }
            _ => None,
        }
    } else {
        None
    };
    let failure = Failure::from(error);
    match message {
        Some(message) => failure.with_message(message),
        None => failure,
    }
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
    check_source(&envelope)?;
    let call = envelope
        .get("call")
        .map(|raw| {
            let value: serde_json::Value = serde_json::from_str(raw.get())
                .map_err(|_| Failure::usage("call is one JSON object"))?;
            let object = value
                .as_object()
                .ok_or_else(|| Failure::usage("call is one JSON object"))?;
            let many = (envelope.contains_key("records") || envelope.contains_key("source"))
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

/// A interface flag read by value: `true` or `false`, absent as `false`.
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

/// One value's JSON text as the crate wrote it, to serialize inside a reply.
pub(crate) fn raw(text: String) -> Result<Box<RawValue>, Failure> {
    RawValue::from_string(text).map_err(|_| Failure::defect("an answer could not be written"))
}

/// One typed value's JSON text, or the interface's defect.
pub(crate) fn written(text: serde_json::Result<String>) -> Result<String, Failure> {
    text.map_err(|_| Failure::defect("an answer could not be written"))
}

fn check_source(envelope: &Members) -> Result<(), Failure> {
    if envelope.contains_key("source")
        && ["evidence", "records", "units"]
            .iter()
            .any(|key| envelope.contains_key(*key))
    {
        return Err(Failure::usage(
            "source replaces evidence, records, and units",
        ));
    }
    Ok(())
}
