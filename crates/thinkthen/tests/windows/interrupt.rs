use super::{
    process,
    support::{self, Scratch},
};
use conformance_backend::{Canned, Listener, Observed, Rendezvous};
use std::sync::{Arc, mpsc};
use std::time::Duration;

fn interrupted(success: bool, lines: bool) -> (std::process::Output, serde_json::Value) {
    let scratch = Scratch::new();
    let acknowledgment = scratch.0.join("ack");
    let release = Arc::new(Rendezvous::new(2));
    let backend_release = Arc::clone(&release);
    let (sent, observed) = mpsc::channel();
    let listener = Listener::answering_with_events(
        move |_| {
            (if success {
                Canned::ok(support::ANSWER)
            } else {
                Canned::status(500, "retry fixture")
            })
            .after_release(Arc::clone(&backend_release))
        },
        sent,
    )
    .expect("held counted backend");
    let mut arguments = vec![
        "decide",
        "Accepted?",
        "--no-cache",
        "--facts",
        "--max-retries",
        "2",
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    if lines {
        arguments.extend(["--lines", "--jobs", "1"]);
    }
    let mut command = scratch.command(&arguments);
    command.env("THINKTHEN_TEST_SIGINT_ACK", &acknowledgment);
    let mut child = process::Owned::console(&mut command).expect("isolated CLI");
    use std::io::Write as _;
    child
        .0
        .as_mut()
        .expect("child")
        .stdin
        .take()
        .expect("stdin")
        .write_all(if lines { b"first\nsecond\n" } else { b"first" })
        .expect("input");
    assert!(matches!(
        observed.recv_timeout(Duration::from_secs(30)),
        Ok(Observed::Request)
    ));
    process::inject(child.id(), "console_injector", Some(&acknowledgment));
    if !child.alive() {
        let output = child.finish().expect("early signal exit");
        panic!("admitted request must remain cooperative: {output:?}");
    }
    process::wait(&acknowledgment);
    release.wait();
    let output = child.finish().expect("cooperative CLI exit");
    assert_eq!(output.status.code(), Some(130));
    assert_eq!(listener.requests().len(), 1);
    (output, support::status(&scratch))
}
#[test]
fn native_ctrl_c_preserves_completed_output_then_stop_then_facts_and_flushes_usage() {
    let (output, totals) = interrupted(true, true);
    assert_eq!(output.stdout, b"{\"input\":\"first\",\"value\":true}\n");
    let stderr = String::from_utf8(output.stderr).expect("stderr");
    let mut lines = stderr.lines();
    assert_eq!(
        lines.next(),
        Some("thinkthen: stopped by a signal; 1 record finished")
    );
    let facts: serde_json::Value =
        serde_json::from_str(lines.next().expect("facts after stopped")).expect("facts JSON");
    assert!(lines.next().is_none());
    assert_eq!(facts["records"], 1);
    assert_eq!(facts["requests_sent"], 1);
    assert_eq!(
        facts["stopped"],
        serde_json::json!({"cause":"cancelled","retryable":false})
    );
    assert_eq!(totals["usage"]["total"]["requests_sent"], 1);
    assert_eq!(totals["usage"]["total"]["input_tokens"], 5);
    assert_eq!(totals["usage"]["total"]["output_tokens"], 1);
}
#[test]
fn native_ctrl_c_stops_retry_admission_after_one_counted_request() {
    let (output, totals) = interrupted(false, false);
    assert!(output.stdout.is_empty());
    assert_eq!(totals["usage"]["total"]["requests_sent"], 1);
    assert_eq!(totals["usage"]["total"]["retries"], 0);
}

fn release_interrupted(records: bool) -> (std::process::Output, serde_json::Value) {
    use crate::child::ChildEnvironment as _;
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    let binary = std::path::PathBuf::from(
        std::env::var_os("THINKTHEN_WINDOWS_RELEASE_BINARY")
            .expect("exact release executable supplied by smoke harness"),
    );
    assert!(binary.is_absolute() && binary.is_file() && !binary.is_symlink());
    let scratch = Scratch::new();
    let release = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&release);
    let (sent, observed) = mpsc::channel();
    let listener = Listener::answering_with_events(
        move |_| Canned::ok(support::ANSWER).after_release(Arc::clone(&held)),
        sent,
    )
    .expect("held counted listener");
    let mut arguments = vec![
        "decide",
        "Accepted?",
        "--no-cache",
        "--facts",
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    if records {
        arguments.extend(["--lines", "--jobs", "1", "--batch", "1"]);
    }
    let mut command = Command::new(binary);
    command
        .clear_environment()
        .home(&scratch.0)
        .args(arguments)
        .env("THINKTHEN_API_KEY", "sk-release-fixture-only")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = process::Owned::console(&mut command).expect("isolated release command");
    child
        .0
        .as_mut()
        .expect("owned")
        .stdin
        .take()
        .expect("input")
        .write_all(if records {
            b"fixture evidence\nsecond evidence\n"
        } else {
            b"fixture evidence"
        })
        .expect("input bytes");
    assert!(matches!(
        observed.recv_timeout(Duration::from_secs(30)),
        Ok(Observed::Request)
    ));
    process::inject(child.id(), "console_injector", None);
    // The injector waits for the target's exit before it detaches. The admitted
    // attempt retains its normal 30-second timeout with the answer still held;
    // first Ctrl-C stops further admission rather than aborting that attempt.
    assert!(!child.alive(), "command exits before the injector detaches");
    let output = child
        .finish_after(Duration::from_secs(45))
        .expect("normal attempt timeout and cleanup");
    release.wait();
    assert_eq!(output.status.code(), Some(130));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.requests().len(), 1);
    (output, support::status(&scratch))
}

/// The smoke harness supplies the exact executable extracted from the checked
/// release archive. No hidden product variable or shortened timeout is used.
#[test]
#[ignore = "release smoke supplies an exact reviewed packed command path"]
fn release_binary_console_interrupt() {
    for records in [false, true] {
        let (output, totals) = release_interrupted(records);
        assert_eq!(totals["usage"]["total"]["requests_sent"], 1);
        assert_eq!(totals["usage"]["total"]["retries"], 0);
        let text = String::from_utf8(output.stderr).expect("diagnostic");
        let mut lines = text.lines();
        if records {
            assert_eq!(
                lines.next(),
                Some("thinkthen: stopped by a signal; 0 records finished")
            );
        }
        let mut facts: serde_json::Value =
            serde_json::from_str(lines.next().expect("facts")).expect("facts JSON");
        assert!(lines.next().is_none());
        let seconds = facts
            .as_object_mut()
            .expect("facts object")
            .remove("seconds");
        let elapsed = seconds
            .expect("elapsed time")
            .as_f64()
            .expect("seconds number");
        assert!((0.0..45.0).contains(&elapsed));
        assert_eq!(
            facts,
            serde_json::json!({
                "schema":"thinkthen.run/1", "records":0, "requests_sent":1,
                "retries":0, "cache_answers":0,
                "stopped":{"cause":"cancelled","retryable":false}
            })
        );
    }
}
