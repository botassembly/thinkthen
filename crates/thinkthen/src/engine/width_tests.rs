//! Process width proofs: selection, the attempt gate, and its transitions.
//! The command setup and combined-path proofs live beside the command.

use std::io::{ErrorKind, Read as _, Write as _};
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use std::{fs, io};

use crate::core::{Backend, Evidence, ModelName, Plan, Question, QuestionText};
use crate::engine::error::{Budget, Error, Kind};
use crate::engine::http::{Client, Exchange, HttpAnswer, Key};
use crate::engine::recorder::Recorder;
use crate::engine::request::{Transport, ask_profile};
use crate::engine::usage::Counters;
use crate::engine::{Cancel, Deadline, Permit, Width, WidthActive, Widths};

const SECOND: Duration = Duration::from_secs(1);

/// Long enough that an attempt that should not start would have started.
const QUIET: Duration = Duration::from_millis(150);

fn widths() -> &'static Widths {
    Box::leak(Box::default())
}

fn width(value: u64) -> Width {
    Width::new(value).expect("a width from 1 through 32")
}

fn held(widths: &'static Widths, count: usize) -> Vec<Permit<'static>> {
    (0..count)
        .map(|_| widths.acquire(&Cancel::default()).expect("free room"))
        .collect()
}

/// One waiter: it reports that it blocked, then the active count it saw on
/// entry, and it holds its permit until the test lets it go.
struct Waiter {
    release: Sender<()>,
    thread: thread::JoinHandle<()>,
}

/// Which waiter entered, and how many attempts held a permit as it did.
type Entry = (usize, usize);

fn waiter(
    widths: &'static Widths,
    blocked: &Sender<()>,
    entered: &Sender<Entry>,
    name: usize,
) -> Waiter {
    let (release, released) = channel::<()>();
    let cancel = Cancel::observed(blocked.clone());
    let entered = entered.clone();
    let thread = thread::spawn(move || {
        let permit = widths.acquire(&cancel).expect("the waiter enters");
        entered
            .send((name, widths.active()))
            .expect("entry observed");
        let _released = released.recv();
        drop(permit);
    });
    Waiter { release, thread }
}

fn blocked(signals: &Receiver<()>, count: usize) {
    for _ in 0..count {
        signals
            .recv_timeout(SECOND * 2)
            .expect("the waiter blocked on a full gate");
    }
}

fn nobody_entered(entered: &Receiver<Entry>) {
    assert_eq!(
        entered.recv_timeout(QUIET),
        Err(RecvTimeoutError::Timeout),
        "no waiter may enter while the gate is full"
    );
}

#[test]
fn implicit_engines_never_select_and_follow_the_first_explicit_width() {
    let widths = widths();
    // The lazy convenience engine and a builder with no width pass `None`.
    for _ in 0..3 {
        assert_eq!(widths.select(None), Ok(Width::FALLBACK));
    }
    assert_eq!(widths.selected(), None);
    assert_eq!(widths.select(Some(width(8))), Ok(width(8)));
    assert_eq!(widths.select(None), Ok(width(8)));
    assert_eq!(widths.selected(), Some(width(8)));
}

#[test]
fn the_first_explicit_width_wins_and_only_a_different_one_is_refused() {
    let table = [1, 4, 8, 32];
    for first in table {
        let widths = widths();
        assert_eq!(widths.select(Some(width(first))), Ok(width(first)));
        assert_eq!(widths.select(Some(width(first))), Ok(width(first)));
        for other in table.into_iter().filter(|other| *other != first) {
            let refused = widths.select(Some(width(other))).expect_err("a conflict");
            assert_eq!(
                refused.to_string(),
                format!(
                    "width {first} is already active for this process; use width {first} or drop the width argument"
                )
            );
        }
        assert_eq!(widths.selected(), Some(width(first)));
    }
    assert_eq!(
        WidthActive(Width::FALLBACK).to_string(),
        "width 4 is already active for this process; use width 4 or drop the width argument"
    );
}

#[test]
fn two_racing_first_widths_select_exactly_one_and_the_loser_names_it() {
    for _ in 0..50 {
        let widths = widths();
        let start = Arc::new(Barrier::new(2));
        let racers = [2, 16].map(|value| {
            let start = Arc::clone(&start);
            thread::spawn(move || {
                start.wait();
                widths.select(Some(width(value)))
            })
        });
        let results = racers.map(|racer| racer.join().expect("racer"));
        let winner = widths.selected().expect("one width won");
        assert_eq!(
            results
                .iter()
                .filter(|result| **result == Ok(winner))
                .count(),
            1
        );
        assert_eq!(
            results
                .iter()
                .filter(|result| **result == Err(WidthActive(winner)))
                .count(),
            1
        );
    }
}

#[test]
fn a_width_exists_only_for_1_through_32() {
    for value in [0, 33, 256, u64::MAX] {
        let refused = Width::new(value).expect_err("outside the range");
        assert_eq!(refused.kind(), Kind::Usage);
        assert!(
            matches!(refused, Error::Usage(message) if message == "a width is a whole number from 1 through 32"),
            "{value}"
        );
    }
    assert_eq!(width(1).get(), 1);
    assert_eq!(width(32).get(), 32);
}

#[test]
fn a_lower_first_width_holds_every_waiter_until_the_old_attempts_drain() {
    let widths = widths();
    let mut fallback = held(widths, 4);
    let (blocked_send, blocked_on) = channel();
    let (entered_send, entered) = channel();
    let mut waiters: Vec<Waiter> = (0..2)
        .map(|name| waiter(widths, &blocked_send, &entered_send, name))
        .collect();
    blocked(&blocked_on, 2);

    assert_eq!(widths.select(Some(width(1))), Ok(width(1)));
    waiters.extend((2..4).map(|name| waiter(widths, &blocked_send, &entered_send, name)));
    blocked(&blocked_on, 2);

    while fallback.len() > 1 {
        drop(fallback.pop());
        nobody_entered(&entered);
        assert_eq!(widths.active(), fallback.len());
    }
    drop(fallback.pop());
    for _ in 0..waiters.len() {
        let (name, seen) = entered.recv_timeout(SECOND * 2).expect("drained room");
        assert_eq!(seen, 1, "one attempt at a time after width 1");
        nobody_entered(&entered);
        waiters[name].release.send(()).expect("release");
    }
    for waiter in waiters {
        waiter.thread.join().expect("waiter");
    }
    assert_eq!(widths.active(), 0);
}

#[test]
fn a_higher_first_width_wakes_waiters_without_passing_it() {
    let widths = widths();
    let fallback = held(widths, 4);
    let (blocked_send, blocked_on) = channel();
    let (entered_send, entered) = channel();
    let waiters: Vec<Waiter> = (0..6)
        .map(|name| waiter(widths, &blocked_send, &entered_send, name))
        .collect();
    blocked(&blocked_on, 6);
    nobody_entered(&entered);

    assert_eq!(widths.select(Some(width(8))), Ok(width(8)));
    for _ in 0..4 {
        let (_, seen) = entered
            .recv_timeout(SECOND * 2)
            .expect("new room wakes one");
        assert!(seen <= 8, "{seen} attempts at width 8");
    }
    nobody_entered(&entered);
    assert_eq!(widths.active(), 8);

    drop(fallback);
    for _ in 0..2 {
        let (_, seen) = entered.recv_timeout(SECOND * 2).expect("freed room");
        assert!(seen <= 8, "{seen} attempts at width 8");
    }
    for waiter in waiters {
        let _unheld = waiter.release.send(());
        waiter.thread.join().expect("waiter");
    }
    assert_eq!(widths.active(), 0);
}

/// A listener nothing should reach, and the URL that names it.
fn silent() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    (listener, url)
}

fn reached(listener: &TcpListener) -> bool {
    !matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock)
}

fn post(
    widths: &'static Widths,
    url: &str,
    cancel: &Cancel,
    attempts: &AtomicUsize,
) -> Result<HttpAnswer, Error> {
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url,
        body: b"{}",
        key: &key,
        max_retries: 2,
        retry_wait: Duration::from_millis(10),
    };
    Client::new(SECOND * 2, false)
        .gated(widths)
        .post_observed(&exchange, cancel, || {
            attempts.fetch_add(1, Ordering::SeqCst);
        })
}

#[test]
fn a_waiter_cancelled_behind_a_full_gate_sends_nothing_and_holds_nothing() {
    let widths = widths();
    assert_eq!(widths.select(Some(width(1))), Ok(width(1)));
    let full = held(widths, 1);
    let (listener, url) = silent();
    let (blocked_send, blocked_on) = channel();
    let cancel = Cancel::observed(blocked_send);
    let attempts = AtomicUsize::new(0);

    let result = thread::scope(|scope| {
        let waiting = scope.spawn(|| post(widths, &url, &cancel, &attempts));
        blocked(&blocked_on, 1);
        cancel.fire();
        waiting.join().expect("waiter")
    });

    assert!(matches!(result, Err(Error::Cancelled)));
    assert_eq!(attempts.load(Ordering::SeqCst), 0);
    assert!(!reached(&listener));
    assert_eq!(widths.active(), 1);
    drop(full);
    assert_eq!(widths.active(), 0);
}

#[test]
fn a_deadline_spent_behind_a_full_gate_sends_nothing_and_holds_nothing() {
    let widths = widths();
    assert_eq!(widths.select(Some(width(1))), Ok(width(1)));
    let full = held(widths, 1);
    let (listener, url) = silent();
    let budget = Duration::from_millis(200);
    let cancel = Cancel::default().with_deadline(Deadline::after(budget));
    let attempts = AtomicUsize::new(0);

    let result = post(widths, &url, &cancel, &attempts);

    assert!(matches!(result, Err(Error::Deadline(Budget(spent))) if spent == budget));
    assert_eq!(attempts.load(Ordering::SeqCst), 0);
    assert!(!reached(&listener));
    assert_eq!(widths.active(), 1);
    drop(full);
}

fn busy(wait_ms: u64) -> String {
    format!(
        "HTTP/1.1 503 Busy\r\nretry-after-ms: {wait_ms}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
    )
}

fn answer(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    )
}

/// Serve these raw responses in order, one per connection, and announce each.
fn serving(responses: Vec<String>) -> (String, Receiver<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    let (answered, served) = channel();
    thread::spawn(move || {
        for response in responses {
            let (mut stream, _) = listener.accept().expect("request");
            let mut request = [0_u8; 4096];
            let _read = stream.read(&mut request).expect("request bytes");
            stream.write_all(response.as_bytes()).expect("response");
            let _observed = answered.send(());
        }
    });
    (url, served)
}

#[test]
fn a_retry_gives_its_permit_back_for_the_wait_and_takes_a_new_one() {
    let widths = widths();
    assert_eq!(widths.select(Some(width(1))), Ok(width(1)));
    let (url, busy) = serving(vec![busy(600), answer("{}")]);
    let attempts = AtomicUsize::new(0);
    let during_attempts = AtomicUsize::new(0);

    let result = thread::scope(|scope| {
        let posting = scope.spawn(|| {
            let key = Key::of("sk-test-value");
            let exchange = Exchange {
                url: &url,
                body: b"{}",
                key: &key,
                max_retries: 1,
                retry_wait: Duration::from_millis(10),
            };
            Client::new(SECOND * 2, false).gated(widths).post_observed(
                &exchange,
                &Cancel::default(),
                || {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    during_attempts.fetch_max(widths.active(), Ordering::SeqCst);
                },
            )
        });
        busy.recv_timeout(SECOND * 2).expect("the busy answer");
        // The retry waits 600 ms; this permit must come free well before.
        let within = Cancel::default().with_deadline(Deadline::after(Duration::from_millis(400)));
        let taken = widths.acquire(&within).expect("the wait holds no permit");
        drop(taken);
        posting.join().expect("poster")
    });

    let answer = result.expect("the retry answered");
    assert_eq!(answer.requests_sent, 2);
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
    assert_eq!(during_attempts.load(Ordering::SeqCst), 1);
    assert_eq!(widths.active(), 0);
}

fn plan() -> Plan {
    Plan::new(
        Evidence::new("evidence").expect("evidence"),
        ModelName::new("jev-latest").expect("model"),
        vec![Question::Decide {
            text: QuestionText::new("Is this relevant?").expect("question"),
            yes: None,
            no: None,
        }],
    )
    .expect("plan")
}

fn ask(
    url: &str,
    recorder: &Recorder,
    widths: &'static Widths,
    cancel: &Cancel,
) -> Result<bool, Error> {
    let backend = Backend::resolve(Some(url), None, "jev-latest").expect("backend");
    let client = Client::new(SECOND * 2, false).gated(widths);
    let usage = Counters::default();
    let transport = Transport {
        client: &client,
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
        usage: &usage,
    };
    ask_profile::<Error>(&backend, &plan(), None, recorder, cancel, transport, || {
        Ok(Key::of("sk-test-value"))
    })
    .map(|answered| answered.replayed)
}

#[test]
fn a_replayed_answer_takes_no_permit() -> io::Result<()> {
    let folder =
        std::env::temp_dir().join(format!("thinkthen-width-replay-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&folder);
    let (url, _served) = serving(vec![answer(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
    )]);
    let open = widths();
    let recording = Recorder::of(Some(&folder), None).expect("recorder");
    assert!(matches!(
        ask(&url, &recording, open, &Cancel::default()),
        Ok(false)
    ));

    let full = widths();
    assert_eq!(full.select(Some(width(1))), Ok(width(1)));
    let _held = held(full, 1);
    let replay = Recorder::of(None, Some(&folder)).expect("replay");
    let within = Cancel::default().with_deadline(Deadline::after(Duration::from_millis(300)));
    assert!(matches!(ask(&url, &replay, full, &within), Ok(true)));
    fs::remove_dir_all(folder)
}
