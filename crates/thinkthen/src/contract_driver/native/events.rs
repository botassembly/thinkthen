//! A retained wake source, independent of the call's sole mutable driver.
use std::sync::Arc;

/// A call's retained event source. This value can outlive its call.
///
/// Read [`Self::version`] before checking call readiness. If readiness reserves
/// input, a host may wait for that version while it also waits for caller input.
/// Row publication, queue changes and cancellation advance the version. A host
/// whose input arrives first calls [`Self::wake`] and joins its waiter before
/// sending input through the sole driver. A wake only requests a readiness
/// recheck: it does not consume a row, release a reservation or cancel the call.
#[derive(Clone, Debug)]
pub struct Events(pub(super) Arc<super::super::signal::Signal>);
impl Events {
    /// Read the event version before the sole driver checks readiness.
    #[must_use]
    pub fn version(&self) -> u64 {
        self.0.version()
    }
    /// Wait until the version differs, including when it changed before entry.
    /// This accesses only the retained signal and never the mutable driver.
    pub fn wait(&self, version: u64) {
        self.0.wait(version, None);
    }
    /// Release current or not-yet-entered waits on an earlier version.
    /// The host joins those waiters before retiring their event handle.
    pub fn wake(&self) {
        self.0.notify();
    }
}
