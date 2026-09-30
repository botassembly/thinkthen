//! `thinkthen cache convert DIR`, by ADR 0111 section 9, from the outside.
#![cfg(feature = "cli")]

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

type Try<T> = Result<T, Box<dyn Error>>;

const URL: &str = "https://api.typesafe.ai/v1/systemone";
const STATE: &str = "Help! My payouts have been failing for 3 days.";
const QUOTED_STATE: &str = "Each question quotes the text it asks about.";
const RESPONSE: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":312,"output_tokens":48}}"#;

/// The key of the single-record question below, hashed outside the program:
///
/// printf 'systemone\nhttps://api.typesafe.ai/v1/systemone\n"jev-latest"\n"Help! My payouts have been failing for 3 days."\n{"type":"noul","instructions":"Does this convey urgency?"}' | sha256sum
const PINNED: &str = "38357d5b4962da56df2b8cace1bacfd88b305db13a4c64e593760b8b0d626897";

/// The key of the same question in the quoted form, hashed outside the program:
///
/// printf 'systemone\nhttps://api.typesafe.ai/v1/systemone\n"jev-latest"\n"Each question quotes the text it asks about."\n{"type":"noul","instructions":"The text is \\"Help! My payouts have been failing for 3 days.\\". Does this convey urgency?"}' | sha256sum
const PINNED_QUOTED: &str = "b27ab9810bcbe97851ee1dbd5a9b2ef206f33705212020d38dd607359aea5ac6";

fn scratch(name: &str) -> Try<PathBuf> {
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("cache-convert-{name}"));
    let _removed = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder)?;
    Ok(folder)
}

fn convert(folder: &Path, extra: &[&str]) -> Try<(Option<i32>, String)> {
    let output: Output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .arg("cache")
        .arg("convert")
        .arg(folder)
        .args(extra)
        .output()?;
    Ok((output.status.code(), String::from_utf8(output.stderr)?))
}

/// Write one old entry under a digest-shaped name of one repeated figure.
fn old_entry(folder: &Path, figure: char, request: &str, quoted: bool) -> Try<()> {
    let flag = if quoted { ",\n  \"quoted\": true" } else { "" };
    let entry = format!(
        "{{\n  \"schema\": \"thinkthen.recording/1\",\n  \"adapter\": \"systemone\",\n  \"url\": \"{URL}\",\n  \"request\": {request},\n  \"response\": {RESPONSE}{flag}\n}}\n"
    );
    fs::write(
        folder.join(format!("{}.json", figure.to_string().repeat(64))),
        entry,
    )?;
    Ok(())
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

/// Two quoted old entries that share one state, as a folder recorded
/// before ADR 0111 holds them.
fn old_folder(folder: &Path) -> Try<()> {
    old_entry(folder, 'a', &single(), true)?;
    old_entry(
        folder,
        'b',
        &single().replace("Does this convey urgency?", "Is it resolved?"),
        true,
    )
}

#[test]
fn converting_old_entries_twice_writes_identical_bytes_and_keeps_every_old_file() {
    let folder = scratch("twice").expect("a folder");
    old_folder(&folder).expect("old entries");
    assert_eq!(
        convert(&folder, &[]).expect("a run"),
        (
            Some(0),
            "thinkthen: cache convert: wrote 2 answers to thinkthen.jsonl, 2 from old entries; skipped 0 entries\n"
                .to_owned()
        )
    );
    let written = fs::read(folder.join("thinkthen.jsonl")).expect("a fixture");
    let answers = answers(&folder).expect("answers");
    assert_eq!(answers.len(), 2);
    assert!(
        answers
            .iter()
            .all(|answer| answer["origin"] == "quoted" && answer["taken_at"] == 0)
    );
    assert_eq!(convert(&folder, &[]).expect("a run").0, Some(0));
    assert_eq!(
        fs::read(folder.join("thinkthen.jsonl")).expect("a fixture"),
        written
    );
    assert_eq!(fs::read_dir(&folder).expect("a folder").count(), 3);
}

/// One old form and each answer it writes, as key, state and origin. An
/// empty key is not pinned.
struct Case {
    name: &'static str,
    request: String,
    quoted: bool,
    options: &'static [&'static str],
    written: Vec<(&'static str, &'static str, &'static str)>,
}

#[test]
fn each_old_form_converts_to_its_keys_states_and_origins() {
    let records = r#"{"state":{"records":[{"id":1}]},"model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"The text is {\"id\":1}. Is it red?"}}}"#;
    let case = |name, request: String, quoted, options, written| Case {
        name,
        request,
        quoted,
        options,
        written,
    };
    let cases = [
        case(
            "single",
            single(),
            false,
            &[],
            vec![(PINNED, STATE, "converted")],
        ),
        case(
            "flagged",
            single(),
            true,
            &[],
            vec![(PINNED, STATE, "quoted")],
        ),
        case(
            "quote",
            single(),
            false,
            &["--quote"],
            vec![
                (PINNED, STATE, "converted"),
                (PINNED_QUOTED, QUOTED_STATE, "quoted"),
            ],
        ),
        case(
            "records",
            records.to_owned(),
            false,
            &["--quote"],
            vec![("", QUOTED_STATE, "converted")],
        ),
        case("unjoined", single().replace(':', ": "), false, &[], vec![]),
    ];
    for case in cases {
        let name = case.name;
        let folder = scratch(name).expect("a folder");
        old_entry(&folder, 'a', &case.request, case.quoted).expect("an entry");
        let (code, said) = convert(&folder, case.options).expect("a run");
        assert_eq!(code, Some(0), "{name}: {said}");
        let lines = lines(&folder).expect("a fixture");
        let written = answers(&folder).expect("answers");
        assert_eq!(written.len(), case.written.len(), "{name}");
        for (answer, (key, state, origin)) in written.iter().zip(case.written) {
            if !key.is_empty() {
                assert_eq!(answer["key"], key, "{name}");
            }
            let held = lines
                .iter()
                .find(|line| line["sha256"] == answer["state"])
                .expect("its state");
            assert_eq!(
                held["state"],
                serde_json::to_string(state).expect("JSON"),
                "{name}"
            );
            assert_eq!(answer["origin"], origin, "{name}");
            assert_eq!(
                (
                    answer["input_tokens"].as_u64(),
                    answer["output_tokens"].as_u64()
                ),
                (Some(312), Some(48))
            );
            assert_eq!(answer["answered_by"], "jev-1.13.0", "{name}");
        }
        if name == "unjoined" {
            assert_eq!(
                said,
                format!(
                    "thinkthen: cache convert: skipped `{}.json`; its request does not rejoin byte for byte from its parts\n\
                     thinkthen: cache convert: wrote 0 answers to thinkthen.jsonl, 0 from old entries; skipped 1 entries\n",
                    "a".repeat(64)
                )
            );
        }
    }
}

/// The store's schema, by ADR 0111 section 3.
const SCHEMA: &str = "
CREATE TABLE states (id INTEGER PRIMARY KEY, sha256 BLOB NOT NULL UNIQUE, state TEXT NOT NULL);
CREATE TABLE answers (id INTEGER PRIMARY KEY, key BLOB NOT NULL UNIQUE, url TEXT NOT NULL,
  model TEXT NOT NULL, state INTEGER NOT NULL REFERENCES states(id), question TEXT NOT NULL,
  answer TEXT NOT NULL, answered_by TEXT NOT NULL, input_tokens INTEGER, output_tokens INTEGER,
  taken_at INTEGER NOT NULL, origin TEXT NOT NULL);
PRAGMA user_version = 1;
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
    connection.execute(
        "INSERT OR IGNORE INTO states (sha256, state) VALUES (?1, ?2)",
        (&sha, state),
    )?;
    connection.execute(
        "INSERT INTO answers (key, url, model, state, question, answer, answered_by, input_tokens, output_tokens, taken_at, origin)
         VALUES (?1, ?2, ?3, (SELECT id FROM states WHERE sha256 = ?4), ?5, ?6, 'jev-1.13.0', 1, 1, ?7, 'live')",
        rusqlite::params![
            bytes(&line["key"])?,
            line["url"].as_str(),
            line["model"].as_str(),
            sha,
            line["question"].as_str(),
            answer,
            taken_at,
        ],
    )?;
    Ok(())
}

#[test]
fn a_fixture_a_live_file_and_old_entries_merge_newest_first_and_a_tie_keeps_the_fixture() {
    let folder = scratch("merge").expect("a folder");
    old_folder(&folder).expect("old entries");
    assert_eq!(convert(&folder, &[]).expect("a run").0, Some(0));
    let mut lines = lines(&folder).expect("a fixture");
    let state = lines[0]["state"].as_str().expect("a state").to_owned();
    let (tie, newer) = (lines[1].clone(), lines[2].clone());
    lines[1]["answer"] = r#"{"type":"noul","noul":0.11}"#.into();
    lines[2]["answer"] = r#"{"type":"noul","noul":0.22}"#.into();
    lines[2]["taken_at"] = 5.into();
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
    live(
        &connection,
        &newer,
        &state,
        r#"{"type":"noul","noul":0.44}"#,
        9,
    )
    .expect("a row");
    drop(connection);

    let (code, said) = convert(&folder, &[]).expect("a run");
    assert_eq!(code, Some(0), "{said}");
    let merged = answers(&folder).expect("answers");
    let found = |key: &Value| {
        merged
            .iter()
            .find(|answer| &answer["key"] == key)
            .expect("a merged key")
    };
    assert_eq!(
        found(&tie["key"])["answer"],
        r#"{"type":"noul","noul":0.11}"#
    );
    let newest = found(&newer["key"]);
    assert_eq!(
        (
            newest["answer"].as_str(),
            newest["taken_at"].as_i64(),
            newest["origin"].as_str()
        ),
        (
            Some(r#"{"type":"noul","noul":0.44}"#),
            Some(9),
            Some("live")
        )
    );
    assert!(!folder.join("thinkthen.sqlite").exists());
    assert_eq!(fs::read_dir(&folder).expect("a folder").count(), 3);
}

#[test]
fn convert_waits_for_a_writer_holding_the_live_file_and_keeps_its_row() {
    let folder = scratch("locked").expect("a folder");
    old_folder(&folder).expect("old entries");
    assert_eq!(convert(&folder, &[]).expect("a run").0, Some(0));
    let lines = lines(&folder).expect("a fixture");
    let state = lines[0]["state"].as_str().expect("a state").to_owned();
    let line = lines[1].clone();
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
        held.send(()).expect("a signal");
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
    holding.recv().expect("the writer holds the lock");
    let started = std::time::Instant::now();
    let (code, said) = convert(&folder, &[]).expect("a run");
    let waited = started.elapsed();
    writer.join().expect("the writer");
    assert_eq!(code, Some(0), "{said}");
    assert!(
        waited >= std::time::Duration::from_millis(250),
        "{waited:?}"
    );
    let written = answers(&folder).expect("answers");
    let found = written
        .iter()
        .find(|answer| answer["key"] == lines[1]["key"])
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
                "thinkthen: the entry `{}.json` was refused: the file is not a recording entry: the JSON at line 1 column 2 is not one\n",
                "b".repeat(64)
            )
        )
    );
    assert!(!folder.join("thinkthen.jsonl").exists());
}
