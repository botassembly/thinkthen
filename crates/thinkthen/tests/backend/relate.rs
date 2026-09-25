//! The compiled relate command against a counted loopback backend.

use std::{
    fs,
    io::Write as _,
    path::PathBuf,
    process::{Command, Output, Stdio},
};

use serde_json::Value;

use crate::harness::{Canned, Listener, finish, spawn};

mod details;

pub(super) fn run(listener: &Listener, options: &[&str], input: &[u8]) -> Output {
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

pub(super) fn answered(body: &[u8]) -> Canned {
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

/// A choice answer the backend gives for a label no option has.
pub(crate) const WRONG: &str = r#"{"type":"choice","probabilities":{"wrong":1.0}}"#;

/// A backend that answers question N of each request with `answers[N]`,
/// repeating the last answer for every later question.
pub(crate) fn scripted(answers: &'static [&'static str]) -> Listener {
    Listener::answering(move |body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let replies = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .enumerate()
            .map(|(place, name)| {
                let answer = answers.get(place).or(answers.last()).expect("an answer");
                (
                    name.clone(),
                    serde_json::from_str::<Value>(answer).expect("answer"),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        Canned::ok(&serde_json::json!({"model":"local-1","answers":replies}).to_string())
    })
    .expect("listener")
}

#[test]
fn dry_run_reports_the_exact_plan_and_the_digest_a_real_run_sends() {
    let listener = Listener::answering(answered).expect("listener");
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#;
    let output = run(
        &listener,
        &["works_for=person:organization", "--dry-run"],
        input,
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.connections(), 0);
    let sent = run(
        &listener,
        &["works_for=person:organization", "--details"],
        input,
    );
    assert_eq!(
        sent.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&sent.stderr)
    );
    let sent: Value = serde_json::from_slice(&sent.stdout).expect("details");
    let digest = sent["meta"]["requests"][0].as_str().expect("sent digest");
    let body = concat!(
        r#"{\"state\":{\"entities\":[{\"id\":\"i1\",\"name\":\"Ada\",\"kind\":\"person\"},"#,
        r#"{\"id\":\"i2\",\"name\":\"Acme\",\"kind\":\"organization\"}],"#,
        r#"\"relation\":{\"name\":\"works_for\",\"source\":\"person\",\"target\":\"organization\","#,
        r#"\"reads\":\"works for\",\"either\":false}},\"model\":\"local-1\","#,
        r#"\"questions\":{\"q1\":{\"type\":\"choice\",\"instructions\":\"Which listed organization "#,
        r#"fills the blank: Item 1 (person \\\"Ada\\\") works for ___? Choose none if no listed "#,
        r#"organization does.\",\"criteria\":{\"i2\":\"Item 2 (organization \\\"Acme\\\")\","#,
        r#"\"none\":\"No listed organization.\"}}}}"#,
    );
    let expected = format!(
        concat!(
            r#"{{"schema":"thinkthen.relate-plan/1","url":"{}/systemone","model":"local-1","#,
            r#""key_env":"THINKTHEN_API_KEY","backend_profile":null,"framing":"document","#,
            r#""fields":{{"name":"/name","kind":"/kind"}},"entity_count":2,"relations":[{{"#,
            r#""name":"works_for","source":"person","target":"organization","reads":"works for","#,
            r#""either":false,"method":"choice","fallback":null,"logical_questions":1,"#,
            r#""request_count":1}}],"logical_questions":1,"request_count":1,"requests":[{{"#,
            r#""digest":"{}","bytes":504,"body_utf8":"{}"}}]}}"#,
            "\n"
        ),
        listener.base(),
        digest,
        body
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
}

#[test]
fn a_wildcard_rule_expands_to_concrete_kinds_in_first_seen_order() {
    let listener = Listener::answering(answered).expect("listener");
    let output = run(
        &listener,
        &["linked=*:organization", "--dry-run"],
        br#"[{"name":"Acme","kind":"organization"},{"name":"Ada","kind":"person"},{"name":"Beta","kind":"organization"}]"#,
    );
    assert_eq!(output.status.code(), Some(0));
    let plan: Value = serde_json::from_slice(&output.stdout).expect("plan");
    let kinds = plan["relations"]
        .as_array()
        .expect("relations")
        .iter()
        .map(|relation| {
            (
                relation["source"].clone(),
                relation["target"].clone(),
                relation["method"].clone(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        kinds,
        [
            (
                "organization".into(),
                "organization".into(),
                "yes_no".into()
            ),
            ("person".into(), "organization".into(), "choice".into()),
        ]
    );
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
fn relation_recording_replays_without_a_key_and_keeps_the_same_identity() {
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
    let output = finish(child, "relate with a closed output").expect("completion");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
}

#[test]
fn a_question_file_beside_inline_rules_is_refused_before_any_send() {
    let listener = Listener::answering(answered).expect("listener");
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#;
    for rules in [["calls", "@q.json"], ["@q.json", "calls"]] {
        for dry in [false, true] {
            let mut options = rules.to_vec();
            if dry {
                options.push("--dry-run");
            }
            let output = run(&listener, &options, input);
            assert_eq!(output.status.code(), Some(2), "{options:?}");
            assert!(output.stdout.is_empty(), "{options:?}");
            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                "thinkthen: relate takes inline relation rules or one @FILE, never both\n",
                "{options:?}"
            );
        }
    }
    assert_eq!(listener.connections(), 0);
}

#[test]
fn a_backend_profile_option_limit_falls_back_per_concrete_relation_in_the_plan() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-fallback");
    fs::create_dir_all(&folder).expect("profile folder");
    let profile = folder.join("narrow.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"narrow","max_options":2}"#,
    )
    .expect("profile");
    let listener = Listener::answering(answered).expect("listener");
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"},{"name":"Acme","kind":"organization"},{"name":"Beta","kind":"organization"},{"name":"Core","kind":"organization"}]"#;
    let mut plans = Vec::new();
    for profiled in [false, true] {
        let mut options = vec!["works_for=person:organization", "--dry-run"];
        if profiled {
            options.extend(["--profile", profile.to_str().expect("path")]);
        }
        let output = run(&listener, &options, input);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        plans.push(serde_json::from_slice::<Value>(&output.stdout).expect("plan"));
    }
    assert_eq!(plans[0]["backend_profile"], Value::Null);
    assert_eq!(plans[0]["relations"][0]["method"], "choice");
    assert_eq!(plans[0]["relations"][0]["fallback"], Value::Null);
    assert_eq!(plans[0]["logical_questions"], 3);
    assert_eq!(plans[1]["backend_profile"], "narrow");
    assert_eq!(plans[1]["relations"][0]["method"], "yes_no");
    assert_eq!(plans[1]["relations"][0]["fallback"], "max_options");
    assert_eq!(plans[1]["logical_questions"], 6);
    assert_eq!(listener.connections(), 0);
}

#[test]
fn the_default_and_named_caches_answer_a_repeated_run_without_a_send() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-caches");
    let _removed = fs::remove_dir_all(&root);
    let xdg = root.join("xdg").to_string_lossy().into_owned();
    let named = root.join("named").to_string_lossy().into_owned();
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#;
    for cache in [None, Some(named.as_str())] {
        let listener = Listener::answering(answered).expect("listener");
        let mut arguments = vec![
            "relate",
            "works_for=person:organization",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--details",
        ];
        if let Some(folder) = cache {
            arguments.extend(["--cache", folder]);
        }
        let environment = [
            ("THINKTHEN_API_KEY", "secret"),
            ("XDG_CACHE_HOME", xdg.as_str()),
        ];
        let first = spawn(&arguments, &environment, input).expect("first run");
        assert_eq!(first.status.code(), Some(0), "{cache:?}");
        assert_eq!(listener.requests().len(), 1, "{cache:?}");
        let second = spawn(&arguments, &environment, input).expect("second run");
        assert_eq!(second.status.code(), Some(0), "{cache:?}");
        assert!(listener.requests().is_empty(), "{cache:?}");
        let first: Value = serde_json::from_slice(&first.stdout).expect("first details");
        let second: Value = serde_json::from_slice(&second.stdout).expect("second details");
        assert_eq!(first["answer"], second["answer"], "{cache:?}");
        assert_eq!(
            first["meta"]["requests"], second["meta"]["requests"],
            "{cache:?}"
        );
        assert_eq!(second["meta"]["cached"], true, "{cache:?}");
        assert_eq!(second["meta"]["requests_sent"], 0, "{cache:?}");
    }
    let named_files = crate::secrecy::written(&root.join("named"));
    assert!(!named_files.is_empty());
    let default_files = crate::secrecy::written(&root.join("xdg"));
    assert!(!default_files.is_empty());
}
