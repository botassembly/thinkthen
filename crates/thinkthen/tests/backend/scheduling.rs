//! The bounds between record input and ordered output.

use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};

use crate::harness::{Canned, Listener, Observed};

const QUESTION: &str = "Does this report a payment failure?";
static CHILDREN: AtomicU64 = AtomicU64::new(0);

fn record(place: usize) -> String {
    format!("{{\"id\":\"R-{place}\",\"body\":\"record {place}\"}}\n")
}

fn records(count: usize) -> String {
    (1..=count).map(record).collect()
}

fn ordinal(body: &[u8]) -> usize {
    String::from_utf8_lossy(body)
        .split_once("record ")
        .and_then(|(_, rest)| rest.split(['"', '\\', ' ']).next())
        .and_then(|digits| digits.parse().ok())
        .unwrap_or(0)
}

fn answered(place: usize) -> String {
    let probability = f64::from(u32::try_from(place).unwrap_or(0)) / 100.0;
    format!(
        concat!(
            r#"{{"model":"jev-1.13.0","answers":{{"q1":{{"type":"noul","noul":{probability}}}}},"#,
            r#""usage":{{"input_tokens":88,"output_tokens":12}}}}"#,
        ),
        probability = probability
    )
}

fn interactive(
    base: &str,
    jobs: Option<&str>,
    events: Option<mpsc::Sender<Observed>>,
) -> io::Result<(Child, ChildStdin, Receiver<String>)> {
    let (child, input, output) = raw_child(base, jobs, &[])?;
    let (send, receive) = mpsc::channel();
    thread::spawn(move || {
        for line in BufReader::new(output).lines() {
            let Ok(line) = line else { return };
            if let Some(events) = events.as_ref() {
                let _ = events.send(Observed::Output(line.clone()));
            }
            if send.send(line).is_err() {
                return;
            }
        }
    });
    Ok((child, input, receive))
}

fn raw_child(
    base: &str,
    jobs: Option<&str>,
    extra: &[&str],
) -> io::Result<(Child, ChildStdin, ChildStdout)> {
    let cache_home = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "scheduling-cache-{}-{}",
        std::process::id(),
        CHILDREN.fetch_add(1, Ordering::Relaxed)
    ));
    let mut arguments = vec![
        "decide",
        QUESTION,
        "--url",
        base,
        "--model",
        "local-1",
        "--jsonl",
        "--field",
        "/body",
        "--details",
    ];
    if let Some(jobs) = jobs {
        arguments.extend(["--jobs", jobs]);
    }
    arguments.extend(extra);
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("XDG_CACHE_HOME", cache_home)
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let output = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("no pipe from standard output"))?;
    Ok((child, input, output))
}

fn wait_promptly(child: &mut Child) -> io::Result<Option<ExitStatus>> {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(Some(status));
        }
        if Instant::now() >= deadline {
            child.kill()?;
            child.wait()?;
            return Ok(None);
        }
        thread::yield_now();
    }
}

#[test]
fn an_answer_arrives_before_the_next_record_at_one_job_and_the_default() {
    for jobs in [Some("1"), None] {
        let listener = Listener::answering(|body| Canned::ok(&answered(ordinal(body))))
            .expect("a loopback listener");
        let (mut child, mut input, output) =
            interactive(listener.base(), jobs, None).expect("the compiled binary runs");

        for place in 1..=2 {
            input
                .write_all(record(place).as_bytes())
                .expect("one record is written");
            input.flush().expect("the record reaches the process");
            let line = output.recv_timeout(Duration::from_secs(2));
            if line.is_err() {
                let _ = child.kill();
                let _ = child.wait();
            }
            let line = line.expect("the answer arrives before another record");
            assert!(line.contains(&format!(r#""id":"R-{place}""#)), "{line}");
        }

        drop(input);
        assert_eq!(child.wait().expect("the process ends").code(), Some(0));
    }
}

#[test]
fn ordered_output_bounds_every_dispatched_row() {
    let release = Arc::new(Barrier::new(2));
    let (completed_send, completed) = mpsc::channel();
    let (events_send, events) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let release = Arc::clone(&release);
            let completed = completed_send.clone();
            move |body| {
                let place = ordinal(body);
                let answer = Canned::ok(&answered(place));
                if place == 1 {
                    answer.after_release(Arc::clone(&release))
                } else {
                    answer.notifying(completed.clone())
                }
            }
        },
        events_send.clone(),
    )
    .expect("a loopback listener");
    let (mut child, mut input, output) = interactive(listener.base(), Some("4"), Some(events_send))
        .expect("the compiled binary runs");
    input
        .write_all(records(8).as_bytes())
        .expect("the records are written");

    let deadline = Instant::now() + Duration::from_secs(2);
    let mut started = 0;
    while started < 4 && Instant::now() < deadline {
        started += listener.requests().len();
        thread::yield_now();
    }
    if started != 4 {
        let _ = child.kill();
        let _ = child.wait();
    }
    assert_eq!(
        started, 4,
        "four requests start before record 1 is released"
    );
    for _ in 0..4 {
        assert!(matches!(events.recv(), Ok(Observed::Request)));
    }
    for _ in 0..3 {
        completed
            .recv_timeout(Duration::from_secs(2))
            .expect("rows 2 through 4 answer while row 1 is held");
    }
    assert!(
        listener.requests().is_empty(),
        "a fifth request started early"
    );

    release.wait();
    let first = events
        .recv_timeout(Duration::from_secs(2))
        .expect("an output or request event follows record 1 release");
    let Observed::Output(first) = first else {
        panic!("request 5 started before record 1 reached standard output");
    };
    assert!(first.contains(r#""id":"R-1""#), "{first}");
    drop(input);

    let rows: Vec<String> = (0..8)
        .map(|_| {
            output
                .recv_timeout(Duration::from_secs(2))
                .expect("an ordered answer")
        })
        .collect();
    assert_eq!(child.wait().expect("the process ends").code(), Some(0));
    for (place, row) in rows.iter().enumerate() {
        assert!(row.contains(&format!(r#""id":"R-{}""#, place + 1)), "{row}");
    }
}

#[test]
fn a_backend_failure_exits_while_standard_input_stays_open() {
    let listener = Listener::answering(|_| Canned::status(500, "{}")).expect("a loopback listener");
    let (mut child, mut input, _output) = raw_child(listener.base(), None, &["--max-retries", "0"])
        .expect("the compiled binary runs");
    input
        .write_all(record(1).as_bytes())
        .expect("one record is written");
    input.flush().expect("the record reaches the process");

    let status = wait_promptly(&mut child)
        .expect("the process can be checked")
        .expect("the process exits promptly");
    assert_eq!(status.code(), Some(4));
    drop(input);
}

#[test]
fn a_closed_output_pipe_exits_while_standard_input_stays_open() {
    let listener = Listener::answering(|body| Canned::ok(&answered(ordinal(body))))
        .expect("a loopback listener");
    let (mut child, mut input, output) =
        raw_child(listener.base(), None, &[]).expect("the compiled binary runs");
    input
        .write_all(record(1).as_bytes())
        .expect("record 1 is written");
    input.flush().expect("record 1 reaches the process");
    let mut output = BufReader::new(output);
    let mut first = String::new();
    output.read_line(&mut first).expect("record 1 is printed");
    assert!(first.contains(r#""id":"R-1""#), "{first}");
    drop(output);

    input
        .write_all(record(2).as_bytes())
        .expect("record 2 is written after stdout closes");
    input.flush().expect("record 2 reaches the process");
    let status = wait_promptly(&mut child)
        .expect("the process can be checked")
        .expect("the process exits promptly");
    assert_eq!(status.code(), Some(0));
    drop(input);
}
