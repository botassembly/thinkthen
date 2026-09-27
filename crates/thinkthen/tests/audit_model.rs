//! `thinkthen audit --write` records the model a bar was tuned on beside the
//! bar, in a single question file only (ticket 0159). A question set holds no
//! model: `audit_write.rs`, `inserts_when_absent`, pins a set's whole bytes
//! after a write.
#![cfg(feature = "cli")]
#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

#[path = "support/measure.rs"]
mod measure_support;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

use std::fs;

use measure_support::{audit, fixture, fixtures, payment_rows};

const WROTE: &str = "thinkthen: audit: wrote threshold 0.59 for the question; it was 0.9\n";
const KEPT: &str = "thinkthen: audit: kept the model for the question; ";

/// LINES, then a failed line and an unlabeled line, each naming the pinned version.
fn failed_and_unlabeled(lines: &str) -> String {
    let first = lines.lines().next().expect("a line");
    let mut failed: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(first).expect("a row");
    failed.insert("input".into(), serde_json::json!({"id": "C-98"}));
    failed.insert("value".into(), serde_json::Value::Null);
    failed.insert("failure".into(), serde_json::json!({"kind": "backend"}));
    assert!(failed.remove("answer").is_some());
    let failed = serde_json::Value::Object(failed);
    let unlabeled = first.replacen("\"C-01\"", "\"C-99\"", 1);
    format!("{lines}{failed}\n{unlabeled}\n")
}

#[test]
fn audit_writes_the_model_it_tuned_on() {
    let folder = std::env::temp_dir().join(format!("thinkthen-0159-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&folder);
    fs::create_dir(&folder).expect("a folder");
    let base = fixture("write/decide.json");
    let tuned = base.replacen("0.9", "0.59", 1);
    let pinned = tuned.replacen("]\r\n}", "],\r\n  \"model\": \"jev-1.13.0\"\r\n}", 1);
    let alias = base.replacen("  \"on\"", "  \"model\": \"jev-latest\",\r\n  \"on\"", 1);
    let band = base.replacen("0.9", "\"0.4:0.8\"", 1);
    fs::write(folder.join("base.json"), &base).expect("a file");
    fs::write(folder.join("band.json"), &band).expect("a file");
    let lines = payment_rows(&folder, "base.json");
    let named = "\"model\":\"jev-1.13.0\"";
    let two = lines.replacen(named, "\"model\":\"jev-1.14.0\"", 1);
    let none = lines.replacen(&format!(",{named}"), "", 1);
    let blank = lines.replace(named, "\"model\":\" \"");
    let more = format!("{WROTE}{KEPT}the results name more than one model\n");
    let unnamed = format!("{WROTE}{KEPT}a result names no model\n");
    let blanked = format!("{WROTE}{KEPT}a result names a blank model\n");
    let kept = "thinkthen: audit: kept the band for the question; audit suggests a single cut\n";
    let replaced = alias
        .replacen("0.9", "0.59", 1)
        .replace("jev-latest", "jev-1.13.0");
    let every = failed_and_unlabeled(&lines);
    let banded = payment_rows(&folder, "band.json");
    // Each row: the file, the result lines, the file afterward, and standard error.
    let cases = [
        ("one version", &base, &lines, &pinned, WROTE),
        ("two versions", &base, &two, &tuned, &*more),
        ("an alias typed", &alias, &lines, &replaced, WROTE),
        ("a line with no model", &base, &none, &tuned, &*unnamed),
        ("a blank model", &base, &blank, &tuned, &*blanked),
        ("failed and unlabeled", &base, &every, &pinned, WROTE),
        ("no bar", &band, &banded, &band, kept),
    ];
    let key = fixtures().join("write/key.jsonl");
    let [results, question] = [folder.join("rows.jsonl"), folder.join("question.json")];
    let paths = [&results, &key, &question].map(|path| path.to_str().expect("a path"));
    for (name, file, lines, after, stderr) in cases {
        fs::write(&question, file).expect("a file");
        fs::write(&results, lines).expect("rows");
        let (code, _, printed) = audit(&[paths[0], paths[1], "--write", paths[2]], b"");
        assert_eq!((code, printed.as_str()), (0, stderr), "{name}");
        assert_eq!(
            &fs::read_to_string(&question).expect("the file"),
            after,
            "{name}"
        );
    }
    let _removed = fs::remove_dir_all(&folder);
}
