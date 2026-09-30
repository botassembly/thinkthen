//! Every `unsafe` block of the binding (ADR 0047 item 3): the entry point,
//! the host's API table, the library pin, the connection handle, the
//! virtual-table glue, and the exit hook that flushes the usage totals.

use std::borrow::Cow;
use std::ffi::{CStr, c_char, c_int};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use rusqlite::ffi::{self, sqlite3};
use rusqlite::functions::Context;
use rusqlite::types::Value;
use rusqlite::vtab::{
    Context as Cell, Filters, IndexInfo, Module, VTab, VTabConfig, VTabConnection, VTabCursor,
};

use crate::many::{ChooseMany, DecideMany, ScoreMany, Store, TagMany};
use crate::tables::{self, Recognizer, Relater, Table};
use crate::{budget, guard, scalars};

/// The host's `sqlite3_api_routines`, extended past the 3.34 bindings of
/// `libsqlite3-sys` to the `is_interrupted` field SQLite 3.41 added. Every
/// field of the tail is a function pointer, so an opaque pointer keeps each
/// offset exact. Only `is_interrupted` is read, after the floor check.
#[repr(C)]
struct ApiRoutines {
    base: ffi::sqlite3_api_routines,
    tail: [*const (); 13],
    is_interrupted: Option<unsafe extern "C" fn(*mut sqlite3) -> c_int>,
}

/// The host's own `is_interrupted`, read from its API table at load.
static IS_INTERRUPTED: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());

/// The host's API table, held between the entry and `init`.
static API_TABLE: AtomicPtr<ffi::sqlite3_api_routines> = AtomicPtr::new(std::ptr::null_mut());

/// The support floor. Below 3.50.0 a CHECK constraint in an untrusted
/// database reaches a volatile direct-only function: SQLite marked a call
/// as from DDL only in its resolver's deterministic branch until 3.50.0.
const FLOOR: c_int = 3_050_000;

/// The refusal for a host below the floor, or `None` for a host at or above it.
fn version_refusal(host: c_int) -> Option<String> {
    (host < FLOOR).then(|| {
        format!(
            "thinkthen needs SQLite 3.50.0 or newer (below 3.50.0 a CHECK constraint in an untrusted database reaches the functions, so a schema could spend money or read files); this host is {}.{}.{} ({host})",
            host / 1_000_000,
            host / 1_000 % 1_000,
            host % 1_000
        )
    })
}

/// Flush the process engine's usage totals at exit (ADR 0113), registered
/// once when the first engine is built. The C library also runs a library's
/// `atexit` hook when that library is unloaded, and a reload registers it again.
pub(crate) fn flush_usage_at_exit() {
    static REGISTERED: AtomicBool = AtomicBool::new(false);
    if REGISTERED.swap(true, Ordering::AcqRel) {
        return;
    }
    // SAFETY: the hook is a plain function of this library, which the C
    // library runs at exit or when it unloads the library.
    let _registered = unsafe { libc::atexit(flush_usage) };
}

/// The exit hook. A panic stays inside `thinkthen::contained`.
extern "C" fn flush_usage() {
    let _flushed = thinkthen::contained(crate::settings::finish_usage);
}

/// Whether SQLite has interrupted the connection. Read on the calling thread only.
pub(crate) fn interrupted(db: *mut sqlite3) -> bool {
    let check = IS_INTERRUPTED.load(Ordering::Acquire);
    if db.is_null() || check.is_null() {
        return false;
    }
    // SAFETY: the pointer is the host's own `is_interrupted`, stored at load
    // after the floor check, and `db` is the live connection of this call.
    let check: unsafe extern "C" fn(*mut sqlite3) -> c_int = unsafe { std::mem::transmute(check) };
    // SAFETY: as above.
    unsafe { check(db) != 0 }
}

/// The connection one function call runs on.
pub(crate) fn handle_of(context: &Context<'_>) -> *mut sqlite3 {
    // SAFETY: the context belongs to the live call. The handle is read, not kept past it.
    unsafe {
        context
            .get_connection()
            .map_or(std::ptr::null_mut(), |connection| connection.handle())
    }
}

/// A connection over a live handle that does not close it when dropped.
pub(crate) fn connection(db: *mut sqlite3) -> rusqlite::Result<Connection> {
    if db.is_null() {
        return Err(rusqlite::Error::InvalidQuery);
    }
    // SAFETY: `db` is the handle of the connection running this virtual
    // table, alive for the whole callback, and `from_handle` never closes it.
    unsafe { Connection::from_handle(db) }
}

/// Keep this library mapped for the life of the process, so a detached
/// worker's code outlives the connection that loaded it.
fn pin() {
    let mut info: libc::Dl_info = libc::Dl_info {
        dli_fname: std::ptr::null(),
        dli_fbase: std::ptr::null_mut(),
        dli_sname: std::ptr::null(),
        dli_saddr: std::ptr::null_mut(),
    };
    let address = sqlite3_thinkthen_init as *const libc::c_void;
    // SAFETY: `dladdr` fills `info` for an address inside a loaded image.
    if unsafe { libc::dladdr(address, &raw mut info) } == 0 || info.dli_fname.is_null() {
        return;
    }
    // SAFETY: a name the loader owns; NOLOAD only reopens what is loaded,
    // and the handle is never closed.
    unsafe {
        libc::dlopen(
            info.dli_fname,
            libc::RTLD_NOW | libc::RTLD_NOLOAD | libc::RTLD_NODELETE,
        )
    };
}

fn init(connection: Connection) -> rusqlite::Result<bool> {
    // SAFETY: the loadable API is initialized; this reads the host's version.
    let host = unsafe { ffi::sqlite3_libversion_number() };
    if let Some(refusal) = version_refusal(host) {
        return Err(rusqlite::Error::SqliteFailure(
            ffi::Error::new(ffi::SQLITE_ERROR),
            Some(refusal),
        ));
    }
    let table = API_TABLE.load(Ordering::Acquire).cast::<ApiRoutines>();
    // SAFETY: the pointer is the host's own table, and the floor check proves
    // the host is 3.41 or newer, so the field exists.
    if let Some(check) = unsafe { table.as_ref() }.and_then(|table| table.is_interrupted) {
        IS_INTERRUPTED.store(check as *mut (), Ordering::Release);
    }
    scalars::register(&connection)?;
    budget::register(&connection)?;
    let store = Arc::new(Mutex::new(Store::default()));
    connection.create_module(c"thinkthen_recognize", &RECOGNIZE, Some(Arc::clone(&store)))?;
    connection.create_module(c"thinkthen_relate", &RELATE, Some(Arc::clone(&store)))?;
    connection.create_module(
        c"thinkthen_decide_many",
        &DECIDE_MANY,
        Some(Arc::clone(&store)),
    )?;
    connection.create_module(
        c"thinkthen_choose_many",
        &CHOOSE_MANY,
        Some(Arc::clone(&store)),
    )?;
    connection.create_module(
        c"thinkthen_score_many",
        &SCORE_MANY,
        Some(Arc::clone(&store)),
    )?;
    connection.create_module(c"thinkthen_tag_many", &TAG_MANY, Some(store))?;
    pin();
    Ok(false)
}

/// The entry point SQLite derives from the file name `libthinkthen0.so`.
///
/// # Safety
///
/// Only SQLite calls this, at extension load, with its own three pointers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_thinkthen_init(
    db: *mut sqlite3,
    message: *mut *mut c_char,
    api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    API_TABLE.store(api, Ordering::Release);
    let loaded = guard("load", || {
        // SAFETY: the host's pointers, passed on as `extension_init2` asks.
        Ok(unsafe { Connection::extension_init2(db, message, api, init) })
    });
    match loaded {
        Ok(code) => code,
        Err(failure) => {
            // SAFETY: SQLite owns this message and frees the allocation.
            unsafe { rusqlite::to_sqlite_error(&rusqlite::Error::from(failure), message) }
        }
    }
}

const RECOGNIZE: Module<'static, Tab<Recognizer>> = Module::eponymous_only_module();
const RELATE: Module<'static, Tab<Relater>> = Module::eponymous_only_module();
const DECIDE_MANY: Module<'static, Tab<DecideMany>> = Module::eponymous_only_module();
const CHOOSE_MANY: Module<'static, Tab<ChooseMany>> = Module::eponymous_only_module();
const SCORE_MANY: Module<'static, Tab<ScoreMany>> = Module::eponymous_only_module();
const TAG_MANY: Module<'static, Tab<TagMany>> = Module::eponymous_only_module();

/// One table-valued function's virtual table.
#[repr(C)]
#[derive(Debug)]
struct Tab<T> {
    base: ffi::sqlite3_vtab,
    db: *mut sqlite3,
    store: Arc<Mutex<Store>>,
    kind: PhantomData<T>,
}

/// One scan of a table-valued function: the rows of one call.
#[repr(C)]
#[derive(Debug)]
struct Cursor<T> {
    base: ffi::sqlite3_vtab_cursor,
    db: *mut sqlite3,
    store: Arc<Mutex<Store>>,
    rows: Arc<Vec<Vec<Value>>>,
    selected: Option<usize>,
    row: usize,
    kind: PhantomData<T>,
}

// SAFETY: `Tab` is `repr(C)` with the base first, and its callbacks only
// register a direct-only, eponymous table and answer from owned rows.
unsafe impl<'vtab, T: Table + 'static> VTab<'vtab> for Tab<T> {
    type Aux = Arc<Mutex<Store>>;
    type Cursor = Cursor<T>;

    fn connect(
        db: &mut VTabConnection,
        store: Option<&Self::Aux>,
        _: &[u8],
        _: &[u8],
        _: &[u8],
        _: &[&[u8]],
    ) -> rusqlite::Result<(Cow<'static, CStr>, Self)> {
        db.config(VTabConfig::DirectOnly)?;
        // SAFETY: the handle belongs to this connection and outlives the table.
        let handle = unsafe { db.handle() };
        Ok((
            Cow::Borrowed(T::SCHEMA),
            Self {
                base: ffi::sqlite3_vtab::default(),
                db: handle,
                store: Arc::clone(store.ok_or(rusqlite::Error::InvalidQuery)?),
                kind: PhantomData,
            },
        ))
    }

    fn best_index(&self, info: &mut IndexInfo) -> rusqlite::Result<bool> {
        Ok(guard(T::NAME, || tables::plan::<T>(info))?)
    }

    fn open(&'vtab mut self) -> rusqlite::Result<Cursor<T>> {
        Ok(Cursor {
            base: ffi::sqlite3_vtab_cursor::default(),
            db: self.db,
            store: Arc::clone(&self.store),
            rows: Arc::new(Vec::new()),
            selected: None,
            row: 0,
            kind: PhantomData,
        })
    }
}

// SAFETY: `Cursor` is `repr(C)` with the base first, and it serves owned rows.
unsafe impl<T: Table> VTabCursor for Cursor<T> {
    fn filter(
        &mut self,
        mask: c_int,
        _: Option<&str>,
        filters: &Filters<'_>,
    ) -> rusqlite::Result<()> {
        let db = self.db;
        let scan = guard(T::NAME, || T::scan(db, mask, filters, &self.store))?;
        self.rows = scan.rows;
        self.selected = scan.selected;
        self.row = 0;
        Ok(())
    }

    fn next(&mut self) -> rusqlite::Result<()> {
        self.row += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        if let Some(at) = self.selected {
            self.row > 0 || at >= self.rows.len()
        } else {
            self.row >= self.rows.len()
        }
    }

    fn column(&self, cell: &mut Cell, at: c_int) -> rusqlite::Result<()> {
        let value = self
            .rows
            .get(self.selected.unwrap_or(self.row))
            .and_then(|row| usize::try_from(at).ok().and_then(|at| row.get(at)))
            .unwrap_or(&Value::Null);
        cell.set_result(value)
    }

    fn rowid(&self) -> rusqlite::Result<i64> {
        Ok(i64::try_from(self.row).unwrap_or(i64::MAX))
    }
}

#[cfg(test)]
mod tests {
    use super::version_refusal;

    /// R2-1: an old host is refused by name, and the floor passes.
    #[test]
    fn a_host_below_the_floor_is_refused_by_name() {
        assert_eq!(
            version_refusal(3_045_001).as_deref(),
            Some(
                "thinkthen needs SQLite 3.50.0 or newer (below 3.50.0 a CHECK constraint in an untrusted database reaches the functions, so a schema could spend money or read files); this host is 3.45.1 (3045001)"
            )
        );
        assert!(version_refusal(3_049_000).is_some());
        assert!(version_refusal(3_050_000).is_none());
    }
}
