//! The store's edge table, by ADR 0111 slice 2.

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use super::{Entries, JSONL, Mode, Row, SQLITE, Store};
use crate::core::pack::{QuestionKey, State};
use crate::core::{Url, Usage};
use crate::engine::Cancel;
use crate::engine::error::Error;

const A: &str = r#"{"type":"noul","instructions":"a"}"#;
const B: &str = r#"{"type":"noul","instructions":"b"}"#;
const FIRST: &str = r#"{"type":"noul","noul":0.1}"#;
const SECOND: &str = r#"{"type":"noul","noul":0.2}"#;

const URL: &str = "http://127.0.0.1:9/v1/systemone";

fn state() -> State {
    State::new(
        "\"Each question quotes the text it asks about.\"".to_owned(),
        44,
    )
}

fn key(question: &str) -> QuestionKey {
    QuestionKey::complete(
        &Url::new(URL).expect("url"),
        "\"jev-1\"",
        "\"jev-1\"",
        state().json(),
        question,
    )
}

fn row<'a>(state: &'a State, question: &'a str, answer: &'a str) -> Row<'a> {
    Row {
        observation_id: crate::core::ObservationId::new("a".repeat(64)).unwrap(),
        batch_size: None,
        key: key(question),
        url: URL,
        model: "jev-1",
        state,
        question,
        answer,
        answered_by: "jev-1",
        usage: Some(crate::core::ReportedUsage::from_complete(Usage::new(7, 1))),
        taken_at: 1_700_000_000,
        origin: "live",
    }
}

fn written(folder: &Path, rows: &[Row<'_>]) {
    let mut store =
        Store::open(folder, Mode::Cache, false, None, &Cancel::default()).expect("open");
    store.write(rows, &Cancel::default()).expect("write");
}

#[test]
fn one_store_scans_once_for_its_own_writes_and_refuses_an_external_corruption() {
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, A, FIRST)]);
    let cancel = Cancel::default();
    let mut operation = Store::open(folder.path(), Mode::Cache, false, None, &Cancel::default())
        .expect("open")
        .prepared(&cancel)
        .expect("admit");
    assert_eq!(operation.validation_scans, 1);
    operation
        .write(&[row(&state, B, SECOND)], &cancel)
        .expect("own answer");
    operation
        .revalidate(&cancel)
        .expect("own write is admitted");
    assert_eq!(operation.validation_scans, 1);

    let outside = rusqlite::Connection::open(folder.path().join(SQLITE)).expect("outside");
    outside
        .execute(
            "UPDATE answers SET answer = 'broken' WHERE question = ?1",
            [A],
        )
        .expect("external change");
    assert!(matches!(
        operation.revalidate(&cancel),
        Err(Error::Entry(..))
    ));
    assert_eq!(operation.validation_scans, 2);
    assert!(matches!(
        Store::open(folder.path(), Mode::Record, false, None, &Cancel::default())
            .expect("separate call")
            .prepared(&cancel),
        Err(Error::Entry(..))
    ));
}

#[test]
fn an_existing_index_needs_no_writer_and_a_missing_index_is_repaired() {
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, A, FIRST)]);
    let path = folder.path().join(SQLITE);
    let cancel = Cancel::default();
    let holder = rusqlite::Connection::open(&path).expect("holder");
    holder
        .execute_batch("BEGIN IMMEDIATE")
        .expect("writer lock");
    Store::open(folder.path(), Mode::Cache, false, None, &Cancel::default())
        .expect("open")
        .prepared(&cancel)
        .expect("read-only admission while a writer holds the file");
    holder.execute_batch("ROLLBACK").expect("release");
    holder
        .execute_batch("DROP INDEX answers_route_question_state")
        .expect("remove index");
    #[allow(unused_mut, reason = "Unix checks disappearance on the open store")]
    let mut repaired = Store::open(folder.path(), Mode::Cache, false, None, &Cancel::default())
        .expect("open")
        .prepared(&cancel)
        .expect("repair index");
    let count: i64 = holder
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE name='answers_route_question_state'",
            [],
            |row| row.get(0),
        )
        .expect("index exists");
    assert_eq!(count, 1);
    #[cfg(not(windows))]
    {
        drop(holder);
        fs::remove_file(&path).expect("external conversion removes sqlite");
        assert!(matches!(
            repaired.revalidate(&cancel),
            Err(Error::RecordingStorage)
        ));
    }
}

pub(super) fn scratch() -> tempdir::Scratch {
    tempdir::Scratch::new()
}

pub(super) mod tempdir {
    use std::path::{Path, PathBuf};

    /// A folder under the target directory, removed on drop.
    pub(crate) struct Scratch(PathBuf);

    impl Scratch {
        pub(super) fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let place = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("thinkthen-store-{}-{place}", std::process::id()));
            let _stale = std::fs::remove_dir_all(&path);
            Self(path)
        }

        pub(crate) fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _removed = std::fs::remove_dir_all(&self.0);
        }
    }
}

#[test]
fn a_cache_answers_a_hit_misses_the_rest_and_replaces_under_record() {
    let folder = scratch();
    let state = state();
    let mut empty =
        Store::open(folder.path(), Mode::Cache, false, None, &Cancel::default()).expect("open");
    assert_eq!(
        empty.lookup(&[key(A)], &Cancel::default()).expect("lookup"),
        [None]
    );
    assert!(!folder.path().exists(), "a lookup creates nothing");
    written(folder.path(), &[row(&state, A, FIRST)]);
    let mut store =
        Store::open(folder.path(), Mode::Cache, false, None, &Cancel::default()).expect("open");
    let found = store
        .lookup(&[key(B), key(A)], &Cancel::default())
        .expect("lookup");
    assert_eq!(found[0], None);
    let hit = found[1].as_ref().expect("a hit");
    assert_eq!(
        (hit.answer.as_str(), hit.answered_by.as_str()),
        (FIRST, "jev-1")
    );
    assert_eq!(
        hit.usage,
        Some(crate::core::ReportedUsage::from_complete(Usage::new(7, 1)))
    );

    let mut record =
        Store::open(folder.path(), Mode::Record, false, None, &Cancel::default()).expect("open");
    assert!(!record.looks_up() && record.writes());
    record
        .write(&[row(&state, A, SECOND)], &Cancel::default())
        .expect("replace");
    let replaced = Store::open(folder.path(), Mode::Replay, false, None, &Cancel::default())
        .expect("open")
        .lookup(&[key(A)], &Cancel::default())
        .expect("lookup");
    assert_eq!(
        replaced[0].as_ref().map(|found| found.answer.as_str()),
        Some(SECOND)
    );
}

#[cfg(unix)]
#[test]
fn a_new_store_is_private_and_a_read_only_replay_writes_nothing() {
    use std::os::unix::fs::PermissionsExt as _;
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, A, FIRST)]);
    let mode = |path: &Path| fs::metadata(path).expect("metadata").permissions().mode() & 0o777;
    assert_eq!(mode(folder.path()), 0o700);
    assert_eq!(mode(&folder.path().join(SQLITE)), 0o600);
    let names = || {
        let mut names: Vec<_> = fs::read_dir(folder.path())
            .expect("list")
            .map(|entry| entry.expect("entry").file_name())
            .collect();
        names.sort();
        names
    };
    let before = (
        names(),
        fs::read(folder.path().join(SQLITE)).expect("bytes"),
    );
    let mut replay =
        Store::open(folder.path(), Mode::Replay, false, None, &Cancel::default()).expect("open");
    assert!(replay.replays() && !replay.writes());
    assert!(
        replay
            .lookup(&[key(A)], &Cancel::default())
            .expect("lookup")[0]
            .is_some()
    );
    drop(replay);
    assert_eq!(
        (
            names(),
            fs::read(folder.path().join(SQLITE)).expect("bytes")
        ),
        before
    );
}

#[test]
fn a_replay_of_a_folder_holding_both_files_is_refused() {
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, A, FIRST)]);
    fs::write(folder.path().join(JSONL), "").expect("fixture");
    assert!(matches!(
        Store::open(folder.path(), Mode::Replay, false, None, &Cancel::default()),
        Err(Error::StoreAmbiguous)
    ));
}

#[test]
fn a_fixture_replays_from_memory_and_writes_the_same_bytes_again() {
    let folder = scratch();
    let state = state();
    written(
        folder.path(),
        &[row(&state, B, SECOND), row(&state, A, FIRST)],
    );
    let connection = super::read_only(&folder.path().join(SQLITE)).expect("open");
    let text = Entries::read(&connection)
        .expect("read")
        .written()
        .expect("text");
    drop(connection);
    assert_eq!(
        Entries::parse(&text)
            .expect("parse")
            .written()
            .expect("text"),
        text
    );
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].starts_with("{\"sha256\":\"4f9c0ce7"));
    assert!(lines[1] < lines[2], "answers sort by key");

    let fixture = scratch();
    fs::create_dir_all(fixture.path()).expect("folder");
    fs::write(fixture.path().join(JSONL), &text).expect("fixture");
    let found = Store::open(
        fixture.path(),
        Mode::Replay,
        false,
        None,
        &Cancel::default(),
    )
    .expect("open")
    .lookup(&[key(A), key(B)], &Cancel::default())
    .expect("lookup");
    assert!(found.iter().all(Option::is_some));
    assert!(!fixture.path().join(SQLITE).exists());

    let edited = text.replacen("noul", "unknown", 1);
    fs::write(fixture.path().join(JSONL), edited).expect("edit");
    assert!(matches!(
        Store::open(fixture.path(), Mode::Replay, false, None, &Cancel::default()),
        Err(Error::Entry(name, why)) if name == JSONL && !why.is_empty()
    ));
}

#[test]
fn a_lookup_waits_through_another_writer_and_then_answers() {
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, A, FIRST)]);
    let mut store =
        Store::open(folder.path(), Mode::Cache, false, None, &Cancel::default()).expect("open");
    let holder = rusqlite::Connection::open(folder.path().join(SQLITE)).expect("open");
    holder.execute_batch("BEGIN EXCLUSIVE").expect("lock");
    // The flag goes up just before the commit, so a lookup that answered
    // without waiting for the writer sees it down (ticket 0352).
    let committing = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let released = std::thread::spawn({
        let committing = std::sync::Arc::clone(&committing);
        move || {
            std::thread::sleep(Duration::from_millis(300));
            committing.store(true, std::sync::atomic::Ordering::SeqCst);
            holder.execute_batch("COMMIT").expect("unlock");
        }
    });
    let found = store.lookup(&[key(A)], &Cancel::default()).expect("lookup");
    assert!(found[0].is_some());
    assert!(
        committing.load(std::sync::atomic::Ordering::SeqCst),
        "the lookup answered before the writer let go"
    );
    released.join().expect("holder");
}

#[test]
fn a_wait_past_the_busy_limit_is_a_storage_failure_and_a_stop_ends_it() {
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, A, FIRST)]);
    let mut store = Store::open(folder.path(), Mode::Cache, false, None, &Cancel::default())
        .expect("open")
        .with_busy_limit(Duration::from_millis(200));
    let mut writer =
        Store::open(folder.path(), Mode::Cache, false, None, &Cancel::default()).expect("open");
    let holder = rusqlite::Connection::open(folder.path().join(SQLITE)).expect("open");
    holder.execute_batch("BEGIN EXCLUSIVE").expect("lock");
    let started = Instant::now();
    assert!(matches!(
        store.lookup(&[key(A)], &Cancel::default()),
        Err(Error::RecordingStorage)
    ));
    let waited = started.elapsed();
    // The holder never lets go, so only the limit ends this wait; the bound is a hang guard.
    assert!(
        waited < Duration::from_secs(10),
        "the limit ends the wait: {waited:?}"
    );
    let cancel = Cancel::default();
    cancel.fire();
    let started = Instant::now();
    assert!(matches!(
        writer.write(&[row(&state, B, FIRST)], &cancel),
        Err(Error::Cancelled)
    ));
    let waited = started.elapsed();
    assert!(
        waited < Duration::from_secs(10),
        "the stop ends the wait: {waited:?}"
    );
    holder.execute_batch("ROLLBACK").expect("unlock");
}

#[test]
fn a_read_only_replay_that_meets_an_unfinished_write_is_refused() {
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, A, FIRST)]);
    let writer = rusqlite::Connection::open(folder.path().join(SQLITE)).expect("open");
    writer
        .execute_batch("PRAGMA cache_size = 1; BEGIN IMMEDIATE;")
        .expect("begin");
    for place in 0..400 {
        writer
            .execute(
                "INSERT INTO states (sha256, state) VALUES (randomblob(32), ?1)",
                [format!("{place:0>2000}")],
            )
            .expect("spill");
    }
    let hot = scratch();
    fs::create_dir_all(hot.path()).expect("folder");
    for name in [SQLITE.to_owned(), format!("{SQLITE}-journal")] {
        fs::copy(folder.path().join(&name), hot.path().join(&name)).expect("copy");
    }
    writer.execute_batch("ROLLBACK").expect("rollback");
    assert!(matches!(
        Store::open(hot.path(), Mode::Replay, false, None, &Cancel::default()),
        Err(Error::StoreHotJournal)
    ));
}

#[test]
fn a_merge_refuses_conflicting_history_and_keeps_identical_snapshots() {
    let entries = |answer: &str, taken_at: i64| {
        let state = state();
        let mut entries = Entries::default();
        let digest = crate::core::hex(state.sha256());
        entries
            .states
            .insert(digest.clone(), state.json().to_owned());
        entries.answers.insert(
            key(A).hex(),
            super::Answer {
                key_version: Some(2),
                adapter: Some(crate::core::adapters::built_in::NAME.into()),
                observation_id: Some(crate::core::ObservationId::new("a".repeat(64)).unwrap()),
                batch_size: None,
                key: key(A).hex(),
                url: URL.into(),
                model: "jev-1".into(),
                state: digest,
                question: A.into(),
                answer: answer.into(),
                answered_by: "jev-1".into(),
                input_tokens: None,
                output_tokens: None,
                taken_at,
                origin: "converted".into(),
            },
        );
        entries
    };
    let mut held = entries(FIRST, 5);
    held.merge(entries(FIRST, 5)).unwrap();
    for next in [entries(SECOND, 5), entries(FIRST, 0), entries(FIRST, 6)] {
        assert!(held.merge(next).is_err());
        assert_eq!(held, entries(FIRST, 5));
    }
}
