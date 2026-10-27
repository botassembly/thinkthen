#![cfg(test)]
//! A compiled C consumer owns the foreign boundary checks.

mod support;
use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex, mpsc};

fn command(mode: &str) -> Command {
    let mut command = support::command("thinkthen-c-consumer");
    command.arg(mode);
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

#[test]
fn c_header_retains_calls_rows_and_cross_thread_error_ownership() {
    let output = lines(command("ownership").output().unwrap());
    assert_eq!(output.len(), 3);
    assert_eq!(output[0]["fields"]["status"], "answered");
    assert_eq!(output[0]["position"], 1);
    assert_eq!(output[1]["fields"]["error"]["code"], "record.invalid");
    assert_eq!(output[2]["error"]["code"], "usage.abi");
}

#[test]
fn single_c_driver_bounds_batches_and_streams_text_and_structured_records() {
    let server = Loopback::noul(0.9).unwrap();
    let client = json!({"url":server.base(),"batch":3});
    let request = json!({"question":{"function":"decide","question":"Ready?","evidence":"/body"}});
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
        assert_eq!(row["fields"]["status"], "answered");
    }
    assert_eq!(output[1]["fields"]["id"], "object");
    let counts: Vec<_> = server
        .requests()
        .iter()
        .map(|request| {
            request.json().unwrap()["questions"]
                .as_object()
                .unwrap()
                .len()
        })
        .collect();
    assert!(counts.iter().all(|&count| (1..=3).contains(&count)));
    assert_eq!(counts.iter().sum::<usize>(), 7);
    assert_eq!(counts.len(), 3);
    let empty = run(&json!({"backend":"stub"}), &request, &[]);
    assert_eq!(empty, [json!({"end":true,"needs":1})]);
}

#[test]
fn failed_admission_returns_its_usage_code_as_owned_json() {
    let output = run(
        &json!({"backend":"stub"}),
        &json!({"question":{"function":"unknown"}}),
        &[],
    );
    assert_eq!(output.len(), 1);
    assert_eq!(output[0]["error"]["code"], "usage.question");
    let bad_client = run(&json!({"backend":"stub","unknown":true}), &json!({}), &[]);
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
        writeln!(
            input,
            "{}\n{}",
            json!({"url":server.base()}),
            json!({"question":{"function":"decide","question":"Ready?"},"records":["held"]})
        )
        .unwrap();
        input.flush().unwrap();
        request.recv().unwrap();
        input.write_all(b"x").unwrap();
        drop(input);
        let output = lines(child.wait_with_output().unwrap());
        if mode == "cancel" {
            assert_eq!(output.len(), 1, "{output:?}");
            assert_eq!(output[0]["error"]["code"], "local.cancelled");
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
    let request =
        json!({"question":{"function":"recognize","kinds":["person"]},"records":["😀 Ada"]});
    let server = Loopback::case(&json!({"replies":[
        {"criterion":"BEGIN", "distribution":{"BEGIN":0,"INSIDE":0,"END":0,"SINGLE":1,"OUT":0}},
        {"distribution":{"person":1,"none of these":0}}
    ]}))
    .unwrap();
    let client = json!({"url":server.base()});
    for (mode, start, end) in [("run", 5, 8), ("utf16", 3, 6), ("points", 2, 5)] {
        let output = run_mode(mode, &client, &request, &[]);
        assert_eq!(output.len(), 2);
        assert_eq!(output[0]["fields"]["answer"][0]["text"], "Ada");
        assert_eq!(output[0]["fields"]["answer"][0]["start"], start);
        assert_eq!(output[0]["fields"]["answer"][0]["end"], end);
    }
    assert_eq!(server.count(), 6);
    let output = run_mode("invalid-offset", &client, &request, &[]);
    assert_eq!(output.len(), 1);
    assert_eq!(output[0]["error"]["code"], "usage.abi");
    assert_eq!(server.count(), 6);
}

#[test]
fn c_single_record_admission_failure_returns_a_row_without_a_call_error() {
    let output = run_mode("single", &json!({"backend":"stub"}), &json!({}), &[]);
    assert_eq!(output.len(), 2);
    assert_eq!(output[0]["fields"]["error"]["code"], "record.missing");
    assert_eq!(output[0]["position"], 1);
    assert_eq!(output[1], json!({"end":true}));
}

#[test]
fn c_threads_share_one_client_and_keep_call_rows_separate() {
    let output = lines(command("concurrent").output().unwrap());
    assert_eq!(output, [json!({"concurrent":true,"rows":32})]);
}

#[test]
fn c_stream_packs_individually_sent_records_into_full_requests() {
    let server = Loopback::noul(0.9).unwrap();
    let records = (0..500).map(|i| format!("Trow {i}")).collect::<Vec<_>>();
    let rows = run(
        &json!({"url":server.base(),"batch":50}),
        &json!({"question":{"function":"decide","question":"Ready?"}}),
        &records,
    );
    assert_eq!(rows.len(), 501);
    for (i, row) in rows[..500].iter().enumerate() {
        assert_eq!(row["position"], i + 1);
        assert_eq!(row["fields"]["answer"], true);
    }
    assert_eq!(server.count(), 10);
}
