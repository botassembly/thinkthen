//! All ten command functions share explicit folder provenance and original evidence.

use crate::harness::{Canned, Listener, spawn};
use crate::input_sources::{folder, text};
use serde_json::{Value, json};
use std::{fs, io, path::PathBuf, process::Output};

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../specification/fixtures/files")
}

fn answer(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let questions = request["questions"].as_object().unwrap();
    if questions.values().any(|q| {
        q["criteria"].get("BEGIN").is_some() || q["criteria"].get("none of these").is_some()
    }) {
        return crate::recognize::automatic(body);
    }
    let answers = questions.iter().map(|(key, question)| {
        let value = if question["type"] == "choice" {
            let labels = question["criteria"].as_object().unwrap();
            let picked = if labels.contains_key("u002") { "u002" } else { labels.keys().next().unwrap() };
            let probabilities = labels.keys().map(|label| (label.clone(), Value::from(if label == picked { 1.0 } else { 0.0 }))).collect::<serde_json::Map<_, _>>();
            json!({"type":"choice","choice":picked,"probabilities":probabilities})
        } else if question["type"] == "score" {
            json!({"type":"score","score":0.8,"confidence":0.9,"legend":{"0":"a","1":"b"},"probabilities":{"0":0.2,"1":0.8}})
        } else { json!({"type":"noul","noul":0.9}) };
        (key.clone(), value)
    }).collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
}

fn call(
    listener: &Listener,
    command: &[&str],
    paths: &[&str],
    extra: &[&str],
) -> io::Result<Output> {
    let mut arguments = command.to_vec();
    arguments.extend(["--url", listener.base(), "--model", "local-1", "--no-cache"]);
    for path in paths {
        arguments.extend(["--input", path]);
    }
    arguments.extend(extra);
    spawn(&arguments, &[], b"ignored stdin")
}

fn rows(output: &Output) -> Vec<Value> {
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[test]
fn ten_folder_examples_retain_original_records_and_physical_sources() -> io::Result<()> {
    let place = fixture();
    let documents = place.join("documents");
    let documents = documents.to_str().unwrap();
    let set = place.join("questions.json");
    let commands: Vec<Vec<&str>> = vec![
        vec!["decide", "Does this document contain a support contract?"],
        vec![
            "choose",
            "Which category fits this document?",
            "billing",
            "support",
        ],
        vec!["tag", "Which labels apply?", "refund", "contract"],
        vec!["score", "How urgent is this document?", "a", "b"],
        vec!["filter", "Does this line describe a refund?"],
        vec!["rank", "Does this document discuss a billing dispute?"],
        vec!["find", "Which line gives the refund policy?"],
        vec!["annotate", set.to_str().unwrap()],
        vec!["recognize", "person", "organization"],
        vec!["relate", "connected"],
    ];
    let listener = Listener::answering(answer)?;
    for command in &commands {
        let verb = command[0];
        let unit = if matches!(verb, "filter" | "find") {
            "line"
        } else {
            "file"
        };
        let output = call(&listener, command, &[documents], &["--unit", unit])?;
        let rows = rows(&output);
        assert!(!rows.is_empty(), "{verb}");
        let requests = listener.requests();
        assert!(!requests.is_empty(), "{verb}");
        check_rows(verb, &rows, documents);
    }
    let output = call(&listener, &commands[4], &[documents], &["--files-only"])?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(
        text(&output.stdout),
        format!(
            "{}\n{}\n",
            place.join("documents/01-policy.txt").display(),
            place.join("documents/02-contract.txt").display()
        )
    );
    Ok(())
}

fn check_rows(verb: &str, rows: &[Value], documents: &str) {
    if verb == "relate" {
        for row in rows {
            check_relation(row, documents);
        }
        return;
    }
    for row in rows {
        assert!(
            row["file"].as_str().unwrap().starts_with(documents),
            "{verb}: {row}"
        );
        assert!(row["input"].is_string(), "{verb}: {row}");
    }
    if verb == "find" {
        assert_eq!(rows[0]["first_line"], 2);
        assert_eq!(
            rows[0]["input"],
            "Customers may request a refund within 30 days."
        );
    }
    if verb == "recognize" {
        let ada = rows[0]["value"]["entities"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["text"] == "Ada")
            .unwrap();
        assert_eq!(
            (ada["first_line"].as_u64(), ada["last_line"].as_u64()),
            (Some(3), Some(3))
        );
    }
}

fn check_relation(row: &Value, documents: &str) {
    for endpoint in ["source", "target"] {
        assert!(
            row[endpoint]["file"]
                .as_str()
                .unwrap()
                .starts_with(documents)
        );
        assert_eq!(row[endpoint]["first_line"], 1);
        assert_eq!(row[endpoint]["last_line"], 4);
        assert!(row[endpoint]["record"].as_str().unwrap().contains('\n'));
    }
}

#[test]
fn physical_paths_do_not_change_provider_requests_or_replay_keys() -> io::Result<()> {
    let place = folder("source-identity")?;
    let one = place.join("one");
    let two = place.join("two");
    let recording = place.join("recording");
    fs::write(&one, "same original evidence\r\n")?;
    fs::write(&two, "same original evidence\r\n")?;
    let listener = Listener::answering(answer)?;
    let command = ["decide", "Does this contain evidence?"];
    let output = call(
        &listener,
        &command,
        &[one.to_str().unwrap()],
        &["--unit", "line", "--record", recording.to_str().unwrap()],
    )?;
    assert_eq!(rows(&output)[0]["first_line"], 1);
    assert_eq!(listener.requests().len(), 1);
    let replay = call(
        &listener,
        &command,
        &[two.to_str().unwrap()],
        &["--unit", "line", "--replay", recording.to_str().unwrap()],
    )?;
    assert_eq!(rows(&replay)[0]["file"], two.to_str().unwrap());
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn invalid_reader_options_and_missing_operands_admit_no_requests() -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    let path = fixture().join("documents");
    for extra in [
        vec!["--unit", "file", "--window", "2"],
        vec!["--window", "0"],
        vec!["--unit", "line", "--jsonl"],
    ] {
        let output = call(
            &listener,
            &["decide", "Clear?"],
            &[path.to_str().unwrap()],
            &extra,
        )?;
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        assert!(listener.requests().is_empty());
    }
    let output = call(
        &listener,
        &["decide", "Clear?"],
        &[path.to_str().unwrap(), "/missing-source-0420"],
        &[],
    )?;
    assert_eq!(output.status.code(), Some(5), "{}", text(&output.stderr));
    assert!(listener.requests().is_empty());
    Ok(())
}
