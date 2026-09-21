//! The size one record may reach before the tool refuses to judge it.

use std::io;
use std::process::Output;

use thinkthen_core::MAX_RECORD_BYTES;

use crate::harness::{Canned, Listener, spawn};

/// The sentence a record past the limit is refused with.
const REFUSED: &str =
    "thinkthen: the record is over 16 MiB, which is far past what a backend reads in one request\n";

/// The response the listener gives to the one question a case asks.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.97}},"#,
    r#""usage":{"input_tokens":88,"output_tokens":12}}"#,
);

/// A listener that answers this many requests and records every one of them.
fn serving(answers: usize) -> io::Result<Listener> {
    Listener::serving((0..answers).map(|_| Canned::ok(ANSWERED)).collect())
}

/// Run `decide` against one base over the bytes on standard input.
fn decide(base: &str, arguments: &[&str], input: &str) -> io::Result<Output> {
    let asked = [
        "decide",
        "Does this report a payment failure?",
        "--url",
        base,
        "--model",
        "local-1",
    ];
    spawn(
        &[&asked[..], arguments].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
}

/// One record of this many bytes, made of a byte no message would quote.
fn wide(bytes: usize) -> String {
    "x".repeat(bytes)
}

/// A record over the limit stops the run before it is sent anywhere.
///
/// The listener counts the requests, so the case proves the huge record never
/// crossed the wire. The first record is answered and printed, which shows the
/// refusal belongs to the one record rather than to the command line.
#[test]
fn a_record_over_the_limit_stops_the_run_and_sends_nothing_for_itself() {
    let listener = serving(2).expect("a loopback listener");
    let good = "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n";
    let input = format!("{good}{{\"body\":\"{}\"}}\n", wide(MAX_RECORD_BYTES));

    let output = decide(listener.base(), &["--jsonl", "--field", "/body"], &input)
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\n");
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "{REFUSED}thinkthen: stopped at record 2; 1 record finished, 0 records from a recording\n"
        )
    );
}

/// One document over the limit is refused with no request at all.
#[test]
fn a_document_over_the_limit_is_refused_before_any_request() {
    let listener = serving(1).expect("a loopback listener");

    let output = decide(listener.base(), &[], &wide(MAX_RECORD_BYTES + 1))
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(String::from_utf8_lossy(&output.stderr), REFUSED);
    assert!(listener.requests().is_empty());
}

/// A record of exactly the limit is judged, however the line that held it ended.
#[test]
fn a_record_of_exactly_the_limit_is_judged() {
    for ending in ["\n", "\r\n"] {
        let listener = serving(1).expect("a loopback listener");
        let input = format!("{}{ending}", wide(MAX_RECORD_BYTES));

        let output =
            decide(listener.base(), &["--lines"], &input).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{ending:?}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "true\n",
            "{ending:?}"
        );
        assert_eq!(listener.requests().len(), 1, "{ending:?}");
    }
}

/// A line past the bound is one record, and its tail never becomes another.
///
/// The reader stops two bytes past the limit, so what follows the cut is the
/// middle of that same record. Framing it as a record of its own would send
/// part of a refused record to the backend. The listener counts the requests,
/// so the case proves the tail was never sent.
#[test]
fn the_tail_of_a_record_past_the_bound_is_never_framed_as_a_record() {
    let listener = serving(4).expect("a loopback listener");
    let input = format!("{}\n", wide(MAX_RECORD_BYTES + 12));

    let output = decide(listener.base(), &["--lines", "--jobs", "4"], &input)
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(listener.requests().is_empty(), "the tail was sent");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "{REFUSED}thinkthen: stopped at record 1; 0 records finished, 0 records from a recording\n"
        )
    );
}
