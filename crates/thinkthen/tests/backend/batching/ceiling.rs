//! The byte setting closes command batches at every address and warns at the built-in one.

use std::fs;

use super::{QUESTION, answering, decide, folder, places, text};
use crate::harness::{Listener, spawn};

type Case<'a> = (&'a str, Vec<&'a str>, Vec<(&'a str, &'a str)>, &'a [usize]);

fn sized() -> String {
    (1..=3)
        .map(|at| format!("{}line {at}\n", "a".repeat(39_994)))
        .collect()
}

fn profile(path: &str, name: &str, limit: usize) -> String {
    let file = format!("{path}/{name}.json");
    fs::write(
        &file,
        format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}","max_request_bytes":{limit}}}"#),
    )
    .expect("a local profile");
    file
}

#[test]
fn the_request_size_closes_batches_at_every_address() {
    let input = sized();
    let directory = folder("size-profiles");
    fs::create_dir_all(&directory).expect("profile folder");
    let wide = profile(&directory, "wide", 200_000);
    let small = profile(&directory, "small", 50_000);
    let cases: [Case<'_>; 6] = [
        ("default", vec![], vec![], &[2, 1]),
        ("flag", vec!["--max-request-bytes", "200000"], vec![], &[3]),
        (
            "variable",
            vec![],
            vec![("THINKTHEN_MAX_REQUEST_BYTES", "200000")],
            &[3],
        ),
        (
            "flag beats variable",
            vec!["--max-request-bytes", "50000"],
            vec![("THINKTHEN_MAX_REQUEST_BYTES", "200000")],
            &[1, 1, 1],
        ),
        (
            "profile does not raise",
            vec!["--profile", &wide],
            vec![],
            &[2, 1],
        ),
        (
            "profile lowers",
            vec!["--profile", &small, "--max-request-bytes", "200000"],
            vec![],
            &[1, 1, 1],
        ),
    ];
    for (name, extra, environment, expected) in cases {
        let listener = Listener::answering(answering).expect("a loopback listener");
        let extra = [&["--no-cache", "--jobs", "1"][..], extra.as_slice()].concat();
        let output = decide(listener.base(), &extra, &environment, &input);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{name}: {}",
            text(&output.stderr)
        );
        let requests = listener.requests();
        let sizes: Vec<usize> = requests
            .iter()
            .map(|request| places(&request.body).len())
            .collect();
        assert_eq!(sizes, expected, "{name}");
        let limit = if name == "flag" || name == "variable" {
            200_000
        } else if name == "profile lowers" || name == "flag beats variable" {
            50_000
        } else {
            96_000
        };
        assert!(
            requests.iter().all(|request| request.body.len() <= limit),
            "{name}"
        );
    }
}

#[test]
fn the_request_size_refuses_bad_values_only_where_it_acts() {
    let input = b"line 1\nline 2\n";
    for value in ["0", "-1", "1.5", "lots", ""] {
        let output = spawn(
            &[
                "decide",
                QUESTION,
                "--lines",
                "--plan",
                "--max-request-bytes",
                value,
            ],
            &[],
            input,
        )
        .expect("command");
        assert_eq!(output.status.code(), Some(2), "{value:?}");
        assert_eq!(
            text(&output.stderr),
            "thinkthen: --max-request-bytes takes a whole number of at least 1\n",
            "{value:?}"
        );
    }
    for value in ["0", "-1", "1.5", "lots", ""] {
        let output = spawn(
            &["decide", QUESTION, "--lines", "--plan"],
            &[("THINKTHEN_MAX_REQUEST_BYTES", value)],
            input,
        )
        .expect("command");
        assert_eq!(output.status.code(), Some(2), "{value:?}");
        assert_eq!(
            text(&output.stderr),
            "thinkthen: THINKTHEN_MAX_REQUEST_BYTES takes a whole number of at least 1\n",
            "{value:?}"
        );
    }
    let one = spawn(
        &["decide", QUESTION, "--plan", "--max-request-bytes", "1"],
        &[],
        b"line 1",
    )
    .expect("one document");
    assert_eq!(one.status.code(), Some(0), "{}", text(&one.stderr));
    let choose = spawn(
        &["choose", "Which?", "a", "b", "--plan"],
        &[("THINKTHEN_MAX_REQUEST_BYTES", "0")],
        b"line 1",
    )
    .expect("unrelated verb");
    assert_eq!(choose.status.code(), Some(0), "{}", text(&choose.stderr));
}

#[test]
fn a_plan_counts_a_closed_batch_and_the_later_singleton_without_sending() {
    // These are the two wire bodies for the input below. The final one uses
    // the established singleton form; it does not quote a one-member batch.
    const FIRST: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". asks for a refund"},"q2":{"type":"noul","instructions":"The text is \"beta\". asks for a refund"}}}"#;
    const SECOND: &str = r#"{"state":"gamma","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}"#;
    assert_eq!((FIRST.len(), SECOND.len()), (248, 108));
    let listener = Listener::answering(answering).expect("loopback listener");
    let output = spawn(
        &[
            "decide",
            "asks for a refund",
            "--lines",
            "--batch",
            "2",
            "--plan",
            "--url",
            listener.base(),
            "--no-cache",
        ],
        &[],
        b"alpha\nbeta\ngamma\n",
    )
    .expect("plan command");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let lines = text(&output.stdout);
    let mut lines = lines.lines();
    let first = lines.next().expect("first disclosed body");
    assert!(first.contains(&format!("\"request\":{FIRST}")), "{first}");
    assert_eq!(
        lines.next(),
        Some(
            r#"{"records":3,"requests":2,"estimated_bytes":356,"estimated_input_tokens":{"lower":183,"upper":324},"upper_bound":false}"#
        )
    );
    assert_eq!(lines.next(), None);
    assert!(listener.requests().is_empty(), "preview sent a request");
}

#[test]
fn a_later_ordinary_command_request_is_refused_by_the_process_cap() {
    let listener = Listener::answering(answering).expect("listener");
    let output = decide(
        listener.base(),
        &["--batch", "1", "--no-cache", "--max-requests-total", "1"],
        &[],
        "line 1\nline 2\n",
    );
    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    assert_eq!(
        text(&output.stderr),
        "thinkthen: the process send budget was spent before another request\nthinkthen: stopped at record 2; 1 record finished\n"
    );
    assert_eq!(
        listener.count(),
        1,
        "the later ordinary request was not sent"
    );
    assert_eq!(places(&listener.requests()[0].body).len(), 1);
}

#[test]
fn plan_refuses_an_invalid_later_record_before_disclosing_any_body() {
    let output = spawn(
        &[
            "decide",
            "asks for a refund",
            "--jsonl",
            "--field",
            "/body",
            "--plan",
            "--url",
            "http://127.0.0.1:1/v1",
        ],
        &[],
        b"{\"body\":\"alpha\"}\n{\"body\":\n",
    )
    .expect("compiled plan");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"thinkthen: the record is not valid JSON\n");
}

#[test]
fn a_raised_size_warns_only_at_the_built_in_address() {
    let warn = "thinkthen: warning: max_request_bytes 200000 is above the default of 96000; the built-in backend refuses a request over 65536 input tokens\n";
    let cases = [
        ("https://api.typesafe.ai/v1", "200000", warn),
        ("https://api.typesafe.ai/v1/", "200000", warn),
        ("https://api.typesafe.ai/v1", "96000", ""),
        ("http://127.0.0.1:9/v1", "200000", ""),
    ];
    for (base, size, expected) in cases {
        let output = spawn(
            &[
                "decide",
                QUESTION,
                "--plan",
                "--url",
                base,
                "--max-request-bytes",
                size,
            ],
            &[],
            b"line 1",
        )
        .expect("dry run");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{base}: {}",
            text(&output.stderr)
        );
        assert_eq!(text(&output.stderr), expected, "{base}");
    }
}
