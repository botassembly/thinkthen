//! `thinkthen_relate(query, rules)`: relations among the rows a query
//! returns (decision 10, ADR 0047 item 9).

use std::collections::HashMap;

use pgrx::Spi;
use pgrx::datum::Array;
use pgrx::prelude::*;
use thinkthen::{Entity, Error, Relate, RelationRule};

use crate::call::{self, OrRaise as _};

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

/// One inline rule through the command's parser, `NAME` or
/// `NAME=SOURCE:TARGET`, refused in this extension's own sentence.
fn inline_rule(text: &str, kinds: bool) -> Result<RelationRule, Error> {
    let rule = RelationRule::parse_inline(text, false).map_err(|_| {
        call::usage(format!(
            "a relate rule is NAME or NAME=SOURCE:TARGET, not '{text}'"
        ))
    })?;
    if !kinds && (rule.source() != ANY || rule.target() != ANY) {
        return Err(call::usage(
            "a two-column relate query reads kind *, so it takes bare relation names",
        ));
    }
    Ok(rule)
}

/// The inline rules as one relate request.
fn inline(rules: &[String], kinds: bool) -> Result<Relate, Error> {
    let mut ask = Relate::builder();
    for rule in rules {
        ask = ask.relation(inline_rule(rule, kinds)?)?;
    }
    ask.build()
}

/// One cell of a wrapped relate row, refused when null.
fn cell<T: pgrx::FromDatum + pgrx::IntoDatum>(
    row: &pgrx::spi::SpiHeapTupleData<'_>,
    at: usize,
    what: &str,
) -> Result<T, Error> {
    row.get::<T>(at)
        .map_err(|error| call::usage(format!("the relate query did not run: {error}")))?
        .ok_or_else(|| call::usage(format!("the relate query returned a null {what}")))
}

/// Whether the query names kinds, and its rows as `(id, name, kind)`,
/// refused at the 256th.
fn rows(query: &str) -> Result<(bool, Vec<Row>), Error> {
    let query = query.trim().trim_end_matches(';');
    if query.is_empty() {
        return Err(call::usage("the relate query is empty"));
    }
    let ran =
        |error: pgrx::spi::SpiError| call::usage(format!("the relate query did not run: {error}"));
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
        let row_limit = Relate::max_record_count() + 1;
        let wrapped = match columns {
            2 => format!(
                "SELECT a.i::bigint, a.n::text, '{ANY}'::text FROM ({query}) AS a(i, n) LIMIT {row_limit}"
            ),
            3 => format!(
                "SELECT a.i::bigint, a.n::text, a.k::text FROM ({query}) AS a(i, n, k) LIMIT {row_limit}"
            ),
            _ => {
                return Err(call::usage(
                    "a relate query returns id and name, or id, name, and kind",
                ));
            }
        };
        let table = client.select(&wrapped, None, &[]).map_err(ran)?;
        Relate::admit_record_count(table.len())
            .map_err(|_| call::usage("thinkthen_relate reads at most 255 rows"))?;
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
fn relate(query: &str, ask: impl FnOnce(bool) -> Result<Relate, Error>) -> Edges {
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
        let items = entities
            .into_iter()
            .map(|entity| {
                let value = serde_json::json!({"name":entity.name(),"kind":entity.kind()});
                crate::request::json(value)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let result = crate::request::run(
            engine,
            thinkthen::RequestFunction::Relate,
            ask.into(),
            items,
            options,
        )?;
        match result.value() {
            thinkthen::RequestValue::Related(row) => Ok(row.result().value().to_vec()),
            _ => Err(crate::request::wrong_result()),
        }
    });
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
            call::raise(call::usage("relate needs at least one relation rule"));
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
        let read =
            |text, kinds| call::shown(inline_rule(text, kinds).map(|rule| rule.name().to_owned()));
        assert_eq!(read("caused_by", false), Ok("caused_by".to_owned()));
        // `ANY` reads as any kind, as the command reads it (ticket 0347).
        assert_eq!(read("caused_by=ANY:ANY", false), Ok("caused_by".to_owned()));
        let rule = inline_rule("works_for=ANY:organization", true).expect("one side any");
        assert_eq!((rule.source(), rule.target()), ("*", "organization"));
        assert_eq!(
            read("works_for=person:organization", true),
            Ok("works_for".to_owned())
        );
        assert_eq!(
            read("works_for=person:organization", false),
            Err("thinkthen usage: a two-column relate query reads kind *, so it takes bare relation names (retryable: no)".to_owned())
        );
        for bad in ["a:b", "a=b", "a=b:c:d", "a=b=c:d", "a=:b", " "] {
            assert_eq!(
                read(bad, true),
                Err(format!(
                    "thinkthen usage: a relate rule is NAME or NAME=SOURCE:TARGET, not '{bad}' (retryable: no)"
                ))
            );
        }
    }
}
