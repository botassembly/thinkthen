//! The bounds between record input and ordered output.

use conformance_backend::Rendezvous;
#[cfg(unix)]
use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
use std::io::{self, BufRead, BufReader, Write};
#[cfg(unix)]
use std::os::fd::AsFd;
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use crate::harness::{Canned, Listener, Observed, finish};

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
    extra: &[&str],
) -> io::Result<(Child, ChildStdin, Receiver<String>)> {
    let (child, input, output) = raw_child(base, jobs, extra)?;
    Ok((child, input, lines(output)))
}

fn lines(output: ChildStdout) -> Receiver<String> {
    let (send, receive) = mpsc::channel();
    thread::spawn(move || {
        for line in BufReader::new(output).lines() {
            let Ok(line) = line else { return };
            if send.send(line).is_err() {
                return;
            }
        }
    });
    receive
}

/// Whether bytes wait in the pipe now, without reading or blocking.
#[cfg(unix)]
fn written(output: &ChildStdout) -> bool {
    let mut polled = [PollFd::new(output.as_fd(), PollFlags::POLLIN)];
    poll(&mut polled, PollTimeout::ZERO).is_ok_and(|ready| ready == 1)
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
    let deadline = Instant::now() + Duration::from_secs(30);
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
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("scheduling-recording-{}", std::process::id()));
    let _removed = std::fs::remove_dir_all(&folder);
    let folder = folder.to_string_lossy();
    let rows: [(Option<&str>, &[&str]); 4] = [
        (Some("1"), &[]),
        (None, &[]),
        (None, &["--record", &folder]),
        (None, &["--replay", &folder]),
    ];
    // One listener serves every row, because a recording's entry names its address.
    let listener = Listener::answering(|body| Canned::ok(&answered(ordinal(body))))
        .expect("a loopback listener");
    for (jobs, extra) in rows {
        let before = listener.count();
        let (mut child, mut input, output) =
            interactive(listener.base(), jobs, extra).expect("the compiled binary runs");

        for place in 1..=2 {
            input
                .write_all(record(place).as_bytes())
                .expect("one record is written");
            input.flush().expect("the record reaches the process");
            let line = output.recv_timeout(Duration::from_secs(30));
            if line.is_err() {
                let _ = child.kill();
                let _ = child.wait();
            }
            let line = line.expect("the answer arrives before another record");
            assert!(line.contains(&format!(r#""id":"R-{place}""#)), "{line}");
        }

        drop(input);
        assert_eq!(
            finish(child, "the scheduled command")
                .expect("the process ends")
                .status
                .code(),
            Some(0)
        );
        if extra.first() == Some(&"--replay") {
            assert_eq!(listener.count(), before, "a replay sends nothing");
        }
    }
}

#[cfg(unix)]
#[test]
fn ordered_output_bounds_every_dispatched_row() {
    let release = Arc::new(Rendezvous::new(2));
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
    let (mut child, mut input, output) =
        raw_child(listener.base(), Some("4"), &["--batch", "1"]).expect("the compiled binary runs");
    input
        .write_all(records(8).as_bytes())
        .expect("the records are written");

    // The unemitted window holds (jobs + 1) x --batch inputs, which is five
    // here, so records 2 to 5 answer while record 1 is held, and record 6
    // waits for record 1's row.
    for _ in 0..5 {
        let started = events.recv_timeout(Duration::from_secs(10));
        if started.is_err() {
            let _ = child.kill();
            let _ = child.wait();
        }
        assert!(
            matches!(started, Ok(Observed::Request)),
            "five requests start before record 1 is released"
        );
    }
    for _ in 0..4 {
        completed
            .recv_timeout(Duration::from_secs(30))
            .expect("rows 2 through 5 answer while row 1 is held");
    }
    // A late sixth request only makes this pass wrongly, never fail wrongly.
    thread::sleep(Duration::from_millis(200));
    assert_eq!(
        listener.count(),
        5,
        "request 6 waits while record 1 is held"
    );
    assert!(
        !written(&output),
        "no row is written while record 1 is held"
    );

    // Nothing reads standard output yet, so the pipe holds whatever the
    // command wrote. The command writes record 1's row before it sends
    // request 6, so the row waits in the pipe when request 6 arrives.
    // Issue: sdlc/issues/closed/2026-09-30-ordered-output-test-races-the-next-request-under-load.md
    release.wait();
    let sixth = events.recv_timeout(Duration::from_secs(10));
    if sixth.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    assert!(
        matches!(sixth, Ok(Observed::Request)),
        "request 6 starts after record 1 is released"
    );
    assert!(
        written(&output),
        "request 6 started before record 1 reached standard output"
    );
    let output = lines(output);
    drop(input);

    let rows: Vec<String> = (0..8)
        .map(|_| {
            output
                .recv_timeout(Duration::from_secs(10))
                .expect("an ordered answer")
        })
        .collect();
    assert_eq!(
        finish(child, "the scheduled command")
            .expect("the process ends")
            .status
            .code(),
        Some(0)
    );
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
