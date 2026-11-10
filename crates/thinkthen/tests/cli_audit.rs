#![cfg(test)]
//! Offline audit exercises the actual command and its separate report stream.
#![cfg(feature = "command")]
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;
use serde_json::{Value, json};
use thinkthen::analysis::Manifest;
use thinkthen::{BackendName, Decide, OffsetUnit, RecordId, Settings};
fn inputs(
    directory: &std::path::Path,
) -> (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf) {
    let question = Decide::new("Is it ready?").unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let row = engine.run(&question, "ready").unwrap();
    let question = question.into();
    let manifest = Manifest {
        version: 1,
        function: row.function(),
        question,
        name: None,
        question_identity: row.source().question_identity().into(),
        backend: BackendName::Stub,
        model: row.source().model().into(),
        input_representation: "text/v1".into(),
        offset_unit: OffsetUnit::Utf8,
        expected_cases: vec![RecordId::Integer(1), RecordId::Integer(2)],
    };
    let rows = directory.join("rows.jsonl");
    let meta = directory.join("run.json");
    let key = directory.join("gold.jsonl");
    let mut value = serde_json::to_value(&row).unwrap();
    value["id"] = json!(1);
    std::fs::write(&rows, format!("{value}\n")).unwrap();
    std::fs::write(&meta, serde_json::to_vec(&manifest).unwrap()).unwrap();
    let gold = json!({"case_id":1,"question_identity":manifest.question_identity,"gold":!row.answer().unwrap()});
    let absent = json!({"case_id":2,"question_identity":manifest.question_identity,"gold":true});
    std::fs::write(&key, format!("{gold}\n{absent}\n")).unwrap();
    (rows, meta, key)
}
#[test]
fn audit_grades_without_credentials_or_network_and_keeps_missing_results() {
    let directory = tempfile::tempdir().unwrap();
    let (rows, manifest, key) = inputs(directory.path());
    let trap = loopback::Loopback::noul(0.9).unwrap();
    let output = support::command()
        .arg("audit")
        .arg(&rows)
        .arg("--manifest")
        .arg(&manifest)
        .arg("--key")
        .arg(&key)
        .args(["--bins", "2"])
        .env("THINKTHEN_URL", trap.base())
        .env("OPENAI_BASE_URL", trap.base())
        .env("THINKTHEN_MAX_REQUESTS", "invalid-inference-setting")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["counts"]["correct"], 0);
    assert_eq!(report["counts"]["wrong"], 1);
    assert_eq!(report["counts"]["labeled_cases"], 2);
    assert_eq!(report["counts"]["missing_results"], 1);
    assert_eq!(report["coverage"], 0.5);
    assert_eq!(report["bins"].as_array().unwrap().len(), 2);
    assert_eq!(trap.count(), 0);
    for option in [
        vec!["--backend", "stub"],
        vec!["--bins", "0"],
        vec!["--max-requests", "0"],
        vec!["--usage=false"],
    ] {
        let output = support::command()
            .arg("audit")
            .arg(&rows)
            .arg("--manifest")
            .arg(&manifest)
            .arg("--key")
            .arg(&key)
            .args(option)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(output.stdout.is_empty());
    }
    let output = support::command()
        .arg("audit")
        .arg(&rows)
        .arg("--manifest")
        .arg(&manifest)
        .arg("--key")
        .arg(&key)
        .arg("--quiet")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn documented_manifest_key_and_rows_form_a_complete_offline_example() {
    let directory = tempfile::tempdir().unwrap();
    let guide = include_str!("../../../docs/surfaces/bash.md")
        .split("## Audit saved answers offline")
        .nth(1)
        .unwrap();
    let documents: Vec<_> = guide
        .split("```json\n")
        .skip(1)
        .map(|block| block.split("```").next().unwrap())
        .collect();
    assert_eq!(documents.len(), 3);
    let paths = [
        directory.path().join("run.json"),
        directory.path().join("rows.jsonl"),
        directory.path().join("gold.jsonl"),
    ];
    for (path, body) in paths.iter().zip(documents) {
        std::fs::write(path, body).unwrap();
    }
    let output = support::command()
        .arg("audit")
        .arg(&paths[1])
        .arg("--manifest")
        .arg(&paths[0])
        .arg("--key")
        .arg(&paths[2])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["counts"]["correct"], 1);
    assert_eq!(report["counts"]["missing_results"], 2);
    assert_eq!(report["coverage"], 1.0);
}
