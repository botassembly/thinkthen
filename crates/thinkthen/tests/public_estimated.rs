//! Final encoded-body admission, measured at the listener in one process.
#![allow(
    clippy::expect_used,
    reason = "a failed loopback fixture stops this proof"
)]

use std::num::NonZeroUsize;
use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use conformance_backend::{Canned, Listener, Rendezvous};
use thinkthen::{BatchSetting, CallOptions, Engine, ErrorKind, EstimatedInputDenial, Question};

#[path = "public_controls/alone.rs"]
mod alone;
#[path = "../src/test_deadline/child.rs"]
mod child;
use child::wait;

const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":312,"output_tokens":48000}}"#;
const NO_USAGE: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#;

#[test]
fn shared_state_questions_raise_only_the_selected_route_estimate_and_refuse_before_sending() {
    alone::alone(
        "shared_state_questions_raise_only_the_selected_route_estimate_and_refuse_before_sending",
        shared_state_case,
    );
}

fn shared_state_case() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"same-model","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1},"q3":{"type":"noul","noul":0.2}}}"#)).expect("listener");
    let question = Question::tag_labels("Which topics apply?")
        .and_then(|labels| labels.label("delivery", None))
        .and_then(|labels| labels.label("refund", None))
        .and_then(|labels| labels.label("support", None))
        .and_then(thinkthen::LabelBuilder::build)
        .expect("labels");
    let evidence = "A public synthetic parcel report. ".repeat(40);
    let build = |name: &str, cap| route_engine(name, listener.base(), cap);
    let ordinary = build("typesafe", None)
        .plan_with(&question, ["parcel"], CallOptions::new().context(&evidence))
        .expect("ordinary plan");
    let revised = build("perplexity", None)
        .plan_with(&question, ["parcel"], CallOptions::new().context(&evidence))
        .expect("revised plan");
    assert_eq!(
        ordinary.first_body(),
        revised.first_body(),
        "accounting preserves wire identity"
    );
    let body = revised.first_body().expect("body");
    let packed: serde_json::Value = serde_json::from_slice(body).expect("packed JSON");
    assert_eq!(
        packed
            .get("questions")
            .expect("questions member")
            .as_object()
            .expect("questions")
            .len(),
        3
    );
    let state_bytes = serde_json::to_string(packed.get("state").expect("state member"))
        .expect("state JSON")
        .len();
    let accounting_bytes = body.len() + state_bytes * 2;
    let revised_high = (accounting_bytes as u64 * 908).div_ceil(1000);
    assert_eq!(
        ordinary.estimated_input_tokens().1 as u64,
        (body.len() as u64 * 908).div_ceil(1000)
    );
    assert_eq!(revised.estimated_input_tokens().1 as u64, revised_high);
    assert_eq!(
        revised.estimated_bytes(),
        body.len(),
        "bytes describe the sent body"
    );
    assert_request_counts(&revised, body.len(), revised_high);
    let cap = ordinary.estimated_input_tokens().1 as u64 + 1;
    assert!(cap < revised_high);
    let denial = build("perplexity", Some(cap))
        .tag_many_complete_with(&question, ["parcel"], CallOptions::new().context(&evidence))
        .expect_err("repeated-state estimate exceeds cap");
    assert_eq!(
        denial.estimated_input_denial(),
        Some(EstimatedInputDenial::InitialRequest { limit: cap })
    );
    assert_eq!(listener.count(), 0, "revised admission sends nothing");
    let ordinary_call = build("typesafe", Some(cap))
        .tag_many_complete_with(&question, ["parcel"], CallOptions::new().context(&evidence))
        .expect("ordinary estimate remains admitted");
    assert_eq!(listener.count(), 1);
    assert_request_counts(
        ordinary_call.facts(),
        body.len(),
        ordinary.estimated_input_tokens().1 as u64,
    );
    let tally = thinkthen::Tally::new();
    let revised_call = tally
        .run(|| {
            build("perplexity", Some(revised_high + cap)).tag_many_complete_with(
                &question,
                ["parcel"],
                CallOptions::new().context(&evidence),
            )
        })
        .expect("revised estimate admits a raised cap");
    assert_request_counts(revised_call.facts(), body.len(), revised_high);
    assert_request_counts(&tally.facts(), body.len(), revised_high);
    assert_eq!(listener.count(), 2);
}

fn route_engine(name: &str, base: &str, cap: Option<u64>) -> Engine {
    Engine::builder()
        .backend(name)
        .expect("backend")
        .base_url(base)
        .expect("base")
        .model("same-model")
        .expect("model")
        .api_key("sk-test")
        .expect("fake key")
        .no_cache()
        .max_retries(0)
        .max_estimated_input_tokens_total(cap)
        .build()
        .expect("engine")
}

fn assert_request_counts(counts: &impl serde::Serialize, bytes: usize, tokens: u64) {
    let counts = serde_json::to_value(counts).expect("request counts");
    assert_eq!(
        counts
            .get("largest_request_bytes")
            .and_then(serde_json::Value::as_u64),
        Some(bytes as u64)
    );
    assert_eq!(
        counts
            .get("largest_request_estimated_input_tokens")
            .and_then(serde_json::Value::as_u64),
        Some(tokens)
    );
}

#[test]
fn final_body_charge_selects_each_limit_and_distinguishes_later_and_retry_denials() {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let build = |limit| {
        Engine::builder()
            .base_url(listener.base())
            .expect("base")
            .api_key("sk-test")
            .expect("fake key")
            .no_cache()
            .max_retries(0)
            .max_estimated_input_tokens_total(limit)
            .build()
            .expect("engine")
    };
    let first_call = build(None)
        .decide(&question, "item one")
        .expect("unbounded admission counts");
    let bodies = listener.requests();
    let first = bodies.first().expect("one captured body").body.len() as u64;
    let charge = (first * 908).div_ceil(1000);
    assert!(
        charge < 312,
        "reported input can exceed the admission estimate"
    );
    assert_eq!(first_call.facts().input_tokens(), Some(312));
    assert_eq!(first_call.facts().output_tokens(), Some(48000));
    assert_eq!(listener.count(), 1);
    let limited = build(Some(charge * 2));
    limited
        .decide(&question, "item two")
        .expect("raised limit admits one");
    let bodies = listener.requests();
    assert_eq!(
        bodies.first().expect("captured body").body.len() as u64,
        first,
        "same-length independent body"
    );
    let denied = limited
        .decide(&question, "item tri")
        .expect_err("spent before first send");
    assert_eq!(denied.kind(), ErrorKind::Usage);
    assert_eq!(
        denied.estimated_input_denial(),
        Some(EstimatedInputDenial::InitialRequest { limit: charge * 2 })
    );
    assert_eq!(
        denied.detail().message(),
        format!(
            "max_estimated_input_tokens_total={} (encoded-body-bytes-908-v1) would be exceeded before this call's first request",
            charge * 2
        )
    );
    assert_eq!(listener.count(), 2, "denial sent nothing");

    let batch_engine = build(Some(charge * 3));
    let later = batch_engine
        .decide_many_with(
            &question,
            ["item red", "item blu"],
            CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::MIN)),
        )
        .collect::<Result<Vec<_>, _>>()
        .expect_err("second request exceeds limit");
    assert_eq!(
        later.estimated_input_denial(),
        Some(EstimatedInputDenial::AdditionalRequest { limit: charge * 3 })
    );
    assert_eq!(listener.count(), 3, "one batch body arrived");
    assert_eq!(
        listener
            .requests()
            .first()
            .expect("captured body")
            .body
            .len() as u64,
        first
    );
    assert!(later.facts().is_some(), "started call retains facts");
    assert!(
        !format!("{later:?}").contains("item blu"),
        "private input withheld"
    );

    retry_refusal(charge, first, &question);
    held_race(charge, first, &question);
    cached_answer(charge, first, &question);
    replayed_answer(charge, first, &question);
    mixed_settings(charge, first, &question);
    absent_usage_still_charges(charge, first, &question);
}

fn retry_refusal(charge: u64, first: u64, question: &Question) {
    let retry_listener =
        Listener::answering(|_| Canned::status(503, "busy").asking("retry-after-ms", "0"))
            .expect("retry listener");
    let retry = Engine::builder()
        .base_url(retry_listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("fake key")
        .no_cache()
        .max_retries(1)
        .max_estimated_input_tokens_total(Some(charge * 4))
        .build()
        .expect("retry engine");
    let refused = retry
        .decide(question, "item six")
        .expect_err("same exchange retry refused");
    assert_eq!(retry_listener.count(), 1);
    assert_eq!(
        retry_listener
            .requests()
            .first()
            .expect("captured body")
            .body
            .len() as u64,
        first
    );
    assert_eq!(
        refused.estimated_input_denial(),
        Some(EstimatedInputDenial::Retry {
            limit: charge * 4,
            last_status: 503
        })
    );
    assert!(
        refused.facts().is_some(),
        "failed started attempt retains facts"
    );
}

fn held_race(charge: u64, first: u64, question: &Question) {
    let gate = Arc::new(Rendezvous::new(2));
    let listeners: Vec<_> = (0..2)
        .map(|_| {
            let held = Arc::clone(&gate);
            Listener::answering(move |_| Canned::ok(ANSWER).after_release(Arc::clone(&held)))
                .expect("contender listener")
        })
        .collect();
    let build = |base: &str| {
        Engine::builder()
            .base_url(base)
            .expect("base")
            .api_key("sk-test")
            .expect("fake key")
            .no_cache()
            .max_retries(0)
            .max_estimated_input_tokens_total(Some(charge * 5))
            .build()
            .expect("engine")
    };
    let engines: Vec<_> = listeners
        .iter()
        .map(|listener| build(listener.base()))
        .collect();
    let start = Barrier::new(3);
    let (send, receive) = mpsc::channel();
    thread::scope(|scope| {
        for (index, (engine, evidence)) in engines.iter().zip(["item ten", "item six"]).enumerate()
        {
            let send = send.clone();
            let start = &start;
            scope.spawn(move || {
                start.wait();
                send.send((index, engine.decide(question, evidence)))
                    .expect("contender result");
            });
        }
        start.wait();
        let until = Instant::now() + Duration::from_secs(30);
        while listeners.iter().map(Listener::count).sum::<usize>() == 0 {
            assert!(
                Instant::now() < until,
                "one admitted contender never arrived"
            );
            thread::sleep(Duration::from_millis(5));
        }
        let (loser, refusal) = receive
            .recv_timeout(Duration::from_secs(30))
            .expect("other contender reaches admission");
        let refusal = refusal.expect_err("one of two contenders loses the only charge");
        assert_eq!(
            refusal.estimated_input_denial(),
            Some(EstimatedInputDenial::InitialRequest { limit: charge * 5 })
        );
        assert_eq!(listeners.get(loser).expect("contender index").count(), 0);
        assert_eq!(listeners.iter().map(Listener::count).sum::<usize>(), 1);
        gate.wait();
        receive
            .recv_timeout(Duration::from_secs(30))
            .expect("held winner completed")
            .1
            .expect("held answer");
    });
    assert_eq!(
        listeners
            .iter()
            .flat_map(Listener::requests)
            .next()
            .expect("captured held body")
            .body
            .len() as u64,
        first
    );
}

fn cached_answer(charge: u64, first: u64, question: &Question) {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("cache listener");
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("estimated-cache-{}", std::process::id()));
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("fake key")
        .cache_at(&folder)
        .expect("cache folder")
        .max_retries(0)
        .max_estimated_input_tokens_total(Some(charge * 6))
        .build()
        .expect("engine");
    engine
        .decide(question, "item one")
        .expect("sixth charge admitted");
    assert_eq!(listener.count(), 1);
    assert_eq!(
        listener
            .requests()
            .first()
            .expect("captured body")
            .body
            .len() as u64,
        first
    );
    engine
        .decide(question, "item one")
        .expect("identical cached answer at spent cap");
    assert_eq!(
        listener.count(),
        1,
        "cache hit adds no live attempt or charge"
    );
    let refusal = engine
        .decide(question, "item two")
        .expect_err("different body refused");
    assert_eq!(
        refusal.estimated_input_denial(),
        Some(EstimatedInputDenial::InitialRequest { limit: charge * 6 })
    );
    assert_eq!(listener.count(), 1);
}

fn replayed_answer(charge: u64, first: u64, question: &Question) {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("record listener");
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("estimated-replay-{}", std::process::id()));
    let base = || {
        Engine::builder()
            .base_url(listener.base())
            .expect("base")
            .api_key("sk-test")
            .expect("fake key")
            .no_cache()
            .max_retries(0)
            .max_estimated_input_tokens_total(Some(charge * 7))
    };
    base()
        .record(&folder)
        .expect("record path")
        .build()
        .expect("record engine")
        .decide(question, "item one")
        .expect("seventh charge admitted");
    assert_eq!(listener.count(), 1);
    assert_eq!(
        listener
            .requests()
            .first()
            .expect("recorded body")
            .body
            .len() as u64,
        first
    );
    base()
        .replay(&folder)
        .expect("replay path")
        .build()
        .expect("replay engine")
        .decide(question, "item one")
        .expect("replay at spent total");
    assert_eq!(
        listener.count(),
        1,
        "replay adds no live attempt or estimate"
    );
}

fn mixed_settings(charge: u64, first: u64, question: &Question) {
    let gate = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&gate);
    let a_listener =
        Listener::answering(move |_| Canned::ok(ANSWER).after_release(Arc::clone(&held)))
            .expect("A listener");
    let b_listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("B listener");
    let build = |base: &str, limit| {
        Engine::builder()
            .base_url(base)
            .expect("base")
            .api_key("sk-test")
            .expect("fake key")
            .no_cache()
            .max_retries(0)
            .max_estimated_input_tokens_total(limit)
            .build()
            .expect("engine")
    };
    let a = build(a_listener.base(), Some(charge * 8));
    let b = build(b_listener.base(), Some(charge * 9));
    thread::scope(|scope| {
        let worker = scope.spawn(|| a.decide(question, "item one"));
        let until = Instant::now() + Duration::from_secs(30);
        while a_listener.count() == 0 {
            assert!(Instant::now() < until, "A did not arrive");
            thread::sleep(Duration::from_millis(5));
        }
        b.decide(question, "item two")
            .expect("B's larger limit admits while A is held");
        assert_eq!(b_listener.count(), 1);
        let refused = a
            .decide(question, "item tri")
            .expect_err("A's selected limit is spent");
        assert_eq!(
            refused.estimated_input_denial(),
            Some(EstimatedInputDenial::InitialRequest { limit: charge * 8 })
        );
        assert_eq!(a_listener.count(), 1);
        gate.wait();
        worker.join().expect("A worker").expect("A answer");
    });
    for listener in [&a_listener, &b_listener] {
        assert_eq!(
            listener
                .requests()
                .first()
                .expect("captured body")
                .body
                .len() as u64,
            first
        );
    }
    let cases = [
        (None, true, 2),
        (Some(charge * 9), false, 2),
        (Some(charge * 11), true, 3),
        (Some(charge * 10), false, 3),
    ];
    for (limit, admitted, arrivals) in cases {
        let result = build(b_listener.base(), limit).decide(question, "item one");
        if admitted {
            result.expect("unset or raised setting admits against retained count");
        } else {
            let refused = result.expect_err("lower or reapplied finite setting refuses");
            assert_eq!(
                refused.estimated_input_denial(),
                Some(EstimatedInputDenial::InitialRequest {
                    limit: limit.expect("finite limit")
                })
            );
        }
        assert_eq!(b_listener.count(), arrivals);
    }
    assert!(
        b_listener
            .requests()
            .iter()
            .all(|body| body.body.len() as u64 == first)
    );
}

fn absent_usage_still_charges(charge: u64, first: u64, question: &Question) {
    for (number, success) in [(12, true), (13, false)] {
        let listener = Listener::answering(move |_| {
            if success {
                Canned::ok(NO_USAGE)
            } else {
                Canned::status(422, "bad request")
            }
        })
        .expect("listener");
        let limit = charge * number;
        let engine = Engine::builder()
            .base_url(listener.base())
            .expect("base")
            .api_key("sk-test")
            .expect("fake key")
            .no_cache()
            .max_retries(0)
            .max_estimated_input_tokens_total(Some(limit))
            .build()
            .expect("engine");
        if success {
            let call = engine
                .decide(question, "item one")
                .expect("answer without usage");
            assert_eq!(call.facts().requests_sent(), 1);
            assert_eq!(call.facts().input_tokens(), None);
            assert_eq!(call.facts().output_tokens(), None);
        } else {
            let failure = engine
                .decide(question, "item one")
                .expect_err("terminal failure");
            assert_eq!(failure.facts().expect("started facts").requests_sent(), 1);
            assert_eq!(failure.facts().expect("started facts").input_tokens(), None);
            assert_eq!(
                failure.facts().expect("started facts").output_tokens(),
                None
            );
        }
        assert_eq!(listener.count(), 1);
        assert_eq!(
            listener.requests().first().expect("actual body").body.len() as u64,
            first
        );
        let denied = engine
            .decide(question, "item two")
            .expect_err("started request kept its charge");
        assert_eq!(
            denied.estimated_input_denial(),
            Some(EstimatedInputDenial::InitialRequest { limit })
        );
        assert_eq!(listener.count(), 1, "denial sent nothing");
    }
}
