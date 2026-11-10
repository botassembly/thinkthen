#![cfg(test)]
#![cfg(feature = "command")]
//! Explicit recording/replay through the real command and its ordinary stream format.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;
use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use std::{io::Write, process::Stdio};

fn invoke(
    server: &Loopback,
    store: &std::path::Path,
    mode: &str,
    key: bool,
    text: &str,
    saved: Option<&std::path::Path>,
) -> std::process::Output {
    let mut command = support::command();
    if let Some(saved) = saved {
        command.arg("run").arg(saved);
    } else {
        command.args(["decide", "Relevant?"]);
    }
    command
        .args([
            "--json",
            "--usage",
            "--url",
            server.base(),
            "--response-store",
        ])
        .arg(store)
        .args([
            "--response-mode",
            mode,
            "--response-namespace",
            "example",
            "--model-generation",
            "revision-1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if key {
        command.env("THINKTHEN_API_KEY", KEY);
    }
    let mut child = command.spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}
fn usage(output: &std::process::Output) -> Value {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .find(|value| value["kind"] == "thinkthen.usage/1")
        .unwrap()
}
#[test]
fn record_then_replay_without_credentials_has_identical_rows_and_zero_sends() {
    let server=Loopback::start(|_|Reply::json(200,&json!({"answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":21,"output_tokens":0}}))).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let store = directory.path().join("responses");
    let recorded = invoke(&server, &store, "record", true, "evidence", None);
    assert!(
        recorded.status.success(),
        "{}",
        String::from_utf8_lossy(&recorded.stderr)
    );
    assert_eq!(usage(&recorded)["usage"]["attempts"], 1);
    let replayed = invoke(&server, &store, "replay", false, "evidence", None);
    assert!(
        replayed.status.success(),
        "{}",
        String::from_utf8_lossy(&replayed.stderr)
    );
    assert_eq!(recorded.stdout, replayed.stdout);
    assert_eq!(usage(&replayed)["usage"]["attempts"], 0);
    assert_eq!(usage(&replayed)["usage"]["replay_hits"], 1);
    assert_eq!(usage(&replayed)["usage"]["input_tokens"]["reported"], 0);
    let missed = invoke(&server, &store, "replay", false, "different evidence", None);
    assert_eq!(missed.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&missed.stderr).contains("local.replay-miss"));
    assert_eq!(server.count(), 1);
}
#[test]
fn incomplete_or_invalid_retention_configuration_refuses_before_credentials() {
    for extra in [
        vec!["--response-mode", "record"],
        vec!["--response-max-bytes", "0"],
    ] {
        let output = support::command()
            .args(["decide", "Relevant?"])
            .args(extra)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(!String::from_utf8_lossy(&output.stderr).contains("key-missing"));
    }
}

#[test]
fn saved_function_steps_record_and_replay_without_keys_or_new_sends() {
    let server = Loopback::start(|_| {
        Reply::json(200, &json!({"answers":{"q1":{"type":"noul","noul":0.9}}}))
    })
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let store = directory.path().join("responses");
    let saved = directory.path().join("run.json");
    let question = json!({"question":{"function":"decide","question":"Relevant?"}});
    std::fs::write(
        &saved,
        json!({"run":[question.clone(),question]}).to_string(),
    )
    .unwrap();
    let recorded = invoke(
        &server,
        &store,
        "record",
        true,
        "\"evidence\"\n",
        Some(&saved),
    );
    assert!(
        recorded.status.success(),
        "{}",
        String::from_utf8_lossy(&recorded.stderr)
    );
    let replayed = invoke(
        &server,
        &store,
        "replay",
        false,
        "\"evidence\"\n",
        Some(&saved),
    );
    assert!(
        replayed.status.success(),
        "{}",
        String::from_utf8_lossy(&replayed.stderr)
    );
    assert_eq!(recorded.stdout, replayed.stdout);
    assert_eq!(usage(&replayed)["usage"]["attempts"], 0);
    assert_eq!(usage(&replayed)["usage"]["replay_hits"], 2);
    assert_eq!(server.count(), 2);
}

#[test]
fn offline_commands_refuse_store_options_without_creating_paths() {
    let directory = tempfile::tempdir().unwrap();
    let store = directory.path().join("must-not-exist");
    for args in [
        vec!["recover", "missing"],
        vec![
            "audit",
            "missing",
            "--manifest",
            "missing",
            "--key",
            "missing",
        ],
        vec![
            "diff",
            "missing",
            "--manifest",
            "missing",
            "--threshold",
            "0.5",
        ],
    ] {
        let output = support::command()
            .args(args)
            .arg("--response-store")
            .arg(&store)
            .env("THINKTHEN_MAX_REQUESTS", "invalid-and-irrelevant")
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("usage.option"));
        assert!(!store.exists());
    }
}
