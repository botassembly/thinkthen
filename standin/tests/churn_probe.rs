//! The reviewer's churn shape, as a standing probe (review 4 finding 1,
//! review 5 item 1): thirty-two threads build fresh engines with timeouts
//! drawn from nearly a thousand distinct values, far past the sixty-four
//! table slots, so every call publishes, evicts, and looks up while the
//! others do the same. Every engine talks to a loopback backend that
//! answers and keeps its connection, so each state holds a real socket.
//!
//! The lock-free table freed or overwrote slot boxes other threads were
//! still reading: a heap-use-after-free under AddressSanitizer, nine
//! data races under ThreadSanitizer, and a SIGSEGV at the C door. The
//! table now reaches every slot under its lock and hands each reader its
//! own counted reference. Run under ThreadSanitizer this probe reports no
//! race; the gate runs it plain, where it proves the evicted pools close.

#![cfg(target_os = "linux")]

mod common;

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use thinkthen_contract::{Connector, Engine, EngineConfig, Question};
use thinkthen_standin::StandinConnector;

/// The question every call asks.
const QUESTION: &str = r#"{"decide":"Does the writer ask for a refund?","threshold":0.5}"#;

/// The table's slot count, from `standin/src/lib.rs`.
const STATE_SLOTS: usize = 64;

#[test]
fn thirtytwo_threads_churn_the_table_without_a_crash_or_a_leak() {
    common::wire_only();
    let base = common::answering_backend();
    let connector = StandinConnector;
    let question = Question::from_json(QUESTION).expect("the question parses");
    let before = common::open_descriptors();

    let answered = AtomicU64::new(0);
    std::thread::scope(|scope| {
        for worker in 0..32_u64 {
            let (connector, question, base, answered) = (&connector, &question, &base, &answered);
            scope.spawn(move || {
                for round in 0..25_u64 {
                    for probe in 0..3_u64 {
                        let timeout = Duration::from_millis(
                            1_000 + (worker * 7919 + round * 31 + probe * 17) % 977,
                        );
                        let engine: Arc<dyn Engine> = connector
                            .connect(&EngineConfig {
                                address: Some(base.clone()),
                                timeout: Some(timeout),
                                max_retries: Some(0),
                                width: Some(1),
                                ..EngineConfig::default()
                            })
                            .expect("the engine connects");
                        let note = format!("please refund {worker} {round} {probe}");
                        engine
                            .decide(question, &note)
                            .expect("the loopback backend answers");
                        answered.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }
    });
    assert_eq!(
        answered.load(Ordering::Relaxed),
        32 * 25 * 3,
        "every call answered"
    );

    // At most one live state per slot, each with at most one idle
    // connection: three descriptors, the client socket and the backend's
    // end of it with its reading clone. Every
    // evicted state closed its pool when its last caller let go; the
    // leaking table held one pool per distinct value, about two thousand
    // descriptors here.
    let bound = before + 3 * STATE_SLOTS + 8;
    let settled = common::settled_descriptors(bound);
    assert!(
        settled <= bound,
        "descriptors went {before} to {settled} across the churn, bound {bound}"
    );
}
