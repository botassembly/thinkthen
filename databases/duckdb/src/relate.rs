//! `thinkthen_relate(query, rules)`: relate from rows on the caller's own
//! database (ticket 0118, ADR 0038).
//!
//! The query returns `id, name, kind`, or `id, name` with every kind `*`.
//! The rules are `'@file.json'`, the relate file as JSON text, or a `LIST`
//! of the command's inline rules. The binding dedupes rows by name and kind
//! in first-seen order, asks the engine once, and returns one row
//! `(relation, source, target, probability)` per matching id pair.
//!
//! The query runs on the kept connection of the caller's database as one
//! read-only `SELECT` under the caller's search path, capped at 255 rows
//! and at `thinkthen_relate_seconds`. A relate nested in a relate query
//! refuses before it waits on the gate its own outer query holds.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use thinkthen::{CallOptions, Engine, Entity, Relate, RelationRule};

use crate::connections::{self, Conn, Kept};
use crate::engines;
use crate::errors::{defect, failure, prefix, usage};
use crate::ffi::{Type, Value};
use crate::questions::Files;
use crate::questions::{Caller, from_file};
use crate::signal::Invoke;
use crate::worker;

pub(crate) mod ffi;

/// The result columns.
pub(crate) const COLUMNS: [(&std::ffi::CStr, Type); 4] = [
    (c"relation", Type::Text),
    (c"source", Type::Text),
    (c"target", Type::Text),
    (c"probability", Type::Double),
];

/// The most rows a relate query may return.
const MOST_ROWS: usize = 255;

/// The time limit's setting and default, in seconds; 0 turns it off.
pub(crate) const SECONDS: &std::ffi::CStr = c"thinkthen_relate_seconds";
const SECONDS_DEFAULT: u64 = 60;

/// The plan guard's setting and default: the most rows the planner may
/// estimate into one step that holds its whole input.
pub(crate) const HOLDING: &std::ffi::CStr = c"thinkthen_relate_holding_rows";
const HOLDING_DEFAULT: u64 = 1_000_000;

/// The `rules` argument as the caller gave it.
#[derive(Debug)]
pub(crate) enum Rules {
    Text(String),
    List(Vec<String>),
}

/// One bound relate: everything the scan needs, read once at bind.
#[derive(Debug)]
pub(crate) struct Bound {
    query: String,
    ask: Relate,
    wildcard: bool,
    caller: Caller,
    kept: Kept,
    seconds: u64,
    holding: u64,
    search_path: Option<String>,
}

/// One output row.
pub(crate) type Row = [Value; 4];

/// Bind one relate: read the arguments and the caller's settings, check the
/// cache folder, find the caller's kept connection, and read the rules.
pub(crate) fn bind(
    files: Files,
    query: Option<String>,
    rules: Option<Rules>,
) -> Result<Bound, String> {
    let query = query
        .filter(|query| !query.trim().is_empty())
        .ok_or_else(|| usage("the relate query is NULL or blank"))?;
    let rules = rules.ok_or_else(|| usage("the relate rules are NULL or hold a NULL rule"))?;
    let asked = files.settings();
    let seconds = count(
        &files,
        SECONDS,
        SECONDS_DEFAULT,
        "a relate time limit is a whole number of seconds, 0 for none",
    )?;
    let holding = count(
        &files,
        HOLDING,
        HOLDING_DEFAULT,
        "a relate holding limit is a whole number of rows",
    )?;
    let search_path = files
        .text(c"search_path")
        .filter(|path| !path.trim().is_empty());
    let engine = engines::engine_for(&asked, |path| files.probe(path))?;
    let kept = connections::for_caller(&files)?;
    let mut caller = Caller::new(engine, asked, files);
    let (ask, wildcard) = read_rules(&mut caller, rules)?;
    Ok(Bound {
        query,
        ask,
        wildcard,
        caller,
        kept,
        seconds,
        holding,
        search_path,
    })
}

fn count(files: &Files, name: &std::ffi::CStr, default: u64, refusal: &str) -> Result<u64, String> {
    files.number(name).map_or(Ok(default), |value| {
        u64::try_from(value).map_err(|_| usage(refusal))
    })
}

/// The ask, and whether every rule is `*:*`.
fn read_rules(caller: &mut Caller, rules: Rules) -> Result<(Relate, bool), String> {
    let text = match rules {
        Rules::List(rules) => return inline(&rules),
        Rules::Text(text) => text,
    };
    let (text, from_file_text) = match text.strip_prefix('@') {
        Some(path) => (caller.named_file(path, "rules file")?, true),
        None => (text, false),
    };
    let ask = Relate::from_json(&text).map_err(|error| {
        if from_file_text {
            from_file(&error)
        } else {
            failure(&error)
        }
    })?;
    let wildcard = serde_json::from_str::<serde_json::Value>(&text)
        .ok()
        .and_then(|file| file.pointer("/relate/relations")?.as_array().cloned())
        .is_some_and(|relations| {
            relations.iter().all(|rule| {
                ["source", "target"]
                    .iter()
                    .all(|side| rule.get(side).and_then(serde_json::Value::as_str) == Some("*"))
            })
        });
    Ok((ask, wildcard))
}

/// The command's inline rules: `NAME=SOURCE:TARGET`, or bare `NAME` for `*:*`.
fn inline(rules: &[String]) -> Result<(Relate, bool), String> {
    if rules.is_empty() {
        return Err(usage("relate needs at least one rule"));
    }
    let mut builder = Relate::builder();
    let mut wildcard = true;
    for rule in rules {
        let (name, source, target) = match rule.split_once('=') {
            None => (rule.as_str(), "*", "*"),
            Some((name, kinds)) => match kinds.split_once(':') {
                Some((source, target)) if !target.contains([':', '=']) => (name, source, target),
                _ => {
                    return Err(usage(&format!(
                        "the rule {rule} is not NAME or NAME=SOURCE:TARGET"
                    )));
                }
            },
        };
        wildcard &= source == "*" && target == "*";
        let made = RelationRule::one_way(name, source, target).map_err(|error| failure(&error))?;
        builder = builder.relation(made).map_err(|error| failure(&error))?;
    }
    Ok((builder.build().map_err(|error| failure(&error))?, wildcard))
}

/// The scan: run the query once on the kept connection, then ask the engine.
pub(crate) fn rows(bound: &Bound) -> Result<Vec<Row>, String> {
    let invoke = Invoke::begin();
    if bound.kept.is_caller(bound.caller.files()) {
        return Err(usage(
            "the relate query calls thinkthen_relate while its own query is running; nested relate cannot run, because the outer query waits on the connection the inner one needs",
        ));
    }
    let at = (bound.seconds > 0)
        .then(|| Instant::now().checked_add(Duration::from_secs(bound.seconds)))
        .flatten();
    let seconds = bound.seconds;
    let _gate = bound.kept.gate(|| {
        if invoke.stopped() {
            Some(cancelled())
        } else if at.is_some_and(|at| Instant::now() >= at) {
            Some(queue_limit(seconds))
        } else {
            None
        }
    })?;
    let found = {
        let _busy = bound.kept.busy();
        let (found, fired) = under_limit(bound.kept.connection(), at, || run_query(bound));
        if invoke.stopped() {
            return Err(cancelled());
        }
        if fired && found.is_err() {
            return Err(time_limit(seconds));
        }
        found.map_err(|message| boundary(bound.caller.files(), message))?
    };
    let (entities, ids) = entities(found, bound.wildcard)?;
    if entities.is_empty() {
        return Ok(Vec::new());
    }
    engines::within_total(&bound.caller.asked, Vec::new())?;
    let (engine, ask): (Arc<Engine>, Relate) =
        (Arc::clone(&bound.caller.engine), bound.ask.clone());
    let edges = worker::run(&invoke, move |token| {
        let options = CallOptions::new().cancel(token);
        let options = match at {
            Some(at) => options.deadline_at(at),
            None => options,
        };
        engine.relate_with(&ask, entities, options)
    })?
    .into_value();
    let key = |entity: &Entity| (entity.name().to_owned(), entity.kind().to_owned());
    let mut rows = Vec::new();
    for edge in edges {
        let (sources, targets) = (ids.get(&key(edge.source())), ids.get(&key(edge.target())));
        for source in sources.into_iter().flatten() {
            for target in targets.into_iter().flatten() {
                rows.push([
                    Value::Text(edge.relation().to_owned()),
                    Value::Text(source.clone()),
                    Value::Text(target.clone()),
                    Value::Double(edge.probability()),
                ]);
            }
        }
    }
    Ok(rows)
}

/// The query's rows as entities in first-seen order, and each entity's ids.
type Read = (Vec<Entity>, HashMap<(String, String), Vec<String>>);

fn entities(found: connections::Rows, wildcard: bool) -> Result<Read, String> {
    if found.rows.len() > MOST_ROWS {
        return Err(usage(
            "the relate query returned more than 255 rows, and relate reads at most 255; add a WHERE or a LIMIT",
        ));
    }
    if !matches!(found.columns, 2 | 3) {
        return Err(usage(
            "the relate query returns id, name, and kind, or id and name",
        ));
    }
    if found.columns == 2 && !wildcard {
        return Err(usage(
            "a relate query of id and name reads every kind as *, so every rule is bare or *:*",
        ));
    }
    let mut entities = Vec::new();
    let mut ids: HashMap<(String, String), Vec<String>> = HashMap::new();
    for (row, values) in found.rows.into_iter().enumerate() {
        let mut values = values.into_iter();
        let (Some(id), Some(name)) = (values.next().flatten(), values.next().flatten()) else {
            return Err(usage(&format!(
                "relate row {} holds a NULL id or name",
                row + 1
            )));
        };
        let kind = match values.next() {
            None => "*".to_owned(),
            Some(kind) => {
                kind.ok_or_else(|| usage(&format!("relate row {} holds a NULL kind", row + 1)))?
            }
        };
        let listed = ids.entry((name.clone(), kind.clone())).or_default();
        if listed.is_empty() {
            entities.push(Entity::new(&name, &kind).map_err(|error| failure(&error))?);
        }
        listed.push(id);
    }
    Ok((entities, ids))
}

/// One statement, of the `SELECT` kind, in a read-only transaction under
/// the caller's search path, with the plan guard, then the capped run.
fn run_query(bound: &Bound) -> Result<connections::Rows, String> {
    let connection = bound.kept.connection();
    let failed = |text: String| match text.find("thinkthen ") {
        Some(at) => text.get(at..).unwrap_or_default().to_owned(),
        None => usage(&format!("the relate query failed: {text}")),
    };
    let refused = || {
        usage(
            "the relate query must be a SELECT; relate reads records, it does not write files, attach databases, change settings, or load extensions",
        )
    };
    let statements = connection
        .statements(&bound.query)
        .map_err(|text| usage(&format!("the relate query did not parse: {text}")))?;
    if statements != 1 {
        return Err(usage(&format!(
            "the relate query is one SQL statement and this one holds {statements}; relate reads records, it does not run scripts"
        )));
    }
    // Only a SELECT parses as a subquery, and a statement that fails to
    // parse binds nothing: preparing `EXPORT DATABASE` itself makes its
    // folder.
    match connection.is_select(&format!(
        "SELECT * FROM ({}) AS thinkthen_kind",
        bound.query
    )) {
        Ok(true) => {}
        Ok(false) => return Err(refused()),
        Err(text) if text.starts_with("Parser Error") => return Err(refused()),
        Err(text) => return Err(failed(text)),
    }
    let capped = format!(
        "SELECT COLUMNS(*)::VARCHAR FROM ({}) AS thinkthen_capped LIMIT {}",
        bound.query,
        MOST_ROWS + 1
    );
    connection
        .execute("BEGIN TRANSACTION READ ONLY")
        .map_err(|text| {
            usage(&format!(
                "the relate query could not start its read-only transaction: {text}"
            ))
        })?;
    let found = in_search_path(connection, bound.search_path.as_deref(), || {
        plan_guard(connection, &capped, bound.holding)?;
        connection.rows(&capped).map_err(failed)
    });
    let _ = connection.execute("ROLLBACK");
    let _ = connection.execute("RESET search_path");
    found
}

fn in_search_path<T>(
    connection: Conn,
    path: Option<&str>,
    body: impl FnOnce() -> Result<T, String>,
) -> Result<T, String> {
    if let Some(path) = path {
        connection
            .execute(&format!("SET search_path = '{}'", path.replace('\'', "''")))
            .map_err(|text| usage(&format!("the relate query could not run under the calling session's search path {path}: {text}")))?;
    }
    body()
}

/// Refuse a plan that feeds more than `limit` estimated rows into a step
/// that holds its whole input before the `LIMIT` can stop it.
fn plan_guard(connection: Conn, capped: &str, limit: u64) -> Result<(), String> {
    let plan = connection
        .rows(&format!("EXPLAIN (FORMAT JSON) {capped}"))
        .map_err(|text| usage(&format!("the relate query failed: {text}")))?;
    let text = plan
        .rows
        .first()
        .and_then(|row| row.get(1).cloned().flatten())
        .ok_or_else(|| defect("the relate plan came back empty"))?;
    let plan: serde_json::Value = serde_json::from_str(&text)
        .map_err(|error| defect(&format!("the relate plan did not read as JSON: {error}")))?;
    match heaviest(&plan) {
        Some((step, rows)) if rows > limit => Err(usage(&format!(
            "the relate query feeds about {rows} rows into the {step} step before its LIMIT, and relate lets at most {limit} rows into a sorting, grouping, windowing, or joining step; filter the rows first or raise SET thinkthen_relate_holding_rows"
        ))),
        _ => Ok(()),
    }
}

/// The holding step with the largest estimated input anywhere in the plan.
fn heaviest(plan: &serde_json::Value) -> Option<(String, u64)> {
    let nodes: Vec<&serde_json::Value> = match plan {
        serde_json::Value::Array(nodes) => nodes.iter().collect(),
        node => vec![node],
    };
    let mut most: Option<(String, u64)> = None;
    for node in nodes {
        let name = node
            .get("name")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let children: Vec<&serde_json::Value> = node
            .get("children")
            .and_then(serde_json::Value::as_array)
            .map(|children| children.iter().collect())
            .unwrap_or_default();
        let held = if name.contains("DELIM_JOIN") {
            children.get(..).unwrap_or_default()
        } else if name.contains("JOIN") || name == "CROSS_PRODUCT" {
            children.get(1..).unwrap_or_default()
        } else if name == "CTE" {
            children.get(..1).unwrap_or_default()
        } else if matches!(
            name,
            "ORDER_BY"
                | "WINDOW"
                | "HASH_GROUP_BY"
                | "PERFECT_HASH_GROUP_BY"
                | "UNGROUPED_AGGREGATE"
        ) {
            children.get(..).unwrap_or_default()
        } else {
            &[]
        };
        let own = held.iter().filter_map(|child| estimate(child)).max();
        let below = children.iter().filter_map(|child| heaviest(child));
        for found in own
            .map(|rows| (name.to_owned(), rows))
            .into_iter()
            .chain(below)
        {
            if most.as_ref().is_none_or(|(_, rows)| found.1 > *rows) {
                most = Some(found);
            }
        }
    }
    most
}

/// A node's row estimate, or the largest below it. A `LIMIT` with no
/// estimate of its own bounds what is below it.
fn estimate(node: &serde_json::Value) -> Option<u64> {
    let own = node
        .pointer("/extra_info/Estimated Cardinality")
        .and_then(|value| match value {
            serde_json::Value::String(text) => text.trim().parse().ok(),
            other => other.as_u64(),
        });
    let name = node
        .get("name")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    if own.is_none() && name.ends_with("LIMIT") {
        return None;
    }
    own.or_else(|| {
        node.get("children")?
            .as_array()?
            .iter()
            .filter_map(estimate)
            .max()
    })
}

/// Run `body` while a timer stands ready to interrupt `connection` at `at`.
/// Answers the body's outcome and whether the timer fired.
fn under_limit<T>(connection: Conn, at: Option<Instant>, body: impl FnOnce() -> T) -> (T, bool) {
    let Some(at) = at else {
        return (body(), false);
    };
    let (done, finished) = std::sync::mpsc::channel::<()>();
    let timer = std::thread::spawn(move || {
        let left = at.saturating_duration_since(Instant::now());
        let fired = matches!(
            finished.recv_timeout(left),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout)
        );
        if fired {
            connection.interrupt();
        }
        fired
    });
    let outcome = body();
    let _ = done.send(());
    (outcome, timer.join().unwrap_or(false))
}

/// A caller-owned temporary table gets the ADR 0038 boundary sentence.
/// Another missing name keeps DuckDB's error and gains the connection rule.
fn boundary(files: &Files, message: String) -> String {
    let Some(name) = message
        .strip_prefix("thinkthen usage: the relate query failed: Catalog Error: Table with name ")
        .and_then(|rest| rest.split_once(" does not exist"))
        .map(|(name, _)| name)
        .map(|name| name.trim().trim_matches('"').to_owned())
    else {
        return message;
    };
    if name.is_empty() {
        return message;
    }
    if files.has_table("temp", &name) {
        return format!(
            "{}the relate query names the temporary table {name}, and the stable C API cannot run a query on the calling connection, so relate cannot see temporary tables; materialize it (CREATE TABLE ... AS SELECT) or run the query directly",
            prefix(thinkthen::ErrorKind::Local)
        );
    }
    let (line, details) = message.split_at(message.find('\n').unwrap_or(message.len()));
    format!(
        "{line}; relate reads only committed tables on its separate connection; if you created this table in an open transaction, commit it before retrying{details}"
    )
}

fn cancelled() -> String {
    format!(
        "{}the call was cancelled",
        prefix(thinkthen::ErrorKind::Cancelled)
    )
}

/// The refusal a relate earns when its limit runs out while it waits behind
/// another relate on the same database.
fn queue_limit(seconds: u64) -> String {
    deadline(&format!(
        "the relate query waited past its {seconds}-second limit in the queue behind another relate on this database and did not run; retry after that relate ends or raise SET thinkthen_relate_seconds (0 turns the limit off)"
    ))
}

fn deadline(message: &str) -> String {
    format!("{}{message}", prefix(thinkthen::ErrorKind::Deadline))
}

fn time_limit(seconds: u64) -> String {
    deadline(&format!(
        "the relate query ran past its {seconds}-second limit and was stopped; filter the rows first or raise SET thinkthen_relate_seconds (0 turns the limit off)"
    ))
}
