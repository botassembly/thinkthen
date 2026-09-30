//! Saved score questions order command records without changing ordinary rank.

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup or a missing field should stop the boundary test"
)]

use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::{Value, json};

use crate::harness::{Canned, Listener, spawn_one as spawn};

const INPUT: &str = "delta\nbeta  \nalpha\ngamma\n";
const FILE: &str = r#"{"score":"How relevant?","levels":{"low":"Unrelated.","middle":"Some relevance.","high":"Perfect match."}}"#;
const BODIES: [&str; 4] = [
    r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"score","instructions":"The text is \"delta\". How relevant?","criteria":["Unrelated.","Some relevance.","Perfect match."]}}}"#,
    r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"score","instructions":"The text is \"beta  \". How relevant?","criteria":["Unrelated.","Some relevance.","Perfect match."]}}}"#,
    r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"score","instructions":"The text is \"alpha\". How relevant?","criteria":["Unrelated.","Some relevance.","Perfect match."]}}}"#,
    r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"score","instructions":"The text is \"gamma\". How relevant?","criteria":["Unrelated.","Some relevance.","Perfect match."]}}}"#,
];
const QUESTION_SHA256: &str = "d20f78e3abbb54d4e8b083e27797cccfd25e673723eb952206f5d69035e221d6";

fn question() -> io::Result<String> {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let path = folder.join("graded-rank-question.json");
    // Each nextest process and each cargo test thread rewrites this name;
    // rename so no reader sees a half-written file.
    let staged = folder.join(format!(
        "graded-rank-question.json.{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    fs::write(&staged, FILE)?;
    fs::rename(&staged, &path)?;
    Ok(format!("@{}", path.display()))
}

fn response(body: &[u8]) -> Canned {
    let sent = String::from_utf8_lossy(body);
    let odds = if sent.contains("alpha") {
        [0.1, 0.3, 0.6]
    } else if sent.contains("beta") {
        [0.5, 0.0, 0.5]
    } else if sent.contains("gamma") {
        [0.0, 1.0, 0.0]
    } else {
        [0.8, 0.1, 0.1]
    };
    Canned::ok(
        &json!({"model":"local-1","answers":{"q1":{
            "type":"score","score":0.0,"confidence":0.9,
            "legend":{"0":"low","1":"middle","2":"high"},
            "probabilities":{"0":odds[0],"1":odds[1],"2":odds[2]}
        }}})
        .to_string(),
    )
}

fn run(verb: &str, file: &str, base: &str, extra: &[&str]) -> io::Result<std::process::Output> {
    let fixed = [
        verb,
        file,
        "--lines",
        "--batch",
        "1",
        "--jobs",
        "1",
        "--no-cache",
        "--url",
        base,
        "--model",
        "local-1",
    ];
    spawn(
        &[&fixed[..], extra].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        INPUT.as_bytes(),
    )
}

fn rows(output: &[u8]) -> io::Result<Vec<Value>> {
    output
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(serde_json::from_slice::<Value>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn assert_literal_bodies(requests: &[conformance_backend::Recorded]) {
    assert_eq!(requests.len(), BODIES.len());
    for (actual, expected) in requests.iter().zip(BODIES) {
        assert_eq!(actual.body, expected.as_bytes());
    }
}

fn assert_present_fixture_digest(row: &Value, body: &str, url: &str) {
    let digests = row.pointer("/meta/requests").and_then(Value::as_array);
    assert_eq!(digests.map(Vec::len), Some(1));
    let [expected] = crate::support::keys(url, body.as_bytes())
        .try_into()
        .expect("one question");
    assert_eq!(
        row.pointer("/meta/requests/0").and_then(Value::as_str),
        Some(expected.as_str())
    );
    assert_eq!(
        row.pointer("/meta/question_sha256").and_then(Value::as_str),
        Some(QUESTION_SHA256)
    );
}

fn compare_score_requests(
    file: &str,
    listener: &Listener,
    ranked_rows: &[Value],
    first_requests: &[conformance_backend::Recorded],
) -> io::Result<()> {
    let scored = run("score", file, listener.base(), &["--details"])?;
    assert_eq!(scored.status.code(), Some(0), "{:?}", scored.stderr);
    let scored_rows = rows(&scored.stdout)?;
    assert_eq!(scored_rows.len(), 4);
    for (ranked, (input, body)) in ranked_rows.iter().zip([
        ("alpha", BODIES[2]),
        ("beta  ", BODIES[1]),
        ("gamma", BODIES[3]),
        ("delta", BODIES[0]),
    ]) {
        assert_eq!(ranked["input"], input);
        assert_present_fixture_digest(ranked, body, listener.url());
        let scored = scored_rows
            .iter()
            .find(|scored| scored["input"] == ranked["input"]);
        assert_eq!(scored.map(|row| &row["value"]), Some(&ranked["value"]));
        if let Some(scored) = scored {
            assert_present_fixture_digest(scored, body, listener.url());
        }
        assert_eq!(
            scored.and_then(|row| row.pointer("/meta/requests")),
            ranked.pointer("/meta/requests")
        );
    }
    let requests = listener.requests();
    assert_eq!(requests.len(), 8);
    assert_literal_bodies(first_requests);
    for group in requests.chunks_exact(4) {
        assert_literal_bodies(group);
    }
    for (ranked, scored) in requests.iter().take(4).zip(requests.iter().skip(4)) {
        assert_eq!(ranked.body, scored.body);
    }
    for (first, later) in first_requests.iter().zip(requests.iter()) {
        assert_eq!(first.body, later.body);
    }
    Ok(())
}

#[test]
fn graded_rank_orders_weighted_positions_and_keeps_the_earlier_top_tie() -> io::Result<()> {
    let file = question()?;
    let listener = Listener::answering(response)?;
    let ranked = run("rank", &file, listener.base(), &["--top", "2", "--facts"])?;
    assert_eq!(
        ranked.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&ranked.stderr)
    );
    assert_eq!(ranked.stdout, b"alpha\nbeta  \n");
    let facts: Value = serde_json::from_slice(&ranked.stderr)?;
    assert_eq!(facts["records"], 4);
    assert_eq!(facts["requests_sent"], 4);
    let first_requests = listener.requests();
    assert_literal_bodies(&first_requests);

    let detail = run("rank", &file, listener.base(), &["--details"])?;
    assert_eq!(detail.status.code(), Some(0), "{:?}", detail.stderr);
    let rows = rows(&detail.stdout)?;
    assert_eq!(rows.len(), 4);
    assert_eq!(
        rows.iter()
            .map(|row| row["input"].as_str())
            .collect::<Vec<_>>(),
        [Some("alpha"), Some("beta  "), Some("gamma"), Some("delta")]
    );
    assert_eq!(
        rows.iter()
            .map(|row| row["value"].as_f64())
            .collect::<Vec<_>>(),
        [Some(1.5), Some(1.0), Some(1.0), Some(0.3)]
    );
    for row in &rows {
        assert_eq!(row["question"]["verb"], "score");
        assert_eq!(row["answer"]["kind"], "score");
        assert!(row["threshold"].is_null());
    }
    assert_ne!(rows[1]["answer"]["level"], rows[2]["answer"]["level"]);

    compare_score_requests(&file, &listener, &rows, &first_requests)?;
    Ok(())
}

#[test]
fn graded_rank_refuses_typed_meanings_and_dry_run_sends_nothing() -> io::Result<()> {
    let file = question()?;
    let listener = Listener::serving(Vec::new())?;
    for flag in ["--true", "--false"] {
        let refused = run("rank", &file, listener.base(), &[flag, "typed side"])?;
        assert_eq!(refused.status.code(), Some(2));
        assert!(refused.stdout.is_empty());
        assert_eq!(
            refused.stderr,
            b"thinkthen: `rank` with a `score` question file takes no --true or --false\n"
        );
        assert!(listener.requests().is_empty());
    }

    let plan = run("rank", &file, listener.base(), &["--plan"])?;
    assert_eq!(plan.status.code(), Some(0), "{:?}", plan.stderr);
    assert!(plan.stderr.is_empty());
    let value: Value = serde_json::from_slice(
        plan.stdout
            .split(|byte| *byte == b'\n')
            .next()
            .expect("plan line"),
    )?;
    assert_eq!(value["request"], serde_json::from_str::<Value>(BODIES[0])?);
    assert_eq!(value["from"]["question"], "file");
    assert_eq!(value["from"]["levels"], "file");
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn graded_rank_keeps_original_jsonl_bytes() -> io::Result<()> {
    let file = question()?;
    let listener = Listener::answering(response)?;
    let input = b"{ \"text\" : \"beta\" }\n{\"text\":\"alpha\"}\n";
    let output = spawn(
        &[
            "rank",
            &file,
            "--jsonl",
            "--batch",
            "1",
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
    assert_eq!(output.status.code(), Some(0), "{:?}", output.stderr);
    assert_eq!(
        output.stdout,
        b"{\"text\":\"alpha\"}\n{ \"text\" : \"beta\" }\n"
    );
    assert_eq!(listener.requests().len(), 2);
    Ok(())
}

#[test]
fn a_later_graded_failure_keeps_rank_stdout_empty() -> io::Result<()> {
    let file = question()?;
    let listener = Listener::serving(vec![response(b"delta"), Canned::status(500, "{}")])?;
    let output = run(
        "rank",
        &file,
        listener.base(),
        &["--top", "2", "--max-retries", "0"],
    )?;
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        concat!(
            "thinkthen: the backend answered with status 500: the backend failed after the allowed attempts; try again later or change --max-retries\n",
            "thinkthen: stopped at record 2; 1 record finished, and nothing was printed because an order needs every record\n",
        )
        .as_bytes()
    );
    assert_eq!(listener.requests().len(), 2);
    Ok(())
}
