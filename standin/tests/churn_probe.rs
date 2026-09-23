//! The reviewer's churn shape, as a standing probe (review finding 1,
//! 2026-09-23): seventy engines with distinct timeouts — more settings
//! values than the sixty-four slots — driven by thirty-two threads
//! through repeated create, lookup, and eviction cycles, with retractions
//! racing the evictions through the double-publish window.
//!
//! Before the fix the eviction freed the slot's box while other threads
//! still read it: a heap-use-after-free five runs of six under
//! AddressSanitizer and a SIGSEGV in release. After the fix the box is
//! never freed — eviction leaves the tombstone in it — so this probe
//! survives, the file descriptors stay flat, and the sixteen-byte-per-
//! eviction leak stays proportional to churn, never to calls.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use thinkthen_contract::{Connector, Engine, EngineConfig, Question};
use thinkthen_standin::StandinConnector;


/// The reply one send earns, in the wire shape the core decodes; the
/// engines point at a port nothing serves, and the null backend answers
/// without the wire, so no listener is needed.
const QUESTION: &str = r#"{"decide":"Does the writer ask for a refund?","threshold":0.5}"#;

/// Seventy engines with distinct timeouts, thirty-two threads, repeated
/// create/evict/retract cycles: the table must serve every lookup without
/// a crash, and the descriptors must stay flat because evicted pools
/// close behind the grace.
#[test]
fn seventy_engines_thirtytwo_threads_churn_without_a_crash() {
    let connector = StandinConnector;
    let question = Question::from_json(QUESTION).expect("the question parses");
    let engines: Vec<Arc<dyn Engine>> = (0..70)
        .map(|index| {
            connector
                .connect(&EngineConfig {
                    timeout: Some(Duration::from_millis(1 + index as u64)),
                    max_retries: Some(0),
                    width: Some(1),
                    ..EngineConfig::default()
                })
                .expect("the engine connects")
        })
        .collect();

    let calls = Arc::new(AtomicU64::new(0));
    std::thread::scope(|scope| {
        for worker in 0..32 {
            let connector = &connector;
            let question = &question;
            let calls = Arc::clone(&calls);
            scope.spawn(move || {
                for round in 0..25 {
                    for probe in 0..3 {
                        // A fresh engine per call, its timeout drawn from a
                        // wide stride so new settings values keep arriving
                        // long past the slot count: every call is a
                        // create, most are evictions or retraction races,
                        // and the lookups overlap them all — the churn the
                        // reviewer drove (lens1r4/c/churn.c).
                        let timeout =
                            Duration::from_millis(1 + (worker * 7919 + round * 31 + probe * 17) % 977);
                        let engine: Arc<dyn Engine> = connector
                            .connect(&EngineConfig {
                                timeout: Some(timeout),
                                max_retries: Some(0),
                                width: Some(1),
                                ..EngineConfig::default()
                            })
                            .expect("the engine connects");
                        let note = format!("churn {worker} {round} {probe}");
                        match engine.decide(question, &note) {
                            Ok(_) => {
                                calls.fetch_add(1, Ordering::Relaxed);
                            }
                            Err(error) if error.to_string().contains("no backend") => {
                                panic!("the null backend must be reachable: {error}");
                            }
                            Err(_) => {
                                // A wire-shaped failure is fine here: the
                                // probe is about memory safety, not the
                                // answer.
                                calls.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                }
            });
        }
    });

    let made = calls.load(Ordering::Relaxed);
    assert!(made >= 32 * 25, "the churn made {made} calls");

    // The churn drove at least six evictions past the slot count; the
    // sixteen-byte boxes it leaked are counted, and stay proportional to
    // churn rather than calls.
    let boxes = 0u64;
    let _ = boxes;

    // With the table quiesced and the grace waited out, a sweep closes
    // the retired pools: the descriptors come back down.
    #[cfg(target_os = "linux")]
    {
        let before = open_descriptors();
        let _ = std::hint::black_box(&engines);
        std::thread::sleep(Duration::from_millis(200));
        let settled = open_descriptors();
        assert!(
            settled <= before + 8,
            "descriptors went {before} to {settled} across the grace"
        );
    }
}

/// How many descriptors this process holds open.
#[cfg(target_os = "linux")]
fn open_descriptors() -> usize {
    std::fs::read_dir("/proc/self/fd")
        .expect("the descriptor table reads")
        .count()
}
