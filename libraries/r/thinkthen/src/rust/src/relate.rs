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
/// `length` is the name's count of scalar values, `end - start + 1` in R.
fn names(found: &Recognized) -> List {
    let entities = found.entities();
    let relations = found.relations().unwrap_or_default();
    let links = list!(
        source = each(relations, |held| held.source().text()),
        source_kind = each(relations, |held| held.source().kind()),
        target = each(relations, |held| held.target().text()),
        target_kind = each(relations, |held| held.target().kind()),
        relation = each(relations, |held| held.relation()),
        probability = each(relations, |held| held.probability())
    );
    list!(
        text = each(entities, |held| held.text()),
        start = each(entities, |held| place(held.start()) + 1.0),
        end = each(entities, |held| place(held.end())),
        length = each(entities, |held| place(held.length())),
        kind = each(entities, |held| held.kind()),
        strength = each(entities, |held| held.strength()),
        relations = links
    )
}

/// One column of R values from a list of items.
fn each<'a, T, U>(items: &'a [T], value: impl Fn(&'a T) -> U) -> Vec<U> {
    items.iter().map(value).collect()
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
            .map(|text| engine.recognize_with(&ask, text, options).map(thinkthen::Call::into_value))
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
    })?
    .into_value();
    Ok(list!(
        source = each(&edges, |edge| edge.source().name()),
        target = each(&edges, |edge| edge.target().name()),
        relation = each(&edges, |edge| edge.relation()),
        probability = each(&edges, |edge| edge.probability()),
        source_kind = each(&edges, |edge| edge.source().kind()),
        target_kind = each(&edges, |edge| edge.target().kind())
    ))
}
