//! `thinkthen cache convert DIR`, by ADR 0111 section 9, from the outside.
#![cfg(feature = "cli")]

use crate::child::ChildEnvironment as _;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::support::reported_keys;
use serde_json::Value;

type Try<T> = Result<T, Box<dyn Error>>;

const URL: &str = "https://api.typesafe.ai/v1/systemone";
const STATE: &str = "Help! My payouts have been failing for 3 days.";
const RESPONSE: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":312,"output_tokens":48}}"#;

fn scratch(name: &str) -> Try<PathBuf> {
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("cache-convert-{name}"));
    let _removed = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder)?;
    Ok(folder)
}

fn convert(folder: &Path, extra: &[&str]) -> Try<(Option<i32>, String)> {
    let output: Output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .clear_environment()
        .home(env!("CARGO_TARGET_TMPDIR"))
        .arg("cache")
        .arg("convert")
        .arg(folder)
        .args(extra)
        .output()?;
    Ok((output.status.code(), String::from_utf8(output.stderr)?))
}

fn single() -> String {
    format!(
        r#"{{"state":"{STATE}","model":"jev-latest","questions":{{"q1":{{"type":"noul","instructions":"Does this convey urgency?"}}}}}}"#
    )
}

/// Every line of a folder's fixture.
fn lines(folder: &Path) -> Try<Vec<Value>> {
    fs::read_to_string(folder.join("thinkthen.jsonl"))?
        .lines()
        .map(|line| Ok(serde_json::from_str(line)?))
        .collect()
}

/// The answer lines alone.
fn answers(folder: &Path) -> Try<Vec<Value>> {
    Ok(lines(folder)?
        .into_iter()
        .filter(|line| line.get("key").is_some())
        .collect())
}

fn current_fixture(folder: &Path) -> Try<()> {
    crate::support::plant_recording(folder, URL, single().as_bytes(), RESPONSE).ok_or("fixture")?;
    Ok(())
}

/// The store's schema, by ADR 0111 section 3.
const SCHEMA: &str = "
CREATE TABLE states (id INTEGER PRIMARY KEY, sha256 BLOB NOT NULL UNIQUE, state TEXT NOT NULL);
CREATE TABLE answers (id INTEGER PRIMARY KEY, key BLOB NOT NULL UNIQUE, url TEXT NOT NULL,
  model TEXT NOT NULL, state INTEGER NOT NULL REFERENCES states(id), question TEXT NOT NULL,
  answer TEXT NOT NULL, answered_by TEXT NOT NULL, input_tokens INTEGER, output_tokens INTEGER,
  taken_at INTEGER NOT NULL, origin TEXT NOT NULL, adapter TEXT NOT NULL, observation_id TEXT NOT NULL, batch_size INTEGER);
PRAGMA user_version = 3;
";

fn bytes(hex: &Value) -> Try<Vec<u8>> {
    let hex = hex.as_str().ok_or("a hex string")?;
    (0..hex.len())
        .step_by(2)
        .map(|at| {
            Ok(u8::from_str_radix(
                hex.get(at..at + 2).ok_or("an even length")?,
                16,
            )?)
        })
        .collect()
}

/// Put one fixture answer line into a scratch live file under a new answer
/// and time.
fn live(
    connection: &rusqlite::Connection,
    line: &Value,
    state: &str,
    answer: &str,
    taken_at: i64,
) -> Try<()> {
    let sha = bytes(&line["state"])?;
    let request = request(line, state)?;
    let key = reported_keys(URL, request.as_bytes(), "jev-1.13.0");
    connection.execute(
        "INSERT OR IGNORE INTO states (sha256, state) VALUES (?1, ?2)",
        (&sha, state),
    )?;
    connection.execute(
        "INSERT INTO answers (key, url, model, state, question, answer, answered_by, input_tokens, output_tokens, taken_at, origin,adapter,observation_id)
         VALUES (?1, ?2, ?3, (SELECT id FROM states WHERE sha256 = ?4), ?5, ?6, 'jev-1.13.0', 1, 1, ?7, 'live','systemone',?8)",
        rusqlite::params![
            bytes(&Value::from(key.first().ok_or("one key")?.clone()))?,
            line["url"].as_str(),
            line["model"].as_str(),
            sha,
            line["question"].as_str(),
            answer,
            taken_at,
            line["observation_id"].as_str(),
        ],
    )?;
    Ok(())
}

fn request(line: &Value, state: &str) -> Try<String> {
    Ok(format!(
        r#"{{"state":{state},"model":{},"questions":{{"q1":{}}}}}"#,
        line["model"],
        line["question"].as_str().ok_or("question")?
    ))
}

#[test]
fn conflicting_fixture_and_live_histories_refuse_without_rewriting_any_source() {
    let folder = scratch("merge").expect("a folder");
    current_fixture(&folder).expect("current fixture");
    assert_eq!(convert(&folder, &[]).expect("a run").0, Some(0));
    let mut lines = lines(&folder).expect("a fixture");
    let state = lines[0]["state"].as_str().expect("a state").to_owned();
    let tie = lines[1].clone();
    lines[1]["answer"] = r#"{"type":"noul","noul":0.11}"#.into();
    let fixture: String = lines.iter().map(|line| format!("{line}\n")).collect();
    fs::write(folder.join("thinkthen.jsonl"), fixture).expect("a fixture");
    let connection =
        rusqlite::Connection::open(folder.join("thinkthen.sqlite")).expect("a live file");
    connection.execute_batch(SCHEMA).expect("the schema");
    live(
        &connection,
        &tie,
        &state,
        r#"{"type":"noul","noul":0.33}"#,
        0,
    )
    .expect("a row");
    drop(connection);

    let before = fs::read(folder.join("thinkthen.sqlite")).expect("database bytes");
    let fixture_before = fs::read(folder.join("thinkthen.jsonl")).expect("fixture bytes");
    let (code, said) = convert(&folder, &[]).expect("a run");
    assert_eq!(code, Some(5), "{said}");
    assert_eq!(
        said,
        "thinkthen: the entry `thinkthen.jsonl` was refused: stored constituents or identity are invalid\n"
    );
    assert_eq!(
        fs::read(folder.join("thinkthen.sqlite")).expect("kept database"),
        before
    );
    assert_eq!(
        fs::read(folder.join("thinkthen.jsonl")).expect("kept fixture"),
        fixture_before
    );
    assert_eq!(fs::read_dir(&folder).expect("a folder").count(), 2);
}

#[test]
fn conversion_keeps_the_committed_row_when_a_writer_holds_the_live_file() {
    let folder = scratch("locked").expect("a folder");
    current_fixture(&folder).expect("current fixture");
    assert_eq!(convert(&folder, &[]).expect("a run").0, Some(0));
    let lines = lines(&folder).expect("a fixture");
    let state = lines[0]["state"].as_str().expect("a state").to_owned();
    let mut line = lines[1].clone();
    line["question"] = line["question"]
        .as_str()
        .expect("question")
        .replace("?", "? Keep the writer's distinct row?")
        .into();
    line["key"] = reported_keys(
        URL,
        request(&line, &state).expect("request").as_bytes(),
        "jev-1.13.0",
    )[0]
    .clone()
    .into();
    let written_key = line["key"].clone();
    let sqlite = folder.join("thinkthen.sqlite");
    rusqlite::Connection::open(&sqlite)
        .expect("a live file")
        .execute_batch(SCHEMA)
        .expect("the schema");
    let (held, holding) = std::sync::mpsc::channel();
    let writer = std::thread::spawn(move || {
        let connection = rusqlite::Connection::open(sqlite).expect("a live file");
        connection
            .execute_batch("BEGIN EXCLUSIVE")
            .expect("the lock");
        // The lock's start time rides the signal, so the wait below is
        // measured from the lock itself however late this thread is woken.
        held.send(std::time::Instant::now()).expect("a signal");
        std::thread::sleep(std::time::Duration::from_millis(300));
        live(
            &connection,
            &line,
            &state,
            r#"{"type":"noul","noul":0.55}"#,
            9,
        )
        .expect("a row");
        connection.execute_batch("COMMIT").expect("a commit");
    });
    let started = holding.recv().expect("the writer holds the lock");
    let (code, said) = convert(&folder, &[]).expect("a run");
    let waited = started.elapsed();
    writer.join().expect("the writer");
    // Windows refuses the existing writable handle before publication.
    // Once that writer closes, a retry must retain its committed row.
    #[cfg(windows)]
    let (code, said) = {
        assert_eq!(code, Some(5), "{said}");
        assert!(folder.join("thinkthen.sqlite").exists());
        convert(&folder, &[]).expect("conversion after the writer closes")
    };
    assert_eq!(code, Some(0), "{said}");
    #[cfg(not(windows))]
    assert!(
        waited >= std::time::Duration::from_millis(300),
        "{waited:?}"
    );
    #[cfg(windows)]
    let _ = waited;
    let written = answers(&folder).expect("answers");
    let found = written
        .iter()
        .find(|answer| answer["key"] == written_key)
        .expect("the written key");
    assert_eq!(
        (found["answer"].as_str(), found["origin"].as_str()),
        (Some(r#"{"type":"noul","noul":0.55}"#), Some("live"))
    );
    assert!(!folder.join("thinkthen.sqlite").exists());
}

#[test]
fn a_missing_folder_or_an_unreadable_entry_exits_5_and_writes_nothing() {
    let folder = scratch("refused").expect("a folder");
    assert_eq!(
        convert(&folder.join("absent"), &[]).expect("a run"),
        (
            Some(5),
            "thinkthen: cache convert takes an existing folder, and the one named is missing or not a folder\n"
                .to_owned()
        )
    );
    fs::write(
        folder.join(format!("{}.json", "b".repeat(64))),
        "not an entry",
    )
    .expect("a file");
    assert_eq!(
        convert(&folder, &[]).expect("a run"),
        (
            Some(5),
            format!(
                "thinkthen: {}.json uses an old store format; delete it\n",
                "b".repeat(64)
            )
        )
    );
    assert!(!folder.join("thinkthen.jsonl").exists());
}
