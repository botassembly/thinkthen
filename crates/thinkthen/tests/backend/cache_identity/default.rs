//! The default cache, a paired recording, and a mixed hit and miss.

use super::*;

#[test]
fn the_default_cache_answers_the_second_run() {
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
    assert_eq!(stored(&default_cache(&home)).expect("the store").len(), 1);
}

#[test]
fn paired_record_replay_fills_then_replays() {
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
    assert!(cache.join("thinkthen.sqlite").is_file());
}

#[test]
fn a_cache_mixes_a_hit_and_a_miss() {
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
