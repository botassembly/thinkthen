//! Whole-call deadline proofs for transport and prepared requests. The
//! scheduler proofs live in `schedule`.

use std::cell::Cell;
use std::fs;
use std::io::{ErrorKind, Read as _, Write as _};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::core::{Backend, Evidence, ModelName, Plan, Question, QuestionText};
use crate::engine::error::{Budget, Error, Kind, TransportKind};
use crate::engine::http::{Client, Exchange, HttpAnswer, Key};
use crate::engine::prepared_request::{Answered, PreparedRequest};
use crate::engine::recorder::{PreparedRecording, Recorder};
use crate::engine::usage::Counters;
use crate::engine::{Cancel, Deadline, cache_lock};

mod schedule;

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

fn named(stop: Error) -> &'static str {
    match stop {
        Error::Deadline(_) => "deadline",
        Error::Cancelled => "cancelled",
        _ => "other",
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
        let mut connections = 0;
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
                    continue;
                }
            };
            stream.write_all(written.as_bytes()).expect("response");
        }
        let _released = released.recv_timeout(SECOND * 2);
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
    Client::new(timeout, false, crate::engine::process_width()).post_observed(
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

    let result = Client::new(SECOND * 30, false, crate::engine::process_width()).post_observed(
        &exchange,
        &within(Duration::from_millis(200)),
        || thread::sleep(Duration::from_millis(500)),
    );

    assert_eq!(deadline_of(result), Duration::from_millis(200));
    assert!(started.elapsed() < SECOND, "the call returned promptly");
    assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
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

    let answer =
        post(&server.url, SECOND * 5, &within(SECOND * 10), &attempts).expect("the retry answers");

    assert_eq!(answer.requests_sent, 2);
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

fn request() -> (Backend, Plan, PreparedRequest) {
    let backend = Backend::resolve(Some("http://127.0.0.1:1/v1/systemone"), None, "jev-latest")
        .expect("backend");
    let plan = Plan::new(
        Evidence::new("evidence").expect("evidence"),
        ModelName::new("jev-latest").expect("model"),
        vec![Question::Decide {
            text: QuestionText::new("Is this relevant?").expect("question"),
            yes: None,
            no: None,
        }],
    )
    .expect("plan");
    let prepared = PreparedRequest::new(&backend, &plan).expect("request");
    (backend, plan, prepared)
}

/// Ask one prepared request, counting key lookups and sends.
fn ask(recorder: &Recorder, cancel: &Cancel, counts: &[AtomicUsize; 2]) -> Result<Answered, Error> {
    let (backend, plan, prepared) = request();
    super::request::ask_prepared(
        &backend,
        &plan,
        prepared,
        recorder,
        cancel,
        &Counters::default(),
        || {
            counts[0].fetch_add(1, Ordering::SeqCst);
            Ok(Key::of("unused"))
        },
        |_, _| {
            counts[1].fetch_add(1, Ordering::SeqCst);
            Err(Error::Defect("send unexpectedly reached"))
        },
    )
}

#[test]
fn a_spent_deadline_stops_a_prepared_request_before_its_key() {
    let recorder = Recorder::of(None, None).expect("no folders");
    let counts = [AtomicUsize::new(0), AtomicUsize::new(0)];
    let cancelled = spent();
    cancelled.fire();

    let result = ask(&recorder, &spent(), &counts);
    let first = ask(&recorder, &cancelled, &counts);

    assert!(matches!(
        result,
        Err(Error::Deadline(Budget(Duration::ZERO)))
    ));
    assert!(matches!(first, Err(Error::Cancelled)));
    assert_eq!(counts.map(|count| count.into_inner()), [0, 0]);
}

fn folder(label: &str) -> PathBuf {
    let path =
        std::env::temp_dir().join(format!("thinkthen-deadline-{label}-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&path);
    path
}

/// Wait behind a held lock until the deadline passes, and prove the waiter blocked.
fn waits_out_the_deadline(recorder: &Recorder) {
    let (blocked_send, blocked) = channel();
    let cancel =
        Cancel::observed(blocked_send).with_deadline(Deadline::after(Duration::from_millis(300)));
    let counts = [AtomicUsize::new(0), AtomicUsize::new(0)];

    let result = ask(recorder, &cancel, &counts);

    assert!(blocked.try_recv().is_ok(), "the waiter met the held lock");
    assert!(matches!(result, Err(Error::Deadline(_))));
    assert_eq!(counts.map(|count| count.into_inner()), [0, 0]);
}

#[test]
fn a_held_folder_ends_as_the_deadline_without_the_owner() {
    let path = folder("folder");
    fs::create_dir_all(&path).expect("recording folder");
    let owner = cache_lock::exclusive_folder(&path).expect("exclusive owner");

    waits_out_the_deadline(&Recorder::of(Some(&path), None).expect("recorder"));

    drop(owner);
    fs::remove_dir_all(path).expect("fixture removed");
}

#[test]
fn a_held_digest_ends_as_the_deadline_without_the_owner() {
    let path = folder("digest");
    let recorder = Recorder::of(Some(&path), None).expect("recorder");
    let (backend, _, prepared) = request();
    let PreparedRecording::Live(owner) = recorder
        .prepare_cancelled(
            &prepared.recorded(&backend),
            &prepared.digest,
            &Cancel::default(),
        )
        .expect("owner prepared")
    else {
        unreachable!("empty recording cannot replay");
    };

    waits_out_the_deadline(&recorder);

    owner.cancel().expect("owner cleanup");
    assert!(path.join(".locks").join(prepared.digest.as_str()).exists());
    fs::remove_dir_all(path).expect("fixture removed");
}
