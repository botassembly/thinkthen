//! A signal can arrive after the last row but before usage persistence ends.

use std::fs;
use std::io::Read as _;
use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
use std::os::unix::process::ExitStatusExt as _;
use std::time::Duration;

use serde_json::{Value, json};

use super::Acknowledgment;
use crate::harness::{Canned, Listener, finish, start};

#[test]
fn a_signal_during_usage_flush_still_marks_the_final_facts_line_stopped() {
    let acknowledgment = Acknowledgment::new();
    let home = acknowledgment.0.with_extension("home");
    let cache = home.join("cache");
    let usage = cache.join("thinkthen-usage");
    fs::create_dir_all(&usage).expect("usage folder");
    fs::set_permissions(&usage, fs::Permissions::from_mode(0o700)).expect("private folder");
    let lock = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(usage.join(".lock"))
        .expect("usage lock file");
    lock.lock().expect("hold usage writer");
    let listener = Listener::serving(vec![Canned::ok(concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
        r#""usage":{"input_tokens":5,"output_tokens":1}}"#,
    ))])
    .expect("listener");
    let environment = [
        ("THINKTHEN_API_KEY", "sk-test-value"),
        (
            "THINKTHEN_TEST_SIGINT_ACK",
            acknowledgment.0.to_str().expect("ack path"),
        ),
        ("XDG_CACHE_HOME", cache.to_str().expect("cache path")),
    ];
    let mut child = start(
        &[
            "decide",
            "Accepted?",
            "--facts",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &environment,
        b"first",
    )
    .expect("command");
    let mut row = [0; 5];
    child
        .stdout
        .as_mut()
        .expect("stdout pipe")
        .read_exact(&mut row)
        .expect("finished row before signal");
    assert_eq!(&row, b"true\n");
    assert_eq!(listener.requests().len(), 1);
    std::thread::sleep(Duration::from_millis(50));
    assert!(child.try_wait().expect("still flushing").is_none());
    assert!(
        crate::child::command("kill", &[])
            .args(["-TERM", &child.id().to_string()])
            .status()
            .expect("send SIGTERM")
            .success()
    );
    acknowledgment.wait().expect("signal reached command");
    drop(lock);
    let output = finish(child, "signal during usage flush").expect("command ends");
    assert_eq!(
        output.status.signal(),
        Some(signal_hook::consts::signal::SIGTERM)
    );
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr.lines().count(), 1, "{stderr}");
    let facts: Value = serde_json::from_str(stderr.trim_end()).expect("facts JSON");
    assert_eq!(facts["records"], 1);
    assert_eq!(facts["requests_sent"], 1);
    assert_eq!(
        facts["stopped"],
        json!({"cause":"cancelled","retryable":false})
    );
}
