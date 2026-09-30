//! The compiled binary never sends a request again after a transport failure.
//!
//! Each failing listener serves three copies of its failure, so the old rule's
//! two default retries would each reach it. The listener's count is the proof.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use crate::harness::{Canned, Listener, spawn_one as spawn};
use crate::support::{keys, stored};

/// The key every case sends, which no output may carry.
const KEY: &str = "sk-resend-secret";

/// The evidence every single-document case sends, which no output may carry.
const EVIDENCE: &str = "private resend evidence";

/// The response a backend gives when it answers the one question asked.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// The diagnostic a close, a reset, or a cut-short body earns.
const CLOSED: &str = "thinkthen: the backend closed the connection before a reply and may have received the request; it was not sent again\n";

/// The diagnostic a stall past the attempt timeout earns.
const TIMED_OUT: &str = "thinkthen: the backend timed out; increase --timeout or try again\n";

/// The most reply bytes one request may earn: 1 MiB, plus 8 for each request byte.
pub(crate) fn limit(request: &[u8]) -> usize {
    1_048_576 + 8 * request.len()
}

/// A backend reply padded with spaces to exactly `size` bytes, which still decodes.
pub(crate) fn padded(reply: &str, size: usize) -> Canned {
    let (head, tail) = reply.split_at(reply.len() - 1);
    Canned::ok(&format!("{head}{}{tail}", " ".repeat(size - reply.len())))
}

/// The sentence a reply past its limit earns.
pub(crate) fn past(limit: usize) -> String {
    format!(
        "the backend's reply passed this request's limit of {limit} bytes, so the answer was not kept; the request was not sent again"
    )
}

/// Run `decide` against the listener with the key and any extra environment.
fn decide(
    listener: &Listener,
    arguments: &[&str],
    environment: &[(&str, &str)],
    input: &str,
) -> io::Result<Output> {
    let asked = ["decide", "asks for a refund", "--url", listener.base()];
    let environment = [&[("THINKTHEN_API_KEY", KEY)][..], environment].concat();
    spawn(
        &[&asked[..], arguments].concat(),
        &environment,
        input.as_bytes(),
    )
}

/// A listener that serves three copies of one failure, one per connection.
fn failing(failure: impl Fn() -> Canned) -> io::Result<Listener> {
    Listener::serving(vec![failure(), failure(), failure()])
}

/// A fresh folder under the test target directory.
fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

/// Check one failed send: one request, exit 4, this sentence, and no secret.
fn failed_once(listener: &Listener, output: &Output, said: &str) {
    assert_eq!(
        listener.requests().len(),
        1,
        "the requests the listener saw"
    );
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(String::from_utf8_lossy(&output.stderr), said);
    let shown = format!(
        "{}{}{output:?}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    for secret in [KEY, EVIDENCE] {
        assert!(!shown.contains(secret), "{secret} leaked");
    }
}

#[test]
fn a_close_after_the_whole_request_is_sent_once() {
    let listener = failing(Canned::close_without_reply).expect("a loopback listener");

    let output = decide(&listener, &[], &[], EVIDENCE).expect("the compiled binary runs");

    failed_once(&listener, &output, CLOSED);
}

#[test]
fn a_reset_after_the_whole_request_is_sent_once() {
    let listener = failing(Canned::reset).expect("a loopback listener");

    let output = decide(&listener, &[], &[], EVIDENCE).expect("the compiled binary runs");

    failed_once(&listener, &output, CLOSED);
}

#[test]
fn a_stall_past_the_timeout_is_sent_once() {
    // The reply waits a minute, so only the 1 s timeout ends the call (ticket 0352).
    let listener =
        Listener::answering(|_| Canned::ok(ANSWERED).after(60_000)).expect("a loopback listener");
    let started = Instant::now();

    let output =
        decide(&listener, &["--timeout", "1"], &[], EVIDENCE).expect("the compiled binary runs");

    failed_once(&listener, &output, TIMED_OUT);
    assert!(started.elapsed() < Duration::from_secs(30));
}

#[test]
fn a_stalled_two_record_batch_names_timeout_advice() {
    let listener =
        Listener::answering(|_| Canned::ok(ANSWERED).after(60_000)).expect("a loopback listener");
    let output = decide(
        &listener,
        &["--jsonl", "--batch", "2", "--timeout", "1", "--no-cache"],
        &[],
        "{\"id\":1}\n{\"id\":2}\n",
    )
    .expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 1);
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: stopped at record 1; the request for records 1 to 2 failed: the backend timed out; increase --timeout or try again; 0 records finished\n"
    );
}

#[test]
fn a_body_cut_short_is_sent_once() {
    let listener = failing(Canned::cut_short).expect("a loopback listener");

    let output = decide(&listener, &[], &[], EVIDENCE).expect("the compiled binary runs");

    failed_once(&listener, &output, CLOSED);
}

#[test]
fn a_reply_past_the_bound_is_sent_once() {
    let sent = Arc::new(AtomicUsize::new(0));
    let seen = Arc::clone(&sent);
    let listener = Listener::answering(move |body| {
        seen.store(limit(body), Ordering::SeqCst);
        padded(ANSWERED, limit(body) + 1)
    })
    .expect("a loopback listener");

    let output = decide(&listener, &[], &[], EVIDENCE).expect("the compiled binary runs");

    let said = format!("thinkthen: {}\n", past(sent.load(Ordering::SeqCst)));
    failed_once(&listener, &output, &said);
}

#[test]
fn a_reply_of_exactly_the_bound_is_kept() {
    let listener =
        Listener::answering(|body| padded(ANSWERED, limit(body))).expect("a loopback listener");

    let output =
        decide(&listener, &["--no-cache"], &[], EVIDENCE).expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 1);
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\n");
}

#[test]
fn a_retried_status_is_still_sent_again() {
    let listener = Listener::serving(vec![Canned::status(503, "{}"), Canned::ok(ANSWERED)])
        .expect("a loopback listener");

    let output =
        decide(&listener, &["--details"], &[], EVIDENCE).expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 2);
    assert_eq!(output.status.code(), Some(0));
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(printed.contains(r#""requests_sent":2,"#), "{printed}");
}

#[test]
fn a_reset_adds_one_request_and_no_cache_answer_to_status() {
    let root = folder("resend-status");
    let cache = root.to_string_lossy().into_owned();
    let environment = [("XDG_CACHE_HOME", cache.as_str())];
    let listener = failing(Canned::reset).expect("a loopback listener");

    let output = decide(&listener, &[], &environment, EVIDENCE).expect("the compiled binary runs");

    let status = spawn(&["status", "--json"], &environment, b"").expect("status runs");
    assert_eq!(status.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(value["usage"]["this_month"]["requests_sent"], 1);
    assert_eq!(value["usage"]["this_month"]["cache_answers"], 0);
    failed_once(&listener, &output, CLOSED);
}

#[test]
fn a_record_run_sends_each_request_at_most_once_and_caches_no_failure() {
    let cache = folder("resend-records");
    let named = cache.to_string_lossy().into_owned();
    let listener = Listener::answering(|body| {
        if String::from_utf8_lossy(body).contains("record 2") {
            Canned::reset()
        } else {
            Canned::ok(ANSWERED)
        }
    })
    .expect("a loopback listener");
    let input: String = (1..=3)
        .map(|place| format!("{{\"id\":\"R-{place}\",\"body\":\"record {place}\"}}\n"))
        .collect();

    let arguments = [
        "--jsonl", "--field", "/body", "--jobs", "4", "--cache", &named,
    ];
    let output = decide(&listener, &arguments, &[], &input).expect("the compiled binary runs");

    let bodies: Vec<Vec<u8>> = listener
        .requests()
        .into_iter()
        .map(|seen| seen.body)
        .collect();
    let failed: Vec<&Vec<u8>> = bodies
        .iter()
        .filter(|body| String::from_utf8_lossy(body).contains("record 2"))
        .collect();
    assert_eq!(failed.len(), 1, "the failed record's sends");
    let stored: Vec<_> = stored(&cache)
        .expect("the store")
        .into_iter()
        .map(|answer| answer["key"].as_str().expect("a key").to_owned())
        .collect();
    for body in &bodies {
        assert_eq!(bodies.iter().filter(|seen| *seen == body).count(), 1);
        let failure = String::from_utf8_lossy(body).contains("record 2");
        for key in keys(listener.url(), body) {
            assert_eq!(stored.contains(&key), !failure, "{key}");
        }
    }
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "{CLOSED}thinkthen: stopped at record 2; 1 record finished, 0 records from a recording\n"
        )
    );
}
