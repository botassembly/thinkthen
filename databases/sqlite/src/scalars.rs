//! The judgment functions: six scalars, `thinkthen_usage`, and the
//! `thinkthen_warm` aggregate, each registered volatile and direct-only.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use rusqlite::Connection;
use rusqlite::functions::{Aggregate, Context, FunctionFlags};
use rusqlite::types::ValueRef;
use thinkthen::{Answer, CallOptions, Judgment, LoadedQuestion, Question, QuestionKind};

use crate::question::{question, set, shown, text};
use crate::{Failure, ffi, guard, settings, worker};

/// Warm sends each question's texts in chunks of this many rows.
const CHUNK: usize = 256;

/// The third argument, milliseconds under ADR 0041: an INTEGER as given, or
/// a finite, whole REAL inside `i64`. Anything else is `usage`.
fn deadline(context: &Context<'_>) -> Result<Option<i64>, Failure> {
    if context.len() < 3 {
        return Ok(None);
    }
    let value = context.get_raw(2);
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

/// One scalar call's question, evidence, and deadline, or `None` for a NULL.
type Inputs = Option<(Arc<LoadedQuestion>, String, Option<i64>)>;

fn inputs(context: &Context<'_>) -> Result<Inputs, Failure> {
    let deadline = deadline(context)?;
    let Some(argument) = text(context.get_raw(0), "the question")? else {
        return Ok(None);
    };
    let Some(evidence) = text(context.get_raw(1), "the text")? else {
        return Ok(None);
    };
    Ok(Some((question(&argument)?, evidence, deadline)))
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

fn decide(context: &Context<'_>) -> rusqlite::Result<Option<i64>> {
    Ok(guard("thinkthen_decide", || {
        let Some((held, evidence, deadline)) = inputs(context)? else {
            return Ok(None);
        };
        let answer = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
            Ok(match &*held {
                LoadedQuestion::Question(asked) => engine.decide_with(asked, &evidence, options)?,
                LoadedQuestion::Banded(asked) => engine.decide_with(asked, &evidence, options)?,
            })
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
    let Some((held, evidence, deadline)) = inputs(context)? else {
        return Ok(None);
    };
    only(&held, name, kind)?;
    let details = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
        Ok(engine.details_with(plain(&held)?, &evidence, options)?)
    })?;
    Ok(Some(details.value().clone()))
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
        let Some((held, evidence, deadline)) = inputs(context)? else {
            return Ok(None);
        };
        only(&held, "thinkthen_score", QuestionKind::Score)?;
        let position = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
            Ok(engine.score_with(plain(&held)?, &evidence, options)?)
        })?;
        Ok(Some(position))
    })?)
}

fn details(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_details", || {
        let Some((held, evidence, deadline)) = inputs(context)? else {
            return Ok(None);
        };
        let details = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
            Ok(match &*held {
                LoadedQuestion::Question(asked) => {
                    engine.details_with(asked, &evidence, options)?
                }
                LoadedQuestion::Banded(asked) => engine.details_with(asked, &evidence, options)?,
            })
        })?;
        Ok(Some(details.to_json()))
    })?)
}

fn try_details(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    if matches!(context.get_raw(0), ValueRef::Null) || matches!(context.get_raw(1), ValueRef::Null)
    {
        return Ok(None);
    }
    let result = guard("thinkthen_try_details", || {
        let Some((held, evidence, deadline)) = inputs(context)? else {
            return Ok(None);
        };
        let details = worker::run(ffi::handle_of(context), deadline, move |engine, options| {
            Ok(match &*held {
                LoadedQuestion::Question(asked) => {
                    engine.details_with(asked, &evidence, options)?
                }
                LoadedQuestion::Banded(asked) => engine.details_with(asked, &evidence, options)?,
            })
        })?;
        let details: serde_json::Value = serde_json::from_str(&details.to_json())
            .map_err(|_| Failure::defect("a result is not JSON"))?;
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

/// One question's texts in a warm pass.
#[derive(Debug)]
struct Group {
    question: Arc<LoadedQuestion>,
    seen: HashSet<String>,
    pending: Vec<String>,
}

/// The warm aggregate's state: groups in first-seen order, found by the
/// question argument's text in constant time (R5-18).
#[derive(Debug, Default)]
pub(crate) struct WarmState {
    groups: Vec<Group>,
    index: HashMap<String, usize>,
    judged: i64,
}

/// A full chunk of one question's texts, ready to send.
type Flush = (Arc<LoadedQuestion>, Vec<String>);

impl WarmState {
    /// Add one row. An equal pair in the same pass is asked once. A group
    /// that reaches a chunk comes back to be sent.
    fn add(
        &mut self,
        argument: &str,
        parse: impl FnOnce() -> Result<Arc<LoadedQuestion>, Failure>,
        evidence: String,
    ) -> Result<Option<Flush>, Failure> {
        let at = match self.index.get(argument) {
            Some(at) => *at,
            None => {
                self.groups.push(Group {
                    question: parse()?,
                    seen: HashSet::new(),
                    pending: Vec::new(),
                });
                self.index
                    .insert(argument.to_owned(), self.groups.len() - 1);
                self.groups.len() - 1
            }
        };
        let group = self
            .groups
            .get_mut(at)
            .ok_or_else(|| Failure::defect("a warm group is missing"))?;
        if group.seen.insert(evidence.clone()) {
            group.pending.push(evidence);
        }
        Ok((group.pending.len() >= CHUNK).then(|| {
            (
                Arc::clone(&group.question),
                std::mem::take(&mut group.pending),
            )
        }))
    }
}

/// Judge one chunk on a worker, one send per text the cache lacks. Under a
/// process request total the chunk is cut to as many rows as requests
/// remain, and once that part is judged the call refuses (decision 17).
fn flush(context: &Context<'_>, (held, mut texts): Flush) -> Result<i64, Failure> {
    // The cut counts rows. A cached row costs no request, so the refusal
    // names the rows judged, not the requests spent.
    let cut = settings::remaining()?.filter(|left| *left < texts.len());
    if let Some(left) = cut {
        texts.truncate(left);
    }
    let count =
        i64::try_from(texts.len()).map_err(|_| Failure::defect("a warm chunk is too long"))?;
    if count == 0 {
        return Ok(0);
    }
    worker::run(ffi::handle_of(context), None, move |engine, options| {
        // The band stays out of the request, so decide reads what this fills.
        match &*held {
            LoadedQuestion::Question(asked) => engine
                .decide_many_with(asked, texts, options)
                .try_for_each(|row| row.map(drop))?,
            LoadedQuestion::Banded(asked) => engine
                .decide_many_with(asked, texts, options)
                .try_for_each(|row| row.map(drop))?,
        }
        Ok(count)
    })?;
    match cut {
        Some(left) => Err(Failure::usage(format!(
            "this warm pass stopped at the remaining total of {left} requests (thinkthen_max_requests_total)"
        ))),
        None => Ok(count),
    }
}

/// Warm's question: decide only, cut or banded (ticket 0129), any other kind
/// named to `thinkthen_decide`.
fn warm_question(argument: &str) -> Result<Arc<LoadedQuestion>, Failure> {
    let held = question(argument)?;
    match &*held {
        LoadedQuestion::Question(asked) if asked.kind() != QuestionKind::Decide => {
            Err(Failure::usage(
                "thinkthen_warm takes a decide question; ask others with thinkthen_decide",
            ))
        }
        _ => Ok(held),
    }
}

/// `thinkthen_warm(question, text)`: judge every distinct pair once, filling
/// the engine's cache, and answer how many pairs it judged.
#[derive(Debug)]
struct Warm;

impl Aggregate<WarmState, i64> for Warm {
    fn init(&self, _: &mut Context<'_>) -> rusqlite::Result<WarmState> {
        Ok(WarmState::default())
    }

    fn step(&self, context: &mut Context<'_>, state: &mut WarmState) -> rusqlite::Result<()> {
        Ok(guard("thinkthen_warm", || {
            let Some(argument) = text(context.get_raw(0), "the question")? else {
                return Ok(());
            };
            let Some(evidence) = text(context.get_raw(1), "the text")? else {
                return Ok(());
            };
            if let Some(chunk) = state.add(&argument, || warm_question(&argument), evidence)? {
                state.judged += flush(context, chunk)?;
            }
            Ok(())
        })?)
    }

    fn finalize(
        &self,
        context: &mut Context<'_>,
        state: Option<WarmState>,
    ) -> rusqlite::Result<i64> {
        Ok(guard("thinkthen_warm", || {
            let Some(mut state) = state else {
                return Ok(0);
            };
            for group in std::mem::take(&mut state.groups) {
                state.judged += flush(context, (group.question, group.pending))?;
            }
            Ok(state.judged)
        })?)
    }
}

/// Register the thirteen names in nineteen arities, none deterministic.
pub(crate) fn register(connection: &Connection) -> rusqlite::Result<()> {
    let volatile = FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DIRECTONLY;
    for arity in [2, 3] {
        connection.create_scalar_function("thinkthen_decide", arity, volatile, decide)?;
        connection.create_scalar_function("thinkthen_choose", arity, volatile, choose)?;
        connection.create_scalar_function("thinkthen_score", arity, volatile, score)?;
        connection.create_scalar_function("thinkthen_tag", arity, volatile, tag)?;
        connection.create_scalar_function("thinkthen_annotate", arity, volatile, annotate)?;
        connection.create_scalar_function("thinkthen_details", arity, volatile, details)?;
        connection.create_scalar_function("thinkthen_try_details", arity, volatile, try_details)?;
    }
    connection.create_scalar_function("thinkthen_usage", -1, volatile, usage)?;
    connection.create_aggregate_function("thinkthen_warm", 2, volatile, Warm)?;
    connection.create_scalar_function("thinkthen_throttle", 1, volatile, settings::throttle)?;
    connection.create_scalar_function(
        "thinkthen_max_requests",
        1,
        volatile,
        settings::max_requests,
    )?;
    connection.create_scalar_function(
        "thinkthen_max_requests_total",
        1,
        volatile,
        settings::max_requests_total,
    )?;
    connection.create_scalar_function("thinkthen_cache", 1, volatile, settings::cache)?;
    connection.create_scalar_function(
        "thinkthen_cache_bytes",
        1,
        volatile,
        settings::cache_bytes,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use thinkthen::{LoadedQuestion, Question};

    use super::WarmState;

    /// R5-18: 200,000 rows under 200,000 distinct questions group in under 1 s.
    #[test]
    fn warm_finds_each_group_in_constant_time() {
        let held = Question::decide("Is this a complaint?")
            .map(|builder| Arc::new(LoadedQuestion::Question(builder.cut())));
        let held = held.map_err(|error| error.to_string()).unwrap();
        let mut state = WarmState::default();
        let started = Instant::now();
        for at in 0..200_000 {
            let flushed = state.add(
                &format!("q{at}"),
                || Ok(Arc::clone(&held)),
                "text".to_owned(),
            );
            assert!(matches!(flushed, Ok(None)));
        }
        assert_eq!(state.groups.len(), 200_000);
        assert!(
            started.elapsed() < Duration::from_secs(1),
            "{:?}",
            started.elapsed()
        );
    }
}
