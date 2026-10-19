//! Command admission refuses credential-bearing addresses before local effects.
#![allow(clippy::expect_used, reason = "a failed isolated command fixture stops the proof")]
#[path = "../../thinkthen/src/test_deadline/child.rs"]
mod child;
use child::ChildEnvironment as _;
#[path = "../../thinkthen/tests/support/key_address.rs"]
mod fixture;
use fixture::REFUSAL;
#[path = "support/command.rs"]
mod command;
use conformance_backend::Listener;
use std::{fs, process::Command};

#[test]
fn command_refuses_before_plan_status_or_recording_output() {
    let home = std::env::temp_dir().join(format!(
        "thinkthen-key-address-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _old = fs::remove_dir_all(&home);
    fs::create_dir(&home).expect("private home");
    let key = "key-path-0210";
    let listener = Listener::serving(vec![]).expect("loopback listener");
    let base = format!("{}/{key}", listener.base());
    let folder = home.join("recording");
    let record = folder.to_string_lossy().into_owned();
    for args in [
        vec!["decide", "a question", "--url", &base, "--plan"],
        vec!["decide", "a question", "--url", &base, "--record", &record],
        vec!["decide", "a question", "--url", &base, "--cache", &record],
        vec!["decide", "a question", "--url", &base, "--replay", &record],
        vec!["find", "a question", "--url", &base, "--plan"],
        vec![
            "recognize",
            "--kind",
            "person=Person",
            "--url",
            &base,
            "--plan",
        ],
        vec!["relate", "linked=person:person", "--url", &base, "--plan"],
        vec!["backends", "check", "--url", &base, "--plan"],
        vec!["status", "--json"],
        vec!["status"],
    ] {
        let output = Command::new(crate::command::binary())
            .args(&args)
            .clear_environment()
            .home(&home)
            .env("THINKTHEN_BASE_URL", &base)
            .env("THINKTHEN_API_KEY", key)
            .output()
            .expect("compiled command runs");
        assert!(!output.status.success(), "{args:?}");
        assert!(output.stdout.is_empty(), "{args:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains(REFUSAL), "{args:?}: {message}");
        assert!(!message.contains(key), "{args:?}: {message}");
        assert!(!message.contains(&base), "{args:?}: {message}");
        assert!(!folder.exists(), "{args:?}");
    }
    assert_eq!(listener.count(), 0);
    for (keyed, expected) in [(true, true), (false, false)] {
        let mut command = Command::new(crate::command::binary());
        command
            .args(["status", "--json"])
            .clear_environment()
            .home(&home)
            .env("THINKTHEN_BASE_URL", listener.base());
        if keyed {
            command.env("THINKTHEN_API_KEY", key);
        }
        let output = command.output().expect("safe status");
        assert!(output.status.success());
        let status: serde_json::Value = serde_json::from_slice(&output.stdout).expect("status");
        assert_eq!(status["backend"]["api_key_set"], expected);
        assert_eq!(
            status["backend"]["url"],
            format!("{}/systemone", listener.base())
        );
    }
    assert_eq!(fs::read_dir(&home).expect("home").count(), 0);
    fs::remove_dir_all(home).expect("fixture cleanup");
}
