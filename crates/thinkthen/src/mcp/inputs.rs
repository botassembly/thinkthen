//! Explicit finite descriptors reuse native composition and authorized readers.
use super::admission::{Options, Source};
use crate::{
    Error, ImageInput, ImageMedia, InputEvidence, QuestionInput, RawRecord, RecordInput,
    RecordReading,
};
use serde::Deserialize;
use serde_json::value::RawValue;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Descriptor {
    pub(super) text: Option<String>,
    #[serde(default, deserialize_with = "raw")]
    pub(super) json: Option<Box<RawValue>>,
    pub(super) source: Option<Source>,
    #[serde(default)]
    pub(super) images: Vec<Attachment>,
    #[serde(default, deserialize_with = "raw")]
    pub(super) context: Option<Box<RawValue>>,
    #[serde(default, deserialize_with = "raw")]
    pub(super) options: Option<Box<RawValue>>,
}
#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum Attachment {
    Path(PathBuf),
    Declared(DeclaredImage),
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DeclaredImage {
    path: PathBuf,
    media: ImageMedia,
}
impl Attachment {
    fn path(&self) -> &Path {
        match self {
            Self::Path(path) => path,
            Self::Declared(image) => &image.path,
        }
    }
    fn read(&self) -> Result<ImageInput, Error> {
        let options = crate::InputReaderOptions {
            reading: crate::ReaderOptions {
                unit: crate::SourceUnit::File,
                window: None,
            },
            media: crate::ReaderMedia::Image,
        };
        let mut source = crate::read_inputs([self.path()], options)?;
        let Some(crate::SourceItem::Image(image)) = source.next().transpose()? else {
            return Err(Error::usage("an attachment requires one image file"));
        };
        if source.next().transpose()?.is_some() {
            return Err(Error::usage("an attachment requires one image file"));
        }
        match self {
            Self::Path(_) => Ok(image.record),
            Self::Declared(declared) => ImageInput::new(declared.media, image.record.bytes()),
        }
    }
}
impl Descriptor {
    /// Header conflicts and unsupported media are refused before any file is opened.
    pub(super) fn admit(&self, tool: super::tools::Tool, options: &Options) -> Result<(), Error> {
        let originals = usize::from(self.text.is_some())
            + usize::from(self.json.is_some())
            + usize::from(self.source.is_some());
        if originals > 1 || (originals == 0 && self.images.is_empty()) {
            return Err(Error::usage(
                "an input requires one of text, json or source, or images",
            ));
        }
        if self.images.len() > crate::MAX_IMAGES {
            return Err(Error::usage("image evidence requires 1 to 8 images"));
        }
        if !self.images.is_empty() && !tool.images() {
            return Err(Error::usage("this function takes text only"));
        }
        if self
            .images
            .iter()
            .any(|image| image.path().as_os_str().is_empty())
        {
            return Err(Error::usage("explicit file paths must not be empty"));
        }
        if let Some(source) = &self.source {
            source.options().validate()?;
            if source.paths.is_empty()
                || source.paths.iter().any(|path| path.as_os_str().is_empty())
                || source.media != crate::ReaderMedia::Text
                || source.reading.unit != crate::SourceUnit::File
            {
                return Err(Error::usage(
                    "an input source requires explicit whole text files",
                ));
            }
        }
        if (self.context.is_some() && options.context_field.is_some())
            || (self.options.is_some() && options.options_field.is_some())
        {
            return Err(Error::usage(
                "explicit input context/options conflict with projection pointers",
            ));
        }
        if self.options.is_some() && tool != super::tools::Tool::Choose {
            return Err(Error::usage("input options apply only to choose"));
        }
        Ok(())
    }
    pub(super) fn compose(
        &self,
        reading: &RecordReading,
        schema: Option<&crate::InputDeclaration>,
    ) -> Result<RecordInput<QuestionInput>, Error> {
        if self.text.is_none() && self.json.is_none() && self.source.is_none() {
            reading.admit_images()?;
        }
        let images = self
            .images
            .iter()
            .map(Attachment::read)
            .collect::<Result<Vec<_>, _>>()?;
        let mut row = if let Some(source) = &self.source {
            let mut items = crate::read_inputs(&source.paths, source.options())?;
            let item = items
                .next()
                .transpose()?
                .ok_or_else(|| Error::usage("an input source requires exactly one item"))?;
            if items.next().transpose()?.is_some() {
                return Err(Error::usage("an input source requires exactly one item"));
            }
            reading.compose_source(item)?
        } else if let Some(json) = &self.json {
            let row = reading.compose(RawRecord::json(json.get())?)?;
            RecordInput {
                examples: None,
                seed_spans: None,
                original: row.original.question_input(),
                context: row.context,
                options: row.options,
            }
        } else if let Some(text) = &self.text {
            let row = reading.compose(RawRecord::text(text)?)?;
            RecordInput {
                examples: None,
                seed_spans: None,
                original: row.original.question_input(),
                context: row.context,
                options: row.options,
            }
        } else {
            RecordInput {
                examples: None,
                seed_spans: None,
                original: QuestionInput::Images(crate::ImageEvidence::new(None, images.clone())?),
                context: None,
                options: None,
            }
        };
        if !images.is_empty()
            && let QuestionInput::Record(record) = row.original
        {
            row.original = QuestionInput::Record(record.with_images(images)?);
        }
        if let Some(context) = &self.context {
            let value = crate::core::Json::parse(context.get()).map_err(Error::refused)?;
            row.context = Some(crate::RecordContext::selected(&value, schema)?);
        }
        if let Some(options) = &self.options {
            row.options = Some(crate::RecordOptions::project(options.get(), "")?);
        }
        Ok(row)
    }
}
fn raw<'de, D: serde::Deserializer<'de>>(de: D) -> Result<Option<Box<RawValue>>, D::Error> {
    Box::<RawValue>::deserialize(de).map(Some)
}
