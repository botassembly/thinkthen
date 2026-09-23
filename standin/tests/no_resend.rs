//! A request whose bytes reached the server is never sent again by the
//! engine (review 5, item 2). The reviewer's probe: a server that reads
//! the whole request and then resets the connection received three full
//! POSTs from one call, and a server that reads it and never answers
//! received three as well, one per timeout. The server counts a body
//! before the client can see the failure, and the call returns only after
//! its last attempt, so the count is final when the call returns. Both now receive exactly one,
//! and the call fails with the transport's own error. A 503 still earns
//! its retries (`backoff.rs`): the server answered, so it said it did not
//! take the request.

mod common;

use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use thinkthen_contract::{Connector, EngineConfig, ErrorKind, Question};
use thinkthen_standin::StandinConnector;

/// What the server does once it has read a whole request.
#[derive(Clone, Copy)]
enum After {
    /// Close with a reset.
    Reset,
    /// Hold the connection and never answer.
    Stall,
}

/// A listener that reads every request whole, counts it, then misbehaves.
fn listener(after: After, bodies: Arc<AtomicUsize>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("binds");
    let port = listener.local_addr().expect("an address").port();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let bodies = Arc::clone(&bodies);
            std::thread::spawn(move || serve(stream, after, &bodies));
        }
    });
    format!("http://127.0.0.1:{port}/v1")
}

/// Read one request whole, count it, then reset or stall.
fn serve(stream: TcpStream, after: After, bodies: &AtomicUsize) {
    let mut reader = BufReader::new(stream.try_clone().expect("clones"));
    if common::read_request(&mut reader).is_none() {
        return;
    }
    bodies.fetch_add(1, Ordering::SeqCst);
    match after {
        After::Reset => {
            // A zero linger turns the close into a reset.
            let linger = libc::linger {
                l_onoff: 1,
                l_linger: 0,
            };
            // SAFETY: the descriptor is this stream's own, and the option
            // value is a live `linger` of the size passed.
            let set = unsafe {
                libc::setsockopt(
                    std::os::fd::AsRawFd::as_raw_fd(&stream),
                    libc::SOL_SOCKET,
                    libc::SO_LINGER,
                    (&raw const linger).cast(),
                    std::mem::size_of::<libc::linger>() as libc::socklen_t,
                )
            };
            assert_eq!(set, 0, "the zero linger sets");
            drop(reader);
            drop(stream);
        }
        After::Stall => std::thread::sleep(Duration::from_secs(10)),
    }
}

/// One decide against the misbehaving server, with the default two
/// retries named explicitly; returns the error and the bodies received.
fn one_call(after: After) -> (thinkthen_contract::Error, usize) {
    let bodies = Arc::new(AtomicUsize::new(0));
    let engine = StandinConnector
        .connect(&EngineConfig {
            address: Some(listener(after, Arc::clone(&bodies))),
            timeout: Some(Duration::from_secs(1)),
            max_retries: Some(2),
            width: Some(1),
            ..EngineConfig::default()
        })
        .expect("the engine connects");
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");
    let error = engine
        .decide(&question, "please refund")
        .expect_err("the server never answers");
    (error, bodies.load(Ordering::SeqCst))
}

#[test]
fn a_reset_after_the_body_left_is_not_sent_again() {
    let (error, bodies) = one_call(After::Reset);
    assert_eq!(
        bodies, 1,
        "the request was sent again after a reset: {error}"
    );
    assert_eq!(error.kind, ErrorKind::Backend, "{error}");
}

#[test]
fn a_read_timeout_after_the_body_left_is_not_sent_again() {
    let (error, bodies) = one_call(After::Stall);
    assert_eq!(
        bodies, 1,
        "the request was sent again after a timeout: {error}"
    );
    assert_eq!(error.kind, ErrorKind::Backend, "{error}");
}
