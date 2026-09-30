//! SIGINT and SIGTERM cooperatively stop new attempts and preserve completed output.

use conformance_backend::Rendezvous;
use std::fs;
use std::io::{self, Write as _};
use std::os::unix::process::ExitStatusExt as _;
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::harness::{Canned, Listener, Observed, finish};

mod facts_flush;

const YES: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
const PICKED: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"u002","#,
    r#""probabilities":{"u001":0.1,"u002":0.9}}}}"#,
);
const RECOGNIZED: &str = r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"IN","probabilities":{"IN":1.0,"OUT":0.0}}}}"#;
const RELATED: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}}}"#;

struct Acknowledgment(std::path::PathBuf);

impl Acknowledgment {
    fn new() -> Self {
        static PATHS: AtomicUsize = AtomicUsize::new(0);
        let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
            "interrupt-ack-{}-{}",
            std::process::id(),
            PATHS.fetch_add(1, Ordering::Relaxed)
        ));
        let _removed = fs::remove_file(&path);
        Self(path)
    }

    fn wait(&self) -> io::Result<()> {
        let deadline = std::time::Instant::now() + Duration::from_secs(30);
        while std::time::Instant::now() < deadline {
            if matches!(fs::read(&self.0), Ok(byte) if byte == b"1") {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "SIGINT acknowledgment",
        ))
    }
}

impl Drop for Acknowledgment {
    fn drop(&mut self) {
        let _removed = fs::remove_file(&self.0);
        let _removed = fs::remove_dir_all(self.0.with_extension("home"));
    }
}

fn spawn(arguments: &[&str], input: &[u8], acknowledgment: &Acknowledgment) -> io::Result<Child> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", acknowledgment.0.with_extension("home"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .env("THINKTHEN_TEST_RETRY_WAIT_MS", "5000")
        .env("THINKTHEN_TEST_SIGINT_ACK", &acknowledgment.0)
        .env("THINKTHEN_BATCH", "1")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("input pipe"))?;
    stdin.write_all(input)?;
    drop(stdin);
    Ok(child)
}

/// Hold `count` requests, send SIGINT, then release them all.
fn held(
    count: usize,
    arguments: &[&str],
    input: &[u8],
    reply: impl Fn() -> Canned + Send + Sync + 'static,
) -> io::Result<Output> {
    held_with_signal(
        signal_hook::consts::signal::SIGINT,
        count,
        arguments,
        input,
        reply,
    )
}

fn held_with_signal(
    signal: i32,
    count: usize,
    arguments: &[&str],
    input: &[u8],
    reply: impl Fn() -> Canned + Send + Sync + 'static,
) -> io::Result<Output> {
    let acknowledgment = Acknowledgment::new();
    let release = Arc::new(Rendezvous::new(count + 1));
    let backend_release = Arc::clone(&release);
    let (events_send, events) = channel();
    let listener = Listener::answering_with_events(
        move |_| reply().after_release(Arc::clone(&backend_release)),
        events_send,
    )?;
    let fixed = ["--url", listener.base(), "--model", "local-1"];
    let child = spawn(&[arguments, &fixed].concat(), input, &acknowledgment)?;
    for _ in 0..count {
        assert!(matches!(
            events.recv_timeout(Duration::from_secs(30)),
            Ok(Observed::Request)
        ));
    }
    let flag = if signal == signal_hook::consts::signal::SIGTERM {
        "-TERM"
    } else {
        "-INT"
    };
    assert!(
        crate::child::command("kill", &[])
            .args([flag, &child.id().to_string()])
            .status()?
            .success()
    );
    acknowledgment.wait()?;
    release.wait();
    let output = finish(child, "the interrupted command")?;
    assert_eq!(
        output.status.signal(),
        Some(signal),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.requests().len(), count);
    Ok(output)
}

fn finish_promptly(mut child: Child, what: &str) -> io::Result<Output> {
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    loop {
        if child.try_wait()?.is_some() {
            return finish(child, what);
        }
        if std::time::Instant::now() >= deadline {
            child.kill()?;
            child.wait()?;
            return Err(io::Error::new(io::ErrorKind::TimedOut, what));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// A held reply ends only at the command's one-second attempt timeout.
fn signal_before_timeout(
    arguments: &[&str],
    input: &[u8],
    first: &str,
    second: Option<&str>,
) -> io::Result<Output> {
    let acknowledgment = Acknowledgment::new();
    let release = Arc::new(Rendezvous::new(2));
    let backend_release = Arc::clone(&release);
    let (events_send, events) = channel();
    let listener = Listener::answering_with_events(
        move |_| Canned::status(503, "{}").after_release(Arc::clone(&backend_release)),
        events_send,
    )?;
    let fixed = ["--url", listener.base(), "--model", "local-1"];
    let child = spawn(&[arguments, &fixed].concat(), input, &acknowledgment)?;
    assert!(matches!(
        events.recv_timeout(Duration::from_secs(30)),
        Ok(Observed::Request)
    ));
    assert!(
        crate::child::command("kill", &[])
            .args([first, &child.id().to_string()])
            .status()?
            .success()
    );
    acknowledgment.wait()?;
    if let Some(signal) = second {
        assert!(
            crate::child::command("kill", &[])
                .args([signal, &child.id().to_string()])
                .status()?
                .success()
        );
    }
    let output = finish_promptly(child, "signal-stopped request");
    release.wait();
    let output = output?;
    assert_eq!(listener.requests().len(), 1);
    Ok(output)
}

#[test]
fn a_signal_after_a_hung_request_is_the_stop() {
    type Case<'a> = (&'a [&'a str], &'a [u8], &'a str);
    let cases: [Case<'_>; 3] = [
        (
            &[
                "decide",
                "Accepted?",
                "--lines",
                "--jobs",
                "1",
                "--timeout",
                "1",
                "--max-retries",
                "0",
            ],
            b"first\n",
            "thinkthen: stopped by a signal; 0 records finished\n",
        ),
        (
            &[
                "decide",
                "Accepted?",
                "--timeout",
                "1",
                "--max-retries",
                "0",
            ],
            b"first",
            "",
        ),
        (
            &[
                "decide",
                "Accepted?",
                "--lines",
                "--batch",
                "2",
                "--jobs",
                "1",
                "--timeout",
                "1",
                "--max-retries",
                "0",
            ],
            b"first\nsecond\n",
            "thinkthen: stopped by a signal; 0 records finished\n",
        ),
    ];
    for (arguments, input, stderr) in cases {
        let output = signal_before_timeout(arguments, input, "-INT", None)
            .expect("interrupted request ends");
        assert_eq!(
            output.status.signal(),
            Some(signal_hook::consts::signal::SIGINT)
        );
        assert!(output.stdout.is_empty());
        assert_eq!(String::from_utf8_lossy(&output.stderr), stderr);
    }
}

#[test]
fn the_facts_line_follows_the_stop_line_before_sigterm() {
    let output = held_with_signal(
        signal_hook::consts::signal::SIGTERM,
        1,
        &[
            "decide",
            "Accepted?",
            "--lines",
            "--jobs",
            "1",
            "--batch",
            "1",
            "--max-retries",
            "0",
            "--facts",
        ],
        b"first\nsecond\n",
        || Canned::status(503, "busy"),
    )
    .expect("interrupted facts run");
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut lines = stderr.lines();
    assert_eq!(
        lines.next(),
        Some("thinkthen: stopped by a signal; 0 records finished")
    );
    let facts: serde_json::Value =
        serde_json::from_str(lines.next().expect("facts line")).expect("run facts JSON");
    assert!(lines.next().is_none());
    assert_eq!(facts["records"], 0);
    assert_eq!(facts["requests_sent"], 1);
    assert!(facts.get("model").is_none());
    assert_eq!(
        facts["stopped"],
        serde_json::json!({"cause":"cancelled","retryable":false})
    );
}

#[test]
fn a_second_signal_of_either_kind_ends_the_run_at_once() {
    for (first, second, exited) in [
        ("-INT", "-TERM", signal_hook::consts::signal::SIGTERM),
        ("-TERM", "-INT", signal_hook::consts::signal::SIGINT),
    ] {
        let output = signal_before_timeout(
            &[
                "decide",
                "Accepted?",
                "--lines",
                "--jobs",
                "1",
                "--timeout",
                "4",
                "--max-retries",
                "0",
                "--facts",
            ],
            b"first\n",
            first,
            Some(second),
        )
        .expect("second signal ends without waiting for the reply");
        assert_eq!(output.status.signal(), Some(exited));
        assert!(output.stdout.is_empty());
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn record_finishes_the_started_row_stops_before_another_and_completes_cache() {
    for signal in [
        signal_hook::consts::signal::SIGINT,
        signal_hook::consts::signal::SIGTERM,
    ] {
        let cache = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("interrupt-cache-{}-{signal}", std::process::id()));
        let _removed = fs::remove_dir_all(&cache);
        let cache_name = cache.to_string_lossy();
        let output = held_with_signal(
            signal,
            1,
            &[
                "decide",
                "Is it accepted?",
                "--lines",
                "--jobs",
                "1",
                "--cache",
                &cache_name,
            ],
            b"first\nsecond\n",
            || Canned::ok(YES),
        )
        .expect("interrupt run");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "{\"input\":\"first\",\"value\":true}\n",
            "signal {signal}: stderr={}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "thinkthen: stopped by a signal; 1 record finished, 0 records from a recording\n"
        );
        let stored = crate::support::stored(&cache).expect("the cache");
        assert_eq!(stored.len(), 1, "the first record's one answer");
    }
}

/// A batched run keeps today's cancellation line, and no new batch starts.
#[test]
fn an_interrupted_batch_finishes_and_starts_no_other() {
    let output = held(
        1,
        &[
            "decide",
            "Is it accepted?",
            "--lines",
            "--jobs",
            "1",
            "--batch",
            "2",
        ],
        b"first\nsecond\nthird\nfourth\n",
        || Canned::ok(RELATED),
    )
    .expect("interrupt run");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"input\":\"first\",\"value\":true}\n{\"input\":\"second\",\"value\":false}\n"
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: stopped by a signal; 2 records finished\n"
    );
}

#[test]
fn sent_single_decide_and_aggregate_commands_flush_before_sigint_status() {
    let cases = [
        (
            YES,
            vec!["decide", "Is it accepted?"],
            b"evidence".as_slice(),
            "true\n",
        ),
        (
            PICKED,
            vec!["find", "Which unit answers?"],
            b"first\nsecond\n".as_slice(),
            "second\n",
        ),
        (
            RELATED,
            vec!["relate", "calls=service:service", "--no-cache"],
            br#"[{"name":"gateway","kind":"service"},{"name":"billing","kind":"service"}]"#,
            "{\"relation\":\"calls\",\"source\":{\"name\":\"gateway\",\"kind\":\"service\"},\"target\":{\"name\":\"billing\",\"kind\":\"service\"},\"probability\":0.9}\n",
        ),
    ];
    for (answer, command, input, expected) in cases {
        let output = held(1, &command, input, move || Canned::ok(answer)).expect("interrupt run");
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn sigint_during_retry_wait_makes_exactly_one_request() {
    let output = held(
        1,
        &["decide", "Is it accepted?", "--max-retries", "2"],
        b"evidence",
        || Canned::status(500, "retry"),
    )
    .expect("interrupt run");
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn sigterm_after_a_check_probe_fails_prints_no_report() {
    let output = held_with_signal(
        signal_hook::consts::signal::SIGTERM,
        1,
        &["check"],
        b"",
        || Canned::status(401, "unauthorized"),
    )
    .expect("interrupted check ends");
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

/// Ticket 0143: a split text holds up to the default width of 4 in flight.
/// The text makes 14 one-question chunks, and none starts after the signal.
#[test]
fn sigint_between_recognition_chunks_starts_no_later_chunk() {
    let profile = Path::new(env!("CARGO_TARGET_TMPDIR")).join("recognize-interrupt-profile.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    )
    .expect("profile");
    let output = held(
        4,
        &[
            "recognize",
            "--profile",
            &profile.to_string_lossy(),
            "--no-cache",
        ],
        b"Ada met Bob at Acme in Paris",
        || Canned::ok(RECOGNIZED),
    )
    .expect("recognize stops");
    assert!(output.stdout.is_empty());
}

/// Ticket 0133: the carrier thread cannot spawn under `RLIMIT_NPROC`, the
/// operating system's own limit. Root ignores the limit, so this fails as root.
#[cfg(target_os = "linux")]
#[test]
fn a_carrier_that_cannot_spawn_is_a_defect_and_sends_nothing() {
    let listener = Listener::answering(|_| Canned::ok(YES)).expect("listener");
    let home = Path::new(env!("CARGO_TARGET_TMPDIR")).join("nproc-home");
    let output = crate::child::command("prlimit", &[])
        .args(["--nproc=1:", env!("CARGO_BIN_EXE_thinkthen")])
        .args(["decide", "Is it accepted?", "--model", "local-1"])
        .env("HOME", &home)
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .env("THINKTHEN_BASE_URL", listener.base())
        .stdin(Stdio::null())
        .output()
        .expect("prlimit runs");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let expected = "thinkthen: defect: SIGINT routing could not be activated\n";
    let why = "RLIMIT_NPROC must bind this user; root ignores it";
    assert_eq!(
        (output.status.code(), stderr.as_ref()),
        (Some(70), expected),
        "{why}"
    );
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
}
