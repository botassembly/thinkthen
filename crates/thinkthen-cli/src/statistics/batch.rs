//! Saved batch observations and command warning rendering.
use crate::saved_json::{Json, Kind};
use serde::Serialize;
use std::{collections::BTreeSet, fmt};
use thinkthen::{BatchSetting, RequestBatch};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub(crate) enum SavedBatch {
    Records(usize),
    Max(&'static str),
}
impl SavedBatch {
    pub(crate) fn of_json(value: &Json) -> Option<Self> {
        let request = match value.kind() {
            Kind::Number(number) => RequestBatch::Count(usize::try_from(number.as_u64()?).ok()?),
            Kind::String(text) => RequestBatch::Named(text.clone()),
            _ => return None,
        };
        Some(match request.native().ok()? {
            BatchSetting::Max => Self::Max("max"),
            BatchSetting::Records(count) => Self::Records(count.get()),
        })
    }
    pub(crate) fn in_results(
        lines: &[(usize, Json)],
    ) -> Result<BTreeSet<Self>, (usize, &'static str)> {
        let mut settings = BTreeSet::new();
        for (line, row) in lines {
            let Some(named) = row
                .member("meta")
                .and_then(|meta| meta.member("batch_setting"))
            else {
                continue;
            };
            settings.insert(Self::of_json(named).ok_or((*line, "meta.batch_setting"))?);
        }
        Ok(settings)
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
impl fmt::Display for SavedBatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Max(value) => formatter.write_str(value),
            Self::Records(value) => value.fmt(formatter),
        }
    }
}
