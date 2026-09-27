//! One retry gate per posting address, shared by this process's engines.

use std::collections::HashMap;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use crate::engine::error::Error;
use crate::engine::{Cancel, process};

#[derive(Debug, Default)]
pub(crate) struct Gates {
    closed: Mutex<HashMap<String, Gate>>,
    changed: Condvar,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Opening {
    At(Instant),
    Never,
}

impl Opening {
    fn after(wait: Duration) -> Self {
        Instant::now()
            .checked_add(wait)
            .map_or(Self::Never, Self::At)
    }

    fn passed(self, now: Instant) -> bool {
        matches!(self, Self::At(at) if at <= now)
    }

    fn remaining(self, now: Instant) -> Duration {
        match self {
            Self::At(at) => at.saturating_duration_since(now),
            Self::Never => Duration::MAX,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Gate {
    until: Opening,
    floor: Option<Opening>,
}

impl Gates {
    fn lock(&self) -> MutexGuard<'_, HashMap<String, Gate>> {
        self.closed.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Keep the later opening when two requests receive a retried status.
    pub(crate) fn close(&self, url: &str, wait: Duration, server_floor: bool) {
        let mut closed = self.lock();
        let opening = Opening::after(wait);
        let gate = closed.entry(url.to_owned()).or_insert(Gate {
            until: opening,
            floor: None,
        });
        gate.until = gate.until.max(opening);
        if server_floor {
            gate.floor = Some(gate.floor.map_or(opening, |floor| floor.max(opening)));
        }
        self.changed.notify_all();
    }

    pub(crate) fn may_send(&self, url: &str, cap: Instant) -> bool {
        let closed = self.lock();
        let now = Instant::now();
        closed.get(url).is_none_or(|gate| {
            gate.until.passed(now)
                || (cap <= now && gate.floor.is_none_or(|floor| floor.passed(now)))
        })
    }

    /// Wait without a send permit. Return false once this attempt's wait cap passes.
    pub(crate) fn wait_open(
        &self,
        url: &str,
        cap: Instant,
        cancel: &Cancel<'_>,
    ) -> Result<bool, Error> {
        let mut closed = self.lock();
        let mut observed = false;
        loop {
            let budget = cancel.stop_or_remaining()?;
            let now = Instant::now();
            let Some(gate) = closed.get(url).copied() else {
                return Ok(true);
            };
            if gate.until.passed(now) {
                return Ok(true);
            }
            if cap <= now && gate.floor.is_none_or(|floor| floor.passed(now)) {
                return Ok(false);
            }
            if !observed {
                observed = true;
                cancel.observed_block();
            }
            let poll = gate
                .until
                .remaining(now)
                .min(
                    gate.floor
                        .map_or(Duration::MAX, |floor| floor.remaining(now)),
                )
                .min(Cancel::poll())
                .min(budget.unwrap_or(Duration::MAX));
            closed = self
                .changed
                .wait_timeout(closed, poll)
                .map_or_else(|poisoned| poisoned.into_inner().0, |(closed, _)| closed);
        }
    }
}

static PROCESS_GATES: process::Guarded<&'static Gates> = process::Guarded::empty();

/// A child process replaces inherited locks before looking inside the table.
pub(crate) fn process_gates(cancel: &Cancel<'_>) -> Result<&'static Gates, Error> {
    PROCESS_GATES
        .current(
            std::process::id(),
            crate::engine::rebuild_wait(cancel),
            || Ok(&*Box::leak(Box::new(Gates::default()))),
        )
        .map(|gates| *gates)
}
