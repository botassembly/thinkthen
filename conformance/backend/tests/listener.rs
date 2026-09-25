//! The scripted listener's edges: a client that stops early, a reset it
//! cannot honor, and a connection past the end of its script.

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use conformance_backend::{Canned, Listener};

type Tested = Result<(), Box<dyn Error>>;

/// How long a client waits for any answer before the test fails.
const PATIENCE: Duration = Duration::from_secs(10);

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
    let listener = Listener::serving(vec![Canned::ok("first")])?;
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
    Ok(())
}
