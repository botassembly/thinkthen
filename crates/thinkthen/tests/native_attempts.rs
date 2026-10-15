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
        .decide_with(
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

#[test]
fn a_connection_that_never_opened_gets_one_retry_even_with_no_status_retries() {
    let closed = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = closed.local_addr().unwrap();
    drop(closed);
    let engine = Engine::builder()
        .base_url(&format!("http://{address}"))
        .unwrap()
        .api_key("fixture-native-attempts")
        .unwrap()
        .max_retries(0)
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let error = engine
        .decide_with(&question, "Refund me.", CallOptions::new().attempts(true))
        .unwrap_err();
    let attempts = error.facts().unwrap().attempts().unwrap();
    assert_eq!(attempts.len(), 2);
    assert!(
        attempts
            .iter()
            .all(|attempt| attempt.outcome() == AttemptOutcome::Transport)
    );
    assert!(attempts.iter().all(|attempt| attempt.status().is_none()));
}

#[test]
fn a_transport_failure_after_the_request_arrived_never_resends() {
    let listener = Listener::serving(vec![Canned::close_without_reply()]).unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let error = engine(&listener)
        .unwrap()
        .decide_with(&question, "Refund me.", CallOptions::new().attempts(true))
        .unwrap_err();
    let received = listener.requests();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0].body, BODY);
    let attempts = error.facts().unwrap().attempts().unwrap();
    assert_eq!(attempts.len(), 1);
    assert_eq!(attempts[0].outcome(), AttemptOutcome::Transport);
}

#[test]
fn a_global_timeout_during_connection_setup_retries_without_sending_http_bytes() {
    use std::io::Read as _;
    use std::time::Duration;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();
    let server = std::thread::spawn(move || {
        let until = std::time::Instant::now() + Duration::from_secs(2);
        let mut connections = Vec::new();
        while connections.len() < 2 && std::time::Instant::now() < until {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // Windows inherits the listener's nonblocking mode on accepted sockets.
                    stream.set_nonblocking(false).unwrap();
                    stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
                    let mut bytes = [0_u8; 4096];
                    let count = stream.read(&mut bytes).unwrap();
                    assert!(count > 0);
                    assert_eq!(bytes[0], 0x16, "only a TLS handshake reached the peer");
                    assert!(!bytes[..count].windows(4).any(|part| part == b"POST"));
                    connections.push(stream);
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(1))
                }
                Err(error) => panic!("accept failed: {error}"),
            }
        }
        std::thread::sleep(Duration::from_millis(80));
        connections.len()
    });
    let engine = Engine::builder()
        .base_url(&format!("https://{address}"))
        .unwrap()
        .api_key("fixture-native-attempts")
        .unwrap()
        .timeout(Duration::from_millis(40))
        .unwrap()
        .max_retries(0)
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let error = engine
        .decide_with(&question, "Refund me.", CallOptions::new().attempts(true))
        .unwrap_err();
    let connections = server.join().unwrap();
    assert_eq!(connections, 2);
    assert_eq!(error.facts().unwrap().attempts().unwrap().len(), 2);
    assert_eq!(error.to_string(), "the backend timed out");
}

#[test]
fn decisions_quota_keeps_the_native_status_without_claiming_a_rate_retry() {
    for message in [
        "You have no credits remaining",
        "fixture-native-attempts",
        "Rejected private evidence marker because it is unsupported",
        "echo(Zml4dHVyZS1uYXRpdmUtYXR0ZW1wdHM=)",
    ] {
        let body = serde_json::json!({"error":{"type":"request_error","code":"insufficient_quota","message":message}}).to_string();
        let listener = Listener::serving(vec![Canned::status(429, &body)]).unwrap();
        let engine = Engine::builder()
            .backend("openai")
            .unwrap()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("fixture-native-attempts")
            .unwrap()
            .max_retries(3)
            .no_cache()
            .build()
            .unwrap();
        let question = Question::decide("Refund?").unwrap().cut();
        let error = engine
            .decide_with(
                &question,
                "private evidence marker",
                CallOptions::new().attempts(true),
            )
            .unwrap_err();
        assert_eq!(error.kind(), thinkthen::ErrorKind::Backend);
        assert_eq!(error.stopped().cause(), thinkthen::StopCause::Status);
        assert_eq!(error.stopped().status(), Some(429));
        assert!(!error.stopped().retryable());
        assert!(!error.retryable());
        assert_eq!(error.facts().unwrap().requests_sent(), 1);
        assert_eq!(listener.requests().len(), 1);
        assert!(
            error
                .to_string()
                .starts_with("the backend openai account is out of credit")
        );
        assert_eq!(
            error.to_string().contains("provider message:"),
            message == "You have no credits remaining"
        );
        assert!(!format!("{error:?}").contains("fixture-native-attempts"));
        assert!(!format!("{error:?}").contains("private evidence marker"));
    }
}

#[test]
fn encoded_key_response_ids_are_omitted_and_ordinary_ids_survive_failures() {
    let normal = "15ca30aa-539c-4bad-a112-9912cbb2ce73";
    for (id, expected) in [("c2stdGVzdC0_", None), (normal, Some(normal))] {
        let listener = Listener::serving(vec![
            Canned::status(400, "{}").asking("x-typesafe-request-id", id),
        ])
        .unwrap();
        let engine = Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("sk-test-?")
            .unwrap()
            .max_retries(0)
            .no_cache()
            .build()
            .unwrap();
        let question = Question::decide("Refund?").unwrap().cut();
        let error = engine
            .decide_with(&question, "Refund me.", CallOptions::new().attempts(true))
            .unwrap_err();
        assert_eq!(listener.requests().len(), 1);
        let attempts = error.facts().unwrap().attempts().unwrap();
        assert_eq!(attempts.len(), 1);
        assert_eq!(attempts[0].status(), Some(400));
        assert_eq!(attempts[0].request_id(), expected);
        assert!(!format!("{error:?}").contains("sk-test-?"));
    }
}

#[test]
fn short_private_evidence_is_omitted_from_native_failure_text_and_debug() {
    let body = r#"{"error":{"message":"Rejected S7code! because it is unsupported"}}"#;
    let listener = Listener::serving(vec![Canned::status(400, body)]).unwrap();
    let engine = engine(&listener).unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let error = engine.decide(&question, "S7code!").unwrap_err();
    assert_eq!(error.kind(), thinkthen::ErrorKind::Backend);
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(error.to_string(), "the backend answered with status 400");
    assert!(!format!("{error:?}").contains("S7code!"));
}
