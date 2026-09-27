//! The scripted listener's edges: a client that stops early, a reset it
//! cannot honor, and a connection past the end of its script.

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use conformance_backend::{Canned, Listener, Observed, Rendezvous};

type Tested = Result<(), Box<dyn Error>>;

/// How long a client waits for any answer before the test fails: past the
/// listener's 8-second idle timeout, which the silent row waits out.
const PATIENCE: Duration = Duration::from_secs(20);

const UNRESETTABLE: &str = "the loopback listener cannot reset a request it had to read";
const EXHAUSTED: &str = "the script has no reply left for this connection";

fn connect(listener: &Listener) -> Result<TcpStream, Box<dyn Error>> {
    let address = listener.origin().trim_start_matches("http://");
    let stream = TcpStream::connect(address)?;
    stream.set_read_timeout(Some(PATIENCE))?;
    Ok(stream)
}

/// Post a body of this many bytes and return the status line and the body.
fn post(listener: &Listener, length: usize) -> Result<(String, String), Box<dyn Error>> {
    let mut stream = connect(listener)?;
    let head = format!("POST /v1/systemone HTTP/1.1\r\ncontent-length: {length}\r\n\r\n");
    stream.write_all(head.as_bytes())?;
    stream.write_all(&vec![b'x'; length])?;
    let mut answer = String::new();
    stream.read_to_string(&mut answer)?;
    let (head, body) = answer.split_once("\r\n\r\n").ok_or("no header end")?;
    let status = head.lines().next().unwrap_or_default().to_owned();
    Ok((status, body.to_owned()))
}

#[test]
fn a_connection_that_ends_before_a_whole_request_takes_no_reply() -> Tested {
    // Half a request then a close, nothing then a close, and nothing on a
    // connection held open past the read timeout.
    for (case, sent, held) in [
        (
            "half",
            &b"POST /v1/systemone HTTP/1.1\r\ncontent-length: 10\r\n\r\nabc"[..],
            false,
        ),
        ("empty", &b""[..], false),
        ("silent", &b""[..], true),
    ] {
        let listener = Listener::serving(vec![Canned::ok("answered")])?;
        let mut early = connect(&listener)?;
        early.write_all(sent)?;
        let kept = held.then_some(early);
        let answered = post(&listener, 4)?;
        drop(kept);
        assert_eq!(
            answered,
            ("HTTP/1.1 200 X".to_owned(), "answered".to_owned()),
            "{case}"
        );
        assert_eq!(listener.connections(), 2, "{case}");
        assert_eq!(listener.requests().len(), 1, "{case}");
    }
    Ok(())
}

#[test]
fn a_reset_of_a_request_too_long_to_peek_answers_the_drift_status() -> Tested {
    let listener = Listener::serving(vec![Canned::reset()])?;
    let answered = post(&listener, 40 * 1024)?;
    assert_eq!(
        answered,
        ("HTTP/1.1 500 X".to_owned(), UNRESETTABLE.to_owned())
    );
    Ok(())
}

#[test]
fn a_connection_past_the_script_answers_the_drift_status_after_a_drop_too() -> Tested {
    let listener = Arc::new(Listener::serving(vec![Canned::ok("first")])?);
    let last_owner = Arc::clone(&listener);
    let origin = listener.origin().to_owned();
    assert_eq!(
        post(&listener, 4)?,
        ("HTTP/1.1 200 X".to_owned(), "first".to_owned())
    );
    let drifted = ("HTTP/1.1 500 X".to_owned(), EXHAUSTED.to_owned());
    assert_eq!(post(&listener, 5)?, drifted);
    assert_eq!(listener.connections(), 2);
    let lines: Vec<usize> = listener
        .requests()
        .iter()
        .map(|request| request.body.len())
        .collect();
    assert_eq!(lines, [4, 5]);
    drop(listener);
    let mut stream = TcpStream::connect(origin.trim_start_matches("http://"))?;
    stream.set_read_timeout(Some(PATIENCE))?;
    stream.write_all(b"POST /v1/systemone HTTP/1.1\r\ncontent-length: 0\r\n\r\n")?;
    let mut answer = String::new();
    stream.read_to_string(&mut answer)?;
    assert!(answer.starts_with("HTTP/1.1 500 X\r\n"), "{answer}");
    assert!(answer.ends_with(EXHAUSTED), "{answer}");
    drop(last_owner);
    assert!(
        TcpStream::connect(origin.trim_start_matches("http://")).is_err(),
        "the final owner left the port listening"
    );
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn retired_listeners_release_descriptors_and_threads_before_drop_returns() -> Tested {
    fn count(path: &str) -> Result<usize, Box<dyn Error>> {
        Ok(std::fs::read_dir(path)?.count())
    }
    let before = (count("/proc/self/fd")?, count("/proc/self/task")?);
    for _ in 0..80 {
        let scripted = Listener::serving(vec![Canned::ok("first")])?;
        assert_eq!(post(&scripted, 0)?.1, "first");
        assert_eq!(post(&scripted, 0)?.1, EXHAUSTED);
        drop(scripted);
        let empty = Listener::serving(Vec::new())?;
        drop(empty);
        let answering = Listener::answering(|_| Canned::ok("ok"))?;
        let idle = connect(&answering)?;
        drop(answering);
        drop(idle);
    }
    let after = (count("/proc/self/fd")?, count("/proc/self/task")?);
    assert!(
        after.0 <= before.0 + 4,
        "descriptors: {before:?} -> {after:?}"
    );
    assert!(after.1 <= before.1 + 2, "threads: {before:?} -> {after:?}");
    Ok(())
}

#[test]
fn a_held_reply_releases_normally_and_retirement_interrupts_an_unreleased_reply() -> Tested {
    let release = Arc::new(Rendezvous::new(2));
    let (arrived, events) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let release = Arc::clone(&release);
            move |_| Canned::ok("released").after_release(Arc::clone(&release))
        },
        arrived,
    )?;
    let address = listener.origin().trim_start_matches("http://").to_owned();
    let (answered, answer) = mpsc::channel();
    let client = thread::spawn(move || {
        let result = (|| -> Result<String, String> {
            let mut stream = TcpStream::connect(address).map_err(|error| error.to_string())?;
            stream
                .set_read_timeout(Some(PATIENCE))
                .map_err(|error| error.to_string())?;
            stream
                .write_all(b"POST /v1/systemone HTTP/1.1\r\ncontent-length: 0\r\n\r\n")
                .map_err(|error| error.to_string())?;
            let mut bytes = [0; 1024];
            let mut body = Vec::new();
            while !body.ends_with(b"released") {
                let seen = stream.read(&mut bytes).map_err(|error| error.to_string())?;
                if seen == 0 {
                    return Err("reply ended early".to_owned());
                }
                body.extend_from_slice(bytes.get(..seen).ok_or("read beyond buffer")?);
            }
            String::from_utf8(body).map_err(|error| error.to_string())
        })();
        let _ = answered.send(result);
    });
    assert!(matches!(
        events.recv_timeout(PATIENCE),
        Ok(Observed::Request)
    ));
    assert_eq!(
        answer.recv_timeout(Duration::from_millis(50)),
        Err(RecvTimeoutError::Timeout)
    );
    release.wait();
    let body = answer
        .recv_timeout(PATIENCE)?
        .map_err(|error| format!("client: {error}"))?;
    assert!(body.starts_with("HTTP/1.1 200 X\r\n"), "{body}");
    assert!(body.ends_with("released"), "{body}");
    client.join().map_err(|_| "client panicked")?;
    drop(listener);

    let release = Arc::new(Rendezvous::new(3));
    let (arrived, events) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let release = Arc::clone(&release);
            move |_| Canned::ok("never released").after_release(Arc::clone(&release))
        },
        arrived,
    )?;
    let mut stream = connect(&listener)?;
    stream.write_all(b"POST /v1/systemone HTTP/1.1\r\ncontent-length: 0\r\n\r\n")?;
    assert!(matches!(
        events.recv_timeout(PATIENCE),
        Ok(Observed::Request)
    ));
    let (awoken, wake) = mpsc::channel();
    let participant = thread::spawn(move || {
        let _ = awoken.send(release.wait());
    });
    let (retired, done) = mpsc::channel();
    let retirement = thread::spawn(move || {
        drop(listener);
        let _ = retired.send(());
    });
    assert_eq!(done.recv_timeout(Duration::from_secs(1)), Ok(()));
    assert_eq!(wake.recv_timeout(Duration::from_secs(1)), Ok(false));
    let mut body = String::new();
    stream.read_to_string(&mut body)?;
    assert!(
        body.is_empty(),
        "an unreleased reply escaped retirement: {body}"
    );
    retirement.join().map_err(|_| "retirement panicked")?;
    participant.join().map_err(|_| "participant panicked")?;
    Ok(())
}
