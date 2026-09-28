//! One process-scoped send count across SQL engines and retries.

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Why a SQL process send budget refused a live attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SendBudgetDenial {
    /// No attempt for this call was sent.
    BeforeFirstSend,
    /// A retry was refused after this backend status was received.
    BeforeRetry {
        /// The backend status whose retry would cross the process total.
        last_status: u16,
    },
}

/// One process's attempted live sends, shared by its SQL engines.
/// A forked child starts a fresh count when it first reserves a send.
#[derive(Clone, Debug)]
pub struct SendBudget(Arc<BudgetCount>);

#[derive(Debug)]
struct BudgetCount {
    owner: AtomicU32,
    resetting: AtomicU32,
    sent: AtomicU64,
}

impl Default for SendBudget {
    fn default() -> Self {
        Self::new()
    }
}

impl SendBudget {
    /// Start a process-scoped budget with no sends counted.
    #[must_use]
    pub fn new() -> Self {
        Self(Arc::new(BudgetCount {
            owner: AtomicU32::new(std::process::id()),
            resetting: AtomicU32::new(0),
            sent: AtomicU64::new(0),
        }))
    }

    fn reset_after_fork(&self) {
        let pid = std::process::id();
        loop {
            let previous = self.0.owner.load(Ordering::Acquire);
            if previous == pid {
                return;
            }
            let marker = self.0.resetting.load(Ordering::Acquire);
            if marker == pid {
                std::hint::spin_loop();
            } else if self
                .0
                .resetting
                .compare_exchange(marker, pid, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                self.0.sent.store(0, Ordering::Release);
                self.0.owner.store(pid, Ordering::Release);
                self.0.resetting.store(0, Ordering::Release);
                return;
            }
        }
    }

    pub(crate) fn reserve(
        &self,
        limit: Option<u64>,
        last_status: Option<u16>,
    ) -> Result<SendReservation, SendBudgetDenial> {
        self.reset_after_fork();
        let denial = last_status.map_or(SendBudgetDenial::BeforeFirstSend, |last_status| {
            SendBudgetDenial::BeforeRetry { last_status }
        });
        let mut sent = self.0.sent.load(Ordering::Acquire);
        loop {
            if limit.is_some_and(|limit| sent >= limit) {
                return Err(denial);
            }
            let Some(next) = sent.checked_add(1) else {
                return Err(denial);
            };
            match self
                .0
                .sent
                .compare_exchange_weak(sent, next, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => {
                    return Ok(SendReservation {
                        count: Arc::clone(&self.0),
                        committed: false,
                    });
                }
                Err(observed) => sent = observed,
            }
        }
    }
}

/// Refund a reservation only when usage could not mark the attempt.
pub(crate) struct SendReservation {
    count: Arc<BudgetCount>,
    committed: bool,
}

impl SendReservation {
    pub(crate) fn commit(mut self) {
        self.committed = true;
    }
}

impl Drop for SendReservation {
    fn drop(&mut self) {
        if !self.committed {
            self.count.sent.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

#[cfg(test)]
#[test]
fn inherited_budget_and_reset_marker_do_not_block_a_child() {
    let budget = SendBudget::new();
    budget.reserve(None, None).expect("parent send").commit();
    let other_pid = std::process::id().wrapping_add(1);
    budget.0.owner.store(other_pid, Ordering::Release);
    budget.0.resetting.store(other_pid, Ordering::Release);
    budget
        .reserve(Some(1), None)
        .expect("fresh child total")
        .commit();
    assert!(budget.reserve(Some(1), None).is_err());
}
