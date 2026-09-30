//! One process-scoped send count across SQL engines and retries. The
//! public API re-exports [`SendBudget`].

use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use crate::core::SendBudgetDenial;

/// One owner's attempted live sends, shared by its engine clones.
/// A forked child starts a fresh count when it first reserves a send.
#[derive(Clone, Debug)]
pub struct SendBudget(Arc<BudgetCount>);

#[derive(Debug)]
struct BudgetCount {
    owner: AtomicU32,
    resetting: AtomicU32,
    sent: AtomicU64,
    estimated: AtomicU64,
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
            estimated: AtomicU64::new(0),
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
                self.0.estimated.store(0, Ordering::Release);
                self.0.owner.store(pid, Ordering::Release);
                self.0.resetting.store(0, Ordering::Release);
                return;
            }
        }
    }

    /// Reserve the version-one estimate of one final encoded request body.
    pub(crate) fn reserve_estimated(
        &self,
        limit: Option<u64>,
        bytes: usize,
    ) -> Result<EstimatedReservation, ()> {
        self.reset_after_fork();
        let bytes = u64::try_from(bytes).map_err(|_| ())?;
        let amount = crate::core::PlanSummary::estimated_input_high(bytes).ok_or(())?;
        let mut spent = self.0.estimated.load(Ordering::Acquire);
        loop {
            let next = spent.checked_add(amount).ok_or(())?;
            if limit.is_some_and(|limit| next > limit) {
                return Err(());
            }
            match self.0.estimated.compare_exchange_weak(
                spent,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    return Ok(EstimatedReservation {
                        count: Arc::clone(&self.0),
                        amount,
                        committed: false,
                    });
                }
                Err(observed) => spent = observed,
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

/// One count for the Rust, command and C constructors in this process.
/// Each caller still selects its own limit at the reservation.
pub(crate) fn process_budget() -> SendBudget {
    static BUDGET: OnceLock<SendBudget> = OnceLock::new();
    BUDGET.get_or_init(SendBudget::new).clone()
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

/// An estimated-input reservation, refunded unless transport starts.
pub(crate) struct EstimatedReservation {
    count: Arc<BudgetCount>,
    amount: u64,
    committed: bool,
}

impl EstimatedReservation {
    pub(crate) fn commit(mut self) {
        self.committed = true;
    }
}

impl Drop for EstimatedReservation {
    fn drop(&mut self) {
        if !self.committed {
            self.count
                .estimated
                .fetch_sub(self.amount, Ordering::AcqRel);
        }
    }
}
