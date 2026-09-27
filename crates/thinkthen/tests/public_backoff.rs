//! Public calls share one address gate while cache hits and other addresses proceed.
#![allow(
    clippy::expect_used,
    reason = "a failed loopback fixture stops the proof"
)]

use std::fs;
use std::time::Duration;

use conformance_backend::{Canned, Listener};
use thinkthen::{Answer, CallOptions, Engine, EngineBuilder, ErrorKind, Question};

const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":1,"output_tokens":1}}"#;

fn engine(base: &str) -> EngineBuilder {
    Engine::builder()
        .base_url(base)
        .expect("loopback address")
        .api_key("sk-test")
        .expect("fake key")
        .max_retries(0)
}

fn quick_call() -> CallOptions<'static> {
    CallOptions::new()
        .deadline_after(Duration::from_millis(100))
        .expect("short deadline")
}

#[test]
fn a_spent_request_closes_the_address_for_other_engines_but_not_cache_or_another_address() {
    let a = Listener::answering(|body| {
        if body
            .windows(b"overload".len())
            .any(|part| part == b"overload")
        {
            Canned::status(503, "busy").asking("retry-after-ms", "500")
        } else {
            Canned::ok(ANSWER)
        }
    })
    .expect("listener A");
    let b = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener B");
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("public-backoff-cache");
    let _old = fs::remove_dir_all(&folder);
    let cached = engine(a.base())
        .cache_at(&folder)
        .expect("cache path")
        .build()
        .expect("cached engine");
    let live = engine(a.base()).no_cache().build().expect("live engine");
    let other = engine(b.base()).no_cache().build().expect("other engine");
    let question = Question::decide("Is it?").expect("question").cut();

    assert_eq!(
        cached.decide(&question, "cached").expect("warm answer"),
        Answer::Yes
    );
    let overloaded = live.decide(&question, "overload").expect_err("503");
    assert_eq!(overloaded.kind(), ErrorKind::Backend);
    assert_eq!(a.count(), 2);

    assert_eq!(
        cached
            .decide_with(&question, "cached", quick_call())
            .expect("cache hit"),
        Answer::Yes
    );
    assert_eq!(a.count(), 2, "cache sent nothing through the closed gate");
    assert_eq!(
        other
            .decide_with(&question, "elsewhere", quick_call())
            .expect("other address"),
        Answer::Yes
    );
    assert_eq!(b.count(), 1);

    let waiting = cached
        .decide_with(&question, "after", quick_call())
        .expect_err("same-address gate outlives the first engine");
    assert_eq!(waiting.kind(), ErrorKind::Deadline);
    assert_eq!(a.count(), 2, "a waiting request sent nothing");
}

#[test]
fn a_zero_retry_after_waits_instead_of_hammering_the_backend() {
    let listener =
        Listener::answering(|_| Canned::status(503, "busy").asking("retry-after-ms", "0"))
            .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("loopback address")
        .api_key("sk-test")
        .expect("fake key")
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("Is it?").expect("question").cut();
    let stopped = engine
        .decide_with(&question, "evidence", quick_call())
        .expect_err("the call deadline ends the header floor");
    assert_eq!(stopped.kind(), ErrorKind::Deadline);
    assert_eq!(listener.count(), 1, "zero did not prompt a second send");
}

#[test]
fn a_redirect_names_the_refusal_and_never_sends_the_key_to_the_next_host() {
    let elsewhere = Listener::serving(vec![Canned::ok(ANSWER)]).expect("other listener");
    let listener = Listener::serving(vec![Canned::redirect(elsewhere.url())]).expect("listener");
    let engine = engine(listener.base()).no_cache().build().expect("engine");
    let question = Question::decide("Is it?").expect("question").cut();
    let error = engine
        .decide(&question, "evidence")
        .expect_err("redirect refused");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(
        error.to_string(),
        "the backend answered with status 302: the redirect was not followed"
    );
    assert_eq!(listener.requests().len(), 1);
    assert!(elsewhere.requests().is_empty());
}
