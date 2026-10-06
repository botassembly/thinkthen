//! Complete recognition uses the scheduler's typed stage distributions.

use serde::ser::SerializeMap;
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
    pub(crate) const fn reported_usage(&self) -> Option<crate::core::ReportedUsage> {
        self.meta.reported_usage
    }

    pub(crate) const fn usage(&self) -> Option<Usage> {
        self.meta.usage
    }
}

impl Serialize for Recognition {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("schema", "thinkthen.result/2")?;
        map.serialize_entry("answer_id", self.identity.answer_id())?;
        map.serialize_entry("value", &self.value)?;
        if let Some(input) = &self.input {
            map.serialize_entry("input", input)?;
        }
        map.serialize_entry("question", &self.question)?;
        map.serialize_entry("answer", &self.answer)?;
        map.serialize_entry(
            "meta",
            &CompleteMeta {
                legacy: &self.meta,
                identity: &self.identity,
            },
        )?;
        map.end()
    }
}
