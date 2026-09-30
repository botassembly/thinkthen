//! Partial backend failures keep neighboring `annotate` answers.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::harness::{Canned, Listener, spawn};
use crate::support::keys;

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
    let asked = keys(listener.url(), &requests[0].body);
    assert_eq!(row["meta"]["requests"], serde_json::json!(asked));
    for (name, key) in ["good", "failed", "not_sure"].into_iter().zip(&asked) {
        assert_eq!(&row["answers"][name]["request"], key);
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

/// A missing member, or a `null` distribution, fails only the question it
/// belongs to, so the neighbors stand and the run exits 6.
#[test]
fn a_missing_member_fails_only_its_question() -> io::Result<()> {
    let file = questions(
        "missing-member",
        r#"{"version":1,"questions":{"d":{"decide":"d?"},"c":{"choose":"c?","options":["a","b"]},"s":{"score":"s?","levels":["low","high"]},"t":{"tag":"t?","labels":["x","y"]}}}"#,
    );
    let base = [
        r#"{"type":"noul","noul":0.9}"#,
        r#"{"type":"choice","probabilities":{"a":0.8,"b":0.2}}"#,
        r#"{"type":"score","probabilities":{"0":0.3,"1":0.7}}"#,
        r#"{"type":"noul","noul":0.8}"#,
        r#"{"type":"noul","noul":0.2}"#,
    ];
    let cases = [
        ("d", 0, r#"{"type":"noul"}"#),
        ("c", 1, r#"{"type":"choice"}"#),
        ("s", 2, r#"{"type":"score"}"#),
        ("t", 4, r#"{"type":"noul"}"#),
        ("c", 1, r#"{"type":"choice","probabilities":null}"#),
    ];
    for (failed, place, broken) in cases {
        let mut answers = base;
        answers[place] = broken;
        let answers = answers
            .iter()
            .enumerate()
            .map(|(at, answer)| format!(r#""q{}":{answer}"#, at + 1));
        let response = format!(
            r#"{{"model":"local-1","answers":{{{}}}}}"#,
            answers.collect::<Vec<_>>().join(",")
        );
        let (_, output) = run(&file, &response, true)?;
        assert_eq!(output.status.code(), Some(6), "{broken}");
        let row: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(row["meta"]["failed_questions"], 1, "{broken}");
        for name in ["d", "c", "s", "t"] {
            let failure = &row["answers"][name]["failure"];
            if name == failed {
                assert_eq!(
                    failure,
                    &serde_json::json!({"kind": "backend", "cause": "missing_probability"})
                );
            } else {
                assert!(failure.is_null(), "{name} {broken}");
            }
        }
    }
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
            "--batch",
            "1",
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
            "--batch",
            "1",
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
            "thinkthen: stopped at record 2; the request for records 2 to 2 failed: ",
            "the backend answered with status 401: the key was refused; 1 record finished\n",
        )
    );
    assert_eq!(listener.requests().len(), 2);
    Ok(())
}

#[test]
fn a_recorded_partial_reply_replays_its_failed_question_as_a_miss() -> io::Result<()> {
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
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    let failed = keys(listener.url(), &sent[0].body).remove(1);

    let mut replay = common.to_vec();
    replay.extend(["--replay", recording.as_ref()]);
    // Failed answers are never stored, so the replay misses that question.
    let second = spawn(&replay, &[], b"The invoice failed.")?;
    assert!(listener.requests().is_empty());
    assert!(second.stdout.is_empty());
    assert_eq!(second.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&second.stderr),
        format!(
            "thinkthen: the annotate request: the replay folder holds no answer for question `{failed}`; the key is the SHA-256 of the adapter, address, model, shared state and question as sent\n"
        )
    );
    Ok(())
}
