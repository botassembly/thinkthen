//! Four keyed judgment tables and the keyed rank table, with eight
//! connection-owned packed answer slots.

use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::{CStr, c_int};
use std::sync::Arc;

use rusqlite::ffi::sqlite3;
use rusqlite::types::{Value, ValueRef};
use rusqlite::vtab::Filters;
use serde::de::{Error as _, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use thinkthen::{For, Judgment, Probabilities, Question, Settings};

use crate::question::{Stamp, call_settings, question_with_settings, stamp};
use crate::tables::{Scan, Table};
use crate::{Failure, worker};

type Rows = Arc<Vec<Vec<Value>>>;
type HeldRows = (Rows, Arc<HashMap<String, usize>>);

/// The rank table's name, whose question never names a file.
const RANK: &str = "thinkthen_rank";

/// A slot owns its argument bytes; no SQLite-owned pointer escapes xFilter.
#[derive(Debug)]
struct Slot {
    kind: &'static str,
    /// Comparison only; never dereference an address after xFilter returns.
    pointers: [usize; 3],
    question: Vec<u8>,
    keyed: Vec<u8>,
    settings: Vec<u8>,
    file_stamp: Option<Stamp>,
    rows: Rows,
    positions: Arc<HashMap<String, usize>>,
}

/// Allocated for one SQLite connection and dropped when that connection closes.
#[derive(Debug, Default)]
pub(crate) struct Store {
    slots: VecDeque<Slot>,
}

impl Store {
    fn hit(
        &mut self,
        kind: &'static str,
        question: &[u8],
        keyed: &[u8],
        settings: &[u8],
        file_stamp: Option<Stamp>,
    ) -> Option<HeldRows> {
        if kind != RANK && question.starts_with(b"@") && file_stamp.is_none() {
            return None;
        }
        let at = self
            .slots
            .iter()
            .position(|slot| {
                slot.kind == kind
                    && slot.file_stamp == file_stamp
                    && slot.pointers
                        == [
                            question.as_ptr() as usize,
                            keyed.as_ptr() as usize,
                            settings.as_ptr() as usize,
                        ]
                    && slot.question.len() == question.len()
                    && slot.keyed.len() == keyed.len()
                    && slot.settings.len() == settings.len()
                    && slot.question == question
                    && slot.keyed == keyed
                    && slot.settings == settings
            })
            .or_else(|| {
                self.slots.iter().position(|slot| {
                    slot.kind == kind
                        && slot.file_stamp == file_stamp
                        && slot.question.len() == question.len()
                        && slot.keyed.len() == keyed.len()
                        && slot.settings.len() == settings.len()
                        && slot.question == question
                        && slot.keyed == keyed
                        && slot.settings == settings
                })
            })?;
        let slot = self.slots.remove(at)?;
        let answer = (Arc::clone(&slot.rows), Arc::clone(&slot.positions));
        self.slots.push_front(slot);
        Some(answer)
    }

    fn insert(&mut self, slot: Slot) {
        self.slots.push_front(slot);
        if self.slots.len() > 8 {
            self.slots.pop_back();
        }
    }
}

fn required<'a>(values: &'a [ValueRef<'a>], at: usize, name: &str) -> Result<&'a [u8], Failure> {
    match values.get(at).copied().unwrap_or(ValueRef::Null) {
        ValueRef::Text(bytes) if !bytes.contains(&0) => Ok(bytes),
        ValueRef::Null => Err(Failure::usage(format!("{name} is required"))),
        _ => Err(Failure::usage(format!("{name} is UTF-8 text"))),
    }
}

fn string(bytes: &[u8], name: &str) -> Result<String, Failure> {
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| Failure::usage(format!("{name} is UTF-8 text")))
}

/// Read this one object without flattening repeated member names. Escaped
/// spellings are decoded before the duplicate comparison.
struct Keyed(Vec<(String, String)>);

fn keyed_map<'de, M: MapAccess<'de>>(mut map: M) -> Result<Keyed, M::Error> {
    let mut seen = HashSet::new();
    let mut rows = Vec::new();
    while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
        if !seen.insert(key.clone()) {
            return Err(M::Error::custom("a keyed record name is repeated"));
        }
        let text = value
            .as_str()
            .filter(|text| !text.trim().is_empty())
            .ok_or_else(|| M::Error::custom("each keyed record is nonblank text"))?;
        rows.push((key, text.to_owned()));
    }
    Ok(Keyed(rows))
}

impl<'de> Deserialize<'de> for Keyed {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Object;
        impl<'de> Visitor<'de> for Object {
            type Value = Keyed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("one JSON object of keyed nonblank text")
            }

            fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Keyed, M::Error> {
                keyed_map(map)
            }
        }
        deserializer.deserialize_map(Object)
    }
}

pub(crate) fn keyed(source: &str) -> Result<Vec<(String, String)>, Failure> {
    let mut deserializer = serde_json::Deserializer::from_str(source);
    let rows = Keyed::deserialize(&mut deserializer)
        .map_err(|error| Failure::usage(format!("keyed records are invalid: {error}")))?;
    deserializer
        .end()
        .map_err(|error| Failure::usage(format!("keyed records are invalid: {error}")))?;
    Ok(rows.0)
}

fn selected(rows: Rows, positions: Arc<HashMap<String, usize>>, lookup: Option<String>) -> Scan {
    Scan {
        rows,
        selected: lookup.map(|key| positions.get(&key).copied().unwrap_or(usize::MAX)),
    }
}

fn lookup(value: ValueRef<'_>) -> Option<String> {
    match value {
        ValueRef::Text(bytes) if !bytes.contains(&0) => {
            std::str::from_utf8(bytes).ok().map(str::to_owned)
        }
        _ => None,
    }
}

fn answer_columns(detail: &thinkthen::Details, kind: &str) -> Result<(Value, Value), Failure> {
    match detail.value() {
        Judgment::Decision(answer) if kind == "thinkthen_decide_many" => {
            let value = match answer {
                thinkthen::Answer::Yes => Value::Integer(1),
                thinkthen::Answer::No => Value::Integer(0),
                thinkthen::Answer::Unsure => Value::Null,
            };
            let Probabilities::YesNo { yes } = detail.probabilities() else {
                return Err(Failure::defect("a decide answer held no yes probability"));
            };
            Ok((value, Value::Real(*yes)))
        }
        Judgment::Choice(label) if kind == "thinkthen_choose_many" => {
            let probability = match (label, detail.probabilities()) {
                (Some(label), Probabilities::Named(named)) => named
                    .iter()
                    .find(|item| item.name() == label)
                    .map_or(Value::Null, |item| Value::Real(item.probability())),
                _ => Value::Null,
            };
            Ok((
                label
                    .as_ref()
                    .map_or(Value::Null, |label| Value::Text(label.clone())),
                probability,
            ))
        }
        Judgment::Score(value) if kind == "thinkthen_score_many" => {
            Ok((Value::Real(*value), Value::Null))
        }
        Judgment::Tags(labels) if kind == "thinkthen_tag_many" => Ok((
            Value::Text(
                serde_json::to_string(labels)
                    .map_err(|_| Failure::defect("tags could not be encoded"))?,
            ),
            Value::Null,
        )),
        _ => Err(Failure::defect("a packed answer held another kind")),
    }
}

fn scan(
    db: *mut sqlite3,
    mask: c_int,
    filters: &Filters<'_>,
    store: &std::sync::Mutex<Store>,
    kind: &'static str,
    verb: For,
) -> Result<Scan, Failure> {
    let mut given = 0;
    let mut values = [ValueRef::Null; 4];
    for (at, value) in values.iter_mut().enumerate() {
        if mask & (1 << at) != 0 {
            *value = filters
                .iter()
                .nth(given)
                .ok_or_else(|| Failure::defect("a bound keyed argument was absent"))?;
            given += 1;
        }
    }
    let question = required(&values, 0, "the question")?;
    let packed = required(&values, 1, "the keyed records")?;
    let settings = match values[2] {
        ValueRef::Null => b"{}".as_slice(),
        ValueRef::Text(bytes) => bytes,
        _ => return Err(Failure::usage("the settings are JSON text")),
    };
    // A numeric or NULL equality is evaluated by SQLite with TEXT affinity;
    // returning all candidates lets its residual predicate decide. The same
    // applies to malformed text, which SQLite can compare without this host
    // interpreting it as a UTF-8 key.
    let lookup = lookup(values[3]);
    // A rank question is literal text, as find's is, so it names no file.
    let file_stamp = if verb == For::Rank {
        None
    } else {
        stamp(question)
    };
    {
        let mut held = store
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some((rows, positions)) = held.hit(kind, question, packed, settings, file_stamp) {
            return Ok(selected(rows, positions, lookup));
        }
    }
    let question_text = string(question, "the question")?;
    let packed_text = string(packed, "the keyed records")?;
    string(settings, "the settings")?;
    let controls = call_settings(ValueRef::Text(settings))?;
    let rows = if verb == For::Rank {
        ranked(db, &question_text, &packed_text, controls)?
    } else {
        answered(db, &question_text, &packed_text, controls, kind, verb)?
    };
    let positions: HashMap<_, _> = rows
        .iter()
        .enumerate()
        .filter_map(|(at, row)| match row.first() {
            Some(Value::Text(key)) => Some((key.clone(), at)),
            _ => None,
        })
        .collect();
    let rows = Arc::new(rows);
    let positions = Arc::new(positions);
    store
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(Slot {
            kind,
            pointers: [
                question.as_ptr() as usize,
                packed.as_ptr() as usize,
                settings.as_ptr() as usize,
            ],
            question: question.to_vec(),
            keyed: packed.to_vec(),
            settings: settings.to_vec(),
            file_stamp,
            rows: Arc::clone(&rows),
            positions: Arc::clone(&positions),
        });
    Ok(selected(rows, positions, lookup))
}

/// One packed call's `(key, value[, probability])` rows in keyed order.
fn answered(
    db: *mut sqlite3,
    question: &str,
    packed: &str,
    controls: Settings,
    kind: &'static str,
    verb: For,
) -> Result<Vec<Vec<Value>>, Failure> {
    let held = question_with_settings(question, &controls, verb)?;
    let records = keyed(packed)?;
    let keys: Vec<_> = records.iter().map(|(key, _)| key.clone()).collect();
    let texts: Vec<_> = records.into_iter().map(|(_, text)| text).collect();
    let shared = controls.context().map(str::to_owned);
    let values = worker::run_settings(db, controls, move |engine, options| {
        let options = if let Some(shared) = &shared {
            options.context(shared)
        } else {
            options
        };
        engine
            .details_many_with(&*held, texts, options)
            .map(|row| answer_columns(&row?.into_parts().1, kind))
            .collect::<Result<Vec<_>, Failure>>()
    })?;
    if keys.len() != values.len() {
        return Err(Failure::defect("a packed call changed its row count"));
    }
    Ok(keys
        .into_iter()
        .zip(values)
        .map(|(key, (value, probability))| {
            if matches!(verb, For::Decide | For::Choose) {
                vec![Value::Text(key), value, probability]
            } else {
                vec![Value::Text(key), value]
            }
        })
        .collect())
}

/// One rank call's `(key, rank, probability)` rows, best first. The question
/// is literal text and takes only `model` among question fields.
fn ranked(
    db: *mut sqlite3,
    question: &str,
    packed: &str,
    controls: Settings,
) -> Result<Vec<Vec<Value>>, Failure> {
    controls
        .check(For::Rank)
        .map_err(|error| Failure::usage(error.to_string()))?;
    let asked = Question::rank(question)?;
    let asked = match controls.model() {
        Some(model) => asked.with_model(model)?,
        None => asked,
    };
    let records = keyed(packed)?;
    if records.is_empty() {
        return Ok(Vec::new());
    }
    let (keys, texts): (Vec<_>, Vec<_>) = records.into_iter().unzip();
    let shared = controls.context().map(str::to_owned);
    let ranked = worker::run_settings(db, controls, move |engine, options| {
        let options = if let Some(shared) = &shared {
            options.context(shared)
        } else {
            options
        };
        Ok(engine.rank_with(&asked, texts, options)?.into_value())
    })?;
    ranked
        .iter()
        .zip(1_i64..)
        .map(|(row, place)| {
            let key = keys
                .get(row.index())
                .ok_or_else(|| Failure::defect("a ranked row lost its record"))?;
            Ok(vec![
                Value::Text(key.clone()),
                Value::Integer(place),
                Value::Real(row.probability()),
            ])
        })
        .collect()
}

macro_rules! many {
    ($name:ident, $sql:literal, $schema:literal, $verb:expr) => {
        #[derive(Debug)]
        pub(crate) struct $name;
        impl Table for $name {
            const NAME: &'static str = $sql;
            const SCHEMA: &'static CStr = $schema;
            const FIRST_HIDDEN: usize = if matches!($verb, For::Decide | For::Choose | For::Rank) {
                3
            } else {
                2
            };
            const COLUMNS: usize = Self::FIRST_HIDDEN + 4;
            const REQUIRED: usize = 2;
            fn rows(_: *mut sqlite3, _: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
                Err(Failure::defect("keyed rows lost their borrowed arguments"))
            }
            fn scan(
                db: *mut sqlite3,
                mask: c_int,
                filters: &Filters<'_>,
                store: &std::sync::Mutex<Store>,
            ) -> Result<Scan, Failure> {
                scan(db, mask, filters, store, $sql, $verb)
            }
        }
    };
}

many!(DecideMany, "thinkthen_decide_many", c"CREATE TABLE x(key TEXT, value INTEGER, probability REAL, question HIDDEN, keyed_json HIDDEN, settings HIDDEN, lookup_key HIDDEN)", For::Decide);
many!(ChooseMany, "thinkthen_choose_many", c"CREATE TABLE x(key TEXT, value TEXT, probability REAL, question HIDDEN, keyed_json HIDDEN, settings HIDDEN, lookup_key HIDDEN)", For::Choose);
many!(ScoreMany, "thinkthen_score_many", c"CREATE TABLE x(key TEXT, value REAL, question HIDDEN, keyed_json HIDDEN, settings HIDDEN, lookup_key HIDDEN)", For::Score);
many!(TagMany, "thinkthen_tag_many", c"CREATE TABLE x(key TEXT, value TEXT, question HIDDEN, keyed_json HIDDEN, settings HIDDEN, lookup_key HIDDEN)", For::Tag);
many!(Rank, "thinkthen_rank", c"CREATE TABLE x(key TEXT, rank INTEGER, probability REAL, question HIDDEN, keyed_json HIDDEN, settings HIDDEN, lookup_key HIDDEN)", For::Rank);
