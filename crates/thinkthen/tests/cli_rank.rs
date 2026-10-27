#![cfg(test)]
//! Ranking arguments, original records and detailed rows through the command.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::io::Write;
use std::process::Stdio;

#[test]
fn rank_prints_originals_or_positions_and_applies_levels_and_top() {
    for details in [false, true] {
        for top in [None, Some("1"), Some("2")] {
            let server = Loopback::answering(move |q| {
                let text = q["instructions"].as_str().unwrap();
                if text.contains("broken") {
                    return json!({});
                }
                let p = if text.contains("low") { 0.1 } else { 0.9 };
                json!({"type":"score", "probabilities":{"0":1.0-p,"1":p}})
            })
            .unwrap();
            let mut command = support::command();
            command
                .args([
                    "rank",
                    "Priority?",
                    "--from",
                    "jsonl",
                    "--evidence",
                    "/body",
                ])
                .env("THINKTHEN_BASE_URL", server.base())
                .env("THINKTHEN_API_KEY", KEY)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            command.args(["--level", "low", "--level", "high"]);
            if details {
                command.arg("--json");
            }
            if let Some(top) = top {
                command.args(["--top", top]);
            }
            let mut child = command.spawn().unwrap();
            let records = ["broken", "low", "high-a", "high-b"]
                .map(|body| format!("{{\"id\":\"{body}\", \"body\":\"{body}\"}}\r\n"));
            child
                .stdin
                .take()
                .unwrap()
                .write_all(records.concat().as_bytes())
                .unwrap();
            let output = child.wait_with_output().unwrap();
            assert_eq!(output.status.code(), Some(6));
            let stdout = String::from_utf8(output.stdout).unwrap();
            let order = if top == Some("1") {
                vec![2, 0]
            } else if top.is_some() {
                vec![2, 3, 0]
            } else {
                vec![2, 3, 1, 0]
            };
            if !details {
                let expected = order
                    .iter()
                    .filter(|index| **index != 0)
                    .map(|index| records[*index].as_str())
                    .collect::<String>();
                assert_eq!(stdout, expected);
            } else {
                let rows: Vec<Value> = stdout
                    .lines()
                    .map(|line| serde_json::from_str(line).unwrap())
                    .collect();
                assert_eq!(rows.len(), order.len());
                for (position, index) in order.iter().enumerate() {
                    let original: Value = serde_json::from_str(&records[*index]).unwrap();
                    assert_eq!(rows[position]["id"], original["id"]);
                    if *index == 0 {
                        assert_eq!(rows[position]["error"]["code"], "backend.reply");
                    } else {
                        assert_eq!(rows[position]["answer"], position + 1);
                    }
                }
            }
        }
    }
}

#[test]
fn saved_rank_requests_retain_their_levels_and_evidence() {
    for override_levels in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("rank.json");
        let levels = if override_levels {
            ["lowest", "highest"]
        } else {
            ["low", "high"]
        };
        let entry = json!({"question":{"function":"rank","question":"Priority?","levels":levels,"evidence":"/body"}});
        std::fs::write(&path, entry.to_string()).unwrap();
        let server = Loopback::answering(move |q| {
            let p = if q["instructions"].as_str().unwrap().contains("low") {
                0.1
            } else {
                0.5
            };
            {
                assert_eq!(
                    q["criteria"],
                    if override_levels {
                        json!(["lowest", "highest"])
                    } else {
                        json!(["low", "high"])
                    }
                );
                json!({"type":"score","probabilities":{"0":1.0-p,"1":p}})
            }
        })
        .unwrap();
        let mut child = support::command()
            .arg("run")
            .arg("--json")
            .arg(&path)
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
                b"{\"id\":\"low\",\"body\":\"low\"}\n{\"id\":\"middle\",\"body\":\"middle\"}\n",
            )
            .unwrap();
        let output = child.wait_with_output().unwrap();
        std::fs::remove_file(path).unwrap();
        assert_eq!(output.status.code(), Some(0));
        let stdout = String::from_utf8(output.stdout).unwrap();
        let rows: Vec<Value> = stdout
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["id"], "middle");
        assert_eq!(rows[0]["answer"], 1);
        assert_eq!(server.count(), 1);
    }
}

#[test]
fn rank_preserves_text_framing() {
    for (args, input, expected, code) in [
        (&[][..], &b"one\r\ntwo"[..], &b"one\r\ntwo"[..], 0),
        (
            &["--from", "lines"][..],
            &b"low\r\n\nhigh"[..],
            &b"highlow\r\n"[..],
            0,
        ),
    ] {
        let server = Loopback::answering(|q| {
            let p = if q["instructions"].as_str().unwrap().contains("low") {
                0.1
            } else {
                0.9
            };
            json!({"type":"score","probabilities":{"0":1.0-p,"1":p}})
        })
        .unwrap();
        let mut child = support::command()
            .args(["rank", "Priority?", "--level", "low", "--level", "high"])
            .args(args)
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input).unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(code));
        assert_eq!(output.stdout, expected);
    }
}

#[cfg(unix)]
#[test]
fn rank_reports_closed_output_for_originals_and_details() {
    for details in [false, true] {
        let server =
            Loopback::answering(|_| json!({"type":"score","probabilities":{"0":0.1,"1":0.9}}))
                .unwrap();
        let (stdout, _inherited_reader) = support::closed_output();
        let mut command = support::command();
        command
            .args([
                "rank",
                "Priority?",
                "--level",
                "low",
                "--level",
                "high",
                "--from",
                "lines",
            ])
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(stdout)
            .stderr(Stdio::piped());
        if details {
            command.arg("--json");
        }
        let mut child = command.spawn().unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"one\ntwo\n")
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(141));
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn rank_refuses_zero_top_before_sending() {
    let server =
        Loopback::answering(|_| json!({"type":"score","probabilities":{"0":0.1,"1":0.9}})).unwrap();
    let output = support::command()
        .args([
            "rank",
            "Priority?",
            "--level",
            "low",
            "--level",
            "high",
            "--top",
            "0",
        ])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let error = thinkthen::Question::from_json(
        r#"{"function":"rank","question":"Priority?","levels":["low","high"],"top":0}"#,
    )
    .unwrap_err();
    assert_eq!(support::reported(&output), error.code().as_str());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        format!("thinkthen: {}: {error}\n", error.code().as_str())
    );
    assert_eq!(server.count(), 0);
}

#[test]
fn rank_keeps_valid_rows_and_reports_invalid_text_as_a_failed_record() {
    for top in [None, Some("1")] {
        let server =
            Loopback::answering(|_| json!({"type":"score","probabilities":{"0":0.1,"1":0.9}}))
                .unwrap();
        let mut command = support::command();
        command
            .args([
                "rank",
                "Priority?",
                "--level",
                "low",
                "--level",
                "high",
                "--from",
                "lines",
                "--json",
                "--batch",
                "1",
            ])
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(top) = top {
            command.args(["--top", top]);
        }
        let mut child = command.spawn().unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"one\r\ntwo\xff\n")
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(6));
        let rows: Vec<Value> = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["position"], 1);
        assert_eq!(rows[0]["answer"], 1);
        assert_eq!(rows[1]["position"], 2);
        assert_eq!(rows[1]["error"]["code"], "record.invalid");
        assert_eq!(server.count(), 1);
    }
}
