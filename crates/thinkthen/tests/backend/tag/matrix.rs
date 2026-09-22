//! The `tag` expansion boundary through the compiled command.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use super::file;
use crate::harness::{Canned, Listener, spawn};

fn answer(probabilities: &[f64]) -> String {
    let answers = probabilities
        .iter()
        .enumerate()
        .map(|(place, probability)| {
            format!(r#""q{}":{{"type":"noul","noul":{probability}}}"#, place + 1)
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(r#"{{"model":"local-1","answers":{{{answers}}}}}"#)
}

fn tag(base: &str, extra: &[&str], evidence: &[u8]) -> std::io::Result<std::process::Output> {
    let fixed = [
        "tag",
        "Which topics?",
        "billing",
        "urgent",
        "--url",
        base,
        "--model",
        "local-1",
    ];
    spawn(
        &[&fixed[..], extra].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        evidence,
    )
}

#[test]
fn one_and_twenty_labels_cross_the_compiled_expansion_boundary() {
    for count in [1, 20] {
        let probabilities = vec![0.9; count];
        let response = answer(&probabilities);
        let listener = Listener::serving(vec![Canned::ok(&response)]).expect("listener");
        let mut owned = vec!["tag".to_owned(), "Which topics?".to_owned()];
        owned.extend((1..=count).map(|place| format!("label{place}")));
        owned.extend([
            "--url".to_owned(),
            listener.base().to_owned(),
            "--model".to_owned(),
            "local-1".to_owned(),
        ]);
        let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
        let output = spawn(
            &borrowed,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            b"evidence",
        )
        .expect("tag runs");
        assert_eq!(output.status.code(), Some(0));
        let request = &listener.requests()[0].body;
        assert_eq!(
            String::from_utf8_lossy(request)
                .matches(r#""type":"noul""#)
                .count(),
            count
        );
        let expected = (1..=count)
            .map(|place| format!(r#""label{place}""#))
            .collect::<Vec<_>>()
            .join(",");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            format!("[{expected}]\n")
        );
    }
}

#[test]
fn a_quoted_backslashed_label_reaches_the_instruction_as_text() {
    let response = answer(&[0.9]);
    let listener = Listener::serving(vec![Canned::ok(&response)]).expect("listener");
    let output = spawn(
        &[
            "tag",
            "Which topics?",
            "say \"yes\"\\now",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"evidence",
    )
    .expect("tag runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&listener.requests()[0].body),
        r#"{"state":"evidence","model":"local-1","questions":{"q1":{"type":"noul","instructions":"Which topics?\n\nDetermine whether the label \"say \\\"yes\\\"\\\\now\" applies to this item."}}}"#
    );
}

#[test]
fn each_invalid_tag_reply_is_a_backend_failure() {
    let cases = [
        (
            r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
            "carries no answer for question `q2`",
        ),
        (
            r#"{"model":"local-1","answers":{"q1":{"type":"choice","probabilities":{}},"q2":{"type":"noul","noul":0.1}}}"#,
            "answer to question `q1` is not the shape",
        ),
        (
            r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":1e400},"q2":{"type":"noul","noul":0.1}}}"#,
            "response is not a systemone response",
        ),
        (
            r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":1.1},"q2":{"type":"noul","noul":0.1}}}"#,
            "question `q1` holds a probability outside zero to one",
        ),
    ];
    for (body, message) in cases {
        let listener = Listener::serving(vec![Canned::ok(body)]).expect("listener");
        let output = tag(listener.base(), &[], b"evidence").expect("tag runs");
        assert_eq!(output.status.code(), Some(4), "{message}");
        assert!(output.stdout.is_empty(), "{message}");
        assert!(String::from_utf8_lossy(&output.stderr).contains(message));
    }
}

#[test]
fn dry_run_prints_the_complete_expansion_without_a_key_or_connection() {
    let listener = Listener::serving(Vec::new()).expect("listener");
    let output = spawn(
        &[
            "tag",
            "Which topics?",
            "--label",
            "billing=Charges.",
            "--label",
            "urgent=Prompt.",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--dry-run",
        ],
        &[],
        b"evidence",
    )
    .expect("tag plans");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!(
            concat!(
                r#"{{"url":"{}/systemone","model":"local-1","key_env":"THINKTHEN_API_KEY","request":{{"state":"evidence","model":"local-1","questions":{{"q1":{{"type":"noul","instructions":"Which topics?\n\nDetermine whether the label \"billing\" applies to this item.","criteria":{{"true":"Charges."}}}},"#,
                r#""q2":{{"type":"noul","instructions":"Which topics?\n\nDetermine whether the label \"urgent\" applies to this item.","criteria":{{"true":"Prompt."}}}}}}}}}}"#,
                "\n",
            ),
            listener.base()
        )
    );
    assert!(listener.requests().is_empty());
}

#[test]
fn delayed_tag_rows_keep_evidence_and_output_order() {
    let listener = Listener::answering(|body| {
        let text = String::from_utf8_lossy(body);
        let place = (1..=6)
            .find(|place| text.contains(&format!("row {place}")))
            .unwrap_or(0);
        let probability = if place % 2 == 0 { 0.1 } else { 0.9 };
        Canned::ok(&answer(&[probability, 0.1])).after((7 - place) as u64 * 15)
    })
    .expect("listener");
    let input: String = (1..=6).map(|place| format!("row {place}\n")).collect();
    let output = tag(
        listener.base(),
        &["--lines", "--jobs", "4"],
        input.as_bytes(),
    )
    .expect("tag runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"input":"row 1","value":["billing"]}"#,
            "\n",
            r#"{"input":"row 2","value":[]}"#,
            "\n",
            r#"{"input":"row 3","value":["billing"]}"#,
            "\n",
            r#"{"input":"row 4","value":[]}"#,
            "\n",
            r#"{"input":"row 5","value":["billing"]}"#,
            "\n",
            r#"{"input":"row 6","value":[]}"#,
            "\n",
        )
    );
    let bodies: Vec<String> = listener
        .requests()
        .into_iter()
        .map(|request| String::from_utf8_lossy(&request.body).into_owned())
        .collect();
    for place in 1..=6 {
        assert!(
            bodies
                .iter()
                .any(|body| body.contains(&format!("row {place}")))
        );
    }
    assert!(listener.peak() > 1);
}

#[test]
fn earliest_tag_record_failure_wins_after_reverse_completion() {
    let listener = Listener::answering(|body| {
        if String::from_utf8_lossy(body).contains("record 1") {
            Canned::status(500, "{}").after(50)
        } else {
            Canned::status(422, "{}")
        }
    })
    .expect("listener");
    let output = tag(
        listener.base(),
        &["--lines", "--jobs", "2"],
        b"record 1\nrecord 2\n",
    )
    .expect("tag runs");
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    let said = String::from_utf8_lossy(&output.stderr);
    assert!(said.contains("status 500"), "{said}");
    assert!(said.contains("stopped at record 1"), "{said}");
    assert_eq!(listener.requests().len(), 4);
}

#[test]
fn a_closed_tag_output_pipe_stops_quietly() {
    let listener =
        Listener::answering(|_| Canned::ok(&answer(&[0.9])).after(20)).expect("listener");
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .args([
            "tag",
            "Which topics?",
            "billing",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--lines",
            "--jobs",
            "4",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("tag starts");
    let input: String = (1..=24).map(|place| format!("row {place}\n")).collect();
    child
        .stdin
        .take()
        .expect("input pipe")
        .write_all(input.as_bytes())
        .expect("records written");
    let mut output = BufReader::new(child.stdout.take().expect("output pipe"));
    let mut first = String::new();
    output.read_line(&mut first).expect("one row");
    drop(output);
    let finished = child.wait_with_output().expect("tag ends");
    assert_eq!(finished.status.code(), Some(0));
    assert_eq!(first, "{\"input\":\"row 1\",\"value\":[\"billing\"]}\n");
    assert!(finished.stderr.is_empty());
    assert!(listener.requests().len() <= 12);
}

#[test]
fn tag_label_diagnostics_name_labels_and_hide_hostile_content() {
    let marker = "marker-evidence\n\u{1b}[31m";
    let cli_cases = [
        (vec!["", "safe"], "a label is text, not white space"),
        (vec!["same", "same"], "a list holds each label once"),
        (
            vec![marker, "safe"],
            "a label is one line of printable text",
        ),
    ];
    for (labels, message) in cli_cases {
        let mut arguments = vec!["tag", "Which topics?"];
        arguments.extend(labels);
        arguments.push("--dry-run");
        let output = spawn(&arguments, &[], b"evidence").expect("tag refuses");
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!("thinkthen: {message}\n")
        );
        assert!(!String::from_utf8_lossy(&output.stderr).contains("marker-evidence"));
    }

    let files = [
        (
            r#"{"tag":"Which topics?","labels":["","safe"]}"#,
            "a label is text, not white space",
        ),
        (
            r#"{"tag":"Which topics?","labels":["same","same"]}"#,
            "a list holds each label once",
        ),
        (
            r#"{"tag":"Which topics?","labels":["marker-evidence\n\u001b[31m","safe"]}"#,
            "a label is one line of printable text",
        ),
    ];
    for (place, (text, message)) in files.into_iter().enumerate() {
        let path = file(&format!("diagnostic-{place}"), text);
        let output = spawn(
            &["tag", &format!("@{}", path.to_string_lossy()), "--dry-run"],
            &[],
            b"evidence",
        )
        .expect("tag refuses");
        assert_eq!(output.status.code(), Some(5));
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!("thinkthen: the question file's `labels`: {message}\n")
        );
        assert!(!String::from_utf8_lossy(&output.stderr).contains("marker-evidence"));
    }
}
