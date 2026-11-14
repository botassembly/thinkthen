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
    let mut writer = child.stdin.take().unwrap();
    let (prefix, rest) = input.split_at(input.len().min(3));
    // A transport preamble may arrive in separate writes.
    for byte in prefix {
        let _ = writer.write_all(std::slice::from_ref(byte));
    }
    let _ = writer.write_all(rest);
    drop(writer);
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

fn yes(id: Value, item: Value) -> Value {
    if id.is_string() {
        json!({"function":"decide","position":1,"id":id,"status":"answered","answer":true,"probability":0.9,"item":item})
    } else {
        json!({"function":"decide","position":id,"status":"answered","answer":true,"probability":0.9,"item":item})
    }
}

fn no(id: Value, item: Value) -> Value {
    json!({"function":"decide","position":id,"status":"answered","answer":false,"probability":0.1,"item":item})
}

#[test]
fn a_stream_preamble_is_not_judged_but_record_characters_are() {
    type Case = (
        &'static [&'static str],
        &'static str,
        &'static [&'static str],
    );
    let cases: [Case; 6] = [
        (&["--json"], "\u{fee0}refund", &["\u{fee0}refund"]),
        (&["--json"], "\u{feff}refund\r\n", &["refund"]),
        (
            &["--from", "lines", "--json"],
            "\u{feff}refund\n\u{feff}later\nx\u{feff}inside\n",
            &["refund", "\u{feff}later", "x\u{feff}inside"],
        ),
        (
            &["--from", "jsonl", "--field", "/body", "--json"],
            "\u{feff}{\"body\":\"refund\"}\n{\"body\":\"\u{feff}later\"}\n",
            &["refund", "\u{feff}later"],
        ),
        (
            &["--from", "jsonl", "--field", "/body", "--into", "answer"],
            "\u{feff}{\"body\":\"refund\"}\n",
            &["refund"],
        ),
        (
            &["--text", "\u{feff}literal", "--json"],
            "",
            &["\u{feff}literal"],
        ),
    ];
    for (args, input, expected) in cases {
        let server = Loopback::refunds().unwrap();
        let output = run(&server, args, None, input.as_bytes());
        assert_eq!(output.status.code(), Some(0), "{args:?}: {output:?}");
        let mut sent: Vec<_> = server
            .requests()
            .iter()
            .flat_map(|request| {
                request.json().unwrap()["questions"]
                    .as_object()
                    .unwrap()
                    .values()
                    .map(|q| q["instructions"].as_str().unwrap().to_owned())
                    .collect::<Vec<_>>()
            })
            .collect();
        let mut expected: Vec<_> = expected
            .iter()
            .map(|text| {
                format!(
                    "The text is {}. Is it a refund request?",
                    serde_json::to_string(text).unwrap()
                )
            })
            .collect();
        sent.sort();
        expected.sort();
        assert_eq!(sent, expected, "{args:?}");
    }
    for args in [
        vec!["--json"],
        vec!["--from", "lines", "--json"],
        vec!["--from", "jsonl", "--json"],
        vec!["--from", "jsonl", "--into", "answer"],
    ] {
        let server = Loopback::refunds().unwrap();
        let empty = run(&server, &args, None, b"");
        let preamble = run(&server, &args, None, b"\xef\xbb\xbf");
        assert_eq!(preamble.status.code(), empty.status.code(), "{args:?}");
        assert_eq!(preamble.stdout, empty.stdout, "{args:?}");
        assert_eq!(preamble.stderr, empty.stderr, "{args:?}");
        assert!(server.requests().is_empty());
    }
    let first = tempfile::NamedTempFile::new().unwrap();
    let second = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(first.path(), "\u{feff}refund\n").unwrap();
    std::fs::write(second.path(), "\u{feff}refund\n").unwrap();
    let server = Loopback::refunds().unwrap();
    let output = run(
        &server,
        &[
            "--from",
            "lines",
            "--json",
            first.path().to_str().unwrap(),
            second.path().to_str().unwrap(),
        ],
        None,
        b"",
    );
    assert_eq!(
        rows(&output),
        vec![
            yes(json!(1), json!("refund")),
            yes(json!(2), json!("refund"))
        ]
    );
    assert_eq!(output.status.code(), Some(0));
    for request in server.requests() {
        for question in request.json().unwrap()["questions"]
            .as_object()
            .unwrap()
            .values()
        {
            assert_eq!(
                question["instructions"],
                "The text is \"refund\". Is it a refund request?"
            );
        }
    }
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
            vec![
                yes(json!(1), json!("refund one")),
                no(json!(2), json!("parcel two")),
                yes(json!(3), json!("refund three")),
            ],
            &["refund one", "parcel two", "refund three"],
        ),
        (
            &["--from", "jsonl", "--json", "--field", "/body"],
            b"{\"id\":\"a\",\"body\":\"refund\"}\r\n{\"body\":\"parcel\"}\n",
            vec![
                yes(json!("a"), json!({"id":"a","body":"refund"})),
                no(json!(2), json!({"body":"parcel"})),
            ],
            &["refund", "parcel"],
        ),
        (
            &["--from", "jsonl", "--json"],
            b"\"refund\"\n",
            vec![yes(json!(1), json!("refund"))],
            &["refund"],
        ),
        (&["--from", "lines", "--json"], b"", vec![], &[]),
    ];
    for (args, input, expected, field) in cases {
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
        let mut quoted: Vec<String> = field
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
            &["--from", "jsonl", "--json", "--field", "/body"],
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
fn a_usage_error_exits_2_and_sends_nothing() {
    // (arguments, the code reported, or None when the argument parser refuses)
    let cases: [(&[&str], Option<&str>); 3] = [
        (
            &["--from", "lines", "--json", "--from", "jsonl", "--json"],
            None,
        ),
        (
            &["--from", "jsonl", "--json", "--field", "body"],
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
    assert_eq!(actual[0], yes(json!(1), json!("refund")));
    assert_eq!(actual[1]["error"]["code"], "record.invalid");
    assert_eq!(actual[2], yes(json!(3), json!("more")));
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

#[test]
fn record_byte_limit_returns_before_the_input_pipe_closes() {
    for (format, preamble) in [
        ("text", b"".as_slice()),
        ("lines", b"".as_slice()),
        ("jsonl", b"".as_slice()),
        ("text", b"\xef\xbb\xbf".as_slice()),
        ("lines", b"\xef\xbb\xbf".as_slice()),
        ("jsonl", b"\xef\xbb\xbf".as_slice()),
    ] {
        let mut child = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", format])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        input.write_all(preamble).unwrap();
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

#[test]
fn field_selects_records() {
    let server = Loopback::noul(0.9).unwrap();
    let input = b"{\"body\":\"refund\",\"evidence\":\"unrelated user data\"}\n";
    let output = run(
        &server,
        &["--from", "jsonl", "--json", "--field", "/body"],
        None,
        input,
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(rows(&output)[0]["answer"], true);
    let sent = server.requests()[0].json().unwrap();
    assert!(sent.to_string().contains("refund"));
    assert!(!sent.to_string().contains("unrelated user data"));
}

#[test]
fn field_environment_selects_records() {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(
        file.path(),
        "{\"body\":\"refund\",\"evidence\":\"caller field\"}\n",
    )
    .unwrap();
    for selected in [true, false] {
        let server = Loopback::noul(0.9).unwrap();
        let mut command = support::command();
        command
            .args(["decide", "Ready?", "--from", "jsonl", "--json"])
            .arg(file.path())
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY);
        if selected {
            command.env("THINKTHEN_FIELD", "/body");
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(0), "{output:?}");
        let sent = server.requests()[0].json().unwrap().to_string();
        assert_eq!(sent.contains("caller field"), !selected);
    }
}

#[test]
fn whole_float_ids_become_integers_and_null_and_other_floats_fail_their_row() {
    let server = Loopback::refunds().unwrap();
    let input = [
        r#"{"id":3.0,"body":"refund"}"#,
        r#"{"id":-2e0,"body":"refund"}"#,
        r#"{"id":9007199254740991.0,"body":"refund"}"#,
        r#"{"id":null,"body":"refund"}"#,
        r#"{"id":1.5,"body":"refund"}"#,
        r#"{"id":9007199254740992.0,"body":"refund"}"#,
    ]
    .join("\n");
    let output = run(
        &server,
        &["--from", "jsonl", "--json", "--field", "/body"],
        None,
        input.as_bytes(),
    );
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    let rows = rows(&output);
    let answered = [json!(3), json!(-2), json!(9_007_199_254_740_991_i64)];
    for (row, id) in rows.iter().zip(&answered) {
        assert_eq!(row["status"], "answered", "{row}");
        assert_eq!(&row["id"], id, "{row}");
    }
    assert_eq!(rows.len(), 6);
    for row in &rows[3..] {
        assert_eq!(row["status"], "failed", "{row}");
        assert_eq!(row["error"]["code"], "record.invalid", "{row}");
        assert!(row.get("id").is_none(), "{row}");
    }
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.contains("id: expected a string or a whole number; received null"),
        "{error}"
    );
}
