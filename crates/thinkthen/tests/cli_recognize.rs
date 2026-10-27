#![cfg(test)]
//! Name recognition command framing and rendering.
#![cfg(feature = "command")]
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Output, Stdio};

fn run(server: &Loopback, args: &[&str], input: &str) -> Output {
    let mut child = support::command()
        .arg("recognize")
        .args(if args.contains(&"person") {
            vec![]
        } else {
            vec!["--kind", "ENTITY"]
        })
        .args(args)
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
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}
fn server(single: bool) -> Loopback {
    Loopback::answering(move |q| if q["criteria"].get("ENTITY").is_some() {json!({"type":"choice","probabilities":{"ENTITY":1.0,"none of these":0.0}})} else {json!({"type":"choice","probabilities":{"BEGIN":0.,"INSIDE":0.,"END":0.,"SINGLE":if single {1.}else{0.},"OUT":if single {0.}else{1.}}})}).unwrap()
}
#[test]
fn names_print_one_json_object_each_and_empty_names_succeed() {
    for single in [false, true] {
        let server = server(single);
        let output = run(&server, &[], "Ada");
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
        if single {
            let name: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(name["text"], "Ada");
            assert_eq!(name["kind"], "ENTITY");
            assert_eq!(name["start"], 0);
            assert_eq!(name["end"], 3);
        } else {
            assert!(output.stdout.is_empty());
        }
    }
}
#[test]
fn details_and_records_render_rows_without_top_probability() {
    for (args, input, count) in [
        (vec!["--json"], "Ada", 1),
        (vec!["--from", "lines", "--json"], "Ada\nBea\n", 2),
        (
            vec!["--from", "jsonl", "--json", "--evidence", "/body"],
            "{\"body\":\"Ada\"}\n",
            1,
        ),
    ] {
        let server = server(true);
        let output = run(&server, &args, input);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
        let text = String::from_utf8(output.stdout).unwrap();
        let rows: Vec<Value> = text
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert_eq!(rows.len(), count);
        assert!(rows[0].get("probability").is_none());
        assert_eq!(rows[0]["status"], "answered");
        assert!(rows[0].get("detail").is_some());
    }
}
#[test]
fn threshold_bands_fail_before_sending() {
    let server = server(true);
    let output = run(&server, &["--threshold", "0.2:0.8"], "Ada");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(support::reported(&output), "usage.question");
    assert_eq!(server.count(), 0);
}

#[test]
fn named_kinds_reach_the_dependent_round() {
    let server = Loopback::answering(|q| {
        if q["criteria"].get("BEGIN").is_some() { json!({"type":"choice","probabilities":{"BEGIN":0.,"INSIDE":0.,"END":0.,"SINGLE":1.,"OUT":0.}}) }
        else { json!({"type":"choice","probabilities":{"person":1.,"none of these":0.}}) }
    }).unwrap();
    let output = run(&server, &["--kind", "person"], "Ada");
    assert_eq!(output.status.code(), Some(0));
    let name: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(name["kind"], "person");
    assert_eq!(server.count(), 2);
}
