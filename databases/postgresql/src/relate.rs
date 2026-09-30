//! `thinkthen_relate(query, rules)`: relations among the rows a query
//! returns (decision 10, ADR 0047 item 9).

use std::collections::HashMap;

use pgrx::Spi;
use pgrx::datum::Array;
use pgrx::prelude::*;
use thinkthen::{Entity, Relate, RelationRule};

use crate::call::{self, OrRaise as _, Refusal};

/// SPI reads at most this many rows, and the last one refuses.
const ROW_LIMIT: i64 = 256;

/// One row: its id, name, and kind.
type Row = (i64, String, String);

/// The kind a two-column query gives every row.
const ANY: &str = "*";

type Edges = TableIterator<
    'static,
    (
        name!(relation, String),
        name!(source, i64),
        name!(target, i64),
        name!(probability, f64),
        name!(either, bool),
    ),
>;

/// One inline rule, as the command spells it: `NAME` for any kind to any
/// kind, or `NAME=SOURCE:TARGET`.
fn inline_rule(text: &str, kinds: bool) -> Result<RelationRule, Refusal> {
    let refused = || {
        Refusal::usage(format!(
            "a relate rule is NAME or NAME=SOURCE:TARGET, not '{text}'"
        ))
    };
    let (name, source, target) = match text.split_once('=') {
        None if text.contains(':') => return Err(refused()),
        None => (text, ANY, ANY),
        Some((name, ends)) => {
            let (source, target) = ends.split_once(':').ok_or_else(refused)?;
            if name.contains('=') || source.contains(['=', ':']) || target.contains(['=', ':']) {
                return Err(refused());
            }
            (name, source, target)
        }
    };
    if !kinds && (source != ANY || target != ANY) {
        return Err(Refusal::usage(
            "a two-column relate query reads kind *, so it takes bare relation names",
        ));
    }
    Ok(RelationRule::one_way(name, source, target)?)
}

/// The inline rules as one relate request. `inline_rule` copies the
/// command's `inline_rule` in `crates/thinkthen/src/core/relate_file.rs:295`,
/// which the public API does not export.
fn inline(rules: &[String], kinds: bool) -> Result<Relate, Refusal> {
    let mut ask = Relate::builder();
    for rule in rules {
        ask = ask.relation(inline_rule(rule, kinds)?)?;
    }
    Ok(ask.build()?)
}

/// One cell of a wrapped relate row, refused when null.
fn cell<T: pgrx::FromDatum + pgrx::IntoDatum>(
    row: &pgrx::spi::SpiHeapTupleData<'_>,
    at: usize,
    what: &str,
) -> Result<T, Refusal> {
    row.get::<T>(at)
        .map_err(|error| Refusal::usage(format!("the relate query did not run: {error}")))?
        .ok_or_else(|| Refusal::usage(format!("the relate query returned a null {what}")))
}

/// Whether the query names kinds, and its rows as `(id, name, kind)`,
/// refused at the 256th.
fn rows(query: &str) -> Result<(bool, Vec<Row>), Refusal> {
    let query = query.trim().trim_end_matches(';');
    if query.is_empty() {
        return Err(Refusal::usage("the relate query is empty"));
    }
    let ran = |error: pgrx::spi::SpiError| {
        Refusal::usage(format!("the relate query did not run: {error}"))
    };
    Spi::connect(|client| {
        let columns = client
            .select(
                &format!("SELECT * FROM ({query}) AS ask LIMIT 0"),
                None,
                &[],
            )
            .map_err(ran)?
            .columns()
            .map_err(ran)?;
        let wrapped = match columns {
            2 => format!(
                "SELECT a.i::bigint, a.n::text, '{ANY}'::text FROM ({query}) AS a(i, n) LIMIT {ROW_LIMIT}"
            ),
            3 => format!(
                "SELECT a.i::bigint, a.n::text, a.k::text FROM ({query}) AS a(i, n, k) LIMIT {ROW_LIMIT}"
            ),
            _ => {
                return Err(Refusal::usage(
                    "a relate query returns id and name, or id, name, and kind",
                ));
            }
        };
        let table = client.select(&wrapped, None, &[]).map_err(ran)?;
        if i64::try_from(table.len()).unwrap_or(i64::MAX) >= ROW_LIMIT {
            return Err(Refusal::usage(format!(
                "thinkthen_relate reads at most {} rows",
                ROW_LIMIT - 1
            )));
        }
        let mut out = Vec::with_capacity(table.len());
        for row in table {
            out.push((
                cell(&row, 1, "id")?,
                cell(&row, 2, "name")?,
                cell(&row, 3, "kind")?,
            ));
        }
        Ok((columns == 3, out))
    })
}

/// Dedupe rows by name and kind in first-seen order, call `relate_with`
/// once, and return one row per matching id pair.
fn relate(query: &str, ask: impl FnOnce(bool) -> Result<Relate, Refusal>) -> Edges {
    let (kinds, rows) = rows(query).or_raise();
    let ask = ask(kinds).or_raise();
    let mut ids: HashMap<(String, String), Vec<i64>> = HashMap::new();
    let mut entities = Vec::new();
    for (id, name, kind) in rows {
        let held = ids.entry((name.clone(), kind.clone())).or_default();
        if held.is_empty() {
            entities.push(Entity::new(&name, &kind).or_raise());
        }
        held.push(id);
    }
    let edges = call::run(call::read(), move |engine, options| {
        engine.relate_with(&ask, entities, options)
    })
    .into_value();
    let pair = |entity: &Entity| {
        ids.get(&(entity.name().to_owned(), entity.kind().to_owned()))
            .cloned()
            .unwrap_or_default()
    };
    let mut out = Vec::new();
    for edge in &edges {
        for source in pair(edge.source()) {
            for target in pair(edge.target()) {
                out.push((
                    edge.relation().to_owned(),
                    source,
                    target,
                    edge.probability(),
                    edge.either(),
                ));
            }
        }
    }
    TableIterator::new(out)
}

/// Inline rules, as the command's `--relation` spells them.
#[pg_extern(parallel_restricted)]
#[allow(
    clippy::type_complexity,
    reason = "pgrx reads the columns from the signature, not an alias"
)]
fn thinkthen_relate(
    query: Option<&str>,
    rules: Option<Array<'_, &str>>,
) -> TableIterator<
    'static,
    (
        name!(relation, String),
        name!(source, i64),
        name!(target, i64),
        name!(probability, f64),
        name!(either, bool),
    ),
> {
    call::guarded(|| {
        let rules: Vec<String> = rules
            .iter()
            .flat_map(|held| held.iter().flatten().map(str::to_owned))
            .collect();
        if rules.is_empty() {
            call::raise(Refusal::usage("relate needs at least one relation rule"));
        }
        relate(query.unwrap_or_default(), |kinds| inline(&rules, kinds))
    })
}

/// A version-one relate file, as `'@file.json'` or JSON text.
#[pg_extern(name = "thinkthen_relate", parallel_restricted)]
#[allow(
    clippy::type_complexity,
    reason = "pgrx reads the columns from the signature, not an alias"
)]
fn thinkthen_relate_file(
    query: Option<&str>,
    rules: Option<&str>,
) -> TableIterator<
    'static,
    (
        name!(relation, String),
        name!(source, i64),
        name!(target, i64),
        name!(probability, f64),
        name!(either, bool),
    ),
> {
    call::guarded(|| {
        let ask = crate::given(rules, "relate file")
            .parse(Relate::from_json)
            .or_raise();
        relate(query.unwrap_or_default(), |_| Ok(ask))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Decision 10: the command's inline spellings, and the two-column
    /// query's refusal of a rule that names kinds.
    #[test]
    fn inline_rules_read_as_the_command_reads_them() {
        assert_eq!(
            inline_rule("caused_by", false).map(|rule| rule.name().to_owned()),
            Ok("caused_by".to_owned())
        );
        assert!(inline_rule("works_for=person:organization", true).is_ok());
        assert_eq!(
            inline_rule("works_for=person:organization", false).err(),
            Some(Refusal::usage(
                "a two-column relate query reads kind *, so it takes bare relation names"
            ))
        );
        for bad in ["a:b", "a=b", "a=b:c:d", "a=b=c:d"] {
            assert_eq!(
                inline_rule(bad, true).err(),
                Some(Refusal::usage(format!(
                    "a relate rule is NAME or NAME=SOURCE:TARGET, not '{bad}'"
                )))
            );
        }
    }
}
