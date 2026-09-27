//! `annotate --dry-run` prints the first request a live run sends, and counts
//! the requests each `on` group makes.
#![cfg(feature = "cli")]

use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[path = "../src/test_deadline/wait.rs"]
mod wait;

fn file(name: &str, text: &str) -> String {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-plan");
    fs::create_dir_all(&folder).expect("a folder");
    let path = folder.join(name);
    fs::write(&path, text).expect("a file");
    path.to_string_lossy().into_owned()
}

/// The one plan line a dry run prints, after it exits 0.
fn dry_run(arguments: &[&str], input: &[u8]) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .args(["annotate", "--model", "local-1", "--no-cache", "--dry-run"])
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the command starts");
    let mut stdin = child.stdin.take().expect("stdin");
    stdin.write_all(input).expect("the input");
    drop(stdin);
    let output = wait::finish(child, "annotate --dry-run").expect("the command ends");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    String::from_utf8(output.stdout).expect("text")
}

#[test]
fn the_plan_shows_each_request() {
    // Edge row 9: 250 questions under a profile of 100 a request.
    let questions = (1..=250)
        .map(|place| format!(r#""q{place}":{{"decide":"Question {place}?"}}"#))
        .collect::<Vec<_>>()
        .join(",");
    let many = file(
        "many.json",
        &format!(r#"{{"version":1,"questions":{{{questions}}}}}"#),
    );
    let profile = file(
        "hundred.json",
        r#"{"schema":"thinkthen.backend-profile/1","name":"hundred","max_questions":100}"#,
    );
    let line = dry_run(&[&many, "--profile", &profile], b"evidence");
    let counts = r#"]},"request_count":3,"group_requests":[3],"request":{"#;
    assert!(line.contains(counts), "{line}");
    let plan: serde_json::Value = serde_json::from_str(&line).expect("one JSON plan");
    let sent = plan["request"]["questions"].as_object().expect("questions");
    let last = sent["q100"]["instructions"].as_str();
    assert_eq!((sent.len(), last), (100, Some("Question 100?")));

    // Edge row 10: two `on` groups and no profile print the first group's request.
    let two = file(
        "two.json",
        r#"{"version":1,"questions":{"concise":{"decide":"Is this concise?","on":"/summary"},"refund":{"decide":"Does this ask for a refund?","on":"/body"}}}"#,
    );
    assert_eq!(
        dry_run(&[&two], br#"{"summary":"Short note.","body":"Refund me."}"#),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"local-1","key_env":"THINKTHEN_API_KEY","#,
            r#""on":{"concise":["/summary"],"refund":["/body"]},"request_count":2,"group_requests":[1,1],"#,
            r#""request":{"state":"Short note.","model":"local-1","questions":{"q1":{"type":"noul","instructions":"Is this concise?"}}}}"#,
            "\n",
        )
    );
}
