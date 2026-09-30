//! Bounded public attempt observation and joined failure receipts.

use super::*;

fn one_question_engine(base: &str) -> Engine {
    Engine::builder()
        .base_url(base)
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .throttle(THROTTLE)
        .expect("throttle")
        .max_retries(0)
        .profile_json(r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#)
        .expect("one question per request")
        .no_cache()
        .build()
        .expect("engine")
}

#[test]
fn a_terminal_batch_error_keeps_the_joined_late_reply_in_final_facts() {
    let _serial = serial();
    let alpha = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let beta = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let held_alpha = std::sync::Arc::clone(&alpha);
    let held_beta = std::sync::Arc::clone(&beta);
    let listener = Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains("alpha") {
            Canned::status(503, "busy").after_release(std::sync::Arc::clone(&held_alpha))
        } else {
            Canned::ok(DECIDED).after_release(std::sync::Arc::clone(&held_beta))
        }
    })
    .expect("listener");
    let engine = one_question_engine(listener.base());
    let asked = question();
    let events = Mutex::new(Vec::new());
    let (noticed, notice) = std::sync::mpsc::channel();
    let caller = thread::current().id();
    let observe = |event: thinkthen::AttemptObservation| {
        assert_eq!(thread::current().id(), caller);
        if event.outcome() == thinkthen::AttemptOutcome::Ok {
            noticed.send(()).expect("notice completed later request");
        }
        events.lock().expect("attempts").push(event);
    };
    let mut rows = engine.decide_many_with(
        &asked,
        ["alpha", "beta"],
        CallOptions::new()
            .batch(BatchSetting::Max)
            .observe_attempt(&observe),
    );
    let error = thread::scope(|scope| {
        let listener = &listener;
        let helper = scope.spawn(move || {
            let began = Instant::now();
            while listener.count() < 2 && began.elapsed() < Duration::from_secs(30) {
                thread::sleep(Duration::from_millis(5));
            }
            assert_eq!(listener.count(), 2, "both requests started");
            assert!(beta.wait(), "release later reply for join");
            notice
                .recv_timeout(Duration::from_secs(30))
                .expect("later attempt observed first");
            assert!(alpha.wait(), "release failed first request");
        });
        let error = rows
            .next()
            .expect("terminal row")
            .expect_err("first request failed");
        helper.join().expect("release helper");
        error
    });
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert!(rows.next().is_none());
    let ended = rows.facts().expect("frozen batch facts");
    let failed = error.facts().expect("same terminal facts");
    for facts in [ended, failed] {
        assert_eq!(
            (
                facts.records(),
                facts.requests_sent(),
                facts.input_tokens(),
                facts.output_tokens()
            ),
            (0, 2, Some(3), Some(1))
        );
    }
    assert_eq!(listener.count(), 2);
    let events = events.lock().expect("attempts");
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].outcome(), thinkthen::AttemptOutcome::Ok);
    assert_eq!(events[1].outcome(), thinkthen::AttemptOutcome::Status);
    let mut ordinals: Vec<_> = events
        .iter()
        .map(thinkthen::AttemptObservation::ordinal)
        .collect();
    ordinals.sort_unstable();
    assert_eq!(
        ordinals,
        [1, 2],
        "send ordinals are distinct despite reverse completion"
    );
}

#[test]
fn attempt_callback_panic_waits_for_a_held_later_worker() {
    let _serial = serial();
    let first = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let later = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let held_first = std::sync::Arc::clone(&first);
    let held_later = std::sync::Arc::clone(&later);
    let listener = Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains("alpha") {
            Canned::ok(DECIDED).after_release(std::sync::Arc::clone(&held_first))
        } else {
            Canned::ok(DECIDED).after_release(std::sync::Arc::clone(&held_later))
        }
    })
    .expect("listener");
    let engine = one_question_engine(listener.base());
    let (noticed, notice) = std::sync::mpsc::channel();
    let caller = thread::current().id();
    let observe = |_: thinkthen::AttemptObservation| {
        assert_eq!(thread::current().id(), caller);
        noticed.send(()).expect("notice");
        std::panic::resume_unwind(Box::new("attempt observer payload"));
    };
    let asked = question();
    let mut rows = engine.decide_many_with(
        &asked,
        ["alpha", "beta"],
        CallOptions::new()
            .batch(BatchSetting::Max)
            .observe_attempt(&observe),
    );
    let released = AtomicUsize::new(0);
    let caught =
        thread::scope(|scope| {
            let listener = &listener;
            let released = &released;
            let helper = scope.spawn(move || {
                let began = Instant::now();
                while listener.count() < 2 && began.elapsed() < Duration::from_secs(30) {
                    thread::sleep(Duration::from_millis(5));
                }
                assert_eq!(listener.count(), 2, "later worker is already in flight");
                assert!(first.wait(), "release first reply");
                notice
                    .recv_timeout(Duration::from_secs(30))
                    .expect("callback fired");
                assert!(later.wait(), "release later reply");
                released.store(1, Ordering::SeqCst);
            });
            let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                while rows.next().is_some() {}
            }));
            assert_eq!(
                released.load(Ordering::SeqCst),
                1,
                "panic resumed after join"
            );
            helper.join().expect("release helper");
            caught
        });
    let payload = caught.expect_err("callback payload resumes on caller");
    assert_eq!(
        payload.downcast_ref::<&str>(),
        Some(&"attempt observer payload")
    );
    assert_eq!(listener.count(), 2);
}

#[test]
fn dropping_a_lazy_batch_drains_attempts_and_resumes_panic_after_join() {
    let _serial = serial();
    let held = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let later = std::sync::Arc::clone(&held);
    let listener = Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains("beta") {
            Canned::ok(DECIDED).after_release(std::sync::Arc::clone(&later))
        } else {
            Canned::ok(DECIDED)
        }
    })
    .expect("listener");
    let engine = one_question_engine(listener.base());
    let caller = thread::current().id();
    let events = Mutex::new(Vec::new());
    let observe = |event: thinkthen::AttemptObservation| {
        assert_eq!(thread::current().id(), caller);
        let mut events = events.lock().expect("events");
        events.push(event);
        let count = events.len();
        drop(events);
        if count == 2 {
            std::panic::resume_unwind(Box::new("drop observer payload"));
        }
    };
    let asked = question();
    let mut rows = engine.decide_many_with(
        &asked,
        ["alpha", "beta"],
        CallOptions::new()
            .batch(BatchSetting::Max)
            .observe_attempt(&observe),
    );
    assert_eq!(
        rows.next().expect("first row").expect("answer").input(),
        &"alpha"
    );
    let caught = thread::scope(|scope| {
        let listener = &listener;
        scope.spawn(|| {
            let began = Instant::now();
            while listener.count() < 2 && began.elapsed() < Duration::from_secs(30) {
                thread::sleep(Duration::from_millis(5));
            }
            assert_eq!(listener.count(), 2, "later attempt was already sent");
            assert!(held.wait(), "release later reply during drop");
        });
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(rows)))
    });
    let payload = caught.expect_err("observer panic resumes after the lazy join");
    assert_eq!(
        payload.downcast_ref::<&str>(),
        Some(&"drop observer payload")
    );
    let events = events.lock().expect("events");
    assert_eq!(
        events.len(),
        2,
        "drop joins and delivers both sent attempts"
    );
}
