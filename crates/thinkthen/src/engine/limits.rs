//! The one owner of a process's send limits. One process has one throttle,
//! by Ian's ruling in ticket 0077, and the gates and totals follow it. One
//! fork rebuild replaces all three (ticket 0304 slice 3d). Ian can overturn it.

use std::fmt;
#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use super::backoff::Gates;
use super::budget::SendBudget;
use super::{CANCEL_POLL, Cancel, Width, error, process};

/// Every send limit of one process.
#[derive(Debug)]
pub(crate) struct Limits {
    /// The throttle: the selected width and the attempts holding a permit.
    pub(crate) widths: Widths,
    /// The 429 gate and the pacer of each posting address.
    pub(crate) gates: Gates,
    /// The request and estimated input totals every engine's own limit reads.
    pub(crate) total: SendBudget,
}

static PROCESS_LIMITS: process::Guarded<&'static Limits> = process::Guarded::empty();

/// The limits process `pid` owns, fresh in a forked child.
pub(crate) fn of(pid: u32, cancel: &Cancel) -> Result<&'static Limits, error::Error> {
    limits_of(pid, rebuild_wait(cancel))
}

/// This process's limits, for a test that holds no call to stop.
#[cfg(test)]
pub(crate) fn process() -> &'static Limits {
    let Ok(limits) = limits_of(std::process::id(), || {
        std::thread::sleep(CANCEL_POLL);
        Ok::<(), std::convert::Infallible>(())
    });
    limits
}

/// Each process leaks one set of limits, as the statics they replace never dropped.
fn limits_of<E>(pid: u32, wait: impl FnMut() -> Result<(), E>) -> Result<&'static Limits, E> {
    PROCESS_LIMITS
        .current(pid, wait, || {
            let (widths, gates, total) = (Widths::new(), Gates::default(), SendBudget::new());
            Ok(&*Box::leak(Box::new(Limits {
                widths,
                gates,
                total,
            })))
        })
        .map(|limits| *limits)
}

/// Wait 50 ms while another thread of this process rebuilds its state, and
/// stop with the call.
pub(crate) fn rebuild_wait<'c>(
    cancel: &'c Cancel<'_>,
) -> impl FnMut() -> Result<(), error::Error> + 'c {
    move || cancel.wait(CANCEL_POLL).map_or(Ok(()), Err)
}

/// A unit-test binary runs many unrelated tests in one process, so a client
/// there gets its own gate unless a width child test asked for the process one.
#[cfg(test)]
pub(crate) static WIDTH_CHILD: AtomicBool = AtomicBool::new(false);

/// The gate a new client of `process` sends through.
pub(crate) fn client_width(process: &'static Widths) -> &'static Widths {
    #[cfg(test)]
    if !WIDTH_CHILD.load(Ordering::Acquire) {
        return Box::leak(Box::default());
    }
    process
}

/// A later explicit width that differs from the one this process selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WidthActive(pub(crate) Width);

impl fmt::Display for WidthActive {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let active = self.0.get();
        write!(
            formatter,
            "throttle {active} is already active for this process; use throttle {active} or drop the throttle argument"
        )
    }
}

/// The width selection and the one attempt gate of one process.
///
/// A forked child replaces this whole value, so it holds the only lock and
/// the only condition variable on the width path.
#[derive(Debug)]
pub(crate) struct Widths {
    state: Mutex<WidthState>,
    freed: Condvar,
}

#[derive(Debug)]
struct WidthState {
    selected: Option<Width>,
    active: usize,
}

impl Default for Widths {
    fn default() -> Self {
        Self::new()
    }
}

impl Widths {
    pub(crate) const fn new() -> Self {
        Self {
            state: Mutex::new(WidthState {
                selected: None,
                active: 0,
            }),
            freed: Condvar::new(),
        }
    }

    /// Register one engine's width and return the width its calls follow.
    ///
    /// `None` never selects. The first explicit width selects itself, the same
    /// width again is accepted, and a different one is refused.
    pub(crate) fn select(&self, asked: Option<Width>) -> Result<Width, WidthActive> {
        let mut state = self.lock();
        match (asked, state.selected) {
            (None, selected) => Ok(selected.unwrap_or(Width::FALLBACK)),
            (Some(asked), None) => {
                state.selected = Some(asked);
                self.freed.notify_all();
                Ok(asked)
            }
            (Some(asked), Some(active)) if asked == active => Ok(active),
            (Some(_), Some(active)) => Err(WidthActive(active)),
        }
    }

    /// The explicit width this process selected, if any.
    #[cfg(test)]
    pub(crate) fn selected(&self) -> Option<Width> {
        self.lock().selected
    }

    /// Attempts holding a permit now.
    #[cfg(test)]
    pub(crate) fn active(&self) -> usize {
        self.lock().active
    }

    /// Wait for room under the effective width, observing cancellation every
    /// 50 ms and the deadline when it passes. A stopped waiter holds nothing.
    pub(crate) fn acquire(&self, cancel: &Cancel) -> Result<Permit<'_>, error::Error> {
        let mut state = self.lock();
        let mut observed = false;
        loop {
            let budget = cancel.stop_or_remaining()?;
            let width = state.selected.unwrap_or(Width::FALLBACK).get();
            if state.active < width {
                state.active += 1;
                return Ok(Permit(self, true));
            }
            if !observed {
                observed = true;
                cancel.observed_block();
            }
            let poll = budget.map_or(CANCEL_POLL, |budget| budget.min(CANCEL_POLL));
            state = self
                .freed
                .wait_timeout(state, poll)
                .map_or_else(|poisoned| poisoned.into_inner().0, |(state, _)| state);
        }
    }

    fn lock(&self) -> MutexGuard<'_, WidthState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Room for one live attempt, given back when dropped.
#[derive(Debug)]
pub(crate) struct Permit<'a>(&'a Widths, bool);

impl Permit<'_> {
    /// Close the provider gate before another request can take this send slot.
    pub(crate) fn release_closing(
        mut self,
        gates: &Gates,
        url: &str,
        wait: Duration,
        server_floor: Option<u16>,
    ) {
        let mut state = self.0.lock();
        gates.close(url, wait, server_floor);
        state.active = state.active.saturating_sub(1);
        self.1 = false;
        self.0.freed.notify_all();
    }
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        if !self.1 {
            return;
        }
        let mut state = self.0.lock();
        state.active = state.active.saturating_sub(1);
        self.0.freed.notify_all();
    }
}

/// A waiter on another thread's rebuild sleeps one 50 ms stop check at a
/// time, and a stop ends its wait at once.
#[cfg(test)]
#[test]
fn a_stop_ends_a_rebuild_wait() {
    let stopped = Cancel::default();
    stopped.fire();
    assert!(matches!(
        rebuild_wait(&stopped)(),
        Err(error::Error::Cancelled)
    ));
    let live = Cancel::default();
    let started = std::time::Instant::now();
    assert!(rebuild_wait(&live)().is_ok());
    assert!(
        started.elapsed() >= CANCEL_POLL,
        "one wait is one stop check"
    );
}
