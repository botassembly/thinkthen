//! Stable identity from validated facts already present in a historical source.

use serde::Serialize;

use crate::core::{ObservationId, RenderError};

#[derive(Serialize)]
pub(crate) struct Metadata<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) answered_by: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_tokens: Option<Option<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output_tokens: Option<Option<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) taken_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) origin: Option<&'a str>,
}

pub(crate) fn observation(
    key: &str,
    canonical_answer: &str,
    metadata: &Metadata<'_>,
) -> Result<ObservationId, RenderError> {
    let metadata = serde_json::to_string(metadata).map_err(|_| RenderError)?;
    let digest = super::framing::digest(
        "thinkthen.legacy-observation/1",
        &[
            key.as_bytes(),
            canonical_answer.as_bytes(),
            metadata.as_bytes(),
        ],
    );
    ObservationId::new(crate::core::hex(&digest)).map_err(|_| RenderError)
}
