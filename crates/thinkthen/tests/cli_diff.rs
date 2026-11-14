#![cfg(test)]
//! Saved-run diff through the real command with no backend credentials.
#![cfg(feature = "command")]
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;
use serde_json::{Value, json};
fn inputs(directory: &std::path::Path) -> [std::path::PathBuf; 3] {
    let documents: Vec<Value> =
        serde_json::from_str(include_str!("../../../construction/audit-example.json")).unwrap();
    assert_eq!(documents.len(), 3);
    let paths = [
        directory.join("run.json"),
        directory.join("rows.jsonl"),
        directory.join("gold.jsonl"),
    ];
    for (path, body) in paths.iter().zip(documents) {
        std::fs::write(path, serde_json::to_vec(&body).unwrap()).unwrap();
    }
    paths
}
#[test]
fn diff_modes_preserve_cases_and_make_no_backend_calls() {
    let directory = tempfile::tempdir().unwrap();
    let paths = inputs(directory.path());
    let trap = loopback::Loopback::noul(0.9).unwrap();
    let output = support::command()
        .arg("diff")
        .arg(&paths[1])
        .arg("--manifest")
        .arg(&paths[0])
        .args(["--threshold", "0.1"])
        .arg("--key")
        .arg(&paths[2])
        .env("THINKTHEN_URL", trap.base())
        .env("OPENAI_BASE_URL", trap.base())
        .env("THINKTHEN_MAX_REQUESTS", "invalid-inference-setting")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["counts"]["lost_correct"], 1);
    assert_eq!(report["counts"]["changed_answer"], 1);
    assert_ne!(
        report["left"]["manifest"]["question_identity"],
        report["right"]["manifest"]["question_identity"]
    );
    for option in [vec!["--max-requests", "0"], vec!["--usage=false"]] {
        let output = support::command()
            .arg("diff")
            .arg(&paths[1])
            .arg("--manifest")
            .arg(&paths[0])
            .args(["--threshold", "0.1"])
            .args(option)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(output.stdout.is_empty());
    }
    let right = directory.path().join("right.jsonl");
    let mut row: Value =
        serde_json::from_str(&std::fs::read_to_string(&paths[1]).unwrap()).unwrap();
    row["answer"] = json!(true);
    row["probability"] = json!(0.8);
    std::fs::write(&right, row.to_string()).unwrap();
    let output = support::command()
        .arg("diff")
        .arg(&paths[1])
        .arg("--left-manifest")
        .arg(&paths[0])
        .arg("--right")
        .arg(&right)
        .arg("--right-manifest")
        .arg(&paths[0])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["counts"]["changed_answer"], 1);
    assert!(report["correctness_change"]["reason"].is_string());
    assert_eq!(trap.count(), 0);
    let output = support::command()
        .arg("diff")
        .arg(&paths[1])
        .arg("--manifest")
        .arg(&paths[0])
        .args(["--threshold", "0.5", "--low", "0.2", "--high", "0.8"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
}

#[test]
fn unsure_choose_find_threshold_replay_refuses_without_network() {
    let directory = tempfile::tempdir().unwrap();
    let paths = inputs(directory.path());
    let trap = loopback::Loopback::noul(0.9).unwrap();
    let engine = thinkthen::Settings::default()
        .with_backend(thinkthen::BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    for function in ["choose", "find"] {
        let (question, sample): (thinkthen::Question, thinkthen::AnyRow) = if function == "choose" {
            let question = thinkthen::Choose::with_strings("Which?", ["a", "b"]).unwrap();
            let row = engine.run(&question, "a").unwrap();
            (question.into(), thinkthen::AnyRow::Choose(row))
        } else {
            let question = thinkthen::Find::new("Which line?").unwrap();
            let row = engine.run(&question, "a").unwrap();
            (question.into(), thinkthen::AnyRow::Find(row))
        };
        let mut manifest: Value =
            serde_json::from_str(&std::fs::read_to_string(&paths[0]).unwrap()).unwrap();
        manifest["question"] = serde_json::to_value(question).unwrap();
        manifest["function"] = json!(function);
        manifest["question_identity"] = json!(sample.source().question_identity());
        manifest["backend"] = json!("stub");
        manifest["model"] = json!(sample.source().model());
        let mut row = serde_json::to_value(sample).unwrap();
        row["id"] = json!(1);
        row["status"] = json!("unsure");
        row["probability"] = json!(0.5);
        row.as_object_mut().unwrap().remove("answer");
        row.as_object_mut().unwrap().remove("detail");
        std::fs::write(&paths[0], manifest.to_string()).unwrap();
        std::fs::write(&paths[1], row.to_string()).unwrap();
        let output = support::command()
            .arg("diff")
            .arg(&paths[1])
            .arg("--manifest")
            .arg(&paths[0])
            .args(["--threshold", "0.8"])
            .env("THINKTHEN_URL", trap.base())
            .env("OPENAI_BASE_URL", trap.base())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("threshold-only diff supports decide/filter")
        );
    }
    assert_eq!(trap.count(), 0);
}
