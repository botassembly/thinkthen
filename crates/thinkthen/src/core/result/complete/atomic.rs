//! Complete atomic judgments share the frozen legacy answer representation.

use serde::{Serialize, Serializer};

use super::{CompleteMeta, ResultIdentity};
use crate::core::{Answer, DecisionResult, Usage};
mod set_rank;

/// The complete canonical document for decide, choose, tag, score, filter or rank.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Atomic {
    pub(crate) function: crate::core::image::InputFunction,
    pub(crate) declarations: crate::core::declaration::QuestionMetadata,
    pub(crate) identity: ResultIdentity,
    pub(crate) legacy: DecisionResult,
    pub(crate) rank_position: Option<std::num::NonZeroUsize>,
    pub(crate) source: Option<super::wire::PhysicalSource>,
    pub(crate) images: Option<Vec<crate::core::image::Image>>,
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
        self.serialize_occurrence(input, question_name, None, serializer)
    }

    pub(crate) fn serialize_occurrence<S: Serializer, T: Serialize>(
        &self,
        input: Option<&T>,
        question_name: Option<&str>,
        index: Option<usize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        self.serialize_occurrence_members(input, question_name, index, None, serializer)
    }

    pub(crate) fn serialize_occurrence_members<'a, S: Serializer, T: Serialize>(
        &'a self,
        input: Option<&'a T>,
        question_name: Option<&'a str>,
        index: Option<usize>,
        members: Option<Vec<super::wire::RankMemberDocument<'a>>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        use crate::core::Value;
        use crate::core::image::InputFunction;
        if let Some(position) = self.rank_position {
            return self
                .document(input, question_name, index, members, position)
                .serialize(serializer);
        }
        match (self.function, &self.legacy.value) {
            (InputFunction::Decide, Value::YesNo(value)) => {
                let meaning = match (&self.legacy.question, value) {
                    (crate::core::Question::Decide { yes, .. }, Some(true)) => yes.as_ref(),
                    (crate::core::Question::Decide { no, .. }, Some(false)) => no.as_ref(),
                    _ => None,
                };
                let value = meaning.map_or(
                    super::wire::DecideValue::Primitive(*value),
                    super::wire::DecideValue::Authored,
                );
                self.document(input, question_name, index, members, value)
                    .serialize(serializer)
            }
            (InputFunction::Choose, Value::Choice(value)) => self
                .document(input, question_name, index, members, value.as_deref())
                .serialize(serializer),
            (InputFunction::Tag, Value::Tag(value)) => self
                .document(input, question_name, index, members, value.as_slice())
                .serialize(serializer),
            (InputFunction::Score, Value::Score(value)) => self
                .document(input, question_name, index, members, *value)
                .serialize(serializer),
            (InputFunction::Filter, Value::YesNo(Some(value))) => self
                .document(input, question_name, index, members, *value)
                .serialize(serializer),
            _ => self
                .document(input, question_name, index, members, &self.legacy.value)
                .serialize(serializer),
        }
    }

    fn document<'a, T: Serialize, V: Serialize>(
        &'a self,
        input: Option<&'a T>,
        question_name: Option<&'a str>,
        index: Option<usize>,
        members: Option<Vec<super::wire::RankMemberDocument<'a>>>,
        value: V,
    ) -> super::wire::AtomicDocument<'a, T, V> {
        let row = &self.legacy;
        super::wire::AtomicDocument {
            schema: super::wire::Version::V2,
            answer_id: self.identity.answer_id(),
            value,
            input,
            index,
            images: self.images.as_deref(),
            source: self.source.as_ref(),
            question_name,
            members,
            question: crate::core::declaration::ReadableQuestion {
                question: &row.question,
                metadata: &self.declarations,
            },
            answer: &row.answer,
            threshold: row.threshold,
            meta: CompleteMeta::of(&row.meta, &self.identity),
        }
    }
}

impl Serialize for Atomic {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_with_input(self.legacy.input.as_ref(), serializer)
    }
}
