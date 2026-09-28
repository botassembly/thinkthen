//! Default cache, paired recording, and mixed-hit identity boundaries.

use super::*;

#[test]
fn default_cache_binds_replays_and_stays_out_of_status_and_prune_counts() {
    let home = folder("default-cache-identity-home");
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let environment = [
        ("HOME", home.to_str().expect("home")),
        ("THINKTHEN_API_KEY", "sk-test-value"),
    ];
    let arguments = [
        "decide",
        QUESTION,
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    for _ in 0..2 {
        let output = spawn(&arguments, &environment, EVIDENCE.as_bytes()).expect("cached run");
        assert_eq!(output.status.code(), Some(0));
    }
    assert_eq!(listener.requests().len(), 1);
    let cache = default_cache(&home);
    let marker = cache.join(".thinkthen-backend.json");
    let before = fs::read(&marker).expect("bound default cache");

    let cache_text = cache.to_str().expect("cache");
    let pruned = spawn(
        &["cache", "prune", cache_text, "--max-size", "99999999"],
        &[],
        &[],
    )
    .expect("prune runs");
    assert_eq!(pruned.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&pruned.stdout).starts_with("removed 0 entries"));
    assert_eq!(fs::read(&marker).expect("marker after prune"), before);

    let status = spawn(
        &["status", "--json"],
        &[("THINKTHEN_CACHE", cache_text)],
        &[],
    )
    .expect("status runs");
    assert_eq!(status.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(value["cache"]["entries"], 1);
}

#[test]
fn paired_record_replay_binds_then_replays() {
    let cache = folder("paired-record-replay-identity");
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let cache_text = cache.to_str().expect("cache");
    let arguments = [
        "decide",
        QUESTION,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--record",
        cache_text,
        "--replay",
        cache_text,
    ];
    for _ in 0..2 {
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            EVIDENCE.as_bytes(),
        )
        .expect("paired run");
        assert_eq!(output.status.code(), Some(0));
    }
    assert_eq!(listener.requests().len(), 1);
    assert!(cache.join(".thinkthen-backend.json").is_file());
}

#[test]
fn a_bound_cache_mixes_a_hit_and_a_miss_without_rebinding() {
    let cache = folder("mixed-hit-miss-identity");
    let listener =
        Listener::serving(vec![Canned::ok(ANSWER), Canned::ok(ANSWER)]).expect("listener");
    let cache_text = cache.to_str().expect("cache");
    let arguments = [
        "decide",
        QUESTION,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        cache_text,
        "--lines",
    ];
    let first = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        format!("{EVIDENCE}\n").as_bytes(),
    )
    .expect("first run");
    assert_eq!(first.status.code(), Some(0));
    let second = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        format!("{EVIDENCE}\nA new record\n").as_bytes(),
    )
    .expect("mixed run");
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    assert_eq!(String::from_utf8_lossy(&second.stdout).lines().count(), 2);
}
