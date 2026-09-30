//! Whole-call deadline proofs for the transport. The pipeline's proofs live
//! in `pipeline/tests.rs`, and the ordered runner's in `cli/schedule/ordered`.

use std::cell::Cell;
use std::fs;
use std::io::{ErrorKind, Read as _, Write as _};
use std::net::TcpListener;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::engine::error::{Budget, Error, Kind, TransportKind};
use crate::engine::http::{Client, Exchange, HttpAnswer, Key};
use crate::engine::usage::Counters;
use crate::engine::{Cancel, Deadline};

const SECOND: Duration = Duration::from_secs(1);

fn within(budget: Duration) -> Cancel<'static> {
    Cancel::default().with_deadline(Deadline::after(budget))
}

fn spent() -> Cancel<'static> {
    within(Duration::ZERO)
}

fn deadline_of(result: Result<HttpAnswer, Error>) -> Duration {
    match result {
        Err(Error::Deadline(Budget(budget))) => budget,
        Err(error) => panic!("expected the deadline, got {error:?}"),
        Ok(_) => panic!("expected the deadline, got an answer"),
    }
}

#[test]
fn the_deadline_error_keeps_its_budget_and_prints_it_in_integers() {
    let cases = [
        (
            SECOND * 5,
            "the deadline of 5 s passed before the call answered",
        ),
        (
            Duration::from_secs(4_294_967_295),
            "the deadline of 4294967295 s passed before the call answered",
        ),
        (
            Duration::from_millis(1500),
            "the deadline of 1500 ms passed before the call answered",
        ),
        (
            Duration::MAX,
            "the deadline of 18446744073709551615999999999 ns passed before the call answered",
        ),
    ];
    for (budget, sentence) in cases {
        assert_eq!(Budget(budget).to_string(), sentence);
    }
    assert_eq!(Error::Deadline(Budget(SECOND)).kind(), Kind::Deadline);
    assert_eq!(Kind::Deadline.as_str(), "deadline");
}

#[test]
fn a_budget_no_instant_can_hold_means_no_deadline() {
    assert!(Deadline::after(Duration::MAX).is_none());
    let unbounded = within(Duration::MAX);
    assert!(matches!(unbounded.stop_or_remaining(), Ok(None)));
    assert!(Cancel::default().wait(Duration::from_millis(1)).is_none());
}

#[test]
fn cancellation_is_observed_before_a_spent_deadline() {
    let cancel = spent();
    assert!(matches!(
        cancel.stop(),
        Some(Error::Deadline(Budget(Duration::ZERO)))
    ));
    cancel.fire();
    assert!(matches!(cancel.stop(), Some(Error::Cancelled)));
    assert!(matches!(cancel.wait(SECOND), Some(Error::Cancelled)));
}

/// What the loopback backend does with each connection, in order.
enum Reply {
    Busy(u64),
    Answer,
    Hold,
}

struct Server {
    url: String,
    release: Sender<()>,
    thread: JoinHandle<(usize, bool)>,
}

impl Server {
    /// Release any held reply and return the connection count and whether a
    /// later connection arrived after the replies ran out.
    fn finish(self) -> (usize, bool) {
        let _released = self.release.send(());
        self.thread.join().expect("server thread")
    }
}

#[allow(clippy::excessive_nesting, reason = "one scripted loopback server")]
fn serve(replies: Vec<Reply>) -> (Server, Receiver<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    let (received_send, received) = channel();
    let (release, released) = channel::<()>();
    let thread = thread::spawn(move || {
        let (mut connections, mut released_once) = (0, false);
        for reply in replies {
            let (mut stream, _) = listener.accept().expect("request");
            connections += 1;
            let mut request = [0_u8; 4096];
            let _read = stream.read(&mut request).expect("request bytes");
            let _observed = received_send.send(());
            let written = match reply {
                Reply::Busy(millis) => format!(
                    "HTTP/1.1 503 Busy\r\nretry-after-ms: {millis}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
                ),
                Reply::Answer => {
                    "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 2\r\n\r\n{}".to_owned()
                }
                Reply::Hold => {
                    let _held = released.recv();
                    released_once = true;
                    continue;
                }
            };
            stream.write_all(written.as_bytes()).expect("response");
        }
        // `finish` sends one release. A held reply took it, so the test is
        // finishing now; waiting again would only spend the 2 s bound.
        if !released_once {
            let _released = released.recv_timeout(SECOND * 2);
        }
        listener.set_nonblocking(true).expect("nonblocking");
        let later =
            !matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock);
        (connections, later)
    });
    (
        Server {
            url,
            release,
            thread,
        },
        received,
    )
}

fn post(
    url: &str,
    timeout: Duration,
    cancel: &Cancel,
    attempts: &Cell<u32>,
) -> Result<HttpAnswer, Error> {
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url,
        body: b"{}",
        key: &key,
        max_retries: 1,
        retry_wait: Duration::from_millis(10),
    };
    Client::new(timeout, false, &crate::engine::limits::process().widths).post_observed(
        &exchange,
        cancel,
        || {
            attempts.set(attempts.get() + 1);
        },
    )
}

#[test]
fn a_spent_deadline_opens_no_connection_and_observes_no_attempt() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    let attempts = Cell::new(0);

    let budget = deadline_of(post(&url, SECOND, &spent(), &attempts));
    let cancelled = spent();
    cancelled.fire();
    let first = post(&url, SECOND, &cancelled, &attempts);

    assert_eq!(budget, Duration::ZERO);
    assert!(matches!(first, Err(Error::Cancelled)));
    assert_eq!(attempts.get(), 0);
    assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
}

/// Slow work in the hook before the attempt outlasts the budget; the attempt
/// must not go out afterwards.
#[test]
fn accounting_that_outlasts_the_budget_sends_nothing() {
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
    let started = Instant::now();
    let folder =
        std::env::temp_dir().join(format!("thinkthen-unsent-attempt-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&folder);
    let counts = Counters::new(Some(folder.clone()));

    let result = Client::new(SECOND * 30, false, &crate::engine::limits::process().widths)
        .post_observed_with_retry(
            &exchange,
            &within(Duration::from_millis(200)),
            &counts,
            |_| thread::sleep(Duration::from_millis(500)),
        );

    assert_eq!(deadline_of(result), Duration::from_millis(200));
    // A hang guard, never a speed claim; the accept below proves no attempt went out.
    assert!(started.elapsed() < SECOND * 10, "the call returned");
    assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
    assert_eq!(counts.snapshot().requests_sent, 0);
    assert_eq!(counts.snapshot().retries, 0);
    assert!(
        !counts.finish(),
        "an unsent attempt cannot fail persistence"
    );
    assert!(!folder.exists(), "an unsent attempt cannot create a ledger");
}

#[test]
fn a_held_response_has_one_visible_in_flight_attempt() {
    let (server, received) = serve(vec![Reply::Hold]);
    let counts = Counters::new(None);
    let key = Key::of("sk-test-value");
    let url = server.url.clone();
    let exchange = Exchange {
        url: &url,
        body: b"{}",
        key: &key,
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
    };
    let client = Client::new(SECOND * 2, false, &crate::engine::limits::process().widths);
    thread::scope(|scope| {
        let sending = scope.spawn(|| {
            client.post_observed_with_retry(&exchange, &Cancel::default(), &counts, |_| ())
        });
        received.recv_timeout(SECOND).expect("the request arrived");
        assert_eq!(counts.snapshot().requests_sent, 1);
        assert_eq!(counts.snapshot().retries, 0);
        assert_eq!(server.finish(), (1, false));
        let _finished = sending.join().expect("sending thread");
        // `finish` no longer waits for a stray connection, so the counts
        // prove the released call sent no retry.
        assert_eq!(counts.snapshot().requests_sent, 1);
        assert_eq!(counts.snapshot().retries, 0);
    });
}

#[test]
fn a_held_reply_ends_as_the_deadline_after_one_send() {
    for cancel_in_flight in [false, true] {
        let (server, received) = serve(vec![Reply::Hold]);
        let cancel = within(Duration::from_millis(300));
        let attempts = Cell::new(0);
        let started = Instant::now();
        let result = thread::scope(|scope| {
            let cancel = &cancel;
            scope.spawn(move || {
                received.recv().expect("the request arrived");
                cancel_in_flight.then(|| cancel.fire());
            });
            post(&server.url, SECOND * 30, cancel, &attempts)
        });

        assert_eq!(deadline_of(result), Duration::from_millis(300));
        assert!(started.elapsed() < SECOND * 10);
        assert_eq!(attempts.get(), 1);
        assert_eq!(server.finish(), (1, false));
    }
}

#[test]
fn without_a_deadline_the_held_reply_reaches_the_attempt_timeout() {
    let (server, _) = serve(vec![Reply::Hold]);
    let attempts = Cell::new(0);

    let result = post(
        &server.url,
        Duration::from_millis(300),
        &Cancel::default(),
        &attempts,
    );

    assert!(matches!(
        result,
        Err(Error::Transport(TransportKind::Timeout))
    ));
    assert_eq!(attempts.get(), 1);
    assert_eq!(server.finish(), (1, false));
}

#[test]
fn a_retry_wait_past_the_budget_ends_as_the_deadline_without_a_second_send() {
    let (server, _) = serve(vec![Reply::Busy(60_000)]);
    let attempts = Cell::new(0);
    let started = Instant::now();

    let result = post(
        &server.url,
        SECOND * 30,
        &within(Duration::from_millis(300)),
        &attempts,
    );

    assert_eq!(deadline_of(result), Duration::from_millis(300));
    assert!(started.elapsed() < SECOND * 10);
    assert_eq!(attempts.get(), 1);
    assert_eq!(server.finish(), (1, false));
}

#[test]
fn a_retry_wait_inside_the_budget_still_retries() {
    let (server, _) = serve(vec![Reply::Busy(10), Reply::Answer]);
    let attempts = Cell::new(0);

    post(&server.url, SECOND * 5, &within(SECOND * 10), &attempts).expect("the retry answers");

    assert_eq!(attempts.get(), 2);
    assert_eq!(server.finish(), (2, false));
}

#[test]
fn a_retry_started_inside_the_budget_ends_its_send_as_the_deadline() {
    let (server, _) = serve(vec![Reply::Busy(10), Reply::Hold]);
    let attempts = Cell::new(0);

    let result = post(
        &server.url,
        SECOND * 30,
        &within(Duration::from_millis(500)),
        &attempts,
    );

    assert_eq!(deadline_of(result), Duration::from_millis(500));
    assert_eq!(attempts.get(), 2);
    assert_eq!(server.finish(), (2, false));
}
