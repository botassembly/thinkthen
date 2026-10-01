//! The global request queue used by `annotate`.

#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a failed fixture setup or a missing field should stop the boundary test"
)]

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use conformance_backend::Rendezvous;

use super::{one_question, set};
use crate::harness::{Canned, Gathering, Listener, finish, spawn};

pub(super) fn yes(model: &str, input: u64, output: u64) -> String {
    format!(
        r#"{{"model":"{model}","answers":{{"q1":{{"type":"noul","noul":0.9}}}},"usage":{{"input_tokens":{input},"output_tokens":{output}}}}}"#
    )
}

/// A reply that answers every question of `body` yes with the named usage.
pub(super) fn all_yes(body: &[u8], model: &str, input: u64, output: u64) -> String {
    let request: serde_json::Value = serde_json::from_slice(body).expect("a request");
    let answers: serde_json::Map<String, serde_json::Value> = request["questions"]
        .as_object()
        .expect("questions")
        .keys()
        .map(|name| (name.clone(), serde_json::json!({"type":"noul","noul":0.9})))
        .collect();
    serde_json::json!({
        "model": model,
        "answers": answers,
        "usage": {"input_tokens": input, "output_tokens": output},
    })
    .to_string()
}

pub(super) fn grouped(name: &str, count: usize) -> PathBuf {
    let questions = (0..count)
        .map(|place| {
            format!(r#""answer_{place}":{{"decide":"question {place}?","on":"/part_{place}"}}"#)
        })
        .collect::<Vec<_>>()
        .join(",");
    set(
        name,
        &format!(r#"{{"version":1,"questions":{{{questions}}}}}"#),
    )
}

pub(super) fn grouped_input(record: usize, count: usize) -> String {
    let fields = (0..count)
        .map(|place| format!(r#""part_{place}":"record {record} group {place}""#))
        .collect::<Vec<_>>()
        .join(",");
    format!(r#"{{"record":{record},{fields}}}"#)
}

pub(super) fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

fn stored(cache: &Path) -> usize {
    crate::support::stored(cache).expect("the store").len()
}

#[test]
fn one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32() {
    let file = grouped("six-groups", 6);
    for jobs in [1_usize, 4, 32] {
        let usage_home = folder(&format!("global-queue-usage-{jobs}"));
        let environment = [
            ("THINKTHEN_API_KEY", "sk-test-value"),
            (
                "XDG_STATE_HOME",
                usage_home.to_str().expect("usage state home"),
            ),
        ];
        let gathering = Gathering::new(jobs.min(6));
        let listener = Listener::answering(move |body| {
            gathering.hold();
            Canned::ok(&all_yes(body, "local-1", 10, 2))
        })
        .expect("a listener");
        let output = spawn(
            &[
                "annotate",
                &file.to_string_lossy(),
                "--url",
                listener.base(),
                "--model",
                "local-1",
                "--jobs",
                &jobs.to_string(),
                "--no-cache",
                "--profile",
                &one_question().to_string_lossy(),
            ],
            &environment,
            grouped_input(1, 6).as_bytes(),
        )
        .expect("document run");
        assert_eq!(output.status.code(), Some(0), "document at {jobs}");
        let requests = listener.requests();
        assert_eq!(requests.len(), 6, "document at {jobs}");
        if jobs == 1 {
            for (place, request) in requests.iter().enumerate() {
                let body = String::from_utf8_lossy(&request.body);
                assert!(body.contains(&format!("record 1 group {place}")), "{body}");
            }
        }
        assert_eq!(listener.peak(), jobs.min(6), "document at {jobs}");

        let listener =
            Listener::answering(|body| Canned::ok(&all_yes(body, "local-1", 10, 2)).after(25))
                .expect("a listener");
        let input = format!("{}\n{}\n", grouped_input(1, 2), grouped_input(2, 2));
        let stream_file = grouped("two-groups", 2);
        let output = spawn(
            &[
                "annotate",
                &stream_file.to_string_lossy(),
                "--url",
                listener.base(),
                "--model",
                "local-1",
                "--jsonl",
                "--batch",
                "1",
                "--jobs",
                &jobs.to_string(),
                "--no-cache",
            ],
            &environment,
            input.as_bytes(),
        )
        .expect("stream run");
        assert_eq!(output.status.code(), Some(0), "stream at {jobs}");
        assert_eq!(output.stdout.lines().count(), 2, "stream at {jobs}");
        let requests = listener.requests();
        // Both groups of one record share its request.
        assert_eq!(requests.len(), 2, "stream at {jobs}");
        if jobs == 1 {
            for (place, request) in requests.iter().enumerate() {
                let body = String::from_utf8_lossy(&request.body);
                let holds = |group| body.contains(&format!("record {} group {group}", place + 1));
                assert!(holds(0) && holds(1), "{body}");
            }
        }
        assert!(listener.peak() <= jobs, "stream at {jobs}");
    }
}

#[test]
fn an_observed_failure_starts_nothing_else_and_keeps_paid_completions() {
    let cache = folder("annotate-global-stop");
    let named = cache.to_string_lossy();
    let failed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    // Records 1 and 2 are both in flight before record 1 fails, however
    // loaded the machine is (ticket 0352).
    let both = Gathering::new(2);
    let listener = Listener::answering({
        let failed = std::sync::Arc::clone(&failed);
        move |body| {
            both.hold();
            let text = String::from_utf8_lossy(body);
            if text.contains("record 1 group 0")
                && !failed.swap(true, std::sync::atomic::Ordering::SeqCst)
            {
                Canned::status(500, "{}")
            } else {
                Canned::ok(&all_yes(body, "local-1", 10, 2)).after(40)
            }
        }
    })
    .expect("a listener");
    let file = grouped("stop-groups", 2);
    let input: String = (1..=3)
        .map(|record| format!("{}\n", grouped_input(record, 2)))
        .collect();
    let args = [
        "annotate",
        &file.to_string_lossy(),
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--jsonl",
        "--batch",
        "1",
        "--jobs",
        "2",
        "--max-retries",
        "0",
        "--cache",
        &named,
    ];
    let first = spawn(
        &args,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
    .expect("failed run");
    // Record 3 waits for a free worker and never starts; record 2 was paid.
    assert_eq!(first.status.code(), Some(4));
    assert_eq!(listener.requests().len(), 2);
    assert_eq!(stored(&cache), 2);

    let second = spawn(
        &args,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
    .expect("resumed run");
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(second.stdout.lines().count(), 3);
    assert_eq!(listener.requests().len(), 2);
    assert_eq!(stored(&cache), 6);
}

#[test]
fn a_cache_can_mix_replayed_and_live_groups_with_checked_usage() {
    let cache = folder("annotate-mixed-cache");
    let named = cache.to_string_lossy();
    let one = grouped("one-cached-group", 1);
    let two = grouped("two-mixed-groups", 2);
    let answer = yes("local-1", 10, 2);
    let listener = Listener::answering(move |_| Canned::ok(&answer)).expect("a listener");
    let common = [
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &named,
    ];
    let first = spawn(
        &[&["annotate", &one.to_string_lossy()][..], &common].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        grouped_input(1, 2).as_bytes(),
    )
    .expect("cache seed");
    assert_eq!(first.status.code(), Some(0));
    let second = spawn(
        &[
            &["annotate", &two.to_string_lossy(), "--details"][..],
            &common,
        ]
        .concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        grouped_input(1, 2).as_bytes(),
    )
    .expect("mixed run");
    assert_eq!(second.status.code(), Some(0));
    let row = String::from_utf8_lossy(&second.stdout);
    assert!(
        row.contains(r#""usage":{"input_tokens":20,"output_tokens":4}"#),
        "{row}"
    );
    assert_eq!(row.matches(r#""cached":"#).count(), 1);
    assert!(!row.contains(r#""replayed":"#), "{row}");
    assert!(row.contains(r#""requests_sent":1,"cached":false"#), "{row}");
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn annotate_equal_records_share_one_request() {
    let cache = folder("annotate-equal-cache-digest");
    let named = cache.to_string_lossy();
    let file = grouped("one-shared-group", 1);
    let answer = yes("local-1", 10, 2);
    // The reply waits a full second, a wide margin for the second record to
    // be read from the already written pipe and join the request (ticket 0352).
    let listener =
        Listener::answering(move |_| Canned::ok(&answer).after(1_000)).expect("a listener");
    let input = format!("{}\n{}\n", grouped_input(1, 1), grouped_input(1, 1));
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--jsonl",
            "--batch",
            "1",
            "--details",
            "--cache",
            &named,
            "--jobs",
            "32",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
    .expect("annotate run");

    assert_eq!(output.status.code(), Some(0));
    // The second record joins the first one's request in flight, so neither
    // row comes from the store.
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    let key = crate::support::keys(listener.url(), &sent[0].body);
    let rows = String::from_utf8_lossy(&output.stdout);
    assert_eq!(rows.lines().count(), 2);
    assert_eq!(rows.matches(r#""cached":false"#).count(), 2);
    let requests = format!(r#""requests":["{}"]"#, key[0]);
    assert_eq!(rows.matches(&requests).count(), 2, "{rows}");
}

#[test]
fn a_closed_output_pipe_stops_annotate_quietly_and_bounds_read_ahead() {
    let listener =
        Listener::answering(|body| Canned::ok(&all_yes(body, "local-1", 1, 1)).after(20))
            .expect("a listener");
    let file = grouped("broken-pipe", 2);
    let state_home = folder("broken-pipe-usage");
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("XDG_STATE_HOME", state_home)
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .args([
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--jsonl",
            "--batch",
            "1",
            "--jobs",
            "4",
            "--no-cache",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary runs");
    let mut input = child.stdin.take().expect("stdin");
    let records: String = (1..=20)
        .map(|record| format!("{}\n", grouped_input(record, 2)))
        .collect();
    input
        .write_all(records.as_bytes())
        .expect("records written");
    input.flush().expect("records flushed");
    let mut output = BufReader::new(child.stdout.take().expect("stdout"));
    let mut first = String::new();
    output.read_line(&mut first).expect("first row");
    assert!(first.contains(r#""record":1"#), "{first}");
    drop(output);

    let deadline = Instant::now() + Duration::from_secs(30);
    let status = loop {
        if let Some(status) = child.try_wait().expect("status") {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("annotate did not stop after the output pipe closed");
        }
        thread::yield_now();
    };
    drop(input);
    assert_eq!(status.code(), Some(0));
    let sent = listener.requests().len();
    // One request a record. The window holds (4 + 1) records, and it slides
    // by the rows written before the closed pipe shows.
    assert!(sent <= 8, "broken pipe sent {sent} requests");
}

#[test]
fn a_backend_failure_after_the_output_pipe_closes_stays_quiet() {
    let cache = folder("annotate-closed-pipe-failure");
    let named = cache.to_string_lossy();
    // Records 2 and 3 answer only once the test has closed the output pipe,
    // so the order holds on a loaded machine (ticket 0352).
    let closed = Arc::new(Rendezvous::new(3));
    let held = Arc::clone(&closed);
    let listener = Listener::answering(move |body| {
        let body = String::from_utf8_lossy(body);
        if body.contains("record 1") {
            Canned::ok(&yes("local-1", 1, 1))
        } else if body.contains("record 2") {
            Canned::ok(&yes("local-1", 1, 1)).after_release(Arc::clone(&held))
        } else {
            Canned::status(500, "{}").after_release(Arc::clone(&held))
        }
    })
    .expect("a listener");
    let file = grouped("closed-pipe-failure", 1);
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("XDG_CACHE_HOME", cache.join(".platform"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .args([
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--jsonl",
            "--batch",
            "1",
            "--jobs",
            "3",
            "--max-retries",
            "0",
            "--cache",
            &named,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary runs");
    let input: String = (1..=3)
        .map(|record| format!("{}\n", grouped_input(record, 1)))
        .collect();
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("records written");
    let mut output = BufReader::new(child.stdout.take().expect("stdout"));
    let mut first = String::new();
    output.read_line(&mut first).expect("first row");
    assert!(first.contains(r#""record":1"#), "{first}");
    let guard = Instant::now() + Duration::from_secs(30);
    while listener.count() < 3 {
        assert!(Instant::now() < guard, "all three records were sent");
        thread::yield_now();
    }
    drop(output);
    assert!(closed.wait(), "the held replies were released");

    let result = finish(child, "annotate").expect("the command ends");
    assert_eq!(result.status.code(), Some(0));
    assert!(result.stderr.is_empty());
    assert_eq!(listener.requests().len(), 3);
    assert_eq!(stored(&cache), 2);
}

#[test]
fn usage_overflow_fails_safely() {
    // One question a request; the two requests' totals overflow together.
    let sent = std::sync::atomic::AtomicBool::new(false);
    let listener = Listener::answering(move |body| {
        let input = if sent.swap(true, std::sync::atomic::Ordering::SeqCst) {
            1
        } else {
            u64::MAX
        };
        Canned::ok(&all_yes(body, "local-1", input, 1))
    })
    .expect("a listener");
    let file = grouped("usage-overflow", 2);
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--details",
            "--profile",
            &one_question().to_string_lossy(),
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        grouped_input(1, 2).as_bytes(),
    )
    .expect("the run");
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        concat!(
            "thinkthen: the backend reported token counts whose total is too large\n",
            "thinkthen: usage counters could not be updated; ",
            "check the usage folder permissions and free space\n",
        )
    );
    assert!(output.stdout.is_empty());
}
