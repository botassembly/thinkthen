//! The counter counts what left the machine, because the vendor bills
//! each send (ruling 5 of the adversarial review, 2026-09-21), and a send
//! that left is never sent again (review 5).
//!
//! The responder reads the first request and closes without answering:
//! the request left, then the connection died. The call fails with one
//! counted send, and no second request arrives. One test per file,
//! because the engine reads the environment once per process.

use std::io::Read;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use thinkthen_contract::{Engine, Question};
use thinkthen_standin::BlockingEngine;

#[test]
fn a_send_that_left_counts_once_and_is_never_sent_again() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("binds");
    let port = listener.local_addr().expect("an address").port();
    let reads = Arc::new(AtomicUsize::new(0));
    let seen = Arc::clone(&reads);
    std::thread::spawn(move || {
        for stream in listener.incoming() {
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
            // The request was read; close without a reply. The client sent
            // once and may be billed once for it.
            seen.fetch_add(1, Ordering::Relaxed);
        }
    });

    // Sound in this binary: the test owns the process, and no engine call
    // runs before this set.
    unsafe { std::env::set_var("THINKTHEN_BASE_URL", format!("http://127.0.0.1:{port}/v1")) };
    let tt = BlockingEngine::from_env();
    let before = tt.usage().requests;
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");
    let failed = tt
        .decide(&question, "i want a refund")
        .expect_err("the first send dies unanswered");
    let after = tt.usage().requests;
    assert_eq!(
        after - before,
        1,
        "one send left and counts once (was {before}, now {after}): {failed}"
    );
    assert_eq!(
        reads.load(Ordering::Relaxed),
        1,
        "the responder read one request: {failed}"
    );
}
