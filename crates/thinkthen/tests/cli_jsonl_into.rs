#![cfg(test)]
#![cfg(feature = "command")]
//! JSON Lines answer fields preserve the input rather than replacing it with rows.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Output, Stdio};

fn run(args: &[&str], input: &str) -> Output {
    let mut child = support::command()
        .args(["--backend", "stub"])
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = child.stdin.take().unwrap().write_all(input.as_bytes());
    child.wait_with_output().unwrap()
}
fn rows(output: &Output) -> Vec<Value> {
    String::from_utf8(output.stdout.clone())
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[test]
fn typed_answers_append_without_changing_original_members() {
    let input = "{\"id\":18446744073709551615,\"text\":\"Please refund my order.\",\"nested\":[false,null],\"precise\":1.234567890123456789012345,\"exponent\":1e123}\n{\"text\":\"Where is my parcel?\"}\n{\"text\":\"I was charged twice.\"}\n";
    for args in [
        vec!["decide", "Refund?"],
        vec![
            "choose", "Topic?", "--choice", "billing", "--choice", "shipping",
        ],
        vec![
            "tag", "Topics?", "--label", "billing", "--label", "shipping",
        ],
        vec![
            "score", "Urgency?", "--level", "routine", "--level", "urgent",
        ],
    ] {
        let mut baseline = args.clone();
        baseline.extend(["--from", "jsonl", "--field", "/text", "--json"]);
        let expected = run(&baseline, input);
        assert_eq!(expected.status.code(), Some(0), "{expected:?}");
        let mut append = args;
        append.extend(["--from", "jsonl", "--field", "/text", "--into", "result"]);
        let actual = run(&append, input);
        assert_eq!(actual.status.code(), Some(0), "{actual:?}");
        let got = rows(&actual);
        assert_eq!(got.len(), 3, "{actual:?}");
        let original: Vec<Value> = input
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        for ((mut object, original), answer) in got.into_iter().zip(original).zip(rows(&expected)) {
            assert_eq!(
                object.as_object_mut().unwrap().remove("result"),
                Some(answer["answer"].clone())
            );
            assert_eq!(object, original);
        }
        let raw = String::from_utf8(actual.stdout).unwrap();
        assert!(raw.contains("18446744073709551615"));
        assert!(raw.contains("1.234567890123456789012345"));
        assert!(raw.contains("1e123"));
    }
}

#[test]
fn unsure_and_failed_fields_are_distinct_from_false() {
    let output = run(
        &[
            "decide", "Refund?", "--from", "jsonl", "--field", "/text", "--low", "0.000001",
            "--high", "0.999999", "--into", "result",
        ],
        "{\"text\":\"Refund please\"}\n{\"other\":1}\n",
    );
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    let got = rows(&output);
    assert_eq!(got[0]["result"], "unsure");
    assert_eq!(got[1]["result"], Value::Null);
    assert!(String::from_utf8_lossy(&output.stderr).contains("record.missing"));
}

#[test]
fn annotation_field_contains_typed_children() {
    let dir = tempfile::tempdir().unwrap();
    let toolbox = dir.path().join("questions.json");
    std::fs::write(&toolbox, json!({"questions":{"refund":{"function":"decide","question":"Refund?"},"topic":{"function":"choose","question":"Topic?","choices":["billing","shipping"]}}}).to_string()).unwrap();
    let output = run(
        &[
            "annotate",
            toolbox.to_str().unwrap(),
            "--from",
            "jsonl",
            "--into",
            "triage",
        ],
        "{\"text\":\"Refund please\"}\n{\"text\":\"Shipping please\"}\n{\"text\":\"Invoice please\"}\n",
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(rows(&output).len(), 3);
    for row in rows(&output) {
        assert!(row["triage"]["refund"].is_boolean());
        assert!(row["triage"]["topic"].is_string());
        assert_eq!(row["triage"].as_object().unwrap().len(), 2);
    }
}

#[test]
fn whole_input_is_admitted_before_any_backend_call_or_output() {
    let server = loopback::Loopback::noul(0.9).unwrap();
    for bad in ["null", "[]", "42", "broken", "", "{\"result\":null}"] {
        let input = format!("{{\"text\":\"first\"}}\n{bad}\n");
        let mut child = support::command()
            .args([
                "decide",
                "Refund?",
                "--from",
                "jsonl",
                "--into",
                "result",
                "--url",
                server.base(),
            ])
            .env("THINKTHEN_API_KEY", loopback::KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let _ = child.stdin.take().unwrap().write_all(input.as_bytes());
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(2), "{bad}: {output:?}");
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("line 2"),
            "{output:?}"
        );
        assert_eq!(server.count(), 0);
    }
}

#[test]
fn annotation_children_keep_unsure_failures_and_false_separate() {
    let dir = tempfile::tempdir().unwrap();
    let toolbox = dir.path().join("questions.json");
    std::fs::write(
        &toolbox,
        json!({"questions":{
            "bad":{"function":"decide","question":"Bad?"},
            "maybe":{"function":"decide","question":"Maybe?","threshold":{"low":0.2,"high":0.8}},
            "no":{"function":"decide","question":"No?"}
        }})
        .to_string(),
    )
    .unwrap();
    let server = loopback::Loopback::answering(|q| {
        let question = q["instructions"].as_str().unwrap();
        json!({"type":"noul","noul":if question.contains("Bad?") {2.0} else if question.contains("Maybe?") {0.5} else {0.1}})
    }).unwrap();
    let input = dir.path().join("input.jsonl");
    std::fs::write(&input, "{\"text\":\"example\"}\n").unwrap();
    let output = support::command()
        .args([
            "annotate",
            toolbox.to_str().unwrap(),
            input.to_str().unwrap(),
            "--into",
            "triage",
            "--url",
            server.base(),
        ])
        .env("THINKTHEN_API_KEY", loopback::KEY)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    assert_eq!(
        rows(&output)[0]["triage"],
        json!({"bad":null,"maybe":"unsure","no":false})
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("backend.reply"));
}

#[test]
fn rank_and_filter_follow_table_output_selection_and_input_order() {
    let input = "{\"id\":1,\"text\":\"first\"}\n{\"id\":2,\"text\":\"second\"}\n{\"id\":3,\"text\":\"third\"}\n";
    for mut args in [
        vec![
            "rank", "Urgency?", "--level", "routine", "--level", "urgent",
        ],
        vec!["filter", "Refund?"],
    ] {
        let mut baseline = args.clone();
        baseline.extend(["--from", "jsonl", "--field", "/text", "--json"]);
        let expected = run(&baseline, input);
        assert_eq!(expected.status.code(), Some(0), "{expected:?}");
        let mut expected = rows(&expected);
        let filter = args[0] == "filter";
        if filter {
            expected.retain(|row| row["kept"] == true);
        }
        expected.sort_by_key(|row| row["position"].as_u64().unwrap());
        args.extend(["--from", "jsonl", "--field", "/text", "--into", "result"]);
        let actual = run(&args, input);
        assert_eq!(actual.status.code(), Some(0), "{actual:?}");
        let actual = rows(&actual);
        assert_eq!(actual.len(), expected.len());
        for (got, want) in actual.iter().zip(expected) {
            assert_eq!(got["id"], want["id"]);
            assert_eq!(got["result"], want["answer"]);
        }
    }
}

#[test]
fn json_row_and_table_only_options_remain_separate() {
    for extra in [
        vec!["--json"],
        vec!["--output", "unused.jsonl"],
        vec!["--in-place"],
        vec!["--details"],
    ] {
        let mut args = vec!["decide", "Refund?", "--from", "jsonl", "--into", "result"];
        args.extend(extra);
        let output = run(&args, "{\"text\":\"first\"}\n");
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(output.stdout.is_empty());
    }
}
