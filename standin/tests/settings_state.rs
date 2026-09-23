//! One state per settings value (review finding 9, 2026-09-22): two engines
//! whose settings differ never share a gate or a pool, a width of 1 holds
//! across them, and the file descriptors stay flat over repeated calls.
//!
//! Before the fix a single process-wide state was rebuilt whenever the
//! settings changed, so alternating calls between two engines built a fresh
//! gate (and a fresh pool) every time: width 1 allowed many concurrent
//! calls and every retired pool leaked its sockets.

mod common;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use thinkthen_contract::{Connector, Engine, EngineConfig, Question};
use thinkthen_standin::StandinConnector;

/// The reply one send earns, in the wire shape the core decodes.
const REPLY: &[u8] = br#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.97}},"usage":{"input_tokens":10,"output_tokens":2}}"#;

/// A listener that answers after a delay, counting how many connections are
/// live at once and the highest that count reached.
struct SlowListener {
    address: String,
    live: Arc<AtomicUsize>,
    peak: Arc<AtomicUsize>,
}

impl SlowListener {
    fn start(delay: Duration) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("binds");
        let address = listener.local_addr().expect("an address");
        let live = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let seen = Arc::clone(&live);
        let top = Arc::clone(&peak);
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.expect("accepts");
                let seen = Arc::clone(&seen);
                let top = Arc::clone(&top);
                std::thread::spawn(move || {
                    let mut buffer = [0_u8; 4096];
                    let _ = stream.read(&mut buffer);
                    // Count concurrency only across the delayed window: a
                    // width-1 gate holds the permit for the whole request,
                    // so overlapping sleeps mean the gate leaked, while the
                    // old accept-to-close count also caught the tail of a
                    // finished response under load (review finding 18,
                    // 2026-09-23: two failures in fifty).
                    let now = seen.fetch_add(1, Ordering::SeqCst) + 1;
                    top.fetch_max(now, Ordering::SeqCst);
                    std::thread::sleep(delay);
                    seen.fetch_sub(1, Ordering::SeqCst);
                    let head = format!(
                        "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                        REPLY.len()
                    );
                    let _ = stream.write_all(head.as_bytes());
                    let _ = stream.write_all(REPLY);
                    let _ = stream.flush();
                });
            }
        });
        Self {
            address: format!("http://{address}/v1"),
            live,
            peak,
        }
    }

    /// The base URL a config names.
    fn base(&self) -> String {
        self.address.clone()
    }

    /// The highest live count seen so far.
    fn peak(&self) -> usize {
        self.peak.load(Ordering::SeqCst)
    }

    /// Zero the peak once every connection has gone.
    fn reset_peak(&self) {
        while self.live.load(Ordering::SeqCst) != 0 {
            std::thread::sleep(Duration::from_millis(5));
        }
        self.peak.store(0, Ordering::SeqCst);
    }
}

/// Run this file's tests one at a time: the descriptor count is the whole
/// process's, so a neighbour's listener and sockets moved it (review 5:
/// "descriptors went 9 to 12" in 3 of 30 loaded runs).
fn alone() -> MutexGuard<'static, ()> {
    static SEAT: Mutex<()> = Mutex::new(());
    SEAT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// An engine for one listener, with the width the caller names.
fn engine(address: &str, width: usize) -> Arc<dyn Engine> {
    StandinConnector
        .connect(&EngineConfig {
            address: Some(address.to_owned()),
            width: Some(width),
            timeout: Some(Duration::from_secs(5)),
            ..EngineConfig::default()
        })
        .expect("builds")
}

/// One call against the engine, expecting the listener's yes.
fn call(engine: &Arc<dyn Engine>, question: &Question, note: &str) {
    engine
        .decide(question, note)
        .expect("the listener answers");
}

/// A width of 1 holds while a second, wider engine runs beside it: no call
/// on the narrow engine ever shares its single permit.
#[test]
fn a_narrow_engine_keeps_its_width_beside_a_wide_one() {
    let _alone = alone();
    let narrow = SlowListener::start(Duration::from_millis(60));
    let wide = SlowListener::start(Duration::from_millis(60));
    let narrow_engine = engine(&narrow.base(), 1);
    let wide_engine = engine(&wide.base(), 4);
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");

    // Warm both states once, sequentially, the way a host starts up.
    call(&narrow_engine, &question, "warm narrow");
    call(&wide_engine, &question, "warm wide");
    narrow.reset_peak();
    wide.reset_peak();

    std::thread::scope(|scope| {
        for worker in 0..6 {
            let narrow_engine = Arc::clone(&narrow_engine);
            let wide_engine = Arc::clone(&wide_engine);
            let question = question.clone();
            scope.spawn(move || {
                for round in 0..4 {
                    // Every thread alternates engines, so the settings
                    // change between calls as fast as the calls go.
                    call(&narrow_engine, &question, &format!("narrow {worker} {round}"));
                    call(&wide_engine, &question, &format!("wide {worker} {round}"));
                }
            });
        }
    });

    assert_eq!(
        narrow.peak(),
        1,
        "the narrow engine's width 1 holds across both engines"
    );
    assert!(
        wide.peak() <= 4,
        "the wide engine keeps its own width, peak was {}",
        wide.peak()
    );
}

/// The file descriptors stay flat over forty alternating calls: one state
/// per settings value means no pool is retired per call.
#[cfg(target_os = "linux")]
#[test]
fn the_file_descriptors_stay_flat() {
    let _alone = alone();
    let narrow = SlowListener::start(Duration::from_millis(5));
    let wide = SlowListener::start(Duration::from_millis(5));
    let narrow_engine = engine(&narrow.base(), 1);
    let wide_engine = engine(&wide.base(), 4);
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");

    for round in 0..4 {
        call(&narrow_engine, &question, &format!("warm narrow {round}"));
        call(&wide_engine, &question, &format!("warm wide {round}"));
    }
    let before = common::open_descriptors();
    for round in 0..18 {
        call(&narrow_engine, &question, &format!("narrow {round}"));
        call(&wide_engine, &question, &format!("wide {round}"));
    }
    // The listener's end of each closed connection closes a moment after
    // the client's.
    let after = common::settled_descriptors(before + 2);
    assert!(
        after <= before + 2,
        "forty calls hold their pools; descriptors went {before} to {after}"
    );
}
