//! Public conversion preserves pre-conversion identities and original exchange bodies.
use super::*;
use std::process::Command;

#[cfg(test)]
fn convert(place: &Path) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command.env_clear().args(["cache", "convert"]).arg(place);
    command.output().unwrap()
}

#[test]
fn explicit_original_bodies_survive_sqlite_to_fixture_conversion_and_zero_send_replay() {
    let response = r#"{ "model":"fixed", "answers":{"q1":{"type":"noul","noul":0.9,"unknown_probability":0.02,"abstained":false}},"usage":{"input_tokens":887} }"#;
    let listener = Listener::answering(move |_| Canned::ok(response)).unwrap();
    let place = folder();
    let engine = build(&listener).record(&place).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let live = engine.decide(&question, "Refund me.").unwrap();
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    assert_eq!(listener.count(), 1);
    drop(engine);
    let converted = convert(&place);
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
    let held = replay.decide(&question, "Refund me.").unwrap();
    assert_eq!(
        held.results()[0].identity().observations(),
        live.results()[0].identity().observations()
    );
    assert_eq!(
        held.results()[0].identity().question_sources()[0].batch_size(),
        Some(1)
    );
    assert_eq!(
        held.results()[0].reported_usage().unwrap().input_tokens(),
        Some(887)
    );
    assert_eq!(
        held.results()[0].reported_usage().unwrap().output_tokens(),
        None
    );
    assert_eq!(listener.count(), 1);
    drop(replay);
    std::fs::remove_dir_all(place).unwrap();
}
