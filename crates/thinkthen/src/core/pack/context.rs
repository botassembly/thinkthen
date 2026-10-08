//! Shared aggregate context surrounds the existing typed state without changing evidence.
use super::State;
use crate::core::adapters::built_in::EncodeError;
use serde::Serialize;

#[derive(Serialize)]
struct Context<'a> {
    context: &'a crate::core::Json,
    evidence: crate::core::Json,
}

#[derive(Serialize)]
struct Examples<'a> {
    examples: &'a [String],
    evidence: crate::core::Json,
}

impl State {
    pub(crate) fn with_examples(&self, examples: &[String]) -> Result<Self, EncodeError> {
        if examples.is_empty() {
            return Ok(self.clone());
        }
        let evidence =
            serde_json::from_str(self.json()).map_err(|error| EncodeError::of(&error))?;
        let json = serde_json::to_string(&Examples { examples, evidence })
            .map_err(|error| EncodeError::of(&error))?;
        let bytes = json.len();
        Ok(Self::new(json, bytes).with_api(self.api()))
    }

    pub(crate) fn with_context_value(
        &self,
        context: &crate::core::Json,
    ) -> Result<Self, EncodeError> {
        let evidence =
            serde_json::from_str(self.json()).map_err(|error| EncodeError::of(&error))?;
        let json = serde_json::to_string(&Context { context, evidence })
            .map_err(|error| EncodeError::of(&error))?;
        let bytes = json.len();
        Ok(Self::new(json, bytes).with_api(self.api()))
    }
}
