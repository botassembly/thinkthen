//! Unchanged single-source bytes, label routes and request identities.

use super::input_sources::{answer, folder, text};
use crate::harness::{Listener, spawn};
use serde_json::json;
use std::fs;
use std::io;

#[cfg(unix)]
#[test]
fn non_utf8_file_names_keep_answers_and_use_display_strings() -> io::Result<()> {
    use crate::child::ChildEnvironment as _;
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt as _;
    use std::process::Command;

    let place = folder("non-utf8")?;
    let one = place.join(OsString::from_vec(b"one-\xff".to_vec()));
    let two = place.join(OsString::from_vec(b"two-\xfe".to_vec()));
    fs::write(&one, "one\n")?;
    fs::write(&two, "two\n")?;
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
        for (details, multiple) in [(false, false), (true, false), (false, true), (true, true)] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
            command
                .clear_environment()
                .home(place.join("home"))
                .args([verb, question])
                .args(labels)
                .args(["--url", listener.base(), "--model", "local-1", "--no-cache"])
                .arg("--input")
                .arg(&one);
            if details {
                command.arg("--details");
            }
            if multiple {
                command.arg("--input").arg(&two);
            }
            let output = command.output()?;
            assert_eq!(
                output.status.code(),
                Some(0),
                "{verb}: {}",
                text(&output.stderr)
            );
            let printed = text(&output.stdout);
            assert_named_output(&printed, verb, details, multiple, [&one, &two]);
        }
        // Names and display flags do not change the actual evidence sent.
        assert!(
            listener.requests().iter().all(|request| {
                let body = text(&request.body);
                !body.contains("one-") && !body.contains("two-")
            }),
            "{verb}"
        );
    }
    Ok(())
}

#[cfg(unix)]
fn assert_named_output(
    printed: &str,
    verb: &str,
    details: bool,
    multiple: bool,
    paths: [&std::path::Path; 2],
) {
    if !details && !multiple {
        let expected = match verb {
            "decide" => "true\n",
            "filter" | "rank" => "one\n",
            "choose" => "\"a\"\n",
            "score" => "0.8\n",
            "tag" => "[\"a\"]\n",
            _ => "{\"ok\":true}\n",
        };
        assert_eq!(printed, expected, "{verb}");
    } else if details || matches!(verb, "decide" | "choose" | "score" | "tag") {
        let rows: Vec<serde_json::Value> = printed
            .lines()
            .map(|line| serde_json::from_str(line).expect("result"))
            .collect();
        assert_eq!(rows.len(), if multiple { 2 } else { 1 }, "{verb}");
        for (row, path) in rows.iter().zip(paths) {
            let display = path.to_string_lossy();
            if details {
                assert_eq!(row["position"], json!({"file":display,"first":1,"last":1}));
            }
            if multiple && matches!(verb, "decide" | "choose" | "score" | "tag") {
                assert_eq!(row["input_file"], json!(display));
            }
        }
    } else {
        assert_eq!(
            printed,
            if verb == "annotate" {
                "{\"ok\":true}\n{\"ok\":true}\n"
            } else {
                "one\ntwo\n"
            }
        );
    }
}

#[test]
fn trailing_words_name_missing_files_and_never_run_instructions() -> io::Result<()> {
    let place = folder("missing-positionals")?;
    let missing = place.join("missing");
    let listener = Listener::answering(answer)?;
    for question in ["a", "if"] {
        let output = spawn(
            &[
                "decide",
                question,
                missing.to_str().expect("path"),
                "--url",
                listener.base(),
                "--no-cache",
            ],
            &[],
            b"Refund me please.",
        )?;
        assert_eq!(output.status.code(), Some(5));
        assert!(output.stdout.is_empty());
        assert!(text(&output.stderr).contains("--input could not be opened"));
    }
    let output = spawn(
        &[
            "decide",
            "Clear?",
            "--url",
            listener.base(),
            "--no-cache",
            "--",
            "--threshold",
            "-.5",
        ],
        &[],
        b"Refund me please.",
    )?;
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert!(text(&output.stderr).starts_with("thinkthen: --input could not be opened: "));
    assert!(!text(&output.stderr).contains("--threshold"));
    assert_eq!(listener.connections(), 0);
    Ok(())
}

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
        let plan: serde_json::Value =
            serde_json::from_str(printed.lines().next().expect("plan")).expect("plan JSON");
        let questions = plan["request"]["questions"].as_object().expect("questions");
        for label in [a, b] {
            let quoted = serde_json::to_string(label).expect("JSON label");
            let authored = questions.values().any(|question| {
                question["criteria"].get(label).is_some()
                    || question["criteria"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .any(|criterion| criterion.as_str() == Some(label))
                    || question["instructions"]
                        .as_str()
                        .unwrap_or_default()
                        .contains(&quoted)
            });
            assert!(authored, "{verb}");
        }
        assert_eq!(listener.connections(), 0);
    }
    Ok(())
}

#[test]
fn aggregate_commands_offer_windows_and_refuse_missing_inputs() -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    for verb in ["find", "recognize", "relate"] {
        let output = spawn(&[verb, "--help"], &[], b"")?;
        assert_eq!(output.status.code(), Some(0));
        assert!(text(&output.stdout).contains("--window"), "{verb}");
    }
    for command in [
        vec!["find", "Clear?", "--input", "one", "--input", "two"],
        vec!["recognize", "--input", "one", "--input", "two"],
        vec!["relate", "member", "--input", "one", "--input", "two"],
    ] {
        let output = spawn(
            &[&command[..], &["--url", listener.base(), "--no-cache"]].concat(),
            &[],
            b"one\ntwo",
        )?;
        assert_eq!(
            output.status.code(),
            Some(5),
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
