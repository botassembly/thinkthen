//! `thinkthen_warm(question, evidence)`: an aggregate that judges every
//! distinct pair once and fills the engine's disk cache, so a later decide
//! over the same pairs reads the cache and sends nothing (decision 4).

use std::collections::HashMap;

use pgrx::pg_sys::FunctionCallInfo;
use pgrx::prelude::*;
use pgrx::{Aggregate, AggregateName};

use crate::call::{self, Refusal};

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

fn warm_escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\x1e', "\\e")
        .replace('\x1f', "\\f")
}

fn warm_unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut held = text.chars();
    while let Some(here) = held.next() {
        match (here, here == '\\') {
            (_, false) => out.push(here),
            (_, true) => match held.next() {
                Some('e') => out.push('\x1e'),
                Some('f') => out.push('\x1f'),
                Some(other) => out.push(other),
                None => out.push('\\'),
            },
        }
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
fn warm_step(state: &str, question: &str, evidence: &str) -> String {
    let count = warm_count(state);
    let mut next = format!("{}\x1e{}", count + 1, warm_tail(state));
    if count > 0 {
        next.push('\x1e');
    }
    next.push_str(&warm_escape(question));
    next.push('\x1f');
    next.push_str(&warm_escape(evidence));
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
fn warm_groups(state: &str) -> Vec<(String, Vec<String>)> {
    let mut groups: Vec<(String, Vec<String>)> = Vec::new();
    let mut seen: HashMap<(String, String), ()> = HashMap::new();
    for row in warm_tail(state).split('\x1e').filter(|row| !row.is_empty()) {
        let (question, evidence) = row.split_once('\x1f').unwrap_or((row, ""));
        let (question, evidence) = (warm_unescape(question), warm_unescape(evidence));
        if seen
            .insert((question.clone(), evidence.clone()), ())
            .is_some()
        {
            continue;
        }
        match groups.iter_mut().find(|(held, _)| *held == question) {
            Some((_, list)) => list.push(evidence),
            None => groups.push((question, vec![evidence])),
        }
    }
    groups
}

#[pg_aggregate]
impl Aggregate<Warm> for Warm {
    type State = String;
    type Args = (Option<String>, Option<String>);
    type Finalize = i64;
    const INITIAL_CONDITION: Option<&'static str> = Some("0");

    fn state(current: String, args: Self::Args, _fcinfo: FunctionCallInfo) -> String {
        let (Some(question), Some(evidence)) = args else {
            return current;
        };
        if warm_count(&current) >= WARM_ROW_CAP {
            over_cap();
        }
        let next = warm_step(&current, &question, &evidence);
        if next.len() > WARM_STATE_CAP {
            call::raise(Refusal::usage(format!(
                "thinkthen_warm holds at most {WARM_STATE_CAP} bytes of questions and evidence per call"
            )));
        }
        next
    }

    fn combine(one: String, two: String, _fcinfo: FunctionCallInfo) -> String {
        if warm_count(&one) + warm_count(&two) > WARM_ROW_CAP {
            over_cap();
        }
        warm_merge(&one, &two)
    }

    fn finalize(current: String, _direct: Self::OrderedSetArgs, _fcinfo: FunctionCallInfo) -> i64 {
        let mut judged: usize = 0;
        for (question, evidence) in warm_groups(&current) {
            // The question resolves on the backend thread, so a bad file
            // names itself before any worker starts.
            let question = crate::question(Some(&question), "", None);
            judged += evidence.len();
            crate::decide_distinct(question, evidence);
        }
        i64::try_from(judged).unwrap_or(i64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            state = warm_step(&state, question, evidence);
        }
        assert_eq!(warm_count(&state), 4);
        let other = warm_step("0", "q2", "d");
        let merged = warm_merge(&state, &other);
        assert_eq!(warm_count(&merged), 5);
        assert_eq!(
            warm_groups(&merged),
            [
                ("q1".to_owned(), vec!["a".to_owned(), "c".to_owned()]),
                ("odd \x1e \x1f \\".to_owned(), vec!["b \x1f".to_owned()]),
                ("q2".to_owned(), vec!["d".to_owned()]),
            ]
        );
        assert_eq!(warm_merge("0", &other), other);
    }
}
