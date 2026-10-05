//! Retain scalar cancellation and the historical explicit-four chunk boundary.

use std::fs;
use std::path::Path;

use super::held;
use crate::harness::Canned;

const RECOGNIZED: &str = r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"IN","probabilities":{"IN":1.0,"OUT":0.0}}}}"#;
// The former single phrase makes seven BILOU chunks. Two copies retain its
// evidence and provide the fourteen one-question chunks required by 0400.
const TEXT: &[u8] = b"Ada met Bob at Acme in Paris Ada met Bob at Acme in Paris";

fn interrupted_chunks(count: usize, framing: &[&str], stderr: &str) {
    let profile = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("recognize-interrupt-profile-{count}.json"));
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    )
    .expect("profile");
    let profile_name = profile.to_string_lossy();
    let arguments = [
        &["recognize", "--profile", &profile_name, "--no-cache"],
        framing,
    ]
    .concat();
    // This single input line makes 14 one-question chunks in either framing.
    // held owns the signal, count+1 rendezvous and exact final request count.
    let output = held(count, &arguments, TEXT, || Canned::ok(RECOGNIZED)).expect("recognize stops");
    assert!(output.stdout.is_empty());
    assert_eq!(String::from_utf8_lossy(&output.stderr), stderr);
}

/// Ticket 0143/0400: scalar omission holds eight chunks and starts no later one.
#[test]
fn sigint_between_recognition_chunks_starts_no_later_chunk() {
    interrupted_chunks(8, &[], "");
}

/// Existing record mode admits explicit four without changing scalar admission.
#[test]
fn sigint_between_record_recognition_chunks_starts_no_later_chunk() {
    interrupted_chunks(
        4,
        &["--lines", "--jobs", "4"],
        "thinkthen: stopped by a signal; 0 records finished\n",
    );
}
