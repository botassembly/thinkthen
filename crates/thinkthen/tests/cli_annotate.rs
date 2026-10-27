#![cfg(feature = "command")]
#![cfg(test)]
//! Whole toolboxes through the command.
mod support;

use loopback::{KEY, Loopback};
use std::io::Write;
use std::process::Stdio;

#[test]
fn single_record_annotations_use_selected_adapter_and_preserve_entry_failure() {
    let definition = r#"{"default":{"function":"decide","question":"Yes?"}}"#;
    let file = toolbox(definition);
    let definition = file.path();
    let rejected = support::command()
        .arg("annotate")
        .arg(definition)
        .args(["--from", "text"])
        .output()
        .unwrap();
    assert_eq!(rejected.status.code(), Some(2));
    assert_eq!(support::reported(&rejected), "usage.key-missing");
    for (probability, expected_code, expected_status) in [(0.9, 0, "answered"), (2.0, 6, "failed")]
    {
        let server = Loopback::start(move |_| {
            loopback::Reply::json(
                200,
                &serde_json::json!({"answers":[{"type":"predicate","probability":probability}]}),
            )
        })
        .unwrap();
        let output = support::command()
            .arg("annotate")
            .arg(definition)
            .args(["--from", "text"])
            .args(["--backend", "openai", "--url", server.base()])
            .env("OPENAI_API_KEY", KEY)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(expected_code));
        let row: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(row["answer"]["default"]["status"], expected_status);
        assert_eq!(server.count(), 1);
        assert_eq!(server.requests()[0].path, "/v1/decisions");
    }
}

#[test]
fn toolbox_command_prints_one_named_row_per_input_line() {
    let definition = r#"{"a":{"function":"decide","question":"A?"},"b":{"function":"decide","question":"B?"},"c":{"function":"decide","question":"C?"}}"#;
    let file = toolbox(definition);
    let definition = file.path();
    let server = Loopback::answering(|q| {
        serde_json::json!({"type":"noul","noul":if q["instructions"].as_str().unwrap().contains("record 0") {0.9} else {0.1}})
    }).unwrap();
    let mut child = support::command()
        .arg("annotate")
        .arg(definition)
        .args(["--from", "lines", "--json"])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(
            (0..2)
                .map(|i| format!("record {i}\n"))
                .collect::<String>()
                .as_bytes(),
        )
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert_eq!(text.lines().count(), 2);
    for (i, line) in text.lines().enumerate() {
        let row: serde_json::Value = serde_json::from_str(line).unwrap();
        assert_eq!(row["position"], i + 1);
        for name in ["a", "b", "c"] {
            assert_eq!(row["answer"][name]["answer"], i == 0);
        }
    }
    let questions: usize = server
        .requests()
        .iter()
        .map(|request| {
            request.json().unwrap()["questions"]
                .as_object()
                .unwrap()
                .len()
        })
        .sum();
    assert_eq!(questions, 6);
}

#[test]
fn flags_override_toolbox_settings_and_entry_failure_sets_exit_code() {
    let definition =
        r#"{"a":{"function":"decide","question":"A?"},"b":{"function":"decide","question":"B?"}}"#;
    let file = toolbox(definition);
    let definition = file.path();
    let server = Loopback::answering(|q| {
        if q["instructions"].as_str().unwrap().contains("B?") {
            serde_json::json!({"type":"noul","noul":2})
        } else {
            serde_json::json!({"type":"noul","noul":0.9})
        }
    })
    .unwrap();
    let mut child = support::command()
        .arg("annotate")
        .arg(definition)
        .args([
            "--from",
            "jsonl",
            "--json",
            "--model",
            "flag",
            "--evidence",
            "/selected",
            "--batch",
            "3",
        ])
        .env("THINKTHEN_BATCH", "1")
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"selected\":\"chosen\"}\n{\"selected\":\"chosen again\"}\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(6));
    assert_eq!(server.count(), 1);
    for request in server.requests() {
        let body = request.json().unwrap();
        assert_eq!(body["model"], "flag");
        assert!(
            body["questions"]
                .as_object()
                .unwrap()
                .values()
                .all(|q| q["instructions"].as_str().unwrap().contains("chosen"))
        );
    }
    let text = String::from_utf8(output.stdout).unwrap();
    assert_eq!(text.lines().count(), 2);
    for line in text.lines() {
        let row: serde_json::Value = serde_json::from_str(line).unwrap();
        assert_eq!(row["answer"]["a"]["answer"], true);
        assert_eq!(row["answer"]["b"]["status"], "failed");
    }
}

#[test]
fn annotation_admission_preserves_shared_error_codes_before_backend_requests() {
    let server = Loopback::answering(|_| serde_json::json!({"type":"noul","noul":0.9})).unwrap();
    for (definition, expected) in [
        (
            r#"{"a":{"function":"decide","question":"A?","threshold":2}}"#,
            "usage.threshold",
        ),
        (
            r#"{"a":{"function":"decide","question":"A?","evidence":"invalid"}}"#,
            "usage.pointer",
        ),
        (
            r#"{"bad\u0000name":{"function":"decide","question":"A?"}}"#,
            "usage.name",
        ),
    ] {
        let file = toolbox(definition);
        let output = support::command()
            .arg("annotate")
            .arg(file.path())
            .args(["--from", "text", "--url", server.base()])
            .env("THINKTHEN_API_KEY", KEY)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(support::reported(&output), expected);
        assert!(output.stdout.is_empty());
        assert_eq!(server.count(), 0);
    }
    let file = toolbox(r#"{"a":{"function":"decide","question":"A?","threshold":0.5}}"#);
    let output = support::command()
        .arg("annotate")
        .arg(file.path())
        .args(["--from", "text", "--url", server.base()])
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["answer"]["a"]["answer"], true);
    assert_eq!(server.count(), 1);
}

fn toolbox(definition: &str) -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    let questions: serde_json::Value = serde_json::from_str(definition).unwrap();
    write!(file, "{}", serde_json::json!({"questions":questions})).unwrap();
    file
}
