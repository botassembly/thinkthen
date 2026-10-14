//! Same-name tables retain ordinary answers and add typed receipt projections.
use crate::many::Store;
use crate::receipt::Receipt;
use crate::tables::{Scan, Table};
use crate::{Failure, receipt_generated};
use rusqlite::ffi::sqlite3;
use rusqlite::types::{Value, ValueRef};
use rusqlite::vtab::Filters;
use std::borrow::Cow;
use std::collections::VecDeque;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

mod input;
mod run;

const FIELDS: &[&str] = &[
    "records",
    "projection",
    "request_question",
    "record_fields",
    "record_context_field",
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
];

#[derive(Debug)]
pub(crate) struct Native<T>(PhantomData<T>);

pub(crate) trait Verb: Table {
    const VERB: &'static str;
    const FRESH_ANSWER: bool = false;
    const STRICT_ANSWER: bool = false;
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
    const FIRST_HIDDEN: usize = T::FIRST_HIDDEN + receipt_generated::COLUMN_COUNT;
    const COLUMNS: usize = T::COLUMNS + receipt_generated::COLUMN_COUNT + FIELDS.len();
    const REQUIRED: usize = 0;

    fn schema() -> Result<Cow<'static, CStr>, Failure> {
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
            .map(|(name, kind)| format!("{name} {kind}"));
        let arguments = columns.iter().skip(T::FIRST_HIDDEN).cloned();
        let extra = FIELDS.iter().map(|field| format!("{field} HIDDEN"));
        let text = visible
            .chain(receipt)
            .chain(arguments)
            .chain(extra)
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
        if (0..T::REQUIRED).any(|at| {
            mask & (1 << at) != 0
                && arguments
                    .get(at)
                    .is_some_and(|value| matches!(value, Value::Null))
        }) {
            return Ok(Scan {
                rows: Arc::new(Vec::new()),
                selected: None,
            });
        }
        let legacy_count = T::COLUMNS - T::FIRST_HIDDEN;
        let extra = arguments.get(legacy_count..).unwrap_or_default();
        let legacy = extra.iter().all(|value| matches!(value, Value::Null));
        if extra.iter().all(|value| matches!(value, Value::Null)) && !run::always_native(T::VERB) {
            let scan = T::scan(db, mask, filters, store)?;
            let rows = scan
                .rows
                .iter()
                .map(|row| {
                    let mut row = row.clone();
                    row.resize(Self::FIRST_HIDDEN, Value::Null);
                    row
                })
                .collect();
            return Ok(Scan {
                rows: Arc::new(rows),
                selected: scan.selected,
            });
        }
        let mut names = hidden::<T>()?;
        names.extend(FIELDS.iter().map(|field| (*field).to_owned()));
        let args = Arguments {
            names,
            values: arguments,
            bound: mask,
        };
        let selection = args.text("projection")?;
        let kind = selection.as_deref().unwrap_or_else(|| run::root(T::VERB));
        crate::receipt::projection(kind)?;
        let identity = input::identity(T::VERB, &args)?;
        let receipt = if T::FRESH_ANSWER && selection.is_none() {
            None
        } else {
            store
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .receipts
                .get(&identity)
        };
        let receipt = if let Some(receipt) = receipt {
            receipt
        } else {
            let receipt = Arc::new(run::execute(db, T::VERB, &args)?);
            store
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .receipts
                .put(identity, Arc::clone(&receipt));
            receipt
        };
        if T::STRICT_ANSWER && selection.is_none() && legacy {
            run::legacy_error(&receipt)?;
        }
        let mut rows = receipt.projected(kind, T::FIRST_HIDDEN)?;
        if selection.is_none() {
            run::answers(T::VERB, &receipt, &mut rows)?;
            let facts = receipt.projected("facts", T::FIRST_HIDDEN)?;
            let errors = receipt.projected("error", T::FIRST_HIDDEN)?;
            if rows.is_empty()
                && !(T::STRICT_ANSWER && legacy)
                && (!facts.is_empty() || !errors.is_empty())
            {
                rows.push(vec![Value::Null; Self::FIRST_HIDDEN]);
            }
            overlay(&mut rows, &facts, &errors);
        }
        Ok(Scan {
            rows: Arc::new(rows),
            selected: None,
        })
    }
}

#[derive(Debug)]
pub(super) struct Arguments {
    names: Vec<String>,
    values: Vec<Value>,
    bound: u64,
}
impl Arguments {
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
        let supplied = self
            .names
            .iter()
            .position(|field| field == name)
            .is_some_and(|at| self.bound & (1 << at) != 0);
        if supplied && matches!(self.value(name), ValueRef::Null) {
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
atomic!(Details, "thinkthen_details", "details", c"CREATE TABLE x(value, confidence REAL, answer_id TEXT, answer_model TEXT, requests_sent INTEGER, question HIDDEN, text HIDDEN, settings HIDDEN)", 5);

#[derive(Debug)]
pub(crate) struct DetailsImages;
impl Table for DetailsImages {
    const NAME: &'static str = "thinkthen_details_images";
    const SCHEMA: &'static CStr = c"CREATE TABLE x(value, confidence REAL, answer_id TEXT, answer_model TEXT, requests_sent INTEGER, question HIDDEN, images HIDDEN, image_text HIDDEN, settings HIDDEN)";
    const FIRST_HIDDEN: usize = 5;
    const COLUMNS: usize = 9 + crate::named::COUNT;
    const REQUIRED: usize = 2;
    fn schema() -> Result<Cow<'static, CStr>, Failure> {
        crate::named::schema(Self::SCHEMA)
    }
    fn rows(_: *mut sqlite3, _: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
        Err(Failure::defect("image details lost their native scan"))
    }
}
impl Verb for DetailsImages {
    const VERB: &'static str = "details_images";
    const FRESH_ANSWER: bool = true;
    const STRICT_ANSWER: bool = true;
}
impl Verb for crate::many::RankSetTable {
    const VERB: &'static str = "rank_set";
    const STRICT_ANSWER: bool = true;
}
impl Verb for crate::many::Rank {
    const VERB: &'static str = "rank";
}
impl Verb for crate::source_filter::Filter {
    const VERB: &'static str = "filter";
}
impl Verb for crate::tables::Recognizer {
    const VERB: &'static str = "recognize";
}
impl Verb for crate::tables::Relater {
    const VERB: &'static str = "relate";
}

fn overlay(rows: &mut [Vec<Value>], facts: &[Vec<Value>], errors: &[Vec<Value>]) {
    let values = facts.first().into_iter().chain(errors.first());
    for held in values {
        for row in &mut *rows {
            for (slot, value) in row
                .iter_mut()
                .zip(held)
                .filter(|(_, value)| !matches!(value, Value::Null))
            {
                *slot = value.clone();
            }
        }
    }
}
