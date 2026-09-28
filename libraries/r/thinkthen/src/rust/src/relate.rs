//! `recognize` and `relate`: named things in texts, and edges among them.

use std::collections::HashSet;
use std::sync::Arc;

use extendr_api::prelude::*;
use std::result::Result;
use thinkthen::{Entity, Recognize, Recognized, Relate};

use crate::calls::receipt::Receipt;
use crate::calls::render;
use crate::calls::{Crossed, Pending, call_owned};
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
    positions: Vec<usize>,
    deadline: Option<f64>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
) -> Crossed<List> {
    let ask = recognize_ask(spec)?;
    if texts.len() != positions.len() {
        return Err(crate::usage("recognize positions do not match texts"));
    }
    let completed = call_owned(
        deadline,
        pending,
        receipt,
        None,
        move |engine, options, account| {
            if texts.is_empty() {
                account.no_work();
            }
            let mut found = Vec::with_capacity(texts.len());
            for (text, original) in texts.iter().zip(positions) {
                let observer = |event: thinkthen::RecordObservation<'_>| {
                    account.observe(event, Some(original))
                };
                let answer = engine.recognize_with(&ask, text, options.observe(&observer));
                match answer {
                    Ok(answer) => {
                        account.add(answer.facts())?;
                        found.push(answer.into_value());
                    }
                    Err(error) => {
                        return Err(account.failed(&error)?);
                    }
                }
            }
            Ok(found)
        },
    )?;
    Ok(render::envelope(completed, |found| {
        List::from_values(found.iter().map(names)).into()
    }))
}

/// `relate` over entities given as names and kinds. A repeated pair is
/// sent once, in first-seen order (ADR 0047 item 9).
pub(crate) fn relate(
    spec: &Spec,
    pairs: Vec<(String, String)>,
    deadline: Option<f64>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
) -> Crossed<List> {
    let ask = relate_ask(spec)?;
    let mut seen = HashSet::new();
    let entities = pairs
        .into_iter()
        .filter(|pair| seen.insert(pair.clone()))
        .map(|(name, kind)| Entity::new(&name, &kind))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| carry(&error))?;
    let completed = call_owned(
        deadline,
        pending,
        receipt,
        None,
        move |engine, options, account| {
            let edges = match engine.relate_with(&ask, entities, options) {
                Ok(value) => value,
                Err(error) => {
                    return Err(account.failed(&error)?);
                }
            };
            account.add(edges.facts())?;
            Ok(edges.into_value())
        },
    )?;
    Ok(render::envelope(completed, |edges| {
        list!(
            source = each(&edges, |edge| edge.source().name()),
            target = each(&edges, |edge| edge.target().name()),
            relation = each(&edges, |edge| edge.relation()),
            probability = each(&edges, |edge| edge.probability()),
            source_kind = each(&edges, |edge| edge.source().kind()),
            target_kind = each(&edges, |edge| edge.target().kind())
        )
        .into()
    }))
}
