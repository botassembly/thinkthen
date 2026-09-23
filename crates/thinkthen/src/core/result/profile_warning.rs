//! The saved calibration name that differs from this run's profile.

use serde::Serialize;

use crate::core::backend_profile::ProfileName;

/// A saved threshold and this run name different backend profiles.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ProfileWarning {
    calibrated: String,
    running: String,
}

impl ProfileWarning {
    /// Build the mismatch only when both names exist and differ.
    pub(crate) fn between(
        calibrated: Option<&ProfileName>,
        running: Option<&ProfileName>,
    ) -> Option<Self> {
        calibrated.zip(running).and_then(|(calibrated, running)| {
            (calibrated != running).then(|| Self {
                calibrated: calibrated.as_str().to_owned(),
                running: running.as_str().to_owned(),
            })
        })
    }

    pub(crate) fn calibrated(&self) -> &str {
        &self.calibrated
    }

    pub(crate) fn running(&self) -> &str {
        &self.running
    }
}
