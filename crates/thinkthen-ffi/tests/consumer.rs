#![cfg(test)]
//! A compiled C consumer owns the foreign boundary checks.

use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex, mpsc};

fn command(mode: &str) -> Command {
    let test = std::env::current_exe().unwrap();
    let binary = test
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("thinkthen-c-consumer");
    let mut command = Command::new(binary);
    command.arg(mode).env_clear();
    if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile);
    }
    command
}

fn lines(output: Output) -> Vec<Value> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn run(client: &Value, request: &Value, records: &[String]) -> Vec<Value> {
    run_mode("run", client, request, records)
}

fn run_mode(mode: &str, client: &Value, request: &Value, records: &[String]) -> Vec<Value> {
    let mut child = command(mode)
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    writeln!(input, "{client}\n{request}").unwrap();
    for record in records {
        writeln!(input, "{record}").unwrap();
    }
    drop(input);
    lines(child.wait_with_output().unwrap())
}

fn contract_row(envelope: &Value) -> Value {
    let mut row = envelope["fields"].clone();
    if !envelope["detail"].is_null() {
        row.as_object_mut()
            .unwrap()
            .insert("detail".to_owned(), envelope["detail"].clone());
    }
    row
}

#[test]
fn c_header_runs_all_shared_stub_cases_and_retains_row_ownership() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../../../construction/stub.json")).unwrap();
    assert_eq!(cases.len(), 10);
    let monitor = Loopback::noul(0.99).unwrap();
    for case in cases {
        let mut client = case["client"].clone();
        client["url"] = json!(monitor.base());
        let output = run(&client, &case["request"], &[]);
        assert_eq!(output.last().unwrap()["end"], true);
        let rows: Vec<_> = output[..output.len() - 1]
            .iter()
            .map(contract_row)
            .collect();
        assert_eq!(
            rows,
            case["expected"].as_array().unwrap().clone(),
            "{}",
            case["request"]["function"]
        );
    }
    assert_eq!(monitor.count(), 0);
    let output = lines(command("ownership").output().unwrap());
    assert_eq!(output.len(), 3);
    assert_eq!(output[0]["error"]["code"], "usage.record");
    assert_eq!(output[1]["fields"]["status"], "answered");
    assert_eq!(output[1]["position"], 1);
    assert_eq!(output[2]["error"]["code"], "usage.record");
}

#[test]
fn single_c_driver_fills_batches_and_streams_text_and_structured_records() {
    let server = Loopback::noul(0.9).unwrap();
    let client = json!({"url":server.base(),"batch":3});
    let request = json!({"function":"decide","question":{"type":"inline","value":"Ready?"},"options":{"on":"/body"}});
    let mut process = command("run");
    process
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = process.spawn().unwrap();
    let mut input = child.stdin.take().unwrap();
    writeln!(input, "{client}\n{request}").unwrap();
    for record in [
        "Tfirst 😀",
        "J{\"id\":\"object\",\"body\":\"second\"}",
        "Tthird",
        "Tfourth",
        "J{\"body\":\"fifth\"}",
        "Tsixth",
        "Tseventh",
    ] {
        writeln!(input, "{record}").unwrap();
    }
    drop(input);
    let output = lines(child.wait_with_output().unwrap());
    assert_eq!(output.len(), 8);
    assert_eq!(output.last().unwrap(), &json!({"end":true,"needs":8}));
    for (position, row) in output[..7].iter().enumerate() {
        assert_eq!(row["fields"]["answer"], true);
        assert_eq!(row["position"], position + 1);
        assert_eq!(row["kept"], 1);
    }
    assert_eq!(output[1]["fields"]["id"], "object");
    let mut counts: Vec<_> = server
        .requests()
        .iter()
        .map(|request| {
            request.json().unwrap()["questions"]
                .as_object()
                .unwrap()
                .len()
        })
        .collect();
    counts.sort_unstable();
    assert_eq!(counts, [1, 3, 3]);
    let empty = run(&json!({"adapter":"stub"}), &request, &[]);
    assert_eq!(empty, [json!({"end":true,"needs":1})]);
}

#[test]
fn failed_admission_returns_its_usage_code_as_owned_json() {
    let output = run(
        &json!({"adapter":"stub"}),
        &json!({"function":"unknown"}),
        &[],
    );
    assert_eq!(output.len(), 1);
    assert_eq!(output[0]["error"]["code"], "usage.question");
    let bad_client = run(&json!({"adapter":"stub","unknown":true}), &json!({}), &[]);
    assert_eq!(bad_client[0]["error"]["code"], "usage.settings");
}

#[test]
fn c_cancel_and_free_return_while_the_loopback_reply_is_still_held() {
    for mode in ["cancel", "free-held"] {
        let (arrived, request) = mpsc::channel();
        let (release, held) = mpsc::channel();
        let held = Arc::new(Mutex::new(held));
        let server = Loopback::start(move |_| {
            arrived.send(()).unwrap();
            held.lock().unwrap().recv().unwrap();
            Reply::json(200, &json!({}))
        })
        .unwrap();
        let mut process = command(mode);
        process
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = process.spawn().unwrap();
        let mut input = child.stdin.take().unwrap();
        writeln!(input, "{}\n{}", json!({"url":server.base()}), json!({"function":"decide","question":{"type":"inline","value":"Ready?"},"records":["held"]})).unwrap();
        input.flush().unwrap();
        request.recv().unwrap();
        input.write_all(b"x").unwrap();
        drop(input);
        let output = lines(child.wait_with_output().unwrap());
        if mode == "cancel" {
            // A blocked next may receive the worker's cancelled row before it
            // observes cancellation itself. Both paths end with the owned error.
            assert!((1..=2).contains(&output.len()), "{output:?}");
            assert_eq!(output.last().unwrap()["error"]["code"], "local.cancelled");
            for row in &output[..output.len() - 1] {
                assert_eq!(row["position"], 1);
                assert_eq!(row["fields"]["status"], "failed");
                assert_eq!(row["fields"]["error"]["code"], "local.cancelled");
            }
        } else {
            assert_eq!(output.len(), 1);
            assert_eq!(output[0]["freed"], true);
        }
        assert_eq!(server.count(), 1);
        release.send(()).unwrap();
    }
}

#[test]
fn c_selector_values_map_to_shared_offset_units() {
    let request = json!({"function":"recognize","question":{"type":"inline","value":[]},"records":["😀 Ada"]});
    let server = Loopback::answering(|_| json!({"type":"choice","probabilities":{"BEGIN":0,"INSIDE":0,"END":0,"SINGLE":1,"OUT":0}})).unwrap();
    let client = json!({"url":server.base()});
    for (mode, start, end) in [("run", 5, 8), ("utf16", 3, 6), ("points", 2, 5)] {
        let output = run_mode(mode, &client, &request, &[]);
        assert_eq!(output.len(), 2);
        assert_eq!(output[0]["fields"]["answer"][0]["text"], "Ada");
        assert_eq!(output[0]["fields"]["answer"][0]["start"], start);
        assert_eq!(output[0]["fields"]["answer"][0]["end"], end);
    }
    assert_eq!(server.count(), 3);
    let output = run_mode("invalid-offset", &client, &request, &[]);
    assert_eq!(output.len(), 1);
    assert_eq!(output[0]["error"]["code"], "usage.question");
    let request = json!({"function":"decide","question":{"type":"inline","value":"Ready?"},"options":{"on":"/body"},"records":[{}]});
    let output = run_mode("single", &json!({"adapter":"stub"}), &request, &[]);
    assert_eq!(output.len(), 2);
    assert_eq!(output[0]["fields"]["error"]["code"], "usage.missing");
    assert_eq!(output[1]["error"]["code"], "usage.missing");
}
