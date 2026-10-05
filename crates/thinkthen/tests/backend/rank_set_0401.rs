//! Independently declared set turns, attribution, intake and original identity.
use crate::harness::{Canned, Listener, spawn};
use crate::intake_0401::{folder, text};
use serde_json::{Value, json};
use std::{fs, io};

pub(super) const SET: &str =
    r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#;

pub(super) fn saved(name: &str, set: &str) -> io::Result<String> {
    let path = folder(name)?.join("questions.json");
    fs::write(&path, set)?;
    Ok(format!("@{}", path.display()))
}

pub(super) fn call(
    listener: &Listener,
    question: &str,
    flags: &[&str],
    input: &[u8],
) -> io::Result<std::process::Output> {
    call_at_width(listener, question, flags, input, Some("1"))
}

pub(super) fn call_at_width(
    listener: &Listener,
    question: &str,
    flags: &[&str],
    input: &[u8],
    jobs: Option<&str>,
) -> io::Result<std::process::Output> {
    let width: Vec<_> = jobs.into_iter().flat_map(|jobs| ["--jobs", jobs]).collect();
    let cache = if flags.contains(&"--cache") {
        vec![]
    } else {
        vec!["--no-cache"]
    };
    spawn(
        &[
            &[
                "rank",
                question,
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ][..],
            &width,
            &cache,
            flags,
        ]
        .concat(),
        &[],
        input,
    )
}

pub(super) fn answer(body: &[u8]) -> Canned {
    let body: Value = serde_json::from_slice(body).expect("request JSON");
    let answers = body["questions"]
        .as_object()
        .expect("questions")
        .iter()
        .map(|(key, question)| {
            let instructions = question["instructions"].as_str().expect("instructions");
            // First independently ranks [a,b,c]. Second ranks [a,c,b].
            // A duplicate-refill merge would incorrectly return [a,c,b].
            let probability = match (
                instructions.contains("Second?"),
                instructions.contains("\"a\""),
                instructions.contains("\"b\""),
            ) {
                (false, true, _) => 0.7,
                (false, _, true) => 0.6,
                (false, _, _) => 0.5,
                (true, true, _) => 1.0,
                (true, _, true) => 0.98,
                (true, _, _) => 0.99,
            };
            (
                key.clone(),
                json!({"type":"noul","noul":probability,"confidence":0.01}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(
        &json!({"model":"local-1","answers":answers,"usage":{"input_tokens":6,"output_tokens":2}})
            .to_string(),
    )
}

#[test]
fn duplicates_consume_visits_without_refill_and_top_follows_the_merge() -> io::Result<()> {
    let question = saved("rank-set-order", SET)?;
    for (top, expected) in [
        (None, "0.7 1:a\n0.6 2:b\n0.99 3:c\n"),
        (Some("1"), "0.7 1:a\n"),
        (Some("2"), "0.7 1:a\n0.6 2:b\n"),
        (Some("3"), "0.7 1:a\n0.6 2:b\n0.99 3:c\n"),
    ] {
        let listener = Listener::answering(answer)?;
        let mut flags = vec!["--batch", "1", "-n", "--scores"];
        if let Some(top) = top {
            flags.extend(["--top", top]);
        }
        let output = call(&listener, &question, &flags, b"a\nb\nc\n")?;
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        assert_eq!(output.stdout, expected.as_bytes());
        assert_eq!(
            listener.requests().len(),
            3,
            "all records are judged even at top one"
        );
    }
    Ok(())
}

#[test]
fn selecting_member_details_have_own_probability_digest_and_receipt() -> io::Result<()> {
    let question = saved("rank-set-details", SET)?;
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        &question,
        &["--batch", "1", "--details"],
        b"a\nb\nc\n",
    )?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let rows: Vec<Value> = text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("details"))
        .collect();
    assert_eq!(
        rows.iter()
            .map(|row| row["question_name"].as_str().expect("name"))
            .collect::<Vec<_>>(),
        ["first", "first", "second"]
    );
    assert_eq!(
        rows.iter()
            .map(|row| row["input"].as_str().expect("input"))
            .collect::<Vec<_>>(),
        ["a", "b", "c"]
    );
    for (row, yes) in rows.iter().zip([0.7, 0.6, 0.99]) {
        assert_eq!(row["question"]["verb"], "decide");
        assert_eq!(row["answer"]["kind"], "yes_no");
        assert_eq!(row["answer"]["probability"], yes);
        assert!(row["threshold"].is_null());
        assert!(row["value"].is_null());
        assert_eq!(row["meta"]["requests"].as_array().expect("keys").len(), 1);
    }
    assert_eq!(
        rows[0]["meta"]["question_sha256"],
        rows[1]["meta"]["question_sha256"]
    );
    assert_ne!(
        rows[0]["meta"]["question_sha256"],
        rows[2]["meta"]["question_sha256"]
    );
    Ok(())
}

#[test]
fn equal_text_positions_and_repeated_sources_survive_stable_member_ties() -> io::Result<()> {
    let question = saved("rank-set-identity", SET)?;
    let place = folder("rank-set-repeated")?;
    let file = place.join("input");
    fs::write(&file, "same\n\nsame\n")?;
    let file = file.to_str().expect("path");
    let listener = Listener::answering(crate::intake_0401::answer)?;
    let output = call(&listener, &question, &["-n", file, file], b"")?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(
        output.stdout,
        format!("{file}:1:same\n{file}:3:same\n{file}:1:same\n{file}:3:same\n").as_bytes()
    );
    let details = call(&listener, &question, &["--details", file, file], b"")?;
    let rows: Vec<Value> = text(&details.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("details"))
        .collect();
    assert_eq!(
        rows.iter()
            .map(|row| row["position"]["first"].as_u64())
            .collect::<Vec<_>>(),
        [Some(1), Some(3), Some(1), Some(3)]
    );
    assert!(rows.iter().all(|row| row["question_name"] == "first"));
    Ok(())
}

#[test]
fn common_jsonl_field_and_named_windows_use_shared_intake_display() -> io::Result<()> {
    let question = saved("rank-set-intake", SET)?;
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        &question,
        &["--jsonl", "--field", "/body", "-n", "--scores"],
        b"{\"body\":\"a\", \"private\":\"hidden\"}\n{\"body\":\"b\"}\n{\"body\":\"c\"}\n",
    )?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(output.stdout, b"0.7 1:{\"body\":\"a\", \"private\":\"hidden\"}\n0.6 2:{\"body\":\"b\"}\n0.99 3:{\"body\":\"c\"}\n");
    for request in listener.requests() {
        assert!(!text(&request.body).contains("hidden"));
    }
    let file = folder("rank-set-window")?.join("input");
    fs::write(&file, "a\n\nb\n\nc\n")?;
    let output = call(
        &listener,
        &question,
        &[
            "--window",
            "2",
            "-n",
            "--around",
            "0",
            file.to_str().expect("file"),
        ],
        b"",
    )?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(output.stdout, b"--\n1:a\n2:\n--\n3:b\n4:\n--\n5:c\n");
    Ok(())
}

#[test]
fn set_facts_count_n_originals_and_selected_details_do_not_sum_member_receipts() -> io::Result<()> {
    let question = saved("rank-set-facts", SET)?;
    let listener = Listener::answering(answer)?;
    let result = call(
        &listener,
        &question,
        &["--details", "--facts", "--batch", "1"],
        b"a\nb\nc\n",
    )?;
    assert_eq!(result.status.code(), Some(0), "{}", text(&result.stderr));
    let rows: Vec<Value> = text(&result.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("detail"))
        .collect();
    assert_eq!(
        rows.iter()
            .map(|row| row["meta"]["requests_sent"].as_u64())
            .collect::<Vec<_>>(),
        [Some(1), Some(1), Some(0)]
    );
    for row in rows {
        assert_eq!(
            row["meta"]["usage"],
            json!({"input_tokens":3,"output_tokens":1})
        );
    }
    let facts: Value = serde_json::from_slice(&result.stderr)?;
    assert_eq!(facts["records"], 3);
    assert_eq!(facts["requests_sent"], 3);
    assert_eq!(facts["input_tokens"], 18);
    assert_eq!(facts["output_tokens"], 6);
    Ok(())
}
