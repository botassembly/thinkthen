#![cfg(feature = "command")]
#![cfg(test)]
//! Saved requests preserve the shared contract's Unicode scalar offsets.
mod support;

use loopback::{KEY, Loopback};
use serde_json::Value;
use std::io::Write;
use std::process::Stdio;

#[test]
fn saved_requests_keep_code_point_offsets_in_answers_and_detail() {
    let directory = tempfile::tempdir().unwrap();
    let mut failures = Vec::new();
    for source in [
        include_str!("../../../conformance/find-unicode.json"),
        include_str!("../../../conformance/recognize-unicode.json"),
    ] {
        let case: Value = serde_json::from_str(source).unwrap();
        let server = Loopback::case(&case).unwrap();
        let mut request = case["request"].clone();
        let records = request.as_object_mut().unwrap().remove("records").unwrap();
        let path = directory.path().join("request.json");
        std::fs::write(&path, serde_json::to_vec(&request).unwrap()).unwrap();
        let mut child = support::command()
            .arg("run")
            .arg(&path)
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        for record in records.as_array().unwrap() {
            writeln!(input, "{record}").unwrap();
        }
        drop(input);
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let actual: Vec<Value> = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let actual = Value::Array(actual);
        if actual != case["expected"] {
            failures.push(format!(
                "{}: expected {}, got {actual}",
                request["function"], case["expected"]
            ));
        }
        assert_eq!(server.count(), case["requests"].as_u64().unwrap() as usize);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
