//! Caller-thread observation with the same join-before-unwind rule as checks.

use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::engine::workers;
use crate::public::results::RecordObservation;

use super::Stop;

impl Stop<'_> {
    pub(crate) fn observing(&self) -> bool {
        self.observer.is_some()
    }

    pub(crate) fn observer_panicked(&self) -> bool {
        self.panic.lock().is_ok_and(|held| held.is_some())
    }

    /// Hold an observer panic until all workers have joined, then resume it
    /// through `finish` on this same caller thread.
    pub(crate) fn observe(&self, observation: RecordObservation<'_>) {
        let Some(observer) = self.observer else {
            return;
        };
        if let Err(payload) = workers::with_host_diagnostics(|| {
            catch_unwind(AssertUnwindSafe(|| observer(observation)))
        }) {
            if let Ok(mut held) = self.panic.lock() {
                held.get_or_insert(payload);
            }
            self.fire();
        }
    }
}
