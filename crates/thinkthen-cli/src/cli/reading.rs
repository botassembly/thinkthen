//! Command framing and presentation over native record admission.

use crate::failure::Failure;
use crate::{
    Error, InputDeclaration, RawRecord, RecordEvidence, RecordInput, RecordReading, RequestFraming,
};
use serde::Serialize;

#[derive(Clone)]
pub(crate) struct Reading {
    framing: RequestFraming,
    fields: Vec<String>,
    composition: RecordReading,
    default: bool,
    item_schemas: Vec<InputDeclaration>,
}

#[derive(Serialize)]
pub(crate) struct ReadingPlan<'a> {
    framing: RequestFraming,
    field: &'a [String],
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<&'static str>,
}

impl Reading {
    pub(crate) fn new(framing: RequestFraming, fields: Vec<String>) -> Result<Self, Failure> {
        if matches!(framing, RequestFraming::Lines) && !fields.is_empty() {
            return Err(Failure::Usage("--field: a text line has no members, so --lines takes no pointer"));
        }
        let pointers = fields.iter().map(String::as_str).collect::<Vec<_>>();
        let composition = RecordReading::new(&pointers, None, None)?;
        Ok(Self {
            framing,
            fields,
            composition,
            default: false,
            item_schemas: Vec::new(),
        })
    }

    pub(crate) fn by_default(mut self) -> Self {
        self.default = true;
        self
    }

    pub(crate) fn fields(&self) -> &[String] {
        &self.fields
    }

    pub(crate) const fn framing(&self) -> RequestFraming {
        self.framing
    }

    pub(crate) fn streams(&self) -> bool {
        !matches!(self.framing, RequestFraming::Document)
    }

    pub(crate) fn has_fields(&self) -> bool {
        !self.fields.is_empty()
    }

    pub(crate) fn with_item_schema(self, schema: Option<InputDeclaration>) -> Self {
        self.with_item_schemas(schema.into_iter().collect())
    }

    pub(crate) fn with_item_schemas(mut self, schemas: Vec<InputDeclaration>) -> Self {
        self.item_schemas = schemas;
        self
    }

    pub(crate) fn declares_item(&self) -> bool {
        !self.item_schemas.is_empty()
    }

    pub(crate) fn plan(&self) -> ReadingPlan<'_> {
        ReadingPlan {
            framing: self.framing,
            field: &self.fields,
            from: self.default.then_some("default"),
        }
    }

    pub(crate) fn record(&self, bytes: &[u8]) -> Result<RawRecord, Error> {
        self.composition.read_original(bytes, self.framing)
    }

    pub(crate) fn annotation_record(&self, bytes: &[u8]) -> Result<RawRecord, Error> {
        if !self.streams() && !self.has_fields() {
            self.composition.read_annotation_document(bytes)
        } else {
            self.record(bytes)
        }
    }

    pub(crate) fn compose(&self, original: RawRecord) -> Result<RecordInput<RecordEvidence>, Error> {
        self.composition.compose(original)
    }

    pub(crate) fn evidence(&self, original: &RawRecord) -> Result<String, Error> {
        let record = self.compose(original.clone())?;
        let selected = record.original.selected();
        selected
            .text()
            .map_or_else(|| selected.to_json(), |text| Ok(text.to_owned()))
    }

    /// Preserve authored spacing after native admission, removing only the framing ending.
    pub(crate) fn as_it_arrived<'a>(&self, bytes: &'a [u8]) -> Result<&'a str, Error> {
        let line = self.streams().then(|| bytes.strip_suffix(b"\n")).flatten();
        let bytes = line.map_or(bytes, |line| line.strip_suffix(b"\r").unwrap_or(line));
        std::str::from_utf8(bytes)
            .map_err(|_| Error::new(crate::ErrorKind::Local, "the record is not valid UTF-8"))
    }

    pub(crate) fn skips(&self, bytes: &[u8]) -> bool {
        matches!(self.framing, RequestFraming::Lines)
            && std::str::from_utf8(bytes).is_ok_and(|text| text.trim().is_empty())
    }
}
