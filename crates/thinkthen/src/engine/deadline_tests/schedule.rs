//! Whole-call deadline proofs for both schedulers.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use super::{named, spent, within};
use crate::engine::schedule::{Completed, Input, Outcome};
use crate::engine::{annotate_schedule, schedule};

type Port<T> = schedule::InputPort<T, T, &'static str>;

/// Answer each input request with the next input until the scheduler leaves.
fn feed<T, E>(
    asked: &Receiver<()>,
    asks: &AtomicUsize,
    mut next: impl FnMut() -> Input<T, E>,
    send: impl Fn(Input<T, E>) -> Result<(), ()>,
) {
    while asked.recv().is_ok() {
        asks.fetch_add(1, Ordering::SeqCst);
        if send(next()).is_err() {
            return;
        }
    }
}

/// A reader that answers every request with the next item, then the end.
fn reader<T: Send + 'static>(
    items: Vec<T>,
    asks: &Arc<AtomicUsize>,
) -> impl FnOnce(Receiver<()>, Port<T>) {
    let asks = Arc::clone(asks);
    move |asked, events| {
        thread::spawn(move || {
            let mut items = items.into_iter();
            feed(
                &asked,
                &asks,
                || items.next().map_or(Input::End, Input::Item),
                |input| events.send(input),
            );
        });
    }
}

const fn done<T>(value: T) -> Result<Completed<T, &'static str>, &'static str> {
    Ok(Completed::one(value, false, false))
}

#[test]
fn a_spent_deadline_over_empty_bulk_input_stops_before_reading() {
    let asks = Arc::new(AtomicUsize::new(0));
    let cancelled = spent();
    cancelled.fire();
    for (cancel, stop) in [(spent(), "deadline"), (cancelled, "cancelled")] {
        let outcome = schedule::run_cancelled(
            2,
            schedule::RecordFlow::Streaming,
            &cancel,
            reader(Vec::<usize>::new(), &asks),
            &|_: &usize| -> Result<Completed<usize, &'static str>, &'static str> {
                unreachable!("no input")
            },
            |_| Ok(true),
            |_| "defect",
            named,
        )
        .expect("metadata");
        assert!(
            matches!(outcome, Outcome::Stopped { finished: 0, cause, .. } if cause == stop),
            "{stop}"
        );
    }
    assert_eq!(asks.load(Ordering::SeqCst), 0);
    for streams in [true, false] {
        let outcome = annotate_schedule::run(
            2,
            streams,
            &spent(),
            |_asked, _events: annotate_schedule::InputPort<(), (), &'static str>| {},
            |()| -> Result<annotate_schedule::Prepared<(), (), ()>, &'static str> {
                unreachable!("no input")
            },
            &|()| -> Result<(), &'static str> { unreachable!("no work") },
            |_: &mut (), ()| Ok(()),
            |(), ()| -> Result<Completed<(), &'static str>, &'static str> {
                unreachable!("no row")
            },
            |()| Ok(true),
            |_| "defect",
            named,
        )
        .expect("metadata");
        let expected = if streams {
            matches!(
                outcome,
                annotate_schedule::Outcome::Stopped {
                    finished: 0,
                    cause: "deadline",
                    ..
                }
            )
        } else {
            matches!(outcome, annotate_schedule::Outcome::Failed("deadline"))
        };
        assert!(expected, "streams {streams}");
    }
}

#[test]
fn one_deadline_spans_every_record_and_starts_no_undispatched_request() {
    let budget = Duration::from_millis(500);
    let asks = Arc::new(AtomicUsize::new(0));
    let starts = AtomicUsize::new(0);
    let held = Barrier::new(2);
    let mut emitted = Vec::new();
    let outcome = thread::scope(|scope| {
        scope.spawn(|| {
            held.wait();
            thread::sleep(budget + Duration::from_millis(200));
            held.wait();
        });
        schedule::run_cancelled(
            1,
            schedule::RecordFlow::Streaming,
            &within(budget),
            reader(vec![0, 1, 2], &asks),
            &|item: &usize| {
                starts.fetch_add(1, Ordering::SeqCst);
                if *item == 1 {
                    held.wait();
                    held.wait();
                }
                done(*item)
            },
            |item| {
                emitted.push(item);
                Ok(true)
            },
            |_| "defect",
            named,
        )
    })
    .expect("metadata");

    assert_eq!(emitted, [0, 1]);
    assert_eq!(starts.load(Ordering::SeqCst), 2);
    assert_eq!(asks.load(Ordering::SeqCst), 2);
    assert!(matches!(
        outcome,
        Outcome::Stopped {
            finished: 2,
            cause: "deadline",
            ..
        }
    ));
}

/// A later result is queued while the coordinator is still emitting; the
/// deadline passes; only then does the coordinator reach its stop check.
fn queued_before_the_check(
    later: Result<Completed<usize, &'static str>, &'static str>,
) -> (Vec<usize>, Outcome<&'static str>, usize) {
    let budget = Duration::from_millis(300);
    let asks = Arc::new(AtomicUsize::new(0));
    let starts = AtomicUsize::new(0);
    let [both_started, emitting, later_released, emit_released] = [(); 4].map(|()| Barrier::new(2));
    let (answered_send, answered) = channel();
    let later = std::sync::Mutex::new(Some(later));
    let mut emitted = Vec::new();
    let outcome = thread::scope(|scope| {
        scope.spawn(|| {
            let answered = answered;
            emitting.wait();
            later_released.wait();
            answered.recv().expect("the later result is queued");
            thread::sleep(budget + Duration::from_millis(200));
            emit_released.wait();
        });
        schedule::run_cancelled(
            2,
            schedule::RecordFlow::Streaming,
            &within(budget),
            reader(vec![0, 1, 2], &asks),
            &|item: &usize| {
                starts.fetch_add(1, Ordering::SeqCst);
                both_started.wait();
                if *item == 0 {
                    return done(0);
                }
                later_released.wait();
                let result = later
                    .lock()
                    .expect("later")
                    .take()
                    .expect("one later result");
                answered_send.send(()).expect("queued");
                result
            },
            |item| {
                if item == 0 {
                    emitting.wait();
                    emit_released.wait();
                }
                emitted.push(item);
                Ok(true)
            },
            |_| "defect",
            named,
        )
    })
    .expect("metadata");
    (emitted, outcome, starts.into_inner())
}

#[test]
fn a_result_queued_before_the_deadline_check_keeps_its_place_and_the_order() {
    let (emitted, outcome, starts) = queued_before_the_check(done(1));
    assert_eq!(emitted, [0, 1]);
    assert_eq!(starts, 2);
    assert!(matches!(
        outcome,
        Outcome::Stopped {
            finished: 2,
            cause: "deadline",
            ..
        }
    ));

    let (emitted, outcome, starts) = queued_before_the_check(Err("backend"));
    assert_eq!(emitted, [0]);
    assert_eq!(starts, 2);
    assert!(matches!(
        outcome,
        Outcome::Stopped {
            finished: 1,
            cause: "backend",
            ..
        }
    ));
}

#[test]
fn a_deadline_clears_undispatched_annotation_groups() {
    let budget = Duration::from_millis(300);
    let held = Barrier::new(2);
    let starts = AtomicUsize::new(0);
    let asks = Arc::new(AtomicUsize::new(0));
    let reader_asks = Arc::clone(&asks);
    let outcome = thread::scope(|scope| {
        scope.spawn(|| {
            held.wait();
            thread::sleep(budget + Duration::from_millis(200));
            held.wait();
        });
        annotate_schedule::run(
            1,
            true,
            &within(budget),
            move |asked, events| {
                thread::spawn(move || {
                    feed(
                        &asked,
                        &reader_asks,
                        || Input::Item(()),
                        |input| events.send(input),
                    );
                });
            },
            |()| {
                Ok::<_, &'static str>(annotate_schedule::Prepared {
                    seed: (),
                    accumulator: Vec::<usize>::new(),
                    work: vec![0, 1],
                })
            },
            &|group: usize| {
                starts.fetch_add(1, Ordering::SeqCst);
                held.wait();
                held.wait();
                Ok::<_, &'static str>(group)
            },
            |answers, answer| {
                answers.push(answer);
                Ok(())
            },
            |(), _| -> Result<Completed<(), &'static str>, &'static str> {
                unreachable!("the stopped row never finishes")
            },
            |()| Ok(true),
            |_| "defect",
            named,
        )
    })
    .expect("metadata");

    assert_eq!(starts.load(Ordering::SeqCst), 1);
    assert_eq!(asks.load(Ordering::SeqCst), 1);
    assert!(matches!(
        outcome,
        annotate_schedule::Outcome::Stopped {
            finished: 0,
            cause: "deadline",
            ..
        }
    ));
}
