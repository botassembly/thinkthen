//! Focused public boundary cases retained in the parent test target.

use super::*;

#[test]
fn caller_prices_round_the_combined_report_and_keep_no_send_zero() {
    let _serial = serial();
    let answer = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":1,"output_tokens":1}}"#;
    let listener = Listener::answering(move |_| Canned::ok(answer)).expect("listener");
    let folder = std::env::temp_dir().join(format!("thinkthen-priced-{}", std::process::id()));
    let _gone = std::fs::remove_dir_all(&folder);
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-controls")
        .expect("key")
        .cache_at(&folder)
        .expect("cache")
        .prices_usd_per_million("0.25", "0.25")
        .expect("prices")
        .build()
        .expect("engine");
    let asked = question();
    assert_eq!(
        engine.estimate_reported_cost(1, 1).as_deref(),
        Some("0.000001")
    );
    let live = engine.decide(&asked, "Refund me.").expect("live");
    assert_eq!(live.facts().estimated_cost_usd(), Some("0.000001"));
    assert!(!format!("{:?}", live.facts()).contains("0.000001"));
    assert_eq!(
        (live.facts().input_tokens(), live.facts().output_tokens()),
        (Some(1), Some(1))
    );
    let cached = engine.decide(&asked, "Refund me.").expect("cached");
    assert_eq!(cached.facts().estimated_cost_usd(), Some("0.000000"));
    assert_eq!(
        (
            cached.facts().requests_sent(),
            cached.facts().cache_answers()
        ),
        (0, 1)
    );
    assert_eq!(listener.count(), 1);
    let plain = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-controls")
        .expect("key")
        .no_cache()
        .build()
        .expect("plain");
    assert_eq!(plain.estimate_reported_cost(1, 1), None);
    assert_eq!(
        plain
            .decide(&asked, "Another.")
            .expect("plain answer")
            .facts()
            .estimated_cost_usd(),
        None
    );
    assert_eq!(listener.count(), 2);
    let zero = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-controls")
        .expect("key")
        .prices_usd_per_million("0", "0")
        .expect("zero prices")
        .no_cache()
        .build()
        .expect("zero engine");
    assert_eq!(
        zero.decide(&asked, "Zero.")
            .expect("zero price answer")
            .facts()
            .estimated_cost_usd(),
        Some("0.000000")
    );
    assert_eq!(listener.count(), 3);
    let _gone = std::fs::remove_dir_all(folder);
}

#[test]
fn malformed_caller_prices_refuse_before_send() {
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    for (input, output) in [
        ("-1", "0"),
        ("1e0", "0"),
        ("0", "1000000.000001"),
        ("0.0000001", "0"),
    ] {
        let result = Engine::builder()
            .base_url(listener.base())
            .expect("base")
            .prices_usd_per_million(input, output);
        assert_eq!(
            result.err().map(|error| error.kind()),
            Some(ErrorKind::Usage)
        );
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn priced_started_failure_facts_are_copied_while_retry_missing_usage_stays_unpriced() {
    let _serial = serial();
    let next = AtomicUsize::new(0);
    let valid = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":1,"output_tokens":1}}"#;
    let invalid = r#"{"model":"jev-latest","answers":{"other":{"type":"noul","noul":0.9}},"usage":{"input_tokens":1,"output_tokens":1}}"#;
    let listener = Listener::answering(move |_| match next.fetch_add(1, Ordering::SeqCst) {
        0 => Canned::status(503, "busy"),
        1 => Canned::ok(valid),
        _ => Canned::ok(invalid),
    })
    .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-controls")
        .expect("key")
        .prices_usd_per_million("1", "0")
        .expect("prices")
        .max_retries(1)
        .no_cache()
        .build()
        .expect("engine");
    let asked = question();
    let partial = engine.decide(&asked, "First.").expect("retried answer");
    assert_eq!(
        (
            partial.facts().requests_sent(),
            partial.facts().input_tokens()
        ),
        (2, Some(1))
    );
    assert_eq!(partial.facts().estimated_cost_usd(), None);
    let failure = engine
        .decide(&asked, "Second.")
        .expect_err("missing answer");
    assert_eq!(failure.kind(), ErrorKind::Backend);
    let copied = failure.facts().expect("started facts").clone();
    drop(engine);
    assert_eq!(listener.count(), 3);
    assert_eq!(copied.estimated_cost_usd(), Some("0.000001"));
    assert_eq!(partial.facts().estimated_cost_usd(), None);
}

#[test]
fn complete_batch_replies_with_overflowed_tokens_omit_call_cost_and_next_call_recovers() {
    let _serial = serial();
    let next = AtomicUsize::new(0);
    let big = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":18446744073709551615,"output_tokens":0}}"#;
    let small = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":1,"output_tokens":0}}"#;
    let listener = Listener::answering(move |_| {
        if next.fetch_add(1, Ordering::SeqCst) == 0 {
            Canned::ok(big)
        } else {
            Canned::ok(small)
        }
    })
    .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-controls")
        .expect("key")
        .prices_usd_per_million("1", "0")
        .expect("prices")
        .no_cache()
        .build()
        .expect("engine");
    let rows = vec!["First.".to_owned(), "Second.".to_owned()];
    let setting = BatchSetting::Records(std::num::NonZeroUsize::new(1).expect("one"));
    let call = engine
        .details_many_recoverable_with(&question(), &rows, CallOptions::new().batch(setting))
        .expect("two complete replies");
    assert_eq!(call.facts().requests_sent(), 2);
    assert_eq!(call.facts().input_tokens(), None);
    assert_eq!(call.facts().estimated_cost_usd(), None);
    assert_eq!(listener.count(), 2);
    let next_call = engine.decide(&question(), "Third.").expect("next call");
    assert_eq!(next_call.facts().estimated_cost_usd(), Some("0.000001"));
    assert_eq!(listener.count(), 3);
}

#[test]
fn counters_and_cache_answers_match_the_real_attempts() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let folder = std::env::temp_dir().join(format!("thinkthen-counters-{}", std::process::id()));
    let _gone = std::fs::remove_dir_all(&folder);
    let engine = Engine::builder()
        .base_url(listener.base())
        .and_then(|b| b.api_key("sk-public-controls"))
        .and_then(|b| b.cache_at(&folder))
        .and_then(thinkthen::EngineBuilder::build)
        .expect("engine");
    let asked = question();
    let sent = engine.decide(&asked, "Refund me.").expect("live answer");
    assert_eq!(*sent.value(), Answer::Yes);
    assert_eq!(
        (sent.facts().records(), sent.facts().requests_sent()),
        (1, 1)
    );
    assert_eq!(
        (sent.facts().input_tokens(), sent.facts().output_tokens()),
        (Some(3), Some(1))
    );
    let cached = engine.decide(&asked, "Refund me.").expect("cached answer");
    assert_eq!(*cached.value(), Answer::Yes);
    assert_eq!(
        (
            cached.facts().records(),
            cached.facts().requests_sent(),
            cached.facts().cache_answers()
        ),
        (1, 0, 1)
    );
    assert_eq!(cached.facts().input_tokens(), None);
    let token = CancelToken::new();
    token.cancel();
    let options = CallOptions::new().cancel(&token);
    assert!(engine.decide_with(&asked, "Another.", options).is_err());
    let usage = engine.usage();
    assert_eq!(listener.count(), 1);
    assert_eq!((usage.requests_sent(), usage.cache_answers()), (1, 1));
    let _gone = std::fs::remove_dir_all(&folder);
}

#[test]
fn each_public_question_model_selects_its_own_cache_freshness() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let folder = std::env::temp_dir().join(format!(
        "thinkthen-public-model-refresh-{}",
        std::process::id()
    ));
    let _gone = std::fs::remove_dir_all(&folder);
    let build = |model: &str| {
        Engine::builder()
            .base_url(listener.base())
            .and_then(|builder| builder.api_key("sk-public-controls"))
            .and_then(|builder| builder.model(model))
            .and_then(|builder| builder.cache_at(&folder))
            .and_then(thinkthen::EngineBuilder::build)
            .expect("engine")
    };
    let alias_question = Question::decide("Does this ask for a refund?")
        .expect("question")
        .model("jev-latest")
        .expect("alias")
        .cut();
    let pinned_question = Question::decide("Does this ask for a refund?")
        .expect("question")
        .model("jev-1.13.0")
        .expect("pin")
        .cut();
    let pinned_engine = build("jev-1.13.0");
    let first = pinned_engine
        .decide(&alias_question, "Refund me.")
        .expect("first alias answer");
    let second = pinned_engine
        .decide(&alias_question, "Refund me.")
        .expect("refreshed alias answer");
    assert_eq!(first.facts().requests_sent(), 1);
    assert_eq!(second.facts().requests_sent(), 1);
    let alias_engine = build("jev-latest");
    let third = alias_engine
        .decide(&pinned_question, "Refund me.")
        .expect("first pinned answer");
    let fourth = alias_engine
        .decide(&pinned_question, "Refund me.")
        .expect("cached pinned answer");
    assert_eq!(third.facts().requests_sent(), 1);
    assert_eq!(fourth.facts().requests_sent(), 0);
    assert_eq!(fourth.facts().cache_answers(), 1);
    assert_eq!(listener.count(), 3);
    let _gone = std::fs::remove_dir_all(folder);
}

/// Ticket 0132: a reply over 1 MiB plus 8 bytes per request byte is refused by name, once.
#[test]
fn a_reply_over_its_limit_names_it() {
    let _serial = serial();
    let listener = Listener::answering(|body| {
        let size = 1_048_576 + 8 * body.len() + 1;
        let (head, tail) = DECIDED.split_at(DECIDED.len() - 1);
        Canned::ok(&format!("{head}{}{tail}", " ".repeat(size - DECIDED.len())))
    })
    .expect("listener");
    let result = engine(listener.base()).decide(&question(), "Refund me.");
    let limit = 1_048_576 + 8 * listener.requests()[0].body.len();
    assert_eq!(kind(&result), Some(ErrorKind::Backend));
    assert_eq!(result.as_ref().err().map(Error::retryable), Some(false));
    assert_eq!(
        message(result),
        format!(
            "the backend's reply passed this request's limit of {limit} bytes, so the answer was not kept; the request was not sent again"
        )
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn observer_panic_waits_for_a_held_later_batch_worker() {
    let _serial = serial();
    let first = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let later = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let held_first = std::sync::Arc::clone(&first);
    let held_later = std::sync::Arc::clone(&later);
    let pair = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::answering(move |body| {
        let body = String::from_utf8_lossy(body);
        if body.contains("gamma") {
            Canned::ok(DECIDED)
        } else if body.contains("beta") {
            Canned::ok(pair).after_release(std::sync::Arc::clone(&held_later))
        } else {
            Canned::ok(pair).after_release(std::sync::Arc::clone(&held_first))
        }
    })
    .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-controls")
        .expect("key")
        .throttle(2)
        .expect("throttle")
        .no_cache()
        .build()
        .expect("engine");
    let (observed, notice) = std::sync::mpsc::channel();
    let observe = |_: thinkthen::RecordObservation<'_>| {
        observed.send(()).expect("notice");
        resume_unwind(Box::new("held observer payload"));
    };
    let setting = BatchSetting::Records(std::num::NonZeroUsize::new(2).expect("two"));
    let asked = question();
    let mut rows = engine.decide_many_with(
        &asked,
        ["alpha", "first", "beta", "second"],
        CallOptions::new().batch(setting).observe(&observe),
    );
    let later_released = AtomicUsize::new(0);
    let (caught, held_worker) = thread::scope(|scope| {
        let listener = &listener;
        let released = &later_released;
        let helper = scope.spawn(move || {
            let began = Instant::now();
            while listener.count() < 2 && began.elapsed() < BOUND {
                thread::sleep(Duration::from_millis(5));
            }
            let held_worker = listener.count() == 2;
            assert!(first.wait(), "release first packed answer");
            notice.recv_timeout(BOUND).expect("observer fired");
            if held_worker {
                assert!(later.wait(), "release later packed answer");
                released.store(1, Ordering::SeqCst);
            }
            held_worker
        });
        let caught = catch_unwind(AssertUnwindSafe(|| rows.next()));
        let returned_after_release = later_released.load(Ordering::SeqCst) == 1;
        let held_worker = helper.join().expect("release helper");
        assert!(
            returned_after_release,
            "observer payload waited for the later reply"
        );
        (caught, held_worker)
    });
    assert!(held_worker, "a later request was in flight at the callback");
    let payload = caught.expect_err("observer panic reaches caller after join");
    assert_eq!(
        payload.downcast_ref::<&str>(),
        Some(&"held observer payload")
    );
    assert_eq!(listener.count(), 2);
    assert_eq!(
        *engine
            .decide(&question(), "gamma")
            .expect("next call")
            .value(),
        Answer::Yes
    );
}

#[test]
fn a_failed_eager_rank_carries_its_started_call_facts() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::status(503, "busy")).expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-controls")
        .expect("key")
        .max_retries(0)
        .no_cache()
        .build()
        .expect("engine");
    let asked = Question::rank("Which asks for a refund?").expect("rank");
    let error = engine
        .rank(&asked, ["alpha", "beta"])
        .expect_err("backend refusal");
    assert_eq!(error.kind(), ErrorKind::Backend);
    let facts = error.facts().expect("started eager facts");
    assert_eq!(
        (facts.records(), facts.requests_sent(), facts.input_tokens()),
        (0, 1, None)
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn ineligible_calls_refuse_shared_context_before_a_send() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let one = question();
    let find = Question::find("Which asks for a refund?").expect("find");
    let recognize = thinkthen::Recognize::builder()
        .kind(thinkthen::Kind::new("person", None).expect("kind"))
        .and_then(thinkthen::RecognizeBuilder::build)
        .expect("recognize");
    let relate = Relate::builder()
        .relation(
            thinkthen::RelationRule::one_way("works_with", "person", "organization").expect("rule"),
        )
        .and_then(thinkthen::RelateBuilder::build)
        .expect("relate");
    let options = CallOptions::new().context("shared evidence");
    let denied = [
        (
            "decide",
            "a single-document call does not take a shared context",
            engine.decide_with(&one, "Ada", options).map(|_| ()),
        ),
        (
            "details",
            "a single-document call does not take a shared context",
            engine.details_with(&one, "Ada", options).map(|_| ()),
        ),
        (
            "find",
            "find does not take a shared context",
            engine
                .find_with(&find, ["Ada", "Acme"], options)
                .map(|_| ()),
        ),
        (
            "recognize",
            "recognize does not take a shared context",
            engine
                .recognize_with(&recognize, "Ada", options)
                .map(|_| ()),
        ),
        (
            "relate",
            "relate does not take a shared context",
            engine
                .relate_with(
                    &relate,
                    [
                        Entity::new("Ada", "person").expect("entity"),
                        Entity::new("Acme", "organization").expect("entity"),
                    ],
                    options,
                )
                .map(|_| ()),
        ),
    ];
    for (name, sentence, result) in denied {
        let error = result.expect_err(name);
        assert_eq!(error.kind(), ErrorKind::Usage, "{name}");
        assert_eq!(error.to_string(), sentence, "{name}");
        assert!(error.facts().is_none(), "{name} never started");
    }
    assert_eq!(listener.count(), 0);
}
