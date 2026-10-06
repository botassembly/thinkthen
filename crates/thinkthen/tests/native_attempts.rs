//! Final public invocation facts retain attempts without changing legacy JSON.

use conformance_backend::{Canned, Listener};
use sha2::{Digest as _, Sha256};
use thinkthen::{AttemptOutcome, CallOptions, Engine, Error, Question, Surface, Tally};

const REPLY: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
const BODY: &[u8] = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}}"#;

fn engine(listener: &Listener) -> Result<Engine, Error> {
    Engine::builder()
        .base_url(listener.base())?
        .model("fixed")?
        .api_key("fixture-native-attempts")?
        .max_retries(1)
        .no_cache()
        .build()
}

#[test]
fn retries_keep_sdk_identity_and_ordered_final_attempts_while_provider_id_stays_distinct() {
    let listener = Listener::serving(vec![
        Canned::status(503, "{}").asking("retry-after-ms", "1"),
        Canned::ok(REPLY)
            .asking("x-typesafe-request-id", "provider-request-1")
            .asking("x-envoy-upstream-service-time", "7"),
    ])
    .unwrap();
    let engine = engine(&listener).unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let call = engine
        .decide_with(&question, "Refund me.", CallOptions::new().attempts(true))
        .unwrap();
    let facts = call.facts();
    let attempts = facts.attempts().unwrap();
    assert_eq!(attempts.len(), 2);
    assert_eq!((attempts[0].ordinal(), attempts[1].ordinal()), (1, 2));
    assert_eq!(attempts[0].outcome(), AttemptOutcome::Status);
    assert_eq!(attempts[1].outcome(), AttemptOutcome::Ok);
    assert_eq!(attempts[0].sdk_request_id(), attempts[1].sdk_request_id());
    assert_eq!(attempts[0].server_ms(), None);
    assert_eq!(attempts[0].request_id(), None);
    assert_eq!(attempts[1].server_ms(), Some(7));
    assert_eq!(attempts[1].request_id(), Some("provider-request-1"));
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    // The released exchange digest is independently specified as adapter LF URL LF body.
    let mut bytes = format!("systemone\n{}/systemone\n", listener.base()).into_bytes();
    bytes.extend_from_slice(BODY);
    let digest: String = Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    for (attempt, request) in attempts.iter().zip(&requests) {
        assert_eq!(request.body, BODY);
        assert_eq!(
            attempt.sdk_request_id().as_str(),
            request.header("X-ThinkThen-Request-Id").unwrap()
        );
        assert_eq!(attempt.request_sha256(), digest);
        assert_eq!(
            facts.call_id().unwrap().as_str(),
            request.header("X-ThinkThen-Call-Id").unwrap()
        );
        assert_ne!(
            attempt.sdk_request_id().as_str(),
            attempt.request_id().unwrap_or_default()
        );
    }
}

#[test]
fn started_failure_retains_attempts_and_unrequested_or_empty_calls_do_not_invent_them() {
    let listener = Listener::answering(|_| Canned::status(400, "{}")).unwrap();
    let engine = engine(&listener).unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let error = engine
        .decide_with(&question, "Refund me.", CallOptions::new().attempts(true))
        .unwrap_err();
    let facts = error.facts().unwrap();
    let attempts = facts.attempts().unwrap();
    assert_eq!(attempts.len(), 1);
    assert_eq!(attempts[0].status(), Some(400));
    assert_eq!(attempts[0].outcome(), AttemptOutcome::Status);
    let complete = serde_json::to_value(facts.complete().unwrap()).unwrap();
    assert_eq!(complete["call_id"], facts.call_id().unwrap().as_str());
    assert_eq!(
        complete["attempts"][0]["sdk_request_id"],
        attempts[0].sdk_request_id().as_str()
    );

    assert_eq!(complete["requests_sent"], 1);
    assert!(complete.get("input_tokens").is_none());
    let legacy = serde_json::to_value(facts).unwrap();
    assert!(legacy.get("call_id").is_none());
    assert!(legacy.get("attempts").is_none());
    assert!(
        serde_json::to_value(&attempts[0])
            .unwrap()
            .get("sdk_request_id")
            .is_none()
    );
    let default = engine.decide(&question, "Refund me.").unwrap_err();
    assert!(default.facts().unwrap().attempts().is_none());
    let rank = Question::rank("Refund?").unwrap();
    let empty = engine
        .rank_with(
            &rank,
            std::iter::empty::<&str>(),
            CallOptions::new().attempts(true),
        )
        .unwrap();
    assert_eq!(empty.facts().requests_sent(), 0);
    assert_eq!(empty.facts().attempts(), Some([].as_slice()));
    let complete = serde_json::to_value(empty.facts().complete().unwrap()).unwrap();
    assert_eq!(complete["attempts"], serde_json::json!([]));
    assert!(complete.get("model").is_none());
    assert!(complete.get("input_tokens").is_none());
    assert!(Tally::new().facts().complete().is_none());
    assert_eq!(listener.count(), 2);
    let requests = listener.requests();
    assert_eq!(
        requests[0].header("X-ThinkThen-Request-Id"),
        Some(attempts[0].sdk_request_id().as_str())
    );
}

#[test]
fn detailed_and_caller_thread_attempt_observers_share_one_retained_event() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let engine = engine(&listener).unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let held = std::sync::Mutex::new(Vec::new());
    let caller = std::thread::current().id();
    let observe = |attempt| {
        assert_eq!(std::thread::current().id(), caller);
        held.lock().unwrap().push(attempt);
    };
    let call = engine
        .details_with(
            &question,
            "Refund me.",
            CallOptions::new().observe_attempt(&observe),
        )
        .unwrap();
    let events = held.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(call.facts().attempts().unwrap(), events.as_slice());
    assert_eq!(listener.count(), 1);
}

#[test]
fn mcp_surface_is_explicit_and_uses_the_compiled_engine_on_the_same_route() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let engine = engine(&listener).unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let call = engine
        .decide_with(
            &question,
            "Refund me.",
            CallOptions::new().surface(Surface::Mcp),
        )
        .unwrap();
    assert_eq!(listener.count(), 1);
    let requests = listener.requests();
    assert_eq!(
        requests[0].header("User-Agent"),
        Some(concat!("thinkthen/", env!("CARGO_PKG_VERSION"), " (mcp)"))
    );
    assert_eq!(
        requests[0].header("X-ThinkThen-Call-Id"),
        call.facts().call_id().map(thinkthen::CallId::as_str)
    );
    assert_eq!(requests[0].body, BODY);
}
