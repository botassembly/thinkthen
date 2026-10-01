//! The longest retry wait, on a loopback backend (ticket 0367).

use std::io::{Read as _, Write as _};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use crate::engine::error::Error;
use crate::engine::http::{Client, Exchange, Key};
use crate::engine::usage::Counters;

/// A loopback backend that answers its first request with a 429 carrying
/// `header`, and every later one with `{}`. Each reply closes its connection.
#[allow(clippy::excessive_nesting, reason = "per-connection server fixture")]
fn refusing_once(header: &'static str) -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let url = format!("http://{}/v1", listener.local_addr().expect("address"));
    let seen = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&seen);
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.expect("connection");
            let (mut request, mut whole) = ([0_u8; 4096], Vec::new());
            while !whole.ends_with(b"\r\n\r\n{}") {
                let read = stream.read(&mut request).expect("request");
                if read == 0 {
                    break;
                }
                whole.extend_from_slice(&request[..read]);
            }
            let reply = if counted.fetch_add(1, Ordering::SeqCst) == 0 {
                format!(
                    "HTTP/1.1 429 Too Many Requests\r\n{header}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
                )
            } else {
                "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 2\r\n\r\n{}".to_owned()
            };
            stream.write_all(reply.as_bytes()).expect("reply");
        }
    });
    (url, seen)
}

/// Main slept for any wait a server asked, so a run had no fixed worst case.
/// A wait up to the cap the doubling wait uses, here the 300 ms timeout, is
/// still honored; one past it fails at once with the status, and so does a
/// later request to that address until the server's time (ticket 0367).
#[test]
fn a_server_wait_past_the_cap_fails_at_once_and_one_at_the_cap_is_honored() {
    let timeout = Duration::from_millis(300);
    let cases = [
        ("Retry-After-Ms: 300", true, 2),
        ("Retry-After-Ms: 301", false, 1),
        ("Retry-After: 86400", false, 1),
        // Zero asks for the one-second floor, past this timeout.
        ("Retry-After: 0", false, 1),
    ];
    let widths = &crate::engine::limits::process().widths;
    let client = Client::new(timeout, false, widths);
    let key = Key::of("");
    for (header, answered, requests) in cases {
        let (url, seen) = refusing_once(header);
        let exchange = Exchange {
            url: &url,
            body: b"{}",
            key: &key,
            max_retries: 3,
            retry_wait: Duration::from_millis(1),
        };
        let counts = Counters::new(None);
        let cancel = crate::engine::Cancel::default();
        let started = std::time::Instant::now();
        let result = client.post_observed_with_retry(&exchange, &cancel, &counts, |_| ());
        let elapsed = started.elapsed();
        assert_eq!(seen.load(Ordering::SeqCst), requests, "{header}");
        assert_eq!(counts.snapshot().requests_sent, requests as u64, "{header}");
        if answered {
            assert_eq!(result.expect(header).body, b"{}");
            assert!(elapsed >= timeout, "{header}: {elapsed:?}");
        } else {
            assert!(matches!(result, Err(Error::Status(429))), "{header}");
            assert!(elapsed < timeout, "{header}: waited {elapsed:?}");
            let started = std::time::Instant::now();
            let later = client.post_observed_with_retry(&exchange, &cancel, &counts, |_| ());
            assert!(matches!(later, Err(Error::Status(429))), "{header}: later");
            assert!(started.elapsed() < timeout, "{header}: later waited");
            assert_eq!(seen.load(Ordering::SeqCst), 1, "{header}: later sent");
        }
    }
}
