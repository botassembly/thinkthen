//! The compiled binary against a loopback backend: the request it sends and the reply it reads.

use std::io;
use std::process::Output;
use std::time::{Duration, Instant};

use crate::harness::{Canned, Listener, spawn};
use crate::support::{digest, encoded_decide};

/// The response a backend gives when it answers the one question that was asked.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// The same response with the probability the case needs.
fn answered(probability: &str) -> String {
    ANSWERED.replace("0.92", probability)
}

/// The bytes the adapter writes for the plan the command was given.
fn encoded(evidence: &str, question: &str) -> Option<Vec<u8>> {
    Some(encoded_decide(evidence, "local-1", question))
}

/// The key a case sends when the case is not about the key itself.
const KEY: Option<&str> = Some("sk-test-value");

/// Run `decide` against one URL, with no environment but what the case names.
///
/// `key` is the value `THINKTHEN_API_KEY` holds, or `None` for a run with the
/// variable unset.
fn decide(base: &str, arguments: &[&str], key: Option<&str>, evidence: &str) -> io::Result<Output> {
    let asked = [
        "decide",
        "asks for a refund",
        "--url",
        base,
        "--model",
        "local-1",
    ];
    spawn(
        &[&asked[..], arguments].concat(),
        &key.map_or_else(Vec::new, |value| vec![("THINKTHEN_API_KEY", value)]),
        evidence.as_bytes(),
    )
}

#[test]
fn the_request_carries_the_encoded_plan_and_the_content_type() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    let output =
        decide(listener.base(), &[], KEY, "Refund me please.").expect("the compiled binary runs");

    let requests = listener.requests();
    let request = requests.first().expect("one request reached the listener");
    assert_eq!(request.line, "POST /v1/systemone HTTP/1.1");
    assert_eq!(request.header("content-type"), Some("application/json"));
    let written = encoded("Refund me please.", "asks for a refund").expect("the plan encodes");
    assert_eq!(
        String::from_utf8_lossy(&request.body),
        String::from_utf8_lossy(&written)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\n");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn every_answer_prints_its_bare_value_and_earns_its_own_exit_code() {
    let cases = [
        ("0.92", "true\n", 0),
        ("0.02", "false\n", 1),
        ("0.5", "null\n", 3),
    ];

    for (probability, printed, code) in cases {
        let listener = Listener::serving(vec![Canned::ok(&answered(probability))])
            .expect("a loopback listener");

        let output = decide(
            listener.base(),
            &["--threshold", "0.1:0.9"],
            KEY,
            "Refund me.",
        )
        .expect("the compiled binary runs");

        assert_eq!(String::from_utf8_lossy(&output.stdout), printed);
        assert_eq!(output.status.code(), Some(code), "{probability}");
    }
}

#[test]
fn a_single_cut_answers_yes_or_no_and_never_leaves_a_run_unresolved() {
    let cases = [("0.92", "true\n", 0), ("0.5", "false\n", 1)];

    for (probability, printed, code) in cases {
        let listener = Listener::serving(vec![Canned::ok(&answered(probability))])
            .expect("a loopback listener");

        let output = decide(listener.base(), &["--threshold", "0.9"], KEY, "Refund me.")
            .expect("the compiled binary runs");

        assert_eq!(String::from_utf8_lossy(&output.stdout), printed);
        assert_eq!(output.status.code(), Some(code), "{probability}");
    }
}

#[test]
fn quiet_prints_nothing_and_keeps_the_exit_code_the_answer_earned() {
    for (probability, code) in [("0.92", 0), ("0.02", 1), ("0.5", 3)] {
        let listener = Listener::serving(vec![Canned::ok(&answered(probability))])
            .expect("a loopback listener");

        let output = decide(
            listener.base(),
            &["--threshold", "0.1:0.9", "--quiet"],
            KEY,
            "Refund me.",
        )
        .expect("the compiled binary runs");

        assert!(output.stdout.is_empty(), "{probability}");
        assert!(output.stderr.is_empty(), "{probability}");
        assert_eq!(output.status.code(), Some(code), "{probability}");
    }
}

#[test]
fn details_prints_the_result_object_and_sends_the_bytes_the_bare_run_sends() {
    let bare = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let detailed = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    decide(bare.base(), &[], KEY, "Refund me.").expect("the compiled binary runs");
    let output = decide(detailed.base(), &["--details"], KEY, "Refund me.")
        .expect("the compiled binary runs");

    let sent = bare.requests();
    let sent = sent.first().expect("one request reached the listener");
    let viewed = detailed.requests();
    let viewed = viewed.first().expect("one request reached the listener");
    assert_eq!(sent.body, viewed.body, "the view changes no request byte");
    let request = digest(detailed.url(), &viewed.body);

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!(
            concat!(
                r#"{{"schema":"thinkthen.result/1","value":true,"#,
                r#""question":{{"verb":"decide","text":"asks for a refund"}},"#,
                r#""answer":{{"kind":"yes_no","probability":0.92}},"threshold":0.5,"#,
                r#""meta":{{"tool":"thinkthen 0.0.1","#,
                r#""question_sha256":"fa2ea2c0b995c700912479bb586ed00efa0227f47d06ede013bf6ac562166c79","#,
                r#""url":"{url}","#,
                r#""model":"jev-1.13.0","usage":{{"input_tokens":312,"output_tokens":48}},"#,
                r#""requests_sent":1,"cached":false,"requests":["{request}"],"failed_questions":0}}}}"#,
                "\n",
            ),
            url = detailed.url(),
            request = request,
        )
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_details_run_carries_the_rule_it_was_judged_under() {
    let listener = Listener::serving(vec![Canned::ok(&answered("0.5"))]).expect("a listener");

    let output = decide(
        listener.base(),
        &["--details", "--threshold", "0.1:0.9"],
        KEY,
        "Refund me.",
    )
    .expect("the compiled binary runs");

    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(printed.contains(r#""value":null,"#), "{printed}");
    assert!(printed.contains(r#""threshold":"0.1:0.9","#), "{printed}");
    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn a_dry_run_prints_the_plan_and_opens_no_connection() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    let output = decide(listener.base(), &["--dry-run"], KEY, "Refund me.")
        .expect("the compiled binary runs");

    assert!(listener.requests().is_empty(), "a plan opens no connection");
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.contains(r#""request":{"state":"Refund me.""#),
        "{printed}"
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_threshold_that_is_refused_stops_before_any_request_goes_out() {
    for bad in ["90", "0", "0.9:0.1", "0.1:", ":0.9", "inf", "NaN", "half"] {
        let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

        let output = decide(listener.base(), &["--threshold", bad], KEY, "Refund me.")
            .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{bad}");
        assert!(listener.requests().is_empty(), "{bad} reached the listener");
        assert!(output.stdout.is_empty(), "{bad}");
    }
}

#[test]
fn a_retried_status_is_sent_again_and_the_second_answer_is_taken() {
    let listener = Listener::serving(vec![
        Canned::status(429, r#"{"error":"slow down"}"#),
        Canned::ok(ANSWERED),
    ])
    .expect("a loopback listener");

    let output = decide(listener.base(), &["--details"], KEY, "Refund me.")
        .expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 2);
    assert_eq!(output.status.code(), Some(0));
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.contains(r#""requests_sent":2,"cached":false"#),
        "{printed}"
    );
}

#[test]
fn retries_run_out_and_the_backend_failure_is_exit_four() {
    let listener = Listener::serving(vec![
        Canned::status(503, "down"),
        Canned::status(503, "down"),
        Canned::status(503, "down"),
    ])
    .expect("a loopback listener");

    let output = decide(listener.base(), &["--max-retries", "2"], KEY, "Refund me.")
        .expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 3);
    assert_eq!(output.status.code(), Some(4));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("503"), "{message}");
    assert!(output.stdout.is_empty());
}

#[test]
fn a_rate_limit_waits_the_seconds_the_backend_asked_for() {
    // The harness shortens the doubling wait to a millisecond, so a run that
    // took a whole second took it from the header and from nowhere else.
    let listener = Listener::serving(vec![
        Canned::status(429, "slow down").asking("retry-after", "1"),
        Canned::ok(ANSWERED),
    ])
    .expect("a loopback listener");

    let started = Instant::now();
    let output = decide(listener.base(), &["--max-retries", "1"], KEY, "Refund me.")
        .expect("the compiled binary runs");
    let took = started.elapsed();

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    assert!(took >= Duration::from_millis(900), "{took:?}");
}

#[test]
fn a_rate_limit_wait_cannot_exceed_the_attempt_timeout() {
    let listener = Listener::serving(vec![
        Canned::status(429, "slow down").asking("retry-after", "30"),
        Canned::ok(ANSWERED),
    ])
    .expect("a loopback listener");

    let started = Instant::now();
    let output = decide(
        listener.base(),
        &["--max-retries", "1", "--timeout", "1"],
        KEY,
        "Refund me.",
    )
    .expect("the compiled binary runs");
    let took = started.elapsed();

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    assert!(took >= Duration::from_millis(900), "{took:?}");
    assert!(took < Duration::from_secs(3), "{took:?}");
}

#[test]
fn zero_retries_never_sleeps_after_the_only_attempt() {
    let listener = Listener::serving(vec![
        Canned::status(429, "slow down").asking("retry-after", "30"),
    ])
    .expect("a loopback listener");
    let started = Instant::now();

    let output = decide(
        listener.base(),
        &["--max-retries", "0", "--timeout", "4"],
        KEY,
        "Refund me.",
    )
    .expect("the compiled binary runs");

    assert!(started.elapsed() < Duration::from_secs(1));
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(output.status.code(), Some(4));
}

#[test]
fn a_rate_limit_in_milliseconds_is_read_before_the_one_in_seconds() {
    // The backend sends both headers. A run that honored the seconds one
    // would sit here for thirty seconds.
    let listener = Listener::serving(vec![
        Canned::status(429, "slow down")
            .asking("retry-after-ms", "900")
            .asking("retry-after", "30"),
        Canned::ok(ANSWERED),
    ])
    .expect("a loopback listener");

    let started = Instant::now();
    let output = decide(listener.base(), &["--max-retries", "1"], KEY, "Refund me.")
        .expect("the compiled binary runs");
    let took = started.elapsed();

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    assert!(took >= Duration::from_millis(800), "{took:?}");
    assert!(took < Duration::from_secs(10), "{took:?}");
}

#[test]
fn an_error_status_that_is_not_retried_fails_at_once() {
    let listener = Listener::serving(vec![Canned::status(
        401,
        r#"{"error":{"message":"Refund me please."}}"#,
    )])
    .expect("a loopback listener");

    let output = decide(listener.base(), &[], Some("sk-bad"), "Refund me please.")
        .expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 1);
    assert_eq!(output.status.code(), Some(4));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("401"), "{message}");
    assert!(!message.contains("Refund me please."), "{message}");
    assert!(!message.contains("sk-bad"), "{message}");
}

#[test]
fn common_request_statuses_name_fixed_actions_and_hide_the_body() {
    let evidence = "private evidence marker";
    let body = format!(r#"{{"error":"{evidence}"}}"#);
    let cases = [
        (
            400,
            "thinkthen: the backend answered with status 400: the backend refused the request; check --model and the request size\n",
        ),
        (
            500,
            "thinkthen: the backend answered with status 500: the backend failed after the allowed attempts; try again later or change --max-retries\n",
        ),
    ];
    for (status, expected) in cases {
        let listener =
            Listener::serving(vec![Canned::status(status, &body)]).expect("a loopback listener");
        let output = decide(listener.base(), &["--max-retries", "0"], KEY, evidence)
            .expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(4), "{status}");
        assert!(output.stdout.is_empty(), "{status}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            expected,
            "{status}"
        );
        assert!(!String::from_utf8_lossy(&output.stderr).contains(evidence));
    }
}

#[test]
fn a_redirect_is_refused_so_no_key_and_no_evidence_reach_another_host() {
    let elsewhere = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let listener =
        Listener::serving(vec![Canned::redirect(elsewhere.url())]).expect("a loopback listener");

    let output = decide(
        listener.base(),
        &["--max-retries", "0"],
        Some("sk-secret-value"),
        "Refund me please.",
    )
    .expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 1);
    assert!(elsewhere.requests().is_empty());
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("302"), "{message}");
}

#[test]
fn a_response_body_past_the_bound_is_exit_four_and_never_fills_memory() {
    let padding = "a".repeat(2 * 1024 * 1024);
    let body = ANSWERED.replace(r#""usage""#, &format!(r#""padding":"{padding}","usage""#));
    let listener = Listener::serving(vec![Canned::ok(&body)]).expect("a loopback listener");

    let output = decide(listener.base(), &["--max-retries", "0"], KEY, "Refund me.")
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
}

#[test]
fn a_reply_the_adapter_refuses_is_exit_four() {
    let cases = [
        r#"{"model":"jev-1.13.0","answers":{"q2":{"type":"noul","noul":0.9}}}"#,
        "not json at all",
        "",
    ];

    for body in cases {
        let listener = Listener::serving(vec![Canned::ok(body)]).expect("a loopback listener");

        let output =
            decide(listener.base(), &[], KEY, "Refund me.").expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(4), "{body}");
        assert!(output.stdout.is_empty(), "{body}");
        assert!(!output.stderr.is_empty(), "{body}");
    }
}

#[test]
fn a_body_the_backend_cut_short_is_exit_four() {
    let listener = Listener::serving(vec![Canned::cut_short()]).expect("a loopback listener");

    let output = decide(listener.base(), &["--max-retries", "0"], KEY, "Refund me.")
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the backend closed the connection before a reply; try again or change --max-retries\n"
    );
}

#[test]
fn a_backend_that_closes_before_headers_fails_promptly() {
    let listener =
        Listener::serving(vec![Canned::close_without_reply()]).expect("a loopback listener");
    let started = Instant::now();

    let output = decide(
        listener.base(),
        &["--max-retries", "0", "--timeout", "4"],
        KEY,
        "private evidence",
    )
    .expect("the compiled binary runs");

    assert!(started.elapsed() < Duration::from_secs(1));
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the backend closed the connection before a reply; try again or change --max-retries\n"
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("private evidence"));
}

#[test]
fn an_open_peer_that_sends_no_reply_reaches_the_timeout_diagnostic() {
    let listener =
        Listener::answering(|_| Canned::ok(ANSWERED).after(2_000)).expect("a loopback listener");
    let started = Instant::now();

    let output = decide(
        listener.base(),
        &["--max-retries", "0", "--timeout", "1"],
        KEY,
        "private evidence",
    )
    .expect("the compiled binary runs");

    assert!(started.elapsed() < Duration::from_secs(3));
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the backend timed out; increase --timeout or try again\n"
    );
}

#[test]
fn a_close_before_headers_follows_the_transport_retry_rule() {
    let listener = Listener::serving(vec![Canned::close_without_reply(), Canned::ok(ANSWERED)])
        .expect("a loopback listener");

    let output = decide(
        listener.base(),
        &["--max-retries", "1", "--details"],
        KEY,
        "Refund me.",
    )
    .expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 2);
    assert_eq!(output.status.code(), Some(0));
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.contains(r#""requests_sent":2,"cached":false"#),
        "{printed}"
    );
}

#[test]
fn a_backend_that_answers_nothing_is_exit_four() {
    let listener = Listener::serving(Vec::new()).expect("a loopback listener");

    let output = decide(listener.base(), &["--max-retries", "0"], KEY, "Refund me.")
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the backend refused the connection; check that it is running and that --url is correct\n"
    );
}

#[test]
fn a_key_variable_that_is_unset_or_blank_is_exit_four_and_never_shows_a_value() {
    let listener = Listener::serving(Vec::new()).expect("a loopback listener");

    for key in [None, Some(""), Some("   ")] {
        let output =
            decide(listener.base(), &[], key, "Refund.").expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(4), "{key:?}");
        assert!(listener.requests().is_empty(), "{key:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("THINKTHEN_API_KEY"), "{message}");
    }
}
