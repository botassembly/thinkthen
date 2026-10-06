//! Complete whole-set find keeps its dedicated question and ordered distribution.

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use super::{CompleteMeta, ResultIdentity};
use crate::core::{FindResult, Usage};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Find {
    pub(crate) identity: ResultIdentity,
    pub(crate) legacy: FindResult,
}

impl Find {
    pub(crate) fn confidence(&self) -> Option<f64> {
        self.legacy.answer.confidence()
    }

    pub(crate) const fn usage(&self) -> Option<Usage> {
        self.legacy.meta.usage
    }
}

impl Serialize for Find {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let row = &self.legacy;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("schema", "thinkthen.result/2")?;
        map.serialize_entry("answer_id", self.identity.answer_id())?;
        map.serialize_entry("value", &row.value)?;
        map.serialize_entry("question", &row.question)?;
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
