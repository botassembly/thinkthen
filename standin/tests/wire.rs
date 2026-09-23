//! Wire tests for the stand-in engine, against the live stub.
//!
//! Every test here needs the stub from
//! the in-repo `tools/wire-stub` running on the loopback, pointed
//! at by `ENGINE_BASE_URL`, with `ENGINE_WIDTH` naming the width under
//! test. A test skips with a message when the stub is not up, so the suite
//! stays green offline.
//!
//! The fork test is experiment 211's point, carried onto the contract: a
//! child forked after the parent's first wire call must answer on the
//! wire, proving the pid check rebuilds the pool and the gate.

use std::time::Duration;
use std::time::Instant;

use thinkthen_contract::{Answer, Cancel, Engine, ErrorKind, Options, Question};
use thinkthen_standin::BlockingEngine;

const QUESTION: &str = r#"{"decide":"Does the writer ask for a refund?","threshold":0.9}"#;

/// One test on the stub at a time: the tests read the stub's shared
/// counters, so a neighbor's posts would read as this test's traffic.
static SEAT: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn seat() -> std::sync::MutexGuard<'static, ()> {
    SEAT.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The width the engine runs at, as the environment names it: the
/// settled spelling first, the deprecated one beside it, so a suite
/// running under `THINKTHEN_WIDTH` reads the same width the engine does
/// (review finding 22, 2026-09-23: a verb suite skipped under the
/// settled spelling because only `ENGINE_` was read).
fn width() -> u64 {
    let spelling = |name: &str| {
        std::env::var(name)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .and_then(|value| value.parse().ok())
    };
    spelling("THINKTHEN_WIDTH")
        .or_else(|| spelling("ENGINE_WIDTH"))
        .unwrap_or(4)
}

/// A caller for the stub's own doors, apart from the engine's pool.
fn stub_client() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_millis(800)))
        .max_redirects(0)
        .build()
        .into()
}

/// Where the stub's doors sit, following `THINKTHEN_BASE_URL` with the
/// deprecated `ENGINE_BASE_URL` beside it, so a suite decides its skip on
/// the same address the engine would use (review finding 22, 2026-09-23:
/// a suite skipped although a backend was reachable under the settled
/// spelling).
fn stub(path: &str) -> String {
    let read = |name: &str| {
        std::env::var(name)
            .ok()
            .filter(|value| !value.trim().is_empty())
    };
    let base = read("THINKTHEN_BASE_URL")
        .or_else(|| read("ENGINE_BASE_URL"))
        .unwrap_or_else(|| "http://127.0.0.1:8091/v1".into());
    format!("{base}{path}")
}

/// Whether the stub answers its stats door.
fn stub_up() -> bool {
    stub_client().get(stub("/stats")).call().is_ok()
}

/// The stub's counters as JSON.
fn stats() -> serde_json::Value {
    let text = stub_client()
        .get(stub("/stats"))
        .call()
        .expect("the stub answers")
        .body_mut()
        .read_to_string()
        .expect("the stats read");
    serde_json::from_str(&text).expect("the stats are JSON")
}

/// Clear the stub's counters.
fn reset() {
    stub_client().post(stub("/reset")).send(&[]).expect("the stub resets");
}

/// The request count alone.
fn requests() -> u64 {
    stats()["requests"].as_u64().expect("requests is a number")
}

/// A cancel mid-batch stops new requests, returns the cancelled kind, and
/// the stub's request count then holds still: the in-flight round finished
/// and nothing new left.
#[test]
fn a_cancel_mid_batch_stops_new_requests() {
    let _seat = seat();
    if !stub_up() {
        eprintln!("skip: the stub is not running");
        return;
    }
    reset();
    let tt = BlockingEngine::from_env();
    let question = Question::from_json(QUESTION).expect("the question parses");
    let records: Vec<String> = (0..40).map(|place| format!("note {place}")).collect();
    let records: Vec<&str> = records.iter().map(String::as_str).collect();
    let token = Cancel::new();
    let trip = token.clone();
    let mut ticks = 0;
    let started = Instant::now();
    let stopped = tt.decide_many_opts(
        &question,
        &records,
        Options::new().cancel(&token),
        Some(&mut || {
            ticks += 1;
            if ticks >= 2 {
                trip.cancel();
            }
        }),
    );
    let wall = started.elapsed();
    assert_eq!(stopped.unwrap_err().kind, ErrorKind::Cancelled, "the batch stops");
    eprintln!(
        "cancel: 40 records, token set at tick 2: wall {wall:?}, ticks {ticks}, requests at return {}",
        requests()
    );
    assert!(wall < Duration::from_secs(5), "the stop lands fast, took {wall:?}");
    let at_return = requests();
    std::thread::sleep(Duration::from_millis(1_500));
    let after = requests();
    assert_eq!(at_return, after, "no request leaves after the cancel drains");
    assert!(at_return <= 40, "no more requests than records left: {at_return}");
}

/// A hundred single-row calls at once, from one process, stay at the
/// width.
#[test]
fn one_hundred_single_calls_stay_at_the_width() {
    let _seat = seat();
    if !stub_up() {
        eprintln!("skip: the stub is not running");
        return;
    }
    reset();
    let width = width();
    let tt = BlockingEngine::from_env();
    let question = Question::from_json(QUESTION).expect("the question parses");
    let started = Instant::now();
    std::thread::scope(|scope| {
        for call in 0..100 {
            let tt = &tt;
            let question = &question;
            scope.spawn(move || {
                let answer = tt.decide(question, &format!("solo note {call}")).expect("a call");
                assert_eq!(answer, Answer::No, "a plain note is no");
            });
        }
    });
    let wall = started.elapsed();
    let max_in_flight = stats()["max_in_flight"].as_u64().expect("max is a number");
    assert!(max_in_flight <= width, "the gate held {width}, saw {max_in_flight}");
    // 100 calls at a 300 ms delay through the width is about four rounds
    // at 32, more at 4; generous for slow machines, tight enough to prove
    // width.
    let rounds = (100 + width - 1) / width;
    let budget = Duration::from_millis(300 * rounds * 4 + 2_000);
    assert!(wall < budget, "the calls ran wide, took {wall:?}");
}

/// A child forked after the parent's first wire call answers on the wire.
#[test]
fn a_forked_child_answers_on_the_wire() {
    let _seat = seat();
    if !stub_up() {
        eprintln!("skip: the stub is not running");
        return;
    }
    let tt = BlockingEngine::from_env();
    let question = Question::from_json(QUESTION).expect("the question parses");
    let warm = tt.decide(&question, "the parent warms the pool").expect("the parent call");
    assert_eq!(warm, Answer::No, "a plain note is no");
    let child = unsafe { libc::fork() };
    assert!(child >= 0, "the fork happens");
    if child == 0 {
        // The child: answer or die by the alarm, never hang the runner.
        unsafe { libc::alarm(10) };
        let answered = Question::from_json(QUESTION).and_then(|question| {
            BlockingEngine::from_env().decide(&question, "the child wants a refund")
        });
        let code = match answered {
            Ok(Answer::Yes) => 0,
            Ok(_) => 2,
            Err(_) => 3,
        };
        unsafe { libc::_exit(code) };
    }
    let mut status = 0;
    unsafe { libc::waitpid(child, &mut status, 0) };
    assert!(libc::WIFEXITED(status), "the child exits, status {status}");
    assert_eq!(libc::WEXITSTATUS(status), 0, "the child answered on the wire");
}
