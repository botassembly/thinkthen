//! The saved batch setting that differs from this run's setting.

use std::collections::BTreeSet;
use std::fmt;

use serde::Serialize;

use crate::core::batch::Setting;
use crate::core::json::Json;

/// One batch setting in the documented number-or-`max` JSON form.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub(crate) enum BatchSetting {
    Records(usize),
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

impl BatchSetting {
    /// The settings named by result metadata; absent metadata means batch one
    /// only when no line names a batched setting.
    pub(crate) fn in_results(lines: &[(usize, Json)]) -> BTreeSet<Self> {
        let mut settings: BTreeSet<_> = lines
            .iter()
            .filter_map(|(_, row)| {
                let value = row.member("meta")?.member("batch")?.member("setting")?;
                Setting::of_json(value).map(Into::into)
            })
            .collect();
        if settings.is_empty() {
            settings.insert(Self::Records(1));
        }
        settings
    }

    pub(crate) fn listed(settings: &BTreeSet<Self>) -> String {
        let mut names: Vec<String> = settings.iter().map(ToString::to_string).collect();
        match names.pop() {
            None => String::new(),
            Some(last) if names.is_empty() => last,
            Some(last) => format!("{} and {last}", names.join(", ")),
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
