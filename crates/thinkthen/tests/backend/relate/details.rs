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
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let text = String::from_utf8(output.stdout).expect("details text");
    assert!(
        text.contains(r#""question":{"verb":"relate","fields":null,"relations":[{"name":"linked","source":"*","target":"*","reads":"linked","either":true}],"threshold":0.5},"#),
        "{text}"
    );
    let result: Value = serde_json::from_str(&text).expect("details");
    assert_eq!(result["meta"]["question_sha256"], "5d1aa1f27838358f922a082922c989994bb79461f6b6e694be95b5a3577af794");
}

#[test]
fn successful_and_failed_choice_entries_keep_the_exact_option_a_shape() {
    let listener = scripted(&[
        r#"{"type":"choice","choice":"i3","probabilities":{"i3":1.0,"none":0.0}}"#,
        WRONG,
    ]);
    let output = run(
        &listener,
        &["works_for=person:organization", "--details"],
        br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"},{"name":"Acme","kind":"organization"}]"#,
    );
    assert_eq!(output.status.code(), Some(6));
    let text = String::from_utf8(output.stdout).expect("details text");
    let result: Value = serde_json::from_str(&text).expect("details");
    let digest = result["meta"]["requests"][0].as_str().expect("digest");
    let successful = format!(
        r#"{{"relation":"works_for","reads":"works for","method":"choice","direction":"source_to_target","asker":{{"role":"source","entity":{{"name":"Ada","kind":"person"}}}},"candidates":[{{"role":"target","entity":{{"name":"Acme","kind":"organization"}},"probability":1.0,"accepted":true}},{{"none":true,"probability":0.0,"accepted":false}}],"pick":{{"role":"target","entity":{{"name":"Acme","kind":"organization"}}}},"request":"{digest}"}}"#
    );
    let failed = format!(
        r#"{{"relation":"works_for","reads":"works for","method":"choice","direction":"source_to_target","asker":{{"role":"source","entity":{{"name":"Grace","kind":"person"}}}},"candidates":[{{"role":"target","entity":{{"name":"Acme","kind":"organization"}}}},{{"none":true}}],"failure":{{"kind":"backend","cause":"missing_probability"}},"request":"{digest}"}}"#
    );
    assert!(text.contains(&successful), "{text}");
    assert!(text.contains(&failed), "{text}");
    assert!(
        text.starts_with(r#"{"schema":"thinkthen.result/1","value":[{"relation":"works_for","source":{"name":"Ada","kind":"person"},"target":{"name":"Acme","kind":"organization"},"probability":1.0}],"#),
        "{text}"
    );
}

#[test]
fn a_target_side_asker_keeps_its_roles_and_the_declared_edge_direction() {
    let listener = Listener::answering(answered).expect("listener");
    let output = run(
        &listener,
        &["works_for=person:organization", "--details"],
        br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"},{"name":"Beta","kind":"organization"}]"#,
    );
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let result: Value = serde_json::from_slice(&output.stdout).expect("details");
    let ada = serde_json::json!({"name":"Ada","kind":"person"});
    let questions = result["answer"]["questions"].as_array().expect("questions");
    assert_eq!(questions.len(), 2);
    for (question, asker) in questions.iter().zip(["Acme", "Beta"]) {
        assert_eq!(question["direction"], "source_to_target");
        assert_eq!(
            question["asker"],
            serde_json::json!({"role":"target","entity":{"name":asker,"kind":"organization"}})
        );
        assert_eq!(question["candidates"][0]["role"], "source");
        assert_eq!(question["candidates"][0]["entity"], ada);
        assert_eq!(question["pick"], serde_json::json!({"role":"source","entity":ada}));
    }
    let edges = result["value"].as_array().expect("edges");
    assert_eq!(edges.len(), 2);
    for (edge, target) in edges.iter().zip(["Acme", "Beta"]) {
        assert_eq!(edge["source"], ada);
        assert_eq!(edge["target"]["name"], target);
    }
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
