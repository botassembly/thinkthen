//! The exact detailed relate result: entry unions, cuts, digests, keys, the
//! planned request, and exit 6.

use serde_json::Value;

use super::{WRONG, answered, plan_json, run, scripted};
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

#[test]
fn dry_run_reports_the_exact_request_a_real_run_sends() {
    let listener = Listener::answering(answered).expect("listener");
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Paris","kind":"place"},{"name":"Acme","kind":"organization"}]"#;
    let output = run(
        &listener,
        &["works_for=person:organization", "--plan"],
        input,
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.connections(), 0);
    let sent = run(
        &listener,
        &["works_for=person:organization", "--details"],
        input,
    );
    assert_eq!(
        sent.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&sent.stderr)
    );
    let sent: Value = serde_json::from_slice(&sent.stdout).expect("details");
    let received = listener.requests();
    let [request] = received.as_slice() else {
        panic!("{} requests", received.len());
    };
    let url = format!("{}/systemone", listener.base());
    assert_eq!(
        sent["meta"]["requests"],
        Value::from(crate::support::keys(&url, &request.body))
    );
    let plan = plan_json(&output);
    assert_eq!(plan["schema"], "thinkthen.relate-plan/1");
    assert_eq!(plan["entity_count"], 3);
    assert_eq!(plan["logical_questions"], 1);
    assert_eq!(plan["request_count"], 1);
    assert_eq!(
        plan["relations"],
        serde_json::json!([{
            "name":"works_for", "source":"person", "target":"organization", "reads":"works for",
            "either":false, "method":"yes_no", "fallback":null,
            "logical_questions":1, "request_count":1
        }])
    );
    assert_eq!(
        plan["requests"][0]["digest"],
        crate::support::digest(&url, &request.body)
    );
    let body = r#"{"state":{"entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Acme","kind":"organization"}]},"model":"local-1","questions":{"q1":{"type":"noul","instructions":"Is it true that i1 works for i2?"}}}"#;
    assert_eq!(plan["requests"][0]["body_utf8"], body);
    assert_eq!(request.body, body.as_bytes());
    assert_eq!(plan["requests"][0]["bytes"], body.len());
}
