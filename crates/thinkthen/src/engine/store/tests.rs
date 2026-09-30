//! The store's edge table, by ADR 0111 slice 2.

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use super::{Entries, JSONL, Mode, Row, SQLITE, Store};
use crate::core::pack::{QuestionKey, State};
use crate::core::{Url, Usage};
use crate::engine::Cancel;
use crate::engine::error::Error;

const URL: &str = "http://127.0.0.1:9/v1/systemone";

fn state() -> State {
    State::new(
        "\"Each question quotes the text it asks about.\"".to_owned(),
        44,
    )
}

fn key(question: &str) -> QuestionKey {
    QuestionKey::of(
        &Url::new(URL).expect("url"),
        "\"jev-1\"",
        state().json(),
        question,
    )
}

fn row<'a>(state: &'a State, question: &'a str, answer: &'a str) -> Row<'a> {
    Row {
        key: key(question),
        url: URL,
        model: "jev-1",
        state,
        question,
        answer,
        answered_by: "jev-1.2",
        usage: Some(Usage::new(7, 1)),
        taken_at: 1_700_000_000,
        origin: "live",
    }
}

fn written(folder: &Path, rows: &[Row<'_>]) {
    let mut store = Store::open(folder, Mode::Cache, false).expect("open");
    store.write(rows, &Cancel::default()).expect("write");
}

fn scratch() -> tempdir::Scratch {
    tempdir::Scratch::new()
}

mod tempdir {
    use std::path::{Path, PathBuf};

    /// A folder under the target directory, removed on drop.
    pub(super) struct Scratch(PathBuf);

    impl Scratch {
        pub(super) fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let place = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("thinkthen-store-{}-{place}", std::process::id()));
            let _stale = std::fs::remove_dir_all(&path);
            Self(path)
        }

        pub(super) fn path(&self) -> &Path {
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
    let mut empty = Store::open(folder.path(), Mode::Cache, false).expect("open");
    assert_eq!(
        empty
            .lookup(&[key("a")], &Cancel::default())
            .expect("lookup"),
        [None]
    );
    assert!(!folder.path().exists(), "a lookup creates nothing");
    written(folder.path(), &[row(&state, "a", "{\"n\":1}")]);
    let mut store = Store::open(folder.path(), Mode::Cache, false).expect("open");
    let found = store
        .lookup(&[key("b"), key("a")], &Cancel::default())
        .expect("lookup");
    assert_eq!(found[0], None);
    let hit = found[1].as_ref().expect("a hit");
    assert_eq!(
        (hit.answer.as_str(), hit.answered_by.as_str()),
        ("{\"n\":1}", "jev-1.2")
    );
    assert_eq!(hit.usage, Some(Usage::new(7, 1)));

    let mut record = Store::open(folder.path(), Mode::Record, false).expect("open");
    assert!(!record.looks_up() && record.writes());
    record
        .write(&[row(&state, "a", "{\"n\":2}")], &Cancel::default())
        .expect("replace");
    let replaced = Store::open(folder.path(), Mode::Replay, false)
        .expect("open")
        .lookup(&[key("a")], &Cancel::default())
        .expect("lookup");
    assert_eq!(
        replaced[0].as_ref().map(|found| found.answer.as_str()),
        Some("{\"n\":2}")
    );
}

#[cfg(unix)]
#[test]
fn a_new_store_is_private_and_a_read_only_replay_writes_nothing() {
    use std::os::unix::fs::PermissionsExt as _;
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, "a", "{}")]);
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
    let mut replay = Store::open(folder.path(), Mode::Replay, false).expect("open");
    assert!(replay.replays() && !replay.writes());
    assert!(
        replay
            .lookup(&[key("a")], &Cancel::default())
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
    written(folder.path(), &[row(&state, "a", "{}")]);
    fs::write(folder.path().join(JSONL), "").expect("fixture");
    assert!(matches!(
        Store::open(folder.path(), Mode::Replay, false),
        Err(Error::StoreAmbiguous)
    ));
}

#[test]
fn a_fixture_replays_from_memory_and_writes_the_same_bytes_again() {
    let folder = scratch();
    let state = state();
    written(
        folder.path(),
        &[row(&state, "b", "{\"n\":2}"), row(&state, "a", "{\"n\":1}")],
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
    let found = Store::open(fixture.path(), Mode::Replay, false)
        .expect("open")
        .lookup(&[key("a"), key("b")], &Cancel::default())
        .expect("lookup");
    assert!(found.iter().all(Option::is_some));
    assert!(!fixture.path().join(SQLITE).exists());

    let edited = text.replacen("{\\\"n\\\":1}", "{\\\"n\\\":9}", 1).replacen(
        "\"question\":\"a\"",
        "\"question\":\"c\"",
        1,
    );
    fs::write(fixture.path().join(JSONL), edited).expect("edit");
    assert!(matches!(
        Store::open(fixture.path(), Mode::Replay, false),
        Err(Error::Entry(name, why)) if name == JSONL && why.contains("hand-edited")
    ));
}

#[test]
fn a_lookup_waits_through_another_writer_and_then_answers() {
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, "a", "{}")]);
    let mut store = Store::open(folder.path(), Mode::Cache, false).expect("open");
    let holder = rusqlite::Connection::open(folder.path().join(SQLITE)).expect("open");
    holder.execute_batch("BEGIN EXCLUSIVE").expect("lock");
    let released = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(300));
        holder.execute_batch("COMMIT").expect("unlock");
    });
    let started = Instant::now();
    let found = store
        .lookup(&[key("a")], &Cancel::default())
        .expect("lookup");
    assert!(found[0].is_some());
    assert!(started.elapsed() >= Duration::from_millis(250));
    released.join().expect("holder");
}

#[test]
fn a_wait_past_the_busy_limit_is_a_storage_failure_and_a_stop_ends_it() {
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, "a", "{}")]);
    let mut store = Store::open(folder.path(), Mode::Cache, false)
        .expect("open")
        .with_busy_limit(Duration::from_millis(200));
    let mut writer = Store::open(folder.path(), Mode::Cache, false).expect("open");
    let holder = rusqlite::Connection::open(folder.path().join(SQLITE)).expect("open");
    holder.execute_batch("BEGIN EXCLUSIVE").expect("lock");
    assert!(matches!(
        store.lookup(&[key("a")], &Cancel::default()),
        Err(Error::RecordingStorage)
    ));
    let cancel = Cancel::default();
    cancel.fire();
    assert!(matches!(
        writer.write(&[row(&state, "b", "{}")], &cancel),
        Err(Error::Cancelled)
    ));
    holder.execute_batch("ROLLBACK").expect("unlock");
}

#[test]
fn a_read_only_replay_that_meets_an_unfinished_write_is_refused() {
    let folder = scratch();
    let state = state();
    written(folder.path(), &[row(&state, "a", "{}")]);
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
    let mut store = Store::open(hot.path(), Mode::Replay, false).expect("open");
    assert!(matches!(
        store.lookup(&[key("a")], &Cancel::default()),
        Err(Error::StoreHotJournal)
    ));
}

#[test]
fn a_merge_keeps_the_newer_answer_and_a_tie_keeps_the_held_one() {
    let entries = |answer: &str, taken_at: i64| {
        let state = state();
        let mut text = format!(
            "{{\"sha256\":\"{}\",\"state\":{}}}\n",
            crate::core::hex(state.sha256()),
            serde_json::to_string(state.json()).expect("json")
        );
        let line = super::Answer {
            key: key("a").hex(),
            url: URL.to_owned(),
            model: "jev-1".to_owned(),
            state: crate::core::hex(state.sha256()),
            question: "a".to_owned(),
            answer: answer.to_owned(),
            answered_by: "jev-1.2".to_owned(),
            input_tokens: None,
            output_tokens: None,
            taken_at,
            origin: "converted".to_owned(),
        };
        text.push_str(&serde_json::to_string(&line).expect("json"));
        Entries::parse(&text).expect("parse")
    };
    let answer = |entries: &Entries| entries.answers[&key("a").hex()].answer.clone();
    let mut held = entries("held", 5);
    held.merge(entries("tie", 5));
    assert_eq!(answer(&held), "held");
    held.merge(entries("older", 0));
    assert_eq!(answer(&held), "held");
    held.merge(entries("newer", 6));
    assert_eq!(answer(&held), "newer");
}
