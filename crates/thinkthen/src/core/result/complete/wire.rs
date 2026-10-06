//! Concrete documents are the serialization and generated-schema source together.
use super::CompleteMeta;
use crate::core::{Answer, AnswerId, Question, Threshold, Value, declaration::ReadableQuestion};
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(crate) enum Version {
    #[serde(rename = "thinkthen.result/2")]
    V2,
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub(crate) enum AtomicValue<'a> {
    Primitive(&'a Value),
    Rank(std::num::NonZeroUsize),
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeAtomic")
)]
pub(crate) struct AtomicDocument<'a, T: Serialize> {
    pub(crate) schema: Version,
    pub(crate) answer_id: &'a AnswerId,
    pub(crate) value: AtomicValue<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input: Option<&'a T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) question_name: Option<&'a str>,
    pub(crate) question: ReadableQuestion<'a, Question>,
    pub(crate) answer: &'a Answer,
    pub(crate) threshold: Option<Threshold>,
    pub(crate) meta: CompleteMeta<'a>,
}
