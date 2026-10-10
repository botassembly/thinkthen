//! Fixture settings delegate their grammar to the native builder.
use serde_json::{Map, Value};
use thinkthen::{Engine, EngineBuilder, Error, ErrorKind};
pub(super) fn engine(settings: Map<String, Value>) -> Result<Engine, Error> {
    let text = serde_json::to_string(&settings)
        .map_err(|_| Error::new(ErrorKind::Defect, "fixture settings could not be written"))?;
    EngineBuilder::from_settings_json(&text)?.build()
}
