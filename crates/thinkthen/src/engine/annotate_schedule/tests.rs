use std::sync::{Arc, atomic::AtomicUsize, atomic::Ordering};
use std::thread;

use super::{Input, Outcome, Prepared};

#[test]
#[allow(clippy::excessive_nesting, reason = "synchronized reader fixture")]
fn annotate_cancellation_clears_undispatched_groups_and_joins() {
    let cancel = crate::engine::Cancel::default();
    let run_cancel = cancel.clone();
    let started = Arc::new(std::sync::Barrier::new(2));
    let answer_started = Arc::clone(&started);
    let release = Arc::new(std::sync::Barrier::new(2));
    let answer_release = Arc::clone(&release);
    let starts = Arc::new(AtomicUsize::new(0));
    let answer_starts = Arc::clone(&starts);
    let active = Arc::new(AtomicUsize::new(0));
    let answer_active = Arc::clone(&active);
    let requests = Arc::new(AtomicUsize::new(0));
    let reader_requests = Arc::clone(&requests);
    let (outcome_send, outcome) = std::sync::mpsc::channel();

    let run = thread::spawn(move || {
        let result = super::run(
            1,
            true,
            &run_cancel,
            move |asked, events| {
                thread::spawn(move || {
                    while asked.recv().is_ok() {
                        reader_requests.fetch_add(1, Ordering::SeqCst);
                        if events.send(Input::Item(())).is_err() {
                            break;
                        }
                    }
                });
            },
            |()| {
                Ok::<_, &'static str>(Prepared {
                    seed: (),
                    accumulator: Vec::<usize>::new(),
                    work: vec![0, 1],
                })
            },
            &move |group| {
                answer_starts.fetch_add(1, Ordering::SeqCst);
                answer_active.fetch_add(1, Ordering::SeqCst);
                answer_started.wait();
                answer_release.wait();
                answer_active.fetch_sub(1, Ordering::SeqCst);
                Ok::<_, &'static str>(group)
            },
            |answers, answer| {
                answers.push(answer);
                Ok(())
            },
            |_, _| -> Result<crate::engine::schedule::Completed<(), &'static str>, &'static str> {
                unreachable!("a cancelled group leaves the row unfinished")
            },
            |_| Ok(true),
            |_| "defect",
            |_| "cancelled",
        );
        outcome_send.send(result).expect("returned outcome");
    });

    started.wait();
    cancel.fire();
    release.wait();
    let result = outcome
        .recv_timeout(std::time::Duration::from_secs(2))
        .expect("the cancelled run returns")
        .expect("the scheduler returns metadata");
    run.join().expect("scheduler thread");

    assert_eq!(requests.load(Ordering::SeqCst), 1);
    assert_eq!(starts.load(Ordering::SeqCst), 1);
    assert_eq!(active.load(Ordering::SeqCst), 0);
    assert!(matches!(
        result,
        Outcome::Stopped {
            at: 1,
            finished: 0,
            cause: "cancelled",
            ..
        }
    ));
}

#[test]
fn aggregate_cancellation_while_waiting_requests_nothing_more() {
    let (waiting_send, waiting) = std::sync::mpsc::channel();
    let cancel = crate::engine::Cancel::observed(waiting_send);
    let run_cancel = cancel.clone();
    let (asked_send, asked_recv) = std::sync::mpsc::channel();
    let (outcome_send, outcome_recv) = std::sync::mpsc::channel();
    let run = thread::spawn(move || {
        let outcome = super::run(
            1,
            false,
            &run_cancel,
            move |asked, _events| asked_send.send(asked).expect("input requests"),
            |_: ()| -> Result<Prepared<(), (), ()>, &'static str> {
                unreachable!("withheld input cannot be prepared")
            },
            &|_: ()| -> Result<(), &'static str> { unreachable!("no work can start") },
            |_, _| Ok(()),
            |_, _| -> Result<crate::engine::schedule::Completed<(), &'static str>, &'static str> {
                unreachable!("no row can finish")
            },
            |_| Ok(true),
            |_| "defect",
            |_| "cancelled",
        );
        outcome_send.send(outcome).expect("returned outcome");
    });
    let asked = asked_recv
        .recv_timeout(std::time::Duration::from_secs(2))
        .expect("request receiver");
    asked
        .recv_timeout(std::time::Duration::from_secs(2))
        .expect("first input request");
    waiting
        .recv_timeout(std::time::Duration::from_secs(2))
        .expect("scheduler entered recv_timeout");

    cancel.fire();
    let outcome = outcome_recv
        .recv_timeout(std::time::Duration::from_secs(2))
        .expect("cancelled scheduler returns")
        .expect("cancelled outcome");
    run.join().expect("scheduler thread");

    assert_eq!(asked.try_iter().count(), 0);
    assert!(matches!(outcome, Outcome::Failed("cancelled")));
}
