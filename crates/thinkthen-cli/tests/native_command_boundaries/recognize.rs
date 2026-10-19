use super::{child, folder};
use conformance_backend::Listener;
use serde_json::Value;
#[path = "../../../thinkthen/tests/support/recognition_proposals.rs"]
mod fixture;
use fixture::response;

#[test]
fn cli_details_show_the_same_proposal_facts_without_changing_bare_output_or_replay_keys() {
    use super::child::ChildEnvironment as _;
    let listener = Listener::answering(response).unwrap();
    let home = folder().unwrap();
    let input = home.join("input.jsonl");
    std::fs::write(
        &input,
        r#"{"text":"!! Ada Bob Åda","seeds":[{"start":11,"end":14}]}"#,
    )
    .unwrap();
    let recording = home.join("recording");
    let mut command = child::command(env!("CARGO_BIN_EXE_thinkthen"), &[]);
    command
        .isolated_home(&home)
        .env("THINKTHEN_API_KEY", "fake")
        .args([
            "recognize",
            "person",
            "--model",
            "fixed",
            "--url",
            listener.base(),
            "--jsonl",
            "--field",
            "/text",
            "--seed-spans-field",
            "/seeds",
            "--input",
        ])
        .arg(&input)
        .arg("--no-cache")
        .arg("--record")
        .arg(&recording)
        .arg("--details");
    let details = child::run::output(&mut command).unwrap();
    assert!(
        details.status.success(),
        "{}",
        String::from_utf8_lossy(&details.stderr)
    );
    let document: Value = serde_json::from_slice(&details.stdout).unwrap();
    assert_eq!(document["answer"]["proposals"].as_array().unwrap().len(), 5);
    assert_eq!(document["answer"]["proposals"][1]["kept"], true);
    assert_eq!(document["answer"]["proposals"][4]["strength"], 0.36);
    assert!(document["answer"]["proposals"][2].get("strength").is_none());
    let before = std::fs::read_dir(&recording)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    let mut replay = child::command(env!("CARGO_BIN_EXE_thinkthen"), &[]);
    replay
        .isolated_home(&home)
        .args([
            "recognize",
            "person",
            "--model",
            "fixed",
            "--url",
            listener.base(),
            "--jsonl",
            "--field",
            "/text",
            "--seed-spans-field",
            "/seeds",
            "--input",
        ])
        .arg(&input)
        .arg("--no-cache")
        .arg("--replay")
        .arg(&recording);
    let bare = child::run::output(&mut replay).unwrap();
    assert!(
        bare.status.success(),
        "{}",
        String::from_utf8_lossy(&bare.stderr)
    );
    let bare: Value = serde_json::from_slice(&bare.stdout).unwrap();
    assert_eq!(bare["value"], document["value"]);
    assert!(bare.get("answer").is_none());
    assert_eq!(listener.count(), 2);
    assert_eq!(listener.questions(), 12);
    assert_eq!(
        before,
        std::fs::read_dir(&recording)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect::<Vec<_>>()
    );
    std::fs::remove_dir_all(home).unwrap();
}
