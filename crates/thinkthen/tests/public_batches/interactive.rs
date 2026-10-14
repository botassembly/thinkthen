//! Caller-owned blocking iterators under explicit and maximal batching.

use super::*;

/// Yield one record, then hold the caller's thread until the test releases it.
fn held_input(
    release: std::sync::mpsc::Receiver<()>,
    pulls: std::sync::Arc<AtomicUsize>,
) -> impl Iterator<Item = &'static str> {
    let local = std::rc::Rc::new(());
    let mut stage = 0;
    std::iter::from_fn(move || {
        let _ = std::rc::Rc::strong_count(&local);
        match stage {
            0 => {
                stage = 1;
                pulls.fetch_add(1, Ordering::SeqCst);
                Some("alpha")
            }
            1 => {
                stage = 2;
                pulls.fetch_add(1, Ordering::SeqCst);
                release
                    .recv_timeout(Duration::from_secs(30))
                    .expect("release held caller input");
                None
            }
            _ => None,
        }
    })
}

#[test]
fn batch_one_returns_before_the_next_held_input_while_max_waits_for_close() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let asked = question();
    let (release_one, held_one) = std::sync::mpsc::channel();
    let pulled_one = std::sync::Arc::new(AtomicUsize::new(0));
    let mut one = crate::api::pull_decisions(
        &engine,
        &asked,
        crate::api::plain_records(held_input(held_one, std::sync::Arc::clone(&pulled_one))),
        CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
    );
    assert_eq!(
        one.next().expect("first row").expect("answer").original(),
        &"alpha"
    );
    assert_eq!(pulled_one.load(Ordering::SeqCst), 1);
    assert_eq!(listener.count(), 1);
    release_one.send(()).expect("release second pull");
    assert!(one.next().is_none());
    assert_eq!(
        one.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 1))
    );

    let set = QuestionSet::from_json(r#"{"version":1,"questions":{"first":{"decide":"First?"}}}"#)
        .expect("annotation set");
    let (release_annotation, held_annotation) = std::sync::mpsc::channel();
    let pulled_annotation = std::sync::Arc::new(AtomicUsize::new(0));
    let mut annotation = crate::api::pull_annotations(
        &engine,
        &set,
        crate::api::plain_records(held_input(
            held_annotation,
            std::sync::Arc::clone(&pulled_annotation),
        )),
        CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
    );
    assert_eq!(
        annotation
            .next()
            .expect("annotation row")
            .expect("answer")
            .original(),
        &"alpha"
    );
    assert_eq!(pulled_annotation.load(Ordering::SeqCst), 1);
    assert_eq!(listener.count(), 2);
    release_annotation
        .send(())
        .expect("release annotation input");
    assert!(annotation.next().is_none());
    assert_eq!(
        annotation
            .facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 1))
    );

    let (release_max, held_max) = std::sync::mpsc::channel();
    let pulled_max = std::sync::Arc::new(AtomicUsize::new(0));
    let mut max = crate::api::pull_decisions(
        &engine,
        &asked,
        crate::api::plain_records(held_input(held_max, std::sync::Arc::clone(&pulled_max))),
        CallOptions::new().batch(BatchSetting::Max),
    );
    let (row, held) = thread::scope(|scope| {
        let watcher = scope.spawn(|| {
            let began = Instant::now();
            while pulled_max.load(Ordering::SeqCst) < 2 && began.elapsed() < Duration::from_secs(30)
            {
                thread::sleep(Duration::from_millis(5));
            }
            let held = (pulled_max.load(Ordering::SeqCst), listener.count());
            release_max.send(()).expect("release Max pull");
            held
        });
        let row = max.next().expect("Max row").expect("answer");
        (row, watcher.join().expect("watcher"))
    });
    assert_eq!(held, (2, 2), "Max sent nothing while input was held");
    assert_eq!(row.original(), &"alpha");
    assert!(max.next().is_none());
    assert_eq!(
        max.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 1))
    );
    assert_eq!(listener.count(), 3);
}

#[test]
fn batch_one_failure_ends_without_pulling_another_caller_record() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::status(503, "busy")).expect("listener");
    let engine = crate::api::builder(listener.base(), "sk-public-batches")
        .max_retries(0)
        .no_cache()
        .build()
        .expect("engine");
    let pulls = AtomicUsize::new(0);
    let observed = AtomicUsize::new(0);
    let observe = |_| {
        observed.fetch_add(1, Ordering::SeqCst);
    };
    let local = std::rc::Rc::new(());
    let records = std::iter::from_fn(|| {
        let _ = std::rc::Rc::strong_count(&local);
        match pulls.fetch_add(1, Ordering::SeqCst) {
            0 => Some("alpha"),
            _ => None,
        }
    });
    let asked = question();
    let mut batch = crate::api::pull_decisions(
        &engine,
        &asked,
        crate::api::plain_records(records),
        CallOptions::new()
            .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
            .observe_attempt(&observe),
    );
    let error = batch
        .next()
        .expect("terminal error")
        .expect_err("failed first request");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(pulls.load(Ordering::SeqCst), 1);
    assert_eq!(
        batch
            .facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((0, 1))
    );
    assert!(batch.next().is_none());
    assert_eq!(pulls.load(Ordering::SeqCst), 1);
    assert_eq!(listener.count(), 1);
    assert_eq!(
        observed.load(Ordering::SeqCst),
        1,
        "one failed send, no future input attempts"
    );
    for facts in [error.facts(), batch.facts()] {
        assert_eq!(
            facts.map(|facts| (facts.records(), facts.requests_sent())),
            Some((0, 1))
        );
    }
}

fn failing_input(pulls: &AtomicUsize) -> impl Iterator<Item = Result<&'static str, Error>> + '_ {
    let local = std::rc::Rc::new(());
    std::iter::from_fn(move || {
        let _ = std::rc::Rc::strong_count(&local);
        let stage = pulls.fetch_add(1, Ordering::SeqCst);
        assert!(stage < 2, "input tail must stay unread");
        Some(if stage == 0 {
            Ok("alpha")
        } else {
            Err(Error::new(ErrorKind::Local, "later reader failure"))
        })
    })
}

fn fallible_rows<'a>(
    engine: &Engine,
    asked: &Question,
    pulls: &'a AtomicUsize,
    function: thinkthen::RequestFunction,
    options: CallOptions<'a>,
) -> thinkthen::RequestRows<'a, &'static str> {
    crate::api::rows(
        engine,
        function,
        asked.clone().into(),
        failing_input(pulls).map(|row| row.map(crate::api::record)),
        options,
    )
    .expect("request rows")
}

fn reader_failure<T>(
    mut batch: thinkthen::Batch<'_, T>,
    pulls: &AtomicUsize,
    check: impl FnOnce(&T),
) {
    check(&batch.next().expect("prefix").expect("answer"));
    assert_eq!(pulls.load(Ordering::SeqCst), 1);
    let error = batch
        .next()
        .expect("ordered reader error")
        .err()
        .expect("reader failure");
    assert_eq!(error.to_string(), "later reader failure");
    for facts in [error.facts(), batch.facts()] {
        assert_eq!(
            facts.map(|f| (f.records(), f.requests_sent())),
            Some((1, 1))
        );
    }
    assert!(batch.next().is_none());
    assert_eq!(pulls.load(Ordering::SeqCst), 2);
}

#[test]
fn fallible_batches_return_the_prefix_then_reader_error_with_final_counts() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let asked = question();
    let options = CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN));
    let pulls = AtomicUsize::new(0);
    reader_failure(
        crate::api::decision_rows(fallible_rows(
            &engine,
            &asked,
            &pulls,
            thinkthen::RequestFunction::Decide,
            options,
        ))
        .expect("decisions"),
        &pulls,
        |_| {},
    );
    let pulls = AtomicUsize::new(0);
    reader_failure(
        crate::api::filter_rows(fallible_rows(
            &engine,
            &asked,
            &pulls,
            thinkthen::RequestFunction::Filter,
            options,
        ))
        .expect("filtered"),
        &pulls,
        |row| assert!(row.result().value()),
    );
    let set = QuestionSet::from_json(r#"{"version":1,"questions":{"first":{"decide":"First?"}}}"#)
        .expect("set");
    let pulls = AtomicUsize::new(0);
    reader_failure(
        crate::api::annotation_rows(
            crate::api::rows(
                &engine,
                thinkthen::RequestFunction::Annotate,
                set.clone().into(),
                failing_input(&pulls).map(|row| row.map(crate::api::record)),
                options,
            )
            .expect("annotation rows"),
        )
        .expect("annotations"),
        &pulls,
        |_| {},
    );
    assert_eq!(listener.count(), 3);
}

#[test]
fn dropping_a_fallible_batch_does_not_read_the_failure_or_tail() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let asked = question();
    let pulls = AtomicUsize::new(0);
    let mut batch = crate::api::filter_rows(fallible_rows(
        &engine,
        &asked,
        &pulls,
        thinkthen::RequestFunction::Filter,
        CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
    ))
    .expect("filtered");
    let row = batch.next().expect("row").expect("answer");
    assert_eq!(row.original(), &"alpha");
    assert!(row.result().value());
    drop(batch);
    assert_eq!(pulls.load(Ordering::SeqCst), 1);
    assert_eq!(listener.count(), 1);
}

#[test]
fn reader_failure_flushes_a_partial_pack_and_drains_its_attempt() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let asked = question();
    let pulls = AtomicUsize::new(0);
    let attempts = AtomicUsize::new(0);
    let observe = |_| {
        attempts.fetch_add(1, Ordering::SeqCst);
    };
    let mut batch = crate::api::filter_rows(fallible_rows(
        &engine,
        &asked,
        &pulls,
        thinkthen::RequestFunction::Filter,
        CallOptions::new()
            .batch(BatchSetting::Records(
                std::num::NonZeroUsize::new(2).expect("two"),
            ))
            .observe_attempt(&observe),
    ))
    .expect("filtered");
    let row = batch.next().expect("prefix").expect("answer");
    assert_eq!(row.original(), &"alpha");
    assert!(row.result().value());
    let error = batch.next().expect("failure").expect_err("reader failure");
    assert_eq!(error.to_string(), "later reader failure");
    assert_eq!(
        error.facts().map(|f| (f.records(), f.requests_sent())),
        Some((1, 1))
    );
    assert_eq!(attempts.load(Ordering::SeqCst), 1);
    assert_eq!(pulls.load(Ordering::SeqCst), 2);
    assert_eq!(listener.count(), 1);
    assert!(batch.next().is_none());
}
