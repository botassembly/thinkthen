//! Native descriptors pass their explicit record framing through the shared decoder.
use super::{
    AdmittedRequest, RequestDefinition, RequestFraming, RequestInput, RequestItem, RequestOriginal,
};
use crate::{Error, RawRecord};

pub(super) fn admit(request: &AdmittedRequest) -> Result<(), Error> {
    if let RequestInput::Feed {
        framing: RequestFraming::Csv | RequestFraming::Tsv,
        ..
    } = request.request.call.arguments().input
    {
        return Err(Error::usage(
            "native session table framing is not implemented",
        ));
    }
    Ok(())
}

pub(super) fn reading(
    request: &AdmittedRequest,
    definition: &RequestDefinition,
) -> Result<(), Error> {
    let args = request.request.call.arguments();
    if let RequestInput::Feed {
        framing: RequestFraming::Lines,
        ..
    } = args.input
    {
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

pub(super) fn item(
    mut item: RequestItem,
    framing: RequestFraming,
) -> Result<Option<RequestItem>, Error> {
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
                return Err(Error::usage(
                    "native session table framing is not implemented",
                ));
            }
        });
    }
    Ok(Some(item))
}
