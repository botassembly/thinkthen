//! Every status names its fixed action. Status 400 names `max_tokens_exceeded`
//! when its body says so, and nothing else from the body (tickets 0123 and 0138).

use crate::harness::{Canned, Listener};
use crate::support::decide;

const EVIDENCE: &str = "private evidence marker";
const REFUSED: &str =
    "thinkthen: the backend answered with status 400: the backend refused the request\n";
const NAMED: &str = r#"{"detail":{"error_type":"max_tokens_exceeded"}}"#;
const RETRIED_OUT: &str =
    "the backend failed after the allowed attempts; try again later or change --max-retries\n";

#[test]
fn a_status_keeps_its_action_and_only_the_documented_token_limit_shape() {
    let marked = format!(r#"{{"detail":{{"error_type":"{EVIDENCE}"}}}}"#);
    let long = format!(
        r#"{{"detail":{{"error_type":"max_tokens_exceeded"}},"pad":"{}"}}"#,
        "x".repeat(4096)
    );
    let too_long = concat!(
        "thinkthen: the backend answered with status 400 (max_tokens_exceeded): the request has ",
        "more input tokens than the backend takes; shorten the text; provider type: max_tokens_exceeded\n"
    );
    let retried = [500, 502, 503, 504, 520, 521, 522, 523, 524, 529].map(|status| {
        let said = format!("thinkthen: the backend answered with status {status}: {RETRIED_OUT}");
        (status, said)
    });
    let mut cases = vec![
        (400, NAMED, too_long),
        (400, marked.as_str(), REFUSED),
        (400, "", REFUSED),
        (400, "not json", REFUSED),
        (
            400,
            r#"{"detail":"max_tokens_exceeded"}"#,
            "thinkthen: the backend answered with status 400: the backend refused the request\n",
        ),
        (
            400,
            long.as_str(),
            "thinkthen: the backend answered with status 400: the backend refused the request; provider type: max_tokens_exceeded\n",
        ),
        (
            413,
            "{}",
            "thinkthen: the backend answered with status 413: the backend refused the request as too large; shorten the text\n",
        ),
        (
            422,
            NAMED,
            "thinkthen: the backend answered with status 422: the backend refused the request as malformed or too large; provider type: max_tokens_exceeded\n",
        ),
        (
            429,
            "{}",
            "thinkthen: the backend answered with status 429: the backend's rate limit was reached after the allowed attempts; try again later or change --max-retries\n",
        ),
    ];
    for (status, said) in &retried {
        cases.push((*status, marked.as_str(), said.as_str()));
    }
    for (status, body, expected) in cases {
        let listener =
            Listener::serving(vec![Canned::status(status, body)]).expect("a loopback listener");
        let output = decide(
            listener.base(),
            &["--max-retries", "0"],
            Some("sk-test-value"),
            EVIDENCE,
        )
        .expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(4), "{status} {body}");
        assert!(output.stdout.is_empty(), "{status} {body}");
        assert_eq!(crate::harness::said(&output), expected, "{status} {body}");
        assert_eq!(listener.requests().len(), 1, "{status} {body}");
    }
}

#[test]
fn quota_stops_without_retry_and_reports_only_the_provider_type() {
    let body =
        r#"{"error":{"type":"insufficient_quota","message":"You have no credits remaining"}}"#;
    let listener = Listener::serving(vec![Canned::status(429, body)]).unwrap();
    let output = decide(
        listener.base(),
        &["--max-retries", "3"],
        Some("sk-test-value"),
        EVIDENCE,
    )
    .unwrap();
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(
        crate::harness::said(&output),
        "thinkthen: the backend systemone account is out of credit; provider type: insufficient_quota\n"
    );
}

#[test]
fn provider_messages_are_discarded_including_credential_and_evidence_echoes() {
    for message in [
        "The selected model is unavailable",
        "sk-test-value",
        EVIDENCE,
        "private \"quoted\" evidence",
        "c2stdGVzdC12YWx1ZQ==",
        "echo:c2stdGVzdC12YWx1ZQ==",
        "echo(c2stdGVzdC12YWx1ZQ==)",
        "echo-c2stdGVzdC12YWx1ZQ==",
        "echo_c2stdGVzdC12YWx1ZQ==",
        "prefixc2stdGVzdC12YWx1ZQ==suffix",
        "echo+c2stdGVzdC12YWx1ZQ==/suffix",
        "echo-c2stdGVzdC12YWx1ZQ",
        "echo_736b2d746573742d76616c7565suffix",
        "echo/736b2d746573742d76616c7565!",
        "echo(sk%2Dtest%2Dvalue)",
        "The backend rejected private evidence marker because it is unsupported",
        "736b2d746573742d76616c7565",
        "echo=736b2d746573742d76616c7565",
        "sk%2Dtest%2Dvalue",
        "Rejected sk%2Dtest%2Dvalue% because it is invalid",
        "%sk%2Dtest%2Dvalue",
        "%2sk%2Dtest%2Dvalue",
        "%GGsk%2Dtest%2Dvalue",
        "sk%2Dtest%2Dvalue%2",
        "sk%2Dtest%2Dvalue%GG",
        "sk%2Dtest%2Dvalue%2G",
        "sk%2Dtest%2Dvalue%G2",
        "sk%2Dtest%GG%2Dvalue",
        "https://example.org/sk%2Dtest%2Dvalue%",
        "https://example.org/?key=sk%2Dtest%2Dvalue%2",
        "%c2stdGVzdC12YWx1ZQ==",
        "c2stdGVzdC12YWx1ZQ==%2",
        "c2stdGVzd%GGC12YWx1ZQ==",
        "%736b2d746573742d76616c7565",
        "736b2d746573742d76616c7565%GG",
        "736b2d746573%GG742d76616c7565",
        "content-type: application/json",
        "Host: example.org",
        "X-Custom: hello",
        "Request failed.\nHost: example.org",
        "Authorization: Bearer something",
        "X-Api-Key: something",
    ] {
        let body = serde_json::json!({"error":{"message":message}}).to_string();
        let listener = Listener::serving(vec![Canned::status(400, &body)]).unwrap();
        let output = decide(
            listener.base(),
            &["--max-retries", "0"],
            Some("sk-test-value"),
            if message == "private \"quoted\" evidence" {
                message
            } else {
                EVIDENCE
            },
        )
        .unwrap();
        let said = crate::harness::said(&output);
        assert_eq!(output.status.code(), Some(4));
        assert!(output.stdout.is_empty());
        assert_eq!(listener.requests().len(), 1);
        assert_eq!(said, REFUSED);
    }
}

#[test]
fn reflected_nested_evidence_and_unrelated_prose_are_withheld() {
    for (evidence, message) in [
        ("Z", "Rejected Z because it is unsupported"),
        ("Ada", "Rejected Ada because it is unsupported"),
        ("S7code!", "Rejected S7code! because it is unsupported"),
        ("S8code!!", "Rejected S8code!! because it is unsupported"),
        (
            r#"{"nested":{"text":"private \"quoted\" evidence"}}"#,
            "Rejected private \"quoted\" evidence because it is unsupported",
        ),
        ("unrelated evidence", "The selected model is unavailable"),
        ("separate evidence", "The selected model is unavailable"),
    ] {
        let body = serde_json::json!({"error":{"message":message}}).to_string();
        let listener = Listener::serving(vec![Canned::status(400, &body)]).unwrap();
        let output = decide(
            listener.base(),
            &["--max-retries", "0"],
            Some("sk-test-value"),
            evidence,
        )
        .unwrap();
        assert_eq!(output.status.code(), Some(4));
        assert!(output.stdout.is_empty());
        assert_eq!(listener.requests().len(), 1);
        let expected = REFUSED;
        assert_eq!(crate::harness::said(&output), expected);
    }
}

#[test]
fn actionable_and_multibyte_provider_prose_is_withheld() {
    for message in [
        "This endpoint does not support the requested model. Load a compatible model first."
            .to_owned(),
        "Your API key has expired. Create a replacement in your account.".to_owned(),
        "é ".repeat(200),
    ] {
        let body = serde_json::json!({"detail":{"message":message}}).to_string();
        let listener = Listener::serving(vec![Canned::status(400, &body)]).unwrap();
        let output = decide(
            listener.base(),
            &["--max-retries", "0"],
            Some("sk-test-value"),
            EVIDENCE,
        )
        .unwrap();
        assert_eq!(output.status.code(), Some(4));
        assert_eq!(listener.requests().len(), 1);
        assert_eq!(crate::harness::said(&output), REFUSED);
    }
}

#[test]
fn plain_text_and_html_are_withheld() {
    for (media_type, message) in [
        (
            "text/plain; charset=utf-8",
            "The local endpoint requires a decision model.",
        ),
        ("text/plain", "sk-test-value"),
        (
            "text/plain",
            "<html>The local endpoint requires a decision model.</html>",
        ),
        ("text/html", "The local endpoint requires a decision model."),
    ] {
        let listener =
            Listener::serving(vec![Canned::status(400, message).media_type(media_type)]).unwrap();
        let output = decide(
            listener.base(),
            &["--max-retries", "0"],
            Some("sk-test-value"),
            EVIDENCE,
        )
        .unwrap();
        assert_eq!(output.status.code(), Some(4));
        assert_eq!(listener.requests().len(), 1);
        let expected = REFUSED;
        assert_eq!(crate::harness::said(&output), expected);
    }
}

#[test]
fn readable_rate_limit_prose_does_not_turn_a_real_limit_into_a_quota_stop() {
    let body =
        r#"{"error":{"type":"rate_limit_exceeded","message":"Requests are arriving too quickly"}}"#;
    let listener = Listener::answering(move |_| Canned::status(429, body)).unwrap();
    let output = decide(
        listener.base(),
        &["--max-retries", "1", "--facts"],
        Some("sk-test-value"),
        EVIDENCE,
    )
    .unwrap();
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(listener.requests().len(), 2);
    let said = crate::harness::said(&output);
    assert!(said.starts_with("thinkthen: the backend answered with status 429: the backend's rate limit was reached after the allowed attempts; try again later or change --max-retries; provider type: rate_limit_exceeded\n"));
    let facts: serde_json::Value = serde_json::from_str(said.lines().last().unwrap()).unwrap();
    assert_eq!(facts["retries"], 1);
    assert_eq!(
        facts["stopped"],
        serde_json::json!({"at":1,"cause":"status","status":429,"retryable":true})
    );
}

#[test]
fn only_known_error_identifier_fields_cross_the_boundary() {
    for (body, suffix) in [
        (
            r#"{"error":{"type":"invalid_request_error","code":"model_missing","message":"private prose"}}"#,
            "; provider type: invalid_request_error; provider code: model_missing",
        ),
        (
            r#"{"detail":{"error_type":"model_missing","message":"private prose"}}"#,
            "; provider type: model_missing",
        ),
        (
            r#"{"type":"request_error","code":"404","message":"private prose"}"#,
            "; provider type: request_error; provider code: 404",
        ),
        (
            r#"{"error":{"type":"sk-test-value","code":"c2stdGVzdC12YWx1ZQ"}}"#,
            "",
        ),
        (
            r#"{"error":{"type":"sk%2Dtest%2Dvalue","code":"736b2d746573742d76616c7565"}}"#,
            "",
        ),
        (
            r#"{"error":{"type":"private evidence marker","code":"Authorization: fixture"}}"#,
            "",
        ),
        (
            r#"{"error":{"type":42,"code":null},"other":{"type":"hidden"}}"#,
            "",
        ),
    ] {
        let listener = Listener::serving(vec![Canned::status(400, body)]).unwrap();
        let output = decide(
            listener.base(),
            &["--max-retries", "0", "--facts"],
            Some("sk-test-value"),
            EVIDENCE,
        )
        .unwrap();
        let said = crate::harness::said(&output);
        assert_eq!(output.status.code(), Some(4));
        let mut lines = said.lines();
        assert_eq!(
            lines.next().unwrap(),
            format!("{}{suffix}", REFUSED.trim_end())
        );
        let facts: serde_json::Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        assert_eq!(
            facts["stopped"],
            serde_json::json!({"at":1,"cause":"status","status":400,"retryable":false})
        );
        assert!(lines.next().is_none());
    }
}

#[test]
fn too_large_remedies_fit_each_commands_request_shape() {
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("size-advice-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let set = folder.join("questions.json");
    std::fs::write(
        &set,
        r#"{"version":1,"questions":{"yes":{"decide":"Is this useful?"}}}"#,
    )
    .unwrap();
    let set = set.to_str().unwrap();
    for (arguments, evidence, advice) in size_advice_cases(set) {
        assert_size_advice(&arguments, evidence, advice);
    }
    std::fs::remove_dir_all(folder).unwrap();
}

fn size_advice_cases(set: &str) -> Vec<(Vec<&str>, &str, &str)> {
    let mut cases = vec![
        (
            vec!["find", "Which fits?"],
            "first\nsecond\n",
            "shorten the text or use fewer candidates",
        ),
        (vec!["annotate", set], "first", "shorten the text"),
        (
            vec!["annotate", set, "--lines"],
            "first\n",
            "shorten the text or set a lower --max-request-bytes",
        ),
        (
            vec!["relate", "works_for=person:organization"],
            r#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#,
            "shorten the text or set a lower --max-request-bytes",
        ),
        (
            vec!["recognize"],
            "Ada",
            "shorten the text or set a lower --max-request-bytes",
        ),
    ];
    for arguments in [
        vec!["decide", "Is this useful?"],
        vec!["filter", "Is this useful?"],
        vec!["rank", "Is this useful?"],
        vec!["choose", "Which fits?", "yes", "no"],
        vec!["tag", "Which fit?", "yes", "no"],
        vec!["score", "How useful?", "low", "high"],
    ] {
        if !matches!(arguments[0], "filter" | "rank") {
            cases.push((arguments.clone(), "first", "shorten the text"));
        }
        let mut records = arguments;
        records.push("--lines");
        cases.push((
            records,
            "first\n",
            "shorten the text or set a lower --max-request-bytes",
        ));
    }
    cases
}

fn assert_size_advice(arguments: &[&str], evidence: &str, advice: &str) {
    let help = crate::harness::spawn(&[arguments[0], "--help"], &[], b"").unwrap();
    assert_eq!(help.status.code(), Some(0));
    for flag in advice
        .split_whitespace()
        .filter(|word| word.starts_with("--"))
    {
        assert!(
            String::from_utf8_lossy(&help.stdout).contains(flag),
            "{} {flag}",
            arguments[0]
        );
    }
    for (status, body, reason) in [
        (
            400,
            NAMED,
            "the backend answered with status 400 (max_tokens_exceeded): the request has more input tokens than the backend takes",
        ),
        (
            413,
            "{}",
            "the backend answered with status 413: the backend refused the request as too large",
        ),
    ] {
        let expected_requests = if arguments[0] == "tag" { 2 } else { 1 };
        let listener = Listener::serving(
            (0..expected_requests)
                .map(|_| Canned::status(status, body))
                .collect(),
        )
        .unwrap();
        let mut arguments = arguments.to_vec();
        arguments.extend([
            "--url",
            listener.base(),
            "--no-cache",
            "--max-retries",
            "0",
            "--facts",
        ]);
        let output = crate::harness::spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "fixture-key")],
            evidence.as_bytes(),
        )
        .unwrap();
        let said = crate::harness::said(&output);
        assert_eq!(output.status.code(), Some(4), "{arguments:?}: {said}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
        assert_eq!(
            listener.requests().len(),
            expected_requests,
            "{arguments:?}"
        );
        let provider = if status == 400 {
            "; provider type: max_tokens_exceeded"
        } else {
            ""
        };
        let sentence = format!("{reason}; {advice}{provider}");
        let expected = if status == 413
            && arguments[0] == "annotate"
            && arguments.contains(&"--lines")
        {
            format!(
                "thinkthen: stopped at record 1; the request for records 1 to 1 failed: {sentence}; 0 records finished"
            )
        } else {
            format!("thinkthen: {sentence}")
        };
        assert_eq!(said.lines().next().unwrap(), expected, "{arguments:?}");
        let facts: serde_json::Value = serde_json::from_str(said.lines().last().unwrap()).unwrap();
        assert_eq!(
            facts["stopped"],
            serde_json::json!({"at":1,"cause":"too_large","retryable":false}),
            "{arguments:?}: {said}"
        );
    }
}

#[test]
fn too_large_backend_probes_suggest_a_backend_with_a_larger_input_limit() {
    let listener = Listener::answering(|_| Canned::status(400, NAMED)).unwrap();
    let output = crate::harness::spawn(
        &["backends", "check", "--url", listener.base()],
        &[("THINKTHEN_API_KEY", "fixture-key")],
        b"",
    )
    .unwrap();
    let shown = String::from_utf8_lossy(&output.stdout);
    assert_eq!(output.status.code(), Some(4), "{shown}");
    assert!(!listener.requests().is_empty());
    assert!(shown.contains("the backend answered with status 400 (max_tokens_exceeded): the request has more input tokens than the backend takes; use a backend with a larger input limit; provider type: max_tokens_exceeded"), "{shown}");
    assert!(!shown.contains("--max-request-bytes"), "{shown}");
}

#[test]
fn image_only_refusals_suggest_a_backend_with_a_larger_input_limit() {
    let image = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/images/red.png");
    for input in [
        vec!["--image", image.to_str().unwrap()],
        vec![
            "--input",
            image.to_str().unwrap(),
            "--unit",
            "file",
            "--media",
            "image",
        ],
    ] {
        let listener = Listener::serving(vec![Canned::status(400, NAMED)]).unwrap();
        let mut arguments = vec![
            "decide",
            "Is red visible?",
            "--backend",
            "liquid",
            "--model",
            "d1",
            "--url",
            listener.base(),
            "--no-cache",
            "--max-retries",
            "0",
            "--facts",
        ];
        arguments.extend(input);
        let output =
            crate::harness::spawn(&arguments, &[("THINKTHEN_API_KEY", "fixture-key")], b"")
                .unwrap();
        let said = crate::harness::said(&output);
        assert_eq!(output.status.code(), Some(4), "{arguments:?}: {said}");
        assert!(output.stdout.is_empty());
        assert_eq!(listener.requests().len(), 1);
        assert_eq!(
            said.lines().next().unwrap(),
            "thinkthen: the backend answered with status 400 (max_tokens_exceeded): the request has more input tokens than the backend takes; use a backend with a larger input limit; provider type: max_tokens_exceeded"
        );
        let facts: serde_json::Value = serde_json::from_str(said.lines().last().unwrap()).unwrap();
        assert_eq!(
            facts["stopped"],
            serde_json::json!({"at":1,"cause":"too_large","retryable":false})
        );
    }
}
