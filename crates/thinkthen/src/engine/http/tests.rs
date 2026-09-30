//! Focused HTTP parsing and transport checks.

use super::{
    Client, Exchange, Key, bounded_wait, draw, honored, io_transport, is_retried, transport,
};
use crate::engine::error::{Error, TransportKind};
use crate::engine::usage::Counters;
use std::cell::Cell;
use std::io::{self, Read as _, Write as _};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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
fn only_an_unheaded_retry_wait_is_capped_and_spread_by_the_draw() {
    let (s, ms) = (Duration::from_secs, Duration::from_millis);
    let timeout = s(2);
    let cases = [
        (Some(s(30)), s(1), 0, s(30)),
        (Some(ms(250)), s(4), u64::MAX, ms(250)),
        (None, s(4), u64::MAX, timeout),
        (None, s(4), 0, s(1)),
        (None, s(1), u64::MAX / 2, ms(750)),
        (None, s(1), 0, ms(500)),
        (None, s(120), u64::MAX, timeout),
    ];
    for (asked, exponential, draw, wait) in cases {
        let got = bounded_wait(asked, exponential, timeout, draw);
        assert!(
            got.abs_diff(wait) <= Duration::from_nanos(1),
            "{asked:?} {exponential:?} {draw}: {got:?}"
        );
    }
}

#[test]
fn parallel_workers_draw_different_unheaded_waits_within_bounds() {
    let (full, timeout) = (Duration::from_secs(1), Duration::from_secs(30));
    let waits: Vec<Duration> = thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| bounded_wait(None, full, timeout, draw())))
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect()
    });
    assert!(
        waits.iter().all(|wait| (full / 2..=full).contains(wait)),
        "{waits:?}"
    );
    assert!(
        waits.windows(2).any(|pair| pair[0] != pair[1]),
        "lockstep: {waits:?}"
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
        let (mut stream, _) = listener.accept().expect("request");
        let mut request = [0_u8; 1024];
        let _read = stream.read(&mut request).expect("request bytes");
        counted.fetch_add(1, Ordering::SeqCst);
        stream
            .write_all(b"HTTP/1.1 503 Unavailable\r\nContent-Length: 0\r\n\r\n")
            .expect("response");
        server_cancel.fire();
    });
    let url = format!("http://{address}/v1/systemone");
    let key = Key::of("sk-test-value");
    let client = Client::new(
        Duration::from_secs(2),
        false,
        &crate::engine::limits::process().widths,
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
    });
    server.join().expect("server thread");

    assert!(matches!(result, Err(Error::Cancelled)));
    assert_eq!(attempts.get(), 1);
    assert_eq!(received.load(Ordering::SeqCst), 1);
}

#[test]
fn a_token_fired_before_the_final_check_counts_and_sends_nothing() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url: &url,
        body: b"{}",
        key: &key,
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
    };
    let shown = format!("{exchange:?}");
    assert!(shown.contains("url: \"<withheld>\""), "{shown}");
    assert!(!shown.contains(&url), "{shown}");
    assert!(!shown.contains("sk-test-value"), "{shown}");
    let token = Arc::new(AtomicBool::new(false));
    let cancel = crate::engine::Cancel::default().with_token(Some(Arc::clone(&token)));
    let counts = Counters::new(None);
    let client = Client::new(
        Duration::from_secs(1),
        false,
        &crate::engine::limits::process().widths,
    );
    let result = client.post_observed_with_retry(&exchange, &cancel, &counts, |_| {
        token.store(true, Ordering::Release);
    });

    assert!(matches!(result, Err(Error::Cancelled)));
    assert_eq!(counts.snapshot().requests_sent, 0);
    assert!(matches!(listener.accept(), Err(error) if error.kind() == io::ErrorKind::WouldBlock));
}

#[test]
fn cancellation_after_reservation_refunds_both_process_charges() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url: &url,
        body: b"{}",
        key: &key,
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
    };
    let budget = crate::engine::budget::SendBudget::new();
    let selected = Some(crate::engine::send_budget::ProcessBudget {
        budget: budget.clone(),
        requests: Some(1),
        estimated: Some(2),
    });
    let token = Arc::new(AtomicBool::new(false));
    let stopped = crate::engine::Cancel::default()
        .with_token(Some(Arc::clone(&token)))
        .with_process_budget(selected.clone());
    let counts = Counters::new(None);
    let client = Client::new(
        Duration::from_secs(1),
        false,
        &crate::engine::limits::process().widths,
    );
    let refused = client.post_observed_after_reservation(&exchange, &stopped, &counts, || {
        token.store(true, Ordering::Release);
    });
    assert!(matches!(refused, Err(Error::Cancelled)));
    assert_eq!(counts.snapshot().requests_sent, 0);
    assert!(matches!(listener.accept(), Err(error) if error.kind() == io::ErrorKind::WouldBlock));

    listener.set_nonblocking(false).expect("blocking server");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("fresh request");
        let mut request = [0_u8; 1024];
        let _read = stream.read(&mut request).expect("request bytes");
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}")
            .expect("reply");
    });
    let fresh = crate::engine::Cancel::default().with_process_budget(selected);
    client
        .post_observed_with_retry(&exchange, &fresh, &counts, |_| ())
        .expect("refunded charge admits fresh send");
    server.join().expect("server");
    assert_eq!(counts.snapshot().requests_sent, 1);
}

#[test]
fn a_deadline_while_width_is_held_reserves_no_send() {
    static WIDTH: crate::engine::Widths = crate::engine::Widths::new();
    WIDTH
        .select(Some(crate::engine::Width::new(1).expect("width")))
        .expect("select");
    let held = WIDTH
        .acquire(&crate::engine::Cancel::default())
        .expect("hold width");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url: &url,
        body: b"{}",
        key: &key,
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
    };
    let budget = crate::engine::budget::SendBudget::new();
    let facts = crate::engine::CallFacts::new();
    let cancel = crate::engine::Cancel::default()
        .with_deadline(crate::engine::Deadline::after(Duration::from_millis(60)))
        .with_send_budget(Some((budget.clone(), Some(1))))
        .with_facts(facts.clone());
    let counts = Counters::new(None);
    let client = Client::new(
        Duration::from_secs(1),
        false,
        &crate::engine::limits::process().widths,
    )
    .gated(&WIDTH);
    let refused = client.post_observed_with_retry(&exchange, &cancel, &counts, |_| ());
    drop(held);
    assert!(matches!(refused, Err(Error::Deadline(_))));
    assert_eq!(counts.snapshot().requests_sent, 0);
    assert_eq!(facts.snapshot().requests_sent, 0);
    assert!(matches!(listener.accept(), Err(error) if error.kind() == io::ErrorKind::WouldBlock));
    budget
        .reserve(Some(1), None)
        .expect("the wait did not spend the send")
        .commit();
}

#[test]
fn a_deadline_during_retry_backoff_reserves_only_the_first_send() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("first request");
        let mut body = [0_u8; 1024];
        let _read = stream.read(&mut body).expect("request bytes");
        stream
            .write_all(b"HTTP/1.1 503 Unavailable\r\nContent-Length: 0\r\n\r\n")
            .expect("response");
    });
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url: &url,
        body: b"{}",
        key: &key,
        max_retries: 1,
        retry_wait: Duration::from_secs(1),
    };
    let budget = crate::engine::budget::SendBudget::new();
    let cancel = crate::engine::Cancel::default()
        .with_deadline(crate::engine::Deadline::after(Duration::from_millis(200)))
        .with_send_budget(Some((budget.clone(), Some(2))));
    let counts = Counters::new(None);
    let client = Client::new(
        Duration::from_secs(1),
        false,
        &crate::engine::limits::process().widths,
    );
    let refused = client.post_observed_with_retry(&exchange, &cancel, &counts, |_| ());
    server.join().expect("server");
    assert!(matches!(refused, Err(Error::Deadline(_))));
    assert_eq!(counts.snapshot().requests_sent, 1);
    assert_eq!(counts.snapshot().retries, 0);
    budget
        .reserve(Some(2), None)
        .expect("only the first send spent a unit")
        .commit();
    assert!(budget.reserve(Some(2), None).is_err());
}

/// Port zero can never listen, so the refusal is deterministic.
#[test]
fn a_refused_attempt_is_observed_once_and_returned_without_a_retry() {
    let key = Key::of("sk-test-value");
    let client = Client::new(
        Duration::from_secs(4),
        false,
        &crate::engine::limits::process().widths,
    );
    let observed = Cell::new(0_u32);
    let counts = Counters::new(None);
    let facts = crate::engine::CallFacts::new();
    let cancel = crate::engine::Cancel::default().with_facts(facts.clone());
    let exchange = Exchange {
        url: "http://127.0.0.1:0/v1/systemone",
        body: b"{}",
        key: &key,
        max_retries: 2,
        retry_wait: Duration::from_millis(1),
    };

    let result = client.post_observed_with_retry(&exchange, &cancel, &counts, |_| {
        observed.set(observed.get() + 1)
    });

    assert_eq!(observed.get(), 1);
    assert_eq!(counts.snapshot().requests_sent, 1);
    assert_eq!(counts.snapshot().retries, 0);
    assert_eq!(facts.snapshot().requests_sent, 1);
    assert!(matches!(
        result,
        Err(Error::Transport(TransportKind::Refused))
    ));
}

/// Ticket 0341. The server models a keep-alive timeout under one second whose
/// close has not reached the client: it drops, unread, any request that comes
/// back on a connection after the client idled. A reused connection would fail
/// that send as a premature close; the pool opens a new one instead.
#[test]
#[allow(clippy::excessive_nesting, reason = "per-connection server fixture")]
fn a_connection_idle_for_a_second_is_not_reused_and_nothing_is_sent_twice() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let url = format!("http://{}/v1", listener.local_addr().expect("address"));
    let tally = Arc::new([AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0)]);
    let server = Arc::clone(&tally);
    thread::spawn(move || {
        for stream in listener.incoming().take(2) {
            let (mut stream, tally) = (stream.expect("connection"), Arc::clone(&server));
            tally[0].fetch_add(1, Ordering::SeqCst);
            thread::spawn(move || {
                let (mut request, mut whole) = ([0_u8; 4096], Vec::new());
                while !whole.ends_with(b"\r\n\r\n{}") {
                    let read = stream.read(&mut request).expect("request");
                    assert!(read > 0, "the whole request arrives");
                    whole.extend_from_slice(&request[..read]);
                }
                tally[1].fetch_add(1, Ordering::SeqCst);
                stream
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}")
                    .expect("reply");
                if stream.read(&mut request).is_ok_and(|read| read > 0) {
                    tally[2].fetch_add(1, Ordering::SeqCst);
                }
            });
        }
    });
    let key = Key::of("");
    let exchange = Exchange {
        url: &url,
        body: b"{}",
        key: &key,
        max_retries: 0,
        retry_wait: Duration::from_millis(1),
    };
    let widths = &crate::engine::limits::process().widths;
    let client = Client::new(Duration::from_secs(5), false, widths);
    let counts = Counters::new(None);
    let cancel = crate::engine::Cancel::default();

    let first = client.post_observed_with_retry(&exchange, &cancel, &counts, |_| ());
    thread::sleep(Duration::from_millis(1200));
    let second = client.post_observed_with_retry(&exchange, &cancel, &counts, |_| ());

    assert_eq!(first.expect("first answer").requests_sent, 1);
    assert_eq!(second.expect("second answer").requests_sent, 1);
    let seen = tally.each_ref().map(|count| count.load(Ordering::SeqCst));
    assert_eq!(seen, [2, 2, 0], "connections, requests read, requests dropped");
    assert_eq!(counts.snapshot().requests_sent, 2);
}
