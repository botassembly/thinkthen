//! Concrete documents are the serialization and generated-schema source together.
use super::CompleteMeta;
use crate::core::{Answer, AnswerId, Question, Threshold, declaration::ReadableQuestion};
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(crate) enum Version {
    #[serde(rename = "thinkthen.result/2")]
    V2,
}
/// Decide alone may present an authored reading instead of its primitive answer.
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub(crate) enum DecideValue<'a> {
    Primitive(Option<bool>),
    Authored(&'a crate::core::text::Meaning),
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeAtomic_{V}")
)]
pub(crate) struct AtomicDocument<'a, T: Serialize, V: Serialize = DecideValue<'a>> {
    pub(crate) schema: Version,
    pub(crate) answer_id: &'a AnswerId,
    pub(crate) value: V,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input: Option<&'a T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) source: Option<&'a PhysicalSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) images: Option<&'a [crate::core::image::Image]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) question_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) members: Option<Vec<RankMemberDocument<'a>>>,
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

/// Borrowed presentation of an actual ordered set-rank member.
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeRankMember")
)]
pub(crate) struct RankMemberDocument<'a> {
    pub(crate) name: &'a str,
    #[cfg_attr(
        test,
        schemars(with = "AtomicDocument<'a, serde_json::Value, std::num::NonZeroUsize>")
    )]
    pub(crate) result: &'a super::Atomic,
}

/// One physical occurrence in an ordered native input set.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sessionInputSource")
)]
pub(crate) struct IndexedSource {
    pub(crate) index: usize,
    pub(crate) source: PhysicalSource,
}
