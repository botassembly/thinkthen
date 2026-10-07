//! Counted binary image boundary; borrowed ranges never reach a worker.

use std::ffi::c_void;
use std::io::{self, BufReader, Read};
use thinkthen::{ImageEvidence, ImageInput, ImageMedia, QuestionInput};

use super::{BridgeSettings, BridgeStop, BridgeText, Reply, reply_boundary, text};
use crate::{engines, errors::RowError};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct BridgeImage {
    media: BridgeText,
    data: BridgeText,
}

fn image(media: &str, data: &BridgeText) -> Result<ImageInput, String> {
    if data.len > thinkthen::MAX_IMAGE_BYTES || data.bytes.is_null() {
        return Err(RowError::usage("invalid counted image bytes or exceeded SDK limit").text);
    }
    let media = match media {
        "image/png" => ImageMedia::Png,
        "image/jpeg" => ImageMedia::Jpeg,
        _ => return Err(RowError::usage("image media must be image/png or image/jpeg").text),
    };
    // SAFETY: C++ retains data.len readable bytes for the synchronous call.
    let bytes = unsafe { std::slice::from_raw_parts(data.bytes, data.len) };
    ImageInput::new(media, bytes).map_err(|error| RowError::from(error).text)
}

fn input(
    images: *const BridgeImage,
    count: usize,
    ancillary: &BridgeText,
) -> Result<QuestionInput, String> {
    if !(1..=thinkthen::MAX_IMAGES).contains(&count) || images.is_null() {
        return Err(RowError::usage("image evidence requires 1 to 8 images").text);
    }
    // SAFETY: C++ retains this array of count BridgeImage entries through return.
    let images = unsafe { std::slice::from_raw_parts(images, count) };
    if images
        .iter()
        .try_fold(0usize, |sum, held| sum.checked_add(held.data.len))
        .is_none_or(|sum| sum > thinkthen::MAX_IMAGE_BYTES)
    {
        return Err(RowError::usage(
            "image evidence exceeds the 25165824 compressed byte SDK limit",
        )
        .text);
    }
    let images = images
        .iter()
        .map(|held| image(text(held.media.bytes, held.media.len)?, &held.data))
        .collect::<Result<Vec<_>, _>>()?;
    let ancillary = if ancillary.bytes.is_null() {
        None
    } else {
        Some(text(ancillary.bytes, ancillary.len)?.to_owned())
    };
    ImageEvidence::new(ancillary, images)
        .map(QuestionInput::Images)
        .map_err(|error| RowError::from(error).text)
}

/// Validate explicit compressed bytes without engine construction or transport.
/// # Safety
/// Both ranges remain readable until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_image(media: BridgeText, data: BridgeText) -> Reply {
    reply_boundary(|| image(text(media.bytes, media.len)?, &data).map(|_| Vec::new()))
}

fn prepare(
    question: &str,
    from_file: bool,
    raw: &str,
    kind: i32,
) -> Result<(thinkthen::LoadedQuestion, thinkthen::Settings), String> {
    let kind = if kind == 2 {
        super::portable_aux::asking_kind(question, from_file, "image details")
            .map_err(|error| error.text)?
    } else {
        kind
    };
    if !matches!(kind, 0 | 4 | 5) {
        return Err(RowError::usage("image judgments take decide, choose or score").text);
    }
    let (_, question, call) = super::portable_many::parse(question, from_file, raw, kind)?;
    Ok((question, call))
}

/// Validate all local values before a DuckDB chunk sends any member.
/// # Safety
/// Caller retains every counted range and array until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_images(
    question: BridgeText,
    from_file: i32,
    images: *const BridgeImage,
    count: usize,
    ancillary: BridgeText,
    settings: BridgeText,
    kind: i32,
) -> Reply {
    reply_boundary(|| {
        prepare(
            text(question.bytes, question.len)?,
            from_file != 0,
            text(settings.bytes, settings.len)?,
            kind,
        )?;
        input(images, count, &ancillary).map(|_| Vec::new())
    })
}

/// One explicit comparison on the existing configured engine and worker.
/// # Safety
/// Every range and host stop callback remains live through return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_images(
    question: BridgeText,
    from_file: i32,
    images: *const BridgeImage,
    count: usize,
    ancillary: BridgeText,
    settings: BridgeText,
    kind: i32,
    due: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let raw = text(settings.bytes, settings.len)?;
        let (question, call) = prepare(
            text(question.bytes, question.len)?,
            from_file != 0,
            raw,
            kind,
        )?;
        let input = input(images, count, &ancillary)?;
        let mut held = super::asked(&session)?;
        if let Some(model) = serde_json::from_str::<serde_json::Value>(raw)
            .ok()
            .and_then(|value| value.get("model")?.as_str().map(str::to_owned))
        {
            held.model = Some(model);
        }
        let engine = engines::engine_for(&held, |path| super::probe(&session, path))?;
        let total = held.max_requests_total;
        let due = match call.deadline_ms() {
            None | Some(-1) => due,
            Some(value) if due < 0 => value,
            Some(value) => due.min(value),
        };
        let batch = if call.batch_max() {
            Some("max".to_owned())
        } else {
            call.batch_records()
                .map(|count| count.to_string())
                .or(super::batch(&session)?)
        };
        let context = call.context().map(str::to_owned);
        super::run_detached(stop, move |token| {
            let options =
                engines::options_for(due, &token, total, batch.as_deref(), context.as_deref())?;
            let details = if context.is_some() {
                engine
                    .details_input_many_with(&question, [input], options)
                    .next()
                    .ok_or_else(|| RowError::usage("image details returned no row").text)?
                    .map(|row| row.into_parts().1)
            } else {
                engine
                    .details_input_with(&question, &input, options)
                    .map(|call| call.into_value())
            }
            .map_err(|error| engines::call_error(error, total).text)?;
            Ok(details.to_json().into_bytes())
        })
    })
}

#[derive(Debug)]
struct HostRead {
    context: *mut c_void,
    read: extern "C" fn(*mut c_void, *mut u8, usize) -> i64,
}
impl Read for HostRead {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        usize::try_from((self.read)(self.context, bytes.as_mut_ptr(), bytes.len()))
            .ok()
            .filter(|&count| count <= bytes.len())
            .ok_or_else(|| io::Error::other("authorized image read failed"))
    }
}

/// Decode a DuckDB-authorized handle through the shared native reader.
/// # Safety
/// The host retains context and its no-throw read callback until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_image_read(
    context: *mut c_void,
    read: extern "C" fn(*mut c_void, *mut u8, usize) -> i64,
) -> Reply {
    reply_boundary(|| {
        let mut reader = thinkthen::InputFileReader::new(
            "",
            BufReader::new(HostRead { context, read }),
            thinkthen::InputReaderOptions {
                reading: thinkthen::ReaderOptions {
                    unit: thinkthen::SourceUnit::File,
                    window: None,
                },
                media: thinkthen::ReaderMedia::Image,
            },
        )
        .map_err(|error| RowError::from(error).text)?;
        let item = reader
            .next()
            .ok_or_else(|| RowError::usage("image reader returned no row").text)?
            .map_err(|error| RowError::from(error).text)?;
        let thinkthen::SourceItem::Image(image) = item else {
            return Err("thinkthen defect: image reader returned text".into());
        };
        let mut bytes = vec![match image.record.media() {
            ImageMedia::Png => 1,
            ImageMedia::Jpeg => 2,
        }];
        bytes.extend_from_slice(image.record.bytes());
        Ok(bytes)
    })
}
