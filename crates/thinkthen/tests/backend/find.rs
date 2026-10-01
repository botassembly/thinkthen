//! The aggregate find request and its dedicated mapped result.

use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::harness::{Canned, Listener, finish, spawn};

const PICKED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"u002","#,
    r#""confidence":0.98,"probabilities":{"u001":0.01,"u002":0.99}}},"#,
    r#""usage":{"input_tokens":42,"output_tokens":9}}"#,
);

const NONE_TIE: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"u001","#,
    r#""probabilities":{"u001":0.5,"u002":0.0,"none":0.5}}}}"#,
);

type Preflight = (Vec<&'static str>, Vec<u8>, i32);

#[test]
fn two_lines_make_one_exact_request_and_the_selected_original_line_returns() {
    let listener = Listener::serving(vec![Canned::ok(PICKED)]).expect("listener");
    let output = spawn(
        &[
            "find",
            "Which unit answers?",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"first line\nsecond line\n",
    )
    .expect("find runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "second line\n");
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body = String::from_utf8_lossy(&requests[0].body);
    assert!(body.contains(r#""state":"[{\"id\":\"u001\",\"evidence\":\"first line\"},{\"id\":\"u002\",\"evidence\":\"second line\"}]""#), "{body}");
    assert!(
        body.contains(r#""criteria":{"u001":null,"u002":null}"#),
        "{body}"
    );
}

#[test]
fn a_none_tie_is_unresolved_but_details_keep_the_first_wire_leader() {
    let listener = Listener::serving(vec![Canned::ok(NONE_TIE)]).expect("listener");
    let output = spawn(
        &[
            "find",
            "Which unit answers?",
            "--none",
            "--details",
            "--url",
            listener.base(),
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"first line\nsecond line\n",
    )
    .expect("find runs");
    assert_eq!(output.status.code(), Some(3));
    let row = String::from_utf8_lossy(&output.stdout);
    assert!(row.starts_with(concat!(r#"{"schema":"thinkthen.result/1","value":null,"question":{"verb":"find","text":"Which unit answers?","none":true},"answer":{"kind":"find","pick":"u001","probabilities":{"u001":0.5,"u002":0.0,"none":0.5}},"threshold":null,"meta":{"tool":"thinkthen "#, env!("CARGO_PKG_VERSION"), r#"","question_sha256":"#)), "{row}");
    assert!(
        row.contains(r#""requests_sent":1,"cached":false,"requests":[""#),
        "{row}"
    );
}

#[test]
fn jsonl_pointer_sends_only_evidence_and_returns_the_whole_original_record() {
    let listener = Listener::serving(vec![Canned::ok(PICKED)]).expect("listener");
    let input = br#"{"id":1,"body":"first line"}
{"id":2,"body":"second line"}
"#;
    let output = spawn(
        &[
            "find",
            "Which unit answers?",
            "--jsonl",
            "--field",
            "/body",
            "--url",
            listener.base(),
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input,
    )
    .expect("find runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"id\":2,\"body\":\"second line\"}\n"
    );
    let requests = listener.requests();
    let body = String::from_utf8_lossy(&requests.first().expect("request").body);
    assert!(!body.contains(r#"\"id\":1"#), "{body}");
    assert!(body.contains(r#"\"evidence\":\"first line\""#), "{body}");
}

#[test]
fn cache_records_once_and_then_replays_without_a_key_or_second_request() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("find-replay");
    let folder_text = folder.to_string_lossy();
    let _removed = fs::remove_dir_all(&folder);
    let listener = Listener::serving(vec![Canned::ok(PICKED)]).expect("listener");
    let arguments = [
        "find",
        "Which unit answers?",
        "--details",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &folder_text,
    ];
    let recorded = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"first line\nsecond line\n",
    )
    .expect("find records");
    let replayed = spawn(&arguments, &[], b"first line\nsecond line\n").expect("find replays");
    assert_eq!(recorded.status.code(), Some(0));
    assert_eq!(replayed.status.code(), Some(0));
    let first = String::from_utf8(recorded.stdout).expect("recorded result");
    let second = String::from_utf8(replayed.stdout).expect("replayed result");
    assert_eq!(
        first
            .replace(r#""requests_sent":1"#, r#""requests_sent":0"#)
            .replace(r#""cached":false"#, r#""cached":true"#),
        second
    );
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn backend_failure_repeats_neither_the_key_nor_the_response_body() {
    let marker = "PRIVATE-FIND-RESPONSE";
    let key = "PRIVATE-FIND-KEY";
    let listener = Listener::serving(vec![Canned::status(500, marker)]).expect("listener");
    let output = spawn(
        &["find", "Which unit answers?", "--url", listener.base()],
        &[("THINKTHEN_API_KEY", key)],
        b"first line\nsecond line\n",
    )
    .expect("find fails safely");
    assert_eq!(output.status.code(), Some(4));
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(!diagnostic.contains(marker), "{diagnostic}");
    assert!(!diagnostic.contains(key), "{diagnostic}");
}

#[test]
fn every_preflight_refusal_is_keyed_and_opens_no_connection() {
    let many = (0..256)
        .map(|place| format!("unit {place}\n"))
        .collect::<String>()
        .into_bytes();
    let oversized = vec![b'x'; crate::support::MAX_RECORD_BYTES + 1];
    let cases: Vec<Preflight> = vec![
        (vec![], b"one\n".to_vec(), 2),
        (vec![], many, 2),
        (vec![], oversized, 2),
        (
            vec!["--jsonl", "--field", "/missing"],
            b"{}\n{}\n".to_vec(),
            2,
        ),
        (vec!["--jsonl"], b"{}\nnot-json\n".to_vec(), 2),
        (vec!["--plan"], b"one\ntwo\n".to_vec(), 0),
        (vec![], Vec::new(), 0),
        (vec!["--csv"], b"a\nb\n".to_vec(), 2),
        (vec!["--tsv"], b"a\nb\n".to_vec(), 2),
        (vec!["--jobs", "2"], b"a\nb\n".to_vec(), 2),
    ];
    for (adds, input, code) in cases {
        let listener = Listener::serving(Vec::new()).expect("listener");
        let mut arguments = vec!["find", "Which?", "--url", listener.base()];
        arguments.extend(adds);
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            &input,
        )
        .expect("find preflight");
        assert_eq!(output.status.code(), Some(code), "{arguments:?}");
        assert!(listener.requests().is_empty(), "{arguments:?}");
    }
    for (question, adds, input) in [
        (" ", Vec::new(), b"one\ntwo\n".as_slice()),
        (
            "Which?",
            vec!["--jsonl", "--field", "not-a-pointer"],
            b"{}\n{}\n".as_slice(),
        ),
        (
            "Which?",
            vec!["--plan", "--record", "unused"],
            b"one\ntwo\n".as_slice(),
        ),
        (
            "Which?",
            vec!["--cache", "cache", "--record", "record"],
            b"one\ntwo\n".as_slice(),
        ),
    ] {
        let listener = Listener::serving(Vec::new()).expect("listener");
        let mut arguments = vec!["find", question, "--url", listener.base()];
        arguments.extend(adds);
        let output = spawn(&arguments, &[("THINKTHEN_API_KEY", "sk-test-value")], input)
            .expect("find preflight");
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(listener.requests().is_empty(), "{arguments:?}");
    }

    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("find-input-directory");
    let _removed = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("directory");
    let directory_text = directory.to_string_lossy();
    let listener = Listener::serving(Vec::new()).expect("listener");
    let output = spawn(
        &[
            "find",
            "Which?",
            "--url",
            listener.base(),
            "--input",
            &directory_text,
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"ignored",
    )
    .expect("directory preflight");
    assert_eq!(output.status.code(), Some(5));
    assert!(listener.requests().is_empty());
}

#[test]
fn line_output_normalizes_crlf_and_a_missing_final_ending_to_one_lf() {
    for input in [
        b"first\r\nsecond".as_slice(),
        b"first\r\nsecond\r\n".as_slice(),
    ] {
        let listener = Listener::serving(vec![Canned::ok(PICKED)]).expect("listener");
        let output = spawn(
            &["find", "Which unit answers?", "--url", listener.base()],
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            input,
        )
        .expect("find runs");
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(output.stdout, b"second\n");
    }

    let listener = Listener::serving(vec![Canned::ok(PICKED)]).expect("listener");
    let output = spawn(
        &[
            "find",
            "Which unit answers?",
            "--jsonl",
            "--field",
            "/body",
            "--url",
            listener.base(),
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"{\"id\":1, \"body\":\"first\"}\r\n{\"id\":2,  \"body\":\"second\"}",
    )
    .expect("JSONL find runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"{\"id\":2,  \"body\":\"second\"}\n");
}

#[test]
fn a_closed_output_pipe_ends_find_quietly_after_the_paid_answer_finishes() {
    let listener = Listener::serving(vec![Canned::ok(PICKED).after(30)]).expect("listener");
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .args([
            "find",
            "Which unit answers?",
            "--url",
            listener.base(),
            "--no-cache",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("find starts");
    child
        .stdin
        .take()
        .expect("input pipe")
        .write_all(b"first\nsecond\n")
        .expect("units written");
    drop(child.stdout.take().expect("output pipe"));
    let output = finish(child, "find with a closed output").expect("find stops");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert_eq!(listener.requests().len(), 1);
}
