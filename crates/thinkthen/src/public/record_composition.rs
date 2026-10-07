//! Shared original records, logical selection and physical provenance.
use crate::core;
use crate::public::{
    Error, ImageEvidence, ImageInput, InputEvidence, QuestionContent, QuestionInput, RecordInput,
    RecordOptions, SourceItem,
};
use serde::{Serialize, Serializer};
use std::fmt;
use std::sync::Arc;

/// Arbitrary original content, parsed once by the native ordered record parser.
#[derive(Clone, Eq, PartialEq)]
pub struct RawRecord(pub(crate) Arc<core::Record>);
impl RawRecord {
    pub(crate) fn retained_bytes(&self) -> Result<usize, Error> {
        self.literal().map_or_else(
            || {
                core::json_line(self)
                    .map(|text| text.len())
                    .map_err(|_| Error::defect("an original record could not be measured"))
            },
            |text| Ok(text.len()),
        )
    }
    /// Retain literal text, including strings that happen to contain JSON.
    /// # Errors
    /// Refuses invalid record size through the ordinary record reader.
    pub fn text(text: &str) -> Result<Self, Error> {
        Self::read(text, core::Framing::Document)
    }
    /// Retain original JSON member order, false, null and numeric values.
    /// # Errors
    /// Refuses invalid, duplicate, nonfinite, overdeep or oversized JSON.
    pub fn json(text: &str) -> Result<Self, Error> {
        Self::read(text, core::Framing::Jsonl)
    }
    fn read(text: &str, framing: core::Framing) -> Result<Self, Error> {
        let reading = core::Reading::new(framing, Vec::new()).map_err(Error::refused)?;
        Ok(Self(Arc::new(
            reading.record(text.as_bytes()).map_err(Error::refused)?,
        )))
    }
    /// Literal original text, absent for a parsed JSON record.
    #[must_use]
    pub fn literal(&self) -> Option<&str> {
        self.0.text()
    }
    /// Arbitrary original JSON, absent for a literal text record.
    #[must_use]
    pub fn content(&self) -> Option<QuestionContent<'_>> {
        self.0.json().map(QuestionContent)
    }
}
impl Serialize for RawRecord {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.as_ref().serialize(serializer)
    }
}
impl fmt::Debug for RawRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Physical location, separate from model evidence and all request identities.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct SourceLocation {
    file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_line: Option<usize>,
}
impl SourceLocation {
    pub(crate) fn image(file: String) -> Self {
        Self {
            file,
            first_line: None,
            last_line: None,
        }
    }
    /// Exact filename and optional one-based inclusive text coordinates.
    /// # Errors
    /// Refuses missing paired coordinates, zero or reversed line ranges.
    pub fn new(
        file: String,
        first_line: Option<usize>,
        last_line: Option<usize>,
    ) -> Result<Self, Error> {
        match (first_line, last_line) {
            (None, None) => {}
            (Some(first), Some(last)) if first > 0 && last >= first => {}
            _ => {
                return Err(Error::usage(
                    "source lines are a paired one-based inclusive range",
                ));
            }
        }
        Ok(Self {
            file,
            first_line,
            last_line,
        })
    }
    /// Exact authored source filename.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }
    /// First physical text line, absent for image files.
    #[must_use]
    pub const fn first_line(&self) -> Option<usize> {
        self.first_line
    }
    /// Last physical text line, absent for image files.
    #[must_use]
    pub const fn last_line(&self) -> Option<usize> {
        self.last_line
    }
}
impl fmt::Debug for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceLocation")
            .field("file", &"<withheld>")
            .field("first_line", &self.first_line)
            .field("last_line", &self.last_line)
            .finish()
    }
}

/// Immutable logical evidence, original content and explicit ancillary inputs.
/// Its private fields cannot introduce protocol or cache state.
#[derive(Clone, Eq, PartialEq)]
pub struct RecordEvidence {
    original: RawRecord,
    pub(crate) evidence: core::Evidence,
    pub(crate) value: core::Json,
    text: String,
    images: Option<ImageEvidence>,
    location: Option<SourceLocation>,
}
impl RecordEvidence {
    /// Whole original content, including fields excluded from model evidence.
    #[must_use]
    pub const fn original(&self) -> &RawRecord {
        &self.original
    }
    /// Selected logical content in native authored order.
    #[must_use]
    pub const fn selected(&self) -> QuestionContent<'_> {
        QuestionContent(&self.value)
    }
    /// Ordered immutable images, including duplicate attachments.
    #[must_use]
    pub fn images(&self) -> &[ImageInput] {
        self.images.as_ref().map_or(&[], ImageEvidence::images)
    }
    /// Physical provenance, absent when the caller supplied none.
    #[must_use]
    pub const fn location(&self) -> Option<&SourceLocation> {
        self.location.as_ref()
    }
    /// Attach a physical location without changing any logical request content.
    #[must_use]
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
    /// Attach explicitly validated images in authored order without transformation.
    /// # Errors
    /// Enforces the existing per-input compressed-byte, text and image count bounds.
    pub fn with_images(mut self, images: Vec<ImageInput>) -> Result<Self, Error> {
        self.images = Some(ImageEvidence::new(Some(self.text.clone()), images)?);
        Ok(self)
    }
    pub(crate) fn plain(&self) -> &str {
        &self.text
    }
    pub(crate) fn image_state(&self) -> Option<core::image::ImageState> {
        self.images.as_ref().map(|images| core::image::ImageState {
            text: self.value.clone(),
            images: images
                .images()
                .iter()
                .map(|image| image.0.clone())
                .collect(),
        })
    }
    pub(crate) fn selected_record(&self) -> core::Record {
        if self.original.literal().is_some() {
            self.original.0.as_ref().clone()
        } else {
            core::Record::from_json(self.value.clone())
        }
    }
    pub(crate) fn batch_record(&self) -> core::batch::BatchRecord {
        core::batch::BatchRecord {
            evidence: self.evidence.clone(),
            value: self.value.clone(),
        }
    }
}
impl InputEvidence for RecordEvidence {
    fn question_input(&self) -> QuestionInput {
        QuestionInput::Record(self.clone())
    }
}
impl Serialize for RecordEvidence {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.original.serialize(serializer)
    }
}
impl fmt::Debug for RecordEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecordEvidence")
            .field("content", &"<withheld>")
            .field("images", &self.images)
            .field("location", &self.location)
            .finish()
    }
}

/// Existing field selection plus separate context and candidate pointers.
#[derive(Clone)]
pub struct RecordReading {
    reading: core::Reading,
    context: Option<core::Pointer>,
    options: Option<core::Pointer>,
    context_schema: Option<super::InputDeclaration>,
}
impl RecordReading {
    /// Admit ordered pointers using the ordinary reader, including field-name clashes.
    /// # Errors
    /// Refuses malformed pointers or ambiguous selected member names.
    pub fn new(
        fields: &[&str],
        context: Option<&str>,
        options: Option<&str>,
    ) -> Result<Self, Error> {
        let pointer = |text: &str| core::Pointer::new(text).map_err(Error::refused);
        let fields = fields
            .iter()
            .map(|field| pointer(field))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            reading: core::Reading::new(core::Framing::Jsonl, fields).map_err(Error::refused)?,
            context: context.map(pointer).transpose()?,
            options: options.map(pointer).transpose()?,
            context_schema: None,
        })
    }
    /// Declare the selected per-item context's actual type before composition.
    /// The question independently admits it again before lookup or sends.
    #[must_use]
    pub fn with_context_schema(mut self, schema: super::InputDeclaration) -> Self {
        self.context_schema = Some(schema);
        self
    }
    /// Compose one original without inserting context, candidates or provenance into evidence.
    /// # Errors
    /// Refuses missing fields, blank evidence, invalid context or candidates.
    pub fn compose(&self, original: RawRecord) -> Result<RecordInput<RecordEvidence>, Error> {
        let selected = self
            .reading
            .batch_record(&original.0)
            .map_err(Error::refused)?;
        let context = self
            .context
            .as_ref()
            .map(|pointer| match self.context_schema.as_ref() {
                Some(schema) => {
                    let value = original.0.context_value(pointer).map_err(Error::refused)?;
                    super::RecordContext::selected(value, Some(schema))
                }
                None => original
                    .0
                    .context_text(pointer)
                    .map(super::RecordContext::from)
                    .map_err(Error::refused),
            })
            .transpose()?;
        let options = self
            .options
            .as_ref()
            .map(|pointer| {
                original
                    .0
                    .choices(pointer)
                    .map_err(Error::refused)
                    .and_then(|labels| RecordOptions::from_labels(&labels))
            })
            .transpose()?;
        let text = selected
            .evidence
            .as_text()
            .map_err(Error::refused)?
            .into_owned();
        Ok(RecordInput {
            original: RecordEvidence {
                original,
                evidence: selected.evidence,
                value: selected.value,
                text,
                images: None,
                location: None,
            },
            context,
            options,
        })
    }
    pub(crate) fn admit_images(&self) -> Result<(), Error> {
        if self.reading.has_fields() || self.context.is_some() || self.options.is_some() {
            return Err(Error::usage(
                "image source records have no field, context or options pointers",
            ));
        }
        Ok(())
    }
    /// Compose an item from the single native explicit reader without rereading a file.
    /// # Errors
    /// Refuses incompatible image pointers or ordinary record admission failures.
    pub fn compose_source(&self, item: SourceItem) -> Result<RecordInput<QuestionInput>, Error> {
        match item {
            SourceItem::Text(source) => {
                let original = if !self.reading.has_fields()
                    && self.context.is_none()
                    && self.options.is_none()
                {
                    RawRecord::text(&source.record)?
                } else {
                    RawRecord::json(&source.record)?
                };
                let mut record = self.compose(original)?;
                record.original = record.original.with_location(SourceLocation::new(
                    source.file,
                    Some(source.first_line),
                    Some(source.last_line),
                )?);
                Ok(RecordInput {
                    original: record.original.question_input(),
                    context: record.context,
                    options: record.options,
                })
            }
            SourceItem::Image(source) => {
                self.admit_images()?;
                Ok(RecordInput {
                    original: source.question_input(),
                    context: None,
                    options: None,
                })
            }
        }
    }
}

impl fmt::Debug for RecordReading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecordReading").finish_non_exhaustive()
    }
}
