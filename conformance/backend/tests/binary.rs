//! The binary a binding's tests start: its port line, its `count`, `release`,
//! `round`, and `wait` lines, the held and delay arms, bind, and exit.

use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::thread;
use std::time::{Duration, Instant};

use conformance_backend::Backend;

type Tested = Result<(), Box<dyn Error>>;

/// One decide question in the wire shape the engine sends.
const DECIDE: &str =
    r#"{"state":"x","model":"m","questions":{"q1":{"type":"noul","instructions":"Is it?"}}}"#;

/// The generic answer to [`DECIDE`].
const ANSWER: &str = r#"{"model":"m","answers":{"q1":{"type":"noul","noul":0.9}}}"#;

const HELD: &str = "/arm/held/v1/systemone";
const WHOLE: &str = "the delay arm needs a whole number of milliseconds";
const CEILING: &str = "the delay arm allows at most 10000 milliseconds";

/// How long any one output line may take before the test fails.
const LINE: Duration = Duration::from_secs(7);

/// A started backend: its process, its input, its output lines, and its port.
struct Started(Child, ChildStdin, Receiver<String>, u16);

/// A status line and a body, or `None` when the request failed.
type Answer = Option<(String, String)>;

fn start() -> Result<Started, Box<dyn Error>> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_conformance-backend"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let input = child.stdin.take().ok_or("no standard input")?;
    let output = BufReader::new(child.stdout.take().ok_or("no standard output")?);
    let (sender, lines) = channel();
    thread::spawn(move || {
        for text in output.lines().map_while(Result::ok) {
            if sender.send(text).is_err() {
                return;
            }
        }
    });
    let port = lines.recv_timeout(LINE)?.parse()?;
    Ok(Started(child, input, lines, port))
}

/// Send one line.
fn send(backend: &mut Started, text: &str) -> Tested {
    writeln!(backend.1, "{text}")?;
    Ok(backend.1.flush()?)
}

/// Send one line and read the next line out.
fn ask(backend: &mut Started, text: &str) -> Result<String, Box<dyn Error>> {
    send(backend, text)?;
    Ok(backend.2.recv_timeout(LINE)?)
}

/// Close standard input, read every line left, and require the exit.
fn finish(Started(mut child, input, lines, _): Started) -> Result<Vec<String>, Box<dyn Error>> {
    drop(input);
    let mut rest = Vec::new();
    loop {
        match lines.recv_timeout(LINE) {
            Ok(text) => rest.push(text),
            Err(RecvTimeoutError::Disconnected) => break,
            Err(RecvTimeoutError::Timeout) => return Err("the output never ended".into()),
        }
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    while child.try_wait()?.is_none() {
        if Instant::now() > deadline {
            child.kill()?;
            return Err("the backend outlived its standard input".into());
        }
        thread::sleep(Duration::from_millis(10));
    }
    Ok(rest)
}

/// The final count, skipping any `wait` line that was still pending.
fn last(backend: Started) -> Result<String, Box<dyn Error>> {
    let rest = finish(backend)?;
    let count = rest
        .into_iter()
        .rev()
        .find(|text| !text.starts_with("wait "));
    Ok(count.ok_or("no final count")?)
}

fn read_text(output: &mut impl BufRead) -> Result<String, Box<dyn Error>> {
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
    let status = read_text(&mut reader)?;
    let mut length = 0;
    loop {
        let header = read_text(&mut reader)?.to_lowercase();
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

/// Post [`DECIDE`] on a thread of its own and hand back where its answer lands.
fn posting(port: u16, path: &str) -> Receiver<Answer> {
    let (answered, answer) = channel();
    let path = path.to_owned();
    thread::spawn(move || answered.send(post(port, &path, DECIDE).ok()));
    answer
}

/// Require no answer yet, 300 ms from now.
fn still_held(answer: &Receiver<Answer>) {
    let early = answer.recv_timeout(Duration::from_millis(300));
    assert_eq!(early, Err(RecvTimeoutError::Timeout), "a reply went early");
}

/// Require the generic answer within 1 s.
fn answered(answer: &Receiver<Answer>) -> Tested {
    let (status, body) = answer
        .recv_timeout(Duration::from_secs(1))?
        .ok_or("the request failed")?;
    assert_eq!((status.as_str(), body.as_str()), ("HTTP/1.1 200 X", ANSWER));
    Ok(())
}

#[test]
fn the_first_line_is_the_port_and_count_lines_count_until_the_input_closes() -> Tested {
    let mut backend = start()?;
    assert_eq!(ask(&mut backend, "count")?, "0");
    let (status, body) = post(backend.3, "/generic/v1/systemone", DECIDE)?;
    assert_eq!((status.as_str(), body.as_str()), ("HTTP/1.1 200 X", ANSWER));
    assert_eq!(ask(&mut backend, "count")?, "1");
    assert_eq!(finish(backend)?, ["1"]);
    Ok(())
}

#[test]
fn a_held_reply_waits_for_a_release_line() -> Tested {
    let mut backend = start()?;
    let answer = posting(backend.3, HELD);
    assert_eq!(ask(&mut backend, "wait 1")?, "wait 1");
    still_held(&answer);
    send(&mut backend, "release")?;
    answered(&answer)?;
    assert_eq!(finish(backend)?, ["1"]);
    Ok(())
}

#[test]
fn the_delay_arm_answers_after_its_delay() -> Tested {
    let backend = start()?;
    let began = Instant::now();
    answered(&posting(backend.3, "/arm/delay/200/v1/systemone"))?;
    assert!(
        began.elapsed() >= Duration::from_millis(200),
        "it answered early"
    );
    assert_eq!(last(backend)?, "1");
    Ok(())
}

#[test]
fn eight_delayed_replies_wait_in_parallel() -> Tested {
    let backend = start()?;
    let began = Instant::now();
    let answers: Vec<_> = (0..8)
        .map(|_| posting(backend.3, "/arm/delay/200/v1/systemone"))
        .collect();
    for answer in &answers {
        answered(answer)?;
    }
    assert!(
        began.elapsed() < Duration::from_millis(800),
        "the delays ran in turn"
    );
    assert_eq!(last(backend)?, "8");
    Ok(())
}

#[test]
fn the_delay_arm_refuses_a_bad_value_and_one_above_its_ceiling_at_once() -> Tested {
    let backend = start()?;
    let refusals = [
        ("abc", WHOLE),
        ("+200", WHOLE),
        ("", WHOLE),
        ("99999999999999999999", WHOLE),
        ("10001", CEILING),
    ];
    for (value, sentence) in refusals {
        let path = format!("/arm/delay/{value}/v1/systemone").replace("//", "/");
        let (status, body) = posting(backend.3, &path)
            .recv_timeout(Duration::from_secs(1))?
            .ok_or("the request failed")?;
        assert_eq!(
            (status.as_str(), body.as_str()),
            ("HTTP/1.1 500 X", sentence),
            "{value}"
        );
    }
    still_held(&posting(backend.3, "/arm/delay/10000/v1/systemone"));
    Ok(())
}

#[test]
fn four_rounds_on_one_backend_each_let_go_only_the_reply_held_then() -> Tested {
    let mut backend = start()?;
    for round in 1..=4 {
        let answer = posting(backend.3, HELD);
        assert_eq!(
            ask(&mut backend, &format!("wait {round}"))?,
            format!("wait {round}")
        );
        still_held(&answer);
        send(&mut backend, "round")?;
        answered(&answer)?;
    }
    assert_eq!(last(backend)?, "4");
    Ok(())
}

#[test]
fn fifty_rounds_back_to_back_each_let_go_the_reply_they_counted() -> Tested {
    let mut backend = start()?;
    let began = Instant::now();
    for round in 1..=50 {
        let answer = posting(backend.3, HELD);
        assert_eq!(
            ask(&mut backend, &format!("wait {round}"))?,
            format!("wait {round}")
        );
        send(&mut backend, "round")?;
        answered(&answer)?;
    }
    assert!(
        began.elapsed() < Duration::from_secs(5),
        "fifty rounds were slow"
    );
    assert_eq!(last(backend)?, "50");
    Ok(())
}

#[test]
fn a_release_stays_open_for_later_held_replies() -> Tested {
    let mut backend = start()?;
    send(&mut backend, "release")?;
    answered(&posting(backend.3, HELD))?;
    assert_eq!(last(backend)?, "1");
    Ok(())
}

#[test]
fn a_wait_line_answers_once_the_count_reaches_it() -> Tested {
    let mut backend = start()?;
    send(&mut backend, "wait 1")?;
    thread::sleep(Duration::from_millis(200));
    let _held = posting(backend.3, HELD);
    assert_eq!(backend.2.recv_timeout(Duration::from_secs(1))?, "wait 1");
    assert_eq!(last(backend)?, "1");
    Ok(())
}

#[test]
fn a_wait_line_gives_up_at_5_s_and_holds_up_no_line_behind_it() -> Tested {
    let mut backend = start()?;
    let began = Instant::now();
    send(&mut backend, "wait 1")?;
    send(&mut backend, "count")?;
    assert_eq!(backend.2.recv_timeout(Duration::from_millis(300))?, "0");
    assert_eq!(backend.2.recv_timeout(LINE)?, "wait 0");
    let waited = began.elapsed();
    assert!(
        waited >= Duration::from_secs(5) && waited < Duration::from_secs(6),
        "{waited:?}"
    );
    assert_eq!(finish(backend)?, ["0"]);
    Ok(())
}

#[test]
fn a_wait_line_with_no_whole_number_prints_nothing() -> Tested {
    let mut backend = start()?;
    for text in ["wait x", "wait +0", "count"] {
        send(&mut backend, text)?;
    }
    assert_eq!(backend.2.recv_timeout(LINE)?, "0");
    let late = backend.2.recv_timeout(Duration::from_millis(300));
    assert_eq!(
        late,
        Err(RecvTimeoutError::Timeout),
        "a bad wait printed a line"
    );
    assert_eq!(finish(backend)?, ["0"]);
    Ok(())
}

/// The port an in-process backend listens on.
fn port(backend: &Backend) -> Result<u16, Box<dyn Error>> {
    Ok(backend
        .origin()
        .rsplit(':')
        .next()
        .ok_or("no port")?
        .parse()?)
}

#[test]
fn a_delay_refusal_says_why_on_standard_error() -> Tested {
    let mut backend = start()?;
    let mut errors = backend.0.stderr.take().ok_or("no standard error")?;
    for path in ["/arm/delay/abc/v1/x", "/arm/delay/10001/v1/x"] {
        post(backend.3, path, DECIDE)?;
    }
    finish(backend)?;
    let mut text = String::new();
    errors.read_to_string(&mut text)?;
    assert_eq!(
        text,
        format!("conformance-backend: {WHOLE}\nconformance-backend: {CEILING}\n")
    );
    Ok(())
}

#[test]
fn closing_the_input_exits_without_waiting_for_a_pending_wait() -> Tested {
    let mut backend = start()?;
    let began = Instant::now();
    send(&mut backend, "wait 1")?;
    assert_eq!(finish(backend)?, ["0"]);
    assert!(began.elapsed() < Duration::from_secs(1), "the exit waited");
    Ok(())
}

#[test]
fn two_backends_in_one_process_keep_their_own_count_gate_and_port() -> Tested {
    let (first, second) = (Backend::start()?, Backend::start()?);
    let (one, two) = (port(&first)?, port(&second)?);
    assert_ne!(one, two);
    let (held, other) = (posting(one, HELD), posting(two, HELD));
    assert_eq!((first.wait(1), second.wait(1)), (1, 1));
    first.round();
    answered(&held)?;
    still_held(&other);
    post(one, "/generic/v1/systemone", DECIDE)?;
    assert_eq!((first.count(), second.count()), (2, 1));
    Ok(())
}

#[test]
fn two_backend_binaries_keep_their_own_count_gate_and_port() -> Tested {
    let (mut first, mut second) = (start()?, start()?);
    assert_ne!(first.3, second.3);
    let (held, other) = (posting(first.3, HELD), posting(second.3, HELD));
    assert_eq!(ask(&mut first, "wait 1")?, "wait 1");
    assert_eq!(ask(&mut second, "wait 1")?, "wait 1");
    send(&mut first, "round")?;
    answered(&held)?;
    still_held(&other);
    post(first.3, "/generic/v1/systemone", DECIDE)?;
    assert_eq!(
        (last(first)?, last(second)?),
        ("2".to_owned(), "1".to_owned())
    );
    Ok(())
}

#[test]
fn twenty_backends_start_at_once_on_twenty_ports() -> Tested {
    let began = Instant::now();
    let started = (0..20).map(|_| start()).collect::<Result<Vec<_>, _>>()?;
    let ports: HashSet<u16> = started.iter().map(|backend| backend.3).collect();
    assert!(
        began.elapsed() < Duration::from_secs(5),
        "the starts were slow"
    );
    assert_eq!(ports.len(), 20);
    for backend in started {
        assert_eq!(finish(backend)?, ["0"]);
    }
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
    assert_eq!(finish(backend)?, ["0"]);
    Ok(())
}
