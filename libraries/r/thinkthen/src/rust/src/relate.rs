//! `recognize` and `relate`: named things in texts, and edges among them.

use std::collections::HashSet;
use std::sync::Arc;

use extendr_api::prelude::*;
use std::result::Result;
use thinkthen::{Entity, Recognize, Relate};

use crate::calls::receipt::Receipt;
use crate::calls::render;
use crate::calls::{Crossed, Pending, call_owned, raw};
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
                let answer = account
                    .run(|| engine.recognize_with(&ask, text, options.observe(&observer)))?;
                found.push(raw(answer.to_json())?);
            }
            Ok(found)
        },
    )?;
    render::envelope(completed)
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
            let edges = account.run(|| engine.relate_with(&ask, entities, options))?;
            edges
                .iter()
                .map(|edge| raw(edge.to_json()))
                .collect::<Crossed<Vec<_>>>()
        },
    )?;
    render::envelope(completed)
}
