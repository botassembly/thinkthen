//! Public request-size planning and retry totals through counted local sends.
#![allow(
    clippy::expect_used,
    reason = "a failed outside-in fixture stops the proof"
)]

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use conformance_backend::{Canned, Listener};
use serde_json::Value;
use thinkthen::{AttemptOutcome, CallOptions, Engine, Entity, ErrorKind, Question, Relate};

fn relation_listener() -> Listener {
    Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request JSON");
        let answers = request
            .get("questions")
            .expect("relation questions")
            .as_object()
            .expect("relation questions")
            .keys()
            .map(|name| (name.clone(), serde_json::json!({"type":"noul","noul":0.9})))
            .collect::<serde_json::Map<_, _>>();
        let model = request.get("model").expect("request model");
        Canned::ok(&serde_json::json!({"model":model,"answers":answers,"usage":{"input_tokens":1,"output_tokens":1}}).to_string())
    })
    .expect("listener")
}

fn entities() -> Vec<Entity> {
    (0..18)
        .map(|index| Entity::new(&format!("item {index}"), "item").expect("entity"))
        .collect()
}

#[test]
fn relation_size_survives_a_model_override_and_a_smaller_profile_wins() {
    let listener = relation_listener();
    let ask = Relate::from_json(
        r#"{"version":1,"relate":{"relations":[{"name":"linked","source":"item","target":"item"}]},"model":"size-override"}"#,
    )
    .expect("relation");
    let build = || {
        Engine::builder()
            .base_url(listener.base())
            .expect("base")
            .api_key("sk-size-local")
            .expect("key")
            .no_cache()
    };

    let default = build()
        .prices_usd_per_million("0.25", "0.25")
        .expect("prices")
        .build()
        .expect("default engine");
    let priced = default.relate(&ask, entities()).expect("default relation");
    assert_eq!(priced.facts().estimated_cost_usd(), Some("0.000001"));
    let default_sends = listener.count();
    assert_eq!(default_sends, 1, "the default holds this fixed body");

    let smaller = build()
        .max_request_bytes(20_000)
        .expect("positive size")
        .build()
        .expect("smaller engine");
    smaller.relate(&ask, entities()).expect("smaller relation");
    let smaller_sends = listener.count() - default_sends;
    assert!(
        smaller_sends > default_sends,
        "the setter splits the relation"
    );

    let profiled = build()
        .max_request_bytes(20_000)
        .expect("positive size")
        .profile_json(r#"{"schema":"thinkthen.backend-profile/1","name":"smaller","max_request_bytes":10000}"#)
        .expect("smaller profile")
        .build()
        .expect("profiled engine");
    profiled
        .relate(&ask, entities())
        .expect("profiled relation");
    assert!(
        listener.count() - default_sends - smaller_sends > smaller_sends,
        "the profile lowers the setter's ceiling"
    );
}

#[test]
fn priced_relate_failure_keeps_complete_started_facts() {
    let reply = r#"{"model":"jev-1.13.0","answers":{"other":{"type":"noul","noul":0.9}},"usage":{"input_tokens":1,"output_tokens":1}}"#;
    let listener = Listener::answering(move |_| Canned::ok(reply)).expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-size-local")
        .expect("key")
        .prices_usd_per_million("0.25", "0.25")
        .expect("prices")
        .no_cache()
        .build()
        .expect("engine");
    let ask = Relate::from_json(r#"{"version":1,"relate":{"relations":[{"name":"linked","source":"person","target":"person"}]}}"#).expect("relation");
    let records = [
        Entity::new("Ada", "person").expect("entity"),
        Entity::new("Bea", "person").expect("entity"),
    ];
    let error = engine
        .relate(&ask, records)
        .expect_err("missing relation answer");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(
        error.facts().and_then(|facts| facts.estimated_cost_usd()),
        Some("0.000001")
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn retry_visibility_counts_live_attempts_and_no_replay_attempt() {
    const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let attempts = AtomicUsize::new(0);
    let retried = Listener::answering(move |_| {
        if attempts.fetch_add(1, Ordering::Relaxed) == 0 {
            Canned::status(503, "busy").asking("retry-after-ms", "0")
        } else {
            Canned::ok(ANSWER)
        }
    })
    .expect("retry listener");
    let question = Question::decide("Is this a refund?")
        .expect("question")
        .cut();
    let engine = Engine::builder()
        .base_url(retried.base())
        .expect("base")
        .api_key("sk-retry-local")
        .expect("key")
        .max_retries(1)
        .no_cache()
        .build()
        .expect("retry engine");
    engine
        .decide(&question, "refund now")
        .expect("retried answer");
    assert_eq!(
        (engine.usage().requests_sent(), engine.usage().retries()),
        (2, 1)
    );
    assert_eq!(retried.count(), 2);

    let normal = Listener::answering(|_| Canned::ok(ANSWER)).expect("normal listener");
    let folder = std::env::temp_dir().join(format!("thinkthen-size-retry-{}", std::process::id()));
    std::fs::create_dir_all(&folder).expect("recording folder");
    let recorded = Engine::builder()
        .base_url(normal.base())
        .expect("base")
        .api_key("sk-retry-local")
        .expect("key")
        .record(&folder)
        .expect("record")
        .build()
        .expect("recording engine");
    recorded
        .decide(&question, "a distinct refund")
        .expect("recorded answer");
    assert_eq!(
        (recorded.usage().requests_sent(), recorded.usage().retries()),
        (1, 0)
    );
    let replay = Engine::builder()
        .base_url(normal.base())
        .expect("base")
        .replay(&folder)
        .expect("replay")
        .build()
        .expect("replay engine");
    replay
        .decide(&question, "a distinct refund")
        .expect("replayed answer");
    assert_eq!(
        (replay.usage().requests_sent(), replay.usage().retries()),
        (0, 0)
    );
    assert_eq!(normal.count(), 1, "strict replay sends nothing");
    std::fs::remove_dir_all(folder).expect("remove test recording");
}

#[test]
fn retry_attempts_use_send_ordinals_and_reject_duplicate_headers() {
    const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let arrived = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| match arrived.fetch_add(1, Ordering::SeqCst) {
        0 => Canned::status(503, "busy")
            .asking("retry-after-ms", "0")
            .asking("x-envoy-upstream-service-time", "12")
            .asking("x-typesafe-request-id", "req-first"),
        1 => Canned::ok(ANSWER)
            .asking("x-envoy-upstream-service-time", "3")
            .asking("x-envoy-upstream-service-time", "4")
            .asking("x-typesafe-request-id", "req-safe")
            .asking("x-typesafe-request-id", "sk-attempt-local")
            .asking("x-forbidden", "forbidden-sentinel"),
        _ => Canned::ok(ANSWER),
    })
    .expect("loopback");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-attempt-local")
        .expect("key")
        .max_retries(1)
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("Is this a refund?")
        .expect("question")
        .cut();
    let caller = std::thread::current().id();
    let seen = Mutex::new(Vec::new());
    let observe = |event| {
        assert_eq!(
            std::thread::current().id(),
            caller,
            "caller-thread callback"
        );
        seen.lock().expect("events").push(event);
    };
    engine
        .decide_with(
            &question,
            "refund now",
            CallOptions::new().observe_attempt(&observe),
        )
        .expect("retried answer");
    assert_eq!(listener.count(), 2);
    let seen = seen.lock().expect("events");
    assert_eq!(seen.len(), 2);
    assert_eq!((seen[0].ordinal(), seen[1].ordinal()), (1, 2));
    assert_eq!(seen[0].request_sha256(), seen[1].request_sha256());
    assert_eq!(seen[0].request_sha256().len(), 64);
    assert!(seen.iter().all(|event| event.wall_ms() >= 1));
    assert_eq!(
        (
            seen[0].outcome(),
            seen[0].status(),
            seen[0].server_ms(),
            seen[0].request_id()
        ),
        (
            AttemptOutcome::Status,
            Some(503),
            Some(12),
            Some("req-first")
        )
    );
    assert_eq!(
        (
            seen[1].outcome(),
            seen[1].status(),
            seen[1].server_ms(),
            seen[1].request_id()
        ),
        (AttemptOutcome::Ok, Some(200), None, None)
    );
    let serialized = serde_json::to_string(&*seen).expect("owned events");
    assert!(!serialized.contains("forbidden-sentinel"));
    assert!(!serialized.contains("sk-attempt-local"));
    assert!(!format!("{seen:?}").contains("req-first"));
}

#[test]
fn separate_calls_reset_ordinals_and_screen_reflected_ids() {
    const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let arrived = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| match arrived.fetch_add(1, Ordering::SeqCst) {
        0 => Canned::ok(ANSWER).asking("x-typesafe-request-id", "prefix-sk-attempt-local-suffix"),
        1 => Canned::ok(ANSWER).asking("x-typesafe-request-id", "systemone"),
        2 => Canned::ok(ANSWER).asking("x-typesafe-request-id", "refund"),
        _ => Canned::ok(ANSWER)
            .asking("x-envoy-upstream-service-time", "-1")
            .asking("x-typesafe-request-id", "bad id"),
    })
    .expect("loopback");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-attempt-local")
        .expect("key")
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("Is this a refund?")
        .expect("question")
        .cut();
    let next = Mutex::new(Vec::new());
    let collect = |event| next.lock().expect("next events").push(event);
    engine
        .decide_with(
            &question,
            "refund now",
            CallOptions::new().observe_attempt(&collect),
        )
        .expect("next call");
    assert_eq!(listener.count(), 1);
    let seen_next = next.lock().expect("next events");
    assert_eq!(seen_next.len(), 1);
    assert_eq!(seen_next[0].ordinal(), 1, "each call has its own scope");
    assert_eq!(
        seen_next[0].request_id(),
        None,
        "active key echo is omitted"
    );
    drop(seen_next);
    for _ in 0..3 {
        engine
            .decide_with(
                &question,
                "refund now",
                CallOptions::new().observe_attempt(&collect),
            )
            .expect("screened response");
    }
    assert_eq!(listener.count(), 4);
    let next = next.lock().expect("screened events");
    assert_eq!(next.len(), 4);
    assert!(next.iter().all(|event| event.request_id().is_none()));
    assert_eq!(next[3].server_ms(), None);
}

#[test]
fn terminal_status_and_transport_failures_keep_one_truthful_attempt() {
    let question = Question::decide("Is this a refund?")
        .expect("question")
        .cut();
    for (name, reply, outcome, status) in [
        (
            "status",
            Canned::status(422, "refused").asking("x-envoy-upstream-service-time", "9"),
            AttemptOutcome::Status,
            Some(422),
        ),
        (
            "closed",
            Canned::close_without_reply(),
            AttemptOutcome::Transport,
            None,
        ),
        (
            "body",
            Canned::cut_short(),
            AttemptOutcome::Transport,
            Some(200),
        ),
    ] {
        let listener = Listener::serving(vec![reply]).expect("loopback");
        let engine = Engine::builder()
            .base_url(listener.base())
            .expect("base")
            .api_key("sk-attempt-failure-local")
            .expect("key")
            .max_retries(0)
            .no_cache()
            .build()
            .expect("engine");
        let seen = Mutex::new(Vec::new());
        let collect = |event| seen.lock().expect("attempts").push(event);
        let error = engine
            .decide_with(
                &question,
                "refund now",
                CallOptions::new().observe_attempt(&collect),
            )
            .expect_err("terminal failure");
        assert_eq!(
            listener.requests().len(),
            1,
            "{name}: one real arrival; {error}"
        );
        assert_eq!(
            error.facts().map(|facts| facts.requests_sent()),
            Some(1),
            "{name}"
        );
        let seen = seen.lock().expect("attempts");
        assert_eq!(seen.len(), 1, "{name}");
        assert_eq!(
            (seen[0].ordinal(), seen[0].outcome(), seen[0].status()),
            (1, outcome, status),
            "{name}"
        );
        assert!(seen[0].wall_ms() >= 1, "{name}");
        assert_eq!(
            seen[0].server_ms(),
            (name == "status").then_some(9),
            "{name}"
        );
    }
}

#[test]
fn a_cached_second_call_has_no_current_attempt() {
    const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let folder =
        std::env::temp_dir().join(format!("thinkthen-attempt-cache-{}", std::process::id()));
    let _gone = std::fs::remove_dir_all(&folder);
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-attempt-cache-local")
        .expect("key")
        .cache_at(&folder)
        .expect("cache")
        .build()
        .expect("engine");
    let question = Question::decide("Is this a refund?")
        .expect("question")
        .cut();
    let seen = Mutex::new(Vec::new());
    let collect = |event| seen.lock().expect("attempts").push(event);
    let first = engine
        .decide_with(
            &question,
            "refund now",
            CallOptions::new().observe_attempt(&collect),
        )
        .expect("live answer");
    assert_eq!(
        (
            listener.count(),
            first.facts().requests_sent(),
            seen.lock().expect("attempts").len()
        ),
        (1, 1, 1)
    );
    seen.lock().expect("attempts").clear();
    let second = engine
        .decide_with(
            &question,
            "refund now",
            CallOptions::new().observe_attempt(&collect),
        )
        .expect("cached answer");
    assert_eq!(
        (
            listener.count(),
            second.facts().requests_sent(),
            second.facts().cache_answers()
        ),
        (1, 0, 1)
    );
    assert!(
        seen.lock().expect("attempts").is_empty(),
        "cache hit has no current send"
    );
    std::fs::remove_dir_all(folder).expect("remove owned cache");
}
