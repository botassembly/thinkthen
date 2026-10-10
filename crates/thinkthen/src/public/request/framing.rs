//! Native descriptors pass their explicit record framing through the shared decoder.
use super::{
    AdmittedRequest, RequestDefinition, RequestFraming, RequestInput, RequestItem, RequestOriginal,
};
use crate::{Error, RawRecord};

pub(super) fn admit(request: &AdmittedRequest) -> Result<(), Error> {
    if let RequestInput::Feed {
        framing: RequestFraming::Csv | RequestFraming::Tsv,
        reading,
        ..
    } = request.request.call.arguments().input
        && (reading.unit != crate::SourceUnit::Line || reading.window.is_some())
    {
        return Err(Error::usage(
            "table session framing requires complete rows without file or window units",
        ));
    }
    Ok(())
}

pub(super) fn reading(
    request: &AdmittedRequest,
    definition: &RequestDefinition,
) -> Result<(), Error> {
    let args = request.request.call.arguments();
    if matches!(
        &args.input,
        RequestInput::Feed {
            framing: RequestFraming::Lines,
            ..
        } | RequestInput::Source {
            source: super::RequestSource {
                framing: Some(RequestFraming::Lines),
                ..
            }
        }
    ) {
        let reading = super::composition::reading(definition, &args.options)?;
        crate::core::Reading::new(crate::core::Framing::Lines, reading.fields().to_vec())
            .map_err(Error::refused)?;
        if args.options.context_field.is_some()
            || args.options.options_field.is_some()
            || args.options.examples_field.is_some()
            || args.options.seed_spans_field.is_some()
        {
            return Err(Error::refused(crate::core::RecordError::TextHasNoMembers));
        }
    }
    Ok(())
}

fn item(mut item: RequestItem, framing: RequestFraming) -> Result<Option<RequestItem>, Error> {
    if matches!(framing, RequestFraming::Document) {
        return Ok(Some(item));
    }
    if matches!(framing, RequestFraming::Lines)
        && matches!(item.original, Some(RequestOriginal::Json { .. }))
    {
        return Err(Error::usage("line framing requires a text original"));
    }
    if let Some(RequestOriginal::Text { text }) = &item.original {
        let reading = crate::core::Reading::new(crate::core::Framing::Lines, Vec::new())
            .map_err(Error::refused)?;
        let original = reading.record(text.as_bytes()).map_err(Error::refused)?;
        let text = original
            .text()
            .ok_or_else(|| Error::defect("line framing lost its text"))?;
        if text.trim().is_empty() && item.images.is_empty() {
            return Ok(None);
        }
        item.original = Some(match framing {
            RequestFraming::Jsonl => RequestOriginal::Json {
                value: RawRecord::json(text)?,
            },
            RequestFraming::Lines => RequestOriginal::Text {
                text: text.to_owned(),
            },
            _ => {
                return Err(Error::usage("table framing bypassed its decoder"));
            }
        });
    }
    Ok(Some(item))
}

pub(super) struct Decoder {
    framing: RequestFraming,
    table: Option<crate::table::framed::Framed>,
    stopped: bool,
}
impl Decoder {
    pub(super) fn new(framing: RequestFraming) -> Self {
        let kind = match framing {
            RequestFraming::Csv => Some(crate::table::Kind::Csv),
            RequestFraming::Tsv => Some(crate::table::Kind::Tsv),
            _ => None,
        };
        Self {
            framing,
            table: kind.map(crate::table::framed::Framed::new),
            stopped: false,
        }
    }

    pub(super) fn item(&mut self, mut value: RequestItem) -> Result<Option<RequestItem>, Error> {
        let Some(table) = &mut self.table else {
            return item(value, self.framing);
        };
        let controls = value.context.is_some()
            || value.options.is_some()
            || value.examples.is_some()
            || value.seed_spans.is_some()
            || !value.images.is_empty();
        if table.needs_header() && controls {
            return Err(Error::usage(
                "table session header cannot carry per-item controls or images",
            ));
        }
        let Some(RequestOriginal::Text { text }) = value.original.take() else {
            return Err(Error::usage(
                "table session framing requires a text original",
            ));
        };
        let header = table.needs_header();
        let Some(record) = table.push(text)? else {
            if !header && controls {
                return Err(Error::usage(
                    "blank table session descriptor cannot carry per-item controls or images",
                ));
            }
            return Ok(None);
        };
        value.original = Some(RequestOriginal::Json {
            value: RawRecord(std::sync::Arc::new(record)),
        });
        Ok(Some(value))
    }

    fn descriptor(
        &mut self,
        descriptor: super::RequestSessionDescriptor,
    ) -> Result<Option<super::RequestSessionDescriptor>, Error> {
        let location = descriptor.location;
        Ok(self
            .item(descriptor.item)?
            .map(|item| super::RequestSessionDescriptor { item, location }))
    }

    pub(super) fn next(
        &mut self,
        queue: &super::session_queue::Queue,
        controls: crate::CallOptions<'_>,
    ) -> Option<Result<super::RequestSessionDescriptor, Error>> {
        if self.stopped {
            return None;
        }
        loop {
            let decoded = match queue.next(controls) {
                Some(descriptor) => descriptor.and_then(|descriptor| self.descriptor(descriptor)),
                None => {
                    self.stopped = true;
                    return controls
                        .reader_admission()
                        .and_then(|()| self.finish())
                        .err()
                        .map(Err);
                }
            };
            match decoded {
                Ok(None) => continue,
                Ok(Some(descriptor)) => return Some(Ok(descriptor)),
                Err(error) => {
                    self.stopped = true;
                    queue.close_intake();
                    return Some(Err(error));
                }
            }
        }
    }

    pub(super) fn finish(&self) -> Result<(), Error> {
        self.table
            .as_ref()
            .map_or(Ok(()), crate::table::framed::Framed::finish)
    }
}
