#![cfg(feature = "command")]
#![cfg(test)]
//! Ranking arguments, original records and detailed rows through the command.

mod support;

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::io::Write;
use std::process::Stdio;

#[test]
fn rank_prints_originals_or_positions_and_applies_levels_and_top() {
    for scale in [false, true] {
        for details in [false, true] {
            for top in [None, Some("1"), Some("2")] {
                let server = Loopback::answering(move |q| {
                    let text = q["instructions"].as_str().unwrap();
                    if text.contains("broken") {
                        return json!({});
                    }
                    let p = if text.contains("low") { 0.1 } else { 0.9 };
                    if scale {
                        json!({"type":"score", "probabilities":{"0":1.0-p,"1":p}})
                    } else {
                        json!({"type":"noul", "noul":p})
                    }
                })
                .unwrap();
                let mut command = support::command();
                command
                    .args(["rank", "Priority?", "--jsonl", "--on", "/body"])
                    .env("THINKTHEN_BASE_URL", server.base())
                    .env("THINKTHEN_API_KEY", KEY)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());
                if scale {
                    command.args(["--level", "low", "--level", "high"]);
                }
                if details {
                    command.arg("--details");
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
                assert_eq!(output.status.code(), Some(4));
                assert_eq!(support::reported(&output), "backend.reply");
                let stdout = String::from_utf8(output.stdout).unwrap();
                let order = if top == Some("1") {
                    vec![2]
                } else if top.is_some() {
                    vec![2, 3]
                } else {
                    vec![2, 3, 1, 0]
                };
                if details {
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
                } else {
                    assert_eq!(
                        stdout,
                        order
                            .into_iter()
                            .map(|index| records[index].as_str())
                            .collect::<String>()
                    );
                }
            }
        }
    }
}

#[test]
fn saved_rank_requests_inherit_question_evidence_and_options() {
    for (function, override_levels) in [("decide", false), ("score", false), ("score", true)] {
        let path = std::env::temp_dir().join(format!(
            "thinkthen-rank-{}-{function}.json",
            std::process::id()
        ));
        let question = if function == "score" {
            json!({"score":"Priority?","levels":["low","high"],"on":"/body"})
        } else {
            json!({"decide":"Priority?","threshold":"0.2:0.8","on":"/body"})
        };
        let options = if override_levels {
            json!({"levels":["lowest","highest"]})
        } else {
            json!({})
        };
        let entry = json!({"function":"rank","question":{"type":"toolbox","value":question},"options":options});
        std::fs::write(&path, entry.to_string()).unwrap();
        let server = Loopback::answering(move |q| {
            let p = if q["instructions"].as_str().unwrap().contains("low") {
                0.1
            } else {
                0.5
            };
            if function == "score" {
                assert_eq!(
                    q["criteria"],
                    if override_levels {
                        json!(["lowest", "highest"])
                    } else {
                        json!(["low", "high"])
                    }
                );
                json!({"type":"score","probabilities":{"0":1.0-p,"1":p}})
            } else {
                json!({"type":"noul","noul":p})
            }
        })
        .unwrap();
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
            &["--lines"][..],
            &b"one\r\n\ntwo"[..],
            &b"one\r\ntwo"[..],
            0,
        ),
    ] {
        let server = Loopback::noul(0.9).unwrap();
        let mut child = support::command()
            .args(["rank", "Priority?"])
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

#[test]
fn rank_reports_closed_output_for_originals_and_details() {
    for details in [false, true] {
        let server = Loopback::noul(0.9).unwrap();
        let (stdout, _inherited_reader) = support::closed_output();
        let mut command = support::command();
        command
            .args(["rank", "Priority?", "--lines"])
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(stdout)
            .stderr(Stdio::piped());
        if details {
            command.arg("--details");
        }
        let mut child = command.spawn().unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"one\ntwo\n")
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(5));
        assert_eq!(support::reported(&output), "local.io");
    }
}

#[test]
fn rank_refuses_zero_top_before_sending() {
    let server = Loopback::noul(0.9).unwrap();
    let output = support::command()
        .args(["rank", "Priority?", "--top", "0"])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let error = thinkthen::RankLimit::new(0).unwrap_err();
    assert_eq!(support::reported(&output), error.kind().code());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        format!("thinkthen: {}: {error}\n", error.kind().code())
    );
    assert_eq!(server.count(), 0);
}

#[test]
fn rank_prints_nothing_on_input_failure_with_top_or_details() {
    for scale in [false, true] {
        for details in [false, true] {
            for top in [None, Some("1")] {
                let server = Loopback::answering(move |_| {
                    if scale {
                        json!({"type":"score","probabilities":{"0":0.1,"1":0.9}})
                    } else {
                        json!({"type":"noul","noul":0.9})
                    }
                })
                .unwrap();
                let mut command = support::command();
                command
                    .args(["rank", "Priority?", "--lines", "--batch", "1"])
                    .env("THINKTHEN_BASE_URL", server.base())
                    .env("THINKTHEN_API_KEY", KEY)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());
                if scale {
                    command.args(["--level", "low", "--level", "high"]);
                }
                if details {
                    command.arg("--details");
                }
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
                assert_eq!(output.status.code(), Some(5));
                assert!(output.stdout.is_empty());
                assert_eq!(support::reported(&output), "local.io");
            }
        }
    }
}
