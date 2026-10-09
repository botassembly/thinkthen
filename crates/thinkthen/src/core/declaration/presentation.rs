//! Complete presentation borrows authored values without changing semantic serialization.
use super::AuthoredReading;
use crate::core::{Question, Setting, text::Description};
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
enum Batch {
    Records(std::num::NonZeroUsize),
    Max(Max),
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(inline))]
enum Max {
    #[serde(rename = "max")]
    Max,
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(super) struct Reading<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch: Option<Batch>,
    #[serde(skip_serializing_if = "<[String]>::is_empty")]
    on: &'a [String],
}
impl<'a> Reading<'a> {
    pub(super) fn of(reading: &'a AuthoredReading) -> Self {
        Self {
            model: reading.model.as_deref(),
            profile: reading.profile.as_deref(),
            batch: reading.batch.map(|batch| match batch {
                Setting::Max => Batch::Max(Max::Max),
                Setting::Records(count) => Batch::Records(count),
            }),
            on: &reading.on,
        }
    }
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(super) struct Label<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a Description>,
}
pub(super) fn labels(question: &Question) -> Option<Vec<Label<'_>>> {
    let labels = match question {
        Question::Choose { options, .. } => options,
        Question::Tag { labels, .. } => labels,
        Question::Score { levels, .. } => levels,
        Question::Decide { .. } => return None,
    };
    labels
        .descriptions()
        .any(|(_, description)| description.is_some())
        .then(|| {
            labels
                .descriptions()
                .map(|(name, description)| Label { name, description })
                .collect()
        })
}
