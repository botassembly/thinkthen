//! Explicit composite images; PostgreSQL never reads server image paths.

use pgrx::datum::Array;
use pgrx::prelude::*;
use thinkthen::{
    ImageAdmission, ImageInput, ImageMedia, Judgment, RequestImage, RequestItem, RequestOriginal,
};

use crate::call::{self, OrRaise as _};
use crate::ffi::RawJson;
use crate::forms::{self, Named};

extension_sql!(
    "CREATE TYPE thinkthen_image_value AS (media text, data bytea, file text);",
    name = "image_value_type",
);

fn image(bytes: &[u8], mime: &str) -> Result<ImageInput, thinkthen::Error> {
    ImageInput::admit_length(bytes.len())?;
    let media = match mime {
        "image/png" => ImageMedia::Png,
        "image/jpeg" => ImageMedia::Jpeg,
        _ => return Err(call::usage("image media must be image/png or image/jpeg")),
    };
    ImageInput::new(media, bytes)
}

#[pg_extern(parallel_restricted, requires = ["image_value_type"])]
fn thinkthen_image(
    bytes: Option<&[u8]>,
    mime: Option<&str>,
) -> Option<pgrx::composite_type!('static, "thinkthen_image_value")> {
    call::guarded(|| {
        let (bytes, mime) = (bytes?, mime?);
        image(bytes, mime).or_raise();
        let mut tuple = PgHeapTuple::new_composite_type("thinkthen_image_value")
            .unwrap_or_else(|_| call::raise(call::defect("image composite type is unavailable")));
        tuple
            .set_by_name("media", mime)
            .unwrap_or_else(|_| call::raise(call::defect("image media field is unavailable")));
        tuple
            .set_by_name("data", bytes)
            .unwrap_or_else(|_| call::raise(call::defect("image data field is unavailable")));
        Some(tuple)
    })
}

pub(crate) fn input(
    images: Array<'_, pgrx::composite_type!("thinkthen_image_value")>,
    text: Option<&str>,
) -> Result<RequestItem, thinkthen::Error> {
    let mut admission = ImageAdmission::new(images.len())?;
    for tuple in images.iter() {
        let tuple = tuple.ok_or_else(|| call::usage("image list contains NULL"))?;
        let bytes = tuple
            .get_by_name::<&[u8]>("data")
            .map_err(|_| call::usage("invalid image data field"))?
            .ok_or_else(|| call::usage("image data is NULL"))?;
        admission.push(bytes.len())?;
    }
    let images = images
        .iter()
        .map(|tuple| {
            let tuple = tuple.ok_or_else(|| call::usage("image list contains NULL"))?;
            let bytes = tuple
                .get_by_name::<&[u8]>("data")
                .map_err(|_| call::usage("invalid image data field"))?
                .ok_or_else(|| call::usage("image data is NULL"))?;
            let mime = tuple
                .get_by_name::<String>("media")
                .map_err(|_| call::usage("invalid image media field"))?
                .ok_or_else(|| call::usage("image media is NULL"))?;
            // file is retained in the caller's composite, never model evidence.
            let image = image(bytes, &mime)?;
            Ok(RequestImage::Bytes {
                media: image.media(),
                bytes: image.bytes().to_vec(),
            })
        })
        .collect::<Result<_, thinkthen::Error>>()?;
    Ok(RequestItem {
        original: text.map(|text| RequestOriginal::Text {
            text: text.to_owned(),
        }),
        images,
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
    })
}

fn judged(
    question: &str,
    images: Array<'_, pgrx::composite_type!("thinkthen_image_value")>,
    text: Option<&str>,
    raw: Option<RawJson>,
    requested: Option<thinkthen::For>,
) -> thinkthen::Details {
    let (settings, controls) = forms::controls(raw.as_ref(), Named::default());
    let verb = requested.unwrap_or_else(|| forms::plan_verb(question, &settings));
    if !matches!(
        verb,
        thinkthen::For::Decide | thinkthen::For::Choose | thinkthen::For::Score
    ) {
        call::raise(call::usage("image judgments take decide, choose or score"));
    }
    let question = forms::question(Some(question), None, verb, &settings);
    let input = input(images, text).or_raise();
    call::run(controls, move |engine, options| {
        crate::request::details(engine, &question, vec![input], options, false)
            .map(|rows| rows.into_iter().next())
    })
    .unwrap_or_else(|| call::raise(call::defect("image details returned no record")))
}

#[pg_extern(parallel_restricted, requires = ["image_value_type"])]
fn thinkthen_decide_images(
    question: Option<&str>,
    images: Option<Array<'_, pgrx::composite_type!("thinkthen_image_value")>>,
    text: default!(Option<&str>, "NULL"),
    settings: default!(Option<RawJson>, "NULL"),
) -> Option<bool> {
    call::guarded(|| {
        let held = judged(
            question?,
            images?,
            text,
            settings,
            Some(thinkthen::For::Decide),
        );
        match held.value() {
            Judgment::Decision(value) => crate::answer_value(*value),
            _ => call::raise(call::defect("image decide held another judgment")),
        }
    })
}

#[pg_extern(parallel_restricted, requires = ["image_value_type"])]
fn thinkthen_choose_images(
    question: Option<&str>,
    images: Option<Array<'_, pgrx::composite_type!("thinkthen_image_value")>>,
    text: default!(Option<&str>, "NULL"),
    settings: default!(Option<RawJson>, "NULL"),
) -> Option<String> {
    call::guarded(|| {
        let held = judged(
            question?,
            images?,
            text,
            settings,
            Some(thinkthen::For::Choose),
        );
        match held.value() {
            Judgment::Choice(value) => value.clone(),
            _ => call::raise(call::defect("image choose held another judgment")),
        }
    })
}

#[pg_extern(parallel_restricted, requires = ["image_value_type"])]
fn thinkthen_score_images(
    question: Option<&str>,
    images: Option<Array<'_, pgrx::composite_type!("thinkthen_image_value")>>,
    text: default!(Option<&str>, "NULL"),
    settings: default!(Option<RawJson>, "NULL"),
) -> Option<f64> {
    call::guarded(|| {
        let held = judged(
            question?,
            images?,
            text,
            settings,
            Some(thinkthen::For::Score),
        );
        match held.value() {
            Judgment::Score(value) => Some(*value),
            _ => call::raise(call::defect("image score held another judgment")),
        }
    })
}

#[pg_extern(parallel_restricted, requires = ["image_value_type"])]
fn thinkthen_details_images(
    question: Option<&str>,
    images: Option<Array<'_, pgrx::composite_type!("thinkthen_image_value")>>,
    text: default!(Option<&str>, "NULL"),
    settings: default!(Option<RawJson>, "NULL"),
) -> Option<pgrx::datum::JsonB> {
    call::guarded(|| {
        let held = judged(question?, images?, text, settings, None);
        Some(crate::jsonb(&held.to_json()))
    })
}
