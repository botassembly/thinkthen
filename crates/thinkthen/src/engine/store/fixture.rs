//! The committed fixture: every state sorted by digest, then every answer
//! sorted by key, one JSON object per line, with the store's columns. The
//! bytes are a function of the entries, so a review reads them as text.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;
use std::sync::Arc;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use super::{Found, JSONL, SQLITE, exists, storage};
use crate::core::pack::{QuestionKey, model_json};
use crate::core::{Url, Usage, bytes_sha256, hex};
use crate::engine::error::Error;

/// One state line.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct StateLine {
    sha256: String,
    state: String,
}

/// One answer line, `state` naming its state's digest.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Answer {
    pub(crate) key: String,
    pub(crate) url: String,
    pub(crate) model: String,
    pub(crate) state: String,
    pub(crate) question: String,
    pub(crate) answer: String,
    pub(crate) answered_by: String,
    pub(crate) input_tokens: Option<u64>,
    pub(crate) output_tokens: Option<u64>,
    pub(crate) taken_at: i64,
    pub(crate) origin: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Line {
    State(StateLine),
    Answer(Answer),
}

/// Every state and answer of one store, keyed as the fixture sorts them.
#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Entries {
    /// State JSON by its SHA-256 in hex.
    pub(crate) states: BTreeMap<String, String>,
    /// Answers by key in hex.
    pub(crate) answers: BTreeMap<String, Answer>,
}

impl Entries {
    /// Add `other`'s answers. On one key the newer `taken_at` wins, and a
    /// tie keeps the answer already held.
    pub(crate) fn merge(&mut self, other: Self) {
        for (key, answer) in other.answers {
            let newer = self
                .answers
                .get(&key)
                .is_none_or(|held| answer.taken_at > held.taken_at);
            if !newer {
                continue;
            }
            if let Some(state) = other.states.get(&answer.state) {
                self.states.insert(answer.state.clone(), state.clone());
            }
            self.answers.insert(key, answer);
        }
    }

    /// The fixture text: states, then answers, each sorted, one per line.
    /// States no answer uses are left out.
    pub(crate) fn written(&self) -> Result<String, Error> {
        let used: std::collections::BTreeSet<&str> = self
            .answers
            .values()
            .map(|answer| answer.state.as_str())
            .collect();
        let mut text = String::new();
        for (sha256, state) in &self.states {
            if !used.contains(sha256.as_str()) {
                continue;
            }
            let line = StateLine {
                sha256: sha256.clone(),
                state: state.clone(),
            };
            text.push_str(&serde_json::to_string(&line).map_err(|_| Error::RecordingStorage)?);
            text.push('\n');
        }
        for answer in self.answers.values() {
            text.push_str(&serde_json::to_string(answer).map_err(|_| Error::RecordingStorage)?);
            text.push('\n');
        }
        Ok(text)
    }

    /// Take one fixture line, or say why it is refused.
    fn take(&mut self, line: &str) -> Result<(), &'static str> {
        match serde_json::from_str::<Line>(line).map_err(|_| "is not a question entry")? {
            Line::State(state) => {
                if bytes_sha256(state.state.as_bytes()) != state.sha256 {
                    return Err("records a state under another digest");
                }
                self.states.insert(state.sha256, state.state);
            }
            Line::Answer(answer) => {
                let state = self
                    .states
                    .get(&answer.state)
                    .ok_or("names a state no earlier line holds")?;
                if key_of(&answer, state).map(|key| key.hex()) != Some(answer.key.clone()) {
                    return Err(
                        "records a different question, so the file was damaged or hand-edited",
                    );
                }
                self.answers.insert(answer.key.clone(), answer);
            }
        }
        Ok(())
    }

    /// Read a fixture's text, checking each answer's key against its parts.
    pub(crate) fn parse(text: &str) -> Result<Self, Error> {
        let mut entries = Self::default();
        for (number, line) in text.lines().enumerate() {
            entries.take(line).map_err(|why| {
                Error::Entry(JSONL.to_owned(), format!("line {} {why}", number + 1))
            })?;
        }
        Ok(entries)
    }

    /// Read every entry of one store's database.
    pub(crate) fn read(connection: &Connection) -> Result<Self, Error> {
        let mut entries = Self::default();
        let mut statement = connection
            .prepare(
                "SELECT a.key, a.url, a.model, s.sha256, s.state, a.question, a.answer, a.answered_by,
                        a.input_tokens, a.output_tokens, a.taken_at, a.origin
                 FROM answers a JOIN states s ON s.id = a.state",
            )
            .map_err(storage)?;
        let rows = statement
            .query_map([], |row| {
                let tokens = |at| {
                    row.get::<_, Option<i64>>(at)
                        .map(|value| value.and_then(|value| u64::try_from(value).ok()))
                };
                Ok((
                    Answer {
                        key: hex(&row.get::<_, Vec<u8>>(0)?),
                        url: row.get(1)?,
                        model: row.get(2)?,
                        state: hex(&row.get::<_, Vec<u8>>(3)?),
                        question: row.get(5)?,
                        answer: row.get(6)?,
                        answered_by: row.get(7)?,
                        input_tokens: tokens(8)?,
                        output_tokens: tokens(9)?,
                        taken_at: row.get(10)?,
                        origin: row.get(11)?,
                    },
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(storage)?;
        for row in rows {
            let (answer, state) = row.map_err(storage)?;
            entries.states.insert(answer.state.clone(), state);
            entries.answers.insert(answer.key.clone(), answer);
        }
        Ok(entries)
    }

    /// Insert every entry inside the caller's transaction. An answer already
    /// held under a key stays, because a store's own writes are newer. A digest
    /// that is not hex, or an answer naming a state the entries lack, fails
    /// as a conversion to SQL.
    pub(super) fn insert_all(&self, connection: &Connection) -> rusqlite::Result<()> {
        let mut ids = BTreeMap::new();
        for (sha256, state) in &self.states {
            let digest = bytes_of(sha256).ok_or_else(unconvertible)?;
            connection.execute(
                "INSERT OR IGNORE INTO states (sha256, state) VALUES (?1, ?2)",
                (digest.as_slice(), state),
            )?;
            let id: i64 = connection.query_row(
                "SELECT id FROM states WHERE sha256 = ?1",
                [digest.as_slice()],
                |row| row.get(0),
            )?;
            ids.insert(sha256.as_str(), id);
        }
        let signed = |value: Option<u64>| value.and_then(|value| i64::try_from(value).ok());
        for answer in self.answers.values() {
            let key = bytes_of(&answer.key).ok_or_else(unconvertible)?;
            let state = ids.get(answer.state.as_str()).ok_or_else(unconvertible)?;
            connection
                .execute(
                    "INSERT OR IGNORE INTO answers (key, url, model, state, question, answer, answered_by, input_tokens, output_tokens, taken_at, origin)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                    rusqlite::params![
                        key.as_slice(),
                        answer.url,
                        answer.model,
                        state,
                        answer.question,
                        answer.answer,
                        answer.answered_by,
                        signed(answer.input_tokens),
                        signed(answer.output_tokens),
                        answer.taken_at,
                        answer.origin,
                    ],
                )?;
        }
        Ok(())
    }
}

/// Read a fixture file.
pub(super) fn read(path: &Path) -> Result<Entries, Error> {
    Entries::parse(&fs::read_to_string(path).map_err(|_| Error::RecordingStorage)?)
}

/// A replay fixture's answers by key. One engine reads its fixture once per
/// process, so a call that asks one line of many does not read it again.
#[derive(Debug)]
pub(crate) struct Replayed(HashMap<[u8; 32], Found>);

impl Replayed {
    /// The fixture a replay folder holds alone, read and checked. A folder
    /// without one, or with a live file beside it, gives `None`, and opening
    /// the store says why when that matters.
    pub(crate) fn of(folder: &Path) -> Result<Option<Arc<Self>>, Error> {
        let (sqlite, jsonl) = (folder.join(SQLITE), folder.join(JSONL));
        if !folder.is_dir() || !exists(&jsonl).unwrap_or(false) || exists(&sqlite).unwrap_or(true) {
            return Ok(None);
        }
        Self::read(&jsonl).map(|read| Some(Arc::new(read)))
    }

    pub(super) fn read(path: &Path) -> Result<Self, Error> {
        let answers = read(path)?.answers.into_values().map(|answer| {
            let key = bytes_of(&answer.key).ok_or(Error::Defect("a checked key was not hex"))?;
            let usage = answer.input_tokens.zip(answer.output_tokens);
            let found = Found {
                answer: answer.answer,
                answered_by: answer.answered_by,
                usage: usage.map(|(input, output)| Usage::new(input, output)),
            };
            Ok((key, found))
        });
        answers.collect::<Result<_, Error>>().map(Self)
    }

    pub(super) fn get(&self, key: &QuestionKey) -> Option<Found> {
        self.0.get(key.bytes()).cloned()
    }
}

/// The key an answer line's parts hash to.
pub(crate) fn key_of(answer: &Answer, state: &str) -> Option<QuestionKey> {
    let url = Url::new(answer.url.clone()).ok()?;
    let model = model_json(&answer.model).ok()?;
    Some(QuestionKey::of(&url, &model, state, &answer.question))
}

/// The error for an entry whose digest or state cannot be written.
fn unconvertible() -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure("an entry's digest or state is unusable".into())
}

fn bytes_of(text: &str) -> Option<[u8; 32]> {
    QuestionKey::parse(text).map(|key| *key.bytes())
}
