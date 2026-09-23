//! The public probability-total diagnostic and its secrecy boundary.

use crate::harness::{Canned, Listener, spawn};
use crate::secrecy::{EVIDENCE, KEY, QUESTION, nothing_leaked};
use std::fs;

#[test]
fn a_distribution_measurement_repeats_no_untrusted_text() {
    let label = "private-label-marker";
    let response = concat!(
        r#"{"model":"jev-1.13.0","private":"private-response-marker","#,
        r#""answers":{"q1":{"type":"choice","probabilities":{"LABEL":0.4,"other":0.4}}}}"#,
    )
    .replace("LABEL", label);
    let listener = Listener::serving(vec![Canned::ok(&response)]).expect("a listener");
    let into = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("distribution-total");
    let _absent = fs::remove_dir_all(&into);
    fs::create_dir_all(&into).expect("a test folder");
    let output = spawn(
        &["choose", QUESTION, label, "other", "--url", listener.base()],
        &[("THINKTHEN_API_KEY", KEY)],
        EVIDENCE.as_bytes(),
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the reply was refused: the answer to question `q1` has probability total 0.8, member count 2, and tolerance 0.01; the total differs from one by more than the tolerance\n"
    );
    let said = String::from_utf8_lossy(&output.stderr);
    for marker in [label, "0.4", "private-response-marker"] {
        assert!(!said.contains(marker), "the diagnostic repeated {marker}");
    }
    nothing_leaked("distribution measurement", &output, &into);
}
