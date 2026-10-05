//! A direct-only, demand-driven cursor over the shared native reader.

use std::borrow::Cow;
use std::ffi::{CStr, c_int};

use rusqlite::Connection;
use rusqlite::ffi;
use rusqlite::types::Value;
use rusqlite::vtab::{
    Context, Filters, IndexConstraintOp, IndexInfo, Module, VTab, VTabConfig, VTabConnection,
    VTabCursor,
};
use thinkthen::SourceRecords;

use crate::{Failure, files, guard};

pub(super) fn register(connection: &Connection) -> rusqlite::Result<()> {
    const MODULE: Module<'static, ReaderTable> = Module::eponymous_only_module();
    connection.create_module(c"thinkthen_read_files", &MODULE, None::<()>)?;
    files::register_span(connection)
}

#[repr(C)]
#[derive(Debug)]
struct ReaderTable {
    base: ffi::sqlite3_vtab,
}

#[repr(C)]
#[derive(Debug)]
struct ReaderCursor {
    base: ffi::sqlite3_vtab_cursor,
    source: Option<SourceRecords>,
    row: Option<Vec<Value>>,
    ordinal: i64,
}

// SAFETY: repr(C), SQLite base first, and connection-owned eponymous callbacks.
unsafe impl<'vtab> VTab<'vtab> for ReaderTable {
    type Aux = ();
    type Cursor = ReaderCursor;

    fn connect(
        db: &mut VTabConnection,
        _: Option<&()>,
        _: &[u8],
        _: &[u8],
        _: &[u8],
        _: &[&[u8]],
    ) -> rusqlite::Result<(Cow<'static, CStr>, Self)> {
        // File access remains direct-only even when judgments use trusted loading.
        db.config(VTabConfig::DirectOnly)?;
        Ok((Cow::Borrowed(c"CREATE TABLE x(ordinal INTEGER, record TEXT, file TEXT, first_line INTEGER, last_line INTEGER, path HIDDEN, reader_options HIDDEN)"),
            Self { base: ffi::sqlite3_vtab::default() }))
    }

    fn best_index(&self, info: &mut IndexInfo) -> rusqlite::Result<bool> {
        let mut bound = [None, None];
        for (at, constraint) in info.constraints().enumerate() {
            if constraint.is_usable()
                && constraint.operator() == IndexConstraintOp::SQLITE_INDEX_CONSTRAINT_EQ
                && let Some(slot) = usize::try_from(constraint.column())
                    .ok()
                    .and_then(|column| column.checked_sub(5))
                    .and_then(|column| bound.get_mut(column))
            {
                *slot = Some(at);
            }
        }
        if bound.first().is_none_or(Option::is_none) {
            return Ok(false);
        }
        let mut place = 0;
        let mut mask = 0;
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

    fn open(&'vtab mut self) -> rusqlite::Result<Self::Cursor> {
        Ok(ReaderCursor {
            base: ffi::sqlite3_vtab_cursor::default(),
            source: None,
            row: None,
            ordinal: 0,
        })
    }
}

impl ReaderCursor {
    fn advance(&mut self) -> Result<(), Failure> {
        self.row = None;
        if let Some(record) = self.source.as_mut().and_then(Iterator::next) {
            let record = record?;
            self.ordinal = self
                .ordinal
                .checked_add(1)
                .ok_or_else(|| Failure::defect("source ordinal is too large"))?;
            self.row = Some(files::values(self.ordinal, record)?);
        }
        Ok(())
    }
}

// SAFETY: repr(C), SQLite base first; each scan owns its reader and current row.
unsafe impl VTabCursor for ReaderCursor {
    fn filter(
        &mut self,
        mask: c_int,
        _: Option<&str>,
        arguments: &Filters<'_>,
    ) -> rusqlite::Result<()> {
        self.source = None;
        self.row = None;
        self.ordinal = 0;
        Ok(guard("thinkthen_read_files", || {
            let path = arguments
                .get::<Value>(0)
                .map_err(|_| Failure::usage("source path must be text"))?;
            let options = if mask & 2 != 0 {
                arguments
                    .get::<Value>(1)
                    .map_err(|_| Failure::usage("reader options must be text"))?
            } else {
                Value::Null
            };
            self.source = Some(files::selection(&path, &options)?);
            self.advance()
        })?)
    }

    fn next(&mut self) -> rusqlite::Result<()> {
        Ok(guard("thinkthen_read_files", || self.advance())?)
    }

    fn eof(&self) -> bool {
        self.row.is_none()
    }

    fn column(&self, context: &mut Context, at: c_int) -> rusqlite::Result<()> {
        context.set_result(
            self.row
                .as_ref()
                .and_then(|row| usize::try_from(at).ok().and_then(|at| row.get(at)))
                .unwrap_or(&Value::Null),
        )
    }

    fn rowid(&self) -> rusqlite::Result<i64> {
        Ok(self.ordinal)
    }
}
