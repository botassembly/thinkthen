//! The recognize and relate replay: the stand-in answers both functions
//! from the recordings, never by inventing one.
//!
//! The table is `standin/data/recognize-replay.json`, generated from
//! `experiments/225-recognize-harvest-package` by
//! `conformance/tools/build_recognize_cases.py`. One row per recorded case:
//! the text, the recorded entities, the recorded relation pairs with their
//! probability maps, the recorded rule names, and the recorded request
//! digests. The C41 row is synthesized from the package rules for the
//! per-host offset proofs and is marked so in the table.
//!
//! Rules of engagement, from the brief:
//!
//! - A text the recordings do not hold is a usage error naming the text.
//! - A rule the recordings do not hold for that text is a usage error
//!   naming the rule and the rules the recording covers.
//! - A rule's named end must be one of the asked kinds; the contract checks
//!   that shape before anything else.
//! - Nothing is invented: every entity, relation, and edge returned is a
//!   recorded answer with the ruled field names (`strength` on names,
//!   `probability` on relations and edges, `source` and `target` for the
//!   ends; the recordings carry the vendor's own field names, mapped here).
//! - The recordings carry no kinds for relate's records, so a kind field or
//!   a named relation end is refused for relate with a usage error that
//!   names the limit.
//!
//! Text cutting, the question count per request, and the method belong to
//! the build team; nothing here touches them.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::Deserialize;
use thinkthen_contract::{Edge, Entity, Error, Recognize, Recognized, Relate, Relation};

/// The generated replay table, embedded at build time so every surface
/// gets it with no path to find.
const TABLE: &str = include_str!("../data/recognize-replay.json");

#[derive(Deserialize)]
struct Table {
    recognize: Vec<RecognizeRow>,
    relate: Vec<RelateRow>,
}

#[derive(Deserialize)]
struct RecognizeRow {
    text: String,
    entities: Vec<RawEntity>,
    rules: Vec<String>,
    pairs: Vec<PairRow>,
}

#[derive(Deserialize)]
struct RawEntity {
    id: u64,
    text: String,
    kind: String,
    start: usize,
    end: usize,
    /// The recordings carry this number under Jev's own field name; the
    /// ruled host-facing field is `strength` (settled 2026-09-21), and the
    /// mapping happens at replay.
    confidence: f64,
}

#[derive(Deserialize)]
struct PairRow {
    pair: [u64; 2],
    options: BTreeMap<String, f64>,
    pick: String,
}

#[derive(Deserialize)]
struct RelateRow {
    /// `pairs` is the ruled form; `per-subject` is a measured arm whose
    /// text can collide with a pairs arm's, and the ruled form wins.
    form: Option<String>,
    text: String,
    records: Vec<String>,
    rules: Vec<String>,
    entries: Vec<EntryRow>,
}

#[derive(Deserialize)]
struct EntryRow {
    pair: Option<[u64; 2]>,
    subject: Option<u64>,
    /// The rule a per-subject entry asks, because its options are object
    /// records and not relation names.
    rule: Option<String>,
    options: BTreeMap<String, f64>,
    pick: String,
}

fn table() -> &'static Table {
    static TABLE_ONCE: OnceLock<Table> = OnceLock::new();
    TABLE_ONCE.get_or_init(|| {
        serde_json::from_str(TABLE).expect("the replay table is committed with the crate")
    })
}

/// The rule name behind a pick: the option minus its `_AB`/`_BA` suffix.
fn rule_name(pick: &str) -> &str {
    pick.strip_suffix("_AB").or_else(|| pick.strip_suffix("_BA")).unwrap_or(pick)
}

/// A pick that means "no relation".
fn is_no_relation(pick: &str) -> bool {
    pick == "NO_RELATION" || pick == "NONE_OF_THESE"
}

/// The first words of a text, for an error that names what is missing.
fn short(text: &str) -> String {
    let mut head: String = text.chars().take(48).collect();
    if text.chars().count() > 48 {
        head.push('…');
    }
    head
}

fn covered(rules: &[String]) -> String {
    if rules.is_empty() {
        "no rules".to_owned()
    } else {
        rules.join(", ")
    }
}

/// Find every name in one text, from the recording.
pub(crate) fn recognize(ask: &Recognize, text: &str) -> Result<Recognized, Error> {
    for rule in &ask.relations {
        rule.check_kinds(&ask.kinds)?;
    }
    let row = table()
        .recognize
        .iter()
        .find(|row| row.text == text)
        .ok_or_else(|| {
            Error::usage(format!(
                "no recorded answer for the text \"{}\"; the stand-in answers only from the recordings",
                short(text)
            ))
        })?;
    for rule in &ask.relations {
        if !row.rules.iter().any(|name| name == &rule.name) {
            return Err(Error::usage(format!(
                "no recorded answer for the rule {} on this text; the recording covers {}",
                rule.name,
                covered(&row.rules)
            )));
        }
    }
    let entities: Vec<Entity> = row
        .entities
        .iter()
        .filter(|entity| ask.kinds.iter().any(|kind| kind == &entity.kind))
        .filter(|entity| entity.confidence >= ask.threshold)
        .map(|entity| Entity {
            id: entity.id,
            text: entity.text.clone(),
            kind: entity.kind.clone(),
            start: entity.start,
            end: entity.end,
            strength: entity.confidence,
        })
        .collect();
    let mut relations = Vec::new();
    for entry in &row.pairs {
        if is_no_relation(&entry.pick) {
            continue;
        }
        let name = rule_name(&entry.pick);
        let Some(rule) = ask.relations.iter().find(|rule| rule.name == name) else {
            continue;
        };
        let probability = entry.options.get(&entry.pick).copied().ok_or_else(|| {
            Error::defect(format!("the replay table holds no probability for {}", entry.pick))
        })?;
        if probability < ask.relation_threshold {
            continue;
        }
        if !entities.iter().any(|entity| entity.id == entry.pair[0])
            || !entities.iter().any(|entity| entity.id == entry.pair[1])
        {
            continue;
        }
        let (mut source, mut target) = if entry.pick.ends_with("_AB") {
            (entry.pair[0], entry.pair[1])
        } else if entry.pick.ends_with("_BA") {
            (entry.pair[1], entry.pair[0])
        } else {
            (entry.pair[0].min(entry.pair[1]), entry.pair[0].max(entry.pair[1]))
        };
        if rule.either {
            (source, target) = (source.min(target), source.max(target));
        }
        relations.push(Relation {
            name: name.to_owned(),
            source,
            target,
            probability,
        });
    }
    relations.sort_by(|one, two| {
        (one.source, one.target, &one.name).cmp(&(two.source, two.target, &two.name))
    });
    Ok(Recognized { entities, relations })
}

/// Say how every record relates to the others, from the recording.
pub(crate) fn relate(ask: &Relate, records: &[&str]) -> Result<Vec<Edge>, Error> {
    if ask.kind_field.is_some() {
        return Err(Error::usage(
            "the recordings carry no kinds for relate's records, so a kind field cannot be honoured",
        ));
    }
    for rule in &ask.relations {
        if !matches!(rule.from, thinkthen_contract::Kind::Any) || !matches!(rule.to, thinkthen_contract::Kind::Any)
        {
            return Err(Error::usage(format!(
                "the rule {} names a kind, and the recordings carry no kinds for relate's records",
                rule.name
            )));
        }
    }
    if ask.relations.is_empty() {
        return Err(Error::usage("relate needs at least one relation rule"));
    }
    let joined = records.join("\n");
    let trimmed = joined.trim_end_matches('\n');
    let candidates: Vec<&RelateRow> = table()
        .relate
        .iter()
        .filter(|row| {
            row.text.trim_end_matches('\n') == trimmed
                || row.records.join("\n").trim_end_matches('\n') == trimmed
        })
        .collect();
    let row = candidates
        .iter()
        .find(|row| row.form.as_deref() != Some("per-subject"))
        .or_else(|| candidates.first())
        .copied()
        .ok_or_else(|| {
            Error::usage(format!(
                "no recorded answer for the records \"{}\"; the stand-in answers only from the recordings",
                short(trimmed)
            ))
        })?;
    for rule in &ask.relations {
        if !row.rules.iter().any(|name| name == &rule.name) {
            return Err(Error::usage(format!(
                "no recorded answer for the rule {} on these records; the recording covers {}",
                rule.name,
                covered(&row.rules)
            )));
        }
    }
    let mut edges = Vec::new();
    for entry in &row.entries {
        if is_no_relation(&entry.pick) {
            continue;
        }
        let name: &str = if entry.pair.is_none() {
            entry.rule.as_deref().ok_or_else(|| {
                Error::defect("a recorded subject entry names no rule")
            })?
        } else {
            rule_name(&entry.pick)
        };
        let Some(rule) = ask.relations.iter().find(|rule| rule.name == name) else {
            continue;
        };
        let probability = entry.options.get(&entry.pick).copied().ok_or_else(|| {
            Error::defect(format!("the replay table holds no probability for {}", entry.pick))
        })?;
        if probability < ask.threshold {
            continue;
        }
        let (source, target) = if let Some(pair) = entry.pair {
            if entry.pick.ends_with("_AB") {
                (pair[0], pair[1])
            } else if entry.pick.ends_with("_BA") {
                (pair[1], pair[0])
            } else {
                (pair[0].min(pair[1]), pair[0].max(pair[1]))
            }
        } else {
            let subject = entry
                .subject
                .ok_or_else(|| Error::defect("a recorded relate entry has no pair and no subject"))?;
            let target = entry
                .pick
                .strip_prefix("r_")
                .and_then(|number| number.parse::<u64>().ok())
                .ok_or_else(|| {
                    Error::defect(format!("the recorded pick {} names no record", entry.pick))
                })?;
            (subject, target)
        };
        let (source_kind, target_kind) = match (&rule.from, &rule.to) {
            (thinkthen_contract::Kind::Named(from), thinkthen_contract::Kind::Named(to)) => {
                (Some(from.clone()), Some(to.clone()))
            }
            _ => (None, None),
        };
        edges.push(Edge {
            name: name.to_owned(),
            source,
            target,
            probability,
            source_kind,
            target_kind,
        });
    }
    edges.sort_by(|one, two| {
        (one.source, one.target, &one.name).cmp(&(two.source, two.target, &two.name))
    });
    Ok(edges)
}
