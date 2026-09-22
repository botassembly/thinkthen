//! The connector door: a host's config wins over the environment, and the
//! stand-in honors the address and width it is given (finding 8).
//!
//! One test, one process: the environment is written before any engine
//! value is built, so nothing else can read it mid-flight.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::time::Instant;

use thinkthen_contract::{Answer, Connector, EngineConfig, Question};
use thinkthen_standin::StandinConnector;

/// The reply one send earns, in the wire shape the core decodes.
const REPLY: &[u8] = br#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.97}},"usage":{"input_tokens":10,"output_tokens":2}}"#;

/// The config's address and width beat the environment's, proven by a
/// listener that answers and counts how many connections are live at once.
#[test]
fn the_config_beats_the_environment() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("binds");
    let port = listener.local_addr().expect("an address").port();
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
                std::thread::sleep(Duration::from_millis(120));
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

    // The environment names a refused address and a wide engine; the config
    // names the listener and width 1. The config must win both, or the
    // calls below refuse on port 1 and the peak goes past one.
    //
    // Sound in this binary: this test owns the process, and no engine call
    // runs before these sets.
    unsafe {
        std::env::set_var("ENGINE_BASE_URL", "http://127.0.0.1:1/v1");
        std::env::set_var("ENGINE_WIDTH", "8");
    }
    let config = EngineConfig {
        address: Some(format!("http://127.0.0.1:{port}/v1")),
        width: Some(1),
        timeout: Some(Duration::from_secs(5)),
        ..EngineConfig::default()
    };
    let engine = StandinConnector.connect(&config).expect("builds");
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");
    let started = Instant::now();
    std::thread::scope(|scope| {
        for call in 0..4 {
            let engine = &engine;
            let question = &question;
            scope.spawn(move || {
                let answer = engine
                    .decide(question, &format!("solo note {call}"))
                    .expect("the listener answers, so the config's address won");
                assert_eq!(answer, Answer::Yes, "a refund note is yes");
            });
        }
    });
    let wall = started.elapsed();
    let peak = peak.load(Ordering::SeqCst);
    assert_eq!(peak, 1, "the config's width 1 holds, peak was {peak}");
    assert!(
        wall >= Duration::from_millis(300),
        "four calls at width 1 and 120 ms each take their time: {wall:?}"
    );
}
