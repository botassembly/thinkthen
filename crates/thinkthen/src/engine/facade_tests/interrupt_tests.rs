//! The host interrupt check, held against a counted loopback listener.
//!
//! Each row holds one call in one wait. The check records the thread and the
//! listener count of every run, and returns `true` on its third run while the
//! call is held, so a call ends only when the check keeps running.

use std::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Mutex, PoisonError};
use std::thread::{self, ThreadId};
use std::time::Duration;

use conformance_backend::{Backend as Loopback, Canned, Listener};

use super::{Scratch, ask, bulk, engine, feed, settings};
use crate::engine::cache_lock;
use crate::engine::error::{Error, Kind};
use crate::engine::facade::{
    Completed, Engine, GroupOutcome, GroupPort, Prepared, RunOutcome, Settings, Storage,
};
use crate::engine::{Cancel, Deadline};

/// The longest a held call waits for its check to stop it.
const BOUND: Duration = Duration::from_secs(3);

/// The runs a held call makes before its check answers `true`.
const HELD_RUNS: usize = 3;

/// What one held call returned and what its check saw.
struct Held<R> {
    result: R,
    caller: ThreadId,
    /// The thread and the listener count of each check run, in order.
    runs: Vec<(ThreadId, usize)>,
    stopped_in_time: bool,
}

/// A call token whose deadline frees a call the check failed to stop.
fn bounded() -> Cancel<'static> {
    Cancel::default().with_deadline(Deadline::after(BOUND))
}

/// Run `call` on its own thread under `base` and a check that stops it on its
/// third run while `holding` holds. `release` lets the held work go once the
/// check stopped the call, or once `BOUND` passed without a stop.
fn hold<R: Send>(
    base: &Cancel<'_>,
    sent: &(dyn Fn() -> usize + Sync),
    holding: &(dyn Fn() -> bool + Sync),
    release: &dyn Fn(),
    call: impl FnOnce(&Cancel<'_>) -> R + Send,
) -> Held<R> {
    let (stopped, stop) = channel();
    thread::scope(|scope| {
        let run = scope.spawn(|| {
            let runs = Mutex::new(Vec::new());
            let during = AtomicUsize::new(0);
            let check = || {
                let mut runs = runs.lock().unwrap_or_else(PoisonError::into_inner);
                runs.push((thread::current().id(), sent()));
                let interrupt = holding() && during.fetch_add(1, Ordering::SeqCst) + 1 >= HELD_RUNS;
                let _told = interrupt.then(|| stopped.send(()));
                interrupt
            };
            let result = call(&base.with_check(&check));
            let runs = runs.into_inner().unwrap_or_else(PoisonError::into_inner);
            (result, thread::current().id(), runs)
        });
        let stopped_in_time = stop.recv_timeout(BOUND).is_ok();
        release();
        let (result, caller, runs) = run.join().expect("the held call");
        Held {
            result,
            caller,
            runs,
            stopped_in_time,
        }
    })
}

/// Check every observation the table rows share.
fn assert_stopped_on_the_caller<R>(name: &str, held: &Held<R>, before: usize) {
    assert!(held.stopped_in_time, "{name}: the check ran at each poll");
    assert!(
        held.runs.iter().all(|(thread, _)| *thread == held.caller),
        "{name}: the check ran on a worker"
    );
    assert_eq!(
        held.runs.first().map(|(_, sent)| *sent),
        Some(before),
        "{name}: the check ran before the first send"
    );
}

fn stop_kind(result: &Result<Option<f64>, Error>) -> Option<Kind> {
    result.as_ref().err().map(Error::kind)
}

/// Hold a call at a width gate that four sends fill. The four ask on the
/// call's own base token when `shared`, so their sends are not the call's.
fn width_gate(name: &str, shared: bool) {
    let loopback = Loopback::start().expect("loopback");
    let gated = &engine(&format!("{}/arm/held/v1", loopback.origin()));
    let base = bounded();
    thread::scope(|scope| {
        let holders: Vec<_> = (0..4)
            .map(|_| {
                let token = if shared { base.clone() } else { bounded() };
                scope.spawn(move || ask(gated, "Refund me.", &token))
            })
            .collect();
        assert_eq!(loopback.wait(4), 4, "{name}: four sends hold the gate");
        let held = hold(
            &base,
            &|| loopback.count(),
            &|| loopback.count() == 4,
            &|| loopback.release(),
            |cancel| ask(gated, "Refund me too.", cancel),
        );
        assert_stopped_on_the_caller(name, &held, 4);
        assert_eq!(stop_kind(&held.result), Some(Kind::Cancelled), "{name}");
        for holder in holders {
            let answer = holder.join().expect("holder");
            assert_eq!(answer.expect("a sent attempt finishes"), Some(0.9));
        }
        assert_eq!(loopback.count(), 4, "{name}: nothing new was sent");
    });
}

/// Run six records through a scheduler that takes the call's token.
type Scheduled<'a> = &'a (dyn Fn(&Engine, &[&'static str], &Cancel<'_>) -> Option<Error> + Sync);

/// The stop cause of six records through the grouped annotate scheduler.
fn grouped(engine: &Engine, texts: &[&'static str], cancel: &Cancel<'_>) -> Option<Error> {
    let items = texts.to_vec();
    let outcome = engine.groups(
        true,
        cancel,
        move |requests, port: GroupPort<&'static str, Option<f64>, Error>| {
            thread::spawn(move || feed(items, &requests, |input| port.send(input)));
        },
        |text| {
            Ok(Prepared {
                seed: (),
                accumulator: (),
                work: vec![text],
            })
        },
        &|text| ask(engine, text, cancel),
        |(), _| Ok(()),
        |(), ()| {
            Ok(Completed {
                value: (),
                replayed: false,
                partial_failure: false,
            })
        },
        |()| Ok(true),
    );
    match outcome {
        Ok(GroupOutcome::Stopped { cause, .. }) => Some(cause),
        _ => None,
    }
}

/// The stop cause of six records through the record scheduler.
fn recorded(engine: &Engine, texts: &[&'static str], cancel: &Cancel<'_>) -> Option<Error> {
    match bulk(engine, texts, cancel, cancel).0 {
        Ok(RunOutcome::Stopped { cause, .. }) => Some(cause),
        _ => None,
    }
}

#[test]
fn a_true_check_stops_each_held_wait_from_the_calling_thread_and_sends_nothing_new() {
    width_gate("width gate", false);
    width_gate("width gate on a shared token", true);

    // Retry wait: the first reply asks for a retry after a wait of 20 s.
    let busy = Listener::answering(|_| Canned::status(503, "")).expect("listener");
    let retrying = Engine::new(Settings {
        max_retries: 3,
        retry_wait: Duration::from_secs(20),
        timeout: Duration::from_secs(30),
        ..settings(busy.base())
    })
    .expect("engine");
    let held = hold(
        &bounded(),
        &|| busy.count(),
        &|| busy.count() == 1,
        &|| (),
        |cancel| ask(&retrying, "Refund me.", cancel),
    );
    assert_stopped_on_the_caller("retry wait", &held, 0);
    assert_eq!(stop_kind(&held.result), Some(Kind::Cancelled));
    assert_eq!(busy.count(), 1, "retry wait: nothing new was sent");

    // Recording lock wait: another owner holds the recording folder.
    let quiet = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let folder = Scratch::new("interrupt-lock");
    std::fs::create_dir_all(&folder.0).expect("folder");
    let owner = Mutex::new(Some(
        cache_lock::exclusive_folder(&folder.0).expect("owner"),
    ));
    let recording = Engine::new(Settings {
        storage: Storage {
            record: Some(folder.0.clone()),
            ..Storage::default()
        },
        ..settings(quiet.base())
    })
    .expect("engine");
    let held = hold(
        &bounded(),
        &|| quiet.count(),
        &|| true,
        &|| drop(owner.lock().map(|mut owner| owner.take())),
        |cancel| ask(&recording, "Refund me.", cancel),
    );
    assert_stopped_on_the_caller("lock wait", &held, 0);
    assert_eq!(stop_kind(&held.result), Some(Kind::Cancelled));
    assert_eq!(quiet.count(), 0, "lock wait: nothing was sent");

    // Bulk reply: six records over four workers, whose four replies are held.
    let schedulers: [(&str, Scheduled<'_>); 2] = [("records", &recorded), ("groups", &grouped)];
    for (name, scheduled) in schedulers {
        let loopback = Loopback::start().expect("loopback");
        let batch = engine(&format!("{}/arm/held/v1", loopback.origin()));
        let texts = ["one", "two", "three", "four", "five", "six"];
        let held = hold(
            &bounded(),
            &|| loopback.count(),
            &|| loopback.count() == 4,
            &|| loopback.release(),
            |cancel| scheduled(&batch, &texts, cancel),
        );
        assert_stopped_on_the_caller(name, &held, 0);
        let kind = held.result.as_ref().map(Error::kind);
        assert_eq!(kind, Some(Kind::Cancelled), "{name}: {:?}", held.result);
        assert_eq!(loopback.count(), 4, "{name}: nothing new was sent");
    }
}

#[test]
fn no_check_runs_during_one_held_single_send() {
    let loopback = Loopback::start().expect("loopback");
    let engine = engine(&format!("{}/arm/held/v1", loopback.origin()));
    let runs = Mutex::new(Vec::new());
    let check = || {
        runs.lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(thread::current().id());
        false
    };
    let count = || runs.lock().unwrap_or_else(PoisonError::into_inner).len();

    let (answer, before_send, during_send) = thread::scope(|scope| {
        let call =
            scope.spawn(|| ask(&engine, "Refund me.", &Cancel::default().with_check(&check)));
        assert_eq!(loopback.wait(1), 1, "the send is held");
        let before_send = count();
        // Six polls pass while the send is held.
        thread::sleep(Cancel::poll() * 6);
        let during_send = count() - before_send;
        loopback.release();
        (call.join().expect("call"), before_send, during_send)
    });

    assert_eq!(answer.expect("the answer"), Some(0.9));
    assert!(before_send >= 1, "the check ran before the send");
    assert_eq!(during_send, 0, "no check ran during the held send");
}

#[test]
fn a_panicking_check_stops_the_call_then_resumes_its_payload_after_the_join() {
    const PAYLOAD: &str = "the host check panicked";
    let loopback = Loopback::start().expect("loopback");
    let gated = engine(&format!("{}/arm/held/v1", loopback.origin()));
    // The check panics on its third run while the gate is held; a call it
    // stopped still waits on no permit, and a call it did not stop would send.
    let runs = AtomicUsize::new(0);
    let (panicking, panicked) = channel();
    let check = || {
        let held = loopback.count() == 4 && runs.fetch_add(1, Ordering::SeqCst) + 1 >= HELD_RUNS;
        if held {
            let _told = panicking.send(());
            resume_unwind(Box::new(PAYLOAD));
        }
        false
    };
    // The deadline bounds the gate wait should the check never panic.
    let bounded = Cancel::default().with_deadline(Deadline::after(BOUND));

    let (payload, holders) = thread::scope(|scope| {
        let holders: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| ask(&gated, "Refund me.", &Cancel::default())))
            .collect();
        assert_eq!(loopback.wait(4), 4, "four sends hold the gate");
        let call = scope.spawn(|| {
            catch_unwind(AssertUnwindSafe(|| {
                ask(&gated, "Refund me too.", &bounded.with_check(&check))
            }))
        });
        // Free the gate two polls after the panic: an uncancelled waiter sends.
        let _panicked = panicked.recv_timeout(BOUND);
        thread::sleep(Cancel::poll() * 2);
        loopback.release();
        let payload = call.join().expect("the caller catches the panic");
        let holders: Vec<_> = holders.into_iter().map(|holder| holder.join()).collect();
        (payload, holders)
    });

    let payload: Box<dyn Any + Send> = match payload {
        Err(payload) => payload,
        Ok(result) => panic!("the panic was swallowed into {result:?}"),
    };
    assert_eq!(payload.downcast_ref::<&str>(), Some(&PAYLOAD));
    for holder in holders {
        assert_eq!(holder.expect("holder").expect("answer"), Some(0.9));
    }
    assert_eq!(loopback.count(), 4, "nothing was sent after the panic");
}
