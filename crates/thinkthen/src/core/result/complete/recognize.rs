//! Complete recognition uses the scheduler's typed stage distributions.

use serde::{Serialize, Serializer};

use super::{CompleteMeta, ResultIdentity};
use crate::core::{Meta, RecognitionOdds, RecognizeSpec, RecognizedValue, Record, Usage};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Recognition {
    pub(crate) identity: ResultIdentity,
    pub(crate) value: RecognizedValue,
    pub(crate) input: Option<Record>,
    pub(crate) question: RecognizeSpec,
    pub(crate) answer: RecognitionOdds,
    pub(crate) meta: Meta,
}

impl Recognition {
    pub(crate) fn metadata(&self) -> crate::core::MetadataFields<'_> {
        self.meta.fields()
    }

    pub(crate) const fn reported_usage(&self) -> Option<crate::core::ReportedUsage> {
        self.meta.reported_usage
    }

    pub(crate) const fn usage(&self) -> Option<Usage> {
        self.meta.usage
    }
}

#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeRecognition")
)]
pub(crate) struct Document<'a, T: Serialize, V: Serialize = RecognizedValue> {
    schema: super::wire::Version,
    answer_id: &'a crate::core::AnswerId,
    value: &'a V,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<&'a T>,
    question: crate::core::declaration::ReadableQuestion<
        'a,
        crate::core::recognize_file::QuestionDocument<'a>,
    >,
    answer: &'a RecognitionOdds,
    meta: CompleteMeta<'a>,
}
impl Recognition {
    pub(crate) fn serialize_with_input<S: Serializer, T: Serialize>(
        &self,
        input: Option<&T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        self.serialize_with_value(input, &self.value, serializer)
    }
    /// Located presentation delegates the same complete document and identity.
    pub(crate) fn serialize_with_value<S: Serializer, T: Serialize, V: Serialize>(
        &self,
        input: Option<&T>,
        value: &V,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Document {
            schema: super::wire::Version::V2,
            answer_id: self.identity.answer_id(),
            value,
            input,
            question: crate::core::declaration::ReadableQuestion {
                question: &self.question.document(),
                metadata: &self.question.metadata,
            },
            answer: &self.answer,
            meta: CompleteMeta::of(&self.meta, &self.identity),
        }
        .serialize(serializer)
    }
}

impl Serialize for Recognition {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_with_input(self.input.as_ref(), serializer)
    }
}
