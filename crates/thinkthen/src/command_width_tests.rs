//! Native process-width and facade proofs moved from command tests.
//! Stays: controlled concurrent requests and private selection counters require native tests.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use crate::engine::http::{Client, Exchange, Key};
use crate::engine::{Cancel, Width, Widths, limits};
use crate::test_deadline::isolated::{child, in_child_at};
mod facade_tests;
const QUIET: Duration = Duration::from_millis(150);
fn in_child(name: &str) {
    in_child_at(&format!("command_width_tests::{name}"));
}
#[test]
fn two_engines_share_one_cap() {
    in_child("shared_cap_child");
}
#[test]
#[ignore = "runs alone in a child process"]
fn shared_cap_child() {
    if child() {
        two_engines_share_the_cap();
    }
}
#[test]
fn width_selection_preserves_omission_and_refuses_conflicts() {
    let widths: &'static Widths = Box::leak(Box::default());
    assert_eq!(widths.select(None).expect("omitted"), Width::new(8).expect("fallback"));
    assert_eq!(widths.selected(), None);
    assert_eq!(widths.select(Width::new(4).ok()).expect("explicit"), Width::new(4).expect("width"));
    assert_eq!(widths.selected(), Width::new(4).ok());
    let active = widths.select(Width::new(8).ok()).expect_err("conflict");
    assert_eq!(Some(active.0), Width::new(4).ok());
    assert!(matches!(Width::new(0), Err(crate::engine::error::Error::Usage(
        "a width is a whole number from 1 through 32"
    ))));
}
/// Answer every question: yes at 0.9, and the first option of any choice.
fn generic(body: &[u8]) -> String {
    let request: Value = serde_json::from_slice(body).expect("request");
    let mut answers = serde_json::Map::new();
    for (name, question) in request["questions"].as_object().expect("questions") {
        let answer = if question["type"] == "noul" {
            json!({"type": "noul", "noul": 0.9})
        } else {
            let options = question["criteria"].as_object().expect("criteria");
            let first = options.keys().next().expect("an option").clone();
            let odds: serde_json::Map<_, _> = options
                .keys()
                .map(|key| (key.clone(), json!(f64::from(u8::from(*key == first)))))
                .collect();
            json!({"type": question["type"], "choice": first, "probabilities": odds})
        };
        answers.insert(name.clone(), answer);
    }
    json!({"model": request["model"], "answers": answers}).to_string()
}

/// A listener that holds every answer until the test lets one go, and says
/// so each time it starts holding one.
struct Held {
    listener: Listener,
    release: Sender<()>,
    holding: Receiver<String>,
}

fn held(busy_once: &'static str) -> Held {
    let (release, released) = channel::<()>();
    let released = Mutex::new(released);
    let (holding_send, holding) = channel();
    let holding_send = Mutex::new(holding_send);
    let busy = Mutex::new(false);
    let listener = Listener::answering(move |body| {
        let text = String::from_utf8_lossy(body).into_owned();
        let mut said_busy = busy.lock().expect("busy flag");
        if text.contains(busy_once) && !*said_busy {
            *said_busy = true;
            return Canned::status(503, "busy").asking("retry-after-ms", "10");
        }
        drop(said_busy);
        let _held = holding_send.lock().expect("sender").send(text);
        // Each token the test sends lets exactly one held answer go.
        let _token = released.lock().expect("release").recv();
        Canned::ok(&generic(body))
    })
    .expect("listener");
    Held {
        listener,
        release,
        holding,
    }
}

/// Hold the first wave until `cap` arrive, then drain after each quiet spell.
/// Fail if more than `cap` are ever held.
/// Return the most held at once and every request body that was held.
fn drain(held: &Held, cap: usize, finished: &AtomicUsize, runs: usize) -> (usize, Vec<String>) {
    let give_up = Instant::now() + Duration::from_secs(120);
    let (mut holding, mut most, mut seen) = (0, 0, Vec::new());
    loop {
        assert!(Instant::now() < give_up, "the run deadlocked");
        match held.holding.recv_timeout(QUIET) {
            Ok(body) => {
                holding += 1;
                most = most.max(holding);
                assert!(
                    holding <= cap,
                    "{holding} requests in flight over a cap of {cap}"
                );
                seen.push(body);
            }
            Err(RecvTimeoutError::Timeout) if holding > 0 && most == cap => {
                for _ in 0..holding {
                    held.release.send(()).expect("release");
                }
                holding = 0;
            }
            Err(RecvTimeoutError::Timeout) if finished.load(Ordering::SeqCst) == runs => break,
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => panic!("the listener stopped"),
        }
    }
    (most, seen)
}

/// One bare attempt through `client`, as a later library engine would send.
fn send_once(client: &Client, url: &str, finished: &AtomicUsize) {
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        headers: &[],
        url,
        body: br#"{"model":"local-1","questions":{"q1":{"type":"noul"}}}"#,
        key: &key,
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
    };
    let sent = client.post_observed(&exchange, &Cancel::default(), || ());
    finished.fetch_add(1, Ordering::SeqCst);
    assert!(sent.is_ok(), "{:?}", sent.err());
}

/// Two clients built with different settings still send under one cap.
fn two_engines_share_the_cap() {
    let held = held("never busy");
    let url = held.listener.url();
    let finished = AtomicUsize::new(0);
    let clients = [30, 31].map(|timeout| {
        Client::new(
            Duration::from_secs(timeout),
            false,
            &limits::process().widths,
        )
    });
    let (most, seen) = thread::scope(|scope| {
        for client in clients.iter().flat_map(|client| [client; 6]) {
            let finished = &finished;
            scope.spawn(move || send_once(client, url, finished));
        }
        drain(&held, 8, &finished, 12)
    });
    assert_eq!(most, 8, "two engines reach the one cap and no more");
    assert_eq!(seen.len(), 12);
    assert_eq!(held.listener.count(), 12);
}

