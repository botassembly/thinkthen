//! Genuine command TLS trust and refusal ordering through the installed selection.
#![allow(clippy::expect_used, clippy::panic, reason = "a failed local certificate fixture stops the boundary proof")]
use crate::child::ChildEnvironment as _;
use std::process::Command;
use std::io::Write;
use std::net::TcpListener;
use std::path::Path;
use std::process::Stdio;
#[path = "../../thinkthen/src/test_deadline/child.rs"]
mod child;
#[path = "../../thinkthen/tests/support/tls.rs"]
mod tls;
use tls::{KEY, RESPONSE, Fixture, Server};

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

