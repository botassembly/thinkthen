//! `recognize` and `relate` on the question cache of ADR 0111, ruling 4:
//! each backend question is stored alone, so a rerun sends nothing and a
//! grown input sends only its new questions. `find` has the same proof in
//! `find.rs`.

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup or a missing field should stop the boundary test"
)]

use std::process::Output;

use serde_json::{Value, json};

use crate::batching::folder;
use crate::harness::{Listener, spawn};
use crate::recognize::{automatic, questions};
use crate::relate::{WRONG, answered, scripted};

const KEY: (&str, &str) = ("THINKTHEN_API_KEY", "secret-value");

/// Run `verb` at the listener with `options`, caching in `cache`.
fn cached(verb: &str, listener: &Listener, cache: &str, options: &[&str], input: &[u8]) -> Output {
    let mut arguments = vec![
        verb,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        cache,
    ];
    arguments.extend_from_slice(options);
    spawn(&arguments, &[KEY], input).expect("the command runs")
}

/// Run `verb` at the listener with `options` and no cache.
fn uncached(verb: &str, listener: &Listener, options: &[&str], input: &[u8]) -> Output {
    let mut arguments = vec![verb, "--url", listener.base(), "--model", "local-1"];
    arguments.extend_from_slice(options);
    arguments.push("--no-cache");
    spawn(&arguments, &[KEY], input).expect("the command runs")
}

fn succeeded(output: &Output) -> String {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout.clone()).expect("output text")
}

/// The question count of each request since the last drain, largest first,
/// since requests may arrive in any order.
fn asked(listener: &Listener) -> Vec<usize> {
    let mut counts: Vec<usize> = listener
        .requests()
        .iter()
        .map(|request| questions(&request.body).len())
        .collect();
    counts.sort_unstable_by(|left, right| right.cmp(left));
    counts
}

/// Entities of `people` people and `organizations` organizations.
fn entities(people: usize, organizations: usize) -> Vec<u8> {
    let named = |kind: &'static str, count: usize| {
        (0..count).map(move |at| json!({"name": format!("{kind} {at}"), "kind": kind}))
    };
    let rows: Vec<Value> = named("person", people)
        .chain(named("organization", organizations))
        .collect();
    Value::from(rows).to_string().into_bytes()
}

const WORKS_FOR: &str = "works_for=person:organization";

/// 420 relation questions go in a request of 400 and one of 20; a rerun
/// sends nothing and prints the same relations.
#[test]
fn relate_sends_each_question_once_and_a_rerun_sends_nothing() {
    let listener = Listener::answering(answered).expect("a loopback listener");
    let cache = folder("relate-rerun");
    let input = entities(21, 20);
    let first = succeeded(&cached("relate", &listener, &cache, &[WORKS_FOR], &input));
    assert_eq!(asked(&listener), [400, 20]);
    let stored = crate::support::stored(std::path::Path::new(&cache)).expect("the store");
    assert_eq!(stored.len(), 420);

    let second = succeeded(&cached("relate", &listener, &cache, &[WORKS_FOR], &input));
    assert_eq!(listener.count(), 2, "the rerun sent no request");
    assert_eq!(second, first);
}

/// A new rule sends only its own questions.
#[test]
fn relate_with_a_new_rule_sends_only_that_rules_questions() {
    const OWNS: &str = "owns=organization:person";
    let alone = Listener::answering(answered).expect("a loopback listener");
    let input = entities(1, 3);
    succeeded(&uncached("relate", &alone, &[OWNS], &input));
    let owns = asked(&alone);
    assert_eq!(owns, [3]);

    let listener = Listener::answering(answered).expect("a loopback listener");
    let cache = folder("relate-grown");
    succeeded(&cached("relate", &listener, &cache, &[WORKS_FOR], &input));
    assert_eq!(asked(&listener), [3]);
    let grown = succeeded(&cached(
        "relate",
        &listener,
        &cache,
        &[WORKS_FOR, OWNS],
        &input,
    ));
    assert_eq!(asked(&listener), owns);
    let whole = succeeded(&uncached("relate", &alone, &[WORKS_FOR, OWNS], &input));
    assert_eq!(grown, whole);
}

/// A reply with one bad answer fails the run but stores the good one, and
/// the rerun sends only the failed question.
#[test]
fn a_partial_relate_reply_keeps_its_good_answer() {
    const GOOD: &str = r#"{"type":"noul","noul":0.9}"#;
    let partial = scripted(&[GOOD, WRONG]);
    let cache = folder("relate-partial");
    let input = entities(1, 2);
    let failed = cached("relate", &partial, &cache, &[WORKS_FOR], &input);
    assert_eq!(
        failed.status.code(),
        Some(6),
        "{}",
        String::from_utf8_lossy(&failed.stderr)
    );
    assert_eq!(asked(&partial), [2]);
    let stored = crate::support::stored(std::path::Path::new(&cache)).expect("the store");
    assert_eq!(stored.len(), 1, "only the good answer");

    // The key holds the address, so the rerun asks the same listener. Its
    // one question comes first in the request and gets the good answer.
    succeeded(&cached("relate", &partial, &cache, &[WORKS_FOR], &input));
    assert_eq!(asked(&partial), [1]);
}

const RECOGNIZE: [&str; 5] = ["person", "organization", "--relation", WORKS_FOR, "--lines"];

/// Every recognize step replays on a rerun, which sends nothing; a new
/// line sends only the questions that line asks alone.
#[test]
fn recognize_reruns_send_nothing_and_a_new_line_sends_only_its_questions() {
    let (first, added) = ("Ada met Acme.\n", "Otto joined Corp.\n");
    let alone = Listener::answering(automatic).expect("a loopback listener");
    succeeded(&uncached("recognize", &alone, &RECOGNIZE, added.as_bytes()));
    let line = alone.questions();

    let listener = Listener::answering(automatic).expect("a loopback listener");
    let cache = folder("recognize-rerun");
    let once = succeeded(&cached(
        "recognize",
        &listener,
        &cache,
        &RECOGNIZE,
        first.as_bytes(),
    ));
    let sent = listener.count();
    assert!(sent >= 3, "{sent} requests for three steps");
    let again = succeeded(&cached(
        "recognize",
        &listener,
        &cache,
        &RECOGNIZE,
        first.as_bytes(),
    ));
    assert_eq!(listener.count(), sent, "the rerun sent no request");
    assert_eq!(again, once);

    let before = listener.questions();
    let both = format!("{first}{added}");
    let grown = succeeded(&cached(
        "recognize",
        &listener,
        &cache,
        &RECOGNIZE,
        both.as_bytes(),
    ));
    assert_eq!(listener.questions() - before, line);
    let whole = succeeded(&uncached("recognize", &alone, &RECOGNIZE, both.as_bytes()));
    assert_eq!(grown, whole);
}
