//! The binary a binding's tests start: its port line, count line, held reply, bind, and exit.

use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::{RecvTimeoutError, channel};
use std::thread;
use std::time::{Duration, Instant};

type Tested = Result<(), Box<dyn Error>>;

/// One decide question in the wire shape the engine sends.
const DECIDE: &str =
    r#"{"state":"x","model":"m","questions":{"q1":{"type":"noul","instructions":"Is it?"}}}"#;

/// A started backend: its process, its pipes, and its port.
struct Started(Child, ChildStdin, BufReader<ChildStdout>, u16);

fn start() -> Result<Started, Box<dyn Error>> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_conformance-backend"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let input = child.stdin.take().ok_or("no standard input")?;
    let mut output = BufReader::new(child.stdout.take().ok_or("no standard output")?);
    let port = line(&mut output)?.parse()?;
    Ok(Started(child, input, output, port))
}

/// Send one line, and read the line it answers with when it answers one.
fn ask(backend: &mut Started, text: &str) -> Result<String, Box<dyn Error>> {
    writeln!(backend.1, "{text}")?;
    backend.1.flush()?;
    if text == "count" {
        line(&mut backend.2)
    } else {
        Ok(String::new())
    }
}

/// Close standard input, read the final count, and require the exit.
fn finish(Started(mut child, input, mut output, _): Started) -> Result<String, Box<dyn Error>> {
    drop(input);
    let last = line(&mut output)?;
    let deadline = Instant::now() + Duration::from_secs(5);
    while child.try_wait()?.is_none() {
        if Instant::now() > deadline {
            child.kill()?;
            return Err("the backend outlived its standard input".into());
        }
        thread::sleep(Duration::from_millis(10));
    }
    Ok(last)
}

fn line(output: &mut impl BufRead) -> Result<String, Box<dyn Error>> {
    let mut text = String::new();
    output.read_line(&mut text)?;
    Ok(text.trim_end().to_owned())
}

/// Post one body to a path and return the status line and the body.
fn post(port: u16, path: &str, body: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))?;
    let length = body.len();
    write!(
        stream,
        "POST {path} HTTP/1.1\r\ncontent-length: {length}\r\n\r\n{body}"
    )?;
    let mut reader = BufReader::new(stream);
    let status = line(&mut reader)?;
    let mut length = 0;
    loop {
        let header = line(&mut reader)?.to_lowercase();
        match header.strip_prefix("content-length: ") {
            _ if header.is_empty() => break,
            Some(value) => length = value.parse()?,
            None => {}
        }
    }
    let mut answer = vec![0; length];
    reader.read_exact(&mut answer)?;
    Ok((status, String::from_utf8(answer)?))
}

#[test]
fn the_first_line_is_the_port_and_count_lines_count_until_the_input_closes() -> Tested {
    let mut backend = start()?;
    assert_eq!(ask(&mut backend, "count")?, "0");
    let (status, body) = post(backend.3, "/generic/v1/systemone", DECIDE)?;
    assert_eq!(status, "HTTP/1.1 200 X");
    assert_eq!(
        body,
        r#"{"model":"m","answers":{"q1":{"type":"noul","noul":0.9}}}"#
    );
    assert_eq!(ask(&mut backend, "count")?, "1");
    assert_eq!(finish(backend)?, "1");
    Ok(())
}

#[test]
fn a_held_reply_waits_for_a_release_line() -> Tested {
    let mut backend = start()?;
    let port = backend.3;
    let (answered, answers) = channel();
    thread::spawn(move || answered.send(post(port, "/arm/held/v1/systemone", DECIDE).ok()));
    let deadline = Instant::now() + Duration::from_secs(5);
    while ask(&mut backend, "count")? != "1" {
        assert!(Instant::now() < deadline, "the held request never arrived");
        thread::sleep(Duration::from_millis(10));
    }
    let early = answers.recv_timeout(Duration::from_millis(300));
    assert_eq!(
        early,
        Err(RecvTimeoutError::Timeout),
        "a held reply went before its release"
    );
    ask(&mut backend, "release")?;
    let answer = answers
        .recv_timeout(Duration::from_secs(5))?
        .ok_or("the held request failed")?;
    assert_eq!(answer.0, "HTTP/1.1 200 X");
    assert_eq!(finish(backend)?, "1");
    Ok(())
}

/// Every address in 127.0.0.0/8 is loopback on Linux, so a socket bound to
/// every address answers 127.0.0.2 and one bound to 127.0.0.1 alone refuses it.
#[cfg(target_os = "linux")]
#[test]
fn it_binds_127_0_0_1_only() -> Tested {
    let backend = start()?;
    assert!(TcpStream::connect(("127.0.0.1", backend.3)).is_ok());
    let elsewhere = TcpStream::connect(("127.0.0.2", backend.3));
    assert!(
        elsewhere.is_err(),
        "the backend answered on another address"
    );
    assert_eq!(finish(backend)?, "0");
    Ok(())
}
