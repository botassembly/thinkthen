//! The batch setting's four tiers and their refusals, by ADR 0048 items 3 and 4.

use std::fs;
use std::process::Output;

use serde_json::{Map, Value};

use super::{KEY, QUESTION, answering, details, folder, lines, text};
use crate::harness::{Listener, spawn};

/// A `decide` question file holding `batch`, as the argument that names it.
fn question_file(place: &str, batch: &str) -> String {
    let path = format!("{place}/{}.json", batch.replace(['"', '.'], "_"));
    let question = format!(r#"{{"decide":"{QUESTION}","batch":{batch}}}"#);
    fs::write(&path, question).expect("a question file");
    format!("@{path}")
}

/// A dry run with the flag, the variable and the file's value given.
fn planning(place: &str, tiers: (Option<&str>, Option<&str>, Option<&str>), input: &str) -> Output {
    let (flag, variable, batch) = tiers;
    let question = batch.map_or_else(|| QUESTION.to_owned(), |batch| question_file(place, batch));
    let mut arguments = vec!["decide", &question, "--dry-run"];
    if input.contains('\n') {
        arguments.push("--lines");
    }
    if let Some(flag) = flag {
        arguments.extend(["--batch", flag]);
    }
    let environment: Vec<(&str, &str)> = variable
        .map(|value| ("THINKTHEN_BATCH", value))
        .into_iter()
        .collect();
    spawn(&arguments, &environment, input.as_bytes()).expect("the command runs")
}

#[test]
fn the_batch_setting_follows_its_tiers() {
    let place = folder("tiers");
    fs::create_dir_all(&place).expect("a folder for the question files");
    let three = lines(1..=3);
    let planned = [
        ((None, None, None), 3),
        ((Some("1"), None, None), 1),
        ((Some("2"), None, None), 2),
        ((Some("max"), None, None), 3),
        ((None, Some("2"), None), 2),
        ((None, Some(""), None), 3),
        ((None, None, Some("1")), 1),
        ((Some("2"), None, Some("1")), 2),
        ((None, Some("max"), Some("1")), 3),
        ((Some("max"), None, Some("2")), 3),
        ((None, Some("1"), Some("2")), 1),
    ];
    for (tiers, count) in planned {
        let output = planning(&place, tiers, &three);
        let plan: Value = serde_json::from_slice(&output.stdout).expect("a plan");
        let questions = plan["request"]["questions"].as_object().map_or(0, Map::len);
        assert_eq!(questions, count, "{tiers:?}: {}", text(&output.stderr));
    }

    let flag = "thinkthen: --batch takes max or a whole number of at least 1\n";
    let variable = "thinkthen: THINKTHEN_BATCH takes max or a whole number of at least 1\n";
    let key = "thinkthen: `batch` in the question file takes max or a whole number of at least 1\n";
    let single =
        "thinkthen: --batch groups the records of a stream, and a single text is one record\n";
    let mut refused: Vec<(Output, i32, &str)> = ["0", "1.5", "fill", ""]
        .into_iter()
        .map(|value| (planning(&place, (Some(value), None, None), &three), 2, flag))
        .collect();
    refused.push((
        planning(&place, (None, Some("0"), None), &three),
        2,
        variable,
    ));
    for value in ["0", "1.5", r#""10""#, r#""fill""#, "true"] {
        refused.push((planning(&place, (None, None, Some(value)), &three), 5, key));
    }
    refused.push((
        planning(&place, (Some("5"), None, None), "line 1"),
        2,
        single,
    ));
    for (output, code, sentence) in refused {
        assert_eq!(
            (output.status.code(), text(&output.stderr).as_str()),
            (Some(code), sentence)
        );
    }
    let one = planning(&place, (None, Some("5"), None), "line 1");
    assert_eq!(one.status.code(), Some(0), "{}", text(&one.stderr));
}

#[test]
fn a_file_batch_leaves_the_question_digest_and_stays_off_annotate_entries() {
    let place = folder("digest");
    fs::create_dir_all(&place).expect("a folder for the question files");
    let listener = Listener::answering(answering).expect("a loopback listener");
    let digests: Vec<Value> = [QUESTION.to_owned(), question_file(&place, "5")]
        .iter()
        .map(|question| {
            let fixed = ["decide", question, "--lines", "--details", "--no-cache"];
            let base = ["--url", listener.base(), "--model", "jev-1.13.0"];
            let output = spawn(&[&fixed[..], &base].concat(), &[KEY], b"line 1\n")
                .expect("the command runs");
            details(&output)[0]["meta"]["question_sha256"].clone()
        })
        .collect();
    assert!(digests[0].is_string());
    assert_eq!(
        digests[0], digests[1],
        "a file's batch leaves the question digest"
    );

    let set = format!("{place}/set.json");
    let entry = r#"{"version":1,"questions":{"ok":{"decide":"Is it yes?","batch":5}}}"#;
    fs::write(&set, entry).expect("a question set");
    let annotate = spawn(
        &["annotate", &set, "--lines", "--dry-run"],
        &[],
        b"line 1\n",
    )
    .expect("the command runs");
    assert_eq!(annotate.status.code(), Some(5));
    assert_eq!(
        text(&annotate.stderr),
        "thinkthen: the question set holds no key `questions.ok.batch`\n"
    );
}
