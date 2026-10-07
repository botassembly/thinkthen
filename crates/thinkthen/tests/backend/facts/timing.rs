//! Command time keeps input/output work outside overlapping backend intervals.
use super::*;
use std::io::Write;
use std::process::Stdio;
use std::time::Duration;

#[test]
fn command_time_excludes_parallel_http_once_and_keeps_post_reply_input_wait() {
    let together = crate::harness::Gathering::new(2);
    let (written, replies) = std::sync::mpsc::channel();
    let listener = Listener::answering(move |_| {
        together.hold();
        std::thread::sleep(Duration::from_millis(200));
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
            .notifying(written.clone())
    })
    .unwrap();
    let mut child = crate::harness::command(
        &[
            "decide",
            "Good?",
            "--lines",
            "--batch",
            "1",
            "--jobs",
            "2",
            "--facts",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "fixed",
        ],
        &[("THINKTHEN_API_KEY", "timing-private")],
    )
    .stdin(Stdio::piped())
    .spawn()
    .unwrap();
    let mut input = child.stdin.take().unwrap();
    input.write_all(b"First.\nSecond.\n").unwrap();
    for _ in 0..2 {
        replies.recv_timeout(Duration::from_secs(10)).unwrap();
    }
    // This is command input time after the replies, not another HTTP interval.
    std::thread::sleep(Duration::from_millis(200));
    drop(input);
    let output = crate::harness::finish(child, "command time").unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        output.stdout,
        b"{\"input\":\"First.\",\"value\":true}\n{\"input\":\"Second.\",\"value\":true}\n"
    );
    let facts = super::line(&output);
    let command = facts["command_ms"].as_u64().unwrap();
    let total = facts["seconds"].as_f64().unwrap() * 1000.0;
    assert!(command >= 150, "post-reply input wait was lost: {facts}");
    assert!(
        (command as f64) + 150.0 < total,
        "backend interval was not excluded: {facts}"
    );
    assert_eq!(facts["requests_sent"], 2);
    assert_eq!(listener.count(), 2);
    let requests: Vec<_> = listener
        .requests()
        .iter()
        .map(|r| serde_json::from_slice::<Value>(&r.body).unwrap())
        .collect();
    for text in ["First.", "Second."] {
        assert!(requests.contains(&json!({"model":"fixed","state":"Each question quotes the text it asks about.","questions":{"q1":{"type":"noul","instructions":format!("The text is \"{text}\". Good?")}}})));
    }
    assert!(!String::from_utf8_lossy(&output.stderr).contains("timing-private"));
}
