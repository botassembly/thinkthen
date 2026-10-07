//! One-member wire parity and individual-member recording/cache reuse.
use super::rank_set::{SET, answer, call, saved};
use crate::harness::Listener;
use crate::input_sources::{folder, text};
use crate::support::stored;
use serde_json::Value;
use std::{fs, io};

#[test]
fn one_member_matches_plain_rank_bytes_requests_keys_and_detailed_answer() -> io::Result<()> {
    let set = saved(
        "rank-set-one",
        r#"{"version":1,"questions":{"first":{"decide":"First?"}}}"#,
    )?;
    let plain = saved("rank-set-plain", r#"{"decide":"First?"}"#)?;
    let listener = Listener::answering(answer)?;
    for view in [
        vec![],
        vec!["-n", "--scores", "--around", "1", "--top", "2"],
    ] {
        let ordinary = call(
            &listener,
            &plain,
            &[&["--batch", "1"][..], &view].concat(),
            b"a\nb\nc\n",
        )?;
        assert_eq!(ordinary.status.code(), Some(0));
        let bodies = listener.requests();
        let multiple = call(
            &listener,
            &set,
            &[&["--batch", "1"][..], &view].concat(),
            b"a\nb\nc\n",
        )?;
        assert_eq!(
            multiple.status.code(),
            Some(0),
            "{}",
            text(&multiple.stderr)
        );
        assert_eq!(ordinary.stdout, multiple.stdout);
        assert_eq!(
            listener
                .requests()
                .iter()
                .map(|request| &request.body)
                .collect::<Vec<_>>(),
            bodies
                .iter()
                .map(|request| &request.body)
                .collect::<Vec<_>>()
        );
    }
    Ok(())
}

#[test]
fn one_member_details_keep_primitive_answers_and_keys_with_distinct_set_reading() -> io::Result<()>
{
    let set = saved(
        "rank-set-detail-one",
        r#"{"version":1,"questions":{"first":{"decide":"First?"}}}"#,
    )?;
    let plain = saved("rank-set-detail-plain", r#"{"decide":"First?"}"#)?;
    let listener = Listener::answering(answer)?;
    let plain_record = crate::recordings::folder("rank-set-one-plain");
    let set_record = crate::recordings::folder("rank-set-one-set");
    let ordinary = call(
        &listener,
        &plain,
        &[
            "--batch",
            "1",
            "--details",
            "--record",
            plain_record.to_str().expect("path"),
        ],
        b"a\n",
    )?;
    let multiple = call(
        &listener,
        &set,
        &[
            "--batch",
            "1",
            "--details",
            "--record",
            set_record.to_str().expect("path"),
        ],
        b"a\n",
    )?;
    let plain: Value = serde_json::from_slice(&ordinary.stdout)?;
    let set: Value = serde_json::from_slice(&multiple.stdout)?;
    assert!(plain.get("question_name").is_none());
    assert_eq!(set["question_name"], "first");
    for field in [
        "question",
        "answer",
        "threshold",
        "value",
        "input",
        "position",
    ] {
        assert_eq!(plain[field], set[field]);
    }
    for field in ["requests", "requests_sent", "cached"] {
        assert_eq!(plain["meta"][field], set["meta"][field]);
    }
    assert_ne!(
        plain["meta"]["question_sha256"],
        set["meta"]["question_sha256"]
    );
    assert_ne!(plain["answer_id"], set["answer_id"]);
    assert_eq!(
        stored(&plain_record)?
            .iter()
            .map(|row| &row["key"])
            .collect::<Vec<_>>(),
        stored(&set_record)?
            .iter()
            .map(|row| &row["key"])
            .collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn individual_member_recordings_replay_sets_and_rename_reorder_without_sends() -> io::Result<()> {
    let record = crate::recordings::folder("rank-set-individual");
    let listener = Listener::answering(answer)?;
    for (question, bytes) in [
        ("First?", b"a\nb\nc\n".as_slice()),
        ("Second?", b"a\nc\nb\n".as_slice()),
    ] {
        let result = call(
            &listener,
            question,
            &["--batch", "1", "--record", record.to_str().expect("path")],
            b"a\nb\nc\n",
        )?;
        assert_eq!(result.status.code(), Some(0));
        assert_eq!(result.stdout, bytes);
    }
    assert_eq!(stored(&record)?.len(), 6);
    let count = listener.connections();
    assert_eq!(listener.requests().len(), 6);
    let set = saved("rank-set-replay", SET)?;
    for (question, view, expected) in [
        (set, vec!["--scores"], "0.7 a\n0.6 b\n0.99 c\n"),
        (
            saved(
                "rank-set-renamed",
                r#"{"version":1,"questions":{"renamed":{"decide":"First?"},"other":{"decide":"Second?"}}}"#,
            )?,
            vec!["--top", "2"],
            "a\nb\n",
        ),
        (
            saved(
                "rank-set-reversed",
                r#"{"version":1,"questions":{"second":{"decide":"Second?"},"first":{"decide":"First?"}}}"#,
            )?,
            vec!["-n", "--scores"],
            "1 1:a\n0.99 3:c\n0.6 2:b\n",
        ),
    ] {
        let result = call(
            &listener,
            &question,
            &[&["--replay", record.to_str().expect("record")][..], &view].concat(),
            b"a\nb\nc\n",
        )?;
        assert_eq!(result.status.code(), Some(0), "{}", text(&result.stderr));
        assert_eq!(result.stdout, expected.as_bytes());
        assert_eq!(listener.connections(), count);
    }
    let missing = saved(
        "rank-set-missing",
        r#"{"version":1,"questions":{"first":{"decide":"First?"},"missing":{"decide":"Absent?"}}}"#,
    )?;
    let result = call(
        &listener,
        &missing,
        &["--replay", record.to_str().expect("record")],
        b"a\nb\nc\n",
    )?;
    assert_eq!(result.status.code(), Some(5), "{}", text(&result.stderr));
    assert!(result.stdout.is_empty());
    assert_eq!(listener.connections(), count);
    Ok(())
}

#[test]
fn normal_cache_reuses_members_and_sends_only_missing_questions() -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    let cache = crate::recordings::folder("rank-set-cache");
    // Explicit cache overrides no-cache at the shared command edge.
    let set = saved("rank-set-cache", SET)?;
    let fixed = ["--cache", cache.to_str().expect("cache")];
    let first = call(&listener, &set, &fixed, b"a\nb\nc\n")?;
    assert_eq!(first.status.code(), Some(0), "{}", text(&first.stderr));
    let count = listener.connections();
    let second = call(
        &listener,
        &set,
        &[&fixed[..], &["--top", "1", "--scores"]].concat(),
        b"a\nb\nc\n",
    )?;
    assert_eq!(second.stdout, b"0.7 a\n");
    assert_eq!(listener.connections(), count);
    let extended = saved(
        "rank-set-cache-miss",
        r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"},"extra":{"decide":"Extra?"}}}"#,
    )?;
    listener.requests();
    let result = call(&listener, &extended, &fixed, b"a\nb\nc\n")?;
    assert_eq!(result.status.code(), Some(0), "{}", text(&result.stderr));
    let bodies = listener.requests();
    assert!(!bodies.is_empty());
    for request in bodies {
        let body = text(&request.body);
        assert!(body.contains("Extra?"));
        assert!(!body.contains("First?"));
        assert!(!body.contains("Second?"));
    }
    Ok(())
}

#[test]
fn mixed_structured_and_text_members_keep_individual_wire_identity_in_any_order() -> io::Result<()>
{
    let listener = Listener::answering(crate::input_sources::answer)?;
    let record = crate::recordings::folder("rank-set-mixed");
    let structured = saved(
        "rank-set-structured-single",
        r#"{"decide":{"check":"Ready?"}}"#,
    )?;
    for question in [&structured[..], "Ready?"] {
        let result = call(
            &listener,
            question,
            &["--batch", "1", "--record", record.to_str().expect("record")],
            b"alpha\nbeta\n",
        )?;
        assert_eq!(result.status.code(), Some(0), "{}", text(&result.stderr));
    }
    assert_eq!(stored(&record)?.len(), 4);
    let count = listener.connections();
    for (name, set) in [
        (
            "rank-set-mixed-forward",
            r#"{"version":1,"questions":{"text":{"decide":"Ready?"},"json":{"decide":{"check":"Ready?"}}}}"#,
        ),
        (
            "rank-set-mixed-reverse",
            r#"{"version":1,"questions":{"json":{"decide":{"check":"Ready?"}},"text":{"decide":"Ready?"}}}"#,
        ),
    ] {
        let question = saved(name, set)?;
        let result = call(
            &listener,
            &question,
            &["--details", "--replay", record.to_str().expect("record")],
            b"alpha\nbeta\n",
        )?;
        assert_eq!(result.status.code(), Some(0), "{}", text(&result.stderr));
        let rows: Vec<Value> = text(&result.stdout)
            .lines()
            .map(|line| serde_json::from_str(line).expect("detail"))
            .collect();
        let first = if name.ends_with("forward") {
            "text"
        } else {
            "json"
        };
        assert!(rows.iter().all(|row| row["question_name"] == first));
        assert_eq!(
            rows.iter()
                .map(|row| row["meta"]["batch_setting"].clone())
                .collect::<Vec<_>>(),
            if first == "text" {
                vec![serde_json::json!("max"); 2]
            } else {
                vec![serde_json::json!(1); 2]
            }
        );
        assert_eq!(listener.connections(), count);
    }
    Ok(())
}

#[test]
fn first_structured_member_does_not_change_selecting_text_members_batch_metadata() -> io::Result<()>
{
    let question = saved(
        "rank-set-mixed-attribution",
        r#"{"version":1,"questions":{"structured":{"decide":{"check":"Ready?"}},"text":{"decide":"Ready?"}}}"#,
    )?;
    let listener = Listener::answering(|body| {
        let body: Value = serde_json::from_slice(body).expect("request");
        let questions = body["questions"].as_object().expect("questions");
        let answers = questions
            .iter()
            .map(|(key, question)| {
                let structured = question["instructions"].is_object();
                let beta = if structured {
                    body["state"].as_str().expect("state").contains("beta")
                } else {
                    question["instructions"]
                        .as_str()
                        .expect("text")
                        .contains("beta")
                };
                let probability = match (structured, beta) {
                    (true, false) => 0.8,
                    (true, true) => 0.7,
                    (false, false) => 0.1,
                    (false, true) => 1.0,
                };
                (
                    key.clone(),
                    serde_json::json!({"type":"noul","noul":probability}),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        crate::harness::Canned::ok(
            &serde_json::json!({"model":"local-1","answers":answers}).to_string(),
        )
    })?;
    let result = call(&listener, &question, &["--details"], b"alpha\nbeta\n")?;
    assert_eq!(result.status.code(), Some(0), "{}", text(&result.stderr));
    let rows: Vec<Value> = text(&result.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("detail"))
        .collect();
    assert_eq!(
        rows.iter()
            .map(|row| (
                row["input"].clone(),
                row["question_name"].clone(),
                row["meta"]["batch_setting"].clone()
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                serde_json::json!("alpha"),
                serde_json::json!("structured"),
                serde_json::json!(1)
            ),
            (
                serde_json::json!("beta"),
                serde_json::json!("text"),
                serde_json::json!("max")
            )
        ]
    );
    Ok(())
}

#[test]
fn preview_packs_two_text_members_once_per_record_and_matches_runtime() -> io::Result<()> {
    super::rank_set::text_members()
}

#[test]
fn preview_keeps_mixed_member_quoting_and_runtime_packing_in_both_orders() -> io::Result<()> {
    super::rank_set::mixed_members()
}

#[test]
fn selecting_member_details_have_own_probability_with_complete_set_digest_and_receipts()
-> io::Result<()> {
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
    for (at, (row, yes)) in rows.iter().zip([0.7, 0.6, 0.99]).enumerate() {
        assert_eq!(row["question"]["verb"], "decide");
        assert_eq!(row["answer"]["kind"], "yes_no");
        assert_eq!(row["answer"]["probability"], yes);
        assert!(row["threshold"].is_null());
        assert_eq!(row["value"], at + 1);
        assert_eq!(row["meta"]["requests"].as_array().expect("keys").len(), 2);
    }
    assert_eq!(
        rows[0]["meta"]["question_sha256"],
        rows[1]["meta"]["question_sha256"]
    );
    assert_eq!(
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
    let listener = Listener::answering(crate::input_sources::answer)?;
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
