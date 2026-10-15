//! The exit flush a host that never drops its engine calls (ADR 0113).

use super::Engine;
use crate::engine::usage::UsagePersistence;

impl Engine {
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
            state.usage.finish_status()
        } else {
            self.usage_persistence()
        }
    }
}
