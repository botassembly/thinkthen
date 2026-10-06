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
use crate::core::pack::{QuestionKey, State, model_json};
use crate::core::{ReportedUsage, Url, bytes_sha256, hex};
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) key_version: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) adapter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) observation_id: Option<crate::core::ObservationId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) batch_size: Option<std::num::NonZeroU32>,
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
    Original(super::Original),
}

/// Every state and answer of one store, keyed as the fixture sorts them.
#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Entries {
    pub(crate) originals: BTreeMap<String, super::Original>,
    /// State JSON by its SHA-256 in hex.
    pub(crate) states: BTreeMap<String, String>,
    /// Answers by key in hex.
    pub(crate) answers: BTreeMap<String, Answer>,
}

impl Entries {
    /// Merge identical snapshots, refusing any conflicting answer or history.
    pub(crate) fn merge(&mut self, other: Self) -> Result<(), Error> {
        for (key, answer) in &other.answers {
            if self.answers.get(key).is_some_and(|held| held != answer) {
                return Err(super::versioned::invalid());
            }
        }
        for (digest, state) in &other.states {
            if self.states.get(digest).is_some_and(|held| held != state) {
                return Err(super::versioned::invalid());
            }
        }
        for (digest, original) in &other.originals {
            if self
                .originals
                .get(digest)
                .is_some_and(|held| held != original)
            {
                return Err(super::versioned::invalid());
            }
        }
        self.originals.extend(other.originals);
        self.states.extend(other.states);
        self.answers.extend(other.answers);
        Ok(())
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
        for original in self.originals.values() {
            text.push_str(
                &serde_json::to_string(original).map_err(|_| super::versioned::invalid())?,
            );
            text.push('\n');
        }
        Ok(text)
    }

    /// Take one fixture line, or say why it is refused.
    fn take(&mut self, line: &str) -> Result<(), &'static str> {
        match serde_json::from_str::<Line>(line).map_err(|_| "is not a question entry")? {
            Line::State(state) => {
                if bytes_sha256(state.state.as_bytes()) != state.sha256
                    && hex(&State::image_sha256(&state.state)) != state.sha256
                {
                    return Err("records a state under another digest");
                }
                self.states.insert(state.sha256, state.state);
            }
            Line::Original(original) => {
                original
                    .validate()
                    .map_err(|_| "holds invalid original exchange bodies")?;
                if self
                    .originals
                    .get(&original.digest)
                    .is_some_and(|held| held != &original)
                {
                    return Err("holds conflicting original exchange bodies");
                }
                self.originals.insert(original.digest.clone(), original);
            }
            Line::Answer(answer) => self.take_answer(answer, line)?,
        }
        Ok(())
    }

    fn take_answer(&mut self, mut answer: Answer, line: &str) -> Result<(), &'static str> {
        let state = self
            .states
            .get(&answer.state)
            .ok_or("names a state no earlier line holds")?;
        if answer.key_version.unwrap_or(1) == 1 {
            if key_of(&answer, state).map(|key| key.hex()) != Some(answer.key.clone()) {
                return Err("records a different question, so the file was damaged or hand-edited");
            }
            if answer.observation_id.is_none() {
                let source = crate::core::Json::parse(line).map_err(|_| "has invalid metadata")?;
                answer.observation_id = Some(
                    super::versioned::legacy_id(&answer, state, Some(&source))
                        .map_err(|_| "has invalid constituents")?,
                );
            }
        }
        if self
            .answers
            .get(&answer.key)
            .is_some_and(|held| held != &answer)
        {
            return Err("repeats a question with conflicting saved history");
        }
        self.answers.insert(answer.key.clone(), answer);
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
        let mut states = connection
            .prepare("SELECT sha256, state FROM states")
            .map_err(storage)?;
        let rows = states
            .query_map([], |row| {
                Ok((hex(&row.get::<_, Vec<u8>>(0)?), row.get::<_, String>(1)?))
            })
            .map_err(storage)?;
        for state in rows {
            let (digest, state) = state.map_err(storage)?;
            if entries.states.insert(digest, state).is_some() {
                return Err(super::versioned::invalid());
            }
        }
        let version: u32 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .map_err(storage)?;
        if !matches!(version, 1 | 2) {
            return Err(super::versioned::invalid());
        }
        let mut statement = connection
            .prepare(
                "SELECT a.key, a.url, a.model, s.sha256, s.state, a.question, a.answer, a.answered_by,
                        a.input_tokens, a.output_tokens, a.taken_at, a.origin
                 FROM answers a LEFT JOIN states s ON s.id = a.state",
            )
            .map_err(storage)?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    Answer {
                        key_version: None,
                        adapter: None,
                        observation_id: None,
                        batch_size: None,
                        key: hex(&row.get::<_, Vec<u8>>(0)?),
                        url: row.get(1)?,
                        model: row.get(2)?,
                        state: hex(&row.get::<_, Vec<u8>>(3)?),
                        question: row.get(5)?,
                        answer: row.get(6)?,
                        answered_by: row.get(7)?,
                        input_tokens: super::rows::count(row, 8)?,
                        output_tokens: super::rows::count(row, 9)?,
                        taken_at: row.get(10)?,
                        origin: row.get(11)?,
                    },
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(storage)?;
        for row in rows {
            let (mut answer, state) = row.map_err(|_| super::versioned::invalid())?;
            if version == 2 {
                let key = bytes_of(&answer.key).ok_or_else(super::versioned::invalid)?;
                let (key_version, adapter, id, batch): (u32, String, String, Option<u32>) = connection.query_row(
                    "SELECT key_version, adapter, observation_id, batch_size FROM answers WHERE key=?1", [key.as_slice()],
                    |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
                ).map_err(storage)?;
                answer.key_version = Some(key_version);
                answer.adapter = Some(adapter);
                answer.observation_id = Some(
                    crate::core::ObservationId::new(id).map_err(|_| super::versioned::invalid())?,
                );
                answer.batch_size = match batch {
                    Some(batch) => Some(
                        std::num::NonZeroU32::new(batch).ok_or_else(super::versioned::invalid)?,
                    ),
                    None => None,
                };
            }
            entries.states.insert(answer.state.clone(), state);
            entries.answers.insert(answer.key.clone(), answer);
        }
        entries.read_originals(connection)?;
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
        for answer in self.answers.values() {
            let key = bytes_of(&answer.key).ok_or_else(unconvertible)?;
            let state = ids.get(answer.state.as_str()).ok_or_else(unconvertible)?;
            connection
                .execute(
                    "INSERT OR IGNORE INTO answers (key, url, model, state, question, answer, answered_by, input_tokens, output_tokens, taken_at, origin, key_version, adapter, observation_id, batch_size)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                    rusqlite::params![
                        key.as_slice(),
                        answer.url,
                        answer.model,
                        state,
                        answer.question,
                        answer.answer,
                        answer.answered_by,
                        super::signed(answer.input_tokens)?,
                        super::signed(answer.output_tokens)?,
                        answer.taken_at,
                        answer.origin,
                        answer.key_version, answer.adapter,
                        answer.observation_id.as_ref().map(crate::core::ObservationId::as_str),
                        answer.batch_size.map(std::num::NonZeroU32::get),
                    ],
                )?;
        }
        for original in self.originals.values() {
            original.insert(connection)?;
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
pub(crate) struct Replayed(HashMap<[u8; 32], Vec<Found>>);

impl Replayed {
    pub(crate) fn of(folder: &Path) -> Result<Option<Arc<Self>>, Error> {
        let (sqlite, jsonl) = (folder.join(SQLITE), folder.join(JSONL));
        if !folder.is_dir() || !exists(&jsonl)? || exists(&sqlite)? {
            return Ok(None);
        }
        Self::read(&jsonl).map(|read| Some(Arc::new(read)))
    }

    pub(super) fn read(path: &Path) -> Result<Self, Error> {
        Self::indexed(read(path)?)
    }

    pub(super) fn connection(connection: &Connection) -> Result<Self, Error> {
        connection.execute_batch("BEGIN").map_err(storage)?;
        let indexed = Entries::read(connection)
            .and_then(Self::indexed)
            .map_err(super::versioned::sqlite_error);
        let ended = connection.execute_batch("ROLLBACK").map_err(storage);
        indexed.and_then(|index| ended.map(|()| index))
    }

    fn indexed(entries: Entries) -> Result<Self, Error> {
        let entries = entries.normalized()?;
        let mut index: HashMap<[u8; 32], Vec<Found>> = HashMap::new();
        for answer in entries.answers.into_values() {
            let state = entries
                .states
                .get(&answer.state)
                .ok_or_else(super::versioned::invalid)?;
            let lookup = super::versioned::lookup_key(&answer, state)?;
            let usage = Some(ReportedUsage::new(
                answer.input_tokens,
                answer.output_tokens,
            ))
            .filter(|usage| usage.input_tokens().is_some() || usage.output_tokens().is_some());
            let found = Found {
                key: QuestionKey::parse(&answer.key).ok_or_else(super::versioned::invalid)?,
                observation_id: answer
                    .observation_id
                    .ok_or_else(super::versioned::invalid)?,
                batch_size: answer.batch_size,
                answer: answer.answer,
                answered_by: answer.answered_by,
                usage,
            };
            index.entry(*lookup.bytes()).or_default().push(found);
        }
        Ok(Self(index))
    }

    pub(super) fn get(&self, key: &QuestionKey) -> Result<Option<Found>, Error> {
        match self.0.get(key.bytes()).map(Vec::as_slice) {
            None | Some([]) => Ok(None),
            Some([found]) => Ok(Some(found.clone())),
            Some(_) => Err(Error::Entry(
                JSONL.to_owned(),
                "multiple historical reported models match this question".to_owned(),
            )),
        }
    }
}

/// The key an answer line's parts hash to.
pub(crate) fn key_of(answer: &Answer, state: &str) -> Option<QuestionKey> {
    let url = Url::new(answer.url.clone()).ok()?;
    let model = model_json(&answer.model).ok()?;
    if hex(&State::image_sha256(state)) == answer.state {
        Some(QuestionKey::images_of(
            &url,
            &model,
            state,
            &answer.question,
        ))
    } else {
        Some(QuestionKey::of(&url, &model, state, &answer.question))
    }
}

/// The error for an entry whose digest or state cannot be written.
fn unconvertible() -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure("an entry's digest or state is unusable".into())
}

fn bytes_of(text: &str) -> Option<[u8; 32]> {
    QuestionKey::parse(text).map(|key| *key.bytes())
}
