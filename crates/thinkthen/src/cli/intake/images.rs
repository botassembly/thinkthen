//! Explicit image inputs on the ordinary intake and question pipeline.
use super::{Data, Intake, Item, Piece, Position, Source};
use crate::args::Common;
use crate::core::Reading;
use crate::failure::Failure;
use crate::public::{
    ImageEvidence, InputFileReader, InputReaderOptions, ReaderMedia, ReaderOptions, SourceItem,
    SourceItems, SourceUnit,
};
use crate::schedule::Placed;
use std::io::{BufReader, Read};

fn options() -> InputReaderOptions {
    InputReaderOptions {
        media: ReaderMedia::Image,
        reading: ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
    }
}
fn refused(error: crate::public::Error) -> Failure {
    if error.kind() == crate::public::ErrorKind::Local {
        Failure::OpenInput(std::io::Error::other(error.to_string()))
    } else {
        Failure::Image(error.to_string())
    }
}
pub(super) fn prepare(
    common: &Common,
    reading: &Reading,
    input: impl Read + Send + 'static,
    has_on: bool,
) -> Result<Intake, Failure> {
    if has_on && common.image.is_empty() {
        return Err(Failure::Usage("images cannot accompany saved on pointers"));
    }
    let source = if common.image.is_empty() {
        Source::Images(crate::public::read_inputs(&common.input, options()).map_err(refused)?)
    } else {
        Source::Attached(Attachment {
            paths: common.image.clone(),
            media: common.image_media.as_deref().map(|media| match media {
                "image/jpeg" => crate::public::ImageMedia::Jpeg,
                _ => crate::public::ImageMedia::Png,
            }),
            source: Box::new(Intake::text(common, reading, input, has_on, false)?.0),
            loaded: None,
        })
    };
    Ok(Intake {
        sources: [source].into(),
        reading: reading.clone(),
        window: 1,
        global: 0,
    })
}

/// Read only when the shared detached reader asks for its first item, so
/// cancellation can end the command even while the input handle waits for EOF.
pub(super) struct Attachment {
    paths: Vec<std::path::PathBuf>,
    media: Option<crate::public::ImageMedia>,
    source: Box<Intake>,
    loaded: Option<Loaded>,
}
impl Attachment {
    pub(super) fn next(&mut self) -> Option<Result<Item, Placed>> {
        if self.loaded.is_none() {
            match self.read() {
                Ok(images) => self.loaded = Some(images),
                Err(error) => return Some(Err(Placed::at(error, 1))),
            }
        }
        Some(self.source.next()?.and_then(|mut item| {
            let (images, names) = self.loaded.as_ref().ok_or_else(|| {
                Placed::at(Failure::Defect("attachments were not loaded"), item.at)
            })?;
            let text = self.caption(&item)?;
            item.images = Some(
                ImageEvidence::new(text, images.clone())
                    .map_err(|error| Placed::at(refused(error), item.at))?,
            );
            let position = item.position.get_or_insert(Position {
                file: None,
                first: None,
                last: None,
                images: None,
                source: 0,
                located: false,
            });
            position.images = Some(names.clone());
            // Attachments have image provenance and no invented image-line coordinates.
            if !position.located {
                position.first = None;
                position.last = None;
            }
            Ok(item)
        }))
    }

    fn caption(&self, item: &Item) -> Result<Option<String>, Placed> {
        let text = match &item.data {
            Data::Record(record) => self.source.reading.evidence(record),
            Data::Bytes(bytes) => self
                .source
                .reading
                .record(bytes)
                .and_then(|record| self.source.reading.evidence(&record)),
            Data::Images(_) => {
                return Err(Placed::at(
                    Failure::Defect("attachment caption is an image"),
                    item.at,
                ));
            }
        };
        let text = match &item.data {
            Data::Bytes(bytes) if bytes.is_empty() => None,
            _ => {
                let refused = |error| {
                    Placed::at(
                        Failure::record(error, self.source.reading.streams()),
                        item.at,
                    )
                };
                let evidence = text.map_err(refused)?;
                Some(
                    evidence
                        .as_text()
                        .map(|text| text.into_owned())
                        .map_err(|error| Placed::at(Failure::from(error), item.at))?,
                )
            }
        };
        Ok(text)
    }

    fn read(&self) -> Result<Loaded, Failure> {
        // Attachments are one indivisible input: read and validate every file
        // before the first pipeline event. A directory never bundles images.
        let mut images = Vec::new();
        let mut names = Vec::new();
        let mut compressed = 0usize;
        for path in &self.paths {
            let name = path.to_str().ok_or_else(|| {
                Failure::OpenInput(std::io::Error::other("source path is not valid UTF-8"))
            })?;
            let reader = crate::edge::source(Some(path), std::io::empty())?;
            let image = InputFileReader::new(name, BufReader::new(reader), options())
                .map_err(refused)?
                .next()
                .ok_or(Failure::Defect("an image reader has no item"))?
                .map_err(refused)?;
            let SourceItem::Image(image) = image else {
                return Err(Failure::Defect("an image reader returned text"));
            };
            let record = match self.media {
                Some(media) => {
                    crate::public::ImageInput::new(media, image.record.bytes()).map_err(refused)?
                }
                None => image.record,
            };
            compressed = compressed
                .checked_add(record.bytes().len())
                .filter(|&bytes| bytes <= crate::public::MAX_IMAGE_BYTES)
                .ok_or(Failure::Usage(
                    "image evidence exceeds the 25165824 compressed byte SDK limit",
                ))?;
            images.push(record);
            names.push(image.file);
        }
        Ok((images, names))
    }
}

pub(super) fn next(items: &mut SourceItems) -> Option<Result<Piece, Failure>> {
    Some(items.next()?.map_err(refused).and_then(|item| {
        let SourceItem::Image(image) = item else {
            return Err(Failure::Defect("an image reader returned text"));
        };
        Ok(Piece {
            position: Some(Position {
                file: Some(image.file),
                first: None,
                last: None,
                images: None,
                source: 0,
                located: true,
            }),
            data: Data::Images(ImageEvidence::new(None, vec![image.record]).map_err(refused)?),
            advance: 1,
        })
    }))
}

type Loaded = (Vec<crate::public::ImageInput>, Vec<String>);
