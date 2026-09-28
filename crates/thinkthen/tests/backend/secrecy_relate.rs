//! Relate's one route the shared sweep cannot reach: a mixed logical failure.
//!
//! Every other backend, recording, cache, and refusal route runs over relate in
//! `secrecy.rs` and `refusals.rs`.

use crate::harness::spawn;
use crate::relate::{WRONG, scripted};
use crate::secrecy::{EVIDENCE, SECOND, environment, folder, nothing_leaked};

#[test]
fn a_partial_relation_answer_prints_and_exits_six_without_quoting_evidence() {
    let into = folder("relate-partial").expect("folder");
    let listener = scripted(&[r#"{"type":"noul","noul":0.9}"#, WRONG]);
    let output = spawn(
        &[
            "relate",
            "linked=person:person",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
            "--details",
        ],
        &environment(true),
        format!(
            r#"[{{"name":"{EVIDENCE}","kind":"person"}},{{"name":"{SECOND}","kind":"person"}}]"#
        )
        .as_bytes(),
    )
    .expect("partial run");
    assert_eq!(output.status.code(), Some(6));
    nothing_leaked("relate partial", &output, &into);
    assert!(!output.stdout.is_empty());
    assert_eq!(listener.connections(), 1);
}
