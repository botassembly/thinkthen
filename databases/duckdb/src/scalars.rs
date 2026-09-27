//! The scalar verbs (ticket 0110 decisions 6, 9, 11, and 12).
//!
//! Each invoke reads its chunk, drops NULL rows, groups the rest by their
//! question and deadline, dedupes texts in first-seen order, and makes one
//! engine call per group on a worker. Answers map back by text, so a
//! repeated text or a NULL never shifts a row (R1-1).

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use thinkthen::{CancelToken, Engine, Error};

use crate::engines::{self, Asked};
use crate::ffi::{Column, Type, Value};
use crate::questions::{Caller, Members, members};
use crate::signal::Invoke;
use crate::worker;

mod calls;
mod ffi;

pub(crate) use calls::decided;
use calls::{annotated, detailed, entities, member, recognize_kinds, recognized, relations};
pub(crate) use ffi::register_scalar;

/// Which verb one registered scalar runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Verb {
    Decide,
    Probability,
    Choose,
    Score,
    Tag,
    Annotate,
    Details,
    Recognize,
    Relations,
}

/// One scalar's SQL name, arguments, and result. A scalar with a deadline
/// also registers the overload with a trailing `BIGINT` of milliseconds.
#[derive(Debug)]
pub(crate) struct Scalar {
    pub(crate) name: &'static str,
    pub(crate) verb: Verb,
    pub(crate) arguments: &'static [Type],
    pub(crate) result: Type,
    pub(crate) deadline: bool,
}

const TEXTS: Type = Type::List(&Type::Text);
const TWO: &[Type] = &[Type::Text, Type::Text];
const LISTED: &[Type] = &[Type::Text, Type::Text, TEXTS];

const RECOGNIZED: Type = Type::List(&Type::Struct(&[
    ("text", Type::Text),
    ("start", Type::BigInt),
    ("end", Type::BigInt),
    ("length", Type::BigInt),
    ("kind", Type::Text),
    ("strength", Type::Double),
]));

const RELATIONS: Type = Type::List(&Type::Struct(&[
    ("relation", Type::Text),
    ("source", Type::Text),
    ("source_kind", Type::Text),
    ("target", Type::Text),
    ("target_kind", Type::Text),
    ("probability", Type::Double),
]));

/// Every scalar the extension registers.
pub(crate) static SCALARS: [Scalar; 9] = [
    scalar("thinkthen_decide", Verb::Decide, TWO, Type::Bool, true),
    scalar(
        "thinkthen_probability",
        Verb::Probability,
        TWO,
        Type::Double,
        true,
    ),
    scalar("thinkthen_choose", Verb::Choose, LISTED, Type::Text, true),
    scalar("thinkthen_score", Verb::Score, LISTED, Type::Double, true),
    scalar("thinkthen_tag", Verb::Tag, LISTED, TEXTS, true),
    scalar("thinkthen_annotate", Verb::Annotate, TWO, Type::Text, true),
    scalar("thinkthen_details", Verb::Details, TWO, Type::Text, true),
    scalar(
        "thinkthen_recognize",
        Verb::Recognize,
        &[Type::Text, TEXTS],
        RECOGNIZED,
        false,
    ),
    scalar("thinkthen_relations", Verb::Relations, TWO, RELATIONS, true),
];

const fn scalar(
    name: &'static str,
    verb: Verb,
    arguments: &'static [Type],
    result: Type,
    deadline: bool,
) -> Scalar {
    Scalar {
        name,
        verb,
        arguments,
        result,
        deadline,
    }
}

/// The rows of one chunk, grouped by a key, with each group's distinct texts.
struct Groups<K> {
    keys: Vec<K>,
    texts: Vec<Vec<String>>,
    /// Each row's group and text, or `None` for a NULL row.
    slots: Vec<Option<(usize, usize)>>,
}

impl<K: Clone + Eq + Hash> Groups<K> {
    fn of(columns: &[Column], rows: usize, key: impl Fn(usize) -> Option<(K, String)>) -> Self {
        let mut groups = Self {
            keys: Vec::new(),
            texts: Vec::new(),
            slots: Vec::with_capacity(rows),
        };
        let mut seen: HashMap<(usize, String), usize> = HashMap::new();
        let mut known: HashMap<K, usize> = HashMap::new();
        for row in 0..rows {
            let found = columns
                .iter()
                .all(|column| column.present(row))
                .then(|| key(row))
                .flatten();
            let Some((key, text)) = found else {
                groups.slots.push(None);
                continue;
            };
            let group = *known.entry(key.clone()).or_insert_with(|| {
                groups.keys.push(key);
                groups.texts.push(Vec::new());
                groups.keys.len() - 1
            });
            let place = if let Some(place) = seen.get(&(group, text.clone())) {
                *place
            } else if let Some(texts) = groups.texts.get_mut(group) {
                texts.push(text.clone());
                seen.insert((group, text), texts.len() - 1);
                texts.len() - 1
            } else {
                groups.slots.push(None);
                continue;
            };
            groups.slots.push(Some((group, place)));
        }
        groups
    }

    /// Answer every group, then read each row's answer through its slot.
    fn answer(
        self,
        mut each: impl FnMut(&K, Vec<String>) -> Result<Vec<Value>, String>,
    ) -> Result<Vec<Value>, String> {
        let mut answers = Vec::with_capacity(self.keys.len());
        for (key, texts) in self.keys.iter().zip(self.texts) {
            let wanted = texts.len();
            let got = each(key, texts)?;
            if got.len() != wanted {
                return Err(crate::errors::defect(
                    "the engine answered a different number of texts",
                ));
            }
            answers.push(got);
        }
        Ok(self
            .slots
            .into_iter()
            .map(|slot| {
                slot.and_then(|(group, place)| {
                    answers.get(group).and_then(|got| got.get(place)).cloned()
                })
                .unwrap_or(Value::Null)
            })
            .collect())
    }
}

/// Run one verb over one chunk.
pub(crate) fn run(
    verb: Verb,
    caller: &mut Caller,
    invoke: &Invoke,
    columns: &[Column],
    rows: usize,
) -> Result<Vec<Value>, String> {
    let text = |column: usize, row: usize| columns.get(column).and_then(|found| found.text(row));
    let list = |column: usize, row: usize| columns.get(column).and_then(|found| found.list(row));
    let listed = matches!(verb, Verb::Choose | Verb::Score | Verb::Tag);
    let deadline = |row: usize| {
        columns
            .get(if listed { 3 } else { 2 })
            .and_then(|found| found.int(row))
    };
    let (engine, asked) = (Arc::clone(&caller.engine), caller.asked.clone());
    match verb {
        Verb::Decide | Verb::Probability | Verb::Details | Verb::Annotate | Verb::Relations => {
            // Relations takes its body first and its recognize file second.
            let (asked, body) = if verb == Verb::Relations {
                (1, 0)
            } else {
                (0, 1)
            };
            let groups = Groups::of(columns, rows, |row| {
                Some((
                    (text(asked, row)?.to_owned(), deadline(row)),
                    text(body, row)?.to_owned(),
                ))
            });
            groups.answer(|(argument, due), texts| {
                asked_once(verb, caller, invoke, argument, texts, *due)
            })
        }
        Verb::Choose | Verb::Score | Verb::Tag => {
            let kind = match verb {
                Verb::Choose => Members::Choose,
                Verb::Score => Members::Score,
                _ => Members::Tag,
            };
            let groups = Groups::of(columns, rows, |row| {
                Some((
                    (
                        text(0, row)?.to_owned(),
                        list(2, row)?.to_vec(),
                        deadline(row),
                    ),
                    text(1, row)?.to_owned(),
                ))
            });
            groups.answer(|(argument, listed, due), texts| {
                let (set, due) = (members(kind, argument, listed)?, *due);
                let records = on_worker(
                    invoke,
                    &engine,
                    &asked,
                    texts,
                    move |engine, texts, token| annotated(engine, &set, texts, token, due),
                )?;
                records.iter().map(member).collect()
            })
        }
        Verb::Recognize => {
            let groups = Groups::of(columns, rows, |row| {
                Some((list(1, row)?.to_vec(), text(0, row)?.to_owned()))
            });
            groups.answer(|kinds, texts| {
                let ask = recognize_kinds(kinds)?;
                on_worker(
                    invoke,
                    &engine,
                    &asked,
                    texts,
                    move |engine, texts, token| {
                        recognized(engine, &ask, texts, token, None, entities)
                    },
                )
            })
        }
    }
}

/// One group of a verb whose question is a single argument: decide,
/// probability, details, annotate, and relations.
fn asked_once(
    verb: Verb,
    caller: &mut Caller,
    invoke: &Invoke,
    argument: &str,
    texts: Vec<String>,
    due: Option<i64>,
) -> Result<Vec<Value>, String> {
    let (engine, asked) = (&Arc::clone(&caller.engine), &caller.asked.clone());
    match verb {
        Verb::Annotate => {
            let set = caller.set(argument)?;
            let records = on_worker(invoke, engine, asked, texts, move |engine, texts, token| {
                annotated(engine, &set, texts, token, due)
            })?;
            Ok(records
                .iter()
                .map(|record| Value::Text(record.value_json()))
                .collect())
        }
        Verb::Relations => {
            let ask = caller.recognize(argument)?;
            on_worker(invoke, engine, asked, texts, move |engine, texts, token| {
                recognized(engine, &ask, texts, token, due, relations)
            })
        }
        Verb::Details => {
            let question = caller.question(argument)?;
            on_worker(invoke, engine, asked, texts, move |engine, texts, token| {
                detailed(engine, &question, &texts, token, due)
            })
        }
        _ => {
            let question = caller.question(argument)?;
            let probability = verb == Verb::Probability;
            on_worker(invoke, engine, asked, texts, move |engine, texts, token| {
                decided(engine, &question, texts, token, due, probability)
            })
        }
    }
}

/// One engine call over `texts` on a detachable worker, within what the
/// process's request total leaves.
fn on_worker<T: Send + 'static>(
    invoke: &Invoke,
    engine: &Arc<Engine>,
    asked: &Asked,
    texts: Vec<String>,
    work: impl FnOnce(&Engine, Vec<String>, &CancelToken) -> Result<T, Error> + Send + 'static,
) -> Result<T, String> {
    let (texts, cut) = engines::within_total(asked, texts)?;
    let engine = Arc::clone(engine);
    let answered = worker::run(invoke, move |token| work(&engine, texts, token))?;
    cut.map_or(Ok(answered), Err)
}
