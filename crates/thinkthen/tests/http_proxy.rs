#![cfg(test)]
//! Local plaintext credentials bypass proxies; TLS keeps ordinary proxy selection.

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use std::process::Command;

use loopback::{KEY, Loopback, Reply, UnavailableBackend};
use serde_json::json;
use thinkthen::{Decide, Key, Outcome, Record, Settings};

#[test]
fn local_plain_http_bypasses_environment_proxies() {
    for variable in ["HTTP_PROXY", "ALL_PROXY", "http_proxy", "all_proxy"] {
        let backend = Loopback::noul(0.9).unwrap();
        let proxy = Loopback::start(|_| Reply::json(503, &json!({}))).unwrap();
        let output = isolated_call(backend.base(), variable, proxy.base(), true);
        assert_eq!(proxy.count(), 0, "{variable} received a plaintext request");
        assert!(
            output.status.success(),
            "{variable}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(backend.count(), 1, "{variable}");
    }
}

#[test]
fn https_keeps_environment_proxy_selection() {
    let backend = UnavailableBackend::new().unwrap();
    let proxy = Loopback::start(|_| Reply::json(503, &json!({}))).unwrap();
    let url = backend.base().replacen("http:", "https:", 1);
    let output = isolated_call(&url, "HTTPS_PROXY", proxy.base(), false);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(proxy.count(), 1);
    assert!(proxy.requests()[0].path.starts_with("127.0.0.1:"));
}

fn isolated_call(url: &str, variable: &str, proxy: &str, answered: bool) -> std::process::Output {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command.env_clear();
    if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile);
    }
    command
        .args(["--exact", "proxy_child_call", "--nocapture"])
        .env(variable, proxy.trim_end_matches("/v1"))
        .env("THINKTHEN_TEST_URL", url)
        .env("THINKTHEN_TEST_ANSWERED", answered.to_string())
        .output()
        .unwrap()
}

#[test]
fn proxy_child_call() {
    let Ok(url) = std::env::var("THINKTHEN_TEST_URL") else {
        return;
    };
    let answered = std::env::var("THINKTHEN_TEST_ANSWERED").unwrap() == "true";
    let engine = Settings::default()
        .url(url)
        .api_key(Key::new(KEY).unwrap())
        .model("model")
        .engine(|_| None)
        .unwrap();
    let rows = engine
        .rows(&Decide::new("Ready?").unwrap(), [Record::text("evidence")])
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        matches!(rows[0].outcome(), Outcome::Answered { .. }),
        answered
    );
    assert_eq!(rows[0].error().is_none(), answered);
}
