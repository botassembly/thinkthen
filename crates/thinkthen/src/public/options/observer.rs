//! Caller-thread observation with the same join-before-unwind rule as checks.

use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::engine::workers;
use crate::public::results::RecordObservation;

use super::Stop;

impl Stop<'_> {
    pub(crate) fn drain_attempts(&self) {
        let Some(receiver) = &self.attempts else {
            return;
        };
        loop {
            let event = receiver
                .lock()
                .ok()
                .and_then(|receiver| receiver.try_recv().ok());
            let Some(event) = event else {
                break;
            };
            if self.observer_panicked() {
                continue;
            }
            let Some(observer) = self.attempt_observer else {
                continue;
            };
            if let Err(payload) = workers::with_host_diagnostics(|| {
                catch_unwind(AssertUnwindSafe(|| observer(event)))
            }) {
                self.hold_panic(payload);
                self.fire();
            }
        }
    }

    pub(crate) fn observing(&self) -> bool {
        self.observer.is_some()
    }

    pub(crate) fn observer_panicked(&self) -> bool {
        self.panic.lock().is_ok_and(|held| held.is_some())
    }

    fn hold_panic(&self, payload: Box<dyn std::any::Any + Send>) {
        if let Ok(mut held) = self.panic.lock() {
            held.get_or_insert(payload);
        }
    }

    /// A dropped lazy batch has no later `finish`; resume only after its join.
    pub(crate) fn resume_panic_after_join(&self) {
        if std::thread::panicking() {
            return;
        }
        let held = self.panic.lock().ok().and_then(|mut held| held.take());
        if let Some(payload) = held {
            self.facts.finish();
            resume_unwind(payload);
        }
    }

    /// Hold an observer panic until all workers have joined, then resume it
    /// through `finish` on this same caller thread.
    pub(crate) fn observe(&self, observation: RecordObservation<'_>) {
        let Some(observer) = self.observer else {
            return;
        };
        if self.observer_panicked() {
            return;
        }
        if let Err(payload) = workers::with_host_diagnostics(|| {
            catch_unwind(AssertUnwindSafe(|| observer(observation)))
        }) {
            self.hold_panic(payload);
            self.fire();
        }
    }
}
