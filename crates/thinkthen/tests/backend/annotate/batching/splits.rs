//! Refused annotate requests and the two ordered half outcomes.

use super::{Canned, Listener, Value, json, set, spawn};
use std::fs;
use std::path::PathBuf;

#[test]
fn a_refused_batch_halves_once_and_keeps_request_order() {
    let file = set(
        "batch-halve",
        r#"{"version":1,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request JSON");
        let count = request["questions"]
            .as_object()
            .map_or(0, serde_json::Map::len);
        if count == 3 {
            Canned::status(413, "{}")
        } else {
            let answers = (1..=count)
                .map(|at| (format!("q{at}"), json!({"type":"noul","noul":0.9})))
                .collect::<serde_json::Map<_, _>>();
            Canned::ok(
                &json!({"model":"local-1","answers":answers,
                "usage":{"input_tokens":9,"output_tokens":3}})
                .to_string(),
            )
        }
    })
    .expect("listener");
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--lines",
            "--batch",
            "3",
            "--details",
            "--facts",
            "--no-cache",
            "--max-retries",
            "0",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"one\ntwo\nthree\n",
    )
    .expect("annotate stream");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let facts: Value = serde_json::from_slice(&output.stderr).expect("run facts");
    assert_eq!(facts["records"], 3);
    assert_eq!(facts["requests_sent"], 3);
    let sizes: Vec<usize> = requests
        .iter()
        .map(|request| {
            let body: Value = serde_json::from_slice(&request.body).expect("request JSON");
            body["questions"]
                .as_object()
                .map_or(0, serde_json::Map::len)
        })
        .collect();
    assert_eq!(sizes, [3, 2, 1]);
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).expect("row JSON"))
        .collect();
    assert_eq!(rows.len(), 3);
    // The first half counts the refused parent attempt.
    let sent: Vec<&Value> = rows
        .iter()
        .map(|row| &row["meta"]["requests_sent"])
        .collect();
    assert_eq!(sent, [&json!(1), &json!(1), &json!(1)]);
    for row in &rows {
        assert_eq!(row["meta"]["requests"].as_array().map(Vec::len), Some(1));
        assert!(row["meta"].get("batches").is_none(), "{row}");
        assert_eq!(row["value"]["ready"], true);
    }
}

#[test]
fn a_failed_second_half_prints_first_half_and_names_next_row() {
    let file = set(
        "batch-second-half-stop",
        r#"{"version":1,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request JSON");
        let count = request["questions"].as_object().map_or(0, serde_json::Map::len);
        match count {
            3 => Canned::status(413, "{}"),
            2 => Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#),
            _ => Canned::status(401, "{}"),
        }
    }).expect("listener");
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--lines",
            "--batch",
            "3",
            "--no-cache",
            "--max-retries",
            "0",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"one\ntwo\nthree\n",
    )
    .expect("annotate stream");
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(
        output.stdout.iter().filter(|&&byte| byte == b'\n').count(),
        2
    );
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(
        diagnostic.contains("stopped at record 3; the request for records 3 to 3 failed:")
            && diagnostic.contains("2 records finished"),
        "{diagnostic}"
    );
    assert_eq!(listener.requests().len(), 3);
}

#[test]
fn a_missing_whole_replay_finds_the_recorded_halves_without_sending() {
    let file = set(
        "batch-replay-halves",
        r#"{"version":1,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-batch-replay-halves");
    let _ = fs::remove_dir_all(&directory);
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request JSON");
        let count = request["questions"]
            .as_object()
            .map_or(0, serde_json::Map::len);
        if count == 3 {
            Canned::status(413, "{}")
        } else {
            let answers = (1..=count)
                .map(|at| (format!("q{at}"), json!({"type":"noul","noul":0.9})))
                .collect::<serde_json::Map<_, _>>();
            Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
        }
    })
    .expect("listener");
    let base = [
        "annotate",
        &file.to_string_lossy(),
        "--lines",
        "--batch",
        "3",
        "--details",
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    let input = b"one\ntwo\nthree\n";
    let recorded = spawn(
        &[&base[..], &["--record", &directory.to_string_lossy()]].concat(),
        &[("THINKTHEN_API_KEY", "key")],
        input,
    )
    .expect("recorded run");
    assert_eq!(
        recorded.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&recorded.stderr)
    );
    assert_eq!(listener.requests().len(), 3);
    let replayed = spawn(
        &[&base[..], &["--replay", &directory.to_string_lossy()]].concat(),
        &[],
        input,
    )
    .expect("replay run");
    assert_eq!(
        replayed.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&replayed.stderr)
    );
    assert_eq!(listener.requests().len(), 0);
    let rows = |output: &std::process::Output| {
        output
            .stdout
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice::<Value>(line).expect("row JSON"))
            .collect::<Vec<_>>()
    };
    let original = rows(&recorded);
    let replay = rows(&replayed);
    assert_eq!(original.len(), 3);
    assert_eq!(replay.len(), 3);
    for (before, after) in original.iter().zip(&replay) {
        assert_eq!(before["value"], after["value"]);
        assert_eq!(before["meta"]["requests"], after["meta"]["requests"]);
        assert_eq!(after["meta"]["cached"], true);
        assert_eq!(after["meta"]["requests_sent"], 0);
    }
}
