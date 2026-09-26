//! SIGINT cooperatively stops new attempts and preserves completed output.

use std::fs;
use std::io::{self, Write as _};
use std::os::unix::process::ExitStatusExt as _;
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc, Barrier};
use std::time::Duration;

use crate::harness::{Canned, Listener, Observed, finish};

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
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
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

fn held(
    arguments: &[&str],
    input: &[u8],
    reply: impl Fn() -> Canned + Send + Sync + 'static,
) -> io::Result<Output> {
    let acknowledgment = Acknowledgment::new();
    let release = Arc::new(Barrier::new(2));
    let backend_release = Arc::clone(&release);
    let (events_send, events) = channel();
    let listener = Listener::answering_with_events(
        move |_| reply().after_release(Arc::clone(&backend_release)),
        events_send,
    )?;
    let fixed = ["--url", listener.base(), "--model", "local-1"];
    let child = spawn(&[arguments, &fixed].concat(), input, &acknowledgment)?;
    assert!(matches!(
        events.recv_timeout(Duration::from_secs(5)),
        Ok(Observed::Request)
    ));
    assert!(
        crate::child::command("kill", &[])
            .args(["-INT", &child.id().to_string()])
            .status()?
            .success()
    );
    acknowledgment.wait()?;
    release.wait();
    let output = finish(child, "the interrupted command")?;
    assert_eq!(
        output.status.signal(),
        Some(signal_hook::consts::signal::SIGINT),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.requests().len(), 1);
    Ok(output)
}

#[test]
fn record_finishes_the_started_row_stops_before_another_and_completes_cache() {
    let cache = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("interrupt-cache-{}", std::process::id()));
    let _removed = fs::remove_dir_all(&cache);
    let cache_name = cache.to_string_lossy();
    let output = held(
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
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: stopped at record 2; 1 record finished, 0 records from a recording\n"
    );
    let (_name, entry) = crate::recordings::only_entry(&cache).expect("one cache entry");
    assert!(serde_json::from_str::<serde_json::Value>(&entry).is_ok());
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
        let output = held(&command, input, move || Canned::ok(answer)).expect("interrupt run");
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn sigint_during_retry_wait_makes_exactly_one_request() {
    let output = held(
        &["decide", "Is it accepted?", "--max-retries", "2"],
        b"evidence",
        || Canned::status(500, "retry"),
    )
    .expect("interrupt run");
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn sigint_between_recognition_chunks_starts_no_later_chunk() {
    let profile = Path::new(env!("CARGO_TARGET_TMPDIR")).join("recognize-interrupt-profile.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    )
    .expect("profile");
    let output = held(
        &[
            "recognize",
            "--profile",
            &profile.to_string_lossy(),
            "--no-cache",
        ],
        b"Ada Acme",
        || Canned::ok(RECOGNIZED),
    )
    .expect("recognize stops");
    assert!(output.stdout.is_empty());
}
