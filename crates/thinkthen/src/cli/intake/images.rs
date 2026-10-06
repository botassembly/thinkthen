//! Explicit image inputs on the ordinary intake and question pipeline.
use super::{Data, Intake, Piece, Position, Source};
use crate::args::Common;
use crate::core::Reading;
use crate::failure::Failure;
use crate::public::{
    ImageEvidence, InputFileReader, InputReaderOptions, ReaderMedia, ReaderOptions, SourceItem,
    SourceItems, SourceUnit,
};
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
    if has_on {
        return Err(Failure::Usage("images cannot accompany saved on pointers"));
    }
    let source = if common.image.is_empty() {
        Source::Images(crate::public::read_inputs(&common.input, options()).map_err(refused)?)
    } else {
        // Attachments are one indivisible input: read and validate every file
        // before the first pipeline event. A directory never bundles images.
        let mut images = Vec::new();
        let mut names = Vec::new();
        let mut compressed = 0usize;
        for path in &common.image {
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
            compressed = compressed
                .checked_add(image.record.bytes().len())
                .filter(|&bytes| bytes <= crate::public::MAX_IMAGE_BYTES)
                .ok_or(Failure::Usage(
                    "image evidence exceeds the 25165824 compressed byte SDK limit",
                ))?;
            images.push(image.record);
            names.push(image.file);
        }
        let mut text = Vec::new();
        input
            .take((crate::core::MAX_RECORD_BYTES + 1) as u64)
            .read_to_end(&mut text)
            .map_err(Failure::Input)?;
        if text.len() > crate::core::MAX_RECORD_BYTES {
            return Err(Failure::Usage(
                "image ancillary text exceeds the 16777216 byte SDK limit",
            ));
        }
        let text = String::from_utf8(text)
            .map_err(|_| Failure::Usage("image ancillary text is not valid UTF-8"))?;
        let evidence =
            ImageEvidence::new((!text.is_empty()).then_some(text), images).map_err(refused)?;
        Source::Attached(Some(Piece {
            position: Some(Position {
                file: None,
                first: None,
                last: None,
                images: Some(names),
                source: 0,
                located: false,
            }),
            data: Data::Images(evidence),
            advance: 1,
        }))
    };
    Ok(Intake {
        sources: [source].into(),
        reading: reading.clone(),
        window: 1,
        global: 0,
    })
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
