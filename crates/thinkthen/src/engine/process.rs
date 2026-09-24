//! Runtime state that belongs to one process, replaced without a lock in a
//! forked child.
//!
//! A forked child inherits every lock in the state its parent built, and a
//! lock held by a parent thread stays held forever, because that thread does
//! not exist in the child. So a caller compares its process ID with the owner
//! before it looks at the state. On a mismatch it builds fresh state, swaps it
//! in, leaks the inherited state untouched, and only then names itself owner.
//! Until fresh state exists, the child uses atomics alone.

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use arc_swap::ArcSwapOption;

/// One piece of process-owned state and the process that built it.
///
/// Process ID 0 names no user process, so it marks an empty owner and an idle
/// rebuild.
#[derive(Debug)]
pub(crate) struct Guarded<T> {
    owner: AtomicU32,
    rebuilding: AtomicU32,
    slot: ArcSwapOption<T>,
}

impl<T> Guarded<T> {
    /// No state yet. The first caller builds it as a fork's child would.
    pub(crate) const fn empty() -> Self {
        Self {
            owner: AtomicU32::new(0),
            rebuilding: AtomicU32::new(0),
            slot: ArcSwapOption::const_empty(),
        }
    }

    /// The state process `pid` owns, built by `fresh` when another process
    /// built the state published here.
    ///
    /// One caller of `pid` builds. The others call `wait` between looks at the
    /// rebuild marker and never wait on a lock. A marker left by another
    /// process names a rebuild that died with the fork, so it is taken over.
    pub(crate) fn current<E>(
        &self,
        pid: u32,
        mut wait: impl FnMut() -> Result<(), E>,
        fresh: impl FnOnce() -> Result<T, E>,
    ) -> Result<Arc<T>, E> {
        loop {
            if self.owner.load(Ordering::Acquire) == pid
                && let Some(state) = self.slot.load_full()
            {
                return Ok(state);
            }
            let marker = self.rebuilding.load(Ordering::Acquire);
            if marker == pid {
                wait()?;
                continue;
            }
            if self
                .rebuilding
                .compare_exchange(marker, pid, Ordering::AcqRel, Ordering::Acquire)
                .is_err()
            {
                continue;
            }
            let _idle = Idle(&self.rebuilding, pid);
            if self.owner.load(Ordering::Acquire) == pid {
                continue;
            }
            let state = Arc::new(fresh()?);
            // The inherited state may hold a lock, a socket, or an open file
            // a vanished thread owned, so it is never dropped here.
            std::mem::forget(self.slot.swap(Some(Arc::clone(&state))));
            self.owner.store(pid, Ordering::Release);
            return Ok(state);
        }
    }
}

/// Clears this process's rebuild marker when its builder returns or unwinds.
struct Idle<'a>(&'a AtomicU32, u32);

impl Drop for Idle<'_> {
    fn drop(&mut self) {
        let _cleared = self
            .0
            .compare_exchange(self.1, 0, Ordering::AcqRel, Ordering::Acquire);
    }
}

#[cfg(test)]
mod tests;
