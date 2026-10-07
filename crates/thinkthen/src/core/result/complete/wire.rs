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
    pub(crate) source: Option<&'a PhysicalSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) question_name: Option<&'a str>,
    pub(crate) question: ReadableQuestion<'a, Question>,
    pub(crate) answer: &'a Answer,
    pub(crate) threshold: Option<Threshold>,
    pub(crate) meta: CompleteMeta<'a>,
}

/// Supplied physical provenance, outside model evidence and result identity.
#[derive(Clone, PartialEq, Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completePhysicalSource")
)]
pub(crate) struct PhysicalSource {
    pub(crate) file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) first_line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_line: Option<usize>,
}
impl std::fmt::Debug for PhysicalSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhysicalSource")
            .field("file", &"<withheld>")
            .field("first_line", &self.first_line)
            .field("last_line", &self.last_line)
            .finish()
    }
}
