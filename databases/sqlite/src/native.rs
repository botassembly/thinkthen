//! Same-name tables retain ordinary answers and add typed receipt projections.
use crate::Failure;
use crate::many::Store;
use crate::receipt::{Cells, Receipt};
use crate::tables::{Scan, Table};
use rusqlite::ffi::sqlite3;
use rusqlite::types::{Value, ValueRef};
use rusqlite::vtab::Filters;
use std::borrow::Cow;
use std::collections::VecDeque;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

mod answers;
pub(crate) mod details;
mod execution;
mod input;
mod inputs;
mod run;

const FIELDS: &[&str] = &[
    "request_records",
    "projection",
    "request_question",
    "record_fields",
    "record_context_field",
    "record_context_schema",
    "record_options_field",
    "record_seed_spans_field",
    "record_examples_field",
    "incremental",
    "attempts",
    "cancelled",
    "request_paths",
    "request_unit",
    "request_window",
    "request_media",
    "request_format",
    "request_files_only",
    "top",
    "mode",
    "snippet_pieces",
    "relation_threshold",
    "seed_spans",
    "examples",
    "max_requests_total",
    "stage_context",
    "image_text",
];

#[derive(Debug)]
pub(crate) struct Native<T>(PhantomData<T>);

pub(crate) trait Verb: Table {
    const VERB: &'static str;
    const FRESH_ANSWER: bool = false;
    const STRICT_ANSWER: bool = false;
    const DETAILS: bool = false;
}

fn hidden<T: Table>() -> Result<Vec<String>, Failure> {
    let schema = T::schema()?;
    let text = schema
        .to_str()
        .map_err(|_| Failure::defect("a table schema is not text"))?;
    Ok(text
        .trim_end_matches(')')
        .split('(')
        .nth(1)
        .ok_or_else(|| Failure::defect("a table schema has no columns"))?
        .split(',')
        .skip(T::FIRST_HIDDEN)
        .map(|field| {
            field
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_owned()
        })
        .collect())
}

impl<T: Verb> Table for Native<T> {
    const NAME: &'static str = T::NAME;
    const SCHEMA: &'static CStr = T::SCHEMA;
    const FIRST_HIDDEN: usize = T::FIRST_HIDDEN;
    const COLUMNS: usize = T::COLUMNS + FIELDS.len();
    const REQUIRED: usize = 0;

    fn schema() -> Result<Cow<'static, CStr>, Failure> {
        if Self::COLUMNS + crate::receipt_generated::COLUMN_COUNT > 2000 {
            return Err(Failure::defect(
                "a receipt table exceeds SQLite's column limit",
            ));
        }
        let base = T::schema()?;
        let base = base
            .to_str()
            .map_err(|_| Failure::defect("a table schema is not text"))?;
        let columns = base
            .trim_end_matches(')')
            .split('(')
            .nth(1)
            .ok_or_else(|| Failure::defect("a table schema has no columns"))?
            .split(',')
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let visible = columns.iter().take(T::FIRST_HIDDEN).cloned();
        let receipt = crate::receipt::columns()
            .into_iter()
            .map(|(name, kind)| format!("{name} {kind} HIDDEN"));
        let arguments = columns.iter().skip(T::FIRST_HIDDEN).cloned();
        let extra = FIELDS.iter().map(|field| format!("{field} HIDDEN"));
        let text = visible
            .chain(arguments)
            .chain(extra)
            .chain(receipt)
            .collect::<Vec<_>>()
            .join(",");
        Ok(Cow::Owned(
            CString::new(format!("CREATE TABLE x({text})"))
                .map_err(|_| Failure::defect("a table schema contains a zero byte"))?,
        ))
    }

    fn rows(_: *mut sqlite3, _: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
        Err(Failure::defect("a native table lost its scan"))
    }

    fn scan(
        db: *mut sqlite3,
        mask: u64,
        filters: &Filters<'_>,
        store: &Mutex<Store>,
    ) -> Result<Scan, Failure> {
        let arguments = crate::tables::arguments::<Self>(mask, filters)?;
        let required = T::REQUIRED;
        if (0..required).any(|at| {
            mask & (1 << at) != 0
                && arguments
                    .get(at)
                    .is_some_and(|value| matches!(value, Value::Null))
        }) {
            return Ok(Scan {
                rows: Arc::new(Vec::new()),
                selected: None,
                projections: None,
            });
        }
        let legacy_count = T::COLUMNS - T::FIRST_HIDDEN;
        let extra = arguments.get(legacy_count..).unwrap_or_default();
        let legacy = extra.iter().all(|value| matches!(value, Value::Null));
        let mut names = hidden::<T>()?;
        names.extend(FIELDS.iter().map(|field| (*field).to_owned()));
        let mut args = Arguments {
            names,
            values: arguments,
            bound: mask,
        };
        if T::DETAILS {
            args = record_arguments(args, T::VERB)?;
        }
        if !T::DETAILS && args.supplied("request_records") {
            return Err(Failure::usage(
                "record lists require the function's _details form",
            ));
        }
        let selection = args.text("projection")?;
        let kind = selection.as_deref().unwrap_or_else(|| run::root(T::VERB));
        crate::receipt::projection(kind)?;
        let receipt = match run::prepare(db, T::VERB, &args, T::DETAILS) {
            Err(error)
                if matches!(
                    error.kind(),
                    thinkthen::ErrorKind::Cancelled
                        | thinkthen::ErrorKind::Deadline
                        | thinkthen::ErrorKind::Defect
                ) =>
            {
                return Err(error.into());
            }
            Err(error) if T::DETAILS => Arc::new(run::admission_details(T::VERB, error)?),
            Err(error) => Arc::new(run::admission(&error)?),
            Ok(invocation) => cached::<T>(db, invocation, selection.is_none(), store)?,
        };
        if T::STRICT_ANSWER && selection.is_none() && legacy {
            run::legacy_error(&receipt)?;
        }
        let mut rows = receipt.projected(kind, Self::COLUMNS)?;
        if T::DETAILS && selection.is_none() {
            rows = details::rows(&receipt, kind, Self::COLUMNS)?;
        }
        if selection.is_none() {
            if !T::DETAILS {
                answers::rows(T::VERB, &receipt, &mut rows)?;
            }
            let facts = receipt.projected("facts", Self::COLUMNS)?;
            let errors = receipt.projected("error", Self::COLUMNS)?;
            if rows.is_empty() && !T::DETAILS && !(T::STRICT_ANSWER && legacy) {
                rows.push(Cells::new());
            }
            if !T::DETAILS {
                overlay(&mut rows, &facts, &errors);
            }
        }
        if selection.is_some() {
            let facts = receipt.projected("facts", Self::COLUMNS)?;
            overlay(&mut rows, &facts, &[]);
        }
        if selection.is_none() && !matches!(args.value("lookup_key"), ValueRef::Null) {
            rows.retain(|row| {
                row.get(&0)
                    .is_some_and(|key| ValueRef::from(key) == args.value("lookup_key"))
            });
        }
        Ok(Scan {
            rows: Arc::new(vec![Vec::new(); rows.len()]),
            selected: None,
            projections: Some(Arc::new(rows)),
        })
    }
}

#[derive(Clone, Debug)]
pub(super) struct Arguments {
    names: Vec<String>,
    values: Vec<Value>,
    bound: u64,
}
impl Arguments {
    fn with_value(&self, name: &str, value: Value) -> Self {
        let mut args = self.clone();
        if let Some(at) = args.names.iter().position(|field| field == name) {
            if let Some(slot) = args.values.get_mut(at) {
                *slot = value;
            }
            args.bound |= 1 << at;
        }
        args
    }
    fn supplied(&self, name: &str) -> bool {
        self.names
            .iter()
            .position(|field| field == name)
            .is_some_and(|at| self.bound & (1 << at) != 0)
    }
    fn value(&self, name: &str) -> ValueRef<'_> {
        self.names
            .iter()
            .position(|field| field == name)
            .and_then(|at| self.values.get(at))
            .map_or(ValueRef::Null, ValueRef::from)
    }
    fn text(&self, name: &str) -> Result<Option<String>, Failure> {
        crate::question::text(self.value(name), name)
    }
    fn number(&self, name: &str) -> Result<Option<i64>, Failure> {
        match self.value(name) {
            ValueRef::Null => Ok(None),
            ValueRef::Integer(number) => Ok(Some(number)),
            _ => Err(Failure::usage(format!("{name} is a SQL integer"))),
        }
    }
    fn flag(&self, name: &str) -> Result<bool, Failure> {
        let supplied = self.supplied(name);
        if supplied && name != "files_only" && matches!(self.value(name), ValueRef::Null) {
            return Err(Failure::usage(format!(
                "{name} requires the SQL integer 0 or 1"
            )));
        }
        match self.number(name)? {
            None | Some(0) => Ok(false),
            Some(1) => Ok(true),
            _ => Err(Failure::usage(format!("{name} is the SQL integer 0 or 1"))),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Cache(VecDeque<(String, Arc<Receipt>)>);
impl Cache {
    fn get(&mut self, identity: &str) -> Option<Arc<Receipt>> {
        let at = self.0.iter().position(|(key, _)| key == identity)?;
        let entry = self.0.remove(at)?;
        let receipt = Arc::clone(&entry.1);
        self.0.push_front(entry);
        Some(receipt)
    }
    fn put(&mut self, identity: String, receipt: Arc<Receipt>) {
        self.0.retain(|(key, _)| key != &identity);
        self.0.push_front((identity, receipt));
        self.0.truncate(8);
    }
}

macro_rules! atomic {
    ($name:ident, $sql:literal, $verb:literal, $schema:literal, $visible:expr) => {
        #[derive(Debug)]
        pub(crate) struct $name;
        impl Table for $name {
            const NAME: &'static str = $sql;
            const SCHEMA: &'static CStr = $schema;
            const FIRST_HIDDEN: usize = $visible;
            const COLUMNS: usize = $visible + 3 + crate::named::COUNT;
            const REQUIRED: usize = 2;
            fn schema() -> Result<Cow<'static, CStr>, Failure> {
                crate::named::schema(Self::SCHEMA)
            }
            fn rows(_: *mut sqlite3, _: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
                Err(Failure::defect("an atomic table lost its native scan"))
            }
        }
        impl Verb for $name {
            const VERB: &'static str = $verb;
            const FRESH_ANSWER: bool = $visible == 5;
            const STRICT_ANSWER: bool = $visible == 5;
        }
    };
}
atomic!(Decide, "thinkthen_decide", "decide", c"CREATE TABLE x(value INTEGER, probability REAL, question HIDDEN, text HIDDEN, settings HIDDEN)", 2);
atomic!(
    Choose,
    "thinkthen_choose",
    "choose",
    c"CREATE TABLE x(value TEXT, probability REAL, question HIDDEN, text HIDDEN, settings HIDDEN)",
    2
);
atomic!(
    Tag,
    "thinkthen_tag",
    "tag",
    c"CREATE TABLE x(value TEXT, question HIDDEN, text HIDDEN, settings HIDDEN)",
    1
);
atomic!(
    Score,
    "thinkthen_score",
    "score",
    c"CREATE TABLE x(value REAL, question HIDDEN, text HIDDEN, settings HIDDEN)",
    1
);
atomic!(
    Find,
    "thinkthen_find",
    "find",
    c"CREATE TABLE x(value TEXT, question HIDDEN, text HIDDEN, settings HIDDEN)",
    1
);
atomic!(
    Annotate,
    "thinkthen_annotate",
    "annotate",
    c"CREATE TABLE x(value TEXT, question HIDDEN, text HIDDEN, settings HIDDEN)",
    1
);
impl Verb for crate::many::RankSetTable {
    const VERB: &'static str = "rank_set";
    const STRICT_ANSWER: bool = true;
}
impl Verb for crate::many::Rank {
    const STRICT_ANSWER: bool = true;
    const VERB: &'static str = "rank";
}
impl Verb for crate::source_filter::Filter {
    const STRICT_ANSWER: bool = true;
    const VERB: &'static str = "filter";
}
impl Verb for crate::tables::Recognizer {
    const STRICT_ANSWER: bool = true;
    const VERB: &'static str = "recognize";
}
impl Verb for crate::tables::Relater {
    const STRICT_ANSWER: bool = true;
    const VERB: &'static str = "relate";
}

fn overlay(rows: &mut [Cells], facts: &[Cells], errors: &[Cells]) {
    for held in facts.first().into_iter().chain(errors.first()) {
        for row in &mut *rows {
            row.extend(held.clone());
        }
    }
}

fn cached<T: Verb>(
    db: *mut sqlite3,
    invocation: run::Invocation,
    answer: bool,
    store: &Mutex<Store>,
) -> Result<Arc<Receipt>, Failure> {
    let identity = invocation.identity.clone();
    let held = if T::FRESH_ANSWER && answer {
        None
    } else {
        store
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .receipts
            .get(&identity)
    };
    if let Some(held) = held {
        return Ok(held);
    }
    let receipt = Arc::new(run::execute(db, invocation)?);
    store
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .receipts
        .put(identity, Arc::clone(&receipt));
    Ok(receipt)
}

fn record_arguments(mut args: Arguments, verb: &str) -> Result<Arguments, Failure> {
    if args.supplied("request_records") {
        return Ok(args);
    }
    for field in ["text", "body", "keyed_json", "paths", "query"] {
        if matches!(args.value(field), ValueRef::Blob(_)) {
            continue;
        }
        let Some(value) = args.text(field)? else {
            continue;
        };
        let Ok(serde_json::Value::Array(rows)) = serde_json::from_str::<serde_json::Value>(&value)
        else {
            continue;
        };
        if rows.iter().all(serde_json::Value::is_string)
            && (field == "paths" || (verb == "find" && field == "text"))
        {
            continue;
        }
        args = args.with_value("request_records", Value::Text(value));
        args = args.with_value(field, Value::Null);
        break;
    }
    Ok(args)
}
