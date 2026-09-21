//! The counter counts what left the machine: a retry that is sent counts
//! again, because the vendor bills each one (ruling 5 of the adversarial
//! review, 2026-09-21).
//!
//! The responder reads the first request and closes without answering —
//! the request left, then the connection died — and answers the second.
//! One judgment, two counted sends. One test per file, because the engine
//! reads the environment once per process.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use thinkthen_contract::{Answer, Engine, Question};
use thinkthen_standin::BlockingEngine;

/// The reply the second send earns, in the wire shape the core decodes.
const REPLY: &[u8] = br#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.97}},"usage":{"input_tokens":10,"output_tokens":2}}"#;

#[test]
fn a_retried_send_counts_twice() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("binds");
    let port = listener.local_addr().expect("an address").port();
    let reads = Arc::new(AtomicUsize::new(0));
    let seen = Arc::clone(&reads);
    std::thread::spawn(move || {
        for (index, stream) in listener.incoming().enumerate() {
            let mut stream = stream.expect("accepts");
            stream
                .set_read_timeout(Some(Duration::from_millis(200)))
                .expect("a read timeout");
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            loop {
                match stream.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => request.extend_from_slice(&buffer[..n]),
                }
            }
            seen.fetch_add(1, Ordering::Relaxed);
            if index == 0 {
                // The request was read; close without a reply. The client
                // sent once and may be billed once for it.
                drop(stream);
                continue;
            }
            let head = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                REPLY.len()
            );
            let _ = stream.write_all(head.as_bytes());
            let _ = stream.write_all(REPLY);
            let _ = stream.flush();
            break;
        }
    });

    // Sound in this binary: the test owns the process, and no engine call
    // runs before this set.
    unsafe { std::env::set_var("THINKTHEN_BASE_URL", format!("http://127.0.0.1:{port}/v1")) };
    let tt = BlockingEngine::from_env();
    let before = tt.usage().requests;
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");
    let answer = tt.decide(&question, "i want a refund").expect("the second send answers");
    assert_eq!(answer, Answer::Yes);
    let after = tt.usage().requests;
    assert_eq!(after - before, 2, "one judgment, two sends (was {before}, now {after})");
    assert!(
        reads.load(Ordering::Relaxed) >= 2,
        "the responder read both requests, saw {}",
        reads.load(Ordering::Relaxed)
    );
}
