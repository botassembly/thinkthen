//! Status 400 names `max_tokens_exceeded` when its body says so, and nothing else from the body (ticket 0123).

use crate::harness::{Canned, Listener, spawn};

const EVIDENCE: &str = "private evidence marker";
const REFUSED: &str = "thinkthen: the backend answered with status 400: the backend refused the request; check --model and the request size\n";
const NAMED: &str = r#"{"detail":{"error_type":"max_tokens_exceeded"}}"#;

#[test]
fn a_400_names_only_the_known_reason_from_a_bounded_body() {
    let marked = format!(r#"{{"detail":{{"error_type":"{EVIDENCE}"}}}}"#);
    let long = format!(
        r#"{{"detail":{{"error_type":"max_tokens_exceeded"}},"pad":"{}"}}"#,
        "x".repeat(4096)
    );
    let too_long = concat!(
        "thinkthen: the backend answered with status 400 (max_tokens_exceeded): the request has ",
        "more input tokens than the backend takes; shorten the text or set a lower ",
        "max_request_bytes with --profile\n"
    );
    let cases = [
        (400, NAMED, too_long),
        (400, marked.as_str(), REFUSED),
        (400, "", REFUSED),
        (400, "not json", REFUSED),
        (400, r#"{"detail":"max_tokens_exceeded"}"#, REFUSED),
        (400, long.as_str(), REFUSED),
        (
            422,
            NAMED,
            "thinkthen: the backend answered with status 422: the backend refused the request as malformed or too large\n",
        ),
    ];
    for (status, body, expected) in cases {
        let listener =
            Listener::serving(vec![Canned::status(status, body)]).expect("a loopback listener");
        let arguments = [
            "decide",
            "asks for a refund",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--max-retries",
            "0",
        ];
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            EVIDENCE.as_bytes(),
        )
        .expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(4), "{status} {body}");
        assert!(output.stdout.is_empty(), "{status} {body}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            expected,
            "{status} {body}"
        );
        assert_eq!(listener.requests().len(), 1, "{status} {body}");
    }
}
