//! Focused HTTP parsing and transport checks.

use super::{Client, Exchange, Key, bounded_wait, honored, io_transport, is_retried, transport};
use crate::engine::error::{Error, TransportKind};
use std::cell::Cell;
use std::io::{self, Read as _, Write as _};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

#[test]
fn a_retry_after_header_is_a_floor_even_above_the_old_ceiling() {
    let cases = [
        (Some("2"), Some(Duration::from_secs(2))),
        (Some("  7 "), Some(Duration::from_secs(7))),
        (Some("0"), Some(Duration::from_secs(1))),
        (Some("99999"), Some(Duration::from_secs(99999))),
        (Some("Wed, 21 Oct 2026 07:28:00 GMT"), None),
        (Some("-1"), None),
        (Some(""), None),
        (None, None),
    ];
    for (header, expected) in cases {
        assert_eq!(honored(None, header), expected, "{header:?}");
    }
}

#[test]
fn the_milliseconds_header_is_read_first_and_the_seconds_header_follows_it() {
    let cases = [
        (Some("250"), None, Some(Duration::from_millis(250))),
        (Some("1500"), Some("9"), Some(Duration::from_millis(1500))),
        (Some("600000"), None, Some(Duration::from_secs(600))),
        // A milliseconds header nobody can read leaves the seconds one.
        (Some("soon"), Some("3"), Some(Duration::from_secs(3))),
        (Some(""), Some("3"), Some(Duration::from_secs(3))),
        (Some("-5"), None, None),
        (None, None, None),
    ];
    for (millis, seconds, expected) in cases {
        assert_eq!(honored(millis, seconds), expected, "{millis:?} {seconds:?}");
    }
}

#[test]
fn only_an_unheaded_retry_wait_stops_at_the_attempt_timeout() {
    let timeout = Duration::from_secs(2);
    assert_eq!(
        bounded_wait(
            Some(Duration::from_secs(30)),
            Duration::from_secs(1),
            timeout
        ),
        Duration::from_secs(30)
    );
    assert_eq!(bounded_wait(None, Duration::from_secs(4), timeout), timeout);
    assert_eq!(
        bounded_wait(
            Some(Duration::from_millis(250)),
            Duration::from_secs(4),
            timeout
        ),
        Duration::from_millis(250)
    );
}

#[test]
fn structured_transport_errors_map_without_reading_their_display_text() {
    let cases = [
        (
            ureq::Error::Timeout(ureq::Timeout::Global),
            TransportKind::Timeout,
        ),
        (ureq::Error::HostNotFound, TransportKind::NameLookup),
        (
            ureq::Error::Tls("hostile certificate text"),
            TransportKind::Tls,
        ),
        (
            ureq::Error::Io(io::Error::new(
                io::ErrorKind::ConnectionRefused,
                "hostile refused text",
            )),
            TransportKind::Refused,
        ),
        (
            ureq::Error::Io(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "hostile close text",
            )),
            TransportKind::PrematureClose,
        ),
        (ureq::Error::ConnectionFailed, TransportKind::Other),
    ];
    for (error, expected) in cases {
        assert_eq!(transport(&error, false), expected, "{error:?}");
    }
    let wrapped = ureq::Error::Io(io::Error::new(
        io::ErrorKind::InvalidData,
        "hostile certificate text",
    ));
    assert_eq!(transport(&wrapped, true), TransportKind::Tls);
    assert_eq!(transport(&wrapped, false), TransportKind::Other);
    for kind in [
        io::ErrorKind::ConnectionReset,
        io::ErrorKind::ConnectionAborted,
        io::ErrorKind::BrokenPipe,
    ] {
        assert_eq!(
            io_transport(&io::Error::new(kind, "hostile close text")),
            TransportKind::PrematureClose
        );
    }
}

#[test]
fn no_transport_failure_is_sent_again() {
    let cases = [
        (Error::Transport(TransportKind::Refused), false),
        (Error::Transport(TransportKind::Timeout), false),
        (Error::Transport(TransportKind::NameLookup), false),
        (Error::Transport(TransportKind::PrematureClose), false),
        (Error::Transport(TransportKind::Tls), false),
        (Error::Transport(TransportKind::Other), false),
        (Error::Status(429), true),
        (Error::Status(500), true),
        (Error::Status(502), true),
        (Error::Status(503), true),
        (Error::Status(504), true),
        (Error::Status(529), true),
        (Error::Status(401), false),
    ];
    for (failure, expected) in cases {
        assert_eq!(is_retried(&failure), expected, "{failure:?}");
    }
}

#[test]
#[allow(clippy::excessive_nesting, reason = "synchronized server fixture")]
fn cancellation_during_a_retry_wait_starts_no_second_attempt() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let address = listener.local_addr().expect("address");
    let received = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&received);
    let cancel = crate::engine::Cancel::default();
    let server_cancel = cancel.clone();
    let server = thread::spawn(move || {
        for stream in listener.incoming().take(2) {
            let mut stream = stream.expect("request");
            let mut request = [0_u8; 1024];
            let _read = stream.read(&mut request).expect("request bytes");
            counted.fetch_add(1, Ordering::SeqCst);
            stream
                .write_all(b"HTTP/1.1 503 Unavailable\r\nContent-Length: 0\r\n\r\n")
                .expect("response");
            if server_cancel.fired() {
                break;
            }
        }
    });
    let url = format!("http://{address}/v1/systemone");
    let key = Key::of("sk-test-value");
    let client = Client::new(
        Duration::from_secs(2),
        false,
        crate::engine::process_width(),
    );
    let attempts = Cell::new(0_u32);
    let exchange = Exchange {
        url: &url,
        body: b"{}",
        key: &key,
        max_retries: 1,
        retry_wait: Duration::from_secs(1),
    };

    let result = client.post_observed(&exchange, &cancel, || {
        attempts.set(attempts.get() + 1);
        cancel.fire();
    });
    server.join().expect("server thread");

    assert!(matches!(result, Err(Error::Cancelled)));
    assert_eq!(attempts.get(), 1);
    assert_eq!(received.load(Ordering::SeqCst), 1);
}

/// Port zero can never listen, so the refusal is deterministic.
#[test]
fn a_refused_attempt_is_observed_once_and_returned_without_a_retry() {
    let key = Key::of("sk-test-value");
    let client = Client::new(
        Duration::from_secs(4),
        false,
        crate::engine::process_width(),
    );
    let observed = Cell::new(0_u32);
    let exchange = Exchange {
        url: "http://127.0.0.1:0/v1/systemone",
        body: b"{}",
        key: &key,
        max_retries: 2,
        retry_wait: Duration::from_millis(1),
    };

    let result = client.post_observed(&exchange, &crate::engine::Cancel::default(), || {
        observed.set(observed.get() + 1)
    });

    assert_eq!(observed.get(), 1);
    assert!(matches!(
        result,
        Err(Error::Transport(TransportKind::Refused))
    ));
}
