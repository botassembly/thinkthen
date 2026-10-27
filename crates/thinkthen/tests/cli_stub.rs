#![cfg(test)]
//! The command constructs every function with the built-in backend.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use loopback::Loopback;
use serde_json::{Value, json};
use std::io::Write;
use std::process::Stdio;

#[test]
fn all_functions_match_the_shared_stub_rows_without_http() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../../../construction/stub.json")).unwrap();
    assert_eq!(cases.len(), 10);
    let server = Loopback::noul(0.99).unwrap();
    let directory = std::env::temp_dir().join(format!("thinkthen-stub-{}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    let path = directory.join("question.json");
    let mut actual = Vec::new();
    for case in &cases {
        let request = &case["request"];
        let function = request["question"]["function"].as_str().unwrap();
        let mut entry = request.as_object().unwrap().clone();
        entry.remove("records");
        std::fs::write(&path, serde_json::to_string(&entry).unwrap()).unwrap();
        let mut child = support::command()
            .arg("run")
            .arg(&path)
            .args(["--backend", "stub", "--url", server.base(), "--json"])
            .env("THINKTHEN_API_KEY", "unused-synthetic-key")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        for record in request["records"].as_array().unwrap() {
            writeln!(input, "{record}").unwrap();
        }
        drop(input);
        let output = child.wait_with_output().unwrap();
        let failed = case["expected"].as_array().unwrap().iter().any(|row| {
            row["status"] == "failed"
                || (row["function"] == "annotate"
                    && row["answer"]
                        .as_object()
                        .unwrap()
                        .values()
                        .any(|entry| entry["status"] == "failed"))
        });
        assert_eq!(
            output.status.code(),
            Some(if failed { 6 } else { 0 }),
            "{function}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let rows: Vec<Value> = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        actual.push(json!({"function":function,"rows":rows}));
    }
    std::fs::remove_dir_all(directory).unwrap();
    assert_eq!(server.count(), 0);
    let expected: Vec<Value> = cases
        .iter()
        .map(|case| {
            let function = &case["request"]["question"]["function"];
            let mut rows = case["expected"].clone();
            if function == "filter" {
                for (row, record) in rows
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .zip(case["request"]["records"].as_array().unwrap())
                {
                    row["item"] = record.clone();
                    row["kept"] = json!(row["status"] == "answered" && row["answer"] == true);
                }
            }
            json!({"function":function,"rows":rows})
        })
        .collect();
    assert_eq!(
        actual,
        expected,
        "observed rows: {}",
        serde_json::to_string_pretty(&actual).unwrap()
    );
}
