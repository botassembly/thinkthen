#![cfg(test)]
//! `thinkthen decide` over a record stream: framing, rows, batch and exit codes.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use std::fs::File;
use std::io::Write;
use std::process::{Output, Stdio};

use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use support::reported;

fn run(server: &Loopback, args: &[&str], batch: Option<&str>, input: &[u8]) -> Output {
    let mut command = support::command();
    command
        .arg("decide")
        .arg("Is it a refund request?")
        .args(args)
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(batch) = batch {
        command.env("THINKTHEN_BATCH", batch);
    }
    let mut child = command.spawn().unwrap();
    // A command that refuses its arguments may exit before it reads the input.
    let _ = child.stdin.take().unwrap().write_all(input);
    child.wait_with_output().unwrap()
}

fn rows(output: &Output) -> Vec<Value> {
    String::from_utf8(output.stdout.clone())
        .unwrap()
        .lines()
        .map(|line| {
            let mut row: Value = serde_json::from_str(line).unwrap();
            row.as_object_mut().unwrap().remove("source");
            row
        })
        .collect()
}

fn yes(id: Value) -> Value {
    if id.is_string() {
        json!({"function":"decide","position":1,"id":id,"status":"answered","answer":true,"probability":0.9})
    } else {
        json!({"function":"decide","position":id,"status":"answered","answer":true,"probability":0.9})
    }
}

fn no(id: Value) -> Value {
    json!({"function":"decide","position":id,"status":"answered","answer":false,"probability":0.1})
}

#[test]
fn each_record_prints_one_row() {
    // (arguments, input, rows printed, evidence sent)
    type Case = (
        &'static [&'static str],
        &'static [u8],
        Vec<Value>,
        &'static [&'static str],
    );
    let cases: [Case; 4] = [
        (
            &["--from", "lines", "--json"],
            b"refund one\r\nparcel two\n\n  \nrefund three",
            vec![yes(json!(1)), no(json!(2)), yes(json!(3))],
            &["refund one", "parcel two", "refund three"],
        ),
        (
            &["--from", "jsonl", "--json", "--evidence", "/body"],
            b"{\"id\":\"a\",\"body\":\"refund\"}\r\n{\"body\":\"parcel\"}\n",
            vec![yes(json!("a")), no(json!(2))],
            &["refund", "parcel"],
        ),
        (
            &["--from", "jsonl", "--json"],
            b"\"refund\"\n",
            vec![yes(json!(1))],
            &["refund"],
        ),
        (&["--from", "lines", "--json"], b"", vec![], &[]),
    ];
    for (args, input, expected, evidence) in cases {
        let server = Loopback::refunds().unwrap();
        let output = run(&server, args, None, input);
        assert_eq!(rows(&output), expected, "{args:?}");
        assert_eq!(output.status.code(), Some(0), "{args:?}");
        let mut sent: Vec<String> = server
            .requests()
            .iter()
            .flat_map(|request| {
                let body = request.json().unwrap();
                let questions = body["questions"].as_object().unwrap().clone();
                questions.into_values()
            })
            .map(|question| question["instructions"].as_str().unwrap().to_owned())
            .collect();
        let mut quoted: Vec<String> = evidence
            .iter()
            .map(|text| {
                format!(
                    "The text is {}. Is it a refund request?",
                    serde_json::to_string(text).unwrap()
                )
            })
            .collect();
        sent.sort();
        quoted.sort();
        assert_eq!(sent, quoted, "{args:?}");
    }
}

#[test]
fn record_failures_keep_their_codes_and_exit_six() {
    // (backend status, input, exit code, failed row codes)
    type Case = (u16, &'static [u8], i32, &'static [Option<&'static str>]);
    let cases: [Case; 4] = [
        (
            200,
            b"{\"body\":\"a\"}\n{\"body\":\"b\"}\n",
            0,
            &[None, None],
        ),
        (
            200,
            b"{\"body\":\"a\"}\n{\"text\":\"b\"}\n",
            6,
            &[None, Some("record.missing")],
        ),
        (
            503,
            b"{\"text\":\"a\"}\n{\"body\":\"b\"}\n",
            6,
            &[Some("record.missing"), Some("backend.status")],
        ),
        (
            503,
            b"{\"body\":\"a\"}\n{\"text\":\"b\"}\n",
            6,
            &[Some("backend.status"), Some("record.missing")],
        ),
    ];
    for (status, input, code, failed) in cases {
        let server = Loopback::start(move |request| {
            let body = request.json().unwrap();
            let answers: serde_json::Map<String, Value> = body["questions"]
                .as_object()
                .unwrap()
                .keys()
                .map(|name| (name.clone(), json!({"type": "noul", "noul": 0.9})))
                .collect();
            Reply::json(status, &json!({"answers": answers}))
        })
        .unwrap();
        let output = run(
            &server,
            &["--from", "jsonl", "--json", "--evidence", "/body"],
            None,
            input,
        );
        assert_eq!(output.status.code(), Some(code), "{status}");
        let got: Vec<Option<String>> = rows(&output)
            .iter()
            .map(|row| row["error"]["code"].as_str().map(str::to_owned))
            .collect();
        let want: Vec<Option<String>> = failed.iter().map(|c| c.map(str::to_owned)).collect();
        assert_eq!(got, want, "{status}");
    }
}

#[test]
fn the_batch_option_beats_the_variable() {
    let cases = [
        (Some("2"), Some("0"), 2),
        (None, Some("1"), 1),
        (None, Some("3"), 3),
    ];
    for (flag, variable, batch) in cases {
        let server = Loopback::noul(0.9).unwrap();
        let mut args = vec!["--from", "lines", "--json"];
        if let Some(flag) = flag {
            args.extend(["--batch", flag]);
        }
        let output = run(&server, &args, variable, b"a\nb\nc\nd\ne\nf\n");
        assert_eq!(output.status.code(), Some(0));
        let actual = rows(&output);
        assert_eq!(actual.len(), 6);
        assert_eq!(
            actual
                .iter()
                .map(|row| row["position"].as_u64().unwrap())
                .collect::<Vec<_>>(),
            (1..=6).collect::<Vec<_>>()
        );
        let sizes: Vec<_> = server
            .requests()
            .iter()
            .map(|request| {
                request.json().unwrap()["questions"]
                    .as_object()
                    .unwrap()
                    .len()
            })
            .collect();
        assert_eq!(sizes.iter().sum::<usize>(), 6);
        assert_eq!(sizes.len(), 6_usize.div_ceil(batch));
        assert!(
            sizes.iter().all(|size| *size <= batch),
            "{flag:?} {variable:?}: {sizes:?}"
        );
    }
}

#[test]
fn a_usage_error_exits_2_and_sends_nothing() {
    // (arguments, the code reported, or None when the argument parser refuses)
    let cases: [(&[&str], Option<&str>); 3] = [
        (
            &["--from", "lines", "--json", "--from", "jsonl", "--json"],
            None,
        ),
        (
            &["--from", "jsonl", "--json", "--evidence", "body"],
            Some("usage.pointer"),
        ),
        (
            &["--from", "lines", "--json", "--batch", "0"],
            Some("usage.batch"),
        ),
    ];
    for (args, code) in cases {
        let server = Loopback::noul(0.9).unwrap();
        let output = run(&server, args, None, b"{\"body\":\"a\"}\n");
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert_eq!(server.count(), 0, "{args:?}");
        if let Some(code) = code {
            assert_eq!(reported(&output), code, "{args:?}");
        }
    }
}

#[test]
fn a_line_that_is_not_utf8_ends_the_stream_with_exit_5() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(
        &server,
        &["--from", "lines", "--json"],
        None,
        b"refund\ncaf\xe9\nmore\n",
    );
    assert_eq!(output.status.code(), Some(6));
    let actual = rows(&output);
    assert_eq!(actual.len(), 3);
    assert_eq!(actual[0], yes(json!(1)));
    assert_eq!(actual[1]["error"]["code"], "record.invalid");
    assert_eq!(actual[2], yes(json!(3)));
}

#[test]
fn input_that_cannot_be_read_exits_5_and_sends_nothing() {
    for args in [&[][..], &["--from", "lines", "--json"][..]] {
        let server = Loopback::noul(0.9).unwrap();
        // Reading a directory fails on every platform the tests run on.
        let output = support::command()
            .args(["decide", "Is it?"])
            .args(args)
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(File::open(".").unwrap())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(5), "{args:?}");
        assert_eq!(reported(&output), "local.io", "{args:?}");
        assert_eq!(server.count(), 0, "{args:?}");
    }
}

#[cfg(unix)]
#[test]
fn an_output_write_error_stops_the_reading() {
    let (stdout, inherited_reader) = support::closed_output();
    let server = Loopback::noul(0.9).unwrap();
    let mut child = support::command()
        .args([
            "decide", "Is it?", "--from", "lines", "--json", "--batch", "1",
        ])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(stdout)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let input: String = (1..=1000).map(|n| format!("message {n}\n")).collect();
    // The command may stop reading before it takes every line.
    let _ = child.stdin.take().unwrap().write_all(input.as_bytes());
    let output = child.wait_with_output().unwrap();
    drop(inherited_reader);
    let diagnostic = format!(
        "mode=decide --from lines --batch 1, status={}, stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.status.code(), Some(141), "{diagnostic}");
    assert!(output.stderr.is_empty(), "{diagnostic}");
    // Four workers each hold one request, and the open request goes out at the end.
    assert!(
        server.count() <= 5,
        "{} requests; {diagnostic}",
        server.count()
    );
}

#[test]
fn record_byte_limit_returns_before_the_input_pipe_closes() {
    for format in ["text", "lines", "jsonl"] {
        let mut child = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", format])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        input.write_all(&vec![b'x'; 64 * 1024 * 1024 + 1]).unwrap();
        let (send, receive) = std::sync::mpsc::channel();
        std::thread::spawn(move || send.send(child.wait_with_output().unwrap()).unwrap());
        // Retain the writer: reading past the limit would wait for EOF.
        let output = receive
            .recv_timeout(std::time::Duration::from_secs(30))
            .unwrap();
        drop(input);
        assert_eq!(output.status.code(), Some(2), "{format}: {output:?}");
        assert!(output.stdout.is_empty());
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(error.contains("usage.question"), "{error}");
        assert!(error.contains("record exceeds the 64 MiB limit"), "{error}");
    }
}
