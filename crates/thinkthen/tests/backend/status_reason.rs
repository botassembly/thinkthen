//! Every status names its fixed action. Status 400 names `max_tokens_exceeded`
//! when its body says so, and nothing else from the body (tickets 0123 and 0138).

use crate::harness::{Canned, Listener};
use crate::support::decide;

const EVIDENCE: &str = "private evidence marker";
const REFUSED: &str = "thinkthen: the backend answered with status 400: the backend refused the request; check --model and the request size\n";
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
        "more input tokens than the backend takes; shorten the text, or set a lower ",
        "--max-request-bytes or max_request_bytes with --profile\n"
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
            "thinkthen: the backend answered with status 400: the backend refused the request; check --model and the request size; provider message: max_tokens_exceeded\n",
        ),
        (400, long.as_str(), REFUSED),
        (
            413,
            "{}",
            "thinkthen: the backend answered with status 413: the backend refused the request as too large; shorten the text, or set a lower --max-request-bytes or max_request_bytes with --profile\n",
        ),
        (
            422,
            NAMED,
            "thinkthen: the backend answered with status 422: the backend refused the request as malformed or too large\n",
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
fn quota_stops_without_retry_and_reports_the_providers_words() {
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
        "thinkthen: the backend systemone account is out of credit; provider message: You have no credits remaining\n"
    );
}

#[test]
fn safe_provider_messages_are_bounded_and_credential_echoes_are_withheld() {
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
        "aabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccdd",
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
        assert_eq!(listener.requests().len(), 1);
        if message == "The selected model is unavailable" {
            assert_eq!(
                said,
                "thinkthen: the backend answered with status 400: the backend refused the request; check --model and the request size; provider message: The selected model is unavailable\n"
            );
        } else {
            assert_eq!(said, REFUSED);
        }
    }
}

#[test]
fn reflected_nested_evidence_is_withheld_without_hiding_unrelated_prose() {
    for (evidence, message, shown) in [
        ("Z", "Rejected Z because it is unsupported", false),
        ("Ada", "Rejected Ada because it is unsupported", false),
        (
            "S7code!",
            "Rejected S7code! because it is unsupported",
            false,
        ),
        (
            "S8code!!",
            "Rejected S8code!! because it is unsupported",
            false,
        ),
        (
            r#"{"nested":{"text":"private \"quoted\" evidence"}}"#,
            "Rejected private \"quoted\" evidence because it is unsupported",
            false,
        ),
        (
            "unrelated evidence",
            "The selected model is unavailable",
            true,
        ),
        (
            "separate evidence",
            "The selected model is unavailable",
            true,
        ),
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
        let expected = if shown {
            format!("{}; provider message: {message}\n", REFUSED.trim_end())
        } else {
            REFUSED.to_owned()
        };
        assert_eq!(crate::harness::said(&output), expected);
    }
}

#[test]
fn provider_prose_keeps_actionable_local_errors_and_trims_at_a_character_boundary() {
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
        let normalized = message.split_whitespace().collect::<Vec<_>>().join(" ");
        let expected = if normalized.chars().count() > 320 {
            normalized.chars().take(320).collect::<String>() + "…"
        } else {
            normalized
        };
        assert_eq!(output.status.code(), Some(4));
        assert_eq!(listener.requests().len(), 1);
        assert_eq!(
            crate::harness::said(&output),
            format!("{}; provider message: {expected}\n", REFUSED.trim_end())
        );
    }
}

#[test]
fn declared_plain_text_uses_the_same_boundary_and_html_is_withheld() {
    for (media_type, message, shown) in [
        (
            "text/plain; charset=utf-8",
            "The local endpoint requires a decision model.",
            true,
        ),
        ("text/plain", "sk-test-value", false),
        (
            "text/plain",
            "<html>The local endpoint requires a decision model.</html>",
            false,
        ),
        (
            "text/html",
            "The local endpoint requires a decision model.",
            false,
        ),
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
        let expected = if shown {
            format!("{}; provider message: {message}\n", REFUSED.trim_end())
        } else {
            REFUSED.to_owned()
        };
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
    assert!(said.starts_with("thinkthen: the backend answered with status 429: the backend's rate limit was reached after the allowed attempts; try again later or change --max-retries; provider message: Requests are arriving too quickly\n"));
    let facts: serde_json::Value = serde_json::from_str(said.lines().last().unwrap()).unwrap();
    assert_eq!(facts["retries"], 1);
    assert_eq!(
        facts["stopped"],
        serde_json::json!({"at":1,"cause":"status","status":429,"retryable":true})
    );
}
