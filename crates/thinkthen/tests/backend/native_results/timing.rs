//! Explicit timing history stores no transport identities and replay leaves it intact.
use super::*;
#[test]
fn details_recording_writes_only_timing_fields_and_replay_sends_nothing() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
            .asking("x-typesafe-request-id", "timing-provider-private")
            .asking("x-envoy-upstream-service-time", "0")
    })
    .unwrap();
    let root = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("complete-command-timing-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let run = |mode| {
        spawn(
            &[
                "decide",
                "Good?",
                "--details",
                "--no-cache",
                mode,
                root.to_str().unwrap(),
                "--url",
                listener.base(),
                "--model",
                "fixed",
                "--max-retries",
                "0",
            ],
            &[("THINKTHEN_API_KEY", "timing-command-private")],
            b"Private evidence.",
        )
        .unwrap()
    };
    let live = run("--record");
    assert_eq!(
        live.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&live.stderr)
    );
    let row: Value = serde_json::from_slice(&live.stdout).unwrap();
    let bytes = std::fs::read(root.join("thinkthen.timing.jsonl")).unwrap();
    let history: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(history["key"], row["meta"]["requests"][0]);
    assert_eq!(history["ordinal"], 1);
    assert_eq!(history["outcome"], "ok");
    assert_eq!(history["status"], 200);
    assert_eq!(history["server_ms"], 0);
    assert_eq!(history.as_object().unwrap().len(), 6);
    assert!(history["wall_ms"].as_u64().is_some());
    let text = String::from_utf8_lossy(&bytes);
    for secret in [
        "timing-command-private",
        "timing-provider-private",
        listener.base(),
        "Private evidence.",
    ] {
        assert!(!text.contains(secret));
    }
    let replay = run("--replay");
    assert_eq!(replay.status.code(), Some(0));
    let held: Value = serde_json::from_slice(&replay.stdout).unwrap();
    assert_eq!(held["answer_id"], row["answer_id"]);
    assert_eq!(held["meta"]["attempts"], json!([]));
    assert_eq!(
        std::fs::read(root.join("thinkthen.timing.jsonl")).unwrap(),
        bytes
    );
    assert_eq!(listener.count(), 1);
    withheld(&live, "timing-command-private");
    withheld(&replay, "timing-command-private");
    std::fs::remove_dir_all(&root).unwrap();
}
