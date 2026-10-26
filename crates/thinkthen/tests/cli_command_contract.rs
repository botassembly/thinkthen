#![cfg(test)]
#![cfg(feature = "command")]
//! Approved command contract expectations. These cases deliberately await the
//! typed engine and command implementation; none are ignored or gated on it.

mod support;

use std::fs;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};

fn execute(command: &mut Command, input: &[u8]) -> Output {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Argument rejection may close stdin before the parent writes.
    let _ = child.stdin.take().unwrap().write_all(input);
    child.wait_with_output().unwrap()
}

fn rows(output: &Output) -> Vec<Value> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn backend(name: &str, probability: f64) -> Loopback {
    if name == "systemone" {
        return Loopback::noul(probability).unwrap();
    }
    Loopback::start(move |request| {
        let request = request.json().unwrap();
        let answers: Vec<Value> = request["questions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|_| json!({"type":"predicate","probability":probability}))
            .collect();
        Reply::json(200, &json!({"answers":answers}))
    })
    .unwrap()
}

fn selected(name: &str, server: &Loopback) -> Command {
    let mut command = support::command();
    command
        .args([
            "--backend",
            name,
            "--url",
            server.base(),
            "--model",
            "fixture",
        ])
        .env(
            if name == "openai" {
                "OPENAI_API_KEY"
            } else {
                "THINKTHEN_API_KEY"
            },
            KEY,
        );
    command
}

#[test]
fn json_rows_keep_the_same_typed_contract_on_both_wire_formats() {
    for name in ["systemone", "openai"] {
        let server = backend(name, 0.87654321);
        let output = execute(
            selected(name, &server).args([
                "decide",
                "Ready?",
                "--from",
                "jsonl",
                "--evidence",
                "/body",
                "--json",
            ]),
            b"{\"id\":42,\"body\":\"one\"}\n{\"id\":\"two\",\"body\":\"two\"}\n",
        );
        assert_eq!(output.status.code(), Some(0), "{name}: {output:?}");
        assert!(output.stderr.is_empty());
        let actual = rows(&output);
        assert_eq!(actual.len(), 2);
        for (index, row) in actual.iter().enumerate() {
            assert_eq!(row["position"], index + 1);
            assert_eq!(row["function"], "decide");
            assert_eq!(row["status"], "answered");
            assert_eq!(row["answer"], true);
            assert_eq!(row["probability"], 0.87654321);
            assert_eq!(row["source"]["backend"], name);
            assert_eq!(row["source"]["model"], "fixture");
            assert!(row["source"]["question_identity"].is_string());
            assert!(row.get("p").is_none());
            assert!(row.get("file").is_none());
        }
        assert_eq!(actual[0]["id"], 42);
        assert_eq!(actual[1]["id"], "two");
        assert!(server.count() > 0);
    }
}

#[test]
fn answer_exit_codes_require_check_on_both_backends() {
    for name in ["systemone", "openai"] {
        for (probability, band, check, expected, word) in [
            (0.9, false, false, 0, "true\n"),
            (0.1, false, false, 0, "false\n"),
            (0.5, true, false, 0, "unsure\n"),
            (0.9, false, true, 0, "true\n"),
            (0.1, false, true, 1, "false\n"),
            (0.5, true, true, 3, "unsure\n"),
        ] {
            let server = backend(name, probability);
            let mut command = selected(name, &server);
            command.args(["decide", "Ready?"]);
            if band {
                command.args(["--low", "0.2", "--high", "0.8"]);
            }
            if check {
                command.arg("--check");
            }
            let output = execute(&mut command, b"one\ntwo\n");
            assert_eq!(output.status.code(), Some(expected), "{name}: {output:?}");
            assert_eq!(output.stdout, word.as_bytes());
        }
    }
}

#[test]
fn record_failures_remain_visible_and_override_filter_check() {
    for name in ["systemone", "openai"] {
        let server = backend(name, 0.9);
        let output = execute(
            selected(name, &server)
                .args(["filter", "Ready?", "--from", "jsonl", "--json", "--check"]),
            b"\"keep\"\nnull\n{\n\"keep again\"\n",
        );
        assert_eq!(output.status.code(), Some(6), "{name}: {output:?}");
        let actual = rows(&output);
        assert_eq!(actual.len(), 4);
        assert_eq!(actual[0]["answer"], true);
        assert_eq!(actual[1]["error"]["code"], "record.missing");
        assert_eq!(actual[2]["error"]["code"], "record.json");
        assert_eq!(actual[3]["answer"], true);
        for index in [1, 2] {
            assert_eq!(actual[index]["position"], index + 1);
            assert_eq!(actual[index]["status"], "failed");
            assert!(actual[index].get("answer").is_none());
            assert!(actual[index].get("probability").is_none());
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("record.missing"));
        assert!(stderr.contains("record.json"));
    }
}

#[test]
fn all_failed_rows_are_a_completed_call_and_never_leak_reply_keys() {
    for name in ["systemone", "openai"] {
        let server = Loopback::start(|_| {
            Reply::json(401, &json!({"error":format!("bad credential {KEY}")}))
        })
        .unwrap();
        let output = execute(
            selected(name, &server).args(["decide", "Ready?", "--json", "--verbose"]),
            b"evidence",
        );
        assert_eq!(output.status.code(), Some(6), "{name}: {output:?}");
        let actual = rows(&output);
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0]["status"], "failed");
        assert_eq!(actual[0]["error"]["code"], "backend.status");
        assert_eq!(actual[0]["source"]["backend"], name);
        assert!(!String::from_utf8_lossy(&output.stdout).contains(KEY));
        assert!(!String::from_utf8_lossy(&output.stderr).contains(KEY));
    }
}

#[test]
fn grammar_rejects_missing_lists_removed_flags_and_ambiguous_check_before_sending() {
    let cases: &[&[&str]] = &[
        &["choose", "Which?"],
        &["tag", "Which?"],
        &["score", "How?"],
        &["rank", "How?"],
        &["recognize"],
        &["relate"],
        &["relate", "--rule", "works_for", "person"],
        &["decide", "Ready?", "--low", "0.2"],
        &["decide", "Ready?", "--threshold", "0.2:0.8"],
        &["find", "Which?", "--low", "0.2", "--high", "0.8"],
        &["decide", "Ready?", "--adapter", "stub"],
        &["decide", "Ready?", "--details"],
        &["decide", "Ready?", "--on", "/body"],
        &["decide", "Ready?", "--lines"],
        &["choose", "Which?", "--option", "a", "--option", "b"],
        &["decide", "Ready?", "--check", "--from", "lines"],
        &[
            "choose", "Which?", "--choice", "a", "--choice", "b", "--check", "--from", "jsonl",
        ],
        &["find", "Which?", "--check", "one.txt", "two.txt"],
        &["decide", "Ready?", "-", "-"],
    ];
    let server = backend("systemone", 0.9);
    for args in cases {
        let output = execute(selected("systemone", &server).args(*args), b"one");
        assert_eq!(output.status.code(), Some(2), "{args:?}: {output:?}");
        assert!(output.stdout.is_empty(), "{args:?}");
        assert!(!String::from_utf8_lossy(&output.stderr).contains(KEY));
    }
    assert_eq!(server.count(), 0);
}

#[test]
fn explicit_values_replace_malformed_environment_before_admission() {
    let mut command = support::command();
    let output = execute(
        command
            .args([
                "--backend",
                "stub",
                "--batch",
                "1",
                "decide",
                "Ready?",
                "--threshold",
                "0.7",
                "--format",
                "json",
                "--check=false",
            ])
            .env("THINKTHEN_BACKEND", "unknown")
            .env("THINKTHEN_BATCH", "bad")
            .env("THINKTHEN_LOW", "bad")
            .env("THINKTHEN_HIGH", "bad")
            .env("THINKTHEN_JSON", "bad")
            .env("THINKTHEN_FORMAT", "bad")
            .env("THINKTHEN_CHECK", "true")
            .env("THINKTHEN_COLUMNS", "bad")
            .env("THINKTHEN_SHEET", "irrelevant"),
        b"one",
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(rows(&output).len(), 1);
}

#[test]
fn json_array_environment_lists_preserve_labels_and_explicit_lists_replace_them() {
    let cases = [
        (
            vec!["choose", "Which?"],
            "THINKTHEN_CHOICES",
            vec!["--choice", "customer care", "--choice", "billing, refunds"],
        ),
        (
            vec!["tag", "Which?"],
            "THINKTHEN_LABELS",
            vec!["--label", "customer care", "--label", "billing, refunds"],
        ),
        (
            vec!["score", "How?"],
            "THINKTHEN_LEVELS",
            vec!["--level", "customer care", "--level", "billing, refunds"],
        ),
        (
            vec!["rank", "How?"],
            "THINKTHEN_LEVELS",
            vec!["--level", "customer care", "--level", "billing, refunds"],
        ),
        (
            vec!["recognize"],
            "THINKTHEN_KINDS",
            vec!["--kind", "customer care", "--kind", "billing, refunds"],
        ),
    ];
    for (args, variable, explicit) in cases {
        let baseline = execute(
            support::command()
                .args(["--backend", "stub", "--json"])
                .args(&args)
                .args(&explicit),
            b"Ada called billing.",
        );
        assert_eq!(baseline.status.code(), Some(0), "{args:?}: {baseline:?}");
        let inherited = execute(
            support::command()
                .args(["--backend", "stub", "--json"])
                .args(&args)
                .env(variable, "[\"customer care\",\"billing, refunds\"]"),
            b"Ada called billing.",
        );
        assert_eq!(
            inherited.status.code(),
            Some(0),
            "{variable}: {inherited:?}"
        );
        assert_eq!(rows(&inherited), rows(&baseline));
        let overridden = execute(
            support::command()
                .args(["--backend", "stub", "--json"])
                .args(&args)
                .args(&explicit)
                .env(variable, "not JSON"),
            b"Ada called billing.",
        );
        assert_eq!(overridden.status.code(), Some(0));
        assert_eq!(rows(&overridden), rows(&baseline));
        let empty = execute(
            support::command()
                .args(["--backend", "stub"])
                .args(&args)
                .env(variable, "[]"),
            b"one",
        );
        assert_eq!(empty.status.code(), Some(2), "{variable}: {empty:?}");
    }
}

#[test]
fn input_files_and_stdin_keep_one_sequence_and_original_identifiers() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(
        directory.path().join("first.jsonl"),
        "{\"id\":7,\"body\":\"first\"}\n",
    )
    .unwrap();
    fs::write(
        directory.path().join("-last.jsonl"),
        "{\"id\":\"last\",\"body\":\"last\"}\n",
    )
    .unwrap();
    let output = execute(
        support::command().current_dir(directory.path()).args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--json",
            "--evidence",
            "/body",
            "--from",
            "jsonl",
            "--",
            "first.jsonl",
            "-",
            "-last.jsonl",
        ]),
        b"{\"id\":8,\"body\":\"middle\"}\n",
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let actual = rows(&output);
    assert_eq!(actual.len(), 3);
    assert_eq!(actual[0]["id"], 7);
    assert_eq!(actual[1]["id"], 8);
    assert_eq!(actual[2]["id"], "last");
    for (index, row) in actual.iter().enumerate() {
        assert_eq!(row["position"], index + 1);
    }
}

#[test]
fn table_json_conflicts_refuse_before_creating_a_destination() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("messages.csv");
    let destination = directory.path().join("result.csv");
    fs::write(&source, "body\none\n").unwrap();
    let server = backend("systemone", 0.9);
    for inherited in [false, true] {
        let mut command = selected("systemone", &server);
        command
            .args(["decide", "Ready?"])
            .arg(&source)
            .args(["--column", "body", "--into", "ready", "--output"])
            .arg(&destination);
        if inherited {
            command.env("THINKTHEN_JSON", "true");
        } else {
            command.arg("--json");
        }
        let output = execute(&mut command, b"");
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(!destination.exists());
    }
    assert_eq!(server.count(), 0);
}

#[test]
fn terminal_local_failures_exit_five_and_setup_validation_exits_two() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("absent.txt");
    let output = execute(
        support::command()
            .args(["decide", "Ready?", "--backend", "stub"])
            .arg(missing),
        b"",
    );
    assert_eq!(output.status.code(), Some(5), "{output:?}");
    let output = execute(support::command().args(["decide", "Ready?"]), b"one");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("usage.key-missing"));
}

#[test]
fn help_version_and_completions_need_no_backend_and_reveal_no_credentials() {
    for args in [
        vec!["--help"],
        vec!["--version"],
        vec!["completions", "zsh"],
    ] {
        let output = execute(
            support::command()
                .args(&args)
                .env("THINKTHEN_API_KEY", KEY)
                .env("OPENAI_API_KEY", KEY)
                .env("THINKTHEN_BACKEND", "invalid")
                .env("THINKTHEN_URL", "not a URL"),
            b"",
        );
        assert_eq!(output.status.code(), Some(0), "{args:?}: {output:?}");
        assert!(!output.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&output.stdout).contains(KEY));
        assert!(!String::from_utf8_lossy(&output.stderr).contains(KEY));
    }
}

#[test]
fn output_selector_groups_and_attached_false_overrides_are_unambiguous() {
    for (arguments, environment, code, json_output) in [
        (
            vec!["--format", "json"],
            vec![("THINKTHEN_JSON", "false")],
            0,
            true,
        ),
        (
            vec!["--json=false"],
            vec![("THINKTHEN_FORMAT", "json")],
            0,
            false,
        ),
        (vec!["--format", "text", "--json=false"], vec![], 0, false),
        (vec!["--format", "json", "--json=false"], vec![], 2, false),
        (
            vec![],
            vec![("THINKTHEN_FORMAT", "json"), ("THINKTHEN_JSON", "false")],
            2,
            false,
        ),
    ] {
        let output = execute(
            support::command()
                .args(["decide", "Ready?", "--backend", "stub"])
                .args(arguments)
                .envs(environment),
            b"one",
        );
        assert_eq!(output.status.code(), Some(code), "{output:?}");
        if code == 0 && json_output {
            assert_eq!(rows(&output).len(), 1);
        } else if code == 0 {
            assert!(matches!(
                output.stdout.as_slice(),
                b"true\n" | b"false\n" | b"unsure\n"
            ));
        }
    }
}

#[test]
fn environment_options_are_scoped_to_the_selected_function() {
    let output = execute(
        support::command()
            .args([
                "tag",
                "Which?",
                "--label",
                "a",
                "--label",
                "b",
                "--backend",
                "stub",
                "--json",
            ])
            .env("THINKTHEN_CHECK", "not a boolean")
            .env("THINKTHEN_TOP", "not an integer")
            .env("THINKTHEN_LEVELS", "not JSON")
            .env("THINKTHEN_RULES", "not JSON"),
        b"one",
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(rows(&output).len(), 1);
}

#[test]
fn typed_relation_environment_and_explicit_direction_have_the_same_mapping() {
    let text = b"Ada joined Acme.";
    let baseline = execute(
        support::command().args([
            "relate",
            "--rule",
            "works_for",
            "person",
            "organization",
            "--either",
            "--backend",
            "stub",
            "--json",
        ]),
        text,
    );
    assert_eq!(baseline.status.code(), Some(0), "{baseline:?}");
    let inherited = execute(support::command().args([
        "relate", "--either", "--backend", "stub", "--json",
    ]).env("THINKTHEN_RULES", r#"[{"name":"works_for","source":"person","target":"organization","either":false}]"#), text);
    assert_eq!(inherited.status.code(), Some(0), "{inherited:?}");
    assert_eq!(rows(&inherited), rows(&baseline));
    let overridden = execute(
        support::command()
            .args([
                "relate",
                "--rule",
                "works_for",
                "person",
                "organization",
                "--either",
                "--backend",
                "stub",
                "--json",
            ])
            .env("THINKTHEN_RULES", "bad JSON"),
        text,
    );
    assert_eq!(overridden.status.code(), Some(0), "{overridden:?}");
    assert_eq!(rows(&overridden), rows(&baseline));
}

#[test]
fn annotate_and_saved_entries_read_paths_and_share_global_options() {
    let directory = tempfile::tempdir().unwrap();
    let questions = directory.path().join("questions.json");
    let saved = directory.path().join("saved.json");
    fs::write(
        &questions,
        json!({"questions":{"ready":{"function":"decide","question":"Ready?"}}}).to_string(),
    )
    .unwrap();
    fs::write(
        &saved,
        json!({"entries":{"ready":{"question":{"function":"decide","question":"Ready?"}}}})
            .to_string(),
    )
    .unwrap();
    let annotation = execute(
        support::command()
            .args(["--backend", "stub", "annotate"])
            .arg(&questions)
            .arg("--json"),
        b"one\ntwo\n",
    );
    assert_eq!(annotation.status.code(), Some(0), "{annotation:?}");
    assert_eq!(rows(&annotation).len(), 2);
    let before = execute(
        support::command()
            .args(["--backend", "stub", "--json", "--batch", "1", "run"])
            .arg(&saved)
            .args(["--entry", "ready", "--from", "lines"]),
        b"one\ntwo\n",
    );
    assert_eq!(before.status.code(), Some(0), "{before:?}");
    let after = execute(
        support::command().arg("run").arg(&saved).args([
            "--entry",
            "ready",
            "--from",
            "lines",
            "--backend",
            "stub",
            "--json",
            "--batch",
            "1",
        ]),
        b"one\ntwo\n",
    );
    assert_eq!(after.status.code(), Some(0), "{after:?}");
    assert_eq!(rows(&before).len(), 2);
    assert_eq!(rows(&after), rows(&before));
}

#[test]
fn delimited_stdin_writes_a_table_and_explicit_destination_replaces_in_place_environment() {
    for (format, input, delimiter) in [
        ("csv", "body\none\ntwo\n", b','),
        ("tsv", "body\none\ntwo\n", b'\t'),
    ] {
        let output = execute(
            support::command().args([
                "decide",
                "Ready?",
                "--backend",
                "stub",
                "--from",
                format,
                "--column",
                "body",
                "--into",
                "ready",
            ]),
            input.as_bytes(),
        );
        assert_eq!(output.status.code(), Some(0), "{format}: {output:?}");
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .from_reader(output.stdout.as_slice());
        let headers = reader.headers().unwrap().clone();
        assert!(headers.iter().any(|header| header == "ready"));
        let records = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(&records[0][0], "one");
        assert_eq!(&records[1][0], "two");
    }
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("messages.csv");
    let destination = directory.path().join("result.csv");
    let original = b"body\none\n";
    fs::write(&source, original).unwrap();
    let output = execute(
        support::command()
            .args(["decide", "Ready?", "--backend", "stub"])
            .arg(&source)
            .args(["--column", "body", "--into", "ready", "--output"])
            .arg(&destination)
            .env("THINKTHEN_IN_PLACE", "true")
            .env("THINKTHEN_OUTPUT", "bad inherited path")
            .env("THINKTHEN_JSON", "true")
            .args(["--format", "text"]),
        b"",
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(fs::read(&source).unwrap(), original);
    assert!(destination.exists());
}

#[test]
fn generated_manual_and_shell_completions_share_command_options() {
    let manual = execute(support::command().arg("man"), b"");
    assert!(manual.status.success());
    let text = String::from_utf8(manual.stdout).unwrap();
    assert!(text.contains(".TH"));
    assert!(text.contains("backend"));
    assert!(text.contains("evidence"));
    for shell in ["bash", "zsh", "fish", "elvish", "powershell"] {
        let completion = execute(support::command().args(["completions", shell]), b"");
        assert!(completion.status.success(), "{shell}: {completion:?}");
        assert!(!completion.stdout.is_empty());
    }
}

#[test]
fn oversized_annotation_toolbox_stops_before_backend_work() {
    let directory = tempfile::tempdir().unwrap();
    let definition = directory.path().join("questions.json");
    fs::write(&definition, " ".repeat(1_048_577)).unwrap();
    let output = execute(
        support::command().args([
            "annotate",
            definition.to_str().unwrap(),
            "--backend",
            "stub",
        ]),
        b"one",
    );
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("toolbox exceeds"));
}

#[test]
fn execution_names_are_explicit_and_saved_names_survive_entry_selection() {
    let named = execute(
        support::command().args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--json",
            "--name",
            "priority",
        ]),
        b"one",
    );
    assert_eq!(named.status.code(), Some(0));
    assert_eq!(rows(&named)[0]["source"]["name"], "priority");
    let directory = tempfile::tempdir().unwrap();
    let saved = directory.path().join("saved.json");
    fs::write(&saved, json!({"entries":{"selected":{"name":"stored","question":{"function":"decide","question":"Ready?"}}}}).to_string()).unwrap();
    let output = execute(
        support::command()
            .arg("run")
            .arg(&saved)
            .args(["--entry", "selected", "--backend", "stub", "--json"])
            .env("THINKTHEN_NAME", "ignored"),
        b"\"one\"\n",
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(rows(&output)[0]["source"]["name"], "stored");
}

#[test]
fn color_controls_diagnostics_and_never_enters_json_rows() {
    for (options, environment, colored) in [
        (vec!["--color", "always"], vec![], true),
        (vec![], vec![("THINKTHEN_COLOR", "always")], true),
        (
            vec![],
            vec![("THINKTHEN_COLOR", "always"), ("NO_COLOR", "1")],
            false,
        ),
        (vec!["--color", "always"], vec![("NO_COLOR", "1")], true),
        (
            vec!["--color", "never"],
            vec![("THINKTHEN_COLOR", "always")],
            false,
        ),
    ] {
        let output = execute(
            support::command()
                .args(["decide", "Ready?", "--unknown"])
                .args(options)
                .envs(environment),
            b"",
        );
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(output.stderr.contains(&0x1b), colored, "{output:?}");
    }
    let output = execute(
        support::command().args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--json",
            "--color",
            "always",
        ]),
        b"one",
    );
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(rows(&output).len(), 1);
    assert!(!output.stdout.contains(&0x1b));
}
