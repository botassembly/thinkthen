//! The question cache of ADR 0111 through the compiled command: each
//! question is stored alone, so a run asks only the questions no earlier run
//! answered.

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup or a missing field should stop the boundary test"
)]

use std::process::Output;

use serde_json::{Value, json};

use crate::batching::{KEY, QUESTION, answering, folder, lines, places};
use crate::harness::{Canned, Listener, spawn, start};

/// `decide QUESTION --lines --facts` over `input` with a cache folder.
fn decide(listener: &Listener, cache: &str, input: &str) -> Output {
    spawn(
        &[
            "decide",
            QUESTION,
            "--lines",
            "--facts",
            "--cache",
            cache,
            "--url",
            listener.base(),
            "--model",
            "jev-1.13.0",
        ],
        &[KEY],
        input.as_bytes(),
    )
    .expect("the command runs")
}

/// The run's `--facts` line, the last line of standard error.
fn facts(output: &Output) -> Value {
    let said = String::from_utf8_lossy(&output.stderr);
    serde_json::from_str(said.lines().last().expect("a facts line")).expect("facts JSON")
}

/// The lines each request asked about, in the order the requests arrived.
fn asked(listener: &Listener) -> Vec<Vec<usize>> {
    listener
        .requests()
        .iter()
        .map(|request| {
            let mut at: Vec<usize> = places(&request.body).iter().map(|&(_, at)| at).collect();
            at.sort_unstable();
            at
        })
        .collect()
}

/// ADR 0111's headline: after 100 records, a run of 120 that includes them
/// sends one request holding exactly the 20 new questions, and reports 100
/// cache answers.
#[test]
fn a_longer_run_sends_only_the_new_records_questions() {
    let listener = Listener::answering(answering).expect("a loopback listener");
    let cache = folder("headline");
    let first = decide(&listener, &cache, &lines(1..=100));
    assert_eq!(
        first.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert_eq!(listener.questions(), 100);
    assert_eq!(facts(&first)["cache_answers"], 0);
    let _first = listener.requests();

    let second = decide(&listener, &cache, &lines(1..=120));
    assert_eq!(
        second.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert_eq!(asked(&listener), [(101..=120).collect::<Vec<_>>()]);
    assert_eq!(
        listener.questions(),
        120,
        "20 questions sent after the first 100"
    );
    let facts = facts(&second);
    assert_eq!(facts["cache_answers"], 100);
    assert_eq!(facts["requests_sent"], 1);
    assert_eq!(facts["records"], 120);
}

/// A partial reply stores its good answers, and the rerun sends only the
/// question that failed.
#[test]
fn a_partial_reply_keeps_its_good_answers_and_a_rerun_asks_only_the_failed_one() {
    // The first run's reply leaves out record 3's answer; later replies are whole.
    let first = std::sync::atomic::AtomicBool::new(true);
    let listener = Listener::answering(move |body| {
        let skip = first.swap(false, std::sync::atomic::Ordering::SeqCst);
        let answers: serde_json::Map<String, Value> = places(body)
            .into_iter()
            .filter(|&(_, at)| !(skip && at == 3))
            .map(|(name, _)| (name, json!({"type": "noul", "noul": 0.9})))
            .collect();
        Canned::ok(&json!({"model": "jev-1.13.0", "answers": answers}).to_string())
    })
    .expect("a loopback listener");
    let cache = folder("partial");
    let partial = decide(&listener, &cache, &lines(1..=5));
    assert_eq!(partial.status.code(), Some(4));
    assert_eq!(String::from_utf8_lossy(&partial.stdout).lines().count(), 2);
    assert_eq!(asked(&listener), [vec![1, 2, 3, 4, 5]]);
    let stored = crate::support::stored(std::path::Path::new(&cache)).expect("the store");
    assert_eq!(stored.len(), 4, "every answer but record 3's");

    let rerun = decide(&listener, &cache, &lines(1..=5));
    assert_eq!(
        rerun.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&rerun.stderr)
    );
    assert_eq!(asked(&listener), [vec![3]]);
    assert_eq!(facts(&rerun)["cache_answers"], 4);
    assert_eq!(String::from_utf8_lossy(&rerun.stdout).lines().count(), 5);
}

/// Adding a label to a `tag` run sends only that label's questions.
#[test]
fn a_new_tag_label_sends_only_its_own_questions() {
    let listener = Listener::answering(|body| {
        let answers: serde_json::Map<String, Value> = places(body)
            .into_iter()
            .map(|(name, _)| (name, json!({"type": "noul", "noul": 0.9})))
            .collect();
        Canned::ok(&json!({"model": "jev-1.13.0", "answers": answers}).to_string())
    })
    .expect("a loopback listener");
    let cache = folder("tag-label");
    let tag = |labels: &[&str]| {
        spawn(
            &[
                &["tag", "Which topics?"][..],
                labels,
                &[
                    "--lines",
                    "--cache",
                    &cache,
                    "--url",
                    listener.base(),
                    "--model",
                    "jev-1.13.0",
                ],
            ]
            .concat(),
            &[KEY],
            lines(1..=3).as_bytes(),
        )
        .expect("the command runs")
    };
    let first = tag(&["billing", "urgent"]);
    assert_eq!(
        first.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert_eq!(listener.questions(), 6);
    let _first = listener.requests();
    let second = tag(&["billing", "urgent", "late"]);
    assert_eq!(
        second.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    let body = String::from_utf8_lossy(&sent[0].body);
    assert_eq!(body.matches("the label \\\"late\\\"").count(), 3, "{body}");
    assert_eq!(listener.questions(), 9);
}

/// Two processes write one store at once, and both succeed. A third run
/// finds every answer either one stored.
#[test]
fn two_processes_write_one_store_at_once() {
    let listener = Listener::answering(|body| answering(body).after(50)).expect("a listener");
    let cache = folder("two-writers");
    let arguments = [
        "decide",
        QUESTION,
        "--lines",
        "--batch",
        "1",
        "--cache",
        cache.as_str(),
        "--url",
        listener.base(),
        "--model",
        "jev-1.13.0",
    ];
    let (odd, even): (Vec<usize>, Vec<usize>) = (1..=40).partition(|at| at % 2 == 1);
    let text = |places: &[usize]| lines(places.iter().copied());
    let (odd, even) = (text(&odd), text(&even));
    let children = [&odd, &even]
        .map(|input| start(&arguments, &[KEY], input.as_bytes()).expect("a child starts"));
    for child in children {
        let output = crate::harness::finish(child, "a writer").expect("a writer ends");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert_eq!(listener.questions(), 40);
    let stored = crate::support::stored(std::path::Path::new(&cache)).expect("the store");
    assert_eq!(stored.len(), 40);
    let again = decide(&listener, &cache, &lines(1..=40));
    assert_eq!(again.status.code(), Some(0));
    assert_eq!(facts(&again)["cache_answers"], 40);
    assert_eq!(listener.questions(), 40, "the third run sends nothing");
}
