//! Unchanged single-source bytes, label routes and request identities.

use super::intake_0401::{answer, folder, text};
use crate::harness::{Listener, spawn};
use serde_json::json;
use std::fs;
use std::io;

#[test]
fn single_file_and_stdin_keep_each_commands_original_bytes_and_requests() -> io::Result<()> {
    let place = folder("compat")?;
    let file = place.join("evidence");
    fs::write(&file, "one\nsecond\n")?;
    let set = place.join("set.json");
    fs::write(
        &set,
        r#"{"version":1,"questions":{"ok":{"decide":"Clear?"}}}"#,
    )?;
    for verb in [
        "decide", "filter", "rank", "choose", "score", "tag", "annotate",
    ] {
        let listener = Listener::answering(answer)?;
        let question = if verb == "annotate" {
            set.to_str().expect("path")
        } else {
            "Clear?"
        };
        let labels: &[&str] = match verb {
            "choose" | "score" => &["a", "b"],
            "tag" => &["a"],
            _ => &[],
        };
        let line = [
            &[verb, question][..],
            labels,
            &["--url", listener.base(), "--model", "local-1", "--no-cache"],
        ]
        .concat();
        let stdin = spawn(&line, &[], b"one\nsecond\n")?;
        let sent = listener.requests();
        let count = sent.len();
        let named = spawn(
            &[&line[..], &["--input", file.to_str().expect("path")]].concat(),
            &[],
            b"unused",
        )?;
        assert_eq!(stdin.status.code(), named.status.code(), "{verb}");
        assert_eq!(stdin.stdout, named.stdout, "{verb}");
        assert_eq!(stdin.stderr, named.stderr, "{verb}");
        assert_eq!(listener.count(), count * 2, "{verb}");
        let second_sent = listener.requests();
        // Bodies may arrive in any order; a source name never joins the wire.
        let mut first: Vec<_> = sent.iter().map(|r| r.body.clone()).collect();
        let mut second: Vec<_> = second_sent.iter().map(|r| r.body.clone()).collect();
        first.sort();
        second.sort();
        assert_eq!(first, second, "{verb}");
        let expected = match verb {
            "decide" => "true\n".to_owned(),
            "filter" | "rank" => "one\nsecond\n".to_owned(),
            "choose" => "\"a\"\n".to_owned(),
            "score" => "0.8\n".to_owned(),
            "tag" => "[\"a\"]\n".to_owned(),
            _ => "{\"ok\":true}\n".to_owned(),
        };
        assert_eq!(text(&stdin.stdout), expected, "{verb}");
    }
    Ok(())
}

#[test]
fn labels_that_name_existing_files_remain_labels() -> io::Result<()> {
    let place = folder("label-paths")?;
    let a = place.join("a");
    let b = place.join("b");
    fs::write(&a, "private evidence")?;
    fs::write(&b, "more private evidence")?;
    let a = a.to_str().expect("path");
    let b = b.to_str().expect("path");
    for verb in ["choose", "score", "tag"] {
        let listener = Listener::answering(answer)?;
        let output = spawn(
            &[
                verb,
                "Clear?",
                a,
                b,
                "--url",
                listener.base(),
                "--model",
                "local-1",
                "--no-cache",
                "--plan",
            ],
            &[],
            b"only stdin",
        )?;
        assert_eq!(
            output.status.code(),
            Some(0),
            "{verb}: {}",
            text(&output.stderr)
        );
        let printed = text(&output.stdout);
        assert!(printed.contains("only stdin"), "{verb}");
        assert!(!printed.contains("private evidence"), "{verb}");
        assert!(printed.contains(a) && printed.contains(b), "{verb}");
        assert_eq!(listener.connections(), 0);
    }
    Ok(())
}

#[test]
fn unchanged_aggregate_commands_hide_window_and_refuse_new_input_forms() -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    for verb in ["find", "recognize", "relate"] {
        let output = spawn(&[verb, "--help"], &[], b"")?;
        assert_eq!(output.status.code(), Some(0));
        assert!(!text(&output.stdout).contains("--window"), "{verb}");
    }
    for command in [
        vec!["find", "Clear?", "--input", "one", "--input", "two"],
        vec!["find", "Clear?", "--window", "2"],
        vec!["recognize", "--window", "2"],
        vec!["relate", "member", "--input", "one", "--input", "two"],
    ] {
        let output = spawn(
            &[&command[..], &["--url", listener.base(), "--no-cache"]].concat(),
            &[],
            b"one\ntwo",
        )?;
        assert_eq!(
            output.status.code(),
            Some(2),
            "{command:?}: {}",
            text(&output.stderr)
        );
    }
    assert_eq!(listener.connections(), 0);
    Ok(())
}

#[test]
fn every_default_document_value_command_associates_named_answers() -> io::Result<()> {
    let place = folder("value-documents")?;
    let one = place.join("one");
    let two = place.join("two");
    fs::write(&one, "one\n")?;
    fs::write(&two, "two\n")?;
    for (verb, value) in [
        ("decide", json!(true)),
        ("choose", json!("a")),
        ("score", json!(0.8)),
        ("tag", json!(["a"])),
    ] {
        let labels: &[&str] = match verb {
            "choose" | "score" => &["a", "b"],
            "tag" => &["a"],
            _ => &[],
        };
        for detailed in [false, true] {
            let listener = Listener::answering(answer)?;
            let view: &[&str] = if detailed { &["--details"] } else { &[] };
            let output = spawn(
                &[
                    &[verb, "Clear?"][..],
                    labels,
                    view,
                    &[
                        "--input",
                        one.to_str().expect("path"),
                        "--input",
                        two.to_str().expect("path"),
                        "--url",
                        listener.base(),
                        "--model",
                        "local-1",
                        "--no-cache",
                    ],
                ]
                .concat(),
                &[],
                b"ignored",
            )?;
            assert_eq!(
                output.status.code(),
                Some(0),
                "{verb}: {}",
                text(&output.stderr)
            );
            let rows: Vec<serde_json::Value> = text(&output.stdout)
                .lines()
                .map(|line| serde_json::from_str(line).expect("result"))
                .collect();
            assert_eq!(rows.len(), 2);
            for (row, path) in rows.iter().zip([&one, &two]) {
                assert_eq!(row["value"], value, "{verb}");
                assert_eq!(row["input_file"], json!(path));
                assert_eq!(
                    row.get("position"),
                    detailed.then_some(&json!({"file":path,"first":1,"last":1}))
                );
            }
            assert_eq!(listener.requests().len(), 2);
        }
    }
    Ok(())
}

#[test]
fn a_later_invalid_table_header_is_a_zero_send_preflight_refusal() -> io::Result<()> {
    let place = folder("bad-header")?;
    let one = place.join("one");
    let two = place.join("two");
    fs::write(&one, "id,text\n1,first\n")?;
    fs::write(&two, "id,id\n2,last\n")?;
    let listener = Listener::answering(answer)?;
    let output = spawn(
        &[
            "decide",
            "Clear?",
            "--csv",
            "--input",
            one.to_str().expect("path"),
            "--input",
            two.to_str().expect("path"),
            "--url",
            listener.base(),
            "--no-cache",
        ],
        &[],
        b"",
    )?;
    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
    Ok(())
}
