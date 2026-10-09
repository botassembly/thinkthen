//! Caller context resolved independently for each recognition stage.
use serde::{Deserialize, Serialize};

/// Optional literal context for the three recognition stages.
#[derive(Clone, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RecognitionStageContext {
    /// Boundary context; empty text clears fallback.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub boundary: Option<String>,
    /// Shared context for kind and edge questions.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub kind_edge: Option<String>,
    /// Relation context.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub relation: Option<String>,
}
fn present<'de, D: serde::Deserializer<'de>>(de: D) -> Result<Option<String>, D::Error> {
    String::deserialize(de).map(Some)
}
impl RecognitionStageContext {
    /// Whether no stage override was supplied.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.boundary.is_none() && self.kind_edge.is_none() && self.relation.is_none()
    }
    pub(crate) fn overlay(&mut self, other: &Self) {
        for (saved, call) in [
            (&mut self.boundary, &other.boundary),
            (&mut self.kind_edge, &other.kind_edge),
            (&mut self.relation, &other.relation),
        ] {
            if call.is_some() {
                saved.clone_from(call);
            }
        }
    }
    pub(crate) fn effective(
        &self,
        stage: &str,
        fallback: Option<&crate::core::Json>,
    ) -> Option<crate::core::Json> {
        let selected = match stage {
            "boundary" => &self.boundary,
            "relation" => &self.relation,
            _ => &self.kind_edge,
        };
        match selected {
            Some(text) if text.is_empty() => None,
            Some(text) => Some(crate::core::Json::String(text.clone())),
            None => fallback.cloned(),
        }
    }
}
impl std::fmt::Debug for RecognitionStageContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RecognitionStageContext(<withheld>)")
    }
}
