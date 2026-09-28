//! Focused public boundary cases retained in the parent test target.

use super::*;

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
    let release = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let held = std::sync::Arc::clone(&release);
    let listener = Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains("beta") {
            Canned::ok(DECIDED).after_release(std::sync::Arc::clone(&held))
        } else {
            Canned::ok(DECIDED)
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
    let setting = BatchSetting::Records(std::num::NonZeroUsize::MIN);
    let asked = question();
    let mut rows = engine.decide_many_with(
        &asked,
        ["alpha", "beta"],
        CallOptions::new().batch(setting).observe(&observe),
    );
    let (caught, held_worker) = thread::scope(|scope| {
        let listener = &listener;
        let release = &release;
        let helper = scope.spawn(move || {
            notice.recv_timeout(BOUND).expect("observer fired");
            let began = Instant::now();
            while listener.count() < 2 && began.elapsed() < BOUND {
                thread::sleep(Duration::from_millis(5));
            }
            let held_worker = listener.count() == 2;
            if held_worker {
                assert!(release.wait(), "release the held answer");
            }
            held_worker
        });
        let caught = catch_unwind(AssertUnwindSafe(|| rows.next()));
        (caught, helper.join().expect("release helper"))
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
