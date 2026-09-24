//! The compiled relate command against a counted loopback backend.

use std::{
    fs,
    io::Write as _,
    path::PathBuf,
    process::{Command, Output, Stdio},
};

use serde_json::Value;

use crate::harness::{Canned, Listener, spawn};

fn run(listener: &Listener, options: &[&str], input: &[u8]) -> Output {
    let mut arguments = vec![
        "relate",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--no-cache",
    ];
    arguments.extend_from_slice(options);
    spawn(&arguments, &[("THINKTHEN_API_KEY", "secret-value")], input).expect("command")
}

fn answered(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let answers = request["questions"]
        .as_object()
        .expect("questions")
        .iter()
        .map(|(name, question)| {
            let answer = if question["type"] == "noul" {
                serde_json::json!({"type":"noul","noul":0.9})
            } else {
                let labels = question["criteria"].as_object().expect("criteria");
                let pick = labels
                    .keys()
                    .find(|label| label.starts_with('i'))
                    .expect("entity option");
                let probabilities = labels
                    .keys()
                    .map(|label| (label.clone(), Value::from(f64::from(label == pick))))
                    .collect::<serde_json::Map<_, _>>();
                serde_json::json!({"type":"choice","choice":pick,"probabilities":probabilities})
            };
            (name.clone(), answer)
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(
        &serde_json::json!({"model":"local-1","answers":answers,"usage":{"input_tokens":10,"output_tokens":2}})
            .to_string(),
    )
}

#[test]
fn dry_run_reports_exact_requests_without_opening_a_connection() {
    let listener = Listener::answering(answered).expect("listener");
    let output = spawn(
        &[
            "relate",
            "works_for=person:organization",
            "--dry-run",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
        ],
        &[],
        br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#,
    )
    .expect("dry run");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).expect("plan JSON");
    assert_eq!(report["schema"], "thinkthen.relate-plan/1");
    assert_eq!(report["framing"], "document");
    assert_eq!(
        report["fields"],
        serde_json::json!({"name":"/name","kind":"/kind"})
    );
    assert_eq!(report["entity_count"], 2);
    assert_eq!(report["relations"][0]["method"], "choice");
    assert_eq!(report["logical_questions"], 1);
    assert_eq!(report["request_count"], 1);
    let body = report["requests"][0]["body_utf8"].as_str().expect("body");
    assert_eq!(report["requests"][0]["bytes"], body.len());
    assert!(body.contains(r#""entities":[{"id":"i1","name":"Ada","kind":"person"}"#));
    assert!(report.get("from").is_none());
    assert_eq!(listener.connections(), 0);
}

#[test]
fn choice_and_h_edges_keep_ruled_order_and_endpoint_shape() {
    let listener = Listener::answering(answered).expect("listener");
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"},{"name":"Beta","kind":"organization"}]"#;
    let output = run(
        &listener,
        &[
            "works_for=person:organization",
            "partners=organization:organization",
            "--either",
        ],
        input,
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let lines = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice::<Value>(line).expect("edge"))
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0]["relation"], "works_for");
    assert_eq!(
        lines[0]["source"],
        serde_json::json!({"name":"Ada","kind":"person"})
    );
    assert_eq!(lines[2]["relation"], "partners");
    assert_eq!(lines[2]["source"]["name"], "Acme");
    assert_eq!(lines[2]["target"]["name"], "Beta");
}

#[test]
fn mixed_logical_failure_prints_details_and_exits_six() {
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let names = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let mut answers = serde_json::Map::new();
        answers.insert(
            names[0].clone(),
            serde_json::json!({"type":"noul","noul":0.9}),
        );
        answers.insert(
            names[1].clone(),
            serde_json::json!({"type":"choice","probabilities":{"wrong":1.0}}),
        );
        Canned::ok(&serde_json::json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let output = run(
        &listener,
        &["follows=person:person", "--details"],
        br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"}]"#,
    );
    assert_eq!(
        output.status.code(),
        Some(6),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout).expect("details");
    assert_eq!(result["schema"], "thinkthen.result/1");
    assert_eq!(result["value"].as_array().expect("edges").len(), 1);
    assert_eq!(
        result["answer"]["questions"]
            .as_array()
            .expect("questions")
            .len(),
        2
    );
    assert_eq!(result["meta"]["failed_questions"], 1);
}

#[test]
fn successful_and_failed_choice_entries_keep_the_exact_option_a_shape() {
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let questions = request["questions"].as_object().expect("questions");
        let mut answers = serde_json::Map::new();
        for (place, (name, question)) in questions.iter().enumerate() {
            let answer = if place == 0 {
                let labels = question["criteria"].as_object().expect("criteria");
                let pick = labels
                    .keys()
                    .find(|label| label.starts_with('i'))
                    .expect("entity option");
                let probabilities = labels
                    .keys()
                    .map(|label| (label.clone(), Value::from(f64::from(label == pick))))
                    .collect::<serde_json::Map<_, _>>();
                serde_json::json!({"type":"choice","choice":pick,"probabilities":probabilities})
            } else {
                serde_json::json!({"type":"choice","probabilities":{"wrong":1.0}})
            };
            answers.insert(name.clone(), answer);
        }
        Canned::ok(&serde_json::json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let output = run(
        &listener,
        &["works_for=person:organization", "--details"],
        br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"},{"name":"Acme","kind":"organization"}]"#,
    );
    assert_eq!(output.status.code(), Some(6));
    let text = String::from_utf8(output.stdout).expect("details text");
    let result: Value = serde_json::from_str(&text).expect("details");
    let digest = result["meta"]["requests"][0].as_str().expect("digest");
    let successful = format!(
        r#"{{"relation":"works_for","reads":"works for","method":"choice","direction":"source_to_target","asker":{{"role":"source","entity":{{"name":"Ada","kind":"person"}}}},"candidates":[{{"role":"target","entity":{{"name":"Acme","kind":"organization"}},"probability":1,"accepted":true}},{{"none":true,"probability":0,"accepted":false}}],"pick":{{"role":"target","entity":{{"name":"Acme","kind":"organization"}}}},"request":"{digest}"}}"#
    );
    let failed = format!(
        r#"{{"relation":"works_for","reads":"works for","method":"choice","direction":"source_to_target","asker":{{"role":"source","entity":{{"name":"Grace","kind":"person"}}}},"candidates":[{{"role":"target","entity":{{"name":"Acme","kind":"organization"}}}},{{"none":true}}],"failure":{{"kind":"backend","cause":"missing_probability"}},"request":"{digest}"}}"#
    );
    assert!(text.contains(&successful), "{text}");
    assert!(text.contains(&failed), "{text}");
}

#[test]
fn no_valid_logical_answer_exits_four_without_output() {
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let answers = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .map(|name| {
                (
                    name.clone(),
                    serde_json::json!({"type":"choice","probabilities":{"wrong":1.0}}),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        Canned::ok(&serde_json::json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let output = run(
        &listener,
        &["follows=person:person", "--details"],
        br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"}]"#,
    );
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
}

#[test]
fn every_settled_empty_input_outcome_is_identical_in_normal_and_dry_runs() {
    let listener = Listener::answering(answered).expect("listener");
    let successful: &[(&[&str], &[u8])] = &[
        (&["linked", "--lines"], b""),
        (&["linked=person:person", "--jsonl"], b""),
        (&["linked=person:person", "--csv"], b"name,kind\n"),
        (&["linked=person:person", "--tsv"], b"name\tkind\n"),
    ];
    for (arguments, input) in successful {
        for dry in [false, true] {
            let mut options = arguments.to_vec();
            if dry {
                options.push("--dry-run");
            }
            let output = run(&listener, &options, input);
            assert_eq!(output.status.code(), Some(0), "{options:?}");
            assert!(output.stdout.is_empty(), "{options:?}");
        }
    }
    let refused: &[(&[&str], &[u8])] = &[
        (&["linked", "--lines"], b"\n"),
        (&["linked=person:person", "--jsonl"], b"\n"),
        (&["linked=person:person", "--csv"], b""),
        (&["linked=person:person", "--tsv"], b""),
        (&["linked=person:person"], b""),
    ];
    for (arguments, input) in refused {
        for dry in [false, true] {
            let mut options = arguments.to_vec();
            if dry {
                options.push("--dry-run");
            }
            let output = run(&listener, &options, input);
            assert_eq!(output.status.code(), Some(2), "{options:?}");
            assert!(output.stdout.is_empty(), "{options:?}");
        }
    }
    assert_eq!(listener.connections(), 0);
}

#[test]
fn complete_set_refusals_finish_before_any_send() {
    let listener = Listener::answering(answered).expect("listener");
    let duplicate = br#"[{"name":"Ada","kind":"person"},{"name":"Ada","kind":"person"}]"#;
    let absent = br#"[{"name":"Ada","kind":"person"}]"#;
    let wrong_type = br#"[{"name":7,"kind":"person"}]"#;
    for (options, input) in [
        (vec!["linked=person:person"], duplicate.as_slice()),
        (vec!["linked=person:organization"], absent.as_slice()),
        (vec!["linked=person:person"], wrong_type.as_slice()),
        (
            vec!["linked=person:person", "--field", "/missing"],
            absent.as_slice(),
        ),
    ] {
        let output = run(&listener, &options, input);
        assert_eq!(output.status.code(), Some(2), "{options:?}");
        assert!(output.stdout.is_empty());
    }
    let too_many = (0..256)
        .map(|place| format!("entity-{place}\n"))
        .collect::<String>();
    let output = run(&listener, &["linked", "--lines"], too_many.as_bytes());
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(listener.connections(), 0);
}

#[test]
fn relation_recording_replays_without_a_key_and_cache_reuses_the_same_identity() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-record-replay");
    let _removed = fs::remove_dir_all(&root);
    let listener = Listener::answering(answered).expect("listener");
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#;
    let recorded = spawn(
        &[
            "relate",
            "works_for=person:organization",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--record",
            &root.to_string_lossy(),
            "--no-cache",
            "--details",
        ],
        &[("THINKTHEN_API_KEY", "secret")],
        input,
    )
    .expect("record");
    assert_eq!(recorded.status.code(), Some(0));
    let _sent = listener.requests();
    let replayed = spawn(
        &[
            "relate",
            "works_for=person:organization",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--replay",
            &root.to_string_lossy(),
            "--no-cache",
            "--details",
        ],
        &[],
        input,
    )
    .expect("replay");
    assert_eq!(replayed.status.code(), Some(0));
    assert!(listener.requests().is_empty());
    let live: Value = serde_json::from_slice(&recorded.stdout).expect("live details");
    let result: Value = serde_json::from_slice(&replayed.stdout).expect("details");
    for key in ["value", "question", "answer"] {
        assert_eq!(live[key], result[key], "{key}");
    }
    assert_eq!(live["meta"]["requests"], result["meta"]["requests"]);
    assert_eq!(result["meta"]["cached"], true);
    assert_eq!(result["meta"]["requests_sent"], 0);
}

#[test]
fn a_closed_output_pipe_ends_the_aggregate_quietly() {
    let listener = Listener::answering(answered).expect("listener");
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("THINKTHEN_API_KEY", "secret")
        .args([
            "relate",
            "works_for=person:organization",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("child");
    child
        .stdin
        .take()
        .expect("input")
        .write_all(br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#)
        .expect("input write");
    drop(child.stdout.take().expect("output"));
    let output = child.wait_with_output().expect("completion");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
}
