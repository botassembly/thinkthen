//! The shared engine-setting cases through the compiled command's flags.
#![cfg(feature = "cli")]
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "shared case fixtures must be complete"
)]

use conformance_backend::Backend;
use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

#[expect(clippy::panic, reason = "an unmapped shared case is a failed test fixture")]
fn flags(
    name: &str,
    value: &Value,
    folder: &std::path::Path,
    profile: &std::path::Path,
) -> Vec<String> {
    match name {
        "cache" if value == &Value::Bool(false) => vec!["--no-cache".to_owned()],
        "cache" | "record" | "replay" => vec![format!("--{name}"), folder.display().to_string()],
        "profile" => vec!["--profile".to_owned(), profile.display().to_string()],
        "max_retries" => vec!["--max-retries".to_owned(), value.to_string()],
        "timeout" | "model" => vec![
            format!("--{name}"),
            value
                .as_str()
                .map_or_else(|| value.to_string(), str::to_owned),
        ],
        _ => panic!("unknown command setting {name}"),
    }
}

#[test]
fn command_settings_match_the_shared_cases() {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../conformance/settings.json")).expect("cases");
    assert_eq!(corpus["schema"], "thinkthen.settings-cases/1");
    for case in corpus["cases"].as_array().expect("cases") {
        if case["setting"] == "max_requests" {
            continue;
        }
        let backend = Backend::start().expect("backend");
        let id = case["id"].as_str().expect("id");
        let base = format!(
            "{}/{}",
            backend.origin(),
            case["arm"].as_str().expect("arm")
        );
        let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("command-settings-{id}-{}", std::process::id()));
        std::fs::create_dir(&folder).expect("fresh folder");
        let profile = folder.join("profile.json");
        if !case["profile"].is_null() {
            std::fs::write(&profile, case["profile"].to_string()).expect("profile");
        }
        for step in case["steps"].as_array().expect("steps") {
            let mut args = vec![
                "decide".to_owned(),
                corpus["question"].as_str().expect("question").to_owned(),
                "--url".to_owned(),
                base.clone(),
            ];
            for (name, value) in step["settings"].as_object().expect("settings") {
                args.extend(flags(name, value, &folder, &profile));
            }
            if step["model"].is_string() {
                args.push("--details".to_owned());
            }
            let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
                .args(&args)
                .env_clear()
                .env("HOME", &folder)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("command");
            child
                .stdin
                .take()
                .expect("stdin")
                .write_all(step["text"].as_str().expect("text").as_bytes())
                .expect("input");
            let output = child.wait_with_output().expect("output");
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            if let Some(kind) = step["error"].as_str() {
                let code = match kind {
                    "usage" => 2,
                    "backend" => 4,
                    "local" => 5,
                    _ => panic!("unknown error kind {kind}"),
                };
                assert_eq!(output.status.code(), Some(code), "{id}: {stderr}");
            } else if let Some(model) = step["model"].as_str() {
                let details: Value = serde_json::from_slice(&output.stdout).expect("details");
                assert_eq!(details["meta"]["model"], model, "{id}");
            } else {
                assert!(
                    output.status.success() && stdout.trim() == "true",
                    "{id}: {stdout} {stderr}"
                );
            }
            assert_eq!(
                backend.count(),
                step["count"].as_u64().expect("count") as usize,
                "{id}"
            );
        }
        std::fs::remove_dir_all(folder).expect("clean folder");
    }
}
