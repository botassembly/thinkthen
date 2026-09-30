//! The judgment functions: volatile scalars, `thinkthen_usage`, and the
//! `thinkthen_warm` aggregate, each registered volatile and direct-only.

use std::sync::Arc;

use rusqlite::Connection;
use rusqlite::functions::{Context, FunctionFlags};
use rusqlite::types::ValueRef;
use thinkthen::{
    Answer, CallOptions, Details, Engine, For, Judgment, LoadedQuestion, Question, QuestionKind,
    Settings,
};

use crate::question::{call_controls, call_settings, question, question_with_settings, set, text};
use crate::{Failure, ffi, guard, recognize_document, settings, worker};

/// One scalar call's checked question, evidence and portable controls.
type Inputs = Option<(Arc<LoadedQuestion>, String, Settings, Option<String>)>;

fn inputs(context: &Context<'_>, requested: Option<For>) -> Result<Inputs, Failure> {
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
    Ok(engine
        .details_many_with(held, [evidence], options)
        .next()
        .ok_or_else(|| Failure::defect("details returned no record"))??
        .into_parts()
        .1)
}

/// One ordinary detail read; all question kinds keep the same engine path.
fn scalar_details(
    engine: &Engine,
    held: &LoadedQuestion,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Details, Failure> {
    Ok(engine.details_with(held, evidence, options)?.into_value())
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

/// The plain question inside a checked score question.
fn plain(held: &LoadedQuestion) -> Result<&Question, Failure> {
    match held {
        LoadedQuestion::Question(asked) => Ok(asked),
        LoadedQuestion::Banded(_) => Err(Failure::defect("a checked question held a band")),
    }
}

fn plain_decide(
    engine: &Engine,
    held: &LoadedQuestion,
    evidence: &str,
    options: CallOptions<'_>,
) -> Result<Answer, Failure> {
    Ok(engine.decide_with(held, evidence, options)?.into_value())
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
                Ok(engine
                    .details_with(&*held, &evidence, options)?
                    .into_value()
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
                    Ok(engine
                        .score_with(plain(&held)?, &evidence, options)?
                        .into_value())
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
                match engine.annotate_with(&questions, [evidence], options).next() {
                    Some(record) => Ok(record?.value_json()),
                    None => Err(Failure::defect("annotate returned no record")),
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

mod find;
mod plan;

fn register_removed(connection: &Connection, volatile: FunctionFlags) -> rusqlite::Result<()> {
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
            "thinkthen_probability was removed; read probability from thinkthen_decide_many or thinkthen_choose_many",
        ),
    ] {
        connection.create_scalar_function(
            name,
            -1,
            volatile,
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
            move |_| -> rusqlite::Result<String> {
                Err(Failure::plain_usage(sentence.clone()).into())
            },
        )?;
    }
    Ok(())
}

/// Register the scalar functions for direct calls, without deterministic flags.
pub(crate) fn register(connection: &Connection) -> rusqlite::Result<()> {
    let volatile = FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DIRECTONLY;
    connection.create_scalar_function(
        "thinkthen_relations",
        2,
        volatile,
        recognize_document::recognize_document,
    )?;
    connection.create_scalar_function(
        "thinkthen_recognize_document",
        -1,
        volatile,
        |_| -> rusqlite::Result<String> {
            Err(Failure::plain_usage(
                "thinkthen_recognize_document was renamed thinkthen_relations",
            )
            .into())
        },
    )?;
    for arity in [2, 3] {
        connection.create_scalar_function("thinkthen_decide", arity, volatile, decide)?;
        connection.create_scalar_function("thinkthen_choose", arity, volatile, choose)?;
        connection.create_scalar_function("thinkthen_score", arity, volatile, score)?;
        connection.create_scalar_function("thinkthen_tag", arity, volatile, tag)?;
        connection.create_scalar_function("thinkthen_annotate", arity, volatile, annotate)?;
        connection.create_scalar_function("thinkthen_details", arity, volatile, details)?;
        connection.create_scalar_function("thinkthen_try_details", arity, volatile, try_details)?;
        connection.create_scalar_function("thinkthen_find", arity, volatile, find::find)?;
    }
    connection.create_scalar_function("thinkthen_usage", -1, volatile, usage)?;
    for arity in [2, 3] {
        connection.create_scalar_function("thinkthen_plan", arity, volatile, plan::plan)?;
    }
    connection.create_scalar_function("thinkthen_configure", 1, volatile, settings::configure)?;
    register_removed(connection, volatile)?;
    Ok(())
}
