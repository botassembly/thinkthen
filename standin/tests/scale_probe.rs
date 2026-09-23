//! The reviewer's scale probes (review findings 18 and the width ceiling,
//! 2026-09-23), in their own binary so no neighbour's sockets or states
//! move the process-wide counters the probes read.
//!
//! Before the fix the state table retired states by freeing their slot
//! boxes while readers still held the pointers, and past the slot count
//! every retired pool leaked its sockets: two hundred distinct settings
//! values took the old table from 4 to 194 open files and 2 to 55 MB
//! resident. After the fix the descriptors settle behind the grace and
//! the sixteen bytes a tombstone box leaks per eviction stay a rounding
//! error beside that.

use std::sync::Arc;
use std::time::Duration;

use thinkthen_contract::{Connector, Engine, EngineConfig, ErrorKind, Question};
use thinkthen_standin::{StandinConnector, retired_boxes};

/// The question every probe asks.
const QUESTION: &str = r#"{"decide":"Does the writer ask for a refund?","threshold":0.5}"#;

/// Two hundred distinct settings values — more than three times the slot
/// count — churn the whole table; the retired pools must close behind the
/// grace and the process must not grow descriptors or residency.
#[cfg(target_os = "linux")]
#[test]
fn two_hundred_settings_values_hold_descriptors_and_memory() {
    let connector = StandinConnector;
    let question = Question::from_json(QUESTION).expect("the question parses");
    let before_files = open_descriptors();
    let before_rss = resident_kilobytes();
    let boxes_before = retired_boxes();

    let mut engines = Vec::with_capacity(200);
    for i in 0..200_u64 {
        // Distinct timeouts are distinct settings values: the home slot
        // hashes differently and the table evicts as the values walk.
        engines.push(
            connector
                .connect(&EngineConfig {
                    timeout: Some(Duration::from_millis(1 + i % 200)),
                    max_retries: Some(0),
                    width: Some(1),
                    ..EngineConfig::default()
                })
                .expect("the engine connects"),
        );
    }
    // One call per engine builds and caches its state; the address is the
    // default loopback with nothing listening, so every send is refused at
    // once and no wire time is spent.
    for engine in &engines {
        let _ = engine.decide(&question, "churn this state");
    }
    drop(engines);

    // The grace (50 ms under test) passes; a further lookup sweeps the
    // retired stack, closing the evicted pools.
    std::thread::sleep(Duration::from_millis(300));
    let sweeper = connector
        .connect(&EngineConfig {
            timeout: Some(Duration::from_millis(1)),
            max_retries: Some(0),
            width: Some(1),
            ..EngineConfig::default()
        })
        .expect("the sweeper connects");
    let _ = sweeper.decide(&question, "sweep the retired");

    let after_files = open_descriptors();
    let after_rss = resident_kilobytes();
    assert!(
        after_files <= before_files + 8,
        "two hundred settings values left {after_files} descriptors open, from {before_files}"
    );
    assert!(
        after_rss <= before_rss + 8_192,
        "two hundred settings values grew residency by {} kB, from {before_rss}",
        after_rss.saturating_sub(before_rss)
    );
    // The churn retired boxes — the bounded sixteen-byte leak — and a
    // couple hundred of them are kilobytes, not the megabytes of a leaked
    // pool.
    let retired = retired_boxes().saturating_sub(boxes_before);
    assert!(retired >= 100, "the churn retired {retired} boxes");
}

/// The width ceiling: a width beyond the recorded bound is refused with
/// the usage kind before any thread or allocation is attempted (review
/// finding, 2026-09-23: width 100,000 panicked to rc 6 at 322 MB).
#[test]
fn an_absurd_width_is_refused_not_panicked() {
    let connector = StandinConnector;
    let question = Question::from_json(QUESTION).expect("the question parses");
    let engine: Arc<dyn Engine> = connector
        .connect(&EngineConfig {
            timeout: Some(Duration::from_millis(1)),
            max_retries: Some(0),
            width: Some(100_000),
            ..EngineConfig::default()
        })
        .expect("the engine itself connects: the ceiling fires on the call");
    let error = engine
        .decide(&question, "i want a refund")
        .expect_err("the absurd width is refused");
    assert_eq!(error.kind, ErrorKind::Usage, "got: {error}");
    assert!(
        error.to_string().contains("ceiling"),
        "the refusal names the ceiling: {error}"
    );
    // The ceiling itself builds and reaches the wire, so the bound refuses
    // only what is beyond it.
    let at_ceiling = connector
        .connect(&EngineConfig {
            timeout: Some(Duration::from_millis(1)),
            max_retries: Some(0),
            width: Some(4_096),
            ..EngineConfig::default()
        })
        .expect("the ceiling builds");
    match at_ceiling.decide(&question, "refused fast") {
        Err(error) => assert_ne!(error.kind, ErrorKind::Usage, "at the ceiling the failure must be the wire, not the bound: {error}"),
        Ok(_) => panic!("the default loopback has no listener; the ceiling call must fail on the wire"),
    }
}

/// How many descriptors this process holds open.
#[cfg(target_os = "linux")]
fn open_descriptors() -> usize {
    std::fs::read_dir("/proc/self/fd")
        .expect("the descriptor table reads")
        .count()
}

/// This process's resident set, in kilobytes, as `/proc` reports it.
#[cfg(target_os = "linux")]
fn resident_kilobytes() -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").expect("the status reads");
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            return rest
                .trim()
                .trim_end_matches("kB")
                .trim()
                .parse()
                .expect("VmRSS is a number");
        }
    }
    panic!("VmRSS is missing from the status");
}
