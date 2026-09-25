//! The two table-valued functions, `thinkthen_recognize` and
//! `thinkthen_relate`. `ffi` holds their virtual-table glue; this module
//! holds what they do.

use std::collections::HashMap;
use std::ffi::{CStr, c_int};

use rusqlite::ffi::sqlite3;
use rusqlite::types::{Value, ValueRef};
use rusqlite::vtab::{Filters, IndexConstraintOp, IndexInfo};
use serde_json::json;
use thinkthen::{Entity, Kind, Recognize, Relate};

use crate::question::{named_file, shown, text};
use crate::{Failure, ffi, worker};

/// The most distinct name and kind pairs one relate call takes.
const MOST_PAIRS: usize = 255;

/// One table-valued function: its schema, its hidden arguments, and its rows.
pub(crate) trait Table {
    /// The function's name, for the panic guard.
    const NAME: &'static str;
    /// The declared columns; the arguments are the hidden ones at the end.
    const SCHEMA: &'static CStr;
    /// The first hidden column.
    const FIRST_HIDDEN: usize;
    /// Every column.
    const COLUMNS: usize;
    /// How many leading arguments must be bound.
    const REQUIRED: usize;
    /// The answer rows, one value per visible column.
    fn rows(db: *mut sqlite3, arguments: &[Value]) -> Result<Vec<Vec<Value>>, Failure>;
}

/// Bind each usable equality on an argument column, in column order.
pub(crate) fn plan<T: Table>(info: &mut IndexInfo) -> Result<bool, Failure> {
    let mut bound = vec![None; T::COLUMNS - T::FIRST_HIDDEN];
    for (at, constraint) in info.constraints().enumerate() {
        let Some(column) = usize::try_from(constraint.column())
            .ok()
            .and_then(|column| column.checked_sub(T::FIRST_HIDDEN))
        else {
            continue;
        };
        if !constraint.is_usable()
            || constraint.operator() != IndexConstraintOp::SQLITE_INDEX_CONSTRAINT_EQ
        {
            return Ok(false);
        }
        if let Some(slot) = bound.get_mut(column) {
            *slot = Some(at);
        }
    }
    if bound.iter().take(T::REQUIRED).any(Option::is_none) {
        return Ok(false);
    }
    let (mut mask, mut place): (c_int, c_int) = (0, 0);
    for (column, at) in bound.iter().enumerate() {
        if let Some(at) = at {
            place += 1;
            mask |= 1 << column;
            let mut usage = info.constraint_usage(*at);
            usage.set_argv_index(place);
            usage.set_omit(true);
        }
    }
    info.set_idx_num(mask);
    info.set_estimated_cost(100.0);
    Ok(true)
}

/// The argument values `plan` bound, with NULL for an argument not given.
pub(crate) fn arguments<T: Table>(
    mask: c_int,
    filters: &Filters<'_>,
) -> Result<Vec<Value>, Failure> {
    let mut given = 0;
    let mut values = Vec::new();
    for column in 0..T::COLUMNS - T::FIRST_HIDDEN {
        if mask & (1 << column) == 0 {
            values.push(Value::Null);
        } else {
            values.push(
                filters
                    .get::<Value>(given)
                    .map_err(|error| Failure::defect(error.to_string()))?,
            );
            given += 1;
        }
    }
    Ok(values)
}

/// One argument as text, `None` for NULL or absent.
fn argument(arguments: &[Value], at: usize, what: &str) -> Result<Option<String>, Failure> {
    arguments
        .get(at)
        .map_or(Ok(None), |value| text(ValueRef::from(value), what))
}

/// A JSON argument or file read as a version-one file, wrapping a bare section.
fn file_json(argument: &str, section: &str) -> Result<(serde_json::Value, bool), Failure> {
    let (source, file) = match argument.strip_prefix('@') {
        Some(_) => (named_file(argument)?.0, true),
        None => (argument.to_owned(), false),
    };
    let value: serde_json::Value = serde_json::from_str(&source)
        .map_err(|error| Failure::usage(format!("the {section} argument is not JSON: {error}")))?;
    let whole = if value.get(section).is_some() {
        value
    } else {
        json!({ "version": 1, section: value })
    };
    Ok((whole, file))
}

/// Read a JSON spec, where a broken rule in a file is `local` (0095, Q16).
fn read<T>(
    whole: &serde_json::Value,
    file: bool,
    parse: fn(&str) -> Result<T, thinkthen::Error>,
) -> Result<T, Failure> {
    parse(&whole.to_string()).map_err(|error| {
        let mut failure = Failure::from(error);
        if file && failure.kind == thinkthen::ErrorKind::Usage {
            failure.kind = thinkthen::ErrorKind::Local;
        }
        failure
    })
}

/// `thinkthen_recognize(text, kinds)`: one row per name in one text.
#[derive(Debug)]
pub(crate) struct Recognizer;

impl Recognizer {
    /// Kinds as a comma list, a JSON recognize section or file, or `'@name'`.
    fn ask(kinds: Option<&str>) -> Result<Recognize, Failure> {
        let kinds = kinds.map(str::trim).unwrap_or_default();
        if kinds.starts_with('{') || kinds.starts_with('@') {
            let (whole, file) = file_json(kinds, "recognize")?;
            if whole.pointer("/recognize/relations").is_some() {
                return Err(Failure::usage(
                    "thinkthen_recognize takes no relations; relate rows with thinkthen_relate",
                ));
            }
            return read(&whole, file, Recognize::from_json);
        }
        let mut builder = Recognize::builder();
        for name in kinds
            .split(',')
            .map(str::trim)
            .filter(|name| !name.is_empty())
        {
            builder = builder.kind(Kind::new(name, None)?)?;
        }
        Ok(builder.build()?)
    }
}

impl Table for Recognizer {
    const NAME: &'static str = "thinkthen_recognize";
    const SCHEMA: &'static CStr =
        c"CREATE TABLE x(name, kind, start, end, strength, text HIDDEN, kinds HIDDEN)";
    const FIRST_HIDDEN: usize = 5;
    const COLUMNS: usize = 7;
    const REQUIRED: usize = 1;

    fn rows(db: *mut sqlite3, arguments: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
        let Some(evidence) = argument(arguments, 0, "the text")? else {
            return Ok(Vec::new());
        };
        let ask = Self::ask(argument(arguments, 1, "the kinds")?.as_deref())?;
        let found = worker::run(db, None, move |engine, options| {
            Ok(engine.recognize_with(&ask, &evidence, options)?)
        })?;
        found
            .entities()
            .iter()
            .map(|entity| {
                let offset = |at: usize| {
                    i64::try_from(at).map_err(|_| Failure::defect("an offset is too large"))
                };
                Ok(vec![
                    Value::Text(entity.name().to_owned()),
                    Value::Text(entity.kind().to_owned()),
                    Value::Integer(offset(entity.start())?),
                    Value::Integer(offset(entity.end())?),
                    Value::Real(entity.strength()),
                ])
            })
            .collect()
    }
}

/// One inline rule: `NAME`, `NAME=SOURCE:TARGET`, each with an optional `either:`.
fn inline(rule: &str) -> Result<serde_json::Value, Failure> {
    let (either, rule) = rule
        .strip_prefix("either:")
        .map_or((false, rule), |rest| (true, rest));
    let (name, ends) = rule.split_once('=').unwrap_or((rule, "*:*"));
    let (source, target) = ends.split_once(':').ok_or_else(|| {
        Failure::usage(format!(
            "the relation rule {rule} names one end; write NAME=SOURCE:TARGET"
        ))
    })?;
    Ok(json!({ "name": name, "source": source, "target": target, "either": either }))
}

/// A row's name or kind, or `usage` naming the row's id.
fn named(row: &rusqlite::Row<'_>, at: usize, what: &str, id: &Value) -> Result<String, Failure> {
    let value = row
        .get_ref(at)
        .map_err(|error| Failure::usage(error.to_string()))?;
    let value = text(value, what)?.unwrap_or_default();
    if value.trim().is_empty() {
        return Err(Failure::usage(format!(
            "the row with id {} has no {what}",
            shown(ValueRef::from(id))
        )));
    }
    Ok(value)
}

/// Add one distinct pair, refusing the 256th.
fn admit(order: &mut Vec<Entity>, (name, kind): &(String, String)) -> Result<(), Failure> {
    if order.len() == MOST_PAIRS {
        return Err(Failure::usage(format!(
            "thinkthen_relate takes at most {MOST_PAIRS} distinct name and kind pairs"
        )));
    }
    order.push(Entity::new(name, kind)?);
    Ok(())
}

/// `thinkthen_relate(table, id_column, name_column, kind_column, rule, …)`:
/// the edges among a table's entities, one row per pair of rows that hold
/// an edge's two ends (ADR 0047 item 9).
#[derive(Debug)]
pub(crate) struct Relater;

/// The distinct entities in first-seen order, and the ids that hold each.
type Read = (Vec<Entity>, HashMap<(String, String), Vec<Value>>);

impl Relater {
    /// One to four inline rules, or one JSON relate section, file, or `'@name'`.
    fn ask(rules: &[String]) -> Result<Relate, Failure> {
        if let [only] = rules
            && (only.starts_with('{') || only.starts_with('@'))
        {
            let (whole, file) = file_json(only, "relate")?;
            return read(&whole, file, Relate::from_json);
        }
        let relations = rules
            .iter()
            .map(|rule| inline(rule))
            .collect::<Result<Vec<_>, _>>()?;
        read(
            &json!({ "version": 1, "relate": { "relations": relations } }),
            false,
            Relate::from_json,
        )
    }

    /// Read the table's entities with a nested read-only `SELECT`.
    fn entities(db: *mut sqlite3, names: &[String]) -> Result<Read, Failure> {
        let quote = |name: &String| format!("\"{}\"", name.replace('"', "\"\""));
        let [table, id, name, kind] = names else {
            return Err(Failure::defect("relate lost its column names"));
        };
        let sql = format!(
            "SELECT {}, {}, {} FROM {}",
            quote(id),
            quote(name),
            quote(kind),
            quote(table)
        );
        let refused = |error: rusqlite::Error| {
            Failure::usage(format!("thinkthen_relate cannot read {table}: {error}"))
        };
        let connection = ffi::connection(db).map_err(refused)?;
        let mut statement = connection.prepare(&sql).map_err(refused)?;
        let mut rows = statement.query([]).map_err(refused)?;
        let (mut order, mut ids) = (Vec::new(), HashMap::<(String, String), Vec<Value>>::new());
        while let Some(row) = rows.next().map_err(refused)? {
            let held: Value = row.get(0).map_err(refused)?;
            let pair = (named(row, 1, "name", &held)?, named(row, 2, "kind", &held)?);
            if !ids.contains_key(&pair) {
                admit(&mut order, &pair)?;
            }
            ids.entry(pair).or_default().push(held);
        }
        Ok((order, ids))
    }
}

impl Table for Relater {
    const NAME: &'static str = "thinkthen_relate";
    const SCHEMA: &'static CStr = c"CREATE TABLE x(relation, source, target, probability, table_name HIDDEN, id_column HIDDEN, name_column HIDDEN, kind_column HIDDEN, rule_1 HIDDEN, rule_2 HIDDEN, rule_3 HIDDEN, rule_4 HIDDEN)";
    const FIRST_HIDDEN: usize = 4;
    const COLUMNS: usize = 12;
    const REQUIRED: usize = 4;

    fn rows(db: *mut sqlite3, arguments: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
        let mut names = Vec::new();
        for (at, what) in [
            "the table",
            "the id column",
            "the name column",
            "the kind column",
        ]
        .iter()
        .enumerate()
        {
            names.push(
                argument(arguments, at, what)?
                    .ok_or_else(|| Failure::usage(format!("thinkthen_relate needs {what}")))?,
            );
        }
        let mut rules = Vec::new();
        for at in 4..8 {
            rules.extend(argument(arguments, at, "a rule")?.filter(|rule| !rule.trim().is_empty()));
        }
        if rules.is_empty() {
            return Err(Failure::usage("thinkthen_relate needs one to four rules"));
        }
        let ask = Self::ask(&rules)?;
        let (entities, ids) = Self::entities(db, &names)?;
        let edges = worker::run(db, None, move |engine, options| {
            Ok(engine.relate_with(&ask, entities, options)?)
        })?;
        let mut rows = Vec::new();
        for edge in &edges {
            let held =
                |entity: &Entity| ids.get(&(entity.name().to_owned(), entity.kind().to_owned()));
            let (Some(sources), Some(targets)) = (held(edge.source()), held(edge.target())) else {
                return Err(Failure::defect("an edge names an entity no row holds"));
            };
            let pairs = sources
                .iter()
                .flat_map(|source| targets.iter().map(move |target| (source, target)));
            rows.extend(pairs.map(|(source, target)| {
                vec![
                    Value::Text(edge.relation().to_owned()),
                    source.clone(),
                    target.clone(),
                    Value::Real(edge.probability()),
                ]
            }));
        }
        Ok(rows)
    }
}
