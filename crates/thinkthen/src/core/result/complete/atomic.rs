//! Complete atomic judgments share the frozen legacy answer representation.

use serde::{Serialize, Serializer};

use super::{CompleteMeta, ResultIdentity};
use crate::core::{Answer, DecisionResult, Usage};
mod set_rank;

/// The complete canonical document for decide, choose, tag, score, filter or rank.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Atomic {
    pub(crate) declarations: crate::core::declaration::QuestionMetadata,
    pub(crate) identity: ResultIdentity,
    pub(crate) legacy: DecisionResult,
    pub(crate) rank_position: Option<std::num::NonZeroUsize>,
}

impl Atomic {
    pub(crate) fn take_input(&mut self) -> Option<crate::core::Record> {
        self.legacy.input.take()
    }

    pub(crate) const fn question(&self) -> &crate::core::Question {
        &self.legacy.question
    }
    pub(crate) const fn threshold(&self) -> Option<crate::core::Threshold> {
        self.legacy.threshold
    }

    pub(crate) fn metadata(&self) -> crate::core::MetadataFields<'_> {
        self.legacy.meta.fields()
    }

    pub(crate) fn ranked(
        mut self,
        record: usize,
        position: std::num::NonZeroUsize,
    ) -> Result<Self, crate::core::RenderError> {
        self.legacy.threshold = None;
        self.rank_position = Some(position);
        self.identity = crate::core::ResultIdentity::of(
            crate::core::image::InputFunction::Rank,
            &crate::core::RecordScope { record },
            self.identity.question_sources().to_vec(),
            self.identity.observations().to_vec(),
            &crate::core::AtomicReading {
                question: &self.legacy.question,
                threshold: None,
                rank_position: Some(position),
            },
            &[],
        )?;
        Ok(self)
    }

    pub(crate) const fn value(&self) -> &crate::core::Value {
        &self.legacy.value
    }

    pub(crate) const fn reported_usage(&self) -> Option<crate::core::ReportedUsage> {
        self.legacy.meta.reported_usage
    }

    pub(crate) const fn usage(&self) -> Option<Usage> {
        self.legacy.meta.usage
    }

    pub(crate) const fn answer(&self) -> &Answer {
        &self.legacy.answer
    }
}

impl Atomic {
    pub(crate) fn serialize_with_input<S: Serializer, T: Serialize>(
        &self,
        input: Option<&T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        self.serialize_with_named_input(input, None, serializer)
    }

    pub(crate) fn serialize_with_named_input<S: Serializer, T: Serialize>(
        &self,
        input: Option<&T>,
        question_name: Option<&str>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let row = &self.legacy;
        super::wire::AtomicDocument {
            schema: super::wire::Version::V2,
            answer_id: self.identity.answer_id(),
            value: self.rank_position.map_or_else(
                || super::wire::AtomicValue::Primitive(&row.value),
                super::wire::AtomicValue::Rank,
            ),
            input,
            question_name,
            question: crate::core::declaration::ReadableQuestion {
                question: &row.question,
                metadata: &self.declarations,
            },
            answer: &row.answer,
            threshold: row.threshold,
            meta: CompleteMeta::of(&row.meta, &self.identity),
        }
        .serialize(serializer)
    }
}

impl Serialize for Atomic {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_with_input(self.legacy.input.as_ref(), serializer)
    }
}
