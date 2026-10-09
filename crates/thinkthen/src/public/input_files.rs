//! Additive image media on the single explicit manifest/open iterator.

use super::{
    Error, FileReader, ImageEvidence, ImageInput, ImageMedia, InputEvidence, QuestionInput,
    ReaderOptions, SourceRecord, SourceUnit,
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::io::{BufRead, Read};
use std::path::{Path, PathBuf};

/// Explicit reader media. Ordinary file reading remains text.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum ReaderMedia {
    /// Existing line/window/file text reader.
    #[default]
    Text,
    /// One validated image per explicit whole file.
    Image,
}
/// Additive reader options; existing ReaderOptions literals remain valid.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InputReaderOptions {
    /// Existing physical-unit controls.
    #[serde(default)]
    pub reading: ReaderOptions,
    /// Explicit content media.
    #[serde(default)]
    pub media: ReaderMedia,
}
impl InputReaderOptions {
    /// Validate media/unit combinations before opening any content.
    /// # Errors
    /// Image mode requires file units without windows.
    pub fn validate(self) -> Result<Self, Error> {
        self.reading.validate()?;
        if self.media == ReaderMedia::Image && self.reading.unit != SourceUnit::File {
            return Err(Error::usage(
                "image reading requires unit file without a window",
            ));
        }
        Ok(self)
    }
}

/// Located image without invented text line positions.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ImageSourceRecord {
    /// Original validated image bytes and media.
    pub record: ImageInput,
    /// Explicit physical source name; never model evidence or cache identity.
    pub file: String,
}
/// One item from an extended explicit reader.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SourceItem {
    /// Existing located text record.
    Text(SourceRecord<String>),
    /// Whole-file image with no line positions.
    Image(ImageSourceRecord),
}
impl InputEvidence for SourceItem {
    fn question_input(&self) -> QuestionInput {
        match self {
            Self::Text(text) => QuestionInput::Text(text.record.clone()),
            Self::Image(image) => image.question_input(),
        }
    }
}
impl InputEvidence for ImageSourceRecord {
    fn question_input(&self) -> QuestionInput {
        QuestionInput::Images(ImageEvidence::one(self.record.clone()).located(self.file.clone()))
    }
}

/// Authorized handle reader over the same text reader or a bounded image decode.
pub struct InputFileReader<R: BufRead>(Mode<R>);

enum Mode<R: BufRead> {
    Text(FileReader<R>),
    Image {
        file: String,
        reader: Option<R>,
        limit: usize,
    },
}
impl<R: BufRead> std::fmt::Debug for InputFileReader<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputFileReader").finish_non_exhaustive()
    }
}
impl<R: BufRead> InputFileReader<R> {
    /// Read an authorized handle using explicit validated media/options.
    /// # Errors
    /// Returns Usage for contradictory media/unit controls.
    pub fn new(
        file: impl Into<String>,
        reader: R,
        options: InputReaderOptions,
    ) -> Result<Self, Error> {
        let options = options.validate()?;
        let file = file.into();
        Ok(Self(match options.media {
            ReaderMedia::Text => Mode::Text(FileReader::new(file, reader, options.reading)?),
            ReaderMedia::Image => Mode::Image {
                file,
                reader: Some(reader),
                limit: super::MAX_IMAGE_BYTES,
            },
        }))
    }
}
impl<R: BufRead> Iterator for InputFileReader<R> {
    type Item = Result<SourceItem, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Mode::Text(reader) => reader.next().map(|record| record.map(SourceItem::Text)),
            Mode::Image {
                file,
                reader,
                limit,
            } => Some(read_image(reader.take()?, *limit).map(|record| {
                SourceItem::Image(ImageSourceRecord {
                    record,
                    file: file.clone(),
                })
            })),
        }
    }
}

fn read_image(reader: impl Read, limit: usize) -> Result<ImageInput, Error> {
    let mut bytes = Vec::new();
    reader
        .take((limit + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::local("source image file could not be read"))?;
    if bytes.len() > super::MAX_IMAGE_BYTES {
        return Err(Error::usage(
            "image exceeds the 25165824 compressed byte SDK limit",
        ));
    }
    if bytes.len() > limit {
        return Err(attachment_budget_error());
    }
    let media = match image::guess_format(&bytes).ok() {
        Some(image::ImageFormat::Jpeg) => ImageMedia::Jpeg,
        Some(image::ImageFormat::Png) => ImageMedia::Png,
        _ => {
            return Err(Error::usage(
                "image reader supports validated JPEG and PNG only",
            ));
        }
    };
    ImageInput::new(media, bytes)
}

/// One bounded manifest, opening one regular file at a time in existing order.
#[derive(Debug)]
pub struct SourceItems {
    paths: VecDeque<(PathBuf, String)>,
    options: InputReaderOptions,
    current: Option<InputFileReader<std::io::BufReader<std::fs::File>>>,
    stopped: bool,
    image_bytes_remaining: Option<usize>,
}
/// Select explicit paths/folders with media specified separately from evidence.
/// # Errors
/// Returns errors for invalid controls or manifest enumeration failures.
pub fn read_inputs(
    paths: impl IntoIterator<Item = impl AsRef<Path>>,
    options: InputReaderOptions,
) -> Result<SourceItems, Error> {
    let options = options.validate()?;
    let paths = super::enumerate_files(paths)?
        .into_iter()
        .map(|path| super::files::source_name(&path).map(|name| (path, name)))
        .collect::<Result<_, _>>()?;
    Ok(SourceItems {
        paths,
        options,
        current: None,
        stopped: false,
        image_bytes_remaining: None,
    })
}
fn attachment_budget_error() -> Error {
    Error::usage("retained attachments exceed the input byte ceiling")
}
impl SourceItems {
    /// Internal transport admission leaves ordinary native readers unchanged.
    #[cfg(feature = "cli")]
    pub(crate) fn bounded_images(
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
        options: InputReaderOptions,
        remaining: usize,
    ) -> Result<Self, Error> {
        if options.media == ReaderMedia::Image && remaining == 0 {
            return Err(attachment_budget_error());
        }
        let mut source = read_inputs(paths, options)?;
        source.image_bytes_remaining = Some(remaining);
        Ok(source)
    }

    pub(crate) fn image_bytes_remaining(&self) -> usize {
        self.image_bytes_remaining.unwrap_or(usize::MAX)
    }

    fn charge_image(&mut self, item: SourceItem) -> Result<SourceItem, Error> {
        if let SourceItem::Image(image) = &item
            && let Some(remaining) = &mut self.image_bytes_remaining
        {
            *remaining = remaining
                .checked_sub(image.record.bytes().len())
                .ok_or_else(attachment_budget_error)?;
        }
        Ok(item)
    }

    fn open(
        &self,
        path: &Path,
        name: String,
    ) -> Result<InputFileReader<std::io::BufReader<std::fs::File>>, Error> {
        let file = super::files::open_regular(path)
            .map_err(|_| Error::local("source file could not be opened"))?;
        let mut reader = InputFileReader::new(name, std::io::BufReader::new(file), self.options)?;
        if let Mode::Image { limit, .. } = &mut reader.0 {
            *limit = (*limit).min(self.image_bytes_remaining());
        }
        Ok(reader)
    }
}
impl Iterator for SourceItems {
    type Item = Result<SourceItem, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }
        loop {
            if let Some(item) = self.current.as_mut().and_then(Iterator::next) {
                let item = item.and_then(|item| self.charge_image(item));
                self.stopped = item.is_err();
                return Some(item);
            }
            self.current = None;
            let (path, name) = self.paths.pop_front()?;
            if self.options.media == ReaderMedia::Image && self.image_bytes_remaining == Some(0) {
                self.stopped = true;
                return Some(Err(attachment_budget_error()));
            }
            self.current = match self.open(&path, name) {
                Ok(reader) => Some(reader),
                Err(error) => {
                    self.stopped = true;
                    return Some(Err(error));
                }
            };
        }
    }
}
