#![cfg(test)]
#![cfg(feature = "command")]
//! Offline recovery exports typed answers and distinguishes torn tails from corruption.
mod support;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::process::Stdio;

fn frame(output: &mut Vec<u8>, value: Value) {
    let bytes = serde_json::to_vec(&value).unwrap();
    output.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    output.extend_from_slice(&Sha256::digest(&bytes));
    output.extend_from_slice(&bytes);
}

#[test]
fn offline_export_preserves_occurrences_and_refuses_committed_corruption() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("paid.recovery");
    let mut child = support::command()
        .args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--json",
            "--from",
            "jsonl",
            "--field",
            "/text",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"id\":7,\"text\":\"alpha\"}\n{\"id\":7,\"text\":\"beta\"}\n{\"text\":\"gamma\"}\n{\"text\":\"delta\"}\n")
        .unwrap();
    let baseline = child.wait_with_output().unwrap();
    assert!(baseline.status.success());
    let expected: Vec<Value> = String::from_utf8(baseline.stdout)
        .unwrap()
        .lines()
        .take(3)
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(expected.len(), 3);
    let mut bytes = b"THINKTHEN-RECOVERY-1\n".to_vec();
    frame(
        &mut bytes,
        json!({"kind":"manifest","metadata":{},"admitted":4}),
    );
    for position in 1..=4 {
        frame(
            &mut bytes,
            json!({"kind":"occurrence","position":position,"source":{"row":position+1}}),
        );
    }
    for row in &expected {
        frame(
            &mut bytes,
            json!({"kind":"row","position":row["position"],"row":row}),
        );
    }
    let committed_len = bytes.len();
    bytes.extend_from_slice(&[20, 0, 0]);
    fs::write(&path, &bytes).unwrap();
    let output = support::command()
        .arg("recover")
        .arg(&path)
        .env("THINKTHEN_API_KEY", "invalid")
        .env("THINKTHEN_BACKEND", "invalid")
        .env("THINKTHEN_MAX_REQUESTS", "invalid-inference-setting")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let rows: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows, expected);
    let report: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(report["committed"], 3);
    assert_eq!(report["missing_ranges"], json!([[4, 4]]));
    assert_eq!(report["torn_tail"], true);
    assert_eq!(report["complete"], false);
    for option in [vec!["--max-requests", "0"], vec!["--usage=false"]] {
        let output = support::command()
            .arg("recover")
            .arg(&path)
            .args(option)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(output.stdout.is_empty());
    }
    bytes.truncate(committed_len);
    frame(
        &mut bytes,
        json!({"kind":"closing","complete":false,"emitted":3,"output":null}),
    );
    bytes.extend_from_slice(&[20, 0, 0]);
    fs::write(&path, &bytes).unwrap();
    let after_closing = support::command()
        .arg("recover")
        .arg(&path)
        .output()
        .unwrap();
    assert_eq!(after_closing.status.code(), Some(5), "{after_closing:?}");
    assert!(String::from_utf8_lossy(&after_closing.stderr).contains("after its closing frame"));
    bytes.truncate(committed_len);
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    fs::write(&path, bytes).unwrap();
    let corrupt = support::command()
        .arg("recover")
        .arg(&path)
        .output()
        .unwrap();
    assert_eq!(corrupt.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&corrupt.stderr).contains("checksum"));
}

#[test]
fn explicit_retention_conflicts_and_existing_destinations_fail_before_sends() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.csv");
    let path = directory.path().join("paid.recovery");
    fs::write(&input, b"Body\nalpha\n").unwrap();
    fs::write(&path, b"another operation").unwrap();
    let output = support::command()
        .args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--column",
            "Body",
            "--into",
            "answer",
            "--recovery",
        ])
        .arg(&path)
        .arg(&input)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(5), "{output:?}");
    assert_eq!(fs::read(&path).unwrap(), b"another operation");
    let output = support::command()
        .args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--column",
            "Body",
            "--into",
            "answer",
            "--recovery",
        ])
        .arg(&path)
        .arg("--no-recovery")
        .arg(&input)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
}

#[cfg(unix)]
#[test]
fn retained_jsonl_answers_preserve_broken_pipe_exit() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("answers.recovery");
    let mut child = support::command()
        .args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--from",
            "jsonl",
            "--into",
            "result",
            "--recovery",
        ])
        .arg(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"text\":\"alpha\"}\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(141), "{output:?}");
    assert!(path.is_file());
    let recovered = support::command()
        .arg("recover")
        .arg(&path)
        .output()
        .unwrap();
    assert_eq!(recovered.status.code(), Some(0), "{recovered:?}");
    assert_eq!(
        String::from_utf8(recovered.stdout).unwrap().lines().count(),
        1
    );
}

#[cfg(unix)]
#[test]
fn retained_stdin_table_survives_broken_pipe_after_rendering() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("table.recovery");
    let mut child = support::command()
        .args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--from",
            "csv",
            "--column",
            "Body",
            "--into",
            "answer",
            "--recovery",
        ])
        .arg(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"Body\nalpha\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(141), "{output:?}");
    assert!(path.is_file());
    let recovered = support::command()
        .arg("recover")
        .arg(&path)
        .output()
        .unwrap();
    assert_eq!(recovered.status.code(), Some(0), "{recovered:?}");
    let rows: Vec<Value> = String::from_utf8(recovered.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["position"], 1);
    assert_eq!(rows[0]["status"], "answered");
    let summary: Value = serde_json::from_slice(&recovered.stderr).unwrap();
    assert_eq!(summary["complete"], false);
    let completed = summary["completed_output"].as_str().unwrap();
    assert!(std::path::Path::new(completed).is_file());
    assert!(fs::read_to_string(completed).unwrap().contains("answer"));
}

#[test]
fn rank_top_recovery_distinguishes_unemitted_inputs_from_unfinished_work() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("rank.recovery");
    let mut child = support::command()
        .args([
            "rank",
            "Priority?",
            "--backend",
            "stub",
            "--json",
            "--from",
            "lines",
            "--level",
            "low",
            "--level",
            "high",
            "--top",
            "1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"alpha\nbeta\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{output:?}");
    let expected: Value = serde_json::from_slice(&output.stdout).unwrap();
    let mut bytes = b"THINKTHEN-RECOVERY-1\n".to_vec();
    frame(
        &mut bytes,
        json!({"kind":"manifest","metadata":{"request":{"question":{"function":"rank","top":1}}},"admitted":2}),
    );
    for position in 1..=2 {
        frame(
            &mut bytes,
            json!({"kind":"occurrence","position":position,"source":{"row":position+1}}),
        );
    }
    fs::write(&path, &bytes).unwrap();
    let unfinished = support::command()
        .arg("recover")
        .arg(&path)
        .output()
        .unwrap();
    assert_eq!(unfinished.status.code(), Some(0), "{unfinished:?}");
    assert!(unfinished.stdout.is_empty());
    let summary: Value = serde_json::from_slice(&unfinished.stderr).unwrap();
    assert_eq!(summary["operation_status"], "incomplete");
    assert_eq!(summary["committed"], 0);
    assert_eq!(summary["missing_ranges"], json!([[1, 2]]));
    frame(
        &mut bytes,
        json!({"kind":"row","position":expected["position"],"row":expected}),
    );
    let rows_end = bytes.len();
    frame(
        &mut bytes,
        json!({"kind":"closing","complete":true,"emitted":1,"output":null}),
    );
    fs::write(&path, &bytes).unwrap();
    let complete = support::command()
        .arg("recover")
        .arg(&path)
        .output()
        .unwrap();
    assert_eq!(complete.status.code(), Some(0), "{complete:?}");
    assert_eq!(
        serde_json::from_slice::<Value>(&complete.stdout).unwrap(),
        expected
    );
    let summary: Value = serde_json::from_slice(&complete.stderr).unwrap();
    assert_eq!(summary["operation_status"], "complete");
    assert_eq!(summary["admitted"], 2);
    assert_eq!(summary["committed"], 1);
    assert_eq!(summary["missing_ranges"], json!([]));
    let omitted = if expected["position"] == 1 { 2 } else { 1 };
    assert_eq!(summary["unemitted_ranges"], json!([[omitted, omitted]]));
    bytes.truncate(rows_end);
    frame(
        &mut bytes,
        json!({"kind":"closing","complete":true,"emitted":2,"output":null}),
    );
    fs::write(&path, &bytes).unwrap();
    let corrupt = support::command()
        .arg("recover")
        .arg(&path)
        .output()
        .unwrap();
    assert_eq!(corrupt.status.code(), Some(5), "{corrupt:?}");
    assert!(String::from_utf8_lossy(&corrupt.stderr).contains("closing count"));
}
