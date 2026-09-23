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
//! the build team; nothing here touches them. Main ruled method H for every
//! relation on 2026-09-23. Every relate row holds a method-H recording, one
//! yes/no per pair per relation, from the relate-methods bake-off or from
//! `conformance/relate-h/`. The pick-one rows are retired
//! (`sdlc/records/surfaces-notes/NOTES-main-parity.md`).

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

/// A method-H relate row. Every row has form `yes-no`.
#[derive(Deserialize)]
struct RelateRow {
    text: String,
    records: Vec<String>,
    rules: Vec<String>,
    /// Whether the recording asked each rule both ways.
    either: BTreeMap<String, bool>,
    entries: Vec<EntryRow>,
}

/// One recorded yes/no question: the ordered pair it asked, the rule, and
/// the probability of yes.
#[derive(Deserialize)]
struct EntryRow {
    pair: [u64; 2],
    rule: String,
    options: BTreeMap<String, f64>,
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
    let row = table()
        .relate
        .iter()
        .find(|row| {
            row.text.trim_end_matches('\n') == trimmed
                || row.records.join("\n").trim_end_matches('\n') == trimmed
        })
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
        if row.either.get(&rule.name) != Some(&rule.either) {
            let way = if rule.either { "one way" } else { "both ways" };
            return Err(Error::usage(format!(
                "no recorded answer for the rule {} as asked; the recording asks {} {way}",
                rule.name, rule.name
            )));
        }
    }
    yes_no_edges(ask, row)
}

/// The edges of a method-H row: one recorded yes/no per pair per relation,
/// each direction of a one-way relation its own question (main's ruling of
/// 2026-09-23). Every question whose yes reaches the bar is an edge, so one
/// pair may carry several relations.
fn yes_no_edges(ask: &Relate, row: &RelateRow) -> Result<Vec<Edge>, Error> {
    let mut edges = Vec::new();
    for entry in &row.entries {
        let Some(rule) = ask.relations.iter().find(|rule| rule.name == entry.rule) else {
            continue;
        };
        let probability = entry.options.get("yes").copied().ok_or_else(|| {
            Error::defect(format!("the replay table holds no yes probability for {}", entry.rule))
        })?;
        if probability >= ask.threshold {
            edges.push(edge(rule, entry.pair[0], entry.pair[1], probability));
        }
    }
    Ok(sorted(edges))
}

/// One edge, with the rule's named ends as its kinds.
fn edge(
    rule: &thinkthen_contract::RelationRule,
    source: u64,
    target: u64,
    probability: f64,
) -> Edge {
    let (source_kind, target_kind) = match (&rule.from, &rule.to) {
        (thinkthen_contract::Kind::Named(from), thinkthen_contract::Kind::Named(to)) => {
            (Some(from.clone()), Some(to.clone()))
        }
        _ => (None, None),
    };
    Edge { name: rule.name.clone(), source, target, probability, source_kind, target_kind }
}

/// Edges in source, target, and name order.
fn sorted(mut edges: Vec<Edge>) -> Vec<Edge> {
    edges.sort_by(|one, two| {
        (one.source, one.target, &one.name).cmp(&(two.source, two.target, &two.name))
    });
    edges
}
