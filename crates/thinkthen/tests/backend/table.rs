//! CSV and TSV parsing at the compiled binary edge.

use crate::child::ChildEnvironment as _;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use super::harness::{Canned, Listener, finish, spawn_one as spawn};

const KEY: [(&str, &str); 1] = [("THINKTHEN_API_KEY", "sk-test-value")];

fn run_with(response: &str, arguments: &[&str]) -> std::io::Result<std::process::Output> {
    let listener = Listener::serving(vec![Canned::ok(response)])?;
    let fixed = ["--url", listener.base(), "--model", "local-1"];
    spawn(
        &[arguments, &["--csv", "--field", "/body"], &fixed].concat(),
        &KEY,
        b"body,id\nyes,1\n",
    )
}

#[test]
fn csv_dry_run_parses_the_first_data_row_as_an_object() {
    let output = spawn(
        &[
            "decide",
            "Does this report a payment failure?",
            "--csv",
            "--field",
            "/body",
            "--plan",
        ],
        &[],
        b"id,body\n7,The payout failed again.\n8,not read\n",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).expect("stdout is text");
    assert!(
        stdout.contains(r#""input":{"framing":"csv","field":["/body"]}"#),
        "{stdout}"
    );
    assert!(
        stdout.contains(r#""instructions":"The text is \"The payout failed again.\". Does"#),
        "{stdout}"
    );
}

#[test]
fn every_record_command_accepts_a_table_and_prints_jsonl() {
    let yes = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let choice = r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"a","confidence":0.9,"probabilities":{"a":0.9,"b":0.1}}}}"#;
    let tag = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}}}"#;
    let score = r#"{"model":"local-1","answers":{"q1":{"type":"score","score":0.8,"confidence":0.8,"legend":{"0":"low","1":"high"},"probabilities":{"0":0.2,"1":0.8}}}}"#;
    let cases: &[(&str, &[&str], &str)] = &[
        (
            yes,
            &["decide", "Is it yes?"],
            "{\"input\":{\"body\":\"yes\",\"id\":\"1\"},\"value\":true}\n",
        ),
        (
            choice,
            &["choose", "Which?", "a", "b"],
            "{\"input\":{\"body\":\"yes\",\"id\":\"1\"},\"value\":\"a\"}\n",
        ),
        (
            tag,
            &["tag", "Which?", "a", "b"],
            "{\"input\":{\"body\":\"yes\",\"id\":\"1\"},\"value\":[\"a\"]}\n",
        ),
        (
            score,
            &["score", "How much?", "low", "high"],
            "{\"input\":{\"body\":\"yes\",\"id\":\"1\"},\"value\":0.8}\n",
        ),
        (
            yes,
            &["filter", "Is it yes?"],
            "{\"body\":\"yes\",\"id\":\"1\"}\n",
        ),
        (
            yes,
            &["rank", "Is it yes?"],
            "{\"body\":\"yes\",\"id\":\"1\"}\n",
        ),
    ];
    for (response, arguments, expected) in cases {
        let output = run_with(response, arguments).expect("the command runs");
        assert_eq!(output.status.code(), Some(0), "{arguments:?}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            *expected,
            "{arguments:?}"
        );
    }

    let set = Path::new(env!("CARGO_TARGET_TMPDIR")).join("table-question-set.json");
    fs::write(
        &set,
        r#"{"version":1,"questions":{"ok":{"decide":"Is it yes?"}}}"#,
    )
    .expect("write question set");
    let set_name = set.to_string_lossy();
    let output = run_with(yes, &["annotate", &set_name]).expect("annotate runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"body\":\"yes\",\"id\":\"1\",\"ok\":true}\n"
    );
}

#[test]
fn tsv_details_carries_the_parsed_object() {
    let response = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::serving(vec![Canned::ok(response)]).expect("a listener");
    let output = spawn(
        &[
            "decide",
            "Is it yes?",
            "--tsv",
            "--field",
            "/body",
            "--details",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
        ],
        &KEY,
        b"body\tid\nyes\t1\n",
    )
    .expect("the command runs");
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains(r#""input":{"body":"yes","id":"1"}"#));
}

#[test]
fn table_rule_failures_are_exact_and_send_nothing() {
    let response = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let cases: &[(&[u8], &str)] = &[
        (
            b"",
            "thinkthen: the CSV header is missing because the input is empty\n",
        ),
        (
            b" ,body\n1,yes\n",
            "thinkthen: the CSV header has a blank name\n",
        ),
        (
            b"body,body\nyes,no\n",
            "thinkthen: the CSV header repeats a name\n",
        ),
        (
            b"body,id\nyes\n",
            concat!(
                "thinkthen: the CSV record has 1 field; its header has 2\n",
                "thinkthen: stopped at record 1; 0 records finished\n",
            ),
        ),
    ];
    for (input, expected) in cases {
        let listener = Listener::serving(vec![Canned::ok(response)]).expect("a listener");
        let output = spawn(
            &[
                "decide",
                "Is it yes?",
                "--csv",
                "--field",
                "/body",
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            &KEY,
            input,
        )
        .expect("the command runs");
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(String::from_utf8_lossy(&output.stderr), *expected);
        assert!(listener.requests().is_empty());
    }
}

fn refused_table(input: &[u8], code: i32, expected: &str) -> std::io::Result<()> {
    let listener = Listener::serving(vec![Canned::ok("unused")])?;
    let output = spawn(
        &[
            "decide",
            "Question",
            "--csv",
            "--field",
            "/body",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &KEY,
        input,
    )?;
    assert_eq!(output.status.code(), Some(code));
    assert_eq!(String::from_utf8_lossy(&output.stderr), expected);
    assert!(output.stdout.is_empty());
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn compiled_table_failures_pin_utf8_control_and_size() {
    refused_table(
        b"body,marker-hostile\xff\nyes,1\n",
        5,
        "thinkthen: the CSV header is not valid UTF-8\n",
    )
    .expect("header UTF-8 case runs");
    refused_table(
        b"body,id\nmarker-hostile\xff,1\n",
        5,
        concat!(
            "thinkthen: the CSV record is not valid UTF-8\n",
            "thinkthen: stopped at record 1; 0 records finished\n",
        ),
    )
    .expect("record UTF-8 case runs");
    refused_table(
        b"body,marker-hostile\x01\nyes,1\n",
        2,
        "thinkthen: the CSV header has a name containing a control character\n",
    )
    .expect("control case runs");

    let mut header = b"marker-hostile".to_vec();
    header.resize(crate::support::MAX_RECORD_BYTES + 1, b'h');
    refused_table(&header, 2, "thinkthen: the CSV header is over 16 MiB\n")
        .expect("header size case runs");

    let mut row = b"body\nmarker-hostile".to_vec();
    row.resize(b"body\n".len() + crate::support::MAX_RECORD_BYTES + 1, b'x');
    refused_table(
        &row,
        2,
        concat!(
            "thinkthen: the CSV record is over 16 MiB\n",
            "thinkthen: stopped at record 1; 0 records finished\n",
        ),
    )
    .expect("record size case runs");
}

#[test]
fn table_flags_are_explicit_and_mutually_exclusive() {
    for pair in [
        ["--csv", "--tsv"],
        ["--csv", "--jsonl"],
        ["--csv", "--lines"],
        ["--tsv", "--jsonl"],
        ["--tsv", "--lines"],
    ] {
        let output = spawn(
            &["decide", "Question", pair[0], pair[1], "--plan"],
            &[],
            b"a\nvalue\n",
        )
        .expect("the command runs");
        assert_eq!(output.status.code(), Some(2), "{pair:?}");
    }
}

#[test]
fn choose_raw_refuses_both_table_framings_before_any_request() {
    for framing in ["--csv", "--tsv"] {
        let listener = Listener::serving(vec![Canned::ok("unused")]).expect("a listener");
        let delimiter = if framing == "--csv" { ',' } else { '\t' };
        let input = format!("body{delimiter}id\nyes{delimiter}1\n");
        let output = spawn(
            &[
                "choose",
                "Which?",
                "a",
                "b",
                framing,
                "--field",
                "/body",
                "--raw",
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            &KEY,
            input.as_bytes(),
        )
        .expect("the command runs");
        assert_eq!(output.status.code(), Some(2), "{framing}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "thinkthen: --raw prints a bare label, but table results stay JSONL; omit --raw or use --lines or --jsonl\n"
        );
        assert!(listener.requests().is_empty(), "{framing}");
    }
}

#[test]
fn table_plan_refuses_a_malformed_later_data_row_before_disclosure() {
    let output = spawn(
        &["decide", "Question", "--csv", "--field", "/body", "--plan"],
        &[],
        b"body,id\nfirst,1\nmalformed\n",
    )
    .expect("the command runs");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        b"thinkthen: the CSV record has 1 field; its header has 2\n"
    );
}

#[test]
fn reverse_completion_keeps_table_order_and_a_bad_row_stops_the_tail() {
    let listener = Listener::answering(|body| {
        let body = String::from_utf8_lossy(body);
        let (probability, delay) = if body.contains("slow") {
            ("0.9", 40)
        } else if body.contains("middle") {
            ("0.1", 10)
        } else {
            ("0.9", 0)
        };
        Canned::ok(&format!(
            r#"{{"model":"local-1","answers":{{"q1":{{"type":"noul","noul":{probability}}}}}}}"#
        ))
        .after(delay)
    })
    .expect("a listener");
    let output = spawn(
        &[
            "decide",
            "Is it yes?",
            "--csv",
            "--field",
            "/body",
            "--jobs",
            "4",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &KEY,
        b"body,id\nslow,1\nmiddle,2\nfast,3\nbad\nafter,5\n",
    )
    .expect("the command runs");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"input":{"body":"slow","id":"1"},"value":true}"#,
            "\n",
            r#"{"input":{"body":"middle","id":"2"},"value":false}"#,
            "\n",
            r#"{"input":{"body":"fast","id":"3"},"value":true}"#,
            "\n",
        )
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    for expected in ["slow", "middle", "fast"] {
        assert!(
            requests
                .iter()
                .any(|request| String::from_utf8_lossy(&request.body).contains(expected))
        );
    }
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        concat!(
            "thinkthen: the CSV record has 1 field; its header has 2\n",
            "thinkthen: stopped at record 4; 3 records finished\n",
        )
    );
}

#[test]
fn one_job_sends_table_requests_in_record_order() {
    let response = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::serving(vec![
        Canned::ok(response),
        Canned::ok(response),
        Canned::ok(response),
    ])
    .expect("a listener");
    let output = spawn(
        &[
            "decide",
            "Question",
            "--csv",
            "--field",
            "/body",
            "--jobs",
            "1",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &KEY,
        b"body\nfirst\nsecond\nthird\n",
    )
    .expect("the command runs");
    assert_eq!(output.status.code(), Some(0));
    for (request, expected) in listener.requests().iter().zip(["first", "second", "third"]) {
        assert!(String::from_utf8_lossy(&request.body).contains(expected));
    }
}

#[test]
fn table_header_diagnostics_hide_evidence_and_key_markers() {
    let marker = "marker-evidence-51d2";
    let output = spawn(
        &["decide", "Question", "--csv", "--plan"],
        &[("THINKTHEN_API_KEY", "marker-key-19aa")],
        format!("{marker},{marker}\nleft,right\n").as_bytes(),
    )
    .expect("the command runs");
    let shown = [output.stdout, output.stderr].concat();
    let shown = String::from_utf8_lossy(&shown);
    assert!(!shown.contains(marker), "{shown}");
    assert!(!shown.contains("marker-key-19aa"), "{shown}");
}

#[test]
fn table_backend_failures_hide_cell_and_key_markers() {
    let evidence = "marker-cell-83ca";
    let key = "marker-key-402b";
    for response in [
        Canned::status(500, evidence),
        Canned::ok(&format!(r#"{{"model":"local-1","echo":"{evidence}"}}"#)),
    ] {
        let listener = Listener::serving(vec![response]).expect("a listener");
        let output = spawn(
            &[
                "decide",
                "Question",
                "--csv",
                "--field",
                "/body",
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            &[("THINKTHEN_API_KEY", key)],
            format!("body\n{evidence}\n").as_bytes(),
        )
        .expect("the command runs");
        assert_eq!(output.status.code(), Some(4));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains(evidence), "{stderr}");
        assert!(!stderr.contains(key), "{stderr}");
        assert!(!String::from_utf8_lossy(&output.stdout).contains(key));
    }
}

#[test]
fn a_closed_table_output_pipe_stops_reading_and_scheduling() {
    let response = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let listener =
        Listener::answering(move |_| Canned::ok(response).after(20)).expect("a listener");
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .clear_environment()
        .home(env!("CARGO_TARGET_TMPDIR"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .env("THINKTHEN_BATCH", "1")
        .args([
            "decide",
            "Question",
            "--csv",
            "--field",
            "/body",
            "--jobs",
            "4",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the command starts");
    let mut input = child.stdin.take().expect("input pipe");
    let rows = (0..24)
        .map(|place| format!("row {place}\n"))
        .collect::<String>();
    input
        .write_all(format!("body\n{rows}").as_bytes())
        .expect("input writes");
    drop(input);
    let mut reader = BufReader::new(child.stdout.take().expect("output pipe"));
    let mut first = String::new();
    reader.read_line(&mut first).expect("one output row");
    drop(reader);
    let output = finish(child, "table").expect("command ends");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(first, "{\"input\":{\"body\":\"row 0\"},\"value\":true}\n");
    assert!(listener.requests().len() <= 12);
}
