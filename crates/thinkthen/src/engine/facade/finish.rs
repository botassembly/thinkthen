//! The exit flush a host that never drops its engine calls (ADR 0113).

use super::Engine;

impl Engine {
    /// Wait for this process's pending usage deltas under the usage lock's
    /// one-second deadline. State another process built, as in a forked
    /// child, is never touched, so nothing inherited is written twice.
    pub(crate) fn finish_usage(&self) {
        if let Some(state) = self.state.owned(std::process::id()) {
            let _failed = state.usage.finish();
        }
    }
}
