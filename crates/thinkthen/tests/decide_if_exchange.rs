//! The compiled binary against a loopback backend: the request it sends and the reply it reads.

mod harness;

use std::io::{self, Write};
use std::process::{Command, Output, Stdio};

use harness::{Canned, Listener};
use thinkthen_core::{Condition, Evidence, ModelName, Plan, Question, systemone};

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
fn encoded(evidence: &str, condition: &str) -> Option<Vec<u8>> {
    let plan = Plan::new(
        Evidence::new(evidence).ok()?,
        ModelName::new("local-1").ok()?,
        vec![Question::new_if(Condition::new(condition).ok()?)],
    )
    .ok()?;
    systemone::encode(&plan).ok()
}

/// Run `decide if` against one URL, with no environment but what the case names.
fn decide(url: &str, arguments: &[&str], key: Option<&str>, evidence: &str) -> io::Result<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .env_clear()
        .env("THINKTHEN_TEST_RETRY_WAIT_MS", "1")
        .args(["decide", "if", "asks for a refund"])
        .args(["--url", url, "--adapter", "systemone", "--model", "local-1"])
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(key) = key {
        command
            .env("LOCAL_KEY", key)
            .args(["--key-env", "LOCAL_KEY"]);
    }
    let mut child = command.spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(evidence.as_bytes());
    drop(input);
    child.wait_with_output()
}

#[test]
fn the_request_carries_the_encoded_plan_the_content_type_and_no_key() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    let output =
        decide(listener.url(), &[], None, "Refund me please.").expect("the compiled binary runs");

    let requests = listener.requests();
    let request = requests.first().expect("one request reached the listener");
    assert_eq!(request.line, "POST /v1/systemone HTTP/1.1");
    assert_eq!(request.header("content-type"), Some("application/json"));
    assert_eq!(request.header("authorization"), None);
    let written = encoded("Refund me please.", "asks for a refund").expect("the plan encodes");
    assert_eq!(
        String::from_utf8_lossy(&request.body),
        String::from_utf8_lossy(&written)
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_named_key_variable_is_sent_as_a_bearer_token_and_never_printed() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    let output = decide(listener.url(), &[], Some("sk-secret-value"), "Refund me.")
        .expect("the compiled binary runs");

    let requests = listener.requests();
    let request = requests.first().expect("one request reached the listener");
    assert_eq!(
        request.header("authorization"),
        Some("Bearer sk-secret-value")
    );
    let printed = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!printed.contains("sk-secret-value"), "{printed}");
}

#[test]
fn the_result_names_the_url_the_adapter_and_the_model_that_answered() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    let output = decide(listener.url(), &["--min-prob", "0.9"], None, "Refund me.")
        .expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!(
            concat!(
                r#"{{"schema":"thinkthen.result/1","#,
                r#""question":{{"verb":"if","condition":"asks for a refund"}},"#,
                r#""answer":{{"kind":"yes_no","probability":0.92}},"#,
                r#""assessment":{{"status":"accepted","value":true,"min_prob":0.9}},"#,
                r#""meta":{{"backend":null,"url":"{url}","adapter":"systemone","#,
                r#""model":"jev-1.13.0","usage":{{"input_tokens":312,"output_tokens":48}}}}}}"#,
                "\n",
            ),
            url = listener.url()
        )
    );
}

#[test]
fn the_status_flag_turns_each_outcome_into_its_own_exit_code() {
    let cases = [
        ("0.92", 0, r#""status":"accepted","value":true"#),
        ("0.02", 1, r#""status":"accepted","value":false"#),
        ("0.5", 3, r#""status":"unsure","value":null"#),
    ];

    for (probability, code, expected) in cases {
        let listener = Listener::serving(vec![Canned::ok(&answered(probability))])
            .expect("a loopback listener");

        let output = decide(
            listener.url(),
            &["--min-prob", "0.9", "--status"],
            None,
            "Refund me.",
        )
        .expect("the compiled binary runs");

        let printed = String::from_utf8_lossy(&output.stdout);
        assert!(printed.contains(expected), "{probability}: {printed}");
        assert_eq!(output.status.code(), Some(code), "{probability}");
    }
}

#[test]
fn without_the_status_flag_every_answer_exits_zero() {
    for probability in ["0.92", "0.02", "0.5"] {
        let listener = Listener::serving(vec![Canned::ok(&answered(probability))])
            .expect("a loopback listener");

        let output = decide(listener.url(), &["--min-prob", "0.9"], None, "Refund me.")
            .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{probability}");
    }
}

#[test]
fn no_pass_mark_leaves_the_result_unassessed() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    let output = decide(listener.url(), &[], None, "Refund me.").expect("the compiled binary runs");

    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.contains(r#""status":"unassessed","value":null,"min_prob":null"#),
        "{printed}"
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_retried_status_is_sent_again_and_the_second_answer_is_taken() {
    let listener = Listener::serving(vec![
        Canned::status(429, r#"{"error":"slow down"}"#),
        Canned::ok(ANSWERED),
    ])
    .expect("a loopback listener");

    let output = decide(listener.url(), &[], None, "Refund me.").expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 2);
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn retries_run_out_and_the_backend_failure_is_exit_four() {
    let listener = Listener::serving(vec![
        Canned::status(503, "down"),
        Canned::status(503, "down"),
        Canned::status(503, "down"),
    ])
    .expect("a loopback listener");

    let output = decide(listener.url(), &["--max-retries", "2"], None, "Refund me.")
        .expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 3);
    assert_eq!(output.status.code(), Some(4));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("503"), "{message}");
    assert!(output.stdout.is_empty());
}

#[test]
fn an_error_status_that_is_not_retried_fails_at_once() {
    let listener = Listener::serving(vec![Canned::status(
        401,
        r#"{"error":{"message":"Refund me please."}}"#,
    )])
    .expect("a loopback listener");

    let output = decide(listener.url(), &[], Some("sk-bad"), "Refund me please.")
        .expect("the compiled binary runs");

    assert_eq!(listener.requests().len(), 1);
    assert_eq!(output.status.code(), Some(4));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("401"), "{message}");
    assert!(!message.contains("Refund me please."), "{message}");
    assert!(!message.contains("sk-bad"), "{message}");
}

#[test]
fn a_redirect_is_refused_so_no_key_and_no_evidence_reach_another_host() {
    let elsewhere = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let listener =
        Listener::serving(vec![Canned::redirect(elsewhere.url())]).expect("a loopback listener");

    let output = decide(
        listener.url(),
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

    let output = decide(listener.url(), &["--max-retries", "0"], None, "Refund me.")
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
            decide(listener.url(), &[], None, "Refund me.").expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(4), "{body}");
        assert!(output.stdout.is_empty(), "{body}");
        assert!(!output.stderr.is_empty(), "{body}");
    }
}

#[test]
fn a_body_the_backend_cut_short_is_exit_four() {
    let listener = Listener::serving(vec![Canned::cut_short()]).expect("a loopback listener");

    let output = decide(listener.url(), &["--max-retries", "0"], None, "Refund me.")
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
}

#[test]
fn a_backend_that_answers_nothing_is_exit_four() {
    let listener = Listener::serving(Vec::new()).expect("a loopback listener");

    let output = decide(listener.url(), &["--max-retries", "0"], None, "Refund me.")
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
}

#[test]
fn a_key_variable_that_is_unset_or_blank_is_exit_four_and_never_shows_a_value() {
    let listener = Listener::serving(Vec::new()).expect("a loopback listener");

    let cases: [(&[&str], Option<&str>); 2] =
        [(&["--key-env", "LOCAL_KEY"], None), (&[], Some("   "))];

    for (arguments, key) in cases {
        let output =
            decide(listener.url(), arguments, key, "Refund.").expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(4), "{key:?}");
        assert!(listener.requests().is_empty(), "{key:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("LOCAL_KEY"), "{message}");
    }
}
