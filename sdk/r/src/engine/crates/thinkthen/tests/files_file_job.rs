#![cfg(test)]
//! File commands publish complete results while preserving named record failures.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use std::fs;
mod support;

#[test]
fn completed_annotations_keep_good_and_failed_cells_with_typed_failure() {
    let directory = tempfile::TempDir::new().unwrap();
    let input = directory.path().join("requests.csv");
    fs::write(&input, "Body\nalpha\n").unwrap();
    let form = directory.path().join("form.json");
    fs::write(&form, r#"{"questions":{"bad":{"function":"decide","question":"Bad?"},"good":{"function":"decide","question":"Good?"}}}"#).unwrap();
    let server = Loopback::answering(|question| {
        if question["instructions"].as_str().unwrap().contains("Bad?") {
            serde_json::json!({"type":"noul","noul":2.0})
        } else {
            serde_json::json!({"type":"noul","noul":0.47})
        }
    })
    .unwrap();
    let result = support::command()
        .arg("annotate")
        .arg(&form)
        .arg(&input)
        .args(["--column", "Body", "--details"])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(6), "{result:?}");
    assert_eq!(support::reported(&result), "backend.reply");
    assert!(String::from_utf8_lossy(&result.stderr).contains("backend.reply: row 2:"));
    let output = fs::read_to_string(directory.path().join("requests.thinkthen.csv")).unwrap();
    assert!(output.starts_with("Body,bad,bad_status,bad_probability,bad_error,bad_detail,bad_source,good,good_status,good_probability,good_error,good_detail,good_source\n"));
    assert!(
        output.contains("alpha,,failed,,backend.reply: "),
        "{output}"
    );
    assert!(output.contains(",FALSE,answered,0.47,,"));
    assert_eq!(fs::read_to_string(input).unwrap(), "Body\nalpha\n");
}

#[test]
fn unmatched_rank_pointers_stop_before_work_and_valid_ranks_publish() {
    let directory = tempfile::TempDir::new().unwrap();
    let input = directory.path().join("requests.csv");
    fs::write(&input, "Body,Other\nalpha,beta\n").unwrap();
    let run = |extra: &[&str]| {
        support::command()
            .args([
                "rank",
                "Ready?",
                "--level",
                "low",
                "--level",
                "high",
                "--backend",
                "stub",
            ])
            .args(["--column", "Body", "--column", "Other", "--into", "rank"])
            .args(extra)
            .arg(&input)
            .output()
            .unwrap()
    };
    let rejected = run(&["--field", "/Missing"]);
    assert_eq!(rejected.status.code(), Some(2), "{rejected:?}");
    assert_eq!(support::reported(&rejected), "usage.column");
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("/Missing"));
    let output = directory.path().join("requests.thinkthen.csv");
    assert!(!output.exists());
    let ranked = run(&[]);
    assert_eq!(ranked.status.code(), Some(0), "{ranked:?}");
    assert_eq!(
        fs::read_to_string(output).unwrap(),
        "Body,Other,rank\nalpha,beta,1\n"
    );
    assert_eq!(
        fs::read_to_string(input).unwrap(),
        "Body,Other\nalpha,beta\n"
    );
}
