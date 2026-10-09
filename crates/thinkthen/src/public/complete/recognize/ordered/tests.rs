//! Deadline and cancellation proofs for the ordered runner, moved with it
//! from the engine's old record scheduler.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use super::{Input, Outcome, Row};
use crate::engine::error::Error;
use crate::engine::{Cancel, Deadline};

type Port<T> = crate::cli::schedule::ordered::Port<T, T, &'static str>;

fn within(budget: Duration) -> Cancel<'static> {
    Cancel::default().with_deadline(Deadline::after(budget))
}

fn spent() -> Cancel<'static> {
    within(Duration::ZERO)
}

fn named(stop: Error) -> &'static str {
    match stop {
        Error::Deadline(_) => "deadline",
        Error::Cancelled => "cancelled",
        _ => "other",
    }
}

/// Run `answer` over the reader's items and emit each row through `emit`.
fn run<T: Send + 'static>(
    jobs: usize,
    cancel: &Cancel,
    start_reader: impl FnOnce(Receiver<()>, Port<T>),
    answer: &(impl Fn(T) -> Result<Row<T>, &'static str> + Sync),
    emit: impl FnMut(T) -> Result<bool, &'static str>,
) -> Result<Outcome<&'static str>, &'static str> {
    crate::cli::schedule::ordered::run(
        jobs,
        cancel,
        start_reader,
        answer,
        emit,
        &named,
        || "defect",
    )
}

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

const fn done<T>(value: T) -> Result<Row<T>, &'static str> {
    Ok(Row {
        value,
        replayed: false,
    })
}

#[test]
fn a_spent_deadline_over_empty_bulk_input_stops_before_reading() {
    let asks = Arc::new(AtomicUsize::new(0));
    let cancelled = spent();
    cancelled.fire();
    for (cancel, stop) in [(spent(), "deadline"), (cancelled, "cancelled")] {
        let outcome = run(
            2,
            &cancel,
            reader(Vec::<usize>::new(), &asks),
            &|_: usize| -> Result<Row<usize>, &'static str> { unreachable!("no input") },
            |_| Ok(true),
        )
        .expect("metadata");
        assert!(
            matches!(outcome, Outcome::Stopped { finished: 0, cause, .. } if cause == stop),
            "{stop}"
        );
    }
    assert_eq!(asks.load(Ordering::SeqCst), 0);
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
        run(
            1,
            &within(budget),
            reader(vec![0, 1, 2], &asks),
            &|item: usize| {
                starts.fetch_add(1, Ordering::SeqCst);
                if item == 1 {
                    held.wait();
                    held.wait();
                }
                done(item)
            },
            |item| {
                emitted.push(item);
                Ok(true)
            },
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
    later: Result<Row<usize>, &'static str>,
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
        run(
            2,
            &within(budget),
            reader(vec![0, 1, 2], &asks),
            &|item: usize| {
                starts.fetch_add(1, Ordering::SeqCst);
                both_started.wait();
                if item == 0 {
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
fn cancellation_while_waiting_for_input_requests_nothing_more() {
    let (waiting_send, waiting) = channel();
    let cancel = Cancel::observed(waiting_send);
    let run_cancel = cancel.clone();
    let (asked_send, asked_recv) = channel();
    let (outcome_send, outcome_recv) = channel();
    let runner = thread::spawn(move || {
        let outcome = run(
            1,
            &run_cancel,
            move |asked, _events| asked_send.send(asked).expect("input requests"),
            &|_: ()| -> Result<Row<()>, &'static str> {
                unreachable!("withheld input cannot be dispatched")
            },
            |()| Ok(true),
        );
        outcome_send.send(outcome).expect("returned outcome");
    });
    let asked = asked_recv
        .recv_timeout(Duration::from_secs(30))
        .expect("request receiver");
    asked
        .recv_timeout(Duration::from_secs(30))
        .expect("first input request");
    waiting
        .recv_timeout(Duration::from_secs(30))
        .expect("runner entered recv_timeout");

    cancel.fire();
    let outcome = outcome_recv
        .recv_timeout(Duration::from_secs(30))
        .expect("cancelled runner returns")
        .expect("cancelled outcome");
    runner.join().expect("runner thread");

    assert_eq!(asked.try_iter().count(), 0);
    assert!(matches!(
        outcome,
        Outcome::Stopped {
            finished: 0,
            cause: "cancelled",
            ..
        }
    ));
}

#[test]
#[allow(clippy::excessive_nesting, reason = "synchronized reader fixture")]
fn cancellation_with_work_in_flight_ignores_later_input_and_joins() {
    let facts = crate::engine::CallFacts::new();
    let cancel = Cancel::default().with_facts(facts.clone());
    let run_cancel = cancel.clone();
    let started = Arc::new(Barrier::new(2));
    let answer_started = Arc::clone(&started);
    let second_requested = Arc::new(Barrier::new(2));
    let reader_requested = Arc::clone(&second_requested);
    let later_input = Arc::new(Barrier::new(2));
    let reader_later = Arc::clone(&later_input);
    let release = Arc::new(Barrier::new(2));
    let answer_release = Arc::clone(&release);
    let active = Arc::new(AtomicUsize::new(0));
    let answer_active = Arc::clone(&active);
    let starts = Arc::new(AtomicUsize::new(0));
    let answer_starts = Arc::clone(&starts);
    let (emitted_send, emitted) = channel();
    let (outcome_send, outcome) = channel();

    let runner = thread::spawn(move || {
        let result = run(
            2,
            &run_cancel,
            move |asked, events: Port<usize>| {
                thread::spawn(move || {
                    asked.recv().expect("first input request");
                    events.send(Input::Item(0)).expect("first input");
                    asked.recv().expect("second input request");
                    reader_requested.wait();
                    reader_later.wait();
                    let _ignored = events.send(Input::Item(1));
                });
            },
            &move |item| {
                answer_starts.fetch_add(1, Ordering::SeqCst);
                answer_active.fetch_add(1, Ordering::SeqCst);
                answer_started.wait();
                answer_release.wait();
                answer_active.fetch_sub(1, Ordering::SeqCst);
                done(item)
            },
            |value| {
                emitted_send.send(value).expect("emitted result");
                Ok(true)
            },
        );
        outcome_send.send(result).expect("returned outcome");
    });

    started.wait();
    second_requested.wait();
    cancel.fire();
    later_input.wait();
    release.wait();
    let result = outcome
        .recv_timeout(Duration::from_secs(30))
        .expect("the cancelled run returns")
        .expect("the runner returns metadata");
    runner.join().expect("runner thread");

    assert_eq!(emitted.try_iter().collect::<Vec<_>>(), [0]);
    assert_eq!(starts.load(Ordering::SeqCst), 1);
    assert_eq!(active.load(Ordering::SeqCst), 0);
    assert_eq!(facts.snapshot().records, 1);
    assert!(matches!(
        result,
        Outcome::Stopped {
            finished: 1,
            cause: "cancelled",
            ..
        }
    ));
}
