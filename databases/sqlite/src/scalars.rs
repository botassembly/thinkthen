//! The judgment functions: six scalars, `thinkthen_usage`, and the
//! `thinkthen_warm` aggregate, each registered volatile and direct-only.

use std::sync::Arc;

use rusqlite::Connection;
use rusqlite::functions::{Context, FunctionFlags};
use rusqlite::types::ValueRef;
use thinkthen::{
    Answer, CallOptions, Details, Engine, Judgment, LoadedQuestion, Question, QuestionKind,
};

use crate::question::{question, set, shown, text};
use crate::{Failure, ffi, guard, recognize_document, settings, worker};

/// One deadline slot, milliseconds under ADR 0041: an INTEGER as given, or
/// a finite, whole REAL inside `i64`. Anything else is `usage`.
fn deadline_at(context: &Context<'_>, slot: usize) -> Result<Option<i64>, Failure> {
    if context.len() <= slot {
        return Ok(None);
    }
    let value = context.get_raw(slot);
    let millis = match value {
        ValueRef::Integer(whole) => Some(whole),
        ValueRef::Real(real)
            if real.is_finite()
                && real.fract() == 0.0
                && (i64::MIN as f64..-(i64::MIN as f64)).contains(&real) =>
        {
            Some(real as i64)
        }
        _ => None,
    };
    let millis = millis.ok_or_else(|| {
        Failure::usage(format!(
            "a deadline of {} is not a whole number of milliseconds",
            shown(value)
        ))
    })?;
    CallOptions::new().deadline_millis(millis)?;
    Ok(Some(millis))
}

/// The established judgment and warm deadline is the third argument.
fn deadline(context: &Context<'_>) -> Result<Option<i64>, Failure> {
    deadline_at(context, 2)
}

/// One scalar call's question, evidence, and deadline, or `None` for a NULL.
type Inputs = Option<(Arc<LoadedQuestion>, String, Option<i64>, Option<String>)>;

/// A final literal context, with SQL NULL meaning the old no-context call.
fn shared(context: &Context<'_>) -> Result<Option<String>, Failure> {
    if context.len() < 4 {
        return Ok(None);
    }
    let value = text(context.get_raw(3), "the context")?;
    if value.as_ref().is_some_and(|value| value.trim().is_empty()) {
        return Err(Failure::usage("context is text, not white space"));
    }
    Ok(value)
}

fn inputs(context: &Context<'_>) -> Result<Inputs, Failure> {
    let deadline = deadline(context)?;
    let Some(argument) = text(context.get_raw(0), "the question")? else {
        return Ok(None);
    };
    let Some(evidence) = text(context.get_raw(1), "the text")? else {
        return Ok(None);
    };
    let shared = shared(context)?;
    Ok(Some((question(&argument)?, evidence, deadline, shared)))
}

/// The shared many-record details path for a contextual singleton.
fn contextual(
    engine: &Engine,
    held: &LoadedQuestion,
    evidence: String,
    options: CallOptions<'_>,
) -> Result<Details, Failure> {
    let row = match held {
        LoadedQuestion::Question(asked) => {
            engine.details_many_with(asked, [evidence], options).next()
        }
        LoadedQuestion::Banded(asked) => {
            engine.details_many_with(asked, [evidence], options).next()
        }
    };
    Ok(row
        .ok_or_else(|| Failure::defect("details returned no record"))??
        .into_parts()
        .1)
}

/// Refuse a question `name` does not take, before any send.
fn only(held: &LoadedQuestion, name: &str, kind: QuestionKind) -> Result<(), Failure> {
    match held {
        LoadedQuestion::Banded(_) => Err(Failure::usage(format!(
            "{name} does not take a banded question; use thinkthen_decide or thinkthen_details"
        ))),
        LoadedQuestion::Question(asked) if asked.kind() == kind => Ok(()),
        LoadedQuestion::Question(asked) => Err(Failure::usage(
            format!(
                "{name} takes a {kind:?} question, not a {:?} question",
                asked.kind()
            )
            .to_lowercase(),
        )),
    }
}

/// The plain question inside a checked one.
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
    Ok(match held {
        LoadedQuestion::Question(asked) => {
            engine.decide_with(asked, evidence, options)?.into_value()
        }
        LoadedQuestion::Banded(asked) => engine.decide_with(asked, evidence, options)?.into_value(),
    })
}

fn decide(context: &Context<'_>) -> rusqlite::Result<Option<i64>> {
    Ok(guard("thinkthen_decide", || {
        let Some((held, evidence, deadline, shared)) = inputs(context)? else {
            return Ok(None);
        };
        let answer = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
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
    let Some((held, evidence, deadline, shared)) = inputs(context)? else {
        return Ok(None);
    };
    only(&held, name, kind)?;
    let details = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
        if let Some(shared) = shared {
            Ok(
                contextual(engine, &held, evidence, options.context(&shared))?
                    .value()
                    .clone(),
            )
        } else {
            Ok(engine
                .details_with(plain(&held)?, &evidence, options)?
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
        let Some((held, evidence, deadline, shared)) = inputs(context)? else {
            return Ok(None);
        };
        only(&held, "thinkthen_score", QuestionKind::Score)?;
        let position = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
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
        let Some((held, evidence, deadline, shared)) = inputs(context)? else {
            return Ok(None);
        };
        let details = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
            if let Some(shared) = shared {
                Ok(contextual(engine, &held, evidence, options.context(&shared))?.to_scalar_json())
            } else {
                Ok(match &*held {
                    LoadedQuestion::Question(asked) => engine
                        .details_with(asked, &evidence, options)?
                        .into_value()
                        .to_json(),
                    LoadedQuestion::Banded(asked) => engine
                        .details_with(asked, &evidence, options)?
                        .into_value()
                        .to_json(),
                })
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
        let Some((held, evidence, deadline, shared)) = inputs(context)? else {
            return Ok(None);
        };
        let details = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
            if let Some(shared) = shared {
                Ok(contextual(engine, &held, evidence, options.context(&shared))?.to_scalar_json())
            } else {
                Ok(match &*held {
                    LoadedQuestion::Question(asked) => engine
                        .details_with(asked, &evidence, options)?
                        .into_value()
                        .to_json(),
                    LoadedQuestion::Banded(asked) => engine
                        .details_with(asked, &evidence, options)?
                        .into_value()
                        .to_json(),
                })
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
        let deadline = deadline(context)?;
        let Some(argument) = text(context.get_raw(0), "the question set")? else {
            return Ok(None);
        };
        let Some(evidence) = text(context.get_raw(1), "the text")? else {
            return Ok(None);
        };
        let questions = set(&argument)?;
        let record =
            worker::run(
                ffi::handle_of(context),
                deadline,
                move |engine, options| match engine
                    .annotate_with(&questions, [evidence], options)
                    .next()
                {
                    Some(record) => Ok(record?.value_json()),
                    None => Err(Failure::defect("annotate returned no record")),
                },
            )?;
        Ok(Some(record))
    })?)
}

/// `thinkthen_usage()`: the engine's totals. It builds no engine, so before
/// the first call every total reads 0 and settings still apply.
fn usage(context: &Context<'_>) -> rusqlite::Result<String> {
    Ok(guard("thinkthen_usage", || {
        if !context.is_empty() {
            let reset = matches!(context.get_raw(0), ValueRef::Text(b"reset"));
            return Err(Failure::usage(if reset {
                "the reset spelling is removed; the counters are cumulative, so take two snapshots and subtract them"
            } else {
                "thinkthen_usage takes no arguments; the counters are cumulative, so subtract two snapshots"
            }));
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

mod warm;
use warm::Warm;
mod find;

/// Register the scalar functions for direct calls, without deterministic flags.
pub(crate) fn register(connection: &Connection) -> rusqlite::Result<()> {
    let volatile = FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DIRECTONLY;
    connection.create_scalar_function(
        "thinkthen_recognize_document",
        2,
        volatile,
        recognize_document::recognize_document,
    )?;
    for arity in [2, 3, 4] {
        connection.create_scalar_function("thinkthen_decide", arity, volatile, decide)?;
        connection.create_scalar_function("thinkthen_choose", arity, volatile, choose)?;
        connection.create_scalar_function("thinkthen_score", arity, volatile, score)?;
        connection.create_scalar_function("thinkthen_tag", arity, volatile, tag)?;
        if arity < 4 {
            connection.create_scalar_function("thinkthen_annotate", arity, volatile, annotate)?;
        }
        connection.create_scalar_function("thinkthen_details", arity, volatile, details)?;
        connection.create_scalar_function("thinkthen_try_details", arity, volatile, try_details)?;
    }
    connection.create_scalar_function("thinkthen_usage", -1, volatile, usage)?;
    for arity in [2, 3, 4] {
        connection.create_aggregate_function("thinkthen_warm", arity, volatile, Warm)?;
        connection.create_scalar_function("thinkthen_find", arity, volatile, find::find)?;
    }
    connection.create_scalar_function("thinkthen_throttle", 1, volatile, settings::throttle)?;
    connection.create_scalar_function("thinkthen_batch", 1, volatile, settings::batch)?;
    connection.create_scalar_function(
        "thinkthen_max_requests",
        1,
        volatile,
        settings::max_requests,
    )?;
    connection.create_scalar_function(
        "thinkthen_max_request_bytes",
        1,
        volatile,
        settings::max_request_bytes,
    )?;
    connection.create_scalar_function(
        "thinkthen_max_requests_total",
        1,
        volatile,
        settings::max_requests_total,
    )?;
    connection.create_scalar_function("thinkthen_cache", 1, volatile, settings::cache)?;
    connection.create_scalar_function("thinkthen_model", 1, volatile, settings::model)?;
    connection.create_scalar_function("thinkthen_timeout", 1, volatile, settings::timeout)?;
    connection.create_scalar_function(
        "thinkthen_max_retries",
        1,
        volatile,
        settings::max_retries,
    )?;
    connection.create_scalar_function("thinkthen_profile", 1, volatile, settings::profile)?;
    connection.create_scalar_function("thinkthen_record", 1, volatile, settings::record)?;
    connection.create_scalar_function("thinkthen_replay", 1, volatile, settings::replay)?;
    Ok(())
}
