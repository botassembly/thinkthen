//! The closed JSON settings object for the C constructor.

use std::num::NonZeroUsize;

use serde_json::Value;
use thinkthen::{BatchSetting, Engine, EngineBuilder};

use crate::failures::Failure;

pub(crate) fn batch(value: &Value) -> Result<BatchSetting, Failure> {
    if value.as_str() == Some("max") {
        return Ok(BatchSetting::Max);
    }
    value
        .as_u64()
        .and_then(|count| usize::try_from(count).ok())
        .and_then(NonZeroUsize::new)
        .map(BatchSetting::Records)
        .ok_or_else(|| Failure::usage("batch takes max or a whole number of at least 1"))
}

pub(crate) fn build(text: &str) -> Result<Engine, Failure> {
    EngineBuilder::from_settings_json(text)?
        .build()
        .map_err(Failure::from)
}
