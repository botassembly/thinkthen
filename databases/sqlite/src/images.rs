//! Explicit persistent binary image values. Every reload is natively validated.

use crate::catalog::Catalog;
use rusqlite::functions::{Context, FunctionFlags};
use rusqlite::types::{Value, ValueRef};
use thinkthen::{ImageAdmission, ImageEvidence, ImageInput, ImageMedia, Judgment, QuestionInput};

use crate::{Failure, ffi, guard, question, worker};

const IMAGE: &[u8] = b"thinkthen.image/1\0";
const IMAGES: &[u8] = b"thinkthen.images/1\0";

fn blob<'a>(value: ValueRef<'a>) -> Result<&'a [u8], Failure> {
    match value {
        ValueRef::Blob(bytes) => Ok(bytes),
        _ => Err(Failure::usage("explicit image values must be tagged BLOBs")),
    }
}

fn take<'a>(bytes: &mut &'a [u8], count: usize) -> Result<&'a [u8], Failure> {
    let (head, tail) = bytes
        .split_at_checked(count)
        .ok_or_else(|| Failure::usage("invalid stored image value"))?;
    *bytes = tail;
    Ok(head)
}

fn length(bytes: &mut &[u8]) -> Result<usize, Failure> {
    let word: [u8; 4] = take(bytes, 4)?
        .try_into()
        .map_err(|_| Failure::usage("invalid stored image length"))?;
    Ok(u32::from_le_bytes(word) as usize)
}

fn encode(image: &ImageInput, file: Option<&str>) -> Result<Vec<u8>, Failure> {
    let file = file.unwrap_or_default().as_bytes();
    let count =
        u32::try_from(file.len()).map_err(|_| Failure::usage("image file name is too long"))?;
    let mut out = IMAGE.to_vec();
    out.push(match image.media() {
        ImageMedia::Png => 1,
        ImageMedia::Jpeg => 2,
    });
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(file);
    out.extend_from_slice(image.bytes());
    Ok(out)
}

type Parts<'a> = (ImageMedia, &'a [u8], &'a str);

fn parts(bytes: &[u8]) -> Result<Parts<'_>, Failure> {
    let mut bytes = bytes
        .strip_prefix(IMAGE)
        .ok_or_else(|| Failure::usage("invalid stored image tag"))?;
    let media = match take(&mut bytes, 1)? {
        [1] => ImageMedia::Png,
        [2] => ImageMedia::Jpeg,
        _ => return Err(Failure::usage("invalid stored image media")),
    };
    let count = length(&mut bytes)?;
    if count > 16 * 1024 * 1024 {
        return Err(Failure::usage("image file name exceeds 16 MiB"));
    }
    let file = std::str::from_utf8(take(&mut bytes, count)?)
        .map_err(|_| Failure::usage("invalid stored image file name"))?;
    ImageInput::admit_length(bytes.len())?;
    Ok((media, bytes, file))
}

fn decode(bytes: &[u8]) -> Result<(ImageInput, Option<String>), Failure> {
    let (media, bytes, file) = parts(bytes)?;
    Ok((
        ImageInput::new(media, bytes)?,
        (!file.is_empty()).then(|| file.to_owned()),
    ))
}

fn bounded(values: &[&[u8]]) -> Result<(), Failure> {
    let mut admission = ImageAdmission::new(values.len())?;
    for value in values {
        let (_, bytes, _) = parts(value)?;
        admission.push(bytes.len())?;
    }
    Ok(())
}

fn collection(bytes: &[u8], text: Option<String>) -> Result<QuestionInput, Failure> {
    let mut bytes = bytes
        .strip_prefix(IMAGES)
        .ok_or_else(|| Failure::usage("images require thinkthen_images tagged values"))?;
    let count = length(&mut bytes)?;
    ImageAdmission::new(count)?;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let size = length(&mut bytes)?;
        values.push(take(&mut bytes, size)?);
    }
    if !bytes.is_empty() {
        return Err(Failure::usage("invalid stored image trailing bytes"));
    }
    bounded(&values)?;
    let images = values
        .into_iter()
        .map(|value| decode(value).map(|held| held.0))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(QuestionInput::Images(ImageEvidence::new(text, images)?))
}

/// Reuse persistent image decoding for the native complete request carrier.
pub(crate) fn complete_record(
    bytes: &[u8],
    text: Option<String>,
) -> Result<thinkthen::RequestItem, Failure> {
    let QuestionInput::Images(input) = collection(bytes, text)? else {
        return Err(Failure::defect("image input held no images"));
    };
    let mut item = crate::request::text(String::new());
    item.original = input.text().map(|text| thinkthen::RequestOriginal::Text {
        text: text.to_owned(),
    });
    item.images = input
        .images()
        .iter()
        .map(|image| thinkthen::RequestImage::Bytes {
            media: image.media(),
            bytes: image.bytes().to_vec(),
        })
        .collect();
    Ok(item)
}

fn constructor(context: &Context<'_>) -> Result<Option<Vec<u8>>, Failure> {
    if matches!(context.get_raw(0), ValueRef::Null) || matches!(context.get_raw(1), ValueRef::Null)
    {
        return Ok(None);
    }
    let mime = question::text(context.get_raw(1), "image media")?.unwrap_or_default();
    let media = match mime.as_str() {
        "image/png" => ImageMedia::Png,
        "image/jpeg" => ImageMedia::Jpeg,
        _ => {
            return Err(Failure::usage(
                "image media must be image/png or image/jpeg",
            ));
        }
    };
    let bytes = blob(context.get_raw(0))?;
    ImageInput::admit_length(bytes.len())?;
    encode(&ImageInput::new(media, bytes)?, None).map(Some)
}

fn pack(context: &Context<'_>) -> Result<Vec<u8>, Failure> {
    ImageAdmission::new(context.len())?;
    let values = (0..context.len())
        .map(|at| blob(context.get_raw(at)))
        .collect::<Result<Vec<_>, _>>()?;
    bounded(&values)?;
    let mut out = IMAGES.to_vec();
    out.extend_from_slice(&(context.len() as u32).to_le_bytes());
    let mut images = Vec::new();
    for bytes in values {
        images.push(decode(bytes)?.0);
        let size =
            u32::try_from(bytes.len()).map_err(|_| Failure::usage("image value is too large"))?;
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(bytes);
    }
    ImageEvidence::new(None, images)?;
    Ok(out)
}

fn file(context: &Context<'_>) -> Result<Option<Vec<u8>>, Failure> {
    let Some(path) = question::text(context.get_raw(0), "image path")? else {
        return Ok(None);
    };
    if !std::path::Path::new(&path).is_file() {
        return Err(Failure::of(
            thinkthen::ErrorKind::Local,
            "source image must be a regular file",
        ));
    }
    let mut reader = thinkthen::read_inputs(
        [path],
        thinkthen::InputReaderOptions {
            reading: thinkthen::ReaderOptions {
                unit: thinkthen::SourceUnit::File,
                window: None,
            },
            media: thinkthen::ReaderMedia::Image,
        },
    )?;
    match reader.next().transpose()? {
        Some(thinkthen::SourceItem::Image(image)) => {
            encode(&image.record, Some(&image.file)).map(Some)
        }
        _ => Err(Failure::usage("image path must select one image")),
    }
}

fn judged(context: &Context<'_>, verb: Option<thinkthen::For>) -> Result<Value, Failure> {
    if !(2..=4).contains(&context.len()) {
        return Err(Failure::usage(
            "image judgment takes question, images, optional text and settings",
        ));
    }
    if matches!(context.get_raw(0), ValueRef::Null) || matches!(context.get_raw(1), ValueRef::Null)
    {
        return Ok(Value::Null);
    }
    let argument = question::text(context.get_raw(0), "question")?.unwrap_or_default();
    let text = if context.len() > 2 {
        question::text(context.get_raw(2), "image text")?
    } else {
        None
    };
    let settings = if context.len() > 3 {
        question::call_settings(context.get_raw(3))?
    } else {
        thinkthen::Settings::default()
    };
    let requested = match verb {
        Some(verb) => verb,
        None => match question::question(&argument)?.kind() {
            thinkthen::QuestionKind::Decide => thinkthen::For::Decide,
            thinkthen::QuestionKind::Choose => thinkthen::For::Choose,
            thinkthen::QuestionKind::Score => thinkthen::For::Score,
            _ => {
                return Err(Failure::usage(
                    "image judgments take decide, choose or score",
                ));
            }
        },
    };
    let held = question::question_with_settings(&argument, &settings, requested)?;
    let wanted = match requested {
        thinkthen::For::Decide => thinkthen::QuestionKind::Decide,
        thinkthen::For::Choose => thinkthen::QuestionKind::Choose,
        thinkthen::For::Score => thinkthen::QuestionKind::Score,
        _ => {
            return Err(Failure::usage(
                "image judgments take decide, choose or score",
            ));
        }
    };
    if held.kind() != wanted {
        return Err(Failure::usage("image function and question kind differ"));
    }
    let item = complete_record(blob(context.get_raw(1))?, text)?;
    let shared = settings.context().map(str::to_owned);
    let details =
        worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
            let records = shared.is_some();
            let options = match &shared {
                Some(shared) => options.context(shared),
                None => options,
            };
            crate::request::details(engine, &held, vec![item], options, records)?
                .into_iter()
                .next()
                .ok_or_else(|| Failure::defect("image details returned no record"))
        })?;
    if verb.is_none() {
        return Ok(Value::Text(details.to_json()));
    }
    Ok(match details.value() {
        Judgment::Decision(thinkthen::Answer::Yes) => Value::Integer(1),
        Judgment::Decision(thinkthen::Answer::No) => Value::Integer(0),
        Judgment::Decision(thinkthen::Answer::Unsure) | Judgment::Choice(None) => Value::Null,
        Judgment::Choice(Some(label)) => Value::Text(label.clone()),
        Judgment::Score(score) => Value::Real(*score),
        _ => return Err(Failure::defect("image details held another judgment")),
    })
}

pub(crate) fn register(connection: &Catalog<'_>, flags: FunctionFlags) -> rusqlite::Result<()> {
    connection.create_scalar_function(
        "thinkthen_image",
        2,
        flags,
        "Construct one native image BLOB from bytes and media type.",
        |ctx| Ok(guard("image", || constructor(ctx))?),
    )?;
    connection.create_scalar_function(
        "thinkthen_images",
        -1,
        flags,
        "Combine ordered native image BLOBs into one collection.",
        |ctx| Ok(guard("images", || pack(ctx))?),
    )?;
    connection.create_scalar_function(
        "thinkthen_image_file",
        1,
        flags,
        "Read an explicitly named image file into a native image BLOB.",
        |ctx| Ok(guard("image file", || file(ctx))?),
    )?;
    connection.create_scalar_function(
        "thinkthen_image_file_name",
        1,
        flags,
        "Return the source file name retained in a native image BLOB.",
        |ctx| {
            Ok(guard("image file name", || {
                if matches!(ctx.get_raw(0), ValueRef::Null) {
                    return Ok(None);
                }
                decode(blob(ctx.get_raw(0))?).map(|(_, file)| file)
            })?)
        },
    )?;
    for (name, verb) in [
        ("thinkthen_decide_images", Some(thinkthen::For::Decide)),
        ("thinkthen_choose_images", Some(thinkthen::For::Choose)),
        ("thinkthen_score_images", Some(thinkthen::For::Score)),
        ("thinkthen_details_images", None),
    ] {
        connection.create_scalar_function(
            name,
            -1,
            flags,
            "Judge an ordered native image collection and return its answer or details.",
            move |ctx| Ok(guard(name, || judged(ctx, verb))?),
        )?;
    }
    Ok(())
}
