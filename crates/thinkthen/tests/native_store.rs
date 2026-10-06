//! Storage behavior through admitted public calls and independent saved exchanges.

use conformance_backend::{Canned, Listener};
use rusqlite::Connection;
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use thinkthen::{BatchSetting, CallOptions, Engine, ErrorKind, Observation, Origin, Question};

const STATE: &str = "\"Each question quotes the text it asks about.\"";
const QUESTION: &str = r#"{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}"#;
const ANSWER: &str = r#"{"type":"noul","noul":0.9}"#;
const REPLY: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#;

#[cfg(test)]
fn folder() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!(
        "thinkthen-native-store-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir(&path).unwrap();
    path
}

#[cfg(test)]
fn build(listener: &Listener) -> thinkthen::EngineBuilder {
    Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("native-store-private")
        .unwrap()
        .no_cache()
        .max_retries(0)
}

fn framed(tag: &str, parts: &[&str]) -> String {
    let mut hash = Sha256::new();
    hash.update(tag);
    hash.update([0]);
    for part in parts {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part);
    }
    hash.finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
fn legacy(place: &Path, url: &str, reported: &str) -> String {
    let key: String = Sha256::digest(format!("systemone\n{url}\n\"fixed\"\n{STATE}\n{QUESTION}"))
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let state = Sha256::digest(STATE);
    let db = Connection::open(place.join("thinkthen.sqlite")).unwrap();
    db.execute_batch("CREATE TABLE states(id INTEGER PRIMARY KEY, sha256 BLOB UNIQUE NOT NULL, state TEXT NOT NULL);
        CREATE TABLE answers(id INTEGER PRIMARY KEY, key BLOB UNIQUE NOT NULL, url TEXT NOT NULL, model TEXT NOT NULL, state INTEGER NOT NULL REFERENCES states(id), question TEXT NOT NULL, answer TEXT NOT NULL, answered_by TEXT NOT NULL, input_tokens INTEGER, output_tokens INTEGER, taken_at INTEGER NOT NULL, origin TEXT NOT NULL);
        PRAGMA user_version=1;").unwrap();
    db.execute(
        "INSERT INTO states VALUES(1,?1,?2)",
        (state.as_slice(), STATE),
    )
    .unwrap();
    let bytes: Vec<u8> = (0..key.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&key[at..at + 2], 16).unwrap())
        .collect();
    db.execute(
        "INSERT INTO answers VALUES(1,?1,?2,'fixed',1,?3,?4,?5,887,NULL,0,'converted')",
        (&bytes, url, QUESTION, ANSWER, reported),
    )
    .unwrap();
    key
}

#[cfg(test)]
fn stored_metadata(db: &Connection) -> (String, u32, String, String, Option<u32>) {
    db.query_row(
        "SELECT lower(hex(key)),key_version,adapter,observation_id,batch_size FROM answers",
        [],
        |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        },
    )
    .unwrap()
}

#[test]
fn validated_v1_replay_is_read_only_and_atomic_upgrade_preserves_legacy_identity() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let url = format!("{}/systemone", listener.base());
    let key = legacy(&place, &url, "fixed");
    let database = place.join("thinkthen.sqlite");
    let original = std::fs::read(&database).unwrap();
    let modified = std::fs::metadata(&database).unwrap().modified().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let replay = build(&listener).replay(&place).unwrap().build().unwrap();
    let historical = replay.details(&question, "Refund me.").unwrap();
    assert_eq!(listener.count(), 0);
    assert_eq!(std::fs::read(&database).unwrap(), original);
    assert_eq!(
        std::fs::metadata(&database).unwrap().modified().unwrap(),
        modified
    );
    let expected_id = framed(
        "thinkthen.legacy-observation/1",
        &[
            &key,
            r#"{"kind":"yes_no","probability":0.9}"#,
            r#"{"answered_by":"fixed","input_tokens":887,"output_tokens":null,"taken_at":0,"origin":"converted"}"#,
        ],
    );
    assert_eq!(
        historical.value().observations(),
        &[Observation::Answered {
            observation_id: expected_id.parse().unwrap()
        }]
    );
    assert_eq!(
        historical.value().question_sources()[0].origin(),
        Origin::Replay
    );
    assert_eq!(historical.value().question_sources()[0].batch_size(), None);
    assert_eq!(
        historical.value().reported_usage().unwrap().input_tokens(),
        Some(887)
    );
    assert_eq!(
        historical.value().reported_usage().unwrap().output_tokens(),
        None
    );
    let working = build(&listener).cache_at(&place).unwrap().build().unwrap();
    let held = working.details(&question, "Refund me.").unwrap();
    assert_eq!(listener.count(), 0);
    assert_eq!(
        held.value().observations(),
        historical.value().observations()
    );
    assert_eq!(held.value().question_sources()[0].origin(), Origin::Cache);
    let db = Connection::open(&database).unwrap();
    assert_eq!(
        db.query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        2
    );
    let expected_key = framed(
        "thinkthen.question-key/2",
        &["systemone", &url, "\"fixed\"", "\"fixed\"", STATE, QUESTION],
    );
    let stored = stored_metadata(&db);
    assert_eq!(
        stored,
        (
            expected_key.clone(),
            2,
            "systemone".into(),
            expected_id,
            None
        )
    );
    assert_eq!(held.value().requests(), &[expected_key]);
    let after = std::fs::read(&database).unwrap();
    working.details(&question, "Refund me.").unwrap();
    assert_eq!(std::fs::read(&database).unwrap(), after);
    drop(db);
    drop(working);
    drop(replay);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn overlapping_sets_cache_each_question_and_replay_original_observed_counts_without_sends() {
    let listener = Listener::answering(|request| {
        let request: Value = serde_json::from_slice(request).unwrap();
        let questions = request["questions"].as_object().unwrap();
        let answers: serde_json::Map<String, Value> = questions
            .keys()
            .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
    })
    .unwrap();
    let place = folder();
    let engine = build(&listener).cache_at(&place).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let sets = [
        (b'A'..=b'M').collect::<Vec<_>>(),
        (b'G'..=b'R').collect(),
        (b'O'..=b'Z').chain(b'A'..=b'D').collect(),
    ];
    let expected_arrivals = [
        (b'A'..=b'M').collect::<Vec<_>>(),
        (b'N'..=b'R').collect(),
        (b'S'..=b'Z').collect(),
    ];
    let mut identities = std::collections::BTreeMap::new();
    for (input, missing) in sets.iter().zip(&expected_arrivals) {
        let records: Vec<String> = input
            .iter()
            .map(|letter| char::from(*letter).to_string())
            .collect();
        let rows = engine
            .details_many_with(
                &question,
                records,
                CallOptions::new().batch(BatchSetting::Max),
            )
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let requests = listener.requests();
        assert_eq!(requests.len(), 1);
        let expected_questions: serde_json::Map<String,Value> = missing.iter().enumerate().map(|(at,letter)| (format!("q{}",at+1),json!({"type":"noul","instructions":format!("The text is \"{}\". Refund?",char::from(*letter))}))).collect();
        assert_eq!(
            serde_json::from_slice::<Value>(&requests[0].body).unwrap(),
            json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":expected_questions})
        );
        for row in rows {
            let detail = row.value();
            let source = &detail.question_sources()[0];
            if missing
                .iter()
                .any(|letter| row.input() == &char::from(*letter).to_string())
            {
                assert_eq!(source.origin(), Origin::Live);
                assert_eq!(source.batch_size(), Some(missing.len() as u32));
                identities.insert(
                    row.input().clone(),
                    (detail.observations().to_vec(), source.batch_size()),
                );
            } else {
                assert_eq!(source.origin(), Origin::Cache);
                assert_eq!(
                    &(detail.observations().to_vec(), source.batch_size()),
                    identities.get(row.input()).unwrap()
                );
            }
        }
    }
    assert_eq!(listener.count(), 3);
    let replay = build(&listener).replay(&place).unwrap().build().unwrap();
    for (record, (observations, count)) in identities {
        let detail = replay.details(&question, &record).unwrap();
        assert_eq!(detail.value().observations(), observations);
        assert_eq!(detail.value().question_sources()[0].batch_size(), count);
    }
    assert_eq!(listener.count(), 3);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn malformed_unrelated_stored_answers_refuse_even_refresh_and_record_before_sending() {
    for mode in ["cache", "refresh", "record", "replay"] {
        let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
        let place = folder();
        legacy(&place, &format!("{}/systemone", listener.base()), "fixed");
        let database = place.join("thinkthen.sqlite");
        let db = Connection::open(&database).unwrap();
        db.execute("UPDATE answers SET answer=?1", ["PRIVATE_DAMAGED"])
            .unwrap();
        drop(db);
        let original = std::fs::read(&database).unwrap();
        let builder = build(&listener);
        let engine = match mode {
            "cache" => builder.cache_at(&place).unwrap(),
            "refresh" => builder.cache_at(&place).unwrap().refresh_cache(true),
            "record" => builder.record(&place).unwrap(),
            "replay" => builder.replay(&place).unwrap(),
            _ => unreachable!(),
        }
        .build()
        .unwrap();
        let error = engine
            .details(&Question::decide("Other?").unwrap().cut(), "Unrelated.")
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Local);
        assert!(!format!("{error:?} {error}").contains("PRIVATE_DAMAGED"));
        assert_eq!(listener.count(), 0);
        assert_eq!(std::fs::read(&database).unwrap(), original);
        drop(engine);
        std::fs::remove_dir_all(place).unwrap();
    }
}

#[path = "native_store/conversion.rs"]
mod conversion;
#[path = "native_store/history.rs"]
mod history;
#[path = "native_store/request_keys.rs"]
mod request_keys;
