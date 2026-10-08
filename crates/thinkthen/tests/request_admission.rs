//! Refused CLI headers do not open question files, enumerate source folders or send.
#![cfg(all(feature = "cli", target_os = "linux"))]
#[path = "../src/test_deadline/child.rs"]
mod child;
use child::ChildEnvironment as _;
use conformance_backend::{Canned, Listener};
use std::process::Command;

#[test]
fn invalid_headers_leave_watched_question_and_source_authority_unread() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let output = Command::new("python3")
        .clear_environment()
        .arg("-c")
        .arg(include_str!("request_admission.py"))
        .arg(env!("CARGO_BIN_EXE_thinkthen"))
        .arg(listener.base())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "question/source access events: 0"
    );
    assert_eq!(listener.count(), 0);
}
