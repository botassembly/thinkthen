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
                    .recv_timeout(Duration::from_secs(3))
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
    let mut one = engine.decide_many_with(
        &asked,
        held_input(held_one, std::sync::Arc::clone(&pulled_one)),
        CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
    );
    assert_eq!(
        one.next().expect("first row").expect("answer").input(),
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
    let mut annotation = engine.annotate_with(
        &set,
        held_input(held_annotation, std::sync::Arc::clone(&pulled_annotation)),
        CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
    );
    assert_eq!(
        annotation
            .next()
            .expect("annotation row")
            .expect("answer")
            .input(),
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
    let mut max = engine.decide_many_with(
        &asked,
        held_input(held_max, std::sync::Arc::clone(&pulled_max)),
        CallOptions::new().batch(BatchSetting::Max),
    );
    let (row, held) = thread::scope(|scope| {
        let watcher = scope.spawn(|| {
            let began = Instant::now();
            while pulled_max.load(Ordering::SeqCst) < 2 && began.elapsed() < Duration::from_secs(3)
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
    assert_eq!(row.input(), &"alpha");
    assert!(max.next().is_none());
    assert_eq!(
        max.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 1))
    );
    assert_eq!(listener.count(), 3);
}
