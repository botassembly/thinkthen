#![cfg(test)]
#![cfg(feature = "command")]
//! Backend rejection stops a command without consuming every batch.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;

#[test]
fn rejected_key_stops_a_thousand_record_call() {
    let server = loopback::Loopback::start(|_| {
        loopback::Reply::json(401, &serde_json::json!({"error":loopback::KEY}))
    })
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("records.txt");
    std::fs::write(&input, "evidence\n".repeat(1000)).unwrap();
    let output = support::command()
        .args([
            "decide", "Ready?", "--json", "--from", "lines", "--batch", "10",
        ])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", loopback::KEY)
        .stdin(std::fs::File::open(input).unwrap())
        .output()
        .unwrap();
    assert!(server.count() <= 8, "sent {} requests", server.count());
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    assert_eq!(support::reported(&output), "backend.status");
    assert!(output.stdout.is_empty());
    assert!(
        !String::from_utf8(output.stderr)
            .unwrap()
            .contains(loopback::KEY)
    );
}

#[cfg(unix)]
#[test]
fn terminal_mixed_run_retains_transport_failed_rows_and_their_original_item() {
    use std::io::Write;
    use std::process::Stdio;
    let unavailable = loopback::UnavailableBackend::new().unwrap();
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("run.json");
    std::fs::write(
        &path,
        serde_json::json!({"run":[{"shell":["cat"]},
        {"question":{"function":"decide","question":"Ready?"}}]})
        .to_string(),
    )
    .unwrap();
    let mut child = support::command()
        .arg("run")
        .arg(path)
        .args(["--allow-shell", "--json"])
        .env("THINKTHEN_BASE_URL", unavailable.base())
        .env("THINKTHEN_API_KEY", loopback::KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"\"evidence\"\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["status"], "failed");
    assert_eq!(row["error"]["code"], "backend.transport");
    assert_eq!(row["item"], "evidence");
    assert!(String::from_utf8_lossy(&output.stderr).contains("backend.transport"));
}

#[cfg(unix)]
#[test]
fn mixed_terminal_failures_keep_invalid_original_records() {
    use std::io::Write;
    use std::process::Stdio;
    let server = loopback::Loopback::noul(0.95).unwrap();
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("run.json");
    std::fs::write(
        &path,
        serde_json::json!({"run":[{"shell":["cat"]},
        {"question":{"function":"decide","question":"Ready?"}}]})
        .to_string(),
    )
    .unwrap();
    let mut child = support::command()
        .arg("run")
        .arg(path)
        .args(["--allow-shell", "--json", "--from", "jsonl"])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", loopback::KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"id\":null,\"text\":\"evidence\"}\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    let rows: Vec<serde_json::Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["status"], "failed");
    assert_eq!(rows[0]["error"]["code"], "record.invalid");
    assert_eq!(
        rows[0]["item"],
        serde_json::json!({"id":null,"text":"evidence"})
    );
    assert_eq!(server.count(), 0);
}
