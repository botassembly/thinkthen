#![cfg(test)]
//! Persistence failure boundaries through the public engine and caller-owned store.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
#[path = "support/response_store.rs"]
#[allow(
    dead_code,
    reason = "the shared fixture also supports the reuse behavior suite"
)]
mod support;
use loopback::Loopback;
use sha2::{Digest, Sha256};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use support::*;
use thinkthen::{BackendName, Code, Decide, ResponseMode};

fn digest(parts: &[&[u8]]) -> Vec<u8> {
    let mut hash = Sha256::new();
    for part in parts {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part);
    }
    hash.finalize().to_vec()
}
// Produce internally consistent historical fixtures without sleeping or relying
// on scheduler timing. These are valid past/future exchanges, not tampering tests.
fn timestamp(path: &std::path::Path, received: i64) {
    let mut db = database(path);
    let tx = db.transaction().unwrap();
    let (id, metadata, request, response, recorded): (i64, Vec<u8>, Vec<u8>, Vec<u8>, i64) = tx
        .query_row(
            "SELECT id,metadata,request,response,recorded FROM exchanges ORDER BY id DESC LIMIT 1",
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
        .unwrap();
    let ex = digest(&[
        &metadata,
        &request,
        &response,
        &received.to_be_bytes(),
        &recorded.to_be_bytes(),
    ]);
    tx.execute(
        "UPDATE exchanges SET received=?1,digest=?2 WHERE id=?3",
        rusqlite::params![received, ex, id],
    )
    .unwrap();
    let answers: Vec<(i64, Vec<u8>, Vec<u8>)> = tx
        .prepare("SELECT slot,key,answer FROM answers WHERE exchange=?1")
        .unwrap()
        .query_map([id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect();
    for (slot, key, answer) in answers {
        tx.execute(
            "UPDATE answers SET digest=?1 WHERE exchange=?2 AND slot=?3",
            rusqlite::params![digest(&[&ex, &key, &slot.to_be_bytes(), &answer]), id, slot],
        )
        .unwrap();
    }
    tx.commit().unwrap();
}

#[test]
fn expired_and_future_entries_miss_but_recordings_replay_regardless_of_age() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    let sent = Arc::new(AtomicUsize::new(0));
    let seen = sent.clone();
    let server = Loopback::start(move |request| {
        seen.fetch_add(1, Ordering::SeqCst);
        response(&request.json().unwrap(), BackendName::SystemOne, 0.8)
    })
    .unwrap();
    let q = Decide::new("Relevant?").unwrap();
    let record = settings(
        &path,
        ResponseMode::Record,
        BackendName::SystemOne,
        server.base(),
    )
    .engine(|_| None)
    .unwrap();
    record.run(&q, "same").unwrap();
    drop(record);
    timestamp(&path, 1);
    assert!(
        replay(&path, server.base())
            .run(&q, "same")
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
    .engine(|_| None)
    .unwrap();
    cache.run(&q, "same").unwrap();
    let future = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        + 3600;
    timestamp(&path, future);
    cache.run(&q, "same").unwrap();
    assert_eq!(sent.load(Ordering::SeqCst), 3);
    assert_eq!(cache.usage().cache_hits, 0);
    assert_eq!(
        database(&path)
            .query_row(
                "SELECT count(*) FROM exchanges WHERE recorded=1",
                [],
                |row| row.get::<_, i64>(0)
            )
            .unwrap(),
        1
    );
}

#[test]
fn eviction_does_not_reuse_exchange_ids_and_record_quota_rolls_back_completely() {
    let dir = tempfile::tempdir().unwrap();
    let server =
        Loopback::start(|request| response(&request.json().unwrap(), BackendName::SystemOne, 0.8))
            .unwrap();
    let q = Decide::new("Relevant?").unwrap();
    let path = dir.path().join("cache");
    let cache = settings(
        &path,
        ResponseMode::Cache,
        BackendName::SystemOne,
        server.base(),
    )
    .response_store(Some(
        config(&path, ResponseMode::Cache)
            .limits(65536, 1, Duration::from_secs(60))
            .unwrap(),
    ))
    .engine(|_| None)
    .unwrap();
    for text in ["first", "second", "third"] {
        cache.run(&q, text).unwrap();
    }
    let db = database(&path);
    assert_eq!(
        db.query_row("SELECT count(*) FROM exchanges", [], |row| row
            .get::<_, i64>(0))
            .unwrap(),
        1
    );
    assert_eq!(
        db.query_row("SELECT max(id) FROM exchanges", [], |row| row
            .get::<_, i64>(0))
            .unwrap(),
        3
    );
    let path = dir.path().join("record");
    let record = settings(
        &path,
        ResponseMode::Record,
        BackendName::SystemOne,
        server.base(),
    )
    .response_store(Some(
        config(&path, ResponseMode::Record)
            .limits(2048, 1, Duration::from_secs(60))
            .unwrap(),
    ))
    .engine(|_| None)
    .unwrap();
    record.run(&q, "first").unwrap();
    let before = database(&path)
        .query_row("SELECT count(*) FROM exchanges", [], |row| {
            row.get::<_, i64>(0)
        })
        .unwrap();
    let error = (0..20).find_map(|_| record.run(&q, "first").err()).unwrap();
    assert_eq!(error.code(), Code::LocalStoreQuota);
    let db = database(&path);
    let after = db
        .query_row("SELECT count(*) FROM exchanges", [], |row| {
            row.get::<_, i64>(0)
        })
        .unwrap();
    assert!(after >= before);
    let answer_count = db
        .query_row("SELECT count(*) FROM answers", [], |row| {
            row.get::<_, i64>(0)
        })
        .unwrap();
    assert_eq!(after, answer_count);
    let recorded = replay(&path, server.base());
    assert!(recorded.run(&q, "first").unwrap().into_answer().unwrap());
    assert_eq!(record.usage().attempts, after as u64 + 1);
}

#[test]
fn corruption_format_and_invalid_answers_have_distinct_terminal_codes() {
    let dir = tempfile::tempdir().unwrap();
    let server =
        Loopback::start(|request| response(&request.json().unwrap(), BackendName::SystemOne, 0.8))
            .unwrap();
    let q = Decide::new("Relevant?").unwrap();
    for (name, expected) in [
        ("checksum", Code::LocalStoreCorrupt),
        ("format", Code::LocalStoreFormat),
        ("answer", Code::LocalStoreAnswer),
        ("oversized", Code::LocalStoreQuota),
    ] {
        let path = dir.path().join(name);
        let recording = settings(
            &path,
            ResponseMode::Record,
            BackendName::SystemOne,
            server.base(),
        )
        .engine(|_| None)
        .unwrap();
        recording.run(&q, "same").unwrap();
        drop(recording);
        let db = database(&path);
        match name {
            "checksum" => {
                db.execute("UPDATE exchanges SET digest=zeroblob(32)", [])
                    .unwrap();
            }
            "format" => {
                db.execute("UPDATE store SET format=1", []).unwrap();
            }
            "answer" => {
                let answer = b"[0.2,0.8]";
                let (ex,key,slot):(Vec<u8>,Vec<u8>,i64)=db.query_row("SELECT e.digest,a.key,a.slot FROM answers a JOIN exchanges e ON e.id=a.exchange",[],|row|Ok((row.get(0)?,row.get(1)?,row.get(2)?))).unwrap();
                let checksum = digest(&[&ex, &key, &slot.to_be_bytes(), answer]);
                db.execute(
                    "UPDATE answers SET answer=?1,digest=?2,size=?3",
                    rusqlite::params![answer, checksum, answer.len() as i64 + 64],
                )
                .unwrap();
            }
            _ => {
                db.execute("UPDATE answers SET answer=zeroblob(8192),size=8256", [])
                    .unwrap();
            }
        }
        drop(db);
        let settings = settings(
            &path,
            ResponseMode::Replay,
            BackendName::SystemOne,
            server.base(),
        )
        .response_store(Some(
            config(&path, ResponseMode::Replay)
                .limits(
                    if name == "oversized" { 1024 } else { 65536 },
                    10,
                    Duration::from_secs(60),
                )
                .unwrap(),
        ));
        let error = match settings.engine(|_| None) {
            Err(error) => error,
            Ok(engine) => engine.run(&q, "same").unwrap_err(),
        };
        assert_eq!(error.code(), expected, "{name}");
    }
}

#[test]
fn a_busy_writer_and_cancellation_never_fall_through_to_live_execution() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    let server =
        Loopback::start(|request| response(&request.json().unwrap(), BackendName::SystemOne, 0.8))
            .unwrap();
    let engine = settings(
        &path,
        ResponseMode::Cache,
        BackendName::SystemOne,
        server.base(),
    )
    .response_store(Some(
        config(&path, ResponseMode::Cache)
            .busy_timeout(Duration::from_millis(20))
            .unwrap(),
    ))
    .engine(|_| None)
    .unwrap();
    let db = database(&path);
    db.execute_batch("BEGIN IMMEDIATE").unwrap();
    let q = Decide::new("Relevant?").unwrap();
    assert_eq!(
        engine.run(&q, "same").unwrap_err().code(),
        Code::LocalResponseStore
    );
    assert_eq!(engine.usage().attempts, 0);
    db.execute_batch("ROLLBACK").unwrap();
    let signal = thinkthen::Cancel::default();
    signal.cancel();
    let cancelled = engine.clone().cancel(signal);
    assert!(cancelled.run(&q, "same").is_err());
    assert_eq!(cancelled.usage().attempts, 0);
    engine.run(&q, "same").unwrap();
    assert_eq!(engine.usage().attempts, 1);
}

#[cfg(unix)]
#[test]
fn replay_needs_no_write_access_and_rejects_substitution_or_shared_permissions() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    let server =
        Loopback::start(|request| response(&request.json().unwrap(), BackendName::SystemOne, 0.8))
            .unwrap();
    let q = Decide::new("Relevant?").unwrap();
    let live = settings(
        &path,
        ResponseMode::Record,
        BackendName::SystemOne,
        server.base(),
    )
    .engine(|_| None)
    .unwrap();
    live.run(&q, "same").unwrap();
    drop(live);
    std::fs::set_permissions(
        path.join("responses.sqlite3"),
        std::fs::Permissions::from_mode(0o400),
    )
    .unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o500)).unwrap();
    let before = std::fs::read(path.join("responses.sqlite3")).unwrap();
    let read = replay(&path, server.base());
    assert!(read.run(&q, "same").unwrap().into_answer().unwrap());
    assert_eq!(
        std::fs::read(path.join("responses.sqlite3")).unwrap(),
        before
    );
    assert_eq!(std::fs::read_dir(&path).unwrap().count(), 1);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
    std::fs::set_permissions(
        path.join("responses.sqlite3"),
        std::fs::Permissions::from_mode(0o600),
    )
    .unwrap();
    std::fs::rename(&path, dir.path().join("old")).unwrap();
    std::fs::create_dir(&path).unwrap();
    assert_eq!(
        read.run(&q, "same").unwrap_err().code(),
        Code::LocalResponseStore
    );
    let linked = dir.path().join("linked");
    symlink(dir.path().join("old"), &linked).unwrap();
    assert_eq!(
        settings(
            &linked,
            ResponseMode::Replay,
            BackendName::SystemOne,
            server.base()
        )
        .engine(|_| None)
        .unwrap_err()
        .code(),
        Code::LocalResponseStore
    );
    std::fs::set_permissions(
        dir.path().join("old"),
        std::fs::Permissions::from_mode(0o777),
    )
    .unwrap();
    assert_eq!(
        settings(
            &dir.path().join("old"),
            ResponseMode::Replay,
            BackendName::SystemOne,
            server.base()
        )
        .engine(|_| None)
        .unwrap_err()
        .code(),
        Code::LocalResponseStore
    );
    std::fs::set_permissions(
        dir.path().join("old"),
        std::fs::Permissions::from_mode(0o700),
    )
    .unwrap();
}

#[test]
fn competing_processes_commit_independent_recordings() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store");
    let server =
        Loopback::start(|request| response(&request.json().unwrap(), BackendName::SystemOne, 0.8))
            .unwrap();
    drop(
        settings(
            &path,
            ResponseMode::Record,
            BackendName::SystemOne,
            server.base(),
        )
        .engine(|_| None)
        .unwrap(),
    );
    let mut children: Vec<_> = ["first", "second"]
        .into_iter()
        .map(|text| {
            std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "store_process_worker", "--nocapture"])
                .env("THINKTHEN_STORE_TEST_PATH", &path)
                .env("THINKTHEN_STORE_TEST_URL", server.base())
                .env("THINKTHEN_STORE_TEST_TEXT", text)
                .spawn()
                .unwrap()
        })
        .collect();
    for child in &mut children {
        assert!(child.wait().unwrap().success());
    }
    let db = database(&path);
    assert_eq!(
        db.query_row(
            "SELECT count(*) FROM exchanges WHERE recorded=1",
            [],
            |row| row.get::<_, i64>(0)
        )
        .unwrap(),
        2
    );
    let read = replay(&path, server.base());
    for text in ["first", "second"] {
        assert!(
            read.run(&Decide::new("Relevant?").unwrap(), text)
                .unwrap()
                .into_answer()
                .unwrap()
        );
    }
}
#[test]
fn store_process_worker() {
    let Some(path) = std::env::var_os("THINKTHEN_STORE_TEST_PATH") else {
        return;
    };
    let url = std::env::var("THINKTHEN_STORE_TEST_URL").unwrap();
    let text = std::env::var("THINKTHEN_STORE_TEST_TEXT").unwrap();
    let engine = settings(
        std::path::Path::new(&path),
        ResponseMode::Record,
        BackendName::SystemOne,
        &url,
    )
    .engine(|_| None)
    .unwrap();
    assert!(
        engine
            .run(&Decide::new("Relevant?").unwrap(), text.as_str())
            .unwrap()
            .into_answer()
            .unwrap()
    );
}
