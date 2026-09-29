//! Public and compiled-command boundaries for an exact configured-key URL collision.
#![allow(
    clippy::expect_used,
    reason = "a failed fixture must stop the boundary proof"
)]

use std::fs;
use std::process::Command;

use conformance_backend::{Canned, Listener};
use thinkthen::{Engine, EngineBuilder, ErrorKind, Question};

const REFUSAL: &str = "the backend address contains the API key; keep the key out of the address";

#[test]
fn public_builder_checks_the_final_effective_key_before_local_effects() {
    for (key, base) in [
        ("key-path-0210", "http://localhost/key-path-0210"),
        ("systemone", "http://localhost"),
        ("秘密", "http://localhost/秘密"),
    ] {
        let builder = Engine::builder()
            .base_url(base)
            .expect("safe loopback base")
            .api_key(key)
            .expect("synthetic key")
            .no_cache();
        let debug = format!("{builder:?}");
        assert!(!debug.contains(base));
        assert!(!debug.contains(key));
        let error = builder.build().expect_err("effective key in final URL");
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(error.to_string(), REFUSAL);
        assert!(!format!("{error:?}").contains(key));
    }

    // The URL is compared literally. Its percent-encoded path is not decoded.
    Engine::builder()
        .base_url("http://localhost/%E7%A7%98%E5%AF%86")
        .expect("safe base")
        .api_key("秘密")
        .expect("synthetic key")
        .no_cache()
        .build()
        .expect("a different byte spelling is safe");
}

#[test]
fn captured_key_is_checked_and_an_explicit_key_overrides_it() {
    if std::env::var_os("THINKTHEN_KEY_ADDRESS_CHILD").is_some() {
        let base = "http://localhost/captured-key-0210";
        let refused = EngineBuilder::from_env()
            .expect("captured settings")
            .base_url(base)
            .expect("base")
            .no_cache()
            .build()
            .expect_err("captured key in address");
        assert_eq!(refused.kind(), ErrorKind::Usage);
        assert_eq!(refused.to_string(), REFUSAL);
        for explicit_first in [false, true] {
            let builder = EngineBuilder::from_env().expect("captured settings");
            let builder = if explicit_first {
                builder
                    .api_key("different-safe-key")
                    .expect("override")
                    .base_url(base)
                    .expect("base")
            } else {
                builder
                    .base_url(base)
                    .expect("base")
                    .api_key("different-safe-key")
                    .expect("override")
            };
            builder.no_cache().build().expect("final key is safe");
        }
        return;
    }
    let output = Command::new(std::env::current_exe().expect("test binary"))
        .args([
            "--exact",
            "captured_key_is_checked_and_an_explicit_key_overrides_it",
        ])
        .env_clear()
        .env(
            "HOME",
            std::env::temp_dir().join(format!("thinkthen-key-child-{}", std::process::id())),
        )
        .env("THINKTHEN_KEY_ADDRESS_CHILD", "1")
        .env("THINKTHEN_API_KEY", "captured-key-0210")
        .output()
        .expect("isolated child test");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn safe_gateway_url_keeps_result_identity_but_not_debug_text() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":1,"output_tokens":1}}"#)
    })
    .expect("loopback listener");
    let base = format!("{}/gateway-marker-0210", listener.base());
    let engine = Engine::builder()
        .base_url(&base)
        .expect("base")
        .api_key("different-key-0210")
        .expect("key")
        .no_cache()
        .build()
        .expect("safe gateway");
    let question = Question::decide("a benign question")
        .expect("question")
        .cut();
    let details = engine
        .details(&question, "benign evidence")
        .expect("details");
    let posting = format!("{base}/systemone");
    assert_eq!(details.value().url(), posting);
    assert!(details.value().to_json().contains(&posting));
    let debug = format!("{details:?}");
    assert!(debug.contains("url: \"<withheld>\""), "{debug}");
    assert!(!debug.contains("gateway-marker-0210"), "{debug}");
    assert_eq!(listener.count(), 1);
}

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
        vec!["check", "--url", &base, "--plan"],
        vec!["status", "--json"],
        vec!["status"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args(&args)
            .env_clear()
            .env("HOME", &home)
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
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command
            .args(["status", "--json"])
            .env_clear()
            .env("HOME", &home)
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
