//! One process-scoped send count across SQL engines and retries. The
//! public API re-exports [`SendBudget`].

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::SendBudgetDenial;
use crate::engine::process::Guarded;

/// One owner's attempted live sends, shared by its engine clones.
/// A forked child starts a fresh count when it first reserves a send.
#[derive(Clone, Debug)]
pub struct SendBudget(Arc<Guarded<BudgetCount>>);

#[derive(Debug, Default)]
struct BudgetCount {
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
        Self(Arc::new(Guarded::empty()))
    }

    /// The counts process `pid` owns. The rebuild allocates two counters, so
    /// a thread that finds another rebuilding only yields.
    fn count(&self, pid: u32) -> Arc<BudgetCount> {
        let Ok(count) = self.0.current(
            pid,
            || {
                std::thread::yield_now();
                Ok::<(), std::convert::Infallible>(())
            },
            || Ok(BudgetCount::default()),
        );
        count
    }

    /// The sends this process's count holds now, reservations included.
    pub(crate) fn sent(&self) -> u64 {
        self.count(std::process::id()).sent.load(Ordering::Acquire)
    }

    /// Reserve the version-one estimate of one final encoded request body.
    pub(crate) fn reserve_estimated(
        &self,
        limit: Option<u64>,
        bytes: usize,
        image_estimate: Option<u64>,
    ) -> Result<EstimatedReservation, ()> {
        let count = self.count(std::process::id());
        let bytes = u64::try_from(bytes).map_err(|_| ())?;
        let amount = match image_estimate {
            Some(tokens) => tokens,
            None => crate::core::PlanSummary::estimated_input_high(bytes).ok_or(())?,
        };
        let mut spent = count.estimated.load(Ordering::Acquire);
        loop {
            let next = spent.checked_add(amount).ok_or(())?;
            if limit.is_some_and(|limit| next > limit) {
                return Err(());
            }
            match count.estimated.compare_exchange_weak(
                spent,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    return Ok(EstimatedReservation {
                        count,
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
        let count = self.count(std::process::id());
        let denial = last_status.map_or(SendBudgetDenial::BeforeFirstSend, |last_status| {
            SendBudgetDenial::BeforeRetry { last_status }
        });
        let mut sent = count.sent.load(Ordering::Acquire);
        loop {
            if limit.is_some_and(|limit| sent >= limit) {
                return Err(denial);
            }
            let Some(next) = sent.checked_add(1) else {
                return Err(denial);
            };
            match count
                .sent
                .compare_exchange_weak(sent, next, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => {
                    return Ok(SendReservation {
                        count,
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
fn a_forked_child_counts_from_zero() {
    let budget = SendBudget::new();
    budget.reserve(Some(1), None).expect("parent send").commit();
    assert!(
        budget.reserve(Some(1), None).is_err(),
        "the parent is spent"
    );
    // A forked child is the parent's memory under another process ID. The
    // rebuild rule, including a marker the parent left, is `process/tests.rs`.
    let child = budget.count(std::process::id().wrapping_add(1));
    assert_eq!(child.sent.load(Ordering::Acquire), 0);
    assert_eq!(child.estimated.load(Ordering::Acquire), 0);
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
