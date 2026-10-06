//! Public conversion preserves pre-conversion identities and original exchange bodies.
use super::*;
use std::process::Command;

#[cfg(test)]
fn convert(place: &Path, quote: bool) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command.env_clear().args(["cache", "convert"]).arg(place);
    if quote {
        command.arg("--quote");
    }
    command.output().unwrap()
}

#[test]
fn old_exchange_identity_precedes_generated_origin_time_quoting_and_usage_shares() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let url = format!("{}/systemone", listener.base());
    let request = r#"{"state":"Refund me.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"Refund?"}}}"#;
    let response = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":887}}"#;
    let exchange: String = Sha256::digest(format!("systemone\n{url}\n{request}"))
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let envelope = format!(
        r#"{{"schema":"thinkthen.recording/1","adapter":"systemone","url":"{url}","request":{request},"response":{response}}}"#
    );
    std::fs::write(place.join(format!("{exchange}.json")), envelope).unwrap();
    let original_key: String = Sha256::digest(format!("systemone\n{url}\n\"fixed\"\n\"Refund me.\"\n{{\"type\":\"noul\",\"instructions\":\"Refund?\"}}")).iter().map(|byte|format!("{byte:02x}")).collect();
    let expected = framed(
        "thinkthen.legacy-observation/1",
        &[
            &original_key,
            r#"{"kind":"yes_no","probability":0.9}"#,
            r#"{"answered_by":"fixed","input_tokens":887}"#,
        ],
    );
    let converted = convert(&place, true);
    assert!(
        converted.status.success(),
        "{}",
        String::from_utf8_lossy(&converted.stderr)
    );
    let fixture = std::fs::read(place.join("thinkthen.jsonl")).unwrap();
    let answers: Vec<Value> = std::str::from_utf8(&fixture)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|line| line.get("key").is_some())
        .collect();
    assert_eq!(answers.len(), 2);
    for answer in answers {
        assert_eq!(answer["observation_id"], expected);
        assert_eq!(answer["taken_at"], 0);
        assert!(answer.get("batch_size").is_none());
    }
    assert!(convert(&place, true).status.success());
    assert_eq!(
        std::fs::read(place.join("thinkthen.jsonl")).unwrap(),
        fixture
    );
    let replay = build(&listener).replay(&place).unwrap().build().unwrap();
    let held = replay
        .details(&Question::decide("Refund?").unwrap().cut(), "Refund me.")
        .unwrap();
    assert_eq!(
        held.value().observations(),
        [Observation::Answered {
            observation_id: expected.parse().unwrap()
        }]
    );
    assert_eq!(listener.count(), 0);
    drop(replay);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn explicit_original_bodies_survive_sqlite_to_fixture_conversion_and_zero_send_replay() {
    let response = r#"{ "model":"fixed", "answers":{"q1":{"type":"noul","noul":0.9,"unknown_probability":0.02,"abstained":false}},"usage":{"input_tokens":887} }"#;
    let listener = Listener::answering(move |_| Canned::ok(response)).unwrap();
    let place = folder();
    let engine = build(&listener).record(&place).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let live = engine.details(&question, "Refund me.").unwrap();
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    assert_eq!(listener.count(), 1);
    drop(engine);
    let converted = convert(&place, false);
    assert!(
        converted.status.success(),
        "{}",
        String::from_utf8_lossy(&converted.stderr)
    );
    assert!(!place.join("thinkthen.sqlite").exists());
    let fixture = std::fs::read_to_string(place.join("thinkthen.jsonl")).unwrap();
    let original: Value = fixture
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .find(|line| line.get("exchange_sha256").is_some())
        .unwrap();
    assert_eq!(
        original["request"],
        std::str::from_utf8(&sent[0].body).unwrap()
    );
    assert_eq!(original["response"], response);
    assert!(!fixture.contains("native-store-private"));
    assert!(!fixture.contains("sdk_request_id"));
    assert!(!fixture.contains("call_id"));
    let replay = build(&listener).replay(&place).unwrap().build().unwrap();
    let held = replay.details(&question, "Refund me.").unwrap();
    assert_eq!(held.value().observations(), live.value().observations());
    assert_eq!(held.value().question_sources()[0].batch_size(), Some(1));
    assert_eq!(
        held.value().reported_usage().unwrap().input_tokens(),
        Some(887)
    );
    assert_eq!(held.value().reported_usage().unwrap().output_tokens(), None);
    assert_eq!(listener.count(), 1);
    drop(replay);
    std::fs::remove_dir_all(place).unwrap();
}
