#![cfg(feature = "command")]
#![cfg(test)]
#![cfg(unix)]
//! Saved entries and declared commands through the command boundary.
mod support;

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::{Value, json};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "thinkthen-run-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn write(&self, name: &str, value: &Value) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, serde_json::to_vec(value).unwrap()).unwrap();
        path
    }
    fn run(&self, args: &[&str], input: &str) -> Output {
        let mut child = support::command()
            .current_dir(&self.0)
            .arg("run")
            .args(args)
            .args(["--adapter", "stub"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        if let Err(error) = child.stdin.take().unwrap().write_all(input.as_bytes()) {
            assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
        }
        child.wait_with_output().unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}
fn question() -> Value {
    json!({"function":"decide","question":{"type":"inline","value":"Ready?"}})
}
fn rows(output: &Output) -> Vec<Value> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[test]
fn saved_questions_use_explicit_files_folders_and_local_lookup() {
    let fixture = Fixture::new();
    fixture.write("single.json", &question());
    fixture.write("thinkthen.json", &json!({"entries":{"ready":question()}}));
    for args in [
        vec!["single.json"],
        vec!["thinkthen.json", "ready"],
        vec!["ready"],
    ] {
        let actual = rows(&fixture.run(&args, "\"alpha\"\n\"evidence 5\"\n"));
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0]["answer"], false);
        assert_eq!(actual[1]["answer"], true);
    }
    std::fs::create_dir(fixture.0.join("thinkthen")).unwrap();
    fixture.write("thinkthen/ready.json", &question());
    assert_eq!(fixture.run(&["ready"], "").status.code(), Some(5));
    std::fs::remove_file(fixture.0.join("thinkthen.json")).unwrap();
    assert_eq!(rows(&fixture.run(&["ready"], "\"alpha\"\n")).len(), 1);
    assert_eq!(
        rows(&fixture.run(&["thinkthen", "ready"], "\"alpha\"\n")).len(),
        1
    );
}

#[test]
fn declared_pipeline_passes_function_rows_to_shell_and_back() {
    let fixture = Fixture::new();
    fixture.write("pipeline.json", &json!({"run":[question(), {"shell":["/bin/sh","-c", "while IFS= read -r line; do case \"$line\" in *'\"answer\":false'*) printf '%s\\n' '\"evidence 5\"';; *) exit 29;; esac; done"]}, question()]}));
    let actual = rows(&fixture.run(&["pipeline.json"], "\"alpha\"\n"));
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0]["answer"], true);
    assert_eq!(actual[0]["p"], 0.77);
}

#[test]
fn shell_failure_keeps_its_status_and_stops_function_output() {
    let fixture = Fixture::new();
    fixture.write(
        "failure.json",
        &json!({"run":[{"shell":["/bin/sh","-c","exit 27"]},question()]}),
    );
    let output = fixture.run(&["failure.json"], "");
    assert_eq!(output.status.code(), Some(27));
    assert!(output.stdout.is_empty());
    fixture.write(
        "invalid.json",
        &json!({"run":[{"shell":["/bin/sh","-c","printf '%s\\n' not-json"]}]}),
    );
    assert_eq!(fixture.run(&["invalid.json"], "").status.code(), Some(5));
}

#[test]
fn ctrl_c_terminates_the_interpreter_and_its_child() {
    let fixture = Fixture::new();
    fixture.write("interrupt.json", &json!({"run":[{"shell":["/bin/sh","-c","sleep 300 & child=$!; printf '%s\\n' \"$child\" >&2; wait \"$child\""]}, question()]}));
    let mut child = support::command()
        .current_dir(&fixture.0)
        .args(["run", "interrupt.json", "--adapter", "stub"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut ready = String::new();
    BufReader::new(child.stderr.as_mut().unwrap())
        .read_line(&mut ready)
        .unwrap();
    let descendant: u32 = ready.trim().parse().unwrap();
    assert!(
        Command::new("kill")
            .args(["-INT", &child.id().to_string()])
            .status()
            .unwrap()
            .success()
    );
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(130));
    assert!(output.stdout.is_empty());
    assert!(
        !Command::new("kill")
            .args(["-0", &descendant.to_string()])
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success()
    );
}

#[test]
fn admission_finishes_before_any_declared_command_starts() {
    let fixture = Fixture::new();
    fixture.write("invalid.json", &json!({"run":[{"shell":["/bin/sh","-c","printf ran > marker"]},{"function":"choose","question":{"type":"inline","value":"Which?"}}]}));
    let rejected = fixture.run(&["invalid.json"], "");
    assert_eq!(rejected.status.code(), Some(2));
    assert_eq!(support::reported(&rejected), "usage.question");
    assert!(!fixture.0.join("marker").exists());
    for text in [
        r#"{"run":[],"run":[]}"#,
        r#"{"run":[]}"#,
        r#"{"run":[{"shell":[]}]}"#,
        r#"{"run":[{"run":[]}]}"#,
        r#"{"function":"decide","question":{"type":"inline","value":"Ready?"},"records":[]}"#,
    ] {
        std::fs::write(fixture.0.join("invalid.json"), text).unwrap();
        assert_eq!(
            fixture.run(&["invalid.json"], "").status.code(),
            Some(5),
            "{text}"
        );
    }
}

#[test]
fn shell_arguments_are_literal_and_text_lines_use_existing_framing() {
    let fixture = Fixture::new();
    fixture.write("literal.json", &json!({"run":[{"shell":["/usr/bin/printf","%s\n","{\"literal\":\"$HOME; touch marker\"}"]}]}));
    let actual = rows(&fixture.run(&["literal.json"], ""));
    assert_eq!(actual, vec![json!({"literal":"$HOME; touch marker"})]);
    assert!(!fixture.0.join("marker").exists());
    fixture.write("single.json", &question());
    let actual = rows(&fixture.run(&["single.json", "--lines"], "alpha\nevidence 5\n"));
    assert_eq!(actual.len(), 2);
    assert_eq!(actual[0]["answer"], false);
    assert_eq!(actual[1]["answer"], true);
    assert_eq!(
        fixture.run(&["single.json"], "not-json\n").status.code(),
        Some(5)
    );
}

#[test]
fn downstream_success_closes_and_reaps_a_still_running_upstream() {
    let fixture = Fixture::new();
    assert!(
        Command::new("mkfifo")
            .arg(fixture.0.join("block"))
            .status()
            .unwrap()
            .success()
    );
    fixture.write("early.json", &json!({"run":[
        {"shell":["/bin/sh","-c","printf '%s\\n' \"$$\" >&2; printf '%s\\n' true; read value < block"]},
        {"shell":["/bin/sh","-c","read value; exit 0"]}
    ]}));
    let mut child = support::command()
        .current_dir(&fixture.0)
        .args(["run", "early.json", "--adapter", "stub"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut ready = String::new();
    BufReader::new(child.stderr.as_mut().unwrap())
        .read_line(&mut ready)
        .unwrap();
    let upstream: u32 = ready.trim().parse().unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    assert!(
        !Command::new("kill")
            .args(["-0", &upstream.to_string()])
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success()
    );
}

#[test]
fn an_early_reader_drains_its_record_through_the_following_function() {
    let fixture = Fixture::new();
    fixture.write(
        "first.json",
        &json!({"run":[
            {"shell":["/bin/sh","-c","while :; do printf '%s\\n' '\"alpha\"'; done"]},
            {"shell":["/bin/sh","-c","read value; printf '%s\\n' \"$value\""]},
            question()
        ]}),
    );
    let actual = rows(&fixture.run(&["first.json"], ""));
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0]["answer"], false);
    assert_eq!(actual[0]["p"], 0.47);
}

#[test]
fn an_early_reader_cancels_only_its_upstream_function_call() {
    let fixture = Fixture::new();
    fixture.write("first.json", &json!({"run":[
        question(),
        {"shell":["/bin/sh","-c","read value; case \"$value\" in *'\"answer\":false'*) printf '%s\\n' '\"evidence 5\"';; *) exit 29;; esac"]},
        question()
    ]}));
    let input = "\"alpha\"\n".repeat(4096);
    let actual = rows(&fixture.run(&["first.json"], &input));
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0]["answer"], true);
    assert_eq!(actual[0]["p"], 0.77);
}

#[test]
fn a_failed_function_row_never_becomes_input_to_a_shell() {
    let fixture = Fixture::new();
    let mut request = question();
    request["options"] = json!({"on":"/missing"});
    fixture.write(
        "failed.json",
        &json!({"run":[
            request,
            {"shell":["/bin/sh","-c","while IFS= read -r record; do printf ran > marker; done"]}
        ]}),
    );
    let output = fixture.run(&["failed.json"], "{\"present\":1}\n");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(support::reported(&output), "usage.missing");
    assert!(output.stdout.is_empty());
    assert!(!fixture.0.join("marker").exists());
}
