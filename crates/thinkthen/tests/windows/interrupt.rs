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
    let mut child = process::Owned::spawn(&mut command).expect("isolated CLI");
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
    process::inject(child.id(), "console_injector");
    process::wait(&acknowledgment);
    assert!(child.alive());
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
