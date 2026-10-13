//! Storage behavior through admitted public calls and independent saved exchanges.

use conformance_backend::{Canned, Listener};
use rusqlite::Connection;
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use thinkthen::{BatchSetting, CallOptions, ErrorKind, Origin, Question};

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
    crate::api::fixed(listener.base(), "native-store-private")
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
fn legacy_key(url: &str) -> String {
    Sha256::digest(format!("systemone\n{url}\n\"fixed\"\n{STATE}\n{QUESTION}"))
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
fn legacy(place: &Path, url: &str, reported: &str) -> String {
    let key = legacy_key(url);
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
fn old_store_refuses_every_mode_without_sends_or_file_changes() {
    for (mode, format) in ["cache", "refresh", "record", "replay"]
        .into_iter()
        .flat_map(|mode| ["sqlite", "jsonl", "digest"].map(|format| (mode, format)))
    {
        let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
        let place = folder();
        let url = format!("{}/systemone", listener.base());
        let database = match format {
            "sqlite" => {
                legacy(&place, &url, "fixed");
                place.join("thinkthen.sqlite")
            }
            "jsonl" => {
                let digest: String = Sha256::digest(STATE)
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect();
                let state = json!({"sha256":digest,"state":STATE});
                let answer = json!({"key":legacy_key(&url),"url":url,"model":"fixed","state":digest,"question":QUESTION,"answer":ANSWER,"answered_by":"fixed","input_tokens":null,"output_tokens":null,"taken_at":0,"origin":"fixture"});
                let path = place.join("thinkthen.jsonl");
                std::fs::write(&path, format!("{state}\n{answer}\n")).unwrap();
                path
            }
            "digest" => {
                let path = place.join(format!("{}.json", "a".repeat(64)));
                std::fs::write(&path, "PRIVATE_OLD_FILE").unwrap();
                path
            }
            _ => unreachable!(),
        };
        let original = std::fs::read(&database).unwrap();
        let modified = std::fs::metadata(&database).unwrap().modified().unwrap();
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
            .details(&Question::decide("Refund?").unwrap().cut(), "Refund me.")
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Local);
        assert_eq!(
            error.to_string(),
            format!(
                "{} uses an old store format; delete it",
                database.file_name().unwrap().to_str().unwrap()
            )
        );
        assert_eq!(listener.count(), 0);
        assert!(!format!("{error:?} {error}").contains("PRIVATE_OLD_FILE"));
        assert_eq!(std::fs::read_dir(&place).unwrap().count(), 1);
        assert_eq!(std::fs::read(&database).unwrap(), original);
        assert_eq!(
            std::fs::metadata(&database).unwrap().modified().unwrap(),
            modified
        );
        drop(engine);
        std::fs::remove_dir_all(place).unwrap();
    }
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
        let rows = crate::api::pull_decisions(
            &engine,
            &question,
            crate::api::plain_records(records),
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
            let detail = row.result();
            let source = &detail.identity().question_sources()[0];
            if missing
                .iter()
                .any(|letter| row.original() == &char::from(*letter).to_string())
            {
                assert_eq!(source.origin(), Origin::Live);
                assert_eq!(source.batch_size(), Some(missing.len() as u32));
                identities.insert(
                    row.original().clone(),
                    (
                        detail.identity().observations().to_vec(),
                        source.batch_size(),
                    ),
                );
            } else {
                assert_eq!(source.origin(), Origin::Cache);
                assert_eq!(
                    &(
                        detail.identity().observations().to_vec(),
                        source.batch_size()
                    ),
                    identities.get(row.original()).unwrap()
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
        for column in ["answer", "origin"] {
            let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
            let place = folder();
            let seed = build(&listener).record(&place).unwrap().build().unwrap();
            seed.details(&Question::decide("Refund?").unwrap().cut(), "Refund me.")
                .unwrap();
            drop(seed);
            let _ = listener.requests();
            let database = place.join("thinkthen.sqlite");
            let db = Connection::open(&database).unwrap();
            db.execute(
                &format!("UPDATE answers SET {column}=?1"),
                ["PRIVATE_DAMAGED"],
            )
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
            assert_eq!(listener.count(), 1);
            assert_eq!(std::fs::read(&database).unwrap(), original);
            drop(engine);
            std::fs::remove_dir_all(place).unwrap();
        }
    }
}

#[path = "native_store/conversion.rs"]
#[cfg(feature = "cli")]
mod conversion;
#[path = "native_store/history.rs"]
mod history;
#[path = "native_store/replay_contention.rs"]
mod replay_contention;
#[path = "native_store/request_keys.rs"]
mod request_keys;

#[path = "native_store/timing.rs"]
mod timing;

#[path = "support/api.rs"]
mod api;
