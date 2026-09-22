//! The environment spellings (review item 4, 2026-09-22): the settled
//! `THINKTHEN_` names win over the older `ENGINE_` names, and the older
//! names keep working on their own.
//!
//! One test file, one process: the environment is written between engine
//! values, and a seat serializes the writes against the reads.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

use thinkthen_contract::{Connector, EngineConfig, Question};
use thinkthen_standin::StandinConnector;

/// The reply one send earns, in the wire shape the core decodes.
const REPLY: &[u8] = br#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.97}}}"#;

/// A listener that answers after a delay, counting the highest live count.
fn slow_listener(delay: Duration) -> (String, Arc<AtomicUsize>, Arc<AtomicUsize>) {
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
                let now = seen.fetch_add(1, Ordering::SeqCst) + 1;
                top.fetch_max(now, Ordering::SeqCst);
                let mut buffer = [0_u8; 4096];
                let _ = stream.read(&mut buffer);
                std::thread::sleep(delay);
                let head = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                    REPLY.len()
                );
                let _ = stream.write_all(head.as_bytes());
                let _ = stream.write_all(REPLY);
                let _ = stream.flush();
                seen.fetch_sub(1, Ordering::SeqCst);
            });
        }
    });
    (format!("http://{address}/v1"), live, peak)
}

/// Serialize the environment writes against the engine builds.
fn seat() -> MutexGuard<'static, ()> {
    static SEAT: Mutex<()> = Mutex::new(());
    SEAT.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The prefixed spelling wins when both are set; the older spelling alone
/// still works.
#[test]
fn the_prefixed_names_win_and_the_old_ones_still_work() {
    let _seat = seat();
    let (address, live, peak) = slow_listener(Duration::from_millis(120));
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");

    // Both spellings set, differently: the prefixed width must win.
    // Sound in this binary: the seat is held across every write and build.
    unsafe {
        std::env::set_var("THINKTHEN_WIDTH", "4");
        std::env::set_var("ENGINE_WIDTH", "1");
    }
    let wide = StandinConnector
        .connect(&EngineConfig {
            address: Some(address.clone()),
            timeout: Some(Duration::from_secs(5)),
            ..EngineConfig::default()
        })
        .expect("builds");
    std::thread::scope(|scope| {
        for worker in 0..4 {
            let wide = Arc::clone(&wide);
            let question = question.clone();
            scope.spawn(move || {
                wide.decide(&question, &format!("prefixed {worker}"))
                    .expect("the listener answers");
            });
        }
    });
    while live.load(Ordering::SeqCst) != 0 {
        std::thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(
        peak.load(Ordering::SeqCst),
        4,
        "THINKTHEN_WIDTH=4 beat ENGINE_WIDTH=1"
    );

    // The older spelling alone keeps working, at its own width.
    peak.store(0, Ordering::SeqCst);
    unsafe {
        std::env::remove_var("THINKTHEN_WIDTH");
        std::env::set_var("ENGINE_WIDTH", "2");
    }
    let older = StandinConnector
        .connect(&EngineConfig {
            address: Some(address.clone()),
            timeout: Some(Duration::from_secs(5)),
            ..EngineConfig::default()
        })
        .expect("builds");
    std::thread::scope(|scope| {
        for worker in 0..3 {
            let older = Arc::clone(&older);
            let question = question.clone();
            scope.spawn(move || {
                older
                    .decide(&question, &format!("older {worker}"))
                    .expect("the listener answers");
            });
        }
    });
    while live.load(Ordering::SeqCst) != 0 {
        std::thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(
        peak.load(Ordering::SeqCst),
        2,
        "ENGINE_WIDTH=2 alone still sets the width"
    );
    unsafe { std::env::remove_var("ENGINE_WIDTH") };
}
