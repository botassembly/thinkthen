//! Ticket 0291's C bridge: `thinkthen_plan_json` previews corpus P1 with no
//! key and no send, and every refusal keeps the out parameters.

use crate::child::ChildEnvironment as _;
use std::process::Command;

use conformance_backend::Backend;

use crate::{compile, crate_dir, scratch, text};

/// The P1 body `databases/duckdb/tools/plan_suite.py` pins independently.
const BODY: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me please.\". asks for a refund"}}}"#;

#[test]
fn p1_plans_with_no_key_and_no_send_and_refusals_keep_the_outputs() {
    assert_eq!(BODY.len(), 182, "the independent P1 byte count");
    let backend = Backend::start().expect("a loopback backend");
    let output = Command::new(compile(&crate_dir().join("tests/c/plan.c")))
        .clear_environment()
        .env(
            "THINKTHEN_BASE_URL",
            format!("{}/generic/v1", backend.origin()),
        )
        .env("THINKTHEN_CACHE", scratch("plan-cache"))
        .env("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=0")
        .output()
        .expect("the plan program ran");
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    // The exact bytes, in the crate's field order.
    let body = serde_json::to_string(BODY).expect("the body as a JSON string");
    let expected = format!(
        r#"{{"records":1,"requests":1,"estimated_bytes":182,"largest_request_bytes":182,"largest_request_estimated_input_tokens":166,"token_estimate_method":"encoded-body-bytes-908-v1","estimated_input_tokens":{{"lower":93,"upper":166}},"upper_bound":false,"first_body_utf8":{body}}}"#
    );
    assert_eq!(text(&output.stdout), format!("{expected}\n"));
    assert_eq!(backend.count(), 0, "a plan sent a request");
}

#[test]
fn canonical_request_preview_owns_bytes_and_sends_nothing() {
    let backend = Backend::start().expect("a loopback backend");
    let output = Command::new(compile(&crate_dir().join("tests/c/request_preview.c")))
        .clear_environment()
        .env(
            "THINKTHEN_BASE_URL",
            format!("{}/generic/v1", backend.origin()),
        )
        .env("THINKTHEN_CACHE", scratch("canonical-preview-cache"))
        .env("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=0")
        .output()
        .expect("the canonical preview program ran");
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    let plan: serde_json::Value = serde_json::from_slice(&output.stdout).expect("native plan JSON");
    assert_eq!(plan["records"], 1);
    assert_eq!(plan["requests"], 1);
    assert_eq!(plan["first_body_utf8"], BODY);
    assert_eq!(backend.count(), 0, "canonical preview sends nothing");
}
