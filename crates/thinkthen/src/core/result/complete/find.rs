//! Complete whole-set find keeps its dedicated question and ordered distribution.

use serde::{Serialize, Serializer};

use super::{CompleteMeta, ResultIdentity};
use crate::core::{FindResult, Usage};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Find {
    pub(crate) declarations: crate::core::declaration::QuestionMetadata,
    pub(crate) identity: ResultIdentity,
    pub(crate) legacy: FindResult,
}

impl Find {
    pub(crate) fn display_question<'a>(
        &'a self,
        declarations: &'a crate::core::declaration::QuestionMetadata,
    ) -> impl Serialize + 'a {
        crate::core::declaration::SemanticReadableQuestion::semantic(
            &self.legacy.question,
            declarations,
        )
    }

    pub(crate) fn profile(&self) -> Option<&str> {
        self.legacy.question.profile()
    }
    pub(crate) fn raw_pick(&self) -> &str {
        self.legacy.answer.pick()
    }
    pub(crate) fn selected(&self) -> Option<usize> {
        self.legacy.answer.selected()
    }
    pub(crate) const fn question(&self) -> (&crate::core::QuestionText, bool) {
        self.legacy.question.parts()
    }

    pub(crate) fn metadata(&self) -> crate::core::MetadataFields<'_> {
        self.legacy.meta.fields()
    }

    pub(crate) fn confidence(&self) -> Option<f64> {
        self.legacy.answer.confidence()
    }

    pub(crate) const fn reported_usage(&self) -> Option<crate::core::ReportedUsage> {
        self.legacy.meta.reported_usage
    }

    pub(crate) const fn usage(&self) -> Option<Usage> {
        self.legacy.meta.usage
    }
}

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "completeFind"))]
pub(crate) struct Document<'a, T: Serialize> {
    schema: super::wire::Version,
    answer_id: &'a crate::core::AnswerId,
    value: Option<&'a T>,
    index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    candidates: Option<&'a [CandidateDocument<'a, T>]>,
    question: crate::core::declaration::SemanticReadableQuestion<
        'a,
        crate::core::find::FindQuestionOwned,
    >,
    answer: &'a crate::core::FindAnswer,
    threshold: Option<()>,
    meta: CompleteMeta<'a>,
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeFindCandidate")
)]
pub(crate) struct CandidateDocument<'a, T: Serialize> {
    pub(crate) index: Option<usize>,
    pub(crate) input: Option<&'a T>,
    pub(crate) probability: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) source: Option<&'a super::wire::PhysicalSource>,
}
impl Find {
    pub(crate) fn serialize_with_value<S: Serializer, T: Serialize>(
        &self,
        value: Option<&T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        self.serialize_candidates(value, None, serializer)
    }
    pub(crate) fn serialize_candidates<S: Serializer, T: Serialize>(
        &self,
        value: Option<&T>,
        candidates: Option<&[CandidateDocument<'_, T>]>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let row = &self.legacy;
        Document {
            schema: super::wire::Version::V2,
            answer_id: self.identity.answer_id(),
            value,
            index: self.selected(),
            candidates,
            question: crate::core::declaration::SemanticReadableQuestion::semantic(
                &row.question,
                &self.declarations,
            ),
            answer: &row.answer,
            threshold: row.threshold,
            meta: CompleteMeta::of(&row.meta, &self.identity),
        }
        .serialize(serializer)
    }
}

impl Serialize for Find {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_with_value(self.legacy.value.as_ref(), serializer)
    }
}
