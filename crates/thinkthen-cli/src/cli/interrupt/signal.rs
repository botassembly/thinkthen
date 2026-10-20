//! The process signal latch belongs to the command, separate from engine controls.
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub(crate) struct Signal(Arc<AtomicBool>);
impl Signal {
    pub(crate) fn fired(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
    pub(crate) fn interrupt(&self) -> Arc<dyn Fn() -> bool + Send + Sync> {
        let signal = self.clone();
        Arc::new(move || signal.fired())
    }
    pub(super) fn flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.0)
    }
    pub(super) fn reset(&self) {
        self.0.store(false, Ordering::Release);
    }
    #[cfg(test)]
    pub(super) fn fire(&self) {
        self.0.store(true, Ordering::Release);
    }
    /// Preserve the acknowledgment carrier's existing polling cadence.
    pub(super) const fn poll() -> Duration {
        Duration::from_millis(50)
    }
}
