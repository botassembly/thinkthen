//! The judgment functions: volatile scalars, `thinkthen_usage`, and the
//! removed spellings. Configuration and controls remain direct-only.

use std::sync::Arc;

use crate::catalog::Catalog;
use rusqlite::functions::{Context, FunctionFlags};
use rusqlite::types::ValueRef;
use thinkthen::{
    Answer, CallOptions, Details, Engine, For, Judgment, LoadedQuestion, QuestionKind, Settings,
};

use crate::question::{call_controls, call_settings, question, question_with_settings, set, text};
use crate::{Failure, Registration, ffi, guard, recognize_document, settings, worker};

/// One scalar call's checked question, evidence and portable controls.
type Inputs = Option<(Arc<LoadedQuestion>, String, Settings, Option<String>)>;

fn inputs(context: &Context<'_>, requested: Option<For>) -> Result<Inputs, Failure> {
    if (0..2).any(|slot| matches!(context.get_raw(slot), ValueRef::Null)) {
        return Ok(None);
    }
    let settings = if context.len() > 2 {
        call_settings(context.get_raw(2))?
    } else {
        Settings::default()
    };
    let Some(argument) = text(context.get_raw(0), "the question")? else {
        return Ok(None);
    };
    let Some(evidence) = text(context.get_raw(1), "the text")? else {
        return Ok(None);
    };
    let verb = if let Some(verb) = requested {
        verb
    } else {
        match question(&argument)?.kind() {
            QuestionKind::Decide => For::Decide,
            QuestionKind::Choose => For::Choose,
            QuestionKind::Score => For::Score,
            QuestionKind::Tag => For::Tag,
            _ => return Err(Failure::usage("details takes a judgment question")),
        }
    };
    let shared = settings.context().map(str::to_owned);
    Ok(Some((
        question_with_settings(&argument, &settings, verb)?,
        evidence,
        settings,
        shared,
    )))
}

/// The shared many-record details path for a contextual singleton.
fn contextual(
    engine: &Engine,
    held: &LoadedQuestion,
    evidence: String,
    options: CallOptions<'_>,
) -> Result<Details, Failure> {
    crate::request::details(
        engine,
        held,
        vec![crate::request::text(evidence)],
        options,
        true,
    )?
    .into_iter()
    .next()
    .ok_or_else(|| Failure::defect("details returned no record"))
}

/// One ordinary detail read; all question kinds keep the same engine path.
fn scalar_details(
    engine: &Engine,
    held: &LoadedQuestion,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Details, Failure> {
    crate::request::details(
        engine,
        held,
        vec![crate::request::text(evidence.to_owned())],
        options,
        false,
    )?
    .into_iter()
    .next()
    .ok_or_else(|| Failure::defect("details returned no record"))
}

/// Refuse a question `name` does not take, before any send.
fn only(held: &LoadedQuestion, name: &str, kind: QuestionKind) -> Result<(), Failure> {
    if matches!(held, LoadedQuestion::Banded(_)) {
        return Err(Failure::usage(format!(
            "{name} does not take a banded question; use thinkthen_decide or thinkthen_details"
        )));
    }
    if held.kind() == kind {
        return Ok(());
    }
    Err(Failure::usage(
        format!(
            "{name} takes a {kind:?} question, not a {:?} question",
            held.kind()
        )
        .to_lowercase(),
    ))
}

fn plain_decide(
    engine: &Engine,
    held: &LoadedQuestion,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Answer, Failure> {
    match scalar_details(engine, held, evidence, options)?.value() {
        Judgment::Decision(answer) => Ok(*answer),
        _ => Err(Failure::defect("a decide answer held no decision")),
    }
}

fn decide(context: &Context<'_>) -> rusqlite::Result<Option<i64>> {
    Ok(guard("thinkthen_decide", || {
        let Some((held, evidence, settings, shared)) = inputs(context, Some(For::Decide))? else {
            return Ok(None);
        };
        let answer =
            worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
                if let Some(shared) = shared {
                    match contextual(engine, &held, evidence, options.context(&shared))?.value() {
                        Judgment::Decision(answer) => Ok(*answer),
                        _ => Err(Failure::defect("a decide answer held no decision")),
                    }
                } else {
                    plain_decide(engine, &held, &evidence, options)
                }
            })?;
        Ok(match answer {
            Answer::Yes => Some(1),
            Answer::No => Some(0),
            Answer::Unsure => None,
        })
    })?)
}

/// `choose` and `tag` read the details value, which costs no added send (G5).
fn judged(
    context: &Context<'_>,
    name: &'static str,
    kind: QuestionKind,
) -> Result<Option<Judgment>, Failure> {
    let verb = match kind {
        QuestionKind::Choose => For::Choose,
        QuestionKind::Tag => For::Tag,
        _ => return Err(Failure::defect("judged received another kind")),
    };
    let Some((held, evidence, settings, shared)) = inputs(context, Some(verb))? else {
        return Ok(None);
    };
    only(&held, name, kind)?;
    let details =
        worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
            if let Some(shared) = shared {
                Ok(
                    contextual(engine, &held, evidence, options.context(&shared))?
                        .value()
                        .clone(),
                )
            } else {
                Ok(scalar_details(engine, &held, &evidence, options)?
                    .value()
                    .clone())
            }
        })?;
    Ok(Some(details))
}

fn choose(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_choose", || {
        match judged(context, "thinkthen_choose", QuestionKind::Choose)? {
            None => Ok(None),
            Some(Judgment::Choice(label)) => Ok(label),
            Some(_) => Err(Failure::defect("a choose answer held no choice")),
        }
    })?)
}

fn tag(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_tag", || {
        match judged(context, "thinkthen_tag", QuestionKind::Tag)? {
            None => Ok(None),
            Some(Judgment::Tags(labels)) => serde_json::to_string(&labels)
                .map(Some)
                .map_err(|error| Failure::defect(error.to_string())),
            Some(_) => Err(Failure::defect("a tag answer held no labels")),
        }
    })?)
}

fn score(context: &Context<'_>) -> rusqlite::Result<Option<f64>> {
    Ok(guard("thinkthen_score", || {
        let Some((held, evidence, settings, shared)) = inputs(context, Some(For::Score))? else {
            return Ok(None);
        };
        only(&held, "thinkthen_score", QuestionKind::Score)?;
        let position =
            worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
                if let Some(shared) = shared {
                    match contextual(engine, &held, evidence, options.context(&shared))?.value() {
                        Judgment::Score(value) => Ok(*value),
                        _ => Err(Failure::defect("a score answer held no score")),
                    }
                } else {
                    match scalar_details(engine, &held, &evidence, options)?.value() {
                        Judgment::Score(value) => Ok(*value),
                        _ => Err(Failure::defect("a score answer held no score")),
                    }
                }
            })?;
        Ok(Some(position))
    })?)
}

fn details(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_details", || {
        let Some((held, evidence, settings, shared)) = inputs(context, None)? else {
            return Ok(None);
        };
        let details =
            worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
                if let Some(shared) = shared {
                    Ok(
                        contextual(engine, &held, evidence, options.context(&shared))?
                            .to_scalar_json(),
                    )
                } else {
                    Ok(scalar_details(engine, &held, &evidence, options)?.to_json())
                }
            })?;
        Ok(Some(details))
    })?)
}

fn try_details(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    if matches!(context.get_raw(0), ValueRef::Null) || matches!(context.get_raw(1), ValueRef::Null)
    {
        return Ok(None);
    }
    let result = guard("thinkthen_try_details", || {
        let Some((held, evidence, settings, shared)) = inputs(context, None)? else {
            return Ok(None);
        };
        let details =
            worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
                if let Some(shared) = shared {
                    Ok(
                        contextual(engine, &held, evidence, options.context(&shared))?
                            .to_scalar_json(),
                    )
                } else {
                    Ok(scalar_details(engine, &held, &evidence, options)?.to_json())
                }
            })?;
        let details: serde_json::Value =
            serde_json::from_str(&details).map_err(|_| Failure::defect("a result is not JSON"))?;
        Ok(Some(
            serde_json::json!({"status":"answered","details":details}).to_string(),
        ))
    });
    match result {
        Ok(value) => Ok(value),
        Err(failure) => match failure.value() {
            Some(value) => Ok(Some(value.to_string())),
            None => Err(failure.into()),
        },
    }
}

fn annotate(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_annotate", || {
        if (0..2).any(|slot| matches!(context.get_raw(slot), ValueRef::Null)) {
            return Ok(None);
        }
        let settings = if context.len() > 2 {
            call_controls(context.get_raw(2))?
        } else {
            Settings::default()
        };
        let Some(argument) = text(context.get_raw(0), "the question set")? else {
            return Ok(None);
        };
        let Some(evidence) = text(context.get_raw(1), "the text")? else {
            return Ok(None);
        };
        let questions = set(&argument)?;
        let shared = settings.context().map(str::to_owned);
        let record =
            worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
                let options = if let Some(shared) = &shared {
                    options.context(shared)
                } else {
                    options
                };
                let call = crate::request::run(
                    engine,
                    thinkthen::RequestFunction::Annotate,
                    (*questions).clone().into(),
                    vec![crate::request::document(evidence)?],
                    options,
                )?;
                match call.value() {
                    thinkthen::RequestValue::Annotations(rows) => rows
                        .first()
                        .ok_or_else(|| Failure::defect("annotate returned no record"))?
                        .result()
                        .value_json()
                        .map_err(Failure::from),
                    _ => Err(crate::request::wrong_result().into()),
                }
            })?;
        Ok(Some(record))
    })?)
}

/// `thinkthen_usage()`: the engine's totals. It builds no engine, so before
/// the first call every total reads 0 and settings still apply.
fn usage(context: &Context<'_>) -> rusqlite::Result<String> {
    Ok(guard("thinkthen_usage", || {
        if !context.is_empty() {
            let reset = matches!(context.get_raw(0), ValueRef::Text(b"reset"));
            return Err(if reset {
                Failure::plain_usage(
                    "the reset spelling is removed; the counters are cumulative, so take two snapshots and subtract them",
                )
            } else {
                Failure::usage(
                    "thinkthen_usage takes no arguments; the counters are cumulative, so subtract two snapshots",
                )
            });
        }
        let totals = settings::built().map(thinkthen::Engine::usage);
        let read = |field: fn(&thinkthen::Counters) -> u64| totals.as_ref().map_or(0, field);
        Ok(format!(
            r#"{{"requests_sent":{},"cache_answers":{},"input_tokens":{},"output_tokens":{}}}"#,
            read(thinkthen::Counters::requests_sent),
            read(thinkthen::Counters::cache_answers),
            read(thinkthen::Counters::input_tokens),
            read(thinkthen::Counters::output_tokens),
        ))
    })?)
}

fn usage_status(context: &Context<'_>) -> rusqlite::Result<String> {
    Ok(guard("thinkthen_usage_status", || {
        if !context.is_empty() {
            return Err(Failure::usage("thinkthen_usage_status takes no arguments"));
        }
        let state = thinkthen::UsagePersistence::aggregate(
            settings::built().map(thinkthen::Engine::usage_persistence),
        );
        let value = match state.advice() {
            Some(advice) => serde_json::json!({"state": state, "advice": advice}),
            None => serde_json::json!({"state": state}),
        };
        Ok(value.to_string())
    })?)
}

mod find;
mod plan;

fn register_removed(connection: &Catalog<'_>, volatile: FunctionFlags) -> rusqlite::Result<()> {
    for name in [
        "thinkthen_decide",
        "thinkthen_choose",
        "thinkthen_score",
        "thinkthen_tag",
        "thinkthen_details",
        "thinkthen_try_details",
        "thinkthen_annotate",
        "thinkthen_find",
    ] {
        let sentence = if name == "thinkthen_find" {
            "find's none and deadline moved into the settings object"
        } else {
            "the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'"
        };
        connection.create_scalar_function(
            name,
            4,
            volatile,
            "Refuse a removed call form and explain the supported replacement.",
            move |_| -> rusqlite::Result<String> { Err(Failure::plain_usage(sentence).into()) },
        )?;
    }
    for (name, sentence) in [
        (
            "thinkthen_warm",
            "thinkthen_warm was removed; pack records with thinkthen_decide_many",
        ),
        (
            "thinkthen_probability",
            "thinkthen_probability was removed; order records with thinkthen_rank",
        ),
    ] {
        connection.create_scalar_function(
            name,
            -1,
            volatile,
            "Refuse a removed call form and explain the supported replacement.",
            move |_| -> rusqlite::Result<String> { Err(Failure::plain_usage(sentence).into()) },
        )?;
    }
    for name in [
        "throttle",
        "batch",
        "max_requests",
        "max_request_bytes",
        "max_requests_total",
        "cache",
        "model",
        "timeout",
        "max_retries",
        "profile",
        "record",
        "replay",
    ] {
        let full = format!("thinkthen_{name}");
        let sentence = format!("thinkthen_{name} was replaced by thinkthen_configure");
        connection.create_scalar_function(
            full.as_str(),
            -1,
            volatile,
            "Refuse a removed setting function and direct callers to thinkthen_configure.",
            move |_| -> rusqlite::Result<String> {
                Err(Failure::plain_usage(sentence.clone()).into())
            },
        )?;
    }
    Ok(())
}

fn register_judgments(connection: &Catalog<'_>, judgment: FunctionFlags) -> rusqlite::Result<()> {
    for arity in [2, 3] {
        connection.create_scalar_function(
            "thinkthen_decide",
            arity,
            judgment,
            "Answer a yes or no question about the evidence.",
            decide,
        )?;
        connection.create_scalar_function(
            "thinkthen_choose",
            arity,
            judgment,
            "Choose the option that best fits the evidence.",
            choose,
        )?;
        connection.create_scalar_function(
            "thinkthen_score",
            arity,
            judgment,
            "Score the evidence on named levels.",
            score,
        )?;
        connection.create_scalar_function(
            "thinkthen_tag",
            arity,
            judgment,
            "Return every label that applies to the evidence as JSON.",
            tag,
        )?;
        connection.create_scalar_function(
            "thinkthen_annotate",
            arity,
            judgment,
            "Apply a saved question set to records and return annotations as JSON.",
            annotate,
        )?;
        connection.create_scalar_function(
            "thinkthen_details",
            arity,
            judgment,
            "Return a complete judgment envelope as JSON.",
            details,
        )?;
        connection.create_scalar_function(
            "thinkthen_try_details",
            arity,
            judgment,
            "Return a judgment envelope or a recoverable failure envelope as JSON.",
            try_details,
        )?;
        connection.create_scalar_function(
            "thinkthen_find",
            arity,
            judgment,
            "Return the unit of text that best answers the question.",
            find::find,
        )?;
    }
    Ok(())
}

/// Register judgments under the initial connection policy, without harmless flags.
pub(crate) fn register(connection: &Catalog<'_>, mode: Registration) -> rusqlite::Result<()> {
    crate::complete::register(connection)?;
    let volatile = FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DIRECTONLY;
    let judgment = match mode {
        Registration::DirectOnly => volatile,
        Registration::Trusted => FunctionFlags::SQLITE_UTF8,
    };
    connection.create_scalar_function(
        "thinkthen_relations",
        2,
        judgment,
        "Recognize entities and relations in one document as JSON.",
        recognize_document::recognize_document,
    )?;
    connection.create_scalar_function(
        "thinkthen_recognize_document",
        -1,
        volatile,
        "Refuse the removed name and explain its replacement by thinkthen_relations.",
        |_| -> rusqlite::Result<String> {
            Err(Failure::plain_usage(
                "thinkthen_recognize_document was renamed thinkthen_relations",
            )
            .into())
        },
    )?;
    register_judgments(connection, judgment)?;
    connection.create_scalar_function(
        "thinkthen_usage",
        -1,
        volatile,
        "Read or configure count-only usage reporting.",
        usage,
    )?;
    connection.create_scalar_function(
        "thinkthen_usage_status",
        -1,
        volatile,
        "Return count-only usage reporting status as JSON.",
        usage_status,
    )?;
    for arity in [2, 3] {
        connection.create_scalar_function(
            "thinkthen_plan",
            arity,
            judgment,
            "Prepare a judgment plan as JSON without sending requests.",
            plan::plan,
        )?;
    }
    connection.create_scalar_function(
        "thinkthen_configure",
        1,
        volatile,
        "Configure process settings from a JSON object.",
        settings::configure,
    )?;
    crate::images::register(connection, volatile)?;
    register_removed(connection, volatile)?;
    Ok(())
}
