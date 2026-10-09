//! Unresolved transport descriptors share native admission and attachment accounting.
#[cfg(feature = "cli")]
use super::{AdmittedRequest, RequestFeed, RequestItem, RequestSource};
use super::{RequestImage, RequestInput};
use crate::{
    Error, ImageMedia, ObjectContext, RawRecord, RecordContext, RecordOption, RecordOptions,
};
use serde_json::{Value, value::RawValue};

#[derive(Clone, Copy, Debug)]
pub(crate) struct TransportAttachmentLimit(usize);
impl TransportAttachmentLimit {
    #[cfg(feature = "cli")]
    pub(crate) fn new(bytes: usize) -> Result<Self, Error> {
        if bytes == 0 {
            return Err(budget_error());
        }
        Ok(Self(bytes))
    }
}
#[cfg(feature = "cli")]
pub(crate) struct TransportDescriptor {
    pub(crate) item: RequestItem,
    pub(crate) source: Option<RequestSource>,
}
#[derive(Debug)]
pub(super) struct AttachmentBudget(Option<usize>);
impl AttachmentBudget {
    pub(super) fn new(limit: Option<TransportAttachmentLimit>) -> Self {
        Self(limit.map(|limit| limit.0))
    }
    pub(super) fn remaining(&self) -> Option<usize> {
        self.0
    }
    pub(super) fn charge(&mut self, bytes: usize) -> Result<(), Error> {
        if let Some(remaining) = &mut self.0 {
            *remaining = remaining.checked_sub(bytes).ok_or_else(budget_error)?;
        }
        Ok(())
    }
}
fn budget_error() -> Error {
    Error::usage("retained attachments exceed the input byte ceiling")
}
fn preflight(images: &[RequestImage], budget: &mut AttachmentBudget) -> Result<(), Error> {
    for image in images {
        if let RequestImage::Bytes { bytes, .. } = image {
            budget.charge(bytes.len())?;
        }
    }
    Ok(())
}
pub(super) fn preflight_input(
    input: &RequestInput,
    limit: Option<TransportAttachmentLimit>,
) -> Result<(), Error> {
    let mut budget = AttachmentBudget::new(limit);
    match input {
        RequestInput::Text { images, .. }
        | RequestInput::Json { images, .. }
        | RequestInput::Feed { images, .. } => preflight(images, &mut budget),
        RequestInput::Records { items }
        | RequestInput::Units { items }
        | RequestInput::Entities { items } => {
            for item in items {
                preflight(&item.images, &mut budget)?;
            }
            Ok(())
        }
        RequestInput::Source { .. } => Ok(()),
    }
}
#[cfg(feature = "cli")]
impl AdmittedRequest {
    pub(crate) fn admit_descriptor_feed(
        &self,
        name: String,
        descriptors: Vec<TransportDescriptor>,
    ) -> Result<RequestFeed<'static>, Error> {
        let args = self.request.call.arguments();
        let RequestInput::Feed {
            name: expected,
            images,
            ..
        } = &args.input
        else {
            return Err(Error::usage("transport descriptors require a named feed"));
        };
        if expected != &name || !images.is_empty() {
            return Err(Error::usage(
                "transport descriptors own their attachments and feed",
            ));
        }
        let mut budget = AttachmentBudget::new(self.attachment_limit);
        // Complete inline preflight precedes every per-item validation clone.
        for descriptor in &descriptors {
            preflight(&descriptor.item.images, &mut budget)?;
        }
        for descriptor in &descriptors {
            self.admit_descriptor(descriptor)?;
        }
        Ok(RequestFeed::descriptors(name, descriptors))
    }
    fn admit_descriptor(&self, descriptor: &TransportDescriptor) -> Result<(), Error> {
        let args = self.request.call.arguments();
        let mut item = descriptor.item.clone();
        if let Some(source) = &descriptor.source {
            if item.original.is_some() {
                return Err(Error::usage("a descriptor selects one original"));
            }
            if source.paths.is_empty()
                || source.paths.iter().any(|path| path.as_os_str().is_empty())
                || source.media != crate::ReaderMedia::Text
                || source.reading.unit != crate::SourceUnit::File
            {
                return Err(Error::usage(
                    "an input source requires explicit whole text files",
                ));
            }
            source.reading.validate()?;
            // Source presence supplies the original without opening its authority.
            item.original = Some(super::RequestOriginal::Text {
                text: String::new(),
            });
        }
        super::admission::admit_item(self.request.call.function(), &item, &args.options)
    }
}

/// Compose a native source with cancellation and original-byte rank admission.
pub(crate) fn source_records<'a>(
    reading: crate::RecordReading,
    mut source: impl Iterator<Item = Result<crate::SourceItem, Error>> + 'a,
    controls: crate::CallOptions<'a>,
    annotate: bool,
    rank: bool,
) -> super::composition::Inputs<'a> {
    let mut remaining = crate::core::MAX_RECORD_BYTES;
    let mut stopped = false;
    Box::new(std::iter::from_fn(move || {
        if stopped {
            return None;
        }
        let result = match controls
            .admission()
            .and_then(|()| source.next().transpose())
        {
            Ok(None) => return None,
            Ok(Some(item)) => (|| {
                if rank && let crate::SourceItem::Text(text) = &item {
                    remaining = remaining
                        .checked_sub(text.record.len())
                        .ok_or_else(rank_budget_error)?;
                }
                super::composition::source_row(item, annotate, &reading)
            })(),
            Err(error) => Err(error),
        };
        stopped = result.is_err();
        Some(result)
    }))
}

fn rank_budget_error() -> Error {
    Error::usage("source rank reads at most 16 MiB across all input records")
}

/// Decode the existing complete-call record envelope at the public Request edge.
/// File authority and terminal reader errors belong to the caller.
/// Duplicate fields refuse; ordered JSON originals retain their authored form.
pub(super) fn record_descriptor(source: &str) -> Result<super::RequestItem, Error> {
    match crate::Settings::parse(&format!("{{\"sql_descriptor\":{source}}}")) {
        Ok(_) | Err(crate::SettingsError::UnknownKey(_)) => {}
        Err(error) => return Err(Error::usage(error.to_string())),
    }
    let fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(source)
            .map_err(|_| Error::usage("descriptor is one object with unique fields"))?;
    if fields.keys().any(|k| {
        !(matches!(
            k.as_str(),
            "text" | "document" | "json_text" | "json" | "context" | "options" | "images"
        ) || matches!(k.as_str(), "examples" | "seed_spans"))
    }) || ["text", "json", "document", "json_text"]
        .iter()
        .filter(|key| fields.contains_key(**key))
        .count()
        > 1
    {
        return Err(Error::usage("invalid complete record descriptor"));
    }
    let images = fields
        .get("images")
        .map(|v| descriptor_images(v.get()))
        .transpose()?
        .unwrap_or_default();
    let original = if let Some(json) = fields.get("json_text") {
        let text = serde_json::from_str::<String>(json.get())
            .map_err(|_| Error::usage("JSON text is text"))?;
        Some(RawRecord::json(&text)?)
    } else if let Some(document) = fields.get("document") {
        let text = serde_json::from_str::<String>(document.get())
            .map_err(|_| Error::usage("document is text"))?;
        Some(RawRecord::json(&text).or_else(|_| RawRecord::text(&text))?)
    } else {
        match (fields.get("text"), fields.get("json")) {
            (Some(text), None) => Some(RawRecord::text(
                &serde_json::from_str::<String>(text.get())
                    .map_err(|_| Error::usage("record text is literal text"))?,
            )?),
            (None, Some(json)) => Some(RawRecord::json(json.get())?),
            (None, None) => None,
            _ => return Err(Error::usage("one original per record")),
        }
    };

    if images.is_empty() && (original.is_none() || fields.contains_key("images")) {
        crate::ImageEvidence::new(None, Vec::new())?;
    }
    let original = original.map(|value| match value.literal() {
        Some(text) => super::RequestOriginal::Text {
            text: text.to_owned(),
        },
        None => super::RequestOriginal::Json { value },
    });
    Ok(super::RequestItem {
        original,
        images,
        context: fields.get("context").map(|v| context(v)).transpose()?,
        options: fields.get("options").map(|v| options(v)).transpose()?,
        seed_spans: fields
            .get("seed_spans")
            .map(|v| {
                serde_json::from_str(v.get())
                    .map_err(|_| Error::usage("seed spans is an ordered array"))
            })
            .transpose()?,
        examples: fields
            .get("examples")
            .map(|v| {
                serde_json::from_str(v.get())
                    .map_err(|_| Error::usage("examples is an ordered array"))
            })
            .transpose()?,
    })
}
fn descriptor_images(source: &str) -> Result<Vec<RequestImage>, Error> {
    let images: Vec<Value> = serde_json::from_str(source)
        .map_err(|_| Error::usage("images is an explicit ordered array"))?;
    images
        .into_iter()
        .map(|value| {
            let media = match value.get("media").and_then(Value::as_str) {
                Some("image/png") => ImageMedia::Png,
                Some("image/jpeg") => ImageMedia::Jpeg,
                _ => return Err(Error::usage("image media is image/png or image/jpeg")),
            };
            let bytes = serde_json::from_value::<Vec<u8>>(
                value
                    .get("bytes")
                    .cloned()
                    .ok_or_else(|| Error::usage("image requires compressed bytes"))?,
            )
            .map_err(|_| Error::usage("image bytes is an integer array"))?;
            Ok(RequestImage::Bytes { media, bytes })
        })
        .collect()
}

fn context(context: &RawValue) -> Result<RecordContext, Error> {
    let value: Value = serde_json::from_str(context.get())
        .map_err(|_| Error::usage("invalid per-record context"))?;
    Ok(match value {
        Value::String(text) => RecordContext::Text(text),
        Value::Object(_) => {
            RecordContext::Object(ObjectContext::new(&RawRecord::json(context.get())?)?)
        }
        _ => RecordContext::Object(ObjectContext::new(&RawRecord::json(context.get())?)?),
    })
}
fn options(options: &RawValue) -> Result<RecordOptions, Error> {
    let values: Vec<Value> = serde_json::from_str(options.get())
        .map_err(|_| Error::usage("options is an ordered array"))?;
    let options = values
        .into_iter()
        .map(|v| {
            let (name, description) = if let Some(name) = v.as_str() {
                (name.to_owned(), None)
            } else {
                (
                    v.get("name")
                        .and_then(Value::as_str)
                        .ok_or_else(|| Error::usage("option requires a name"))?
                        .to_owned(),
                    v.get("description")
                        .map(|v| crate::Description::from_json(&v.to_string()))
                        .transpose()?,
                )
            };
            Ok(RecordOption { name, description })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    RecordOptions::new(options)
}
