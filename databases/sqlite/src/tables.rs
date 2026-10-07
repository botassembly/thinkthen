//! The two table-valued functions, `thinkthen_recognize` and
//! `thinkthen_relate`. `ffi` holds their virtual-table glue; this module
//! holds what they do.

use std::collections::HashMap;
use std::ffi::{CStr, c_int};
use std::sync::{Arc, Mutex};

use rusqlite::ffi::sqlite3;
use rusqlite::types::{Value, ValueRef};
use rusqlite::vtab::{Filters, IndexConstraintOp, IndexInfo};
use serde_json::json;
use thinkthen::{Entity, Kind, Recognize, Relate, Settings};

use crate::many::Store;
use crate::question::{call_controls, from_file, named_file, shown, text};
use crate::{Failure, ffi, worker};

/// A scan borrows one immutable answer set, with an optional key selection.
#[derive(Debug)]
pub(crate) struct Scan {
    pub(crate) rows: Arc<Vec<Vec<Value>>>,
    pub(crate) selected: Option<usize>,
}

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
    /// The default scan copies only the few scalar hidden arguments.
    fn scan(
        db: *mut sqlite3,
        mask: c_int,
        filters: &Filters<'_>,
        _: &Mutex<Store>,
    ) -> Result<Scan, Failure> {
        Ok(Scan {
            rows: Arc::new(Self::rows(db, &arguments::<Self>(mask, filters)?)?),
            selected: None,
        })
    }
}

/// Bind each usable equality on an argument column, in column order.
pub(crate) fn plan<T: Table>(info: &mut IndexInfo) -> Result<bool, Failure> {
    let mut bound = vec![None; T::COLUMNS - T::FIRST_HIDDEN];
    for (at, constraint) in info.constraints().enumerate() {
        let lookup = T::NAME.ends_with("_many") && constraint.column() == 0;
        // A keyed row has TEXT affinity, but SQLite may apply a caller's
        // collation or coerce a numeric right operand. Only binary text can
        // use our byte-for-byte candidate lookup; SQLite always rechecks it.
        if lookup && !matches!(info.collation(at), Ok("BINARY")) {
            continue;
        }
        let Some(column) = usize::try_from(constraint.column())
            .ok()
            .and_then(|column| column.checked_sub(T::FIRST_HIDDEN))
            .or_else(|| lookup.then_some(3))
        else {
            continue;
        };
        if !constraint.is_usable()
            || constraint.operator() != IndexConstraintOp::SQLITE_INDEX_CONSTRAINT_EQ
        {
            // An outer-row key equality may be unavailable to this plan.
            // Leave it to SQLite; required hidden arguments are checked below.
            continue;
        }
        if let Some(slot) = bound.get_mut(column) {
            *slot = Some((at, lookup));
        }
    }
    if bound.iter().take(T::REQUIRED).any(Option::is_none) {
        return Ok(false);
    }
    let (mut mask, mut place): (c_int, c_int) = (0, 0);
    for (column, at) in bound.iter().enumerate() {
        if let Some((at, visible_lookup)) = at {
            place += 1;
            mask |= 1 << column;
            let mut usage = info.constraint_usage(*at);
            usage.set_argv_index(place);
            usage.set_omit(!visible_lookup);
        }
    }
    info.set_idx_num(mask);
    info.set_estimated_cost(100.0);
    Ok(true)
}

/// The argument values `plan` bound, with NULL for an argument not given.
pub(crate) fn arguments<T: Table + ?Sized>(
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

fn controls(arguments: &[Value], at: usize) -> Result<Settings, Failure> {
    call_controls(arguments.get(at).map_or(ValueRef::Null, ValueRef::from))
}

/// Validate a JSON argument or file while retaining its original member order.
fn json_source(
    argument: &str,
    section: &str,
) -> Result<(String, serde_json::Value, bool), Failure> {
    let (source, file) = match argument.strip_prefix('@') {
        Some(_) => (named_file(argument, &format!("{section} spec"))?.0, true),
        None => (argument.to_owned(), false),
    };
    let value: serde_json::Value = serde_json::from_str(&source)
        .map_err(|error| Failure::usage(format!("the {section} argument is not JSON: {error}")))?;
    Ok((source, value, file))
}

/// A JSON argument or file read as a version-one file, wrapping a bare section.
pub(crate) fn file_json(
    argument: &str,
    section: &str,
) -> Result<(serde_json::Value, bool), Failure> {
    let (_, value, file) = json_source(argument, section)?;
    let whole = if value.get(section).is_some() {
        value
    } else {
        json!({ "version": 1, section: value })
    };
    Ok((whole, file))
}

/// The same wrapper, preserving object order for recognition's request identity.
pub(crate) fn ordered_file_json(argument: &str, section: &str) -> Result<(String, bool), Failure> {
    let (source, value, file) = json_source(argument, section)?;
    let whole = if value.get(section).is_some() {
        source
    } else {
        format!(r#"{{"version":1,"{section}":{source}}}"#)
    };
    Ok((whole, file))
}

/// Read a JSON spec, where a broken rule in a file is `local` (0095, Q16).
pub(crate) fn read<T>(
    whole: &serde_json::Value,
    file: bool,
    parse: fn(&str) -> Result<T, thinkthen::Error>,
) -> Result<T, Failure> {
    parse(&whole.to_string()).map_err(|error| from_file(error, file))
}

/// `thinkthen_recognize(body, kinds)`: one row per name in one text.
#[derive(Debug)]
pub(crate) struct Recognizer;

impl Recognizer {
    /// Kinds as a comma list, a JSON recognize section or file, or `'@name'`.
    fn ask(kinds: Option<&str>) -> Result<Recognize, Failure> {
        let kinds = kinds.map(str::trim).unwrap_or_default();
        if kinds.starts_with('[') {
            return Err(Failure::usage(
                "recognize kinds take comma names or a recognize JSON object, not a JSON array",
            ));
        }
        if kinds.starts_with('{') || kinds.starts_with('@') {
            let (source, file) = ordered_file_json(kinds, "recognize")?;
            let ask = Recognize::from_json(&source).map_err(|error| from_file(error, file))?;
            let whole: serde_json::Value = serde_json::from_str(&source)
                .map_err(|_| Failure::defect("validated recognize JSON could not be read"))?;
            if whole.pointer("/recognize/relations").is_some() {
                return Err(Failure::plain_usage(
                    "thinkthen_recognize takes no relations; relate rows with thinkthen_relate",
                ));
            }
            return Ok(ask);
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
        c"CREATE TABLE x(text, start, end, length, kind, strength, body HIDDEN, kinds HIDDEN, settings HIDDEN)";
    const FIRST_HIDDEN: usize = 6;
    const COLUMNS: usize = 9;
    const REQUIRED: usize = 1;

    fn rows(db: *mut sqlite3, arguments: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
        let Some(evidence) = argument(arguments, 0, "the text")? else {
            return Ok(Vec::new());
        };
        let ask = Self::ask(argument(arguments, 1, "the kinds")?.as_deref())?;
        let settings = controls(arguments, 2)?;
        let shared = settings.context().map(str::to_owned);
        let found = worker::run_settings(db, settings, move |engine, options| {
            let options = if let Some(shared) = &shared {
                options.context(shared)
            } else {
                options
            };
            Ok(engine.recognize_with(&ask, &evidence, options)?)
        })?
        .into_value();
        found
            .entities()
            .iter()
            .map(|entity| {
                let offset = |at: usize| {
                    i64::try_from(at).map_err(|_| Failure::defect("an offset is too large"))
                };
                Ok(vec![
                    Value::Text(entity.text().to_owned()),
                    Value::Integer(offset(entity.start())?),
                    Value::Integer(offset(entity.end())?),
                    Value::Integer(offset(entity.length())?),
                    Value::Text(entity.kind().to_owned()),
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

/// `thinkthen_relate(query, rules[, settings])`: edges among selected entities.
#[derive(Debug)]
pub(crate) struct Relater;

/// The distinct entities in first-seen order, and the ids that hold each.
type Read = (Vec<Entity>, HashMap<(String, String), Vec<Value>>);

impl Relater {
    /// One to four inline rules, or one JSON relate section, file, or `'@name'`.
    fn ask(argument: &str) -> Result<Relate, Failure> {
        if argument.starts_with('{') || argument.starts_with('@') {
            let (whole, file) = file_json(argument, "relate")?;
            return read(&whole, file, Relate::from_json);
        }
        let rules: Vec<String> = if argument.starts_with('[') {
            serde_json::from_str(argument)
                .map_err(|_| Failure::usage("relate rules are a JSON array of text"))?
        } else {
            vec![argument.to_owned()]
        };
        if rules.is_empty() || rules.len() > 4 {
            return Err(Failure::usage("thinkthen_relate needs one to four rules"));
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

    /// Read exactly id, name and optional kind from the caller's read-only SELECT.
    fn entities(db: *mut sqlite3, query: &str) -> Result<Read, Failure> {
        if !query
            .trim_start()
            .to_ascii_uppercase()
            .starts_with("SELECT ")
        {
            return Err(Failure::usage(
                "relate takes a query and rules; pass 'SELECT id, name, kind FROM …'",
            ));
        }
        let refused = |error: rusqlite::Error| {
            Failure::usage(format!("thinkthen_relate cannot read its query: {error}"))
        };
        let connection = ffi::connection(db).map_err(refused)?;
        let mut statement = connection.prepare(query).map_err(refused)?;
        if !statement.readonly() || !matches!(statement.column_count(), 2 | 3) {
            return Err(Failure::usage(
                "relate takes a read-only SELECT returning id, name, kind",
            ));
        }
        let has_kind = statement.column_count() == 3;
        let mut rows = statement.query([]).map_err(refused)?;
        let (mut order, mut ids) = (Vec::new(), HashMap::<(String, String), Vec<Value>>::new());
        let mut source_rows = 0;
        while let Some(row) = rows.next().map_err(refused)? {
            source_rows += 1;
            if source_rows > MOST_PAIRS {
                return Err(Failure::usage(
                    "thinkthen_relate takes at most 255 source rows",
                ));
            }
            let held: Value = row.get(0).map_err(refused)?;
            let pair = (
                named(row, 1, "name", &held)?,
                if has_kind {
                    named(row, 2, "kind", &held)?
                } else {
                    "*".to_owned()
                },
            );
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
    const SCHEMA: &'static CStr = c"CREATE TABLE x(relation, source, target, probability, either, query HIDDEN, rules HIDDEN, settings HIDDEN, legacy_4 HIDDEN, legacy_5 HIDDEN, legacy_6 HIDDEN, legacy_7 HIDDEN, legacy_8 HIDDEN, legacy_9 HIDDEN)";
    const FIRST_HIDDEN: usize = 5;
    const COLUMNS: usize = 14;
    const REQUIRED: usize = 2;

    fn rows(db: *mut sqlite3, arguments: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
        if arguments
            .iter()
            .skip(3)
            .any(|value| !matches!(value, Value::Null))
        {
            return Err(Failure::plain_usage(
                "relate takes a query and rules; pass 'SELECT id, name, kind FROM …'",
            ));
        }
        let query = argument(arguments, 0, "the query")?
            .ok_or_else(|| Failure::usage("thinkthen_relate needs a query"))?;
        let rules = argument(arguments, 1, "the rules")?
            .ok_or_else(|| Failure::usage("thinkthen_relate needs rules"))?;
        let ask = Self::ask(&rules)?;
        let settings = controls(arguments, 2)?;
        let shared = settings.context().map(str::to_owned);
        let (entities, ids) = Self::entities(db, &query)?;
        let edges = worker::run_settings(db, settings, move |engine, options| {
            let options = if let Some(shared) = &shared {
                options.context(shared)
            } else {
                options
            };
            Ok(engine.relate_with(&ask, entities, options)?)
        })?
        .into_value();
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
                    Value::Integer(i64::from(edge.either())),
                ]
            }));
        }
        Ok(rows)
    }
}
