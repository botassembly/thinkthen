#![cfg(test)]
//! Opt-in reuse through the public Rust client; no paid backends.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
#[path = "support/response_store.rs"]
mod support;
use loopback::{KEY, Loopback, Reply};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use support::*;
use thinkthen::{BackendName, Code, Decide, Key, ResponseMode, ResponseStore, Settings, Threshold};

#[cfg(unix)]
#[test]
fn ancestor_alias_records_reuses_and_replays_without_losing_directory_identity() {
    use std::os::unix::fs::symlink;
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().canonicalize().unwrap();
    let real = root.join("real");
    std::fs::create_dir(&real).unwrap();
    let alias = root.join("alias");
    symlink(&real, &alias).unwrap();
    let path = alias.join("store");
    let sends = Arc::new(AtomicUsize::new(0));
    let observed = sends.clone();
    let server = Loopback::start(move |request| {
        let index = observed.fetch_add(1, Ordering::SeqCst);
        let body = request.json().unwrap();
        let probability = if index == 0 { 0.8 } else { 0.2 };
        response(&body, BackendName::SystemOne, probability)
    })
    .unwrap();
    let question = Decide::new("Relevant?").unwrap();
    let recorded = settings(
        &path,
        ResponseMode::Record,
        BackendName::SystemOne,
        server.base(),
    )
    .engine(|_| None)
    .unwrap();
    for (text, answer) in [("yes", true), ("no", false)] {
        assert_eq!(
            recorded
                .run(&question, text)
                .unwrap()
                .into_answer()
                .unwrap(),
            answer
        );
    }
    drop(recorded);
    let cached = settings(
        &path,
        ResponseMode::Cache,
        BackendName::SystemOne,
        server.base(),
    )
    .max_requests(Some(0))
    .engine(|_| None)
    .unwrap();
    let replayed = replay(&path, server.base());
    for (text, answer) in [("yes", true), ("no", false)] {
        assert_eq!(
            cached.run(&question, text).unwrap().into_answer().unwrap(),
            answer
        );
        assert_eq!(
            replayed
                .run(&question, text)
                .unwrap()
                .into_answer()
                .unwrap(),
            answer
        );
    }
    assert_eq!(sends.load(Ordering::SeqCst), 2);
    assert_eq!(cached.usage().cache_hits, 2);
    assert_eq!(replayed.usage().replay_hits, 2);
    let replacement = root.join("replacement");
    std::fs::create_dir(&replacement).unwrap();
    std::fs::remove_file(&alias).unwrap();
    symlink(&replacement, &alias).unwrap();
    assert_eq!(
        cached.run(&question, "yes").unwrap_err().code(),
        Code::LocalResponseStore
    );
    assert_eq!(
        replayed.run(&question, "yes").unwrap_err().code(),
        Code::LocalResponseStore
    );
    assert_eq!(sends.load(Ordering::SeqCst), 2);
    // The caller alias can still resolve to the held directory while the
    // canonical pathname names a replacement. Both identities must be checked.
    std::fs::remove_file(&alias).unwrap();
    let moved = root.join("moved");
    std::fs::rename(&real, &moved).unwrap();
    symlink(&moved, &alias).unwrap();
    let mut private = std::fs::DirBuilder::new();
    use std::os::unix::fs::DirBuilderExt;
    private.mode(0o700).recursive(true).create(&real).unwrap();
    private.create(real.join("store")).unwrap();
    assert_eq!(
        cached.run(&question, "yes").unwrap_err().code(),
        Code::LocalResponseStore
    );
    assert_eq!(
        replayed.run(&question, "yes").unwrap_err().code(),
        Code::LocalResponseStore
    );
    assert_eq!(sends.load(Ordering::SeqCst), 2);
}

#[test]
fn cache_reinterprets_thresholds_and_repacks_only_misses_on_both_wires() {
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let dir = tempfile::tempdir().unwrap();
        let sent = Arc::new(AtomicUsize::new(0));
        let seen = sent.clone();
        let server = Loopback::start(move |request| {
            let body = request.json().unwrap();
            seen.fetch_add(asks(&body, backend), Ordering::SeqCst);
            response(&body, backend, 0.7)
        })
        .unwrap();
        let engine = settings(
            &dir.path().join("store"),
            ResponseMode::Cache,
            backend,
            server.base(),
        )
        .max_requests(Some(2))
        .engine(|_| None)
        .unwrap();
        let question = Decide::new("Relevant?").unwrap();
        assert!(engine.run(&question, "old").unwrap().into_answer().unwrap());
        let strict = question.clone().threshold(Threshold::cut(0.8).unwrap());
        assert!(!engine.run(&strict, "old").unwrap().into_answer().unwrap());
        let rows: Vec<_> = engine
            .rows(
                &question,
                vec![
                    thinkthen::Record::text("old"),
                    thinkthen::Record::text("new"),
                ],
            )
            .unwrap()
            .collect();
        assert_eq!(rows.len(), 2);
        assert!(
            rows.into_iter()
                .all(|row| row.unwrap().into_answer().unwrap())
        );
        let usage = engine.settled_usage();
        assert_eq!(sent.load(Ordering::SeqCst), 2);
        assert_eq!(usage.attempts, 2);
        assert_eq!(usage.cache_hits, 2);
        assert_eq!(usage.replay_hits, 0);
        assert_eq!(usage.input_tokens.reported, Some(26));
        assert!(engine.run(&question, "new").unwrap().into_answer().unwrap());
        assert_eq!(
            engine.run(&question, "unseen").unwrap_err().code(),
            Code::LocalRequestLimit
        );
    }
}

#[test]
fn replay_reads_a_fixed_recording_without_keys_or_network_and_ignores_cache_eviction() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    let sent = Arc::new(AtomicUsize::new(0));
    let seen = sent.clone();
    let server = Loopback::start(move |request| {
        let index = seen.fetch_add(1, Ordering::SeqCst);
        response(
            &request.json().unwrap(),
            BackendName::SystemOne,
            if index == 0 { 0.8 } else { 0.2 },
        )
    })
    .unwrap();
    let record = settings(
        &path,
        ResponseMode::Record,
        BackendName::SystemOne,
        server.base(),
    )
    .engine(|_| None)
    .unwrap();
    let question = Decide::new("Relevant?").unwrap();
    assert!(
        record
            .run(&question, "same")
            .unwrap()
            .into_answer()
            .unwrap()
    );
    let replay = replay(&path, server.base());
    assert!(
        !record
            .run(&question, "same")
            .unwrap()
            .into_answer()
            .unwrap()
    );
    assert!(
        replay
            .run(&question, "same")
            .unwrap()
            .into_answer()
            .unwrap()
    );
    assert!(
        !replay_fresh(&path, server.base())
            .run(&question, "same")
            .unwrap()
            .into_answer()
            .unwrap()
    );
    let cache = settings(
        &path,
        ResponseMode::Cache,
        BackendName::SystemOne,
        server.base(),
    )
    .response_store(Some(
        config(&path, ResponseMode::Cache)
            .limits(64 * 1024 * 1024, 1, Duration::from_secs(86400))
            .unwrap(),
    ))
    .engine(|_| None)
    .unwrap();
    cache.run(&question, "evict").unwrap();
    assert!(
        replay
            .run(&question, "same")
            .unwrap()
            .into_answer()
            .unwrap()
    );
    assert_eq!(
        replay.run(&question, "evict").unwrap_err().code(),
        Code::LocalReplayMiss
    );
    assert_eq!(sent.load(Ordering::SeqCst), 3);
    let usage = replay.settled_usage();
    assert_eq!(usage.attempts, 0);
    assert_eq!(usage.replay_hits, 2);
    assert_eq!(usage.input_tokens.reported, Some(0));
    assert_eq!(usage.output_tokens.unknown_attempts, 0);
    let db = database(&path);
    let recordings: i64 = db
        .query_row(
            "SELECT count(*) FROM exchanges WHERE recorded=1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(recordings, 2);
}

#[test]
fn identity_changes_invalidate_answers_but_keys_and_row_names_do_not_partition_them() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    let sent = Arc::new(AtomicUsize::new(0));
    let seen = sent.clone();
    let server = Loopback::start(move |request| {
        seen.fetch_add(1, Ordering::SeqCst);
        response(&request.json().unwrap(), BackendName::SystemOne, 0.8)
    })
    .unwrap();
    let base = settings(
        &path,
        ResponseMode::Cache,
        BackendName::SystemOne,
        server.base(),
    );
    let question = Decide::new("Relevant?").unwrap();
    base.engine(|_| None)
        .unwrap()
        .run(&question, "same")
        .unwrap();
    base.clone()
        .api_key(Key::new("another-test-credential").unwrap())
        .engine(|_| None)
        .unwrap()
        .named("new-name")
        .unwrap()
        .run(&question, "same")
        .unwrap();
    assert_eq!(sent.load(Ordering::SeqCst), 1);
    for changed in [
        base.clone().model("other-model"),
        base.clone().url(format!("{}/other", server.base())),
        base.clone().response_store(Some(
            ResponseStore::new(&path, ResponseMode::Cache, "other-namespace", "v1").unwrap(),
        )),
        base.clone().response_store(Some(
            ResponseStore::new(&path, ResponseMode::Cache, "test", "v2").unwrap(),
        )),
    ] {
        changed
            .engine(|_| None)
            .unwrap()
            .run(&question, "same")
            .unwrap();
    }
    base.engine(|_| None)
        .unwrap()
        .run(&question.clone().yes_means("different meaning"), "same")
        .unwrap();
    base.engine(|_| None)
        .unwrap()
        .run(&question, "changed input")
        .unwrap();
    assert_eq!(sent.load(Ordering::SeqCst), 7);
    assert!(
        !std::fs::read(path.join("responses.sqlite3"))
            .unwrap()
            .windows(KEY.len())
            .any(|bytes| bytes == KEY.as_bytes())
    );
}

#[test]
fn cache_only_answers_are_not_recordings_and_failed_replies_never_become_hits() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    let sent = Arc::new(AtomicUsize::new(0));
    let seen = sent.clone();
    let server = Loopback::start(move |request| {
        if seen.fetch_add(1, Ordering::SeqCst) == 0 {
            Reply::json(200, &json!({"answers":{}}))
        } else {
            response(&request.json().unwrap(), BackendName::SystemOne, 0.8)
        }
    })
    .unwrap();
    let engine = settings(
        &path,
        ResponseMode::Cache,
        BackendName::SystemOne,
        server.base(),
    )
    .engine(|_| None)
    .unwrap();
    let q = Decide::new("Relevant?").unwrap();
    assert!(engine.run(&q, "same").unwrap().into_answer().is_err());
    assert!(engine.run(&q, "same").unwrap().into_answer().unwrap());
    assert!(engine.run(&q, "same").unwrap().into_answer().unwrap());
    assert_eq!(engine.usage().attempts, 2);
    assert_eq!(engine.usage().cache_hits, 1);
    assert_eq!(
        replay(&path, server.base())
            .run(&q, "same")
            .unwrap_err()
            .code(),
        Code::LocalReplayMiss
    );
}

#[test]
fn unsafe_routes_and_stub_are_rejected_before_credentials_or_disk_creation() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    for url in [
        "http://127.0.0.1/v1?tenant=secret",
        "http://127.0.0.1/v1#fragment",
        "http://user:password@127.0.0.1/v1",
    ] {
        let result = Settings::default()
            .url(url)
            .response_store(Some(config(&path, ResponseMode::Record)))
            .engine(|name| {
                assert!(!name.ends_with("KEY"));
                None
            });
        assert!(result.is_err());
        assert!(!path.exists());
    }
    assert_eq!(
        Settings::default()
            .with_backend(BackendName::Stub)
            .response_store(Some(config(&path, ResponseMode::Cache)))
            .engine(|_| None)
            .unwrap_err()
            .code(),
        Code::UsageSettings
    );
    assert!(!path.exists());
    assert!(ResponseStore::new(&path, ResponseMode::Cache, " ", "v1").is_err());
    assert!(
        config(&path, ResponseMode::Cache)
            .limits(0, 1, Duration::from_secs(1))
            .is_err()
    );
}

#[test]
fn quota_and_credential_echo_stop_after_honestly_counting_the_live_response() {
    let dir = tempfile::tempdir().unwrap();
    for (name, body, limit) in [
        (
            "large",
            json!({"padding":"x".repeat(4096),"answers":{"q1":{"type":"noul","noul":0.8}}}),
            1024,
        ),
        (
            "secret",
            json!({"echo":KEY,"answers":{"q1":{"type":"noul","noul":0.8}}}),
            65536,
        ),
    ] {
        let path = dir.path().join(name);
        let server = Loopback::start(move |_| Reply::json(200, &body)).unwrap();
        let engine = settings(
            &path,
            ResponseMode::Record,
            BackendName::SystemOne,
            server.base(),
        )
        .response_store(Some(
            config(&path, ResponseMode::Record)
                .limits(limit, 10, Duration::from_secs(60))
                .unwrap(),
        ))
        .engine(|_| None)
        .unwrap();
        let error = engine
            .run(&Decide::new("Relevant?").unwrap(), "same")
            .unwrap_err();
        assert_eq!(
            error.code(),
            if name == "large" {
                Code::LocalStoreQuota
            } else {
                Code::LocalResponseStore
            }
        );
        assert_eq!(engine.usage().attempts, 1);
        assert_eq!(
            database(&path)
                .query_row("SELECT count(*) FROM exchanges", [], |row| row
                    .get::<_, i64>(0))
                .unwrap(),
            0
        );
        assert!(
            !std::fs::read(path.join("responses.sqlite3"))
                .unwrap()
                .windows(KEY.len())
                .any(|bytes| bytes == KEY.as_bytes())
        );
    }
}

#[test]
fn dependent_rounds_replay_their_own_asks_and_a_late_miss_stays_terminal() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    let server=Loopback::answering(|question| if question["criteria"].get("BEGIN").is_some() {
        json!({"type":"choice","probabilities":{"BEGIN":0.0,"INSIDE":0.0,"END":0.0,"SINGLE":1.0,"OUT":0.0}})
    } else {json!({"type":"choice","probabilities":{"person":0.9,"none of these":0.1}})}).unwrap();
    let question = thinkthen::Recognize::new(["person"]).unwrap();
    let live = settings(
        &path,
        ResponseMode::Record,
        BackendName::SystemOne,
        server.base(),
    )
    .engine(|_| None)
    .unwrap();
    let original = live.run(&question, "Ada").unwrap().into_answer().unwrap();
    assert_eq!(original.len(), 1);
    assert_eq!(server.count(), 2);
    let read = replay(&path, server.base());
    let saved = read.run(&question, "Ada").unwrap().into_answer().unwrap();
    assert_eq!(
        serde_json::to_value(saved).unwrap(),
        serde_json::to_value(original).unwrap()
    );
    assert_eq!(read.usage().replay_hits, 2);
    assert_eq!(server.count(), 2);
    // Removing the later recorded exchange leaves the initial boundary ask but
    // must not turn an unavailable dependent kind into successful empty output.
    let db = database(&path);
    db.execute_batch(
        "PRAGMA foreign_keys=ON; DELETE FROM exchanges WHERE id=(SELECT MAX(id) FROM exchanges);",
    )
    .unwrap();
    let incomplete = replay(&path, server.base());
    assert_eq!(
        incomplete.run(&question, "Ada").unwrap_err().code(),
        Code::LocalReplayMiss
    );
    assert_eq!(incomplete.usage().replay_hits, 1);
    assert_eq!(incomplete.usage().attempts, 0);
}

#[test]
fn ordered_options_and_descriptions_belong_to_the_key() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    let server = Loopback::answering(|question| {
        let probabilities: serde_json::Map<String, serde_json::Value> = question["criteria"]
            .as_object()
            .unwrap()
            .keys()
            .map(|key| (key.clone(), json!(if key == "a" { 0.9 } else { 0.1 })))
            .collect();
        json!({"type":"choice","probabilities":probabilities})
    })
    .unwrap();
    let engine = settings(
        &path,
        ResponseMode::Cache,
        BackendName::SystemOne,
        server.base(),
    )
    .engine(|_| None)
    .unwrap();
    let first = thinkthen::Choose::new("Which?", ["a", "b"]).unwrap();
    let reversed = thinkthen::Choose::new("Which?", ["b", "a"]).unwrap();
    let described = thinkthen::Choose::with_options(
        "Which?",
        [
            thinkthen::questions::AnswerOption::new("a")
                .unwrap()
                .description("meaning"),
            thinkthen::questions::AnswerOption::new("b").unwrap(),
        ],
    )
    .unwrap();
    for question in [&first, &reversed, &described, &first] {
        engine.run(question, "same").unwrap();
    }
    assert_eq!(server.count(), 3);
    assert_eq!(engine.usage().cache_hits, 1);
}

#[test]
fn sparse_cache_hits_keep_current_positions_ids_and_discriminating_answers() {
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("store");
        let server=Loopback::start(move |request| {
            let body=request.json().unwrap();let probability=|question:&serde_json::Value| { let text=question["instructions"].as_str().unwrap(); if text.contains("cached-yes") {0.9} else if text.contains("cached-no") {0.2} else if text.contains("fresh-yes") {0.8} else {0.1} };
            let answers=if backend==BackendName::SystemOne {serde_json::Value::Object(body["questions"].as_object().unwrap().iter().map(|(slot,question)|(slot.clone(),json!({"type":"noul","noul":probability(question)}))).collect())}else{json!(body["questions"].as_array().unwrap().iter().map(|question|json!({"type":"predicate","probability":probability(question)})).collect::<Vec<_>>())};
            Reply::json(200,&json!({"answers":answers}))
        }).unwrap();
        let engine = settings(&path, ResponseMode::Cache, backend, server.base())
            .engine(|_| None)
            .unwrap();
        let question = Decide::new("Relevant?").unwrap();
        let seed: Vec<_> = engine
            .rows(
                &question,
                [
                    thinkthen::Record::text("cached-yes"),
                    thinkthen::Record::text("cached-no"),
                ],
            )
            .unwrap()
            .collect();
        assert_eq!(seed.len(), 2);
        let texts = ["cached-yes", "fresh-no", "cached-no", "fresh-yes"];
        let rows: Vec<_> = engine
            .rows(
                &question,
                texts.into_iter().enumerate().map(|(i, text)| {
                    thinkthen::Record::text(text)
                        .with_id(thinkthen::RecordId::String(format!("row-{i}")))
                }),
            )
            .unwrap()
            .map(|row| serde_json::to_value(row.unwrap()).unwrap())
            .collect();
        assert_eq!(rows.len(), 4);
        for (index, (row, (expected, probability))) in rows
            .iter()
            .zip([(true, 0.9), (false, 0.1), (false, 0.2), (true, 0.8)])
            .enumerate()
        {
            assert_eq!(row["id"], format!("row-{index}"));
            assert_eq!(row["position"], index + 1);
            assert_eq!(row["answer"], expected);
            assert_eq!(row["probability"], probability);
        }
        assert_eq!(engine.usage().cache_hits, 2);
        assert_eq!(server.count(), 2);
        assert_eq!(asks(&server.requests()[1].json().unwrap(), backend), 2);
    }
}

#[test]
fn escaped_credentials_in_json_values_or_member_names_are_never_retained() {
    for member in [false, true] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("store");
        let server = Loopback::start(move |_| {
            let mut body = json!({"answers":{"q1":{"type":"noul","noul":0.8}}});
            body[if member { KEY } else { "echo" }] =
                json!(if member { "placeholder" } else { KEY });
            let mut reply = Reply::json(200, &body);
            let escaped: String = KEY.bytes().map(|byte| format!("\\u{byte:04x}")).collect();
            reply.body = String::from_utf8(reply.body)
                .unwrap()
                .replace(KEY, &escaped)
                .into_bytes();
            reply
        })
        .unwrap();
        let engine = settings(
            &path,
            ResponseMode::Record,
            BackendName::SystemOne,
            server.base(),
        )
        .engine(|_| None)
        .unwrap();
        assert_eq!(
            engine
                .run(&Decide::new("Relevant?").unwrap(), "same")
                .unwrap_err()
                .code(),
            Code::LocalResponseStore
        );
        assert_eq!(engine.usage().attempts, 1);
        assert_eq!(
            database(&path)
                .query_row("SELECT count(*) FROM exchanges", [], |row| row
                    .get::<_, i64>(0))
                .unwrap(),
            0
        );
    }
}
