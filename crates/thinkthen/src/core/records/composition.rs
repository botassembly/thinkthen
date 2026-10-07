//! Shared typed record composition, over the ordinary record and pointer reader.
use super::{Held, Reading, Record, RecordError, Selected, found};
use crate::core::{Json, Pointer};

impl Record {
    pub(crate) fn validate_item(
        &self,
        schema: Option<&crate::core::InputDeclaration>,
    ) -> Result<(), RecordError> {
        let Some(schema) = schema else {
            return Ok(());
        };
        let valid = match &self.0 {
            Held::Json(value) => schema.accepts(value),
            Held::Text(text) => schema.accepts(&Json::String(text.clone())),
        };
        if valid {
            Ok(())
        } else {
            Err(RecordError::ItemSchema)
        }
    }

    pub(crate) fn from_json(value: Json) -> Self {
        Self(Held::Json(value))
    }

    pub(crate) fn context_text(&self, pointer: &Pointer) -> Result<&str, RecordError> {
        self.context_value(pointer)?
            .as_str()
            .ok_or_else(|| RecordError::ContextText(pointer.as_str().to_owned()))
    }

    pub(crate) fn context_value(&self, pointer: &Pointer) -> Result<&Json, RecordError> {
        let Held::Json(value) = &self.0 else {
            return Err(RecordError::TextHasNoMembers);
        };
        found(pointer, value)
    }

    pub(crate) fn text(&self) -> Option<&str> {
        match &self.0 {
            Held::Text(text) => Some(text),
            Held::Json(_) => None,
        }
    }

    pub(crate) fn json(&self) -> Option<&Json> {
        match &self.0 {
            Held::Text(_) => None,
            Held::Json(value) => Some(value),
        }
    }
}

impl Reading {
    pub(crate) fn has_fields(&self) -> bool {
        !self.fields.is_empty()
    }
}

impl Reading {
    pub(crate) fn with_item_schema(
        mut self,
        schema: Option<crate::core::InputDeclaration>,
    ) -> Self {
        self.item_schema = schema.into_iter().collect();
        self
    }
    pub(crate) fn with_item_schemas(mut self, schemas: Vec<crate::core::InputDeclaration>) -> Self {
        self.item_schema = schemas;
        self
    }
    pub(crate) const fn declares_item(&self) -> bool {
        !self.item_schema.is_empty()
    }
    pub(super) fn selected<'a>(&self, record: &'a Record) -> Result<Selected<'a>, RecordError> {
        let selected = self.selected_unchecked(record)?;
        if !self.item_schema.is_empty() {
            let value = match &selected {
                Selected::Text(text) => Json::String((*text).to_owned()),
                Selected::Whole(value) => (*value).clone(),
                Selected::Chosen(value) => value.clone(),
            };
            if self
                .item_schema
                .iter()
                .any(|schema| !schema.accepts(&value))
            {
                return Err(RecordError::ItemSchema);
            }
        }
        Ok(selected)
    }
}
