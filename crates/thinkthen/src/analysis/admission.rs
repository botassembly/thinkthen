//! Admission preserves the shared typed row contract and exact case identities.
use super::{AuditOptions, invalid};
use crate::{AnyRow, BackendName, Error, FunctionName, OffsetUnit, Question, RecordId};
use serde::{Deserialize, Serialize};

/// One saved run's explicit question and input representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// Format version; currently one.
    pub version: u32,
    /// The complete admitted question, including thresholds and selected fields.
    pub question: Question,
    /// Optional execution name used by the original run.
    pub name: Option<String>,
    /// Identity emitted by the engine for this question and name.
    pub question_identity: String,
    /// Function discriminator, checked against the admitted question.
    pub function: FunctionName,
    /// Attempted backend.
    pub backend: BackendName,
    /// Actual saved model identifier; aliases are not immutable revisions.
    pub model: String,
    /// Caller-defined identity for the record/candidate presentation.
    pub input_representation: String,
    /// Convention used by saved find spans and gold spans.
    pub offset_unit: OffsetUnit,
    /// Complete expected case inventory, including cases without result rows.
    pub expected_cases: Vec<RecordId>,
}
impl Manifest {
    pub(super) fn validate(&self, options: AuditOptions) -> Result<(), Error> {
        if self.version != 1 || self.function != self.question.function() {
            return Err(invalid(
                "audit manifest version or function is incompatible",
            ));
        }
        if !matches!(
            self.function,
            FunctionName::Decide | FunctionName::Filter | FunctionName::Choose | FunctionName::Find
        ) {
            return Err(invalid("audit supports decide, filter, choose and find"));
        }
        if self.model.trim().is_empty() || self.input_representation.trim().is_empty() {
            return Err(invalid(
                "audit requires model and input representation metadata",
            ));
        }
        if self.expected_cases.len() > options.max_cases.get()
            || options.bins.get() > options.max_cases.get()
        {
            return Err(invalid(
                "audit inventory or bins exceed the configured case limit",
            ));
        }
        if self.question_identity
            != crate::contract_driver::identity(&self.question, self.name.as_deref())?
        {
            return Err(invalid(
                "manifest identity does not match its complete question and name",
            ));
        }
        Ok(())
    }
    pub(super) fn check_row(&self, row: &AnyRow) -> Result<(), Error> {
        let source = row.source();
        if row.function() != self.function
            || source.question_identity() != self.question_identity
            || source.name() != self.name.as_deref()
            || source.backend() != self.backend
            || source.model() != self.model
        {
            return Err(invalid(
                "saved row belongs to a different question, backend or model group",
            ));
        }
        Ok(())
    }
}
/// A label in the admitted domain of the selected function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum Gold {
    /// A decide/filter truth value (the target for yes probability).
    Boolean(bool),
    /// A choose label.
    Choice(String),
    /// A find span in the manifest's declared units, with an exclusive end
    /// except for the explicitly declared inclusive convention.
    Span {
        /// Beginning of the gold span.
        start: usize,
        /// End of the gold span.
        end: usize,
    },
    /// A find candidate, resolved through this key entry's explicit span map.
    Candidate {
        /// Exact candidate identity; strings and integers remain distinct.
        candidate_id: RecordId,
    },
    /// Explicit JSON null means a labeled find-none case.
    None(()),
}
impl Gold {
    pub(super) fn validate(&self, manifest: &Manifest) -> Result<(), Error> {
        let function = manifest.function;
        if let (Self::Choice(gold), Question::Choose(question)) = (self, &manifest.question)
            && !question
                .inner
                .options
                .iter()
                .any(|option| option.label() == gold)
        {
            return Err(invalid(
                "a gold choice is absent from the admitted question options",
            ));
        }
        if matches!(
            (function, self),
            (
                FunctionName::Decide | FunctionName::Filter,
                Self::Boolean(_)
            ) | (FunctionName::Choose, Self::Choice(_))
                | (FunctionName::Find, Self::None(()))
        ) || matches!((function,self), (FunctionName::Find, Self::Span { start,end }) if start <= end)
        {
            Ok(())
        } else {
            Err(invalid(
                "a gold label is outside the selected function's domain",
            ))
        }
    }
}
fn present<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<Gold>, D::Error> {
    Gold::deserialize(deserializer).map(Some)
}
/// One label joined by exact ID and question identity, never by input position.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoldCase {
    /// Exact string or integer case identifier.
    pub case_id: RecordId,
    /// Engine identity of the question this label grades.
    pub question_identity: String,
    /// Missing is unlabeled; explicit null is `Some(Gold::None(()))`.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    pub gold: Option<Gold>,
    /// Optional per-case candidate identities and spans for find gold.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<CandidateSpan>,
}

/// One candidate's exact identity and span in the manifest's offset unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateSpan {
    /// Candidate identifier, unique within its case.
    pub id: RecordId,
    /// Span beginning.
    pub start: usize,
    /// Span end, using the manifest's convention.
    pub end: usize,
}
impl GoldCase {
    pub(super) fn resolve(self, manifest: &Manifest) -> Result<Option<Gold>, Error> {
        let mut candidates = std::collections::BTreeMap::new();
        let mut spans = std::collections::BTreeSet::new();
        for candidate in self.candidates {
            let valid = if manifest.offset_unit == OffsetUnit::CodePointsInclusive {
                candidate.start > 0 && candidate.start <= candidate.end
            } else {
                candidate.start < candidate.end
            };
            if !valid
                || !spans.insert((candidate.start, candidate.end))
                || candidates
                    .insert(
                        super::id_key(&candidate.id)?,
                        (candidate.start, candidate.end),
                    )
                    .is_some()
            {
                return Err(invalid(
                    "find candidate spans require valid spans, unique IDs and unique spans",
                ));
            }
        }
        match self.gold {
            Some(Gold::Candidate { candidate_id }) if manifest.function == FunctionName::Find => {
                let (start, end) = candidates
                    .get(&super::id_key(&candidate_id)?)
                    .copied()
                    .ok_or_else(|| {
                        invalid("gold candidate ID is absent from its explicit span map")
                    })?;
                Ok(Some(Gold::Span { start, end }))
            }
            Some(Gold::Span { start, end }) => {
                let valid = if manifest.offset_unit == OffsetUnit::CodePointsInclusive {
                    start > 0 && start <= end
                } else {
                    start < end
                };
                if !valid {
                    return Err(invalid(
                        "gold span does not match the declared offset convention",
                    ));
                }
                Ok(Some(Gold::Span { start, end }))
            }
            gold => Ok(gold),
        }
    }
}

/// Decode one current typed row, including its optional caller-owned item.
/// # Errors
/// Refuses malformed rows, duplicate fields and unknown extra fields.
pub fn decode_row(json: &str) -> Result<AnyRow, Error> {
    let fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str::<Fields>(json)
            .map_err(|_| invalid("the saved result must be an object with unique field names"))?
            .0;
    let encoded =
        serde_json::to_string(&fields).map_err(|_| invalid("saved row encoding failed"))?;
    let row: AnyRow = serde_json::from_str(&encoded)
        .map_err(|_| invalid("the saved result does not match the current typed row contract"))?;
    Ok(row)
}

struct Fields(std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>);
impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Fields;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an object with unique field names")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Fields, A::Error> {
                let mut fields = std::collections::BTreeMap::new();
                while let Some((key, value)) =
                    map.next_entry::<String, Box<serde_json::value::RawValue>>()?
                {
                    if fields.insert(key, value).is_some() {
                        return Err(serde::de::Error::custom("duplicate row field"));
                    }
                }
                Ok(Fields(fields))
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
