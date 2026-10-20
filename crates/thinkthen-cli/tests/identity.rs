//! Command transport correlation and admission facts through the compiled executable.
#[path = "../../thinkthen/src/test_deadline/child.rs"]
mod child;
use child::ChildEnvironment as _;
use conformance_backend::{Canned, Listener};

#[path = "../../thinkthen/tests/support/identity.rs"]
mod fixture;
use fixture::{BODY, REPLY};
use thinkthen::CallId;

#[test]
fn cli_headers_and_final_facts_share_the_admitted_invocation_id() {
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    for (verb, reply, input) in [
        ("decide", REPLY, b"Refund me.".as_slice()),
        (
            "find",
            r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"u001":0.9,"u002":0.1}}}}"#,
            b"Refund me.\nKeep it.\n".as_slice(),
        ),
    ] {
        let listener = Listener::answering(move |_| Canned::ok(reply)).unwrap();
        let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args([
                verb,
                "Refund?",
                "--details",
                "--url",
                listener.base(),
                "--model",
                "fixed",
                "--no-cache",
                "--facts",
            ])
            .clear_environment()
            .env("THINKTHEN_API_KEY", "fixture-native-identity")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input).unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(0));
        let complete: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        if verb == "find" {
            assert_eq!(complete["value"], "Refund me.");
        } else {
            assert_eq!(complete["value"], true);
        }
        assert_eq!(listener.count(), 1);
        let requests = listener.requests();
        assert_eq!(requests.len(), 1);
        let request = &requests[0];
        if verb == "decide" {
            assert_eq!(request.body, BODY);
        }
        assert_eq!(
            request.header("User-Agent"),
            Some(concat!("thinkthen/", env!("CARGO_PKG_VERSION"), " (cli)"))
        );
        let lines = std::str::from_utf8(&output.stderr).unwrap();
        let facts: serde_json::Value = serde_json::from_str(lines.lines().last().unwrap()).unwrap();
        let call_id = facts
            .get("call_id")
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<CallId>()
            .unwrap();
        assert_eq!(
            request.header("X-ThinkThen-Call-Id"),
            Some(call_id.as_str())
        );
        assert_eq!(
            complete["meta"]["attempts"][0]["sdk_request_id"],
            request.header("X-ThinkThen-Request-Id").unwrap()
        );
        assert_eq!(facts["records"], 1);
        assert_eq!(facts["requests_sent"], 1);
    }
}

#[test]
fn cli_typed_admission_refusal_emits_no_invocation_id_and_sends_nothing() {
    use std::process::{Command, Stdio};
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args([
            "decide",
            "",
            "--url",
            listener.base(),
            "--no-cache",
            "--facts",
        ])
        .clear_environment()
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.count(), 0);
    let lines = std::str::from_utf8(&output.stderr).unwrap();
    let facts: serde_json::Value = serde_json::from_str(lines.lines().last().unwrap()).unwrap();
    assert!(facts.get("call_id").is_none());
    assert_eq!(facts.get("requests_sent"), Some(&serde_json::json!(0)));
}
