//! The compiled relate command against a counted loopback backend.

use crate::child::ChildEnvironment as _;
use std::{
    fs,
    io::Write as _,
    path::PathBuf,
    process::{Command, Output, Stdio},
};

use serde_json::Value;

use crate::harness::{Canned, Listener, finish, spawn};

mod at_once;
mod both_ways;
mod ceiling;
mod details;
mod menu;

fn plan_json(output: &Output) -> Value {
    let shown = String::from_utf8_lossy(&output.stdout);
    let mut lines = shown.lines();
    let plan = serde_json::from_str(lines.next().expect("plan report")).expect("plan JSON");
    let counts: Value =
        serde_json::from_str(lines.next().expect("whole-input counts")).expect("counts JSON");
    assert_eq!(counts["upper_bound"], true);
    assert_eq!(lines.next(), None);
    plan
}

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
            assert_eq!(question["type"], "noul", "relation question");
            let answer = serde_json::json!({"type":"noul","noul":0.9});
            (name.clone(), answer)
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(
        &serde_json::json!({"model":"local-1","answers":answers,"usage":{"input_tokens":10,"output_tokens":2}})
            .to_string(),
    )
}

/// A wrong-kind answer for one yes/no pair.
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
fn a_saved_relate_name_reaches_details_identity_and_warning() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-calibration");
    fs::create_dir_all(&folder).expect("folder");
    let question = folder.join("question.json");
    fs::write(&question, r#"{"version":1,"relate":{"relations":[{"name":"works_for","source":"person","target":"organization"}]},"profile":"old"}"#)
        .expect("question file");
    let running = folder.join("runtime.json");
    fs::write(
        &running,
        r#"{"schema":"thinkthen.backend-profile/1","name":"new","max_evidence_bytes":1000}"#,
    )
    .expect("runtime profile");
    let listener = Listener::answering(answered).expect("listener");
    let output = run(
        &listener,
        &[
            &format!("@{}", question.display()),
            "--details",
            "--profile",
            &running.to_string_lossy(),
        ],
        br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#,
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let row: Value = serde_json::from_slice(&output.stdout).expect("details");
    assert_eq!(
        row["meta"]["question_sha256"],
        "29399c0b883118fca36e97855e1edb34877efbb3348925b5dfce2582f3472774"
    );
    assert_eq!(
        row["meta"]["profile_warning"],
        serde_json::json!({"tuned_for":"old","running":"new"})
    );
    assert_eq!(listener.connections(), 1);
}

#[test]
fn a_wildcard_rule_expands_to_concrete_kinds_in_first_seen_order() {
    let listener = Listener::answering(answered).expect("listener");
    let output = run(
        &listener,
        &["linked=*:organization", "--plan"],
        br#"[{"name":"Acme","kind":"organization"},{"name":"Ada","kind":"person"},{"name":"Beta","kind":"organization"}]"#,
    );
    assert_eq!(output.status.code(), Some(0));
    let plan = plan_json(&output);
    assert_eq!(plan["relations"].as_array().expect("relations").len(), 1);
    assert_eq!(plan["relations"][0]["source"], "*");
    assert_eq!(plan["relations"][0]["target"], "organization");
    assert_eq!(plan["relations"][0]["method"], "yes_no");
    assert_eq!(plan["logical_questions"], 4);
    assert!(
        !plan["requests"][0]["body_utf8"]
            .as_str()
            .expect("body")
            .contains("\"relation\"")
    );
}

#[test]
fn every_true_edge_survives_an_unrelated_entity() {
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let names = request["state"]["entities"].as_array().expect("entities");
        let answers = request["questions"]
            .as_object()
            .expect("questions")
            .iter()
            .map(|(key, question)| {
                let words = question["instructions"].as_str().expect("words");
                let yes = ["Help!", "Girl", "In My Life"].iter().any(|song| {
                    let song_id = names
                        .iter()
                        .position(|item| item["name"] == *song)
                        .expect("song")
                        + 1;
                    words == format!("Is it true that i1 wrote i{song_id}?")
                });
                (
                    key.clone(),
                    serde_json::json!({"type":"noul","noul":if yes {0.98} else {0.02}}),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        Canned::ok(&serde_json::json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let base = [
        r#"{"name":"John Lennon","kind":"person"}"#,
        r#"{"name":"Paul McCartney","kind":"person"}"#,
        r#"{"name":"George Harrison","kind":"person"}"#,
        r#"{"name":"Help!","kind":"song"}"#,
        r#"{"name":"Girl","kind":"song"}"#,
        r#"{"name":"In My Life","kind":"song"}"#,
    ];
    let expected = concat!(
        "{\"relation\":\"wrote\",\"source\":{\"name\":\"John Lennon\",\"kind\":\"person\"},\"target\":{\"name\":\"Help!\",\"kind\":\"song\"},\"probability\":0.98}\n",
        "{\"relation\":\"wrote\",\"source\":{\"name\":\"John Lennon\",\"kind\":\"person\"},\"target\":{\"name\":\"Girl\",\"kind\":\"song\"},\"probability\":0.98}\n",
        "{\"relation\":\"wrote\",\"source\":{\"name\":\"John Lennon\",\"kind\":\"person\"},\"target\":{\"name\":\"In My Life\",\"kind\":\"song\"},\"probability\":0.98}\n",
    );
    for extra in [false, true] {
        let mut names = base.to_vec();
        if extra {
            names.push(r#"{"name":"Yesterday","kind":"song"}"#);
        }
        let input = format!("[{}]", names.join(","));
        let output = run(&listener, &["wrote=person:song"], input.as_bytes());
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
    }
    assert_eq!(listener.requests().len(), 2);
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
                options.push("--plan");
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
                options.push("--plan");
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
    let mut expected_answer = live["answer"].clone();
    expected_answer["questions"] = crate::native_results::compatibility::retrieved_members(
        expected_answer["questions"].clone(),
        "replay",
    );
    assert_eq!(expected_answer, result["answer"]);
    for key in ["value", "question"] {
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
        .clear_environment()
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
                options.push("--plan");
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
fn an_option_limit_does_not_change_pair_requests() {
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
        let mut options = vec!["works_for=person:organization", "--plan"];
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
        plans.push(plan_json(&output));
    }
    assert_eq!(plans[0]["backend_profile"], Value::Null);
    assert_eq!(plans[0]["relations"][0]["method"], "yes_no");
    assert_eq!(plans[0]["relations"][0]["fallback"], Value::Null);
    assert_eq!(plans[0]["logical_questions"], 6);
    assert_eq!(plans[1]["backend_profile"], "narrow");
    assert_eq!(plans[1]["requests"], plans[0]["requests"]);
    assert_eq!(plans[1]["logical_questions"], 6);
    assert_eq!(listener.connections(), 0);
}

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[test]
fn the_default_and_named_caches_answer_a_repeated_run_without_a_send() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-caches");
    let _removed = fs::remove_dir_all(&root);
    let moved = crate::child::Folder::Cache.variable(&root.join("xdg"));
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
        let environment = [("THINKTHEN_API_KEY", "secret"), (moved.0, moved.1.as_str())];
        let first = spawn(&arguments, &environment, input).expect("first run");
        assert_eq!(first.status.code(), Some(0), "{cache:?}");
        assert_eq!(listener.requests().len(), 1, "{cache:?}");
        let second = spawn(&arguments, &environment, input).expect("second run");
        assert_eq!(second.status.code(), Some(0), "{cache:?}");
        assert!(listener.requests().is_empty(), "{cache:?}");
        let first: Value = serde_json::from_slice(&first.stdout).expect("first details");
        let second: Value = serde_json::from_slice(&second.stdout).expect("second details");
        let mut expected_answer = first["answer"].clone();
        expected_answer["questions"] = crate::native_results::compatibility::retrieved_members(
            expected_answer["questions"].clone(),
            "cache",
        );
        assert_eq!(expected_answer, second["answer"], "{cache:?}");
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

/// A large pair set stays under the shared 400-question request bound.
#[test]
#[ignore = "release-only large-input boundary; run sdlc/scripts/test-full-cases --run"]
fn release_only_a_relation_of_180_names_sends_bounded_requests() {
    let backend = conformance_backend::Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let names = (0..180)
        .map(|place| format!("Name{place}\n"))
        .collect::<String>();
    let arguments = ["relate", "r", "--lines", "--url", &base, "--no-cache"];
    let output = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "secret-value")],
        names.as_bytes(),
    )
    .expect("command");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert_eq!(output.status.code(), Some(0));
    let printed = String::from_utf8_lossy(&output.stdout);
    assert_eq!(printed.lines().count(), 32_220);
    assert_eq!(
        printed.lines().next(),
        Some(
            r#"{"relation":"r","source":{"name":"Name0","kind":"*"},"target":{"name":"Name1","kind":"*"},"probability":0.9}"#
        )
    );
    assert_eq!(backend.count(), 81);
}
