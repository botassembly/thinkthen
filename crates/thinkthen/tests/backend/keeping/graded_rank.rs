//! Saved score questions order command records without changing ordinary rank.

use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::{Value, json};

use crate::harness::{Canned, Listener, spawn_one as spawn};

const INPUT: &str = "delta\nbeta  \nalpha\ngamma\n";
const FILE: &str = r#"{"score":"How relevant?","levels":{"low":"Unrelated.","middle":"Some relevance.","high":"Perfect match."}}"#;
const BODY: &str = r#"{"state":"delta","model":"local-1","questions":{"q1":{"type":"score","instructions":"How relevant?","criteria":["Unrelated.","Some relevance.","Perfect match."]}}}"#;

fn question() -> io::Result<String> {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("graded-rank-question.json");
    fs::write(&path, FILE)?;
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
    for ranked in ranked_rows {
        let scored = scored_rows
            .iter()
            .find(|scored| scored["input"] == ranked["input"]);
        assert_eq!(scored.map(|row| &row["value"]), Some(&ranked["value"]));
        assert_eq!(
            scored.and_then(|row| row.pointer("/meta/requests")),
            ranked.pointer("/meta/requests")
        );
    }
    let requests = listener.requests();
    assert_eq!(requests.len(), 8);
    assert_eq!(first_requests.len(), 4);
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
    assert_eq!(first_requests.len(), 4);
    assert_eq!(first_requests[0].body, BODY.as_bytes());

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

    let plan = run("rank", &file, listener.base(), &["--dry-run"])?;
    assert_eq!(plan.status.code(), Some(0), "{:?}", plan.stderr);
    assert!(plan.stderr.is_empty());
    let value: Value = serde_json::from_slice(&plan.stdout)?;
    assert_eq!(value["request"], serde_json::from_str::<Value>(BODY)?);
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
