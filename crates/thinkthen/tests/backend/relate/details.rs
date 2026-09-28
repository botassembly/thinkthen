//! The exact detailed relate result: entry unions, cuts, digests, and exit 6.

use serde_json::Value;

use super::{WRONG, answered, run, scripted};
use crate::harness::Listener;

#[test]
fn an_h_entry_at_the_cut_is_accepted_and_bare_partial_output_exits_six() {
    let listener = scripted(&[r#"{"type":"noul","noul":0.5}"#, WRONG]);
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"}]"#;
    let details = run(&listener, &["follows=person:person", "--details"], input);
    assert_eq!(details.status.code(), Some(6));
    let text = String::from_utf8(details.stdout).expect("details text");
    let result: Value = serde_json::from_str(&text).expect("details");
    let digest = result["meta"]["requests"][0].as_str().expect("digest");
    let entry = format!(
        concat!(
            r#"{{"relation":"follows","reads":"follows","method":"yes_no","#,
            r#""direction":"source_to_target","source":{{"name":"Ada","kind":"person"}},"#,
            r#""target":{{"name":"Grace","kind":"person"}},"probability":0.5,"accepted":true,"#,
            r#""request":"{}"}}"#
        ),
        digest
    );
    assert!(text.contains(&entry), "{text}");
    let bare = run(&listener, &["follows=person:person"], input);
    assert_eq!(bare.status.code(), Some(6));
    assert_eq!(
        String::from_utf8_lossy(&bare.stdout),
        concat!(
            r#"{"relation":"follows","source":{"name":"Ada","kind":"person"},"#,
            r#""target":{"name":"Grace","kind":"person"},"probability":0.5}"#,
            "\n"
        )
    );
}

#[test]
fn mixed_logical_failure_prints_details_and_exits_six() {
    let listener = scripted(&[r#"{"type":"noul","noul":0.9}"#, WRONG]);
    let output = run(
        &listener,
        &["follows=person:person", "--details"],
        br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"}]"#,
    );
    assert_eq!(
        output.status.code(),
        Some(6),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout).expect("details");
    assert_eq!(result["schema"], "thinkthen.result/1");
    assert_eq!(result["value"].as_array().expect("edges").len(), 1);
    assert_eq!(
        result["answer"]["questions"]
            .as_array()
            .expect("questions")
            .len(),
        2
    );
    assert_eq!(result["meta"]["failed_questions"], 1);
}

#[test]
fn the_detailed_question_digest_hashes_the_printed_line_question() {
    let listener = Listener::answering(answered).expect("listener");
    let output = run(
        &listener,
        &["linked", "--either", "--lines", "--details"],
        b"Ada\nGrace\n",
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).expect("details text");
    assert!(
        text.contains(r#""question":{"verb":"relate","fields":null,"relations":[{"name":"linked","source":"*","target":"*","reads":"linked","either":true}],"threshold":0.5},"#),
        "{text}"
    );
    let result: Value = serde_json::from_str(&text).expect("details");
    assert_eq!(
        result["meta"]["question_sha256"],
        "5d1aa1f27838358f922a082922c989994bb79461f6b6e694be95b5a3577af794"
    );
}

#[test]
fn no_valid_logical_answer_exits_four_without_output() {
    let listener = scripted(&[WRONG]);
    let output = run(
        &listener,
        &["follows=person:person", "--details"],
        br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"}]"#,
    );
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
}
