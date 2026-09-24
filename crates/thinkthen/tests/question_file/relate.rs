//! Relate question-file grammar, precedence, and exit classes.

use serde_json::Value;

use crate::harness::{CLOSED, run, written};

#[test]
fn file_backed_dry_run_reports_every_independent_precedence_source() {
    let file = written(
        "relate-precedence",
        r#"{"version":1,"relate":{"fields":{"name":"/title","kind":"/type"},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}]},"threshold":0.6,"model":"file-model","profile":"measured"}"#,
    );
    let output = run(
        &[
            "relate",
            &file,
            "--field",
            "/name",
            "--threshold",
            "0.7",
            "--model",
            "cli-model",
            "--dry-run",
            "--url",
            CLOSED,
            "--no-cache",
        ],
        br#"[{"name":"Ada","type":"person"},{"name":"Acme","type":"organization"}]"#,
    )
    .expect("binary runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let plan: Value = serde_json::from_slice(&output.stdout).expect("plan");
    assert_eq!(plan["model"], "cli-model");
    assert_eq!(plan["backend_profile"], Value::Null);
    assert_eq!(
        plan["fields"],
        serde_json::json!({"name":"/name","kind":"/type"})
    );
    assert_eq!(
        plan["from"],
        serde_json::json!({
            "question":"file","threshold":"command line","model":"command line",
            "field":"command line","kind_field":"file","profile":"file"
        })
    );
}

#[test]
fn invalid_unreadable_and_wrong_verb_files_keep_their_ruled_exit_codes() {
    let invalid = written(
        "relate-invalid",
        r#"{"version":1,"relate":{"relations":[{"name":"x","source":"a","target":"b"}]},"extra":true}"#,
    );
    let wrong = written("relate-wrong", r#"{"decide":"Is this valid?"}"#);
    for (file, code) in [
        (invalid, 5),
        (wrong, 2),
        ("@/path/that/does/not/exist.json".to_owned(), 5),
    ] {
        let output = run(&["relate", &file, "--dry-run"], b"[]").expect("binary runs");
        assert_eq!(output.status.code(), Some(code), "{file}");
        assert!(output.stdout.is_empty());
    }
}
