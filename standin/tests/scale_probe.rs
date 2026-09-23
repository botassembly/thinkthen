//! The reviewer's scale probe (review 4 finding 18, review 5 item 4), in
//! its own binary so no neighbour's sockets or states move the
//! process-wide counts it reads.
//!
//! Six hundred distinct settings values, many times the slot count, each
//! make one call against a loopback backend that keeps the
//! connection, so every state holds a real idle socket. The table that
//! leaked a pool per evicted state went from 4 to 194 open files and 2 to
//! 55 MB resident on this shape. Now each evicted pool closes as soon as
//! its call ends.

#![cfg(target_os = "linux")]

mod common;

use std::time::Duration;

use thinkthen_contract::{Connector, EngineConfig, Question};
use thinkthen_standin::StandinConnector;

/// The question every call asks.
const QUESTION: &str = r#"{"decide":"Does the writer ask for a refund?","threshold":0.5}"#;

/// The table's slot count, from `standin/src/lib.rs`.
const STATE_SLOTS: usize = 64;

#[test]
fn six_hundred_settings_values_hold_descriptors_and_memory() {
    common::wire_only();
    let base = common::answering_backend();
    let connector = StandinConnector;
    let question = Question::from_json(QUESTION).expect("the question parses");
    let before_files = common::open_descriptors();
    let call = |i: u64| {
        let engine = connector
            .connect(&EngineConfig {
                address: Some(base.clone()),
                timeout: Some(Duration::from_millis(1_000 + i)),
                max_retries: Some(0),
                width: Some(1),
                ..EngineConfig::default()
            })
            .expect("the engine connects");
        engine
            .decide(&question, "please refund")
            .expect("the loopback backend answers");
    };
    for i in 0..200 {
        call(i);
    }

    // At most one live state per slot, each with at most one idle
    // connection: three descriptors, the client socket and the backend's
    // end of it with its reading clone.
    let bound = before_files + 3 * STATE_SLOTS + 8;
    let after_files = common::settled_descriptors(bound);
    assert!(
        after_files <= bound,
        "two hundred settings values left {after_files} descriptors open, from {before_files}, bound {bound}"
    );
    // Residency: the first two hundred values fill the table, the
    // allocator, and the backend's threads. Four hundred more values must
    // then cost next to nothing; the leaking table grew about a quarter
    // of a megabyte per value.
    let warm_rss = resident_kilobytes();
    for i in 200..600 {
        call(i);
    }
    let after_rss = resident_kilobytes();
    assert!(
        after_rss <= warm_rss + 4_096,
        "four hundred more settings values grew residency by {} kB, from {warm_rss}",
        after_rss.saturating_sub(warm_rss)
    );
}

/// This process's resident set, in kilobytes, as `/proc` reports it.
fn resident_kilobytes() -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").expect("the status reads");
    let line = status
        .lines()
        .find_map(|line| line.strip_prefix("VmRSS:"))
        .expect("VmRSS is in the status");
    line.trim()
        .trim_end_matches("kB")
        .trim()
        .parse()
        .expect("VmRSS is a number")
}
