//! Explicit timing recording retains actual attempts, bounds and concurrent writers.
use super::*;
#[cfg(test)]
fn history(place: &Path) -> Vec<Value> {
    std::fs::read_to_string(place.join("thinkthen.timing.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
#[cfg(test)]
fn only_timing(entry: &Value) {
    let keys = entry
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    assert!(keys.iter().all(|name| {
        [
            "key",
            "ordinal",
            "wall_ms",
            "outcome",
            "status",
            "server_ms",
        ]
        .contains(name)
    }));
    assert!(entry["key"].as_str().unwrap().len() == 64);
    assert!(entry["ordinal"].as_u64().unwrap() > 0);
}
#[test]
fn recording_attempts_is_optional_retry_timing_is_header_free_and_replay_never_appends() {
    let next = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| match next.fetch_add(1, Ordering::Relaxed) {
        1 => Canned::status(503, "{}").asking("retry-after-ms", "1"),
        _ => Canned::ok(REPLY)
            .asking("x-typesafe-request-id", "provider-private")
            .asking("x-envoy-upstream-service-time", "0"),
    })
    .unwrap();
    let place = folder();
    let engine = build(&listener)
        .max_retries(1)
        .record(&place)
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    engine
        .decide_complete_with(&question, "Refund me.", CallOptions::new())
        .unwrap();
    assert!(!place.join("thinkthen.timing.jsonl").exists());
    let recorded = engine
        .decide_complete_with(&question, "Refund me.", CallOptions::new().attempts(true))
        .unwrap();
    let entries = history(&place);
    assert_eq!(entries.len(), 2);
    for (entry, attempt) in entries.iter().zip(recorded.facts().attempts().unwrap()) {
        only_timing(entry);
        assert_eq!(entry["key"], recorded.value().meta().requests()[0]);
        assert_eq!(entry["ordinal"], attempt.ordinal());
        assert_eq!(entry["wall_ms"], attempt.wall_ms());
    }
    assert_eq!(entries[0]["outcome"], "status");
    assert_eq!(entries[0]["status"], 503);
    assert!(entries[0].get("server_ms").is_none());
    assert_eq!(entries[1]["outcome"], "ok");
    assert_eq!(entries[1]["server_ms"], 0);
    let bytes = std::fs::read(place.join("thinkthen.timing.jsonl")).unwrap();
    let text = String::from_utf8_lossy(&bytes);
    for secret in [
        "native-store-private",
        "provider-private",
        listener.base(),
        "Refund me.",
    ] {
        assert!(!text.contains(secret));
    }
    let replay = build(&listener).replay(&place).unwrap().build().unwrap();
    let held = replay
        .decide_complete_with(&question, "Refund me.", CallOptions::new().attempts(true))
        .unwrap();
    assert!(held.facts().attempts().unwrap().is_empty());
    assert_eq!(held.value().answer_id(), recorded.value().answer_id());
    assert_eq!(
        std::fs::read(place.join("thinkthen.timing.jsonl")).unwrap(),
        bytes
    );
    assert_eq!(listener.count(), 3);
}
#[test]
fn split_children_attribute_the_actual_parent_and_own_attempt_without_transient_ids() {
    let next = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| {
        if next.fetch_add(1, Ordering::Relaxed) == 0 {
            Canned::status(413, "{}")
        } else {
            Canned::ok(REPLY)
        }
    })
    .unwrap();
    let place = folder();
    let engine = build(&listener).record(&place).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let call = engine
        .decide_many_complete_with(
            &question,
            ["First.", "Second."],
            CallOptions::new().attempts(true),
        )
        .unwrap();
    assert_eq!(call.facts().attempts().unwrap().len(), 3);
    let entries = history(&place);
    assert_eq!(entries.len(), 4);
    for entry in &entries {
        only_timing(entry);
    }
    for (row, ordinals) in call.value().iter().zip([[1, 2], [1, 3]]) {
        let meta = row.result().meta();
        let key = &meta.requests()[0];
        assert_eq!(
            entries
                .iter()
                .filter(|entry| entry["key"] == *key)
                .map(|entry| entry["ordinal"].as_u64().unwrap())
                .collect::<Vec<_>>(),
            ordinals
        );
        assert_eq!(
            row.result().identity().question_sources()[0].batch_size(),
            Some(1)
        );
    }
    assert_eq!(listener.count(), 3);
}
#[test]
fn timing_entry_or_byte_overflow_keeps_old_answers_and_history_and_never_retries() {
    for bytes_limit in [false, true] {
        let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
        let place = folder();
        let engine = build(&listener)
            .max_retries(3)
            .record(&place)
            .unwrap()
            .build()
            .unwrap();
        let question = Question::decide("Refund?").unwrap().cut();
        engine
            .decide_complete_with(&question, "Refund me.", CallOptions::new().attempts(true))
            .unwrap();
        let seeded = if bytes_limit {
            vec![b'x'; 8 * 1024 * 1024 + 1]
        } else {
            let mut entry = history(&place).remove(0);
            entry.as_object_mut().unwrap().remove("status");
            let line = format!("{}\n", entry);
            let seed = line.repeat(65_536).into_bytes();
            assert!(seed.len() <= 8 * 1024 * 1024);
            seed
        };
        std::fs::write(place.join("thinkthen.timing.jsonl"), &seeded).unwrap();
        let other = Question::decide("Other?").unwrap().cut();
        let failed = engine
            .decide_complete_with(&other, "Other.", CallOptions::new().attempts(true))
            .unwrap_err();
        assert_eq!(failed.kind(), ErrorKind::Local);
        assert_eq!(failed.facts().unwrap().requests_sent(), 1);
        assert_eq!(listener.count(), 2);
        assert_eq!(
            std::fs::read(place.join("thinkthen.timing.jsonl")).unwrap(),
            seeded
        );
        let db = Connection::open(place.join("thinkthen.sqlite")).unwrap();
        let answers: i64 = db
            .query_row("SELECT count(*) FROM answers", [], |row| row.get(0))
            .unwrap();
        let originals: i64 = db
            .query_row("SELECT count(*) FROM exchanges", [], |row| row.get(0))
            .unwrap();
        assert_eq!((answers, originals), (1, 1));
        assert!(!std::fs::read_dir(&place).unwrap().any(|entry| {
            entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .ends_with(".tmp")
        }));
    }
}
#[test]
fn nonstorable_recording_creates_no_timing_history_or_saved_answers() {
    let listener =
        Listener::answering(|_| Canned::ok(REPLY).asking("Cache-Control", "no-store")).unwrap();
    let place = folder();
    let engine = build(&listener).record(&place).unwrap().build().unwrap();
    let error = engine
        .decide_complete_with(
            &Question::decide("Refund?").unwrap().cut(),
            "Refund me.",
            CallOptions::new().attempts(true),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(error.facts().unwrap().attempts().unwrap().len(), 1);
    assert!(!place.join("thinkthen.timing.jsonl").exists());
    assert!(!place.join("thinkthen.sqlite").exists());
    assert!(!place.join(".thinkthen.timing.lock").exists());
    assert_eq!(listener.count(), 1);
}
#[test]
fn concurrent_recording_calls_append_both_histories_under_one_stable_writer_lock() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let engines = [
        build(&listener).record(&place).unwrap().build().unwrap(),
        build(&listener).record(&place).unwrap().build().unwrap(),
    ];
    std::thread::scope(|scope| {
        let handles = engines
            .iter()
            .zip(["First?", "Second?"])
            .map(|(engine, text)| {
                scope.spawn(move || {
                    engine
                        .decide_complete_with(
                            &Question::decide(text).unwrap().cut(),
                            "Text.",
                            CallOptions::new().attempts(true),
                        )
                        .unwrap()
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            assert_eq!(handle.join().unwrap().facts().requests_sent(), 1);
        }
    });
    let entries = history(&place);
    assert_eq!(entries.len(), 2);
    assert_ne!(entries[0]["key"], entries[1]["key"]);
    let db = Connection::open(place.join("thinkthen.sqlite")).unwrap();
    let saved = db
        .prepare("SELECT lower(hex(key)) FROM answers")
        .unwrap()
        .query_map([], |row| row.get::<_, String>(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    for entry in entries {
        only_timing(&entry);
        assert!(saved.iter().any(|key| entry["key"] == *key));
    }
    assert_eq!(listener.count(), 2);
}
