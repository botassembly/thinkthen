//! `thinkthen_warm(question, evidence)`: an aggregate that judges every
//! distinct pair once and fills the engine's disk cache, so a later decide
//! over the same pairs reads the cache and sends nothing (decision 4).

use std::collections::HashSet;

use pgrx::pg_sys::FunctionCallInfo;
use pgrx::prelude::*;
use pgrx::{Aggregate, AggregateName};
use thinkthen::{LoadedQuestion, QuestionKind};

use crate::call::{self, OrRaise as _, Refusal};

/// The most rows one warm call takes, bounding its memory and spend.
const WARM_ROW_CAP: u64 = 20_000;

/// The most escaped question and evidence bytes one state holds. Each step
/// copies the text state across the datum boundary, so this bounds the copy work.
const WARM_STATE_CAP: usize = 2 * 1024 * 1024;

fn over_cap() -> ! {
    call::raise(Refusal::usage(
        "thinkthen_warm takes at most 20,000 rows per call",
    ));
}

/// The aggregate. Its text state is the row count, then each row's escaped
/// question and evidence joined by unit separators; a step never parses it.
#[derive(AggregateName)]
#[aggregate_name = "thinkthen_warm"]
struct Warm;

#[derive(AggregateName)]
#[aggregate_name = "thinkthen_warm"]
struct WarmContext;

type WarmGroup = (String, Option<String>, Vec<String>);

fn warm_escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\x1e', "\\e")
        .replace('\x1f', "\\f")
}

fn warm_unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut held = text.chars();
    while let Some(here) = held.next() {
        let escaped = if here == '\\' { held.next() } else { None };
        out.push(match escaped {
            Some('e') => '\x1e',
            Some('f') => '\x1f',
            Some(other) => other,
            None => here,
        });
    }
    out
}

fn warm_count(state: &str) -> u64 {
    state
        .split_once('\x1e')
        .map_or(0, |(head, _)| head.parse().unwrap_or(0))
}

fn warm_tail(state: &str) -> &str {
    state.split_once('\x1e').map_or("", |(_, tail)| tail)
}

/// One row appended: the count grows, the tail carries over, nothing parses.
fn warm_step(state: &str, question: &str, evidence: &str, context: Option<&str>) -> String {
    let count = warm_count(state);
    let mut next = format!("{}\x1e{}", count + 1, warm_tail(state));
    if count > 0 {
        next.push('\x1e');
    }
    next.push_str(&warm_escape(question));
    next.push('\x1f');
    next.push_str(&warm_escape(evidence));
    next.push('\x1f');
    match context {
        Some(text) => {
            next.push('1');
            next.push_str(&warm_escape(text));
        }
        None => next.push('0'),
    }
    next
}

/// Two states merged: the counts add, the tails join.
fn warm_merge(one: &str, two: &str) -> String {
    let mut merged = format!(
        "{}\x1e{}",
        warm_count(one) + warm_count(two),
        warm_tail(one)
    );
    if !warm_tail(two).is_empty() {
        if !warm_tail(one).is_empty() {
            merged.push('\x1e');
        }
        merged.push_str(warm_tail(two));
    }
    merged
}

/// Every row back, unescaped, grouped by question text in first-seen order,
/// each group's evidence distinct in first-seen order.
fn warm_groups(state: &str) -> Vec<WarmGroup> {
    let mut groups: Vec<WarmGroup> = Vec::new();
    let mut seen: HashSet<(String, Option<String>, String)> = HashSet::new();
    for row in warm_tail(state).split('\x1e').filter(|row| !row.is_empty()) {
        let (question, rest) = row.split_once('\x1f').unwrap_or((row, ""));
        let (evidence, encoded_context) = rest.split_once('\x1f').unwrap_or((rest, "0"));
        let (question, evidence) = (warm_unescape(question), warm_unescape(evidence));
        let context = encoded_context.strip_prefix('1').map(warm_unescape);
        if !seen.insert((question.clone(), context.clone(), evidence.clone())) {
            continue;
        }
        match groups
            .iter_mut()
            .find(|(held, shared, _)| *held == question && *shared == context)
        {
            Some((_, _, list)) => list.push(evidence),
            None => groups.push((question, context, vec![evidence])),
        }
    }
    groups
}

fn state(
    current: String,
    question: Option<String>,
    evidence: Option<String>,
    context: Option<String>,
) -> String {
    let (Some(question), Some(evidence)) = (question, evidence) else {
        return current;
    };
    if context
        .as_deref()
        .is_some_and(|text| text.trim().is_empty())
    {
        call::raise(Refusal::usage("context must not be blank"));
    }
    if warm_count(&current) >= WARM_ROW_CAP {
        over_cap();
    }
    let next = warm_step(&current, &question, &evidence, context.as_deref());
    if next.len() > WARM_STATE_CAP {
        call::raise(Refusal::usage(format!(
            "thinkthen_warm holds at most {WARM_STATE_CAP} bytes of questions and evidence per call"
        )));
    }
    next
}

fn combine(one: String, two: String) -> String {
    if warm_count(&one) + warm_count(&two) > WARM_ROW_CAP {
        over_cap();
    }
    let merged = warm_merge(&one, &two);
    if merged.len() > WARM_STATE_CAP {
        call::raise(Refusal::usage(format!(
            "thinkthen_warm holds at most {WARM_STATE_CAP} bytes of questions and evidence per call"
        )));
    }
    merged
}

fn finalize(current: String) -> i64 {
    let mut groups = Vec::new();
    for (question, context, evidence) in warm_groups(&current) {
        // Resolve every file and question on the backend thread before spawn.
        let question = crate::question(Some(&question), "", None);
        if matches!(&question, LoadedQuestion::Question(held) if held.kind() != QuestionKind::Decide)
        {
            call::raise(Refusal::usage(
                "thinkthen_warm takes a decide question; ask others with thinkthen_decide",
            ));
        }
        groups.push((question, context, evidence));
    }
    if groups.is_empty() {
        return 0;
    }
    let call = call::read();
    let most = call.max_requests();
    let result: Result<i64, Refusal> = call::run(call, move |engine, options| {
        let mut judged = 0usize;
        for (question, context, evidence) in groups {
            if let Err(refusal) = call::record_limit(evidence.len(), most) {
                return Ok(Err(refusal));
            }
            let options = context
                .as_deref()
                .map_or(options, |text| options.context(text));
            let rows = match &question {
                LoadedQuestion::Question(held) => engine
                    .decide_many_with(held, evidence.iter().map(String::as_str), options)
                    .collect::<Result<Vec<_>, _>>(),
                LoadedQuestion::Banded(held) => engine
                    .decide_many_with(held, evidence.iter().map(String::as_str), options)
                    .collect::<Result<Vec<_>, _>>(),
            }?;
            judged += rows.len();
        }
        Ok(Ok(i64::try_from(judged).unwrap_or(i64::MAX)))
    });
    result.or_raise()
}

#[pg_aggregate]
impl Aggregate<Warm> for Warm {
    type State = String;
    type Args = (Option<String>, Option<String>);
    type Finalize = i64;
    const INITIAL_CONDITION: Option<&'static str> = Some("0");

    fn state(current: String, args: Self::Args, _fcinfo: FunctionCallInfo) -> String {
        state(current, args.0, args.1, None)
    }

    fn combine(one: String, two: String, _fcinfo: FunctionCallInfo) -> String {
        combine(one, two)
    }

    fn finalize(current: String, _direct: Self::OrderedSetArgs, _fcinfo: FunctionCallInfo) -> i64 {
        finalize(current)
    }
}

#[pg_aggregate]
impl Aggregate<WarmContext> for WarmContext {
    type State = String;
    type Args = (Option<String>, Option<String>, Option<String>);
    type Finalize = i64;
    const INITIAL_CONDITION: Option<&'static str> = Some("0");

    fn state(current: String, args: Self::Args, _fcinfo: FunctionCallInfo) -> String {
        state(current, args.0, args.1, args.2)
    }

    fn combine(one: String, two: String, _fcinfo: FunctionCallInfo) -> String {
        combine(one, two)
    }

    fn finalize(current: String, _direct: Self::OrderedSetArgs, _fcinfo: FunctionCallInfo) -> i64 {
        finalize(current)
    }
}

#[cfg(test)]
mod tests {
    use super::{warm_count, warm_groups, warm_merge, warm_step};

    /// The warm state round-trips separators and slashes, counts from its
    /// head, merges by adding counts, and groups distinct pairs.
    #[test]
    fn the_warm_state_round_trips_and_groups() {
        let mut state = String::from("0");
        for (question, evidence) in [
            ("q1", "a"),
            ("odd \x1e \x1f \\", "b \x1f"),
            ("q1", "a"),
            ("q1", "c"),
        ] {
            state = warm_step(&state, question, evidence, None);
        }
        assert_eq!(warm_count(&state), 4);
        let other = warm_step("0", "q2", "d", Some("shared"));
        let merged = warm_merge(&state, &other);
        assert_eq!(warm_count(&merged), 5);
        assert_eq!(
            warm_groups(&merged),
            [
                ("q1".to_owned(), None, vec!["a".to_owned(), "c".to_owned()]),
                (
                    "odd \x1e \x1f \\".to_owned(),
                    None,
                    vec!["b \x1f".to_owned()]
                ),
                (
                    "q2".to_owned(),
                    Some("shared".to_owned()),
                    vec!["d".to_owned()]
                ),
            ]
        );
        assert_eq!(warm_merge("0", &other), other);
    }
}
