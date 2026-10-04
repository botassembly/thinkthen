//! Shared seven-command file intake, document run exits and text windows.

use crate::harness::{Canned, Listener, spawn};
use serde_json::{Value, json};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Output;

pub(super) fn folder(name: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("0401-{name}"));
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub(super) fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}
fn rows(output: &Output) -> Vec<Value> {
    text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("result JSON"))
        .collect()
}

pub(super) fn answer(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request JSON");
    let answers = request["questions"].as_object().expect("questions").iter().map(|(key, question)| {
        let value = match question["type"].as_str() {
            Some("choice") => json!({"type":"choice","choice":"a","confidence":0.9,"probabilities":{"a":0.9,"b":0.1}}),
            Some("score") => json!({"type":"score","score":0.8,"confidence":0.9,"legend":{"0":"a","1":"b"},"probabilities":{"0":0.2,"1":0.8}}),
            _ => json!({"type":"noul","noul":0.9}),
        };
        (key.clone(), value)
    }).collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
}

fn call(listener: &Listener, command: &[&str]) -> io::Result<Output> {
    spawn(
        &[
            command,
            &["--url", listener.base(), "--model", "local-1", "--no-cache"],
        ]
        .concat(),
        &[],
        b"ignored stdin",
    )
}

#[test]
fn seven_commands_read_files_in_order_and_restart_positions() -> io::Result<()> {
    let place = folder("seven")?;
    let one = place.join("one.txt");
    let two = place.join("two.txt");
    fs::write(&one, "one\n\nlast\n")?;
    fs::write(&two, "two\nend")?;
    let one = one.to_str().expect("path");
    let two = two.to_str().expect("path");
    let set = place.join("set.json");
    fs::write(
        &set,
        r#"{"version":1,"questions":{"ok":{"decide":"Is it clear?"}}}"#,
    )?;
    for verb in [
        "decide", "filter", "rank", "choose", "score", "tag", "annotate",
    ] {
        let listener = Listener::answering(answer)?;
        let question = if verb == "annotate" {
            set.to_str().expect("path")
        } else {
            "Is it clear?"
        };
        let labels: &[&str] = match verb {
            "choose" | "score" => &["a", "b"],
            "tag" => &["a"],
            _ => &[],
        };
        let output = call(
            &listener,
            &[
                &[verb, question][..],
                labels,
                &[
                    "--lines",
                    "--details",
                    "--batch",
                    "1",
                    "--input",
                    one,
                    "--input",
                    two,
                ],
            ]
            .concat(),
        )?;
        assert_eq!(
            output.status.code(),
            Some(0),
            "{verb}: {}",
            text(&output.stderr)
        );
        assert_eq!(listener.requests().len(), 4, "{verb}");
        let rows = rows(&output);
        assert_eq!(rows.len(), 4, "{verb}");
        for (row, file, line) in [
            (&rows[0], one, 1),
            (&rows[1], one, 3),
            (&rows[2], two, 1),
            (&rows[3], two, 2),
        ] {
            assert_eq!(
                row["position"],
                json!({"file":file,"first":line,"last":line}),
                "{verb}"
            );
        }
    }
    Ok(())
}

#[test]
fn documents_keep_single_exits_and_complete_multiple_answers_as_a_run() -> io::Result<()> {
    let place = folder("documents")?;
    let one = place.join("one.txt");
    let two = place.join("two.txt");
    fs::write(&one, "one\n")?;
    fs::write(&two, "two\n")?;
    let one = one.to_str().expect("path");
    let two = two.to_str().expect("path");
    for (probability, threshold, expected, exit) in [
        ("0.1", "0.5", json!(false), 1),
        ("0.5", "0.1:0.9", Value::Null, 3),
    ] {
        let reply = json!({"model":"local-1","answers":{"q1":{"type":"noul","noul":probability.parse::<f64>().expect("number")}}}).to_string();
        let single = Listener::serving(vec![Canned::ok(&reply)])?;
        let output = call(
            &single,
            &["decide", "Clear?", "--threshold", threshold, "--input", one],
        )?;
        assert_eq!(output.status.code(), Some(exit));
        assert_eq!(rows(&output), std::slice::from_ref(&expected));
        let multi = Listener::serving(vec![Canned::ok(&reply), Canned::ok(&reply)])?;
        let output = call(
            &multi,
            &["decide", "Clear?", "--threshold", threshold, one, two],
        )?;
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        assert_eq!(
            rows(&output),
            [
                json!({"input_file":one,"value":expected}),
                json!({"input_file":two,"value":expected})
            ]
        );
        assert_eq!(multi.requests().len(), 2);
    }
    Ok(())
}

#[test]
fn later_document_failure_keeps_the_completed_prefix_and_actual_error() -> io::Result<()> {
    let place = folder("later-failure")?;
    let one = place.join("one");
    let two = place.join("two");
    fs::write(&one, "one")?;
    fs::write(&two, "")?;
    let one = one.to_str().expect("path");
    let two = two.to_str().expect("path");
    let listener = Listener::answering(answer)?;
    let output = call(&listener, &["decide", "Clear?", one, two])?;
    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(rows(&output), [json!({"input_file":one,"value":true})]);
    fs::write(two, "two")?;
    let listener = Listener::answering(|body| {
        if text(body).contains("two") {
            Canned::ok("invalid")
        } else {
            answer(body)
        }
    })?;
    let output = call(&listener, &["decide", "Clear?", one, two])?;
    assert_eq!(output.status.code(), Some(4), "{}", text(&output.stderr));
    assert_eq!(listener.requests().len(), 2);
    assert_eq!(rows(&output), [json!({"input_file":one,"value":true})]);
    Ok(())
}

#[test]
fn windows_preserve_blanks_final_lines_and_file_edges() -> io::Result<()> {
    let place = folder("windows")?;
    let one = place.join("one");
    let two = place.join("two");
    fs::write(&one, "a\n\nb\n\n\nlast")?;
    fs::write(&two, "next\n\n")?;
    let one = one.to_str().expect("path");
    let two = two.to_str().expect("path");
    let set = place.join("set.json");
    fs::write(
        &set,
        r#"{"version":1,"questions":{"ok":{"decide":"Clear?"}}}"#,
    )?;
    for verb in [
        "decide", "filter", "rank", "choose", "score", "tag", "annotate",
    ] {
        let listener = Listener::answering(answer)?;
        let labels: &[&str] = match verb {
            "choose" | "score" => &["a", "b"],
            "tag" => &["a"],
            _ => &[],
        };
        let output = call(
            &listener,
            &[
                &[
                    verb,
                    if verb == "annotate" {
                        set.to_str().expect("path")
                    } else {
                        "Clear?"
                    },
                ][..],
                labels,
                &[
                    "--window",
                    "2",
                    "--details",
                    "--batch",
                    "1",
                    "--input",
                    one,
                    "--input",
                    two,
                ],
            ]
            .concat(),
        )?;
        assert_eq!(
            output.status.code(),
            Some(0),
            "{verb}: {}",
            text(&output.stderr)
        );
        let rows = rows(&output);
        assert_eq!(
            rows.iter().map(|r| r["input"].clone()).collect::<Vec<_>>(),
            [json!("a\n"), json!("b\n"), json!("\nlast"), json!("next\n")]
        );
        assert_eq!(rows[2]["position"], json!({"file":one,"first":5,"last":6}));
        assert_eq!(rows[3]["position"], json!({"file":two,"first":1,"last":2}));
        assert_eq!(listener.requests().len(), 4);
    }
    Ok(())
}

#[test]
fn preflight_refusals_send_nothing() -> io::Result<()> {
    let place = folder("refusals")?;
    let one = place.join("one");
    fs::write(&one, "one")?;
    let question = place.join("question.json");
    fs::write(&question, r#"{"decide":"Clear?","on":"/text"}"#)?;
    let set = place.join("set.json");
    fs::write(
        &set,
        r#"{"version":1,"questions":{"ok":{"decide":"Clear?","on":"/text"}}}"#,
    )?;
    let one = one.to_str().expect("path");
    let missing = place.join("missing");
    let missing = missing.to_str().expect("path");
    let saved = format!("@{}", question.display());
    let cases: Vec<Vec<&str>> = vec![
        vec!["decide", "Clear?", "--input", one, "--input", missing],
        vec![
            "filter",
            "Clear?",
            "--input",
            one,
            "--input",
            place.to_str().expect("path"),
        ],
        vec!["rank", "Clear?", one, "--input", one],
        vec![
            "decide", "Clear?", "--input", one, "--input", one, "--quiet",
        ],
        vec![
            "choose", "Clear?", "a", "b", "--input", one, "--input", one, "--raw",
        ],
        vec!["decide", "Clear?", "--window", "0"],
        vec!["decide", "Clear?", "--window", "+2"],
        vec!["decide", "Clear?", "--window", "2", "--jsonl"],
        vec!["filter", "Clear?", "--window", "2", "--csv"],
        vec!["rank", "Clear?", "--window", "2", "--tsv"],
        vec!["decide", "Clear?", "--window", "2", "--field", "/text"],
        vec!["rank", &saved, "--window", "2"],
        vec!["annotate", set.to_str().expect("path"), "--window", "2"],
        vec!["recognize", "--input", one, "--input", one],
        vec!["relate", "member", "--window", "2"],
    ];
    let listener = Listener::answering(answer)?;
    for case in cases {
        let output = call(&listener, &case)?;
        assert!(
            output.status.code().is_some_and(|code| code >= 2),
            "{case:?}"
        );
        assert!(output.stdout.is_empty(), "{case:?}");
    }
    assert_eq!(listener.connections(), 0);
    Ok(())
}

#[test]
fn each_table_file_has_a_header_and_no_position() -> io::Result<()> {
    let place = folder("tables")?;
    for (mode, separator) in [("--csv", ','), ("--tsv", '\t')] {
        let one = place.join("one");
        let two = place.join("two");
        fs::write(&one, format!("id{separator}text\n1{separator}first\n"))?;
        fs::write(&two, format!("id{separator}text\n2{separator}second\n"))?;
        let listener = Listener::answering(answer)?;
        let output = call(
            &listener,
            &[
                "decide",
                "Clear?",
                mode,
                "--details",
                "--batch",
                "1",
                "--input",
                one.to_str().expect("path"),
                "--input",
                two.to_str().expect("path"),
            ],
        )?;
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        let rows = rows(&output);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1]["input"], json!({"id":"2","text":"second"}));
        assert!(rows.iter().all(|row| row.get("position").is_none()));
        assert_eq!(listener.requests().len(), 2);
    }
    Ok(())
}

#[test]
fn empty_and_blank_windows_advance_positions_without_sends() -> io::Result<()> {
    let place = folder("blank-windows")?;
    let empty = place.join("empty");
    let blank = place.join("blank");
    fs::write(&empty, "")?;
    fs::write(&blank, " \n\t\nlast\n")?;
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        &[
            "decide",
            "Clear?",
            "--window",
            "2",
            "--details",
            "--input",
            empty.to_str().expect("path"),
            "--input",
            blank.to_str().expect("path"),
        ],
    )?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(listener.requests().len(), 1);
    let rows = rows(&output);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["input"], "last");
    assert_eq!(
        rows[0]["position"],
        json!({"file":blank,"first":3,"last":3})
    );
    Ok(())
}

#[test]
fn joined_window_size_is_checked_before_admission() -> io::Result<()> {
    let place = folder("window-size")?;
    let file = place.join("large");
    let listener = Listener::answering(answer)?;
    let max = 16 * 1024 * 1024;
    for (extra, ending) in [(0, ""), (0, "\n"), (0, "\r\n"), (1, "\n")] {
        // The internal feed belongs to the item; only its final ending leaves.
        let input = format!(
            "{}\n{}{}",
            "a".repeat(max / 2),
            "b".repeat(max / 2 - 1 + extra),
            ending
        );
        fs::write(&file, input)?;
        let output = call(
            &listener,
            &[
                "decide",
                "Clear?",
                "--window",
                "2",
                "--plan",
                "--input",
                file.to_str().expect("path"),
            ],
        )?;
        assert_eq!(
            output.status.code(),
            Some(if extra == 0 { 0 } else { 2 }),
            "ending {ending:?}: {}",
            text(&output.stderr)
        );
    }
    assert_eq!(listener.connections(), 0);
    Ok(())
}

#[test]
fn batches_and_splits_preserve_global_labels_and_file_positions() -> io::Result<()> {
    let place = folder("splits")?;
    let one = place.join("one");
    let two = place.join("two");
    fs::write(&one, "one\n\nlast\n")?;
    fs::write(&two, "two\nend\n")?;
    let profile = place.join("profile.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"split","max_questions":1}"#,
    )?;
    for split in [false, true] {
        let listener = Listener::answering(answer)?;
        let profile_args = if split {
            vec!["--profile", profile.to_str().expect("path")]
        } else {
            vec![]
        };
        let output = call(
            &listener,
            &[
                &[
                    "decide",
                    "Clear?",
                    "--lines",
                    "--details",
                    "--batch",
                    "max",
                    "--input",
                    one.to_str().expect("path"),
                    "--input",
                    two.to_str().expect("path"),
                ][..],
                &profile_args,
            ]
            .concat(),
        )?;
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        let rows = rows(&output);
        assert_eq!(rows.len(), 4);
        assert_eq!(
            rows.iter().map(|r| r["input"].clone()).collect::<Vec<_>>(),
            [json!("one"), json!("last"), json!("two"), json!("end")]
        );
        assert_eq!(rows[1]["position"]["first"], 3);
        assert_eq!(rows[2]["position"], json!({"file":two,"first":1,"last":1}));
        assert_eq!(listener.requests().len(), if split { 4 } else { 1 });
    }
    Ok(())
}
