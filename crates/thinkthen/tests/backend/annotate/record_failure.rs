//! An isolated missing `on` pointer leaves later annotate records judged.

use std::io;

use super::set;
use crate::harness::{Canned, Listener, spawn};
use crate::support::digest;
use serde_json::{Value, json};

const INPUT: &[u8] =
    b"{\"id\":\"a\",\"body\":\"first\"}\n{\"id\":\"b\"}\n{\"id\":\"c\",\"body\":\"third\"}\n";
const SET: &str =
    r#"{"version":1,"questions":{"urgent":{"decide":"Is this urgent?","on":"/body"}}}"#;
const YES: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":4,"output_tokens":1}}"#;
const FIRST_REQUEST: &str = r#"{"state":"first","model":"local-1","questions":{"q1":{"type":"noul","instructions":"Is this urgent?"}}}"#;
const THIRD_REQUEST: &str = r#"{"state":"third","model":"local-1","questions":{"q1":{"type":"noul","instructions":"Is this urgent?"}}}"#;
const SET_SHA: &str = "0b08fe6760bb1fdbe07f6631e37028a8ae0c1f145ac657705b2512c883cf5604";
const NO: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.1}},"usage":{"input_tokens":6,"output_tokens":2}}"#;

#[test]
fn pointer_miss_is_one_error_row_and_later_record_is_answered() -> io::Result<()> {
    let file = set("record-missing-pointer", SET);
    let listener = Listener::serving(vec![Canned::ok(YES), Canned::ok(NO)])?;
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--details",
            "--batch",
            "1",
            "--on-error",
            "continue",
            "--jobs",
            "1",
            "--max-retries",
            "0",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        INPUT,
    )?;
    assert_eq!(output.status.code(), Some(7));
    assert_eq!(output.stderr, b"thinkthen: 1 record skipped\n");
    assert!(
        !output
            .stdout
            .windows(b"sk-test-value".len())
            .any(|bytes| bytes == b"sk-test-value")
    );
    assert!(
        !output
            .stderr
            .windows(b"sk-test-value".len())
            .any(|bytes| bytes == b"sk-test-value")
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].body, FIRST_REQUEST.as_bytes());
    assert_eq!(requests[1].body, THIRD_REQUEST.as_bytes());
    let first = digest(listener.url(), FIRST_REQUEST.as_bytes());
    let third = digest(listener.url(), THIRD_REQUEST.as_bytes());
    let lines: Vec<&[u8]> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[1], br#"{"schema":"thinkthen.record-error/1","at":2,"failure":{"kind":"usage","cause":"missing_pointer","pointer":"/body"}}"#);
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(serde_json::from_slice)
        .collect::<Result<_, _>>()?;
    assert_eq!(
        rows,
        vec![
            json!({"schema":"thinkthen.result/1","input":{"id":"a","body":"first"},
            "value":{"urgent":true},"answers":{"urgent":{"value":true,
                "question":{"verb":"decide","text":"Is this urgent?"},
                "answer":{"kind":"yes_no","probability":0.9},"threshold":0.5,
                "request":first}},"meta":{"tool":"thinkthen 0.0.1",
                "questions_sha256":SET_SHA,"url":listener.url(),"model":"local-1",
                "usage":{"input_tokens":4,"output_tokens":1},"requests_sent":1,
                "cached":false,"requests":[first],"failed_questions":0}}),
            json!({"schema":"thinkthen.record-error/1","at":2,
            "failure":{"kind":"usage","cause":"missing_pointer","pointer":"/body"}}),
            json!({"schema":"thinkthen.result/1","input":{"id":"c","body":"third"},
            "value":{"urgent":false},"answers":{"urgent":{"value":false,
                "question":{"verb":"decide","text":"Is this urgent?"},
                "answer":{"kind":"yes_no","probability":0.1},"threshold":0.5,
                "request":third}},"meta":{"tool":"thinkthen 0.0.1",
                "questions_sha256":SET_SHA,"url":listener.url(),"model":"local-1",
                "usage":{"input_tokens":6,"output_tokens":2},"requests_sent":1,
                "cached":false,"requests":[third],"failed_questions":0}}),
        ]
    );
    Ok(())
}

#[test]
fn default_stops_at_the_pointer_and_a_later_backend_failure_stays_terminal() -> io::Result<()> {
    let file = set("record-stop-policy", SET);
    let default = Listener::serving(vec![Canned::ok(YES)])?;
    let stopped = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--details",
            "--batch",
            "1",
            "--jobs",
            "1",
            "--no-cache",
            "--url",
            default.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        INPUT,
    )?;
    assert_eq!(stopped.status.code(), Some(2));
    assert_eq!(
        stopped
            .stdout
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .count(),
        1
    );
    assert_eq!(stopped.stderr, b"thinkthen: the record holds nothing at `/body`\nthinkthen: stopped at record 2; 1 record finished\n");
    let requests = default.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].body, FIRST_REQUEST.as_bytes());

    let backend = Listener::serving(vec![Canned::ok(YES), Canned::status(401, "{}")])?;
    let terminal = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--details",
            "--batch",
            "1",
            "--on-error",
            "continue",
            "--jobs",
            "1",
            "--max-retries",
            "0",
            "--no-cache",
            "--url",
            backend.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        INPUT,
    )?;
    assert_eq!(terminal.status.code(), Some(4));
    let rows: Vec<Value> = terminal
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(serde_json::from_slice)
        .collect::<Result<_, _>>()?;
    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[1],
        json!({"schema":"thinkthen.record-error/1","at":2,
        "failure":{"kind":"usage","cause":"missing_pointer","pointer":"/body"}})
    );
    assert_eq!(terminal.stderr, b"thinkthen: 1 record skipped\nthinkthen: the backend answered with status 401: the key was refused\nthinkthen: stopped at record 3; 2 records finished\n");
    let requests = backend.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].body, FIRST_REQUEST.as_bytes());
    assert_eq!(requests[1].body, THIRD_REQUEST.as_bytes());
    Ok(())
}

#[test]
fn details_preserve_shadowed_input_while_bare_mode_refuses_it() -> io::Result<()> {
    let file = set("record-shadowed-name", SET);
    let input = br#"{"urgent":"original","body":"first"}"#;
    let listener = Listener::serving(vec![Canned::ok(YES)])?;
    let detailed = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input,
    )?;
    assert_eq!(
        detailed.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&detailed.stderr)
    );
    assert!(detailed.stderr.is_empty());
    let row: Value = serde_json::from_slice(&detailed.stdout)?;
    let request = digest(listener.url(), FIRST_REQUEST.as_bytes());
    assert_eq!(
        row,
        json!({"schema":"thinkthen.result/1",
        "input":{"urgent":"original","body":"first"},"value":{"urgent":true},
        "answers":{"urgent":{"value":true,
            "question":{"verb":"decide","text":"Is this urgent?"},
            "answer":{"kind":"yes_no","probability":0.9},
            "threshold":0.5,"request":request}},
        "meta":{"tool":"thinkthen 0.0.1","questions_sha256":SET_SHA,
            "url":listener.url(),"model":"local-1",
            "usage":{"input_tokens":4,"output_tokens":1},
            "requests_sent":1,"cached":false,"requests":[request],
            "failed_questions":0}})
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].body, FIRST_REQUEST.as_bytes());

    let bare = Listener::serving(vec![])?;
    let refused = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--no-cache",
            "--url",
            bare.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input,
    )?;
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    assert_eq!(refused.stderr, b"thinkthen: the record already holds `urgent`, so that question cannot be appended\nthinkthen: stopped at record 1; 0 records finished\n");
    assert!(bare.requests().is_empty());
    let planned = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--details",
            "--dry-run",
            "--url",
            bare.base(),
            "--model",
            "local-1",
        ],
        &[],
        input,
    )?;
    assert_eq!(planned.status.code(), Some(0));
    assert!(planned.stderr.is_empty());
    assert_eq!(bare.requests().len(), 0);
    Ok(())
}

#[test]
fn continue_refuses_unsupported_modes_before_any_request() -> io::Result<()> {
    let file = set("record-continue-modes", SET);
    let missing = file.with_extension("missing");
    assert!(!missing.exists());
    let listener = Listener::serving(vec![])?;
    let modes: [(&[&str], &str); 5] = [
        (
            &["--jsonl", "--details", "--on-error", "continue"],
            "--on-error continue needs --jsonl --details --batch 1 and cannot accompany --dry-run",
        ),
        (
            &["--jsonl", "--batch", "1", "--on-error", "continue"],
            "--on-error continue needs --jsonl --details --batch 1 and cannot accompany --dry-run",
        ),
        (
            &["--details", "--batch", "1", "--on-error", "continue"],
            "--on-error continue needs --jsonl --details --batch 1 and cannot accompany --dry-run",
        ),
        (
            &[
                "--jsonl",
                "--details",
                "--batch",
                "1",
                "--dry-run",
                "--on-error",
                "continue",
            ],
            "--on-error continue needs --jsonl --details --batch 1 and cannot accompany --dry-run",
        ),
        (
            &["--jsonl", "--details", "--batch", "1", "--on-error", "stop"],
            "--on-error takes continue",
        ),
    ];
    for (flags, message) in modes {
        let mut args = vec!["annotate", missing.to_str().expect("path")];
        args.extend_from_slice(flags);
        args.extend_from_slice(&["--url", listener.base(), "--model", "local-1"]);
        let output = spawn(&args, &[("THINKTHEN_API_KEY", "sk-test-value")], INPUT)?;
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!("thinkthen: {message}\n")
        );
    }
    let wrong_verb = spawn(
        &[
            "decide",
            "Is this urgent?",
            "--on-error",
            "continue",
            "--url",
            listener.base(),
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"first",
    )?;
    assert_eq!(wrong_verb.status.code(), Some(2));
    assert!(wrong_verb.stdout.is_empty());
    assert!(String::from_utf8_lossy(&wrong_verb.stderr).contains("--on-error"));
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn skipped_row_counts_as_finished_with_only_known_token_usage() -> io::Result<()> {
    let file = set("record-continue-facts", SET);
    let no_usage = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.1}}}"#;
    let listener = Listener::serving(vec![Canned::ok(YES), Canned::ok(no_usage)])?;
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--details",
            "--batch",
            "1",
            "--on-error",
            "continue",
            "--facts",
            "--jobs",
            "1",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        INPUT,
    )?;
    assert_eq!(output.status.code(), Some(7));
    let lines: Vec<&[u8]> = output
        .stderr
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], b"thinkthen: 1 record skipped");
    let mut facts: Value = serde_json::from_slice(lines[1])?;
    facts.as_object_mut().expect("run facts").remove("seconds");
    assert_eq!(
        facts,
        json!({"schema":"thinkthen.run/1","records":3,
        "requests_sent":2,"retries":0,"cache_answers":0,
        "model":"local-1"})
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].body, FIRST_REQUEST.as_bytes());
    assert_eq!(requests[1].body, THIRD_REQUEST.as_bytes());
    Ok(())
}

#[test]
fn skipped_row_outranks_a_completed_partial_answer_without_erasing_it() -> io::Result<()> {
    let file = set(
        "record-continue-partial",
        r#"{"version":1,"questions":{"first":{"decide":"First?","on":"/body"},"second":{"decide":"Second?","on":"/body"}}}"#,
    );
    let reply = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::serving(vec![Canned::ok(reply)])?;
    let input = b"{\"body\":\"first\"}\n{\"id\":\"miss\"}\n";
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--details",
            "--batch",
            "1",
            "--on-error",
            "continue",
            "--jobs",
            "1",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input,
    )?;
    assert_eq!(output.status.code(), Some(7));
    assert_eq!(output.stderr, b"thinkthen: 1 record skipped\n");
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(serde_json::from_slice)
        .collect::<Result<_, _>>()?;
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["value"]["first"], true);
    assert_eq!(
        rows[0]["value"]["second"],
        json!({"failed":{"kind":"backend","cause":"missing_answer"}})
    );
    assert_eq!(rows[0]["meta"]["failed_questions"], 1);
    assert_eq!(
        rows[1],
        json!({"schema":"thinkthen.record-error/1","at":2,
        "failure":{"kind":"usage","cause":"missing_pointer","pointer":"/body"}})
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].body, br#"{"state":"first","model":"local-1","questions":{"q1":{"type":"noul","instructions":"First?"},"q2":{"type":"noul","instructions":"Second?"}}}"#);
    Ok(())
}

#[test]
fn missing_first_and_last_positions_keep_the_middle_answer() -> io::Result<()> {
    let file = set("record-continue-edges", SET);
    let listener = Listener::serving(vec![Canned::ok(YES)])?;
    let input = b"{\"id\":\"a\"}\n{\"id\":\"b\",\"body\":\"first\"}\n{\"id\":\"c\"}\n";
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--details",
            "--batch",
            "1",
            "--on-error",
            "continue",
            "--jobs",
            "1",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input,
    )?;
    assert_eq!(output.status.code(), Some(7));
    assert_eq!(output.stderr, b"thinkthen: 2 records skipped\n");
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(serde_json::from_slice)
        .collect::<Result<_, _>>()?;
    assert_eq!(rows.len(), 3);
    let failure = |at| {
        json!({"schema":"thinkthen.record-error/1","at":at,
        "failure":{"kind":"usage","cause":"missing_pointer","pointer":"/body"}})
    };
    assert_eq!(rows[0], failure(1));
    assert_eq!(rows[1]["input"], json!({"id":"b","body":"first"}));
    assert_eq!(rows[1]["value"], json!({"urgent":true}));
    assert_eq!(rows[2], failure(3));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].body, FIRST_REQUEST.as_bytes());
    Ok(())
}
