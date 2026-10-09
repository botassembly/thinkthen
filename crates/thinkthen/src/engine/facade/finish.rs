//! The exit flush a host that never drops its engine calls (ADR 0113).

use super::Engine;
use crate::engine::usage::UsagePersistence;

impl Engine {
    /// Wait for this process's pending usage deltas under the usage lock's
    /// one-second deadline. State another process built, as in a forked
    /// child, is never touched, so nothing inherited is written twice.
    pub(crate) fn finish_usage(&self) {
        let _status = self.finish_usage_status();
    }

    pub(crate) fn usage_persistence(&self) -> UsagePersistence {
        self.state.owned(std::process::id()).map_or_else(
            || {
                if self.usage_path.is_some() {
                    UsagePersistence::Written
                } else {
                    UsagePersistence::Disabled
                }
            },
            |state| state.usage.persistence(),
        )
    }

    pub(crate) fn finish_usage_status(&self) -> UsagePersistence {
        if let Some(state) = self.state.owned(std::process::id()) {
            let _failed = state.usage.finish();
            state.usage.persistence()
        } else {
            self.usage_persistence()
        }
    }
}
