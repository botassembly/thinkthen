//! Genuine command TLS trust and refusal ordering through the installed selection.
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed local certificate fixture stops the boundary proof"
)]
use crate::child::ChildEnvironment as _;
use std::io::Write;
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
#[path = "../../thinkthen/src/test_deadline/child.rs"]
mod child;
#[path = "../../thinkthen/tests/support/tls.rs"]
mod tls;
use tls::{Fixture, KEY, RESPONSE, Server};

#[path = "support/command.rs"]
mod command;

fn command(url: &str, ca: Option<&Path>, home: &Path, key: Option<&str>) -> std::process::Output {
    let mut child = Command::new(crate::command::binary());
    child
        .args([
            "decide",
            "Does this ask for a refund?",
            "--url",
            url,
            "--model",
            "local-1",
            "--no-cache",
        ])
        .clear_environment()
        .home(home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(key) = key {
        child.env("THINKTHEN_API_KEY", key);
    }
    if let Some(ca) = ca {
        child.env("THINKTHEN_CA_BUNDLE", ca);
    }
    let mut child = child.spawn().expect("compiled command");
    child
        .stdin
        .take()
        .expect("command input")
        .write_all(b"Please refund this.\n")
        .expect("evidence input");
    child.wait_with_output().expect("command result")
}

#[test]
fn genuine_tls_trust_replaces_default_and_keeps_hostname_verification() {
    let fixture = Fixture::new();
    for (leaf, bundle, success) in [
        ("localhost", None, false),
        ("localhost", Some("ca.pem"), true),
        ("localhost", Some("wrong.pem"), false),
        ("other-host", Some("ca.pem"), false),
    ] {
        let server = Server::start(leaf, RESPONSE);
        let output = command(
            &server.url,
            bundle.map(|name| fixture.at(name)).as_deref(),
            &fixture.0,
            Some(KEY),
        );
        let requests = server.finish();
        let said = String::from_utf8_lossy(&output.stderr);
        if success {
            assert_eq!(output.status.code(), Some(0), "{said}");
            assert_eq!(output.stdout, b"true\n");
            assert_eq!(
                requests.matches("POST /systemone").count(),
                1,
                "{requests:?}"
            );
        } else {
            assert_eq!(output.status.code(), Some(4), "{said}");
            assert!(
                said.contains("the TLS connection or certificate check failed"),
                "{said}"
            );
            assert!(!requests.contains("POST /systemone"), "{requests}");
        }
        assert!(!said.contains(KEY), "{said}");
    }
}

#[test]
fn bounded_bundle_refusals_precede_send_and_replay_folder_work() {
    let fixture = Fixture::new();
    let base = "https://localhost:1";
    let missing = fixture.at("missing.pem");
    let output = Command::new(crate::command::binary())
        .args(["backends", "check", "--url", base, "--plan"])
        .clear_environment()
        .home(&fixture.0)
        .env("THINKTHEN_CA_BUNDLE", &missing)
        .output()
        .expect("compiled check");
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("THINKTHEN_CA_BUNDLE"));

    // This is an ordinary live decide path with no key, not a dry-run path.
    // The invalid bundle must win before either the key reader or a socket.
    let listener = TcpListener::bind("127.0.0.1:0").expect("counted local socket");
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    let url = format!(
        "https://127.0.0.1:{}",
        listener.local_addr().expect("address").port()
    );
    let output = command(&url, Some(&missing), &fixture.0, None);
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("THINKTHEN_CA_BUNDLE"));
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}

#[cfg(unix)]
#[test]
fn certificate_admission_precedes_cache_and_cost_warnings() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt as _;
    const TRUST: &str = "thinkthen: warning: another user may change this named cache or recording folder; its writers decide the answers read from it\n";
    const COST: &str = "thinkthen: warning: a mutable model alias or --refresh-cache sends each planned cache request live and may incur a charge\n";
    const NO_KEY: &str = "thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent\n";
    let fixture = Fixture::new();
    let cache = fixture.at("shared-cache");
    fs::create_dir(&cache).unwrap();
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o707)).unwrap();
    let malformed = fixture.at("malformed.pem");
    fs::write(&malformed, b"distinctive-private-certificate-marker").unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let url = format!(
        "https://127.0.0.1:{}",
        listener.local_addr().unwrap().port()
    );
    let run = |bundle: &Path, cache: &Path, model: &str, refresh: bool| {
        let mut command = Command::new(crate::command::binary());
        command
            .args([
                "decide", "Fits?", "--url", &url, "--model", model, "--cache",
            ])
            .arg(cache)
            .clear_environment()
            .home(&fixture.0)
            .env("THINKTHEN_CA_BUNDLE", bundle)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if refresh {
            command.arg("--refresh-cache");
        }
        let mut child = command.spawn().unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"Evidence.\n")
            .unwrap();
        child.wait_with_output().unwrap()
    };
    for (bundle, code, sentence) in [
        (
            fixture.at("missing.pem"),
            5,
            "THINKTHEN_CA_BUNDLE file could not be read",
        ),
        (
            malformed,
            2,
            "THINKTHEN_CA_BUNDLE accepts only PEM CERTIFICATE blocks",
        ),
    ] {
        let output = run(&bundle, &cache, "jev-latest", true);
        assert_eq!(output.status.code(), Some(code));
        assert!(output.stdout.is_empty());
        assert_eq!(output.stderr, format!("thinkthen: {sentence}\n").as_bytes());
        assert_eq!(fs::read_dir(&cache).unwrap().count(), 0);
    }
    let output = run(&fixture.at("ca.pem"), &cache, "jev-latest", true);
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, format!("{TRUST}{COST}{NO_KEY}").as_bytes());
    let output = run(&fixture.at("ca.pem"), &cache, "fixed-model", false);
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(output.stderr, format!("{TRUST}{NO_KEY}").as_bytes());

    let blocked = fixture.at("cache-is-file");
    fs::write(&blocked, b"unchanged storage marker").unwrap();
    let output = run(&fixture.at("ca.pem"), &blocked, "jev-latest", true);
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.starts_with(COST), "{stderr}");
    assert!(stderr[COST.len()..].starts_with("thinkthen: "), "{stderr}");
    assert_eq!(fs::read(&blocked).unwrap(), b"unchanged storage marker");
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}
