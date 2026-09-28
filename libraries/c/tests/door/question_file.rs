//! A real named C question on a captured arm, plus bounded local refusals.

use conformance_backend::Backend;
use serde_json::Value;

use crate::cases::{Script, replies};
use crate::{compile, crate_dir, run, scratch, text};

const CASES: &str = include_str!("../../../../conformance/cases.json");

#[test]
fn named_file_keeps_the_captured_request_and_refuses_bad_files_without_sends() {
    let cases: Value = serde_json::from_str(CASES).expect("the shared corpus");
    let one = cases["cases"]
        .as_array()
        .and_then(|all| {
            all.iter()
                .find(|case| case["id"] == "01-decide-yes-captured")
        })
        .expect("the captured decide case");
    let folder = scratch("question-file-boundary");
    let valid = folder.join("valid.json");
    std::fs::write(&valid, one["question"].to_string()).expect("the question file");
    let malformed = folder.join("malformed.json");
    std::fs::write(&malformed, r#"{"decide":"   "}"#).expect("the bad question file");
    let too_large = folder.join("too-large.json");
    std::fs::write(&too_large, vec![b'x'; 1_048_577]).expect("the large question file");
    let invalid_utf8 = folder.join("utf8.json");
    std::fs::write(&invalid_utf8, [0xff]).expect("the invalid UTF-8 question file");
    let private_marker = "SYNTHETIC_PRIVATE_MARKER_0244";
    let unknown_key = folder.join("unknown-key.json");
    std::fs::write(
        &unknown_key,
        format!(r#"{{"decide":"Question?","{private_marker}":1}}"#),
    )
    .expect("the private-key question file");
    let wrong_verb = folder.join("choose.json");
    std::fs::write(&wrong_verb, r#"{"choose":"Which?","options":["a","b"]}"#)
        .expect("the other question file");
    let missing = folder.join("missing.json");

    let backend = Backend::start().expect("the loopback backend");
    let base = format!("{}/case/01-decide-yes-captured/v1", backend.origin());
    let evidence = one["exchanges"][0]["evidence"].as_str().expect("evidence");
    let mut script = Script::default();
    script.ask("file", &[&base, &valid.to_string_lossy(), evidence]);
    for bad in [
        &missing,
        &malformed,
        &too_large,
        &invalid_utf8,
        &unknown_key,
    ] {
        script.ask("file_probe", &[&base, &bad.to_string_lossy()]);
    }
    script.ask("file_probe", &[&base, ""]);
    script.ask("file", &[&base, &wrong_verb.to_string_lossy(), evidence]);
    let output = run(
        &compile(&crate_dir().join("tests/c/driver.c")),
        "",
        &script.0,
    );
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    assert_eq!(got.len(), 8);
    assert_eq!(got[0].0, 0, "the valid file failed: {:?}", got[0]);
    let (outcome, probability) = got[0].1.split_once(' ').expect("one judgment");
    assert_eq!(outcome, "1");
    assert_eq!(probability.parse::<f64>().expect("a probability"), 0.99);
    for (index, refused) in got[1..].iter().enumerate() {
        assert_eq!(
            refused.0,
            if index >= 5 { 1 } else { 4 },
            "a file or argument failure has its own kind: {refused:?}"
        );
        assert!(
            refused.1.starts_with("0 "),
            "a file failure is non-retryable: {refused:?}"
        );
        assert!(!refused.1.contains(&folder.to_string_lossy().to_string()));
        assert!(!refused.1.contains(private_marker));
    }
    assert!(got[5].1.contains("invalid question content"));
    assert_eq!(backend.count(), 1, "only the captured valid question sends");
}
