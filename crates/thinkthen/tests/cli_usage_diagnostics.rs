#![cfg(test)]
//! Command admission reports fields without exposing caller payloads.
mod support;
use std::process::{Command, Stdio};

fn failure(command: &mut Command, field: &str, suggestion: Option<&str>) {
    let output = command
        .env("THINKTHEN_BACKEND", "stub")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(if field == "shell" || field == "entry" {
            5
        } else {
            2
        }),
        "{output:?}"
    );
    let message = String::from_utf8(output.stderr).unwrap();
    assert!(
        message.contains(if field == "shell" || field == "entry" {
            "local.toolbox:"
        } else {
            "usage.question:"
        }),
        "{message}"
    );
    assert!(message.contains(field), "{message}");
    assert!(message.contains(": expected "), "{message}");
    assert!(message.contains("; received "), "{message}");
    if let Some(name) = suggestion {
        assert!(
            message.contains(&format!("did you mean `{name}`?")),
            "{message}"
        );
    }
    assert!(!message.contains("private-payload"), "{message}");
}

#[test]
fn command_options_report_wrong_types_and_closest_names() {
    failure(
        support::command().args(["decide", "Ready?", "--batch", "private-payload"]),
        "batch",
        None,
    );
    failure(
        support::command().args(["decide", "Ready?", "--batc", "private-payload"]),
        "batc",
        Some("--batch"),
    );
    failure(
        support::command().args(["decide", "Ready?", "--json=private-payload"]),
        "json",
        None,
    );
    failure(
        support::command().arg("relate").env(
            "THINKTHEN_RULES",
            r#"[{"naem":"private-payload","source":"person","target":"company"}]"#,
        ),
        "rules",
        Some("name"),
    );
}

#[test]
fn command_framing_reports_wrong_types_and_closest_names() {
    failure(
        support::command()
            .args(["decide", "Ready?"])
            .env("THINKTHEN_COLUMNS", "[23]")
            .args(["--from", "csv"]),
        "column",
        None,
    );
    failure(
        support::command().args(["decide", "Ready?", "--from", "jsnol"]),
        "from",
        Some("jsonl"),
    );
}

#[test]
fn command_saved_runs_report_wrong_types_and_closest_names() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("saved.json");
    for (json, field, suggestion) in [
        (r#"{"run":[{"shell":[23]}]}"#, "shell", None),
        (
            r#"{"run":[{"shel":["private-payload"]}]}"#,
            "shel",
            Some("shell"),
        ),
        (
            r#"{"run":[{"qusetion":{"function":"decide","question":"private-payload"}}]}"#,
            "qusetion",
            Some("question"),
        ),
    ] {
        std::fs::write(&path, json).unwrap();
        failure(support::command().arg("run").arg(&path), field, suggestion);
    }
}

#[test]
fn command_grammar_reports_missing_and_unknown_arguments_without_values() {
    for arguments in [
        vec!["decide"],
        vec!["decide", "Ready?", "--batch"],
        vec!["relate", "--rule", "works_for", "person"],
        vec!["decide", "Ready?", "--batc=private-payload"],
    ] {
        let mut command = support::command();
        command.args(arguments);
        failure(&mut command, "", None);
    }
    failure(support::command().arg("deicde"), "command", Some("decide"));
}

#[test]
fn command_option_suggestions_use_the_parsed_command() {
    failure(
        support::command().args(["--model", "rank", "decide", "Ready?", "--lo"]),
        "option",
        Some("--low"),
    );
}

#[test]
fn command_saved_entry_selection_suggests_an_available_entry() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("saved.json");
    std::fs::write(
        &path,
        r#"{"entries":{"ready":{"question":{"function":"decide","question":"Ready?"}}}}"#,
    )
    .unwrap();
    failure(
        support::command()
            .arg("run")
            .arg(path)
            .args(["--entry", "raedy"]),
        "entry",
        Some("ready"),
    );
}

#[test]
fn command_enum_diagnostics_redact_unknown_value_contents() {
    for (option, environment) in [
        ("from", "THINKTHEN_FROM"),
        ("format", "THINKTHEN_FORMAT"),
        ("color", "THINKTHEN_COLOR"),
    ] {
        failure(
            support::command().args([
                "decide",
                "Ready?",
                &format!("--{option}"),
                "private-payload",
            ]),
            option,
            None,
        );
        failure(
            support::command()
                .args(["decide", "Ready?"])
                .env(environment, "private-payload"),
            option,
            None,
        );
    }
}

#[test]
fn evidence_pointers_refuse_unstructured_text_before_reading_input() {
    for framing in [None, Some("text"), Some("lines")] {
        for environment in [false, true] {
            let mut command = support::command();
            command.args(["decide", "Ready?", "--backend", "stub"]);
            if let Some(framing) = framing {
                command.args(["--from", framing]);
            }
            if environment {
                command.env("THINKTHEN_EVIDENCE", "/body");
            } else {
                command.args(["--evidence", "/body"]);
            }
            let output = command.stdin(Stdio::null()).output().unwrap();
            assert_eq!(output.status.code(), Some(2), "{output:?}");
            assert!(output.stdout.is_empty());
            let message = String::from_utf8(output.stderr).unwrap();
            assert!(
                message.starts_with("thinkthen: usage.question: "),
                "{message}"
            );
            assert!(message.contains("evidence"), "{message}");
        }
    }
}
