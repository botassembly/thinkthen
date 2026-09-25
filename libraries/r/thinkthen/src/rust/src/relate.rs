//! `recognize` and `relate`: named things in texts, and edges among them.

use std::collections::HashSet;

use extendr_api::prelude::*;
use std::result::Result;
use thinkthen::{Entity, Recognize, Recognized, Relate};

use crate::calls::{Crossed, Pending, call};
use crate::carry;

/// Where a recognize or relate spec comes from: the file's JSON, or a path.
#[derive(Debug)]
pub(crate) enum Spec {
    Json(String),
    Path(String),
}

fn recognize_ask(spec: &Spec) -> Crossed<Recognize> {
    match spec {
        Spec::Json(json) => Recognize::from_json(json),
        Spec::Path(path) => Recognize::load(path),
    }
    .map_err(|error| carry(&error))
}

fn relate_ask(spec: &Spec) -> Crossed<Relate> {
    match spec {
        Spec::Json(json) => Relate::from_json(json),
        Spec::Path(path) => Relate::load(path),
    }
    .map_err(|error| carry(&error))
}

/// One record's names as R's own: `substr(text, start, end)` is the name,
/// because offsets count Unicode scalar values and `start` shifts to one-based.
fn names(found: &Recognized) -> List {
    let entities = found.entities();
    let name: Vec<&str> = entities.iter().map(|held| held.name()).collect();
    let kind: Vec<&str> = entities.iter().map(|held| held.kind()).collect();
    let start: Vec<f64> = entities
        .iter()
        .map(|held| place(held.start()) + 1.0)
        .collect();
    let end: Vec<f64> = entities.iter().map(|held| place(held.end())).collect();
    let strength: Vec<f64> = entities.iter().map(|held| held.strength()).collect();
    let relations = found.relations().unwrap_or_default();
    let links = list!(
        source = relations
            .iter()
            .map(|held| held.source().name())
            .collect::<Vec<_>>(),
        source_kind = relations
            .iter()
            .map(|held| held.source().kind())
            .collect::<Vec<_>>(),
        target = relations
            .iter()
            .map(|held| held.target().name())
            .collect::<Vec<_>>(),
        target_kind = relations
            .iter()
            .map(|held| held.target().kind())
            .collect::<Vec<_>>(),
        relation = relations
            .iter()
            .map(|held| held.relation())
            .collect::<Vec<_>>(),
        probability = relations
            .iter()
            .map(|held| held.probability())
            .collect::<Vec<_>>()
    );
    list!(
        name = name,
        kind = kind,
        start = start,
        end = end,
        strength = strength,
        relations = links
    )
}

/// An offset as R's number. Offsets fit a double exactly.
fn place(offset: usize) -> f64 {
    u32::try_from(offset).map_or(f64::MAX, f64::from)
}

/// `recognize` over a column: one list of names a text, in input order.
pub(crate) fn recognize(
    spec: &Spec,
    texts: Vec<String>,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<List> {
    let ask = recognize_ask(spec)?;
    let found = call(deadline, pending, move |engine, options| {
        texts
            .iter()
            .map(|text| engine.recognize_with(&ask, text, options))
            .collect::<Result<Vec<_>, _>>()
    })?;
    Ok(List::from_values(found.iter().map(names)))
}

/// `relate` over entities given as names and kinds. A repeated pair is
/// sent once, in first-seen order (ADR 0047 item 9).
pub(crate) fn relate(
    spec: &Spec,
    pairs: Vec<(String, String)>,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<List> {
    let ask = relate_ask(spec)?;
    let mut seen = HashSet::new();
    let entities = pairs
        .into_iter()
        .filter(|pair| seen.insert(pair.clone()))
        .map(|(name, kind)| Entity::new(&name, &kind))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| carry(&error))?;
    let edges = call(deadline, pending, move |engine, options| {
        engine.relate_with(&ask, entities, options)
    })?;
    Ok(list!(
        source = edges
            .iter()
            .map(|edge| edge.source().name())
            .collect::<Vec<_>>(),
        target = edges
            .iter()
            .map(|edge| edge.target().name())
            .collect::<Vec<_>>(),
        relation = edges.iter().map(|edge| edge.relation()).collect::<Vec<_>>(),
        probability = edges
            .iter()
            .map(|edge| edge.probability())
            .collect::<Vec<_>>(),
        source_kind = edges
            .iter()
            .map(|edge| edge.source().kind())
            .collect::<Vec<_>>(),
        target_kind = edges
            .iter()
            .map(|edge| edge.target().kind())
            .collect::<Vec<_>>()
    ))
}
