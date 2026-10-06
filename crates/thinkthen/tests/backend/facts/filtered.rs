//! Filtered output still accounts for every observed request.

use super::{answered, line};
use crate::batching::{self, KEY, QUESTION};
use crate::child::Folder;
use crate::harness::{Listener, spawn};
use serde_json::{Value, json};
use std::{fs, path::Path};

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[test]
fn the_facts_line_counts_filtered_records_and_matches_status() {
    let home = Path::new(env!("CARGO_TARGET_TMPDIR")).join("facts-filter-home");
    let _removed = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).expect("private home");
    let state = Folder::Usage.variable(&home);
    let listener = Listener::answering(answered).expect("loopback");
    let input = batching::lines(1..=25);
    let arguments = [
        "filter",
        QUESTION,
        "--lines",
        "--batch",
        "10",
        "--facts",
        "--no-cache",
        "--url",
        listener.base(),
    ];
    let output = spawn(
        &arguments,
        &[KEY, (state.0, state.1.as_str())],
        input.as_bytes(),
    )
    .expect("compiled filter");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        output.stdout.iter().filter(|&&byte| byte == b'\n').count(),
        11
    );
    assert_eq!(listener.count(), 3);
    let facts = line(&output);
    let call_id = facts["call_id"]
        .as_str()
        .expect("started call identity")
        .parse::<thinkthen::CallId>()
        .expect("validated call identity");
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    for request in requests {
        assert_eq!(
            request.header("X-ThinkThen-Call-Id"),
            Some(call_id.as_str())
        );
    }
    let mut pinned = facts.clone();
    pinned
        .as_object_mut()
        .expect("facts object")
        .remove("seconds");
    pinned
        .as_object_mut()
        .expect("facts object")
        .remove("call_id");
    assert_eq!(
        pinned,
        json!({"schema":"thinkthen.run/1","records":25,"requests_sent":3,
            "retries":0,"cache_answers":0,"input_tokens":300,"output_tokens":30,
            "model":"jev-1.13.0"})
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr).lines().count(), 1);
    let status =
        spawn(&["status", "--json"], &[(state.0, &state.1)], b"").expect("compiled status");
    let status: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    for field in [
        "requests_sent",
        "retries",
        "cache_answers",
        "input_tokens",
        "output_tokens",
    ] {
        assert_eq!(facts[field], status["usage"]["total"][field], "{field}");
    }
    let ordinary = spawn(
        &[
            "filter",
            QUESTION,
            "--lines",
            "--batch",
            "10",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[KEY],
        input.as_bytes(),
    )
    .expect("ordinary filter");
    assert_eq!(ordinary.status.code(), Some(0));
    assert_eq!(ordinary.stdout, output.stdout);
    assert!(ordinary.stderr.is_empty());
}
