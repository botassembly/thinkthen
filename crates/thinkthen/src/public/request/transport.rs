//! Unresolved transport descriptors share native admission and attachment accounting.
use super::{AdmittedRequest, RequestFeed, RequestImage, RequestInput, RequestItem, RequestSource};
use crate::Error;

#[derive(Clone, Copy, Debug)]
pub(crate) struct TransportAttachmentLimit(usize);
impl TransportAttachmentLimit {
    pub(crate) fn new(bytes: usize) -> Result<Self, Error> {
        if bytes == 0 {
            return Err(budget_error());
        }
        Ok(Self(bytes))
    }
}
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
                    remaining = remaining.checked_sub(text.record.len()).ok_or_else(|| {
                        Error::usage("source rank reads at most 16 MiB across all input records")
                    })?;
                }
                super::composition::source_row(item, annotate, &reading)
            })(),
            Err(error) => Err(error),
        };
        stopped = result.is_err();
        Some(result)
    }))
}
