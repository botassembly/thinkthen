//! The saved calibration name that differs from this run's profile.

use serde::Serialize;

use crate::core::backend_profile::ProfileName;

/// A saved threshold and this run name different backend profiles.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename = "profileWarning"))]
pub(crate) struct ProfileWarning {
    tuned_for: String,
    running: String,
}

impl ProfileWarning {
    /// Build the mismatch only when both names exist and differ.
    pub(crate) fn between(
        tuned_for: Option<&ProfileName>,
        running: Option<&ProfileName>,
    ) -> Option<Self> {
        tuned_for.zip(running).and_then(|(tuned_for, running)| {
            (tuned_for != running).then(|| Self {
                tuned_for: tuned_for.as_str().to_owned(),
                running: running.as_str().to_owned(),
            })
        })
    }

    pub(crate) fn tuned_for(&self) -> &str {
        &self.tuned_for
    }

    pub(crate) fn running(&self) -> &str {
        &self.running
    }
}
