#![cfg(test)]
#![cfg(feature = "command")]
//! Answer-field diagnostics identify failed inputs and count missing answers without discarding unsure rows.
mod support;
use serde_json::{Value, json};
use std::process::Output;

#[test]
fn failed_answer_fields_name_exact_ids_and_source_files() {
    let dir = tempfile::tempdir().unwrap();
    let first = dir.path().join("first.jsonl");
    let second = dir.path().join("second.jsonl");
    std::fs::write(
        &first,
        "{\"id\":\"good\",\"body\":\"Refund please\"}\n{\"id\":18446744073709551615,\"other\":1}\n",
    )
    .unwrap();
    std::fs::write(&second, "{\"id\":\"customer-7\",\"other\":2}\n").unwrap();
    let invoke = |into: bool| {
        let mut command = support::command();
        command
            .args(["decide", "Refund?", "--backend", "stub", "--field", "/body"])
            .arg(&first)
            .arg(&second);
        if into {
            command.args(["--into", "result"]);
        } else {
            command.arg("--json");
        }
        command.output().unwrap()
    };
    let ordinary = invoke(false);
    let appended = invoke(true);
    assert_eq!(ordinary.status.code(), Some(6), "{ordinary:?}");
    assert_eq!(
        appended.status.code(),
        ordinary.status.code(),
        "{appended:?}"
    );
    let diagnostic = String::from_utf8(appended.stderr.clone()).unwrap();
    for expected in [
        format!(
            "position 2 id 18446744073709551615 file {}",
            first.display()
        ),
        format!("position 3 id customer-7 file {}", second.display()),
    ] {
        assert!(String::from_utf8_lossy(&ordinary.stderr).contains(&expected));
        assert!(diagnostic.contains(&expected), "{diagnostic}");
    }
    let rows = rows(&appended);
    assert_eq!(rows.len(), 3);
    assert!(rows[0]["result"].is_boolean());
    assert_eq!(rows[1], json!({"id":u64::MAX,"other":1,"result":null}));
    assert_eq!(rows[2], json!({"id":"customer-7","other":2,"result":null}));
}

#[test]
fn unsure_answers_are_kept_and_csv_counts_respect_quiet() {
    let dir = tempfile::tempdir().unwrap();
    let jsonl = dir.path().join("input.jsonl");
    let csv = dir.path().join("input.csv");
    std::fs::write(
        &jsonl,
        "{\"body\":\"first\"}\n{\"body\":\"second\"}\n{\"body\":\"third\"}\n",
    )
    .unwrap();
    std::fs::write(&csv, "body\nfirst\nsecond\nthird\n").unwrap();
    for quiet in [false, true] {
        let invoke = |table: bool| {
            let mut command = support::command();
            command.args([
                "decide",
                "Refund?",
                "--backend",
                "stub",
                "--low",
                "0.000001",
                "--high",
                "0.999999",
                "--into",
                "result",
            ]);
            if table {
                command
                    .arg(&csv)
                    .args(["--column", "body", "--output"])
                    .arg(
                        dir.path()
                            .join(if quiet { "quiet.csv" } else { "normal.csv" }),
                    );
            } else {
                command.arg(&jsonl).args(["--field", "/body"]);
            }
            if quiet {
                command.arg("--quiet");
            }
            command.output().unwrap()
        };
        let table = invoke(true);
        let appended = invoke(false);
        assert_eq!(table.status.code(), Some(0), "{table:?}");
        assert_eq!(appended.status.code(), table.status.code(), "{appended:?}");
        assert!(appended.stderr.is_empty(), "{appended:?}");
        if quiet {
            assert!(appended.stdout.is_empty());
        } else {
            let answers = rows(&appended);
            assert_eq!(answers.len(), 3);
            assert!(answers.iter().all(|row| row["result"] == "unsure"));
        }
        if quiet {
            assert!(table.stderr.is_empty());
        } else {
            assert_eq!(
                String::from_utf8(table.stderr).unwrap(),
                "thinkthen: 3 rows not sure, 0 rows failed\n"
            );
        }
    }
}

#[test]
fn quiet_keeps_failure_diagnostics_but_suppresses_counts() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("input.jsonl");
    std::fs::write(
        &path,
        "{\"id\":\"rejected\",\"body\":\"first\"}\n{\"id\":\"failed\",\"other\":1}\n",
    )
    .unwrap();
    for quiet in [false, true] {
        let mut command = support::command();
        command
            .args([
                "filter",
                "Refund?",
                "--backend",
                "stub",
                "--threshold",
                "1",
                "--field",
                "/body",
                "--into",
                "result",
            ])
            .arg(&path);
        if quiet {
            command.arg("--quiet");
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(6), "{output:?}");
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("record.missing: position 2 id failed file"),
            "{stderr}"
        );
        assert_eq!(
            stderr.contains("0 rows not sure, 1 rows failed"),
            !quiet,
            "{stderr}"
        );
    }
}

fn rows(output: &Output) -> Vec<Value> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
