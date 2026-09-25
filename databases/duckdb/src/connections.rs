//! The kept connections: one per loaded database, found for a caller through
//! an identity no SQL can forge, and released only when no bound relate
//! still holds one (ticket 0118 decisions 1 and 2, ADR 0038).
//!
//! A table function cannot run SQL on its caller's connection, so LOAD opens
//! one kept connection per database for relate's query. Two databases in one
//! process share these statics. Each kept connection therefore attaches an
//! in-memory probe database named from 128 random bits, with a marker table
//! named the same way. A caller matches an entry only when its own context
//! resolves the probe name and finds the marker inside it. The registry never
//! routes by a database's name. A database that cannot attach a probe, such
//! as a read-only one, is never routed to.
//!
//! The reaper releases a kept connection once its database has no other
//! connection, so a closed file database drops its lock. A counted guard
//! keeps a bound relate's connection alive across any reaper pass (R3-1).
//! The reaper's sleep doubles while nothing is released, up to 2 s (R3-23).

use std::io::Read;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use libduckdb_sys::duckdb_database;

use crate::errors::{defect, usage};
use crate::questions::Files;

mod ffi;

pub(crate) use ffi::{Conn, Rows};

/// One database's shared state.
#[derive(Debug, Default)]
struct State {
    /// Serializes every query on the kept connection: relate's and the
    /// reaper's. One gate per database, so one database's relate never
    /// waits on another's.
    gate: Mutex<()>,
    /// Live guards.
    users: AtomicUsize,
    /// The reaper found this database alone while a guard held it.
    retired: AtomicBool,
    /// A relate query is running on the kept connection now.
    busy: AtomicBool,
}

/// A counted handle to one database's kept connection.
#[derive(Debug)]
pub(crate) struct Kept {
    connection: Conn,
    id: u64,
    state: Arc<State>,
}

impl Kept {
    pub(crate) const fn connection(&self) -> Conn {
        self.connection
    }

    /// Whether this callback runs inside a query the kept connection is
    /// itself running: a relate nested in a relate query.
    pub(crate) fn is_caller(&self, files: &Files) -> bool {
        files.connection_id() == self.id
    }

    /// Wait for this database's gate. `stop` says when to give up.
    pub(crate) fn gate(
        &self,
        stop: impl Fn() -> Option<String>,
    ) -> Result<MutexGuard<'_, ()>, String> {
        loop {
            match self.state.gate.try_lock() {
                Ok(held) => return Ok(held),
                Err(std::sync::TryLockError::Poisoned(held)) => return Ok(held.into_inner()),
                Err(std::sync::TryLockError::WouldBlock) => {}
            }
            if let Some(refusal) = stop() {
                return Err(refusal);
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    /// Mark the kept connection busy for the SIGINT bridge until the
    /// returned value drops.
    pub(crate) fn busy(&self) -> Busy<'_> {
        crate::signal::start_bridge(interrupt_busy);
        self.state.busy.store(true, Ordering::Release);
        Busy(self)
    }
}

/// The kept connection is running a query while this lives.
#[derive(Debug)]
pub(crate) struct Busy<'a>(&'a Kept);

impl Drop for Busy<'_> {
    fn drop(&mut self) {
        self.0.state.busy.store(false, Ordering::Release);
    }
}

impl Drop for Kept {
    fn drop(&mut self) {
        self.state.users.fetch_sub(1, Ordering::AcqRel);
        if self.state.retired.load(Ordering::Acquire) {
            POKE.notify_all();
        }
    }
}

/// A probe database's name and its marker table's name.
type Probe = (String, String);

/// One registry entry.
#[derive(Debug)]
struct Entry {
    /// The probe database's name and its marker table's name.
    probe: Option<Probe>,
    connection: Conn,
    id: u64,
    state: Arc<State>,
}

impl Entry {
    fn guard(&self) -> Kept {
        self.state.users.fetch_add(1, Ordering::AcqRel);
        Kept {
            connection: self.connection,
            id: self.id,
            state: Arc::clone(&self.state),
        }
    }
}

static KEPT: Mutex<Vec<Entry>> = Mutex::new(Vec::new());
static REAPER: AtomicBool = AtomicBool::new(false);
static SLEEP_MS: AtomicU64 = AtomicU64::new(200);
static CURSOR: AtomicUsize = AtomicUsize::new(0);
static TICK: Mutex<()> = Mutex::new(());
/// Set once the process begins exiting, so the reaper and the bridge stop
/// touching connections the host tears down underneath them. `atexit`
/// runs before static destruction, so the flag lands first.
static SHUTDOWN: AtomicBool = AtomicBool::new(false);

extern "C" fn stop_at_exit() {
    SHUTDOWN.store(true, Ordering::SeqCst);
}
static POKE: Condvar = Condvar::new();

/// How many entries one reaper pass reads, so a pass stays bounded.
const BUDGET: usize = 8;

fn kept() -> MutexGuard<'static, Vec<Entry>> {
    KEPT.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Open the loading database's kept connection and attach its probe.
pub(crate) fn register(database: duckdb_database) -> Result<(), String> {
    let connection = Conn::open(database).map_err(|text| defect(&text))?;
    let probe = attach_probe(connection).ok();
    let id = connection.id();
    kept().push(Entry {
        probe,
        connection,
        id,
        state: Arc::default(),
    });
    SLEEP_MS.store(200, Ordering::Release);
    spawn_reaper();
    Ok(())
}

/// 128 bits from `/dev/urandom` as 32 hex characters (R5-26).
fn random_hex() -> Result<String, String> {
    let mut bytes = [0_u8; 16];
    std::fs::File::open("/dev/urandom")
        .and_then(|mut source| source.read_exact(&mut bytes))
        .map_err(|error| format!("the OS random source did not answer: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn attach_probe(connection: Conn) -> Result<Probe, String> {
    let probe = format!("thinkthen_instance_{}", random_hex()?);
    let marker = format!("thinkthen_marker_{}", random_hex()?);
    connection.execute(&format!("ATTACH ':memory:' AS {probe}"))?;
    if let Err(text) = connection.execute(&format!("CREATE TABLE {probe}.{marker}(i INTEGER)")) {
        let _ = connection.execute(&format!("DETACH {probe}"));
        return Err(text);
    }
    Ok((probe, marker))
}

/// The kept connection of the caller's own database, as a counted guard.
pub(crate) fn for_caller(files: &Files) -> Result<Kept, String> {
    let candidates: Vec<(Kept, Probe)> = kept()
        .iter()
        .filter(|entry| !entry.state.retired.load(Ordering::Acquire))
        .filter_map(|entry| Some((entry.guard(), entry.probe.clone()?)))
        .collect();
    let mut hits: Vec<Kept> = candidates
        .into_iter()
        .filter(|(_, (probe, marker))| files.has_table(probe, marker))
        .map(|(guard, _)| guard)
        .collect();
    match (hits.pop(), hits.is_empty()) {
        (Some(hit), true) => Ok(hit),
        (Some(_), false) => Err(defect(
            "two loaded databases answered the caller's identity probe",
        )),
        (None, _) => Err(usage(
            "this connection's database answers no loaded identity probe, so relate cannot find its own connection; LOAD the extension again on a writable database, since a read-only database cannot carry a probe and a released one lost it",
        )),
    }
}

/// Interrupt every kept connection running a relate query. The SIGINT
/// bridge thread calls this, never the handler.
fn interrupt_busy() {
    if SHUTDOWN.load(Ordering::SeqCst) {
        return;
    }
    // The registry stays locked across each interrupt, so the reaper cannot
    // close a connection between the read and the interrupt.
    for entry in kept().iter() {
        if entry.state.busy.load(Ordering::Acquire) {
            entry.connection.interrupt();
        }
    }
}

fn spawn_reaper() {
    if REAPER.swap(true, Ordering::SeqCst) {
        return;
    }
    ffi::at_exit(stop_at_exit);
    let spawned = std::thread::Builder::new()
        .name("thinkthen-reaper".to_owned())
        .spawn(|| while tick() {});
    if spawned.is_err() {
        REAPER.store(false, Ordering::SeqCst);
    }
}

/// One reaper wait and pass. False once the process is exiting, which
/// ends the thread before any pass touches a closing database.
fn tick() -> bool {
    let sleep = Duration::from_millis(SLEEP_MS.load(Ordering::Acquire));
    let clock = TICK.lock().unwrap_or_else(PoisonError::into_inner);
    drop(
        POKE.wait_timeout(clock, sleep)
            .unwrap_or_else(PoisonError::into_inner),
    );
    if SHUTDOWN.load(Ordering::SeqCst) {
        return false;
    }
    let next = if reap(false) > 0 {
        200
    } else {
        SLEEP_MS
            .load(Ordering::Acquire)
            .saturating_mul(2)
            .min(2_000)
    };
    SLEEP_MS.store(next, Ordering::Release);
    true
}

/// One pass over a bounded slice of the registry. A database with no other
/// connection is released, or marked retired while a guard holds it.
/// `force` treats every database as alone: the `test-hooks` pass for R3-1.
pub(crate) fn reap(force: bool) -> usize {
    let slice: Vec<(Conn, Arc<State>)> = {
        let held = kept();
        let start = CURSOR.fetch_add(BUDGET, Ordering::AcqRel);
        (0..BUDGET.min(held.len()))
            .filter_map(|offset| held.get((start + offset) % held.len()))
            .map(|entry| (entry.connection, Arc::clone(&entry.state)))
            .collect()
    };
    let mut released = 0;
    for (connection, state) in slice {
        let alone = force
            || state.gate.try_lock().is_ok_and(|_held| {
                connection
                    .rows("SELECT COLUMNS(*)::VARCHAR FROM duckdb_connection_count()")
                    .ok()
                    .and_then(|found| found.rows.first()?.first()?.clone())
                    .and_then(|count| count.parse::<u64>().ok())
                    .is_some_and(|count| count <= 1)
            });
        if !alone {
            state.retired.store(false, Ordering::Release);
            continue;
        }
        state.retired.store(true, Ordering::Release);
        let mut held = kept();
        if state.users.load(Ordering::Acquire) > 0 {
            continue;
        }
        if let Some(at) = held
            .iter()
            .position(|entry| Arc::ptr_eq(&entry.state, &state))
        {
            let entry = held.remove(at);
            if let Some((probe, marker)) = &entry.probe {
                let _ = connection.execute(&format!(
                    "DROP TABLE IF EXISTS {probe}.{marker}; DETACH {probe}"
                ));
            }
            connection.close();
            released += 1;
        }
    }
    released
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    /// Once `atexit` has run, the reaper's next tick ends its thread before
    /// any pass. Dropping the flag check keeps the thread reaping.
    #[test]
    fn the_reaper_stops_once_the_process_exits() {
        super::SLEEP_MS.store(1, Ordering::Release);
        super::stop_at_exit();
        assert!(!super::tick());
        super::SHUTDOWN.store(false, Ordering::SeqCst);
    }

    /// R5-26: identity names are 32 lowercase hex characters, and a
    /// thousand never repeat.
    #[test]
    fn identity_names_are_random_hex() -> Result<(), String> {
        let names = (0..1_000)
            .map(|_| super::random_hex())
            .collect::<Result<std::collections::HashSet<String>, String>>()?;
        assert_eq!(names.len(), 1_000);
        assert!(names.iter().all(|name| {
            name.len() == 32
                && name
                    .chars()
                    .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
        }));
        Ok(())
    }
}
