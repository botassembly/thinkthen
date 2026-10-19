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
    let lines = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 2, "owned Plan graph and Rust serializer");
    assert_eq!(
        lines.first(),
        lines.last(),
        "Plan graph preserves exact Rust serialization"
    );
    let plan: serde_json::Value =
        serde_json::from_slice(lines.last().expect("Plan JSON")).expect("native plan JSON");
    assert_eq!(plan["records"], 1);
    assert_eq!(plan["requests"], 1);
    assert_eq!(plan["first_body_utf8"], BODY);
    assert_eq!(backend.count(), 0, "canonical preview sends nothing");
}

#[test]
fn retained_input_abi_refuses_invalid_carriers_and_owns_converted_values() {
    let backend = Backend::start().expect("a loopback backend");
    let base = format!("{}/generic/v1", backend.origin());
    for (source, variable) in [
        ("request_preview.c", "LEGACY_INPUT_ABI"),
        ("session.c", "SESSION_NAMED_REFUSALS"),
    ] {
        let output = super::run_with(
            &compile(&crate_dir().join("tests/c").join(source)),
            &base,
            b"",
            &[(variable, std::path::Path::new("1"))],
        );
        super::successful(&output);
        if source == "request_preview.c" {
            let lines: Vec<_> = output
                .stdout
                .split(|byte| *byte == b'\n')
                .filter(|line| !line.is_empty())
                .collect();
            assert_eq!(lines.len(), 3, "retained Plan representations");
            assert!(
                lines.iter().all(|line| Some(line) == lines.first()),
                "retained Plan owners preserve exact serialization after source destruction"
            );
        }
    }
    assert_eq!(
        backend.count(),
        0,
        "retained ABI refusals and preview send nothing"
    );
}
