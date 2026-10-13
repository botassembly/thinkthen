//! The closed JSON settings object for the C constructor.

use thinkthen::{Engine, EngineBuilder};

use crate::failures::Failure;

pub(crate) fn build(text: &str) -> Result<Engine, Failure> {
    EngineBuilder::from_settings_json(text)?
        .build()
        .map_err(Failure::from)
}
