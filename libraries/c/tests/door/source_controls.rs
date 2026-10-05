//! Source controls apply to the complete call and stop before an unread tail.
use super::sources::{all_no_answers, ask_settings};
use crate::child::ChildEnvironment as _;
use crate::{compile, crate_dir, scratch, text};
use conformance_backend::{Backend, Listener};
use serde_json::{Value, json};

fn files(label: &str) -> Vec<std::path::PathBuf> {
    let folder = scratch(label);
    std::fs::create_dir_all(&folder).expect("folder");
    ["Ada", "Bea", "Cy"]
        .iter()
        .enumerate()
        .map(|(at, name)| {
            let path = folder.join(format!("{at}.txt"));
            std::fs::write(&path, name).expect("record");
            path
        })
        .collect()
}

fn recognition(paths: &[std::path::PathBuf]) -> Value {
    json!({"version":1,"recognize":{"kinds":{"person":null}},"source":{"paths":paths,"unit":"file"}})
}

#[test]
fn source_recognition_caps_records_across_scalar_calls_and_leaves_tail_unread() {
    let paths = files("recognize-record-cap");
    std::fs::write(&paths[2], [0xff]).expect("invalid unread tail");
    let listener = Listener::answering(all_no_answers).expect("listener");
    let replies = ask_settings(
        listener.base(),
        &[recognition(&paths)],
        json!({"cache":false,"max_retries":0,"max_requests":1}),
    );
    let (code, reply) = &replies[0];
    assert_eq!(*code, 1, "{reply}");
    assert_eq!(
        reply["message"],
        "this engine answers at most 1 records in one call"
    );
    assert_eq!(reply["facts"]["records"], 1);
    assert_eq!(reply["facts"]["requests_sent"], 1);
    assert_eq!(listener.count(), 1);
}

#[test]
fn source_recognition_uses_one_deadline_for_every_record() {
    let paths = files("recognize-call-deadline");
    let listener = Listener::answering(|body| all_no_answers(body).after(80)).expect("listener");
    let output = std::process::Command::new(compile(
        &crate_dir().join("tests/c/source_recognize_deadline.c"),
    ))
    .clear_environment()
    .env("THINKTHEN_BASE_URL", listener.base())
    .env("THINKTHEN_API_KEY", "sk-source-deadline-loopback")
    .arg(recognition(&paths).to_string())
    .output()
    .expect("recognition deadline");
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    let facts: Value = serde_json::from_slice(&output.stdout).expect("deadline facts");
    assert_eq!(facts["records"], 1);
    assert_eq!(facts["requests_sent"], 2);
    assert_eq!(listener.count(), 2);
}

#[test]
fn source_plan_applies_count_and_aggregate_bytes_before_reading_tail() {
    let backend = Backend::start().expect("backend");
    for (label, cap, large, sentence) in [
        (
            "plan-record-cap",
            json!(1),
            false,
            "this engine answers at most 1 records in one call",
        ),
        (
            "plan-byte-cap",
            Value::Null,
            true,
            "source plan input exceeds 16 MiB",
        ),
    ] {
        let paths = files(label);
        if large {
            for path in &paths[..2] {
                std::fs::write(path, "x".repeat(8 * 1024 * 1024 + 1)).expect("large records");
            }
        }
        std::fs::write(&paths[2], [0xff]).expect("invalid unread tail");
        let request =
            json!({"verb":"decide","question":"Q?","source":{"paths":paths,"unit":"file"}});
        let settings = json!({"cache":false,"max_requests":cap});
        let output =
            std::process::Command::new(compile(&crate_dir().join("tests/c/source_plan_refusal.c")))
                .clear_environment()
                .env(
                    "THINKTHEN_BASE_URL",
                    format!("{}/generic/v1", backend.origin()),
                )
                .args([
                    settings.to_string(),
                    request.to_string(),
                    sentence.to_owned(),
                ])
                .output()
                .expect("plan refusal");
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
    }
    assert_eq!(backend.count(), 0);
}
