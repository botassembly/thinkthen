//! Relation field extraction shares the existing ordered record and pointer reader.
use crate::core::{Pointer, Record, RecordError, RelateSpec};
impl RelateSpec {
    /// Recognized names use `text` when the default name member is absent.
    pub(crate) fn record_name<'a>(&self, record: &'a Record) -> Result<&'a str, RecordError> {
        match record.entity_text(self.name_field()) {
            Err(missed @ RecordError::Missed(_)) if self.name_field().as_str() == "/name" => {
                Pointer::new("/text")
                    .ok()
                    .and_then(|text| record.entity_text(&text).ok())
                    .ok_or(missed)
            }
            read => read,
        }
    }
    pub(crate) fn record_pair(&self, record: &Record) -> Result<(String, String), RecordError> {
        Ok((
            self.record_name(record)?.to_owned(),
            record.entity_text(self.kind_field())?.to_owned(),
        ))
    }
}
