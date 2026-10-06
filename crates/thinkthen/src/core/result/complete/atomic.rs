//! Complete atomic judgments share the frozen legacy answer representation.

use serde::ser::SerializeMap;
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
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("schema", "thinkthen.result/2")?;
        map.serialize_entry("answer_id", self.identity.answer_id())?;
        match self.rank_position {
            Some(position) => map.serialize_entry("value", &position)?,
            None => map.serialize_entry("value", &row.value)?,
        }
        if let Some(input) = input {
            map.serialize_entry("input", input)?;
        }
        if let Some(name) = question_name {
            map.serialize_entry("question_name", name)?;
        }
        map.serialize_entry(
            "question",
            &crate::core::declaration::ReadableQuestion {
                question: &row.question,
                metadata: &self.declarations,
            },
        )?;
        map.serialize_entry("answer", &row.answer)?;
        map.serialize_entry("threshold", &row.threshold)?;
        map.serialize_entry(
            "meta",
            &CompleteMeta {
                legacy: &row.meta,
                identity: &self.identity,
            },
        )?;
        map.end()
    }
}

impl Serialize for Atomic {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_with_input(self.legacy.input.as_ref(), serializer)
    }
}
