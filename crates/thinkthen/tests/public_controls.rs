//! The caller's controls through the public API, held on counted loopback listeners.
//!
//! Each row protects one promise of 0084 and 0095: a spent call sends
//! nothing, a stopped call sends nothing new, sent work finishes, the check
//! runs on the calling thread, and a check's panic reaches the caller whole.
//! The facade rows these replace tested the private token; these test the
//! doors a host holds. The process throttle is shared by every test here, so
//! each row that sends holds one lock.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex, MutexGuard, PoisonError};
use std::thread::{self, ThreadId};
use std::time::{Duration, Instant};

use conformance_backend::{Backend, Canned, Listener};
use thinkthen::{
    Answer, CallOptions, CancelToken, Engine, Entity, Error, ErrorKind, Question, Relate,
};

const DECIDED: &str = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":3,"output_tokens":1}}"#;
const MOST: &str = "4294967295 seconds";
const BOUND: Duration = Duration::from_secs(3);

static SERIAL: Mutex<()> = Mutex::new(());

fn serial() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(PoisonError::into_inner)
}

fn engine(base: &str) -> Engine {
    Engine::builder()
        .base_url(base)
        .and_then(|builder| builder.api_key("sk-public-controls"))
        .map(thinkthen::EngineBuilder::no_cache)
        .and_then(thinkthen::EngineBuilder::build)
        .expect("engine")
}

fn question() -> Question {
    Question::decide("Does this ask for a refund?")
        .expect("question")
        .cut()
}

fn kind<T>(result: &Result<T, Error>) -> Option<ErrorKind> {
    result.as_ref().err().map(Error::kind)
}

fn message<T>(result: Result<T, Error>) -> String {
    result
        .err()
        .map(|error| error.to_string())
        .unwrap_or_default()
}

/// One decide on a fresh listener under these options: its result and the sends.
fn decided(options: CallOptions<'_>) -> (Result<Answer, Error>, usize) {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let result = engine(listener.base()).decide_with(&question(), "Refund me.", options);
    (result, listener.count())
}

#[test]
fn deadline_numbers_follow_the_host_table_and_the_last_call_wins() {
    let none = CallOptions::new();
    assert_eq!(
        message(none.deadline_after(Duration::MAX)),
        format!(
            "a deadline of {} seconds is above the most, {MOST}",
            u64::MAX
        )
    );
    assert!(
        none.deadline_after(Duration::from_secs(4_294_967_295))
            .is_ok()
    );
    let refused = |value: &str, unit: &str| {
        format!("a deadline of {value} {unit} is not -1, 0, or a positive budget of at most {MOST}")
    };
    for value in [
        -0.5,
        -2.0,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        4_294_967_296.0,
    ] {
        let result = none.deadline_seconds(value);
        assert_eq!(kind(&result), Some(ErrorKind::Usage), "{value}");
        assert_eq!(message(result), refused(&value.to_string(), "seconds"));
    }
    // A number longer than 20 characters prints in exponent form.
    for (value, written) in [
        (1e300, "1e300"),
        (-1e-300, "-1e-300"),
        (f64::MAX, "1.7976931348623157e308"),
        (1e19, "10000000000000000000"),
        (1e20, "1e20"),
    ] {
        assert_eq!(
            message(none.deadline_seconds(value)),
            refused(written, "seconds")
        );
    }
    for value in [-2, i64::MIN, 4_294_967_295_001, i64::MAX] {
        let result = none.deadline_millis(value);
        assert_eq!(kind(&result), Some(ErrorKind::Usage), "{value}");
        assert_eq!(message(result), refused(&value.to_string(), "milliseconds"));
    }
    for value in [-1.0, 0.0, 0.5, 4_294_967_295.0] {
        assert!(none.deadline_seconds(value).is_ok(), "{value}");
    }
    for value in [-1, 0, 1, 4_294_967_295_000] {
        assert!(none.deadline_millis(value).is_ok(), "{value}");
    }

    // `0` is spent: the call returns Deadline and sends nothing.
    let spent = [
        none.deadline_seconds(0.0),
        none.deadline_millis(0),
        none.deadline_seconds(30.0)
            .and_then(|o| o.deadline_millis(0)),
        none.deadline_after(BOUND)
            .and_then(|o| o.deadline_seconds(0.0)),
    ];
    for options in spent {
        let (result, sent) = decided(options.expect("options"));
        assert_eq!((kind(&result), sent), (Some(ErrorKind::Deadline), 0));
    }
    // `-1` clears an earlier deadline, and a later budget replaces a spent one.
    let live = [
        none.deadline_seconds(0.0)
            .and_then(|o| o.deadline_seconds(-1.0)),
        none.deadline_millis(0).and_then(|o| o.deadline_millis(-1)),
        none.deadline_millis(0)
            .and_then(|o| o.deadline_after(BOUND)),
    ];
    for options in live {
        let (result, sent) = decided(options.expect("options"));
        assert_eq!((result.ok(), sent), (Some(Answer::Yes), 1));
    }
}

#[test]
fn a_throttle_outside_one_through_thirty_two_is_refused_at_the_step() {
    for value in [0, 33, 255] {
        let result = Engine::builder().throttle(value);
        assert_eq!(kind(&result), Some(ErrorKind::Usage), "{value}");
        assert_eq!(
            message(result),
            "a throttle is a whole number from 1 through 32"
        );
    }
    for value in [1, 32] {
        assert!(Engine::builder().throttle(value).is_ok(), "{value}");
    }
}

/// Shared cases 23 and 24 cover a spent single call.
#[test]
fn a_spent_batch_sends_nothing() {
    let token = CancelToken::new();
    token.cancel();
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let options = CallOptions::new().cancel(&token);
    let rows: Vec<_> = engine
        .filter_with(&question(), ["one", "two"], options)
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(kind(&rows[0]), Some(ErrorKind::Cancelled));
    assert_eq!(listener.count(), 0);
}

/// Ticket 0166: a token fired during a send ends the call cancelled, whatever
/// the reply, and a retry the reply asks for never goes.
#[test]
fn a_token_fired_during_a_send_ends_the_call_cancelled() {
    let _serial = serial();
    for status in [503, 422] {
        let release = Arc::new(Barrier::new(2));
        let held = Arc::clone(&release);
        let first = AtomicUsize::new(0);
        let listener = Listener::answering(move |_| {
            if first.fetch_add(1, Ordering::SeqCst) > 0 {
                return Canned::ok(DECIDED);
            }
            let refused = Canned::status(status, "").after_release(Arc::clone(&held));
            refused.asking("retry-after-ms", "0")
        })
        .expect("listener");
        let (token, engine) = (CancelToken::new(), engine(listener.base()));
        let result = thread::scope(|scope| {
            scope.spawn(|| {
                while listener.count() < 1 {
                    thread::sleep(Duration::from_millis(5));
                }
                token.cancel();
                release.wait();
            });
            let options = CallOptions::new().cancel(&token);
            engine.decide_with(&question(), "Refund me.", options)
        });
        assert_eq!(kind(&result), Some(ErrorKind::Cancelled), "{status}");
        assert_eq!(
            listener.count(),
            1,
            "{status}: nothing was sent after the fire"
        );
    }
}

/// Ticket 0166: a token fired after a batch's last row ends the batch cancelled.
#[test]
fn a_token_fired_before_a_batch_ends_ends_it_cancelled() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let (token, asked) = (CancelToken::new(), question());
    let options = CallOptions::new().cancel(&token);
    let mut rows = engine.decide_many_with(&asked, ["Refund me."], options);
    assert_eq!(
        rows.next().map(|row| row.map(|row| *row.value()).ok()),
        Some(Some(Answer::Yes))
    );
    token.cancel();
    assert_eq!(
        rows.next().as_ref().and_then(kind),
        Some(ErrorKind::Cancelled)
    );
    assert!(rows.next().is_none());
    assert_eq!(listener.count(), 1);
}

/// What a check saw: the thread of each run and the sends at that run.
#[derive(Default)]
struct Runs(Mutex<Vec<(ThreadId, usize)>>);

impl Runs {
    fn record(&self, sent: usize) -> usize {
        let mut runs = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        runs.push((thread::current().id(), sent));
        runs.len()
    }

    fn all_on(&self, caller: ThreadId) -> bool {
        let runs = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        !runs.is_empty() && runs.iter().all(|(thread, _)| *thread == caller)
    }

    fn first_sent(&self) -> Option<usize> {
        let runs = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        runs.first().map(|(_, sent)| *sent)
    }
}

/// Four held sends fill the process throttle; a fifth call waits at the gate.
#[test]
fn a_stop_at_the_throttle_gate_sends_nothing_new_and_sent_work_finishes() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let gated = engine(&format!("{}/arm/held/v1", backend.origin()));
    let asked = question();
    let shared = CancelToken::new();
    thread::scope(|scope| {
        let (gated, asked) = (&gated, &asked);
        let holders: Vec<_> = (0..4)
            .map(|_| {
                let options = CallOptions::new().cancel(&shared);
                scope.spawn(move || gated.decide_with(asked, "Refund me.", options))
            })
            .collect();
        assert_eq!(backend.wait(4), 4, "four sends hold the gate");

        // The caller's check stops the fifth call from the calling thread.
        let runs = Runs::default();
        let check = || runs.record(backend.count()) >= 3;
        let options = CallOptions::new().cancel(&shared).interrupt(&check);
        let result = gated.decide_with(asked, "Refund me too.", options);
        assert_eq!(kind(&result), Some(ErrorKind::Cancelled));
        assert!(
            runs.all_on(thread::current().id()),
            "the check ran on a worker"
        );
        assert_eq!(runs.first_sent(), Some(4));
        // A true check stops its own call alone; the shared token stays clear.
        assert!(!shared.is_cancelled());

        // A token fired from another thread stops a waiting call.
        let token = CancelToken::new();
        let result = thread::scope(|inner| {
            inner.spawn(|| {
                thread::sleep(Duration::from_millis(150));
                token.cancel();
            });
            gated.decide_with(asked, "Refund me three.", CallOptions::new().cancel(&token))
        });
        assert_eq!(kind(&result), Some(ErrorKind::Cancelled));

        backend.release();
        for holder in holders {
            assert_eq!(holder.join().expect("holder").ok(), Some(Answer::Yes));
        }
    });
    assert_eq!(backend.count(), 4, "nothing new was sent");
}

#[test]
fn a_stop_during_a_retry_wait_sends_nothing_new() {
    let _serial = serial();
    // The backend asks for a ten-second wait, so an early end is plain on a loaded machine.
    let busy = Listener::answering(|_| Canned::status(503, "").asking("retry-after", "10"))
        .expect("listener");
    let engine = engine(busy.base());
    let asked = question();

    let runs = Runs::default();
    let check = || runs.record(busy.count()) > 1 && busy.count() == 1;
    let started = Instant::now();
    let result = engine.decide_with(&asked, "Refund me.", CallOptions::new().interrupt(&check));
    assert_eq!(kind(&result), Some(ErrorKind::Cancelled));
    assert!(runs.all_on(thread::current().id()));
    assert_eq!(busy.count(), 1, "the retry after the wait never went");
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "the wait ended early"
    );

    let started = Instant::now();
    let options = CallOptions::new().deadline_after(Duration::from_millis(300));
    let result = engine.decide_with(&asked, "Refund me.", options.expect("options"));
    assert_eq!(kind(&result), Some(ErrorKind::Deadline));
    assert_eq!(busy.count(), 2, "one more first attempt, no retry");
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "the wait ended early"
    );
}

/// The consumer's `a_held_reply_ends_the_call_at_its_deadline` covers a deadline during a held send.
#[test]
fn a_check_runs_before_a_held_send_and_never_during_it() {
    let _serial = serial();
    let asked = question();
    let second = Backend::start().expect("backend");
    let engine = engine(&format!("{}/arm/held/v1", second.origin()));
    let runs = AtomicUsize::new(0);
    let check = || {
        runs.fetch_add(1, Ordering::SeqCst);
        false
    };
    let answer = thread::scope(|scope| {
        let call = scope.spawn(|| {
            engine.decide_with(&asked, "Refund me.", CallOptions::new().interrupt(&check))
        });
        assert_eq!(second.wait(1), 1, "the send is held");
        let before = runs.load(Ordering::SeqCst);
        thread::sleep(Duration::from_millis(300));
        assert_eq!(
            runs.load(Ordering::SeqCst),
            before,
            "a check ran in the send"
        );
        assert!(before >= 1, "the check ran before the send");
        second.release();
        call.join().expect("call")
    });
    assert_eq!(answer.ok(), Some(Answer::Yes));
}

/// Ticket 0143: one-request relations fill the throttle of 4. A host check
/// that fires while 4 are held feeds no fifth, and the 4 finish. With 4
/// rules every relation is already fed, and the check still cancels.
#[test]
fn a_host_interrupt_during_relate_chunks_sends_nothing_new() {
    let _serial = serial();
    let entities = ["gateway", "billing"].map(|name| Entity::new(name, "service").expect("entity"));
    for count in [6, 4] {
        let backend = Backend::start().expect("backend");
        let held = engine(&format!("{}/arm/held/v1", backend.origin()));
        let rules: Vec<_> = (1..=count)
            .map(|n| format!(r#"{{"name":"r{n}","source":"service","target":"service"}}"#))
            .collect();
        let ask = Relate::from_json(&format!(
            r#"{{"version":1,"relate":{{"relations":[{}]}}}}"#,
            rules.join(",")
        ))
        .expect("relate file");
        let runs = Runs::default();
        let check = || {
            runs.record(backend.count());
            backend.count() == 4
        };
        let result = thread::scope(|scope| {
            scope.spawn(|| {
                backend.wait(4);
                thread::sleep(Duration::from_millis(400));
                backend.release();
            });
            held.relate_with(&ask, entities.clone(), CallOptions::new().interrupt(&check))
        });
        assert!(
            runs.all_on(thread::current().id()),
            "a check ran on a worker"
        );
        assert_eq!(kind(&result), Some(ErrorKind::Cancelled), "{count} rules");
        assert_eq!(backend.count(), 4, "{count} rules: nothing new was sent");
    }
}

#[test]
fn a_stop_during_a_batch_or_a_cache_lock_wait_sends_nothing_new() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let batch = engine(&format!("{}/arm/held/v1", backend.origin()));
    let asked = question();
    let texts = ["one", "two", "three", "four", "five", "six"];
    let runs = Runs::default();
    let check = || runs.record(backend.count()) >= 3 && backend.count() == 4;
    let options = CallOptions::new().interrupt(&check);
    let rows: Vec<_> = thread::scope(|scope| {
        scope.spawn(|| {
            backend.wait(4);
            thread::sleep(Duration::from_millis(400));
            backend.release();
        });
        batch.filter_with(&asked, texts, options).collect()
    });
    assert!(
        runs.all_on(thread::current().id()),
        "the check ran on a worker"
    );
    assert_eq!(rows.last().and_then(kind), Some(ErrorKind::Cancelled));
    let kept: Vec<_> = rows.iter().filter_map(|row| row.as_ref().ok()).collect();
    assert!(kept.len() <= 4 && kept.iter().zip(texts).all(|(row, text)| **row == text));
    assert_eq!(backend.count(), 4, "nothing new was sent");

    // A second engine on one cache folder waits for the first's answer.
    let folder = std::env::temp_dir().join(format!("thinkthen-controls-{}", std::process::id()));
    let _gone = std::fs::remove_dir_all(&folder);
    let cached = || {
        Engine::builder()
            .base_url(&format!("{}/arm/held/v1", backend.origin()))
            .and_then(|b| b.api_key("sk-public-controls"))
            .and_then(|b| b.cache_at(&folder))
            .and_then(thinkthen::EngineBuilder::build)
            .expect("engine")
    };
    let (first, second) = (cached(), cached());
    thread::scope(|scope| {
        let owner = scope.spawn(|| first.decide(&asked, "Cache me."));
        assert_eq!(backend.wait(5), 5, "the first engine sends");
        let waited = Runs::default();
        let check = || waited.record(backend.count()) >= 3;
        let options = CallOptions::new().interrupt(&check);
        let result = second.decide_with(&asked, "Cache me.", options);
        assert_eq!(kind(&result), Some(ErrorKind::Cancelled), "{result:?}");
        assert!(waited.all_on(thread::current().id()));
        backend.release();
        assert_eq!(owner.join().expect("owner").ok(), Some(Answer::Yes));
    });
    assert_eq!(backend.count(), 5, "the waiting engine sent nothing");
    assert_eq!(second.usage().requests_sent(), 0);
    let _gone = std::fs::remove_dir_all(&folder);
}

#[test]
fn a_panicking_check_stops_the_call_then_resumes_its_payload_after_the_join() {
    const PAYLOAD: &str = "the host check panicked";
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let gated = engine(&format!("{}/arm/held/v1", backend.origin()));
    let asked = question();
    let runs = AtomicUsize::new(0);
    let check = || {
        if backend.count() == 4 && runs.fetch_add(1, Ordering::SeqCst) >= 2 {
            resume_unwind(Box::new(PAYLOAD));
        }
        false
    };
    let texts = ["one", "two", "three", "four", "five", "six"];
    let caught = thread::scope(|scope| {
        let holders: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| gated.decide(&asked, "Hold the gate.")))
            .collect();
        assert_eq!(backend.wait(4), 4, "four sends hold the gate");
        let single = catch_unwind(AssertUnwindSafe(|| {
            gated.decide_with(&asked, "Refund me.", CallOptions::new().interrupt(&check))
        }));
        // An uncancelled waiter would send once the gate frees.
        backend.release();
        for holder in holders {
            assert_eq!(holder.join().expect("holder").ok(), Some(Answer::Yes));
        }
        single
    });
    let Err(payload) = caught else {
        panic!("the panic never reached the caller");
    };
    assert_eq!(payload.downcast_ref::<&str>(), Some(&PAYLOAD));
    assert_eq!(backend.count(), 4, "nothing was sent after the panic");

    // In a batch, the payload waits until every worker has joined.
    let held = Backend::start().expect("backend");
    let batch = engine(&format!("{}/arm/held/v1", held.origin()));
    let runs = AtomicUsize::new(0);
    let check = || {
        if held.count() == 4 && runs.fetch_add(1, Ordering::SeqCst) >= 2 {
            resume_unwind(Box::new(PAYLOAD));
        }
        false
    };
    let caught = thread::scope(|scope| {
        scope.spawn(|| {
            held.wait(4);
            thread::sleep(Duration::from_millis(400));
            held.release();
        });
        catch_unwind(AssertUnwindSafe(|| {
            batch
                .filter_with(&asked, texts, CallOptions::new().interrupt(&check))
                .count()
        }))
    });
    let Err(payload) = caught else {
        panic!("the panic never reached the caller");
    };
    assert_eq!(payload.downcast_ref::<&str>(), Some(&PAYLOAD));
    assert_eq!(held.count(), 4, "nothing was sent after the panic");
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
    for _ in 0..2 {
        assert_eq!(engine.decide(&asked, "Refund me.").ok(), Some(Answer::Yes));
    }
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
