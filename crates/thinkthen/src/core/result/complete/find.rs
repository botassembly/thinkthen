//! Complete whole-set find keeps its dedicated question and ordered distribution.

use serde::ser::SerializeMap;
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
    pub(crate) fn profile(&self) -> Option<&str> {
        self.legacy.question.profile()
    }
    pub(crate) fn raw_pick(&self) -> &str {
        self.legacy.answer.pick()
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

impl Find {
    pub(crate) fn serialize_with_value<S: Serializer, T: Serialize>(
        &self,
        value: Option<&T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let row = &self.legacy;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("schema", "thinkthen.result/2")?;
        map.serialize_entry("answer_id", self.identity.answer_id())?;
        map.serialize_entry("value", &value)?;
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

impl Serialize for Find {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_with_value(self.legacy.value.as_ref(), serializer)
    }
}
