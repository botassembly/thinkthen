//! Shared aggregate context surrounds the existing typed state without changing evidence.
use super::State;
use crate::core::adapters::built_in::EncodeError;
use serde::Serialize;

#[derive(Serialize)]
struct Context<'a> {
    context: &'a str,
    evidence: crate::core::Json,
}

impl State {
    pub(crate) fn with_context(&self, context: &str) -> Result<Self, EncodeError> {
        let evidence =
            serde_json::from_str(self.json()).map_err(|error| EncodeError::of(&error))?;
        let json = serde_json::to_string(&Context { context, evidence })
            .map_err(|error| EncodeError::of(&error))?;
        let bytes = json.len();
        Ok(Self::new(json, bytes))
    }
}
