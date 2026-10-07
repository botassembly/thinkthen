//! Shared family edge: concrete native execution, with no host policy.
pub(crate) mod inputs;
pub(crate) mod questions;
pub(crate) mod stream;
use questions::Asked;
use serde::Deserialize;
use thinkthen::{Call, CallOptions, Engine, Error, ErrorKind, Facts};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Request {
    pub(crate) verb: String,
    pub(crate) question: questions::Question,
    pub(crate) input: inputs::Input,
    #[serde(default)]
    pub(crate) attempts: bool,
    #[serde(default)]
    pub(crate) cancel: bool,
    pub(crate) context: Option<String>,
}

pub(crate) fn usage(message: &str) -> Error {
    Error::new(ErrorKind::Usage, message)
}
fn defect(message: &str) -> Error {
    Error::new(ErrorKind::Defect, message)
}

pub(crate) fn parse(text: &str) -> Result<Request, Error> {
    serde_json::from_str(text).map_err(|_| usage("complete call requires a typed request"))
}

#[derive(serde::Serialize)]
struct Packet<'a, T> {
    results: &'a T,
    ordinals: Vec<Option<usize>>,
    inputs: Vec<inputs::InputView>,
    facts: thinkthen::CompleteFacts<'a>,
}
pub(crate) fn written<T: serde::Serialize>(
    call: Call<T>,
    ordinals: Vec<Option<usize>>,
    inputs: Vec<inputs::InputView>,
) -> Result<(String, Facts), Error> {
    let packet = Packet {
        results: call.value(),
        ordinals,
        inputs,
        facts: call
            .facts()
            .complete()
            .ok_or_else(|| defect("completed call has no complete facts"))?,
    };
    let value = serde_json::to_string(&packet)
        .map_err(|_| defect("complete result could not be written"))?;
    Ok((value, call.facts().clone()))
}
pub(crate) fn rows<T, R>(
    call: Call<Vec<thinkthen::CompleteRecord<T, R>>>,
    inputs: Vec<inputs::InputView>,
) -> Result<(String, Facts), Error>
where
    thinkthen::CompleteRecord<T, R>: serde::Serialize,
{
    let ordinals = call.value().iter().map(|r| Some(r.ordinal())).collect();
    written(call, ordinals, inputs)
}

/// The only dispatch is the caller's explicit named function.
pub(crate) fn execute(
    engine: &Engine,
    request: Request,
    options: CallOptions<'_>,
) -> Result<(String, Facts), Error> {
    let cancelled = || request.cancel;
    let options = if request.cancel {
        options.interrupt(&cancelled)
    } else {
        options
    };
    let options = if let Some(context) = request.context.as_deref() {
        options.context(context)
    } else {
        options
    };
    let asked = questions::load(&request.verb, request.question)?;
    let reading = asked.reading();
    let records = inputs::read(engine, request.input, reading, request.verb == "annotate")?;
    let inputs = records.iter().map(|r| r.original.view()).collect();
    let options = options.attempts(request.attempts);
    match (request.verb.as_str(), asked) {
        ("decide", Asked::Atomic(thinkthen::LoadedQuestion::Question(q))) => rows(
            engine.decide_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("decide", Asked::Atomic(thinkthen::LoadedQuestion::Banded(q))) => rows(
            engine.decide_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("choose", Asked::Atomic(thinkthen::LoadedQuestion::Question(q))) => rows(
            engine.choose_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("choose", Asked::Dynamic(q)) => rows(
            engine.choose_dynamic_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("tag", Asked::Atomic(thinkthen::LoadedQuestion::Question(q))) => rows(
            engine.tag_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("score", Asked::Atomic(thinkthen::LoadedQuestion::Question(q))) => rows(
            engine.score_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("filter", Asked::Atomic(thinkthen::LoadedQuestion::Question(q))) => rows(
            engine.filter_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("rank", Asked::Rank(q)) => rows(
            engine.rank_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("rank", Asked::SetRank(q)) => rows(
            engine.rank_set_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("find", Asked::Find(q, _)) => {
            let call = engine.find_records_complete_with(&q, records, options)?;
            let ordinal = match call.value().selection() {
                thinkthen::FindSelection::Unit(at) => Some(at),
                thinkthen::FindSelection::None => None,
            };
            written(call, vec![ordinal], inputs)
        }
        ("annotate", Asked::Set(q)) => rows(
            engine.annotate_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("recognize", Asked::Recognize(q, _)) => rows(
            engine.recognize_records_complete_with(&q, records, options)?,
            inputs,
        ),
        ("relate", Asked::Relate(q)) => written(
            engine.relate_records_complete_with(&q, records, options)?,
            vec![None],
            inputs,
        ),
        _ => Err(usage("question does not match the named function")),
    }
}
