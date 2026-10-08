//! Actual transport correlation through the public Rust call boundary.

#[cfg(feature = "cli")]
#[path = "../src/test_deadline/child.rs"]
mod child;
#[cfg(feature = "cli")]
use child::ChildEnvironment as _;

use conformance_backend::{Canned, Listener};
use thinkthen::{BatchSetting, CallId, CallOptions, Engine, Question, SdkRequestId, Surface};

const REPLY: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
// The quote form is fixed by specification/records.md, independently of headers.
const BODY: &[u8] = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}}"#;

#[test]
fn public_prepared_send_has_compiled_surface_and_distinct_validated_ids_without_body_changes() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fixture-native-identity")
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let call = engine.decide(&question, "Refund me.").unwrap();
    assert_eq!(call.facts().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
    let requests = listener.requests();
    let request = &requests[0];
    assert_eq!(
        request.header("User-Agent"),
        Some(concat!("thinkthen/", env!("CARGO_PKG_VERSION"), " (rust)"))
    );
    let call_id = request
        .header("X-ThinkThen-Call-Id")
        .unwrap()
        .parse::<CallId>()
        .unwrap();
    let request_id = request
        .header("X-ThinkThen-Request-Id")
        .unwrap()
        .parse::<SdkRequestId>()
        .unwrap();
    assert_ne!(call_id.as_str(), request_id.as_str());
    assert_eq!(call.facts().call_id(), Some(&call_id));
    assert_eq!(request.body, BODY);
}

#[test]
fn status_retries_retain_both_ids_and_a_later_invocation_gets_new_ids() {
    let listener = Listener::serving(vec![
        Canned::status(503, "{}").asking("retry-after-ms", "1"),
        Canned::ok(REPLY),
        Canned::ok(REPLY),
    ])
    .unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fixture-native-identity")
        .unwrap()
        .max_retries(1)
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let first = engine.decide(&question, "Refund me.").unwrap();
    let second = engine.decide(&question, "Refund me.").unwrap();
    assert_eq!(first.facts().requests_sent(), 2);
    assert_eq!(second.facts().requests_sent(), 1);
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    for name in ["X-ThinkThen-Call-Id", "X-ThinkThen-Request-Id"] {
        assert_eq!(requests[0].header(name), requests[1].header(name));
        assert_ne!(requests[1].header(name), requests[2].header(name));
    }
    assert_ne!(first.facts().call_id(), second.facts().call_id());
    for request in requests {
        assert_eq!(request.body, BODY);
    }
}

#[test]
fn refused_parent_and_split_children_share_invocation_but_have_separate_prepared_ids() {
    let listener = Listener::serving(vec![
        Canned::status(413, "{}"),
        Canned::ok(REPLY),
        Canned::ok(REPLY),
    ])
    .unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fixture-native-identity")
        .unwrap()
        .max_retries(0)
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let options = CallOptions::new().batch(BatchSetting::Records(
        std::num::NonZeroUsize::new(2).unwrap(),
    ));
    let mut batch = engine.decide_many_with(&question, ["alpha", "beta"], options);
    assert_eq!(
        batch.by_ref().collect::<Result<Vec<_>, _>>().unwrap().len(),
        2
    );
    let facts = batch.facts().unwrap();
    assert_eq!(facts.requests_sent(), 3);
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    for request in &requests {
        assert_eq!(
            request.header("X-ThinkThen-Call-Id"),
            facts.call_id().map(CallId::as_str)
        );
    }
    let prepared: std::collections::HashSet<_> = requests
        .iter()
        .map(|r| {
            r.header("X-ThinkThen-Request-Id")
                .unwrap()
                .parse::<SdkRequestId>()
                .unwrap()
        })
        .collect();
    assert_eq!(prepared.len(), 3);
    let expected = [
        br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#.as_slice(),
        br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"}}}"#.as_slice(),
        br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#.as_slice(),
    ];
    for (request, expected) in requests.iter().zip(expected) {
        assert_eq!(request.body, expected);
    }
}

#[test]
fn concurrent_calls_have_distinct_invocations_and_prepared_ids() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fixture-native-identity")
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let calls = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| engine.decide(&question, "Refund me.")))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap().unwrap())
            .collect::<Vec<_>>()
    });
    let ids: std::collections::HashSet<_> = calls
        .iter()
        .map(|c| c.facts().call_id().unwrap().clone())
        .collect();
    assert_eq!(ids.len(), 4);
    let requests = listener.requests();
    assert_eq!(requests.len(), 4);
    let prepared: std::collections::HashSet<_> = requests
        .iter()
        .map(|r| {
            r.header("X-ThinkThen-Request-Id")
                .unwrap()
                .parse::<SdkRequestId>()
                .unwrap()
        })
        .collect();
    assert_eq!(prepared.len(), 4);
    for request in requests {
        assert!(
            ids.contains(
                &request
                    .header("X-ThinkThen-Call-Id")
                    .unwrap()
                    .parse::<CallId>()
                    .unwrap()
            )
        );
        assert_eq!(request.body, BODY);
    }
}

#[test]
fn started_failure_and_zero_send_success_keep_ids_and_explicit_outer_surface_is_truthful() {
    let listener = Listener::answering(|_| Canned::status(503, "{}")).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fixture-native-identity")
        .unwrap()
        .max_retries(0)
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let error = engine
        .decide_with(
            &question,
            "Refund me.",
            CallOptions::new().surface(Surface::Cpp),
        )
        .unwrap_err();
    let facts = error.facts().unwrap();
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        requests[0].header("X-ThinkThen-Call-Id"),
        facts.call_id().map(CallId::as_str)
    );
    assert_eq!(
        requests[0].header("User-Agent"),
        Some(concat!("thinkthen/", env!("CARGO_PKG_VERSION"), " (cpp)"))
    );
    let rank = Question::rank("Refund?").unwrap();
    let zero = engine.rank(&rank, Vec::<&str>::new()).unwrap();
    assert!(zero.value().is_empty());
    assert!(zero.facts().call_id().is_some());
    assert_ne!(zero.facts().call_id(), facts.call_id());
    assert_eq!(zero.facts().requests_sent(), 0);
    assert!(zero.facts().model().is_none());
    assert_eq!(listener.count(), 1);
}

#[cfg(feature = "cli")]
#[test]
fn cli_headers_and_final_facts_share_the_admitted_invocation_id() {
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args([
            "decide",
            "Refund?",
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
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"Refund me.")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"true\n");
    assert_eq!(listener.count(), 1);
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let request = &requests[0];
    assert_eq!(request.body, BODY);
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
}

#[cfg(feature = "cli")]
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
