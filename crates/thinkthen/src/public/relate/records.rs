//! Saved relation record fields remain typed through ordinary native preparation.
use super::Relate;
use crate::public::Error;
use std::path::Path;
impl Relate {
    pub(crate) fn validate_pairs(&self, pairs: &[(String, String)]) -> Result<(), Error> {
        for (at, (name, kind)) in pairs.iter().enumerate() {
            let record = crate::core::Record::from_json(crate::core::Json::Object(vec![
                ("name".to_owned(), crate::core::Json::String(name.clone())),
                ("kind".to_owned(), crate::core::Json::String(kind.clone())),
            ]));
            record
                .validate_item(self.0.metadata.item_schema.as_ref())
                .map_err(|error| Error::refused(error).at_record(at))?;
        }
        Ok(())
    }

    /// Read a saved relation plan whose field pointers will select native records.
    /// # Errors
    /// Refuses invalid fields, rules and saved settings through the ordinary grammar.
    pub fn from_records_json(value: &str) -> Result<Self, Error> {
        crate::core::RelateSpec::parse(value)
            .map(Self)
            .map_err(Error::refused)
    }
    /// Load a saved record relation plan with the ordinary bounded question reader.
    /// # Errors
    /// Unreadable or invalid saved plans return Local before execution.
    pub fn load_records(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = crate::public::question_file::load_text(path.as_ref(), "relate file")?;
        Self::from_records_json(&text).map_err(|error| Error::local(error.detail().message()))
    }
    /// Override the exact native name and kind pointers for record execution.
    /// # Errors
    /// Refuses malformed RFC 6901 pointers.
    pub fn record_fields(mut self, name: &str, kind: &str) -> Result<Self, Error> {
        self.0
            .override_fields(Some(name), Some(kind))
            .map_err(Error::refused)?;
        Ok(self)
    }
}
