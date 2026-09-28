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
fn a_status_names_its_fixed_action_and_only_the_known_reason() {
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
        (400, r#"{"detail":"max_tokens_exceeded"}"#, REFUSED),
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
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            expected,
            "{status} {body}"
        );
        assert_eq!(listener.requests().len(), 1, "{status} {body}");
    }
}
