//! Relation refusals after cached recognition answers.

use crate::harness::{Canned, Listener, spawn};
use serde_json::Value;
use std::{fs, path::PathBuf};

fn automatic(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let answers = request["questions"]
        .as_object()
        .expect("questions")
        .iter()
        .map(|(name, question)| {
            let answer = if question["type"] == "noul" {
                serde_json::json!({"type":"noul","noul":0.9})
            } else {
                let criteria = question["criteria"].as_object().expect("criteria");
                let instructions = question["instructions"].as_str().expect("instructions");
                let pick = if criteria.contains_key("IN") {
                    if instructions.contains("[[x]]") {
                        "OUT"
                    } else {
                        "IN"
                    }
                } else if criteria.contains_key("person") {
                    if instructions.contains("[[O") {
                        "organization"
                    } else {
                        "person"
                    }
                } else {
                    criteria
                        .keys()
                        .find(|label| label.starts_with('i'))
                        .map_or("none", String::as_str)
                };
                let probabilities = criteria
                    .keys()
                    .map(|label| {
                        (
                            label.clone(),
                            Value::from(f64::from(label.as_str() == pick)),
                        )
                    })
                    .collect::<serde_json::Map<_, _>>();
                serde_json::json!({"type":"choice","choice":pick,"probabilities":probabilities})
            };
            (name.clone(), answer)
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&serde_json::json!({"model":"local-1","answers":answers}).to_string())
}

#[test]
fn impossible_relation_state_uses_cached_recognition_and_sends_nothing() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-relation-refusal-cache");
    let _removed = fs::remove_dir_all(&root);
    let listener = Listener::answering(automatic).expect("listener");
    let cache = root.to_string_lossy();
    let common = [
        "recognize",
        "person",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &cache,
    ];
    let filled =
        spawn(&common, &[("THINKTHEN_API_KEY", "key")], b"Ada x Grace").expect("fill cache");
    assert_eq!(
        filled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&filled.stderr)
    );
    assert_eq!(listener.requests().len(), 1);

    let profile = root.with_extension("profile.json");
    fs::write(&profile, r#"{"schema":"thinkthen.backend-profile/1","name":"source-only","max_evidence_bytes":11,"max_options":2}"#).expect("profile");
    let refused = spawn(
        &[
            "recognize",
            "person",
            "--relation",
            "knows=person:person",
            "--profile",
            &profile.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--cache",
            &cache,
        ],
        &[],
        b"Ada x Grace",
    )
    .expect("refusal");
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    assert_eq!(listener.requests().len(), 0);
    assert!(!String::from_utf8_lossy(&refused.stderr).contains("key"));
}

#[test]
fn byte_fallback_refuses_oversized_h_after_cached_recognition_without_a_send() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-byte-h-refusal");
    let _removed = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("fixture root");
    let cache = root.join("cache");
    let one = root.join("one-question.json");
    fs::write(
        &one,
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    )
    .expect("one-question profile");
    let long = "z".repeat(1_200);
    let input = format!("P1{long} x P2{long} x O1{long} x O2{long}");
    let listener = Listener::answering(automatic).expect("listener");
    let cache_text = cache.to_string_lossy();
    let one_text = one.to_string_lossy();
    let base = [
        "recognize",
        "person",
        "organization",
        "--profile",
        &one_text,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &cache_text,
    ];
    let filled = spawn(
        &base,
        &[("THINKTHEN_API_KEY", "PRIVATE-KEY")],
        input.as_bytes(),
    )
    .expect("fill recognition cache");
    assert_eq!(
        filled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&filled.stderr)
    );
    let recognition = listener.requests();
    let limit = recognition
        .iter()
        .map(|request| request.body.len())
        .max()
        .expect("recognition requests");

    let mut choice_arguments = base.to_vec();
    choice_arguments.extend(["--relation", "works=person:organization"]);
    let choice = spawn(
        &choice_arguments,
        &[("THINKTHEN_API_KEY", "PRIVATE-KEY")],
        input.as_bytes(),
    )
    .expect("choice probe");
    assert_eq!(
        choice.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&choice.stderr)
    );
    let choice_requests = listener.requests();
    assert!(choice_requests.iter().all(|request| {
        serde_json::from_slice::<Value>(&request.body).expect("choice body")["questions"]
            .as_object()
            .expect("questions")
            .values()
            .all(|question| question["type"] == "choice")
    }));
    let choice_bytes = choice_requests.first().expect("choice request").body.len();

    let h_profile = root.join("h.json");
    fs::write(
        &h_profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"h","max_questions":1,"max_options":2}"#,
    )
    .expect("H profile");
    let h_text = h_profile.to_string_lossy();
    let h_arguments = [
        "recognize",
        "person",
        "organization",
        "--profile",
        &h_text,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &cache_text,
        "--relation",
        "works=person:organization",
    ];
    let h = spawn(
        &h_arguments,
        &[("THINKTHEN_API_KEY", "PRIVATE-KEY")],
        input.as_bytes(),
    )
    .expect("H probe");
    assert_eq!(h.status.code(), Some(0));
    let h_requests = listener.requests();
    assert!(h_requests.iter().all(|request| {
        serde_json::from_slice::<Value>(&request.body).expect("H body")["questions"]
            .as_object()
            .expect("questions")
            .values()
            .all(|question| question["type"] == "noul")
    }));
    let h_bytes = h_requests.first().expect("H request").body.len();
    assert!(choice_bytes > limit);
    assert!(h_bytes > limit);
    assert_ne!(choice_bytes, h_bytes);

    let final_profile = root.join("final.json");
    fs::write(&final_profile, format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"final-byte","max_questions":1,"max_request_bytes":{limit}}}"#)).expect("final profile");
    let final_text = final_profile.to_string_lossy();
    let final_arguments = [
        "recognize",
        "person",
        "organization",
        "--profile",
        &final_text,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &cache_text,
        "--relation",
        "works=person:organization",
    ];
    let refused = spawn(&final_arguments, &[], input.as_bytes()).expect("final refusal");
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    assert_eq!(
        String::from_utf8(refused.stderr).expect("safe stderr"),
        format!(
            "thinkthen: profile final-byte allows at most {limit} request bytes; this request has {h_bytes}\n"
        )
    );
    assert!(listener.requests().is_empty());
}
