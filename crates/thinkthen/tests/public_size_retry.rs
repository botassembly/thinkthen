//! Public request-size planning and retry totals through counted local sends.
#![allow(
    clippy::expect_used,
    reason = "a failed outside-in fixture stops the proof"
)]

use std::sync::atomic::{AtomicUsize, Ordering};

use conformance_backend::{Canned, Listener};
use serde_json::Value;
use thinkthen::{Engine, Entity, ErrorKind, Question, Relate};

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
