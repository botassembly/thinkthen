//! The typed interfaces' bodies on the Rust side: every argument arrives checked
//! and borrowed, and every answer leaves as a value the FFI edge writes.

use crate::ffi::values as abi;
use thinkthen::{
    Answer, CallOptions, CancelToken, Engine, Entity, Facts, LoadedQuestion, Probabilities,
    Question, QuestionFileError, Recognize, Relate,
};

use crate::Judgment as Reply;
use crate::failures::Failure;

/// The most records one `relate` takes.
const MOST_RELATED: usize = 255;

/// The header's outcome code for an answer.
pub(crate) const fn outcome(answer: Answer) -> i32 {
    match answer {
        Answer::Yes => abi::THINKTHEN_YES,
        Answer::No => abi::THINKTHEN_NO,
        Answer::Unsure => abi::THINKTHEN_UNSURE,
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
    let source = thinkthen::read_question_file(path).map_err(|reason| {
        Failure::local(match reason {
            QuestionFileError::Unreadable(_) => "the question file could not be read",
            QuestionFileError::TooLarge => "the question file is too large",
            QuestionFileError::NotUtf8 => "the question file is not UTF-8",
        })
    })?;
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
    let definition = decide_definition(question)?;
    if text.trim().is_empty() {
        return Err(Failure::usage("evidence is text, not white space"));
    }
    let call = execute(
        engine,
        definition,
        text_input(text.to_owned()),
        options,
        None,
    )?;
    let thinkthen::RequestValue::Decisions(rows) = call.value() else {
        return Err(Failure::defect(
            "a decide request returned another answer kind",
        ));
    };
    let details = rows
        .first()
        .ok_or_else(|| Failure::defect("a decide request held no answer"))?
        .result();
    if let Some(completed) = completed {
        *completed = Some(call.facts().clone());
    }
    match details.probabilities() {
        Probabilities::YesNo { yes } => Ok(reply(details.value(), yes)),
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
    let definition = decide_definition(question)?;
    let call = execute(
        engine,
        definition,
        thinkthen::RequestInput::Feed {
            name: "c-array".into(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
        options,
        Some(thinkthen::RequestFeed::new(
            "c-array",
            texts.iter().map(|text| {
                Ok(item(thinkthen::RequestOriginal::Text {
                    text: (*text).to_owned(),
                }))
            }),
        )),
    )?;
    if let Some(completed) = completed {
        *completed = Some(call.facts().clone());
    }
    let thinkthen::RequestValue::Decisions(values) = call.value() else {
        return Err(Failure::defect(
            "a decide request returned another answer kind",
        ));
    };
    let rows = values
        .iter()
        .map(|row| match row.result().probabilities() {
            Probabilities::YesNo { yes } => Ok(reply(row.result().value(), yes)),
            _ => Err(Failure::defect(
                "a decide answer held no probability of yes",
            )),
        })
        .collect::<Result<Vec<_>, Failure>>()?;
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
    let call = execute(
        engine,
        thinkthen::RequestDefinition::Recognition(ask),
        text_input(text.to_owned()),
        options,
        None,
    )?;
    *completed = Some(call.facts().clone());
    let thinkthen::RequestValue::Recognized(rows) = call.value() else {
        return Err(Failure::defect(
            "a recognition request returned another answer kind",
        ));
    };
    Ok(rows
        .first()
        .ok_or_else(|| Failure::defect("a recognition request held no answer"))?
        .result()
        .value()
        .to_json())
}

/// Refuse a relate call past [`MOST_RELATED`] records.
pub(crate) fn capped(count: usize) -> Result<(), Failure> {
    if count > MOST_RELATED {
        return Err(Failure::usage("relate takes at most 255 records"));
    }
    Ok(())
}

/// Relate's records as entities, under the cap, each read through the
/// crate's one record parser.
pub(crate) fn entities(records: &[&str]) -> Result<Vec<Entity>, Failure> {
    capped(records.len())?;
    let read = records.iter().map(|record| Entity::from_record(record));
    Ok(read.collect::<Result<_, _>>()?)
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

/// The interface's relate value: each edge as the crate writes it.
#[derive(serde::Serialize)]
pub(crate) struct Edges {
    pub(crate) edges: Vec<Box<serde_json::value::RawValue>>,
}

pub(crate) fn relate_with_facts(
    engine: &Engine,
    spec: &str,
    entities: Vec<Entity>,
    options: CallOptions<'_>,
    completed: &mut Option<Facts>,
) -> Result<String, Failure> {
    let ask = Relate::from_json(spec)?;
    let items = entities
        .iter()
        .map(|entity| {
            Ok(item(thinkthen::RequestOriginal::Json {
                value: thinkthen::RawRecord::json(&crate::call::written(serde_json::to_string(
                    &serde_json::json!({"name":entity.name(),"kind":entity.kind()}),
                ))?)?,
            }))
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let call = execute(
        engine,
        thinkthen::RequestDefinition::Relate(ask),
        thinkthen::RequestInput::Records { items },
        options,
        None,
    )?;
    *completed = Some(call.facts().clone());
    let thinkthen::RequestValue::Related(row) = call.value() else {
        return Err(Failure::defect(
            "a relation request returned another answer kind",
        ));
    };
    let edges = row
        .result()
        .value()
        .iter()
        .map(|edge| crate::call::raw(edge.to_json()))
        .collect::<Result<_, _>>()?;
    crate::call::written(serde_json::to_string(&Edges { edges }))
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

/// Execute one prepared question through shared admission and execution.
pub(crate) fn execute(
    engine: &Engine,
    definition: thinkthen::RequestDefinition,
    input: thinkthen::RequestInput,
    controls: CallOptions<'_>,
    feed: Option<thinkthen::RequestFeed<'_>>,
) -> Result<thinkthen::Call<thinkthen::RequestValue>, Failure> {
    let request =
        thinkthen::Request::for_definition(definition, input, thinkthen::RequestOptions::default())
            .admit()?;
    Ok(executed(
        engine,
        &request,
        thinkthen::RequestEnvironment { controls, feed },
    )?)
}

/// Project shared execution without losing native failure facts.
pub(crate) fn executed(
    engine: &Engine,
    request: &thinkthen::AdmittedRequest,
    environment: thinkthen::RequestEnvironment<'_>,
) -> Result<thinkthen::Call<thinkthen::RequestValue>, thinkthen::Error> {
    match engine.execute_request(request, environment)? {
        thinkthen::RequestOutcome::Complete(call) => Ok(call),
        thinkthen::RequestOutcome::Failed { error, .. } => Err(error),
    }
}
/// Materialize one C text without inventing image attachments.
pub(crate) fn text_input(text: String) -> thinkthen::RequestInput {
    thinkthen::RequestInput::Text {
        text,
        images: Vec::new(),
    }
}
pub(crate) fn item(original: thinkthen::RequestOriginal) -> thinkthen::RequestItem {
    thinkthen::RequestItem {
        original: Some(original),
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
        images: Vec::new(),
    }
}

fn decide_definition(question: &LoadedQuestion) -> Result<thinkthen::RequestDefinition, Failure> {
    if matches!(question, LoadedQuestion::Question(asked) if asked.kind() != thinkthen::QuestionKind::Decide)
    {
        return Err(Failure::usage("decide takes a decide question"));
    }
    Ok(question.clone().into())
}
