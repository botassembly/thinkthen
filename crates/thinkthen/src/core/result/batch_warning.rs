//! The saved batch setting that differs from this run's setting.

use std::fmt;

use serde::Serialize;

use crate::core::batch::Setting;

/// One batch setting in the documented number-or-`max` JSON form.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "batchSetting"))]
pub(crate) enum BatchSetting {
    Records(#[cfg_attr(test, schemars(range(min = 1)))] usize),
    Max(&'static str),
}

impl From<Setting> for BatchSetting {
    fn from(setting: Setting) -> Self {
        match setting {
            Setting::Max => Self::Max("max"),
            Setting::Records(number) => Self::Records(number.get()),
        }
    }
}

impl fmt::Display for BatchSetting {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Max(value) => formatter.write_str(value),
            Self::Records(value) => value.fmt(formatter),
        }
    }
}

/// A file's tuned batch setting and the different setting used by this run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "batchWarning"))]
pub(crate) struct BatchWarning {
    tuned_for: BatchSetting,
    running: BatchSetting,
}

impl BatchWarning {
    pub(crate) fn between(tuned_for: Setting, running: Setting) -> Option<Self> {
        (tuned_for != running).then(|| Self {
            tuned_for: tuned_for.into(),
            running: running.into(),
        })
    }

    pub(crate) const fn tuned_for(&self) -> BatchSetting {
        self.tuned_for
    }

    pub(crate) const fn running(&self) -> BatchSetting {
        self.running
    }
}
