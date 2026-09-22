//! Partial backend failures keep neighboring `annotate` answers.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::harness::{Canned, Listener, spawn};
use crate::support::digest;

fn questions(name: &str, body: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-partial");
    let _created = fs::create_dir_all(&folder);
    let path = folder.join(format!("{name}.json"));
    let _written = fs::write(&path, body);
    path
}

fn run(file: &Path, response: &str, details: bool) -> io::Result<(Listener, std::process::Output)> {
    let listener = Listener::serving(vec![Canned::ok(response)])?;
    let file = file.to_string_lossy();
    let mut arguments = vec![
        "annotate",
        file.as_ref(),
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    if details {
        arguments.push("--details");
    }
    let output = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"The invoice failed.",
    )?;
    Ok((listener, output))
}

#[test]
fn bare_and_detailed_rows_distinguish_failed_from_not_sure() -> io::Result<()> {
    let file = questions(
        "one-failed",
        r#"{"version":1,"questions":{"good":{"decide":"good?"},"failed":{"decide":"failed?"},"not_sure":{"decide":"not sure?","threshold":"0.1:0.9"}}}"#,
    );
    let response = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"choice","probabilities":{"x":1.0}},"#,
        r#""q3":{"type":"noul","noul":0.5}},"usage":{"input_tokens":9,"output_tokens":3}}"#,
    );

    let (_, bare) = run(&file, response, false)?;
    assert_eq!(bare.status.code(), Some(6));
    assert!(bare.stderr.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&bare.stdout),
        "{\"good\":true,\"failed\":{\"failed\":{\"kind\":\"backend\",\"cause\":\"wrong_kind\"}},\"not_sure\":null}\n"
    );

    let (listener, detailed) = run(&file, response, true)?;
    assert_eq!(detailed.status.code(), Some(6));
    assert!(detailed.stderr.is_empty());
    let row: serde_json::Value = serde_json::from_slice(&detailed.stdout)?;
    assert_eq!(row["value"]["not_sure"], serde_json::Value::Null);
    assert_eq!(row["meta"]["model"], "local-1");
    assert_eq!(row["meta"]["usage"]["input_tokens"], 9);
    assert_eq!(row["meta"]["usage"]["output_tokens"], 3);
    assert_eq!(row["meta"]["failed_questions"], 1);

    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let request = digest(listener.url(), &requests[0].body);
    let request = request.as_str();
    assert_eq!(row["meta"]["requests"], serde_json::json!([request]));
    for name in ["good", "failed", "not_sure"] {
        assert_eq!(row["answers"][name]["request"], request);
    }

    let failed = row["answers"]["failed"]
        .as_object()
        .ok_or_else(|| io::Error::other("the failed answer is not an object"))?;
    assert_eq!(
        failed.keys().map(String::as_str).collect::<Vec<_>>(),
        ["failure", "question", "request"]
    );
    assert_eq!(
        failed["question"],
        serde_json::json!({"verb": "decide", "text": "failed?"})
    );
    assert_eq!(
        failed["failure"],
        serde_json::json!({"kind": "backend", "cause": "wrong_kind"})
    );
    Ok(())
}

#[test]
fn one_bad_tag_member_fails_one_logical_question() -> io::Result<()> {
    let file = questions(
        "failed-tag",
        r#"{"version":1,"questions":{"good":{"decide":"good?"},"topics":{"tag":"topics?","labels":["billing","urgent"]}}}"#,
    );
    let response = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"noul","noul":0.8},"q3":{"type":"score","probabilities":{"0":1.0}}}}"#,
    );

    let (_, output) = run(&file, response, true)?;
    assert_eq!(output.status.code(), Some(6));
    let row = String::from_utf8_lossy(&output.stdout);
    assert!(row.contains(r#""failed_questions":1"#), "{row}");
    assert!(
        row.contains(r#""topics":{"question":{"verb":"tag","text":"topics?","labels":["billing","urgent"]},"failure":{"kind":"backend","cause":"wrong_kind"},"request":"#),
        "{row}"
    );
    Ok(())
}

#[test]
fn a_group_with_no_valid_answer_remains_a_backend_failure() -> io::Result<()> {
    let file = questions(
        "all-failed",
        r#"{"version":1,"questions":{"first":{"decide":"first?"},"second":{"decide":"second?"}}}"#,
    );
    let response = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"choice","probabilities":{"x":1.0}},"#,
        r#""q2":{"type":"choice","probabilities":{"x":1.0}}}}"#,
    );

    let (_, output) = run(&file, response, false)?;
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the reply was refused: the answer to question `q1` is not the shape the question asked for\n"
    );
    Ok(())
}

#[test]
fn a_record_stream_finishes_after_partial_replies_and_exits_six() -> io::Result<()> {
    let file = questions(
        "stream-partial",
        r#"{"version":1,"questions":{"good":{"decide":"good?"},"failed":{"decide":"failed?"}}}"#,
    );
    let response = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"choice","probabilities":{"x":1.0}}}}"#,
    );
    let listener = Listener::serving(vec![Canned::ok(response), Canned::ok(response)])?;
    let file = file.to_string_lossy();
    let output = spawn(
        &[
            "annotate",
            file.as_ref(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--lines",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"first\nsecond\n",
    )?;

    assert_eq!(output.status.code(), Some(6));
    assert!(output.stderr.is_empty());
    assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 2);
    assert_eq!(listener.requests().len(), 2);
    Ok(())
}

#[test]
fn a_later_whole_run_failure_keeps_its_code_and_stop_boundary() -> io::Result<()> {
    let file = questions(
        "partial-then-stop",
        r#"{"version":1,"questions":{"good":{"decide":"good?"},"failed":{"decide":"failed?"}}}"#,
    );
    let partial = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"choice","probabilities":{"x":1.0}}}}"#,
    );
    let listener = Listener::serving(vec![Canned::ok(partial), Canned::status(401, "{}")])?;
    let file = file.to_string_lossy();
    let output = spawn(
        &[
            "annotate",
            file.as_ref(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--lines",
            "--jobs",
            "1",
            "--max-retries",
            "0",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"first\nsecond\nthird\n",
    )?;

    assert_eq!(output.status.code(), Some(4));
    assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 1);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        concat!(
            "thinkthen: the backend answered with status 401: the key was refused\n",
            "thinkthen: stopped at record 2; 1 record finished\n",
        )
    );
    assert_eq!(listener.requests().len(), 2);
    Ok(())
}

#[test]
fn a_recorded_partial_reply_replays_with_the_same_result() -> io::Result<()> {
    let file = questions(
        "recorded-partial",
        r#"{"version":1,"questions":{"good":{"decide":"good?"},"failed":{"decide":"failed?"}}}"#,
    );
    let response = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"choice","probabilities":{"x":1.0}}}}"#,
    );
    let listener = Listener::serving(vec![Canned::ok(response)])?;
    let recording = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-partial-recording");
    let _absent = fs::remove_dir_all(&recording);
    let file = file.to_string_lossy();
    let common = [
        "annotate",
        file.as_ref(),
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--details",
    ];
    let mut record = common.to_vec();
    let recording = recording.to_string_lossy();
    record.extend(["--record", recording.as_ref()]);
    let first = spawn(
        &record,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"The invoice failed.",
    )?;
    assert_eq!(first.status.code(), Some(6));

    let mut replay = common.to_vec();
    replay.extend(["--replay", recording.as_ref()]);
    let second = spawn(&replay, &[], b"The invoice failed.")?;
    assert_eq!(second.status.code(), Some(6));
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(
        String::from_utf8_lossy(&first.stdout)
            .replace(r#""requests_sent":1"#, r#""requests_sent":0"#)
            .replace(r#""replayed":false"#, r#""replayed":true"#),
        String::from_utf8_lossy(&second.stdout)
    );
    Ok(())
}
