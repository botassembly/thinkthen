//! Canonical Request preview owns its JSON and sends nothing.

use crate::child::ChildEnvironment as _;
use std::process::Command;

use conformance_backend::Backend;

use crate::{compile, crate_dir, scratch};

/// The P1 body `databases/duckdb/tools/plan_suite.py` pins independently.
const BODY: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me please.\". asks for a refund"}}}"#;

#[test]
fn canonical_request_preview_owns_bytes_and_sends_nothing() {
    let backend = Backend::start().expect("a loopback backend");
    let output = Command::new(compile(&crate_dir().join("tests/c/request_preview.c")))
        .clear_environment()
        .isolated_home(scratch("canonical-preview-home"))
        .current_dir(scratch("canonical-preview-work"))
        .env(
            "THINKTHEN_BASE_URL",
            format!("{}/generic/v1", backend.origin()),
        )
        .env("THINKTHEN_CACHE", scratch("canonical-preview-cache"))
        .env("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=0")
        .output()
        .expect("the canonical preview program ran");
    super::successful(&output);
    let plan: serde_json::Value = serde_json::from_slice(&output.stdout).expect("native plan JSON");
    assert_eq!(plan["records"], 1);
    assert_eq!(plan["requests"], 1);
    assert_eq!(plan["first_body_utf8"], BODY);
    assert_eq!(backend.count(), 0, "canonical preview sends nothing");
}
