//! Native composed feeds retain their originals while sharing request admission.
use super::{AdmittedRequest, RequestFraming, RequestInput};
use crate::{CallOptions, Error, QuestionInput};

pub(super) fn records<'a>(
    request: &'a AdmittedRequest,
    contents: super::execution::FeedContents<'a>,
    controls: CallOptions<'a>,
    image_refusal: Option<String>,
) -> Result<super::composition::Inputs<'a>, Error> {
    header(request)?;
    let super::execution::FeedContents::Records(records) = contents else {
        return Err(Error::defect("native feed lost its records"));
    };
    let function = request.request.call.function();
    Ok(Box::new(records.map(move |row| {
        controls.admission()?;
        let row = row?;
        validate(function, &row, &row.original, image_refusal.as_deref())?;
        Ok(row)
    })))
}

pub(super) fn header(request: &AdmittedRequest) -> Result<(), Error> {
    let args = request.request.call.arguments();
    let RequestInput::Feed {
        framing,
        reading,
        images,
        ..
    } = &args.input
    else {
        return Err(Error::defect("native records require their named feed"));
    };
    let options = &args.options;
    if !matches!(framing, RequestFraming::Document)
        || reading.unit != crate::SourceUnit::Line
        || reading.window.is_some()
        || !images.is_empty()
        || options.field.is_some()
        || options.context_field.is_some()
        || options.options_field.is_some()
        || options.seed_spans_field.is_some()
        || options.examples_field.is_some()
    {
        return Err(Error::usage(
            "native composed records own framing, projections and attachments",
        ));
    }
    Ok(())
}

pub(super) fn validate<T>(
    function: super::RequestFunction,
    row: &crate::RecordInput<T>,
    input: &QuestionInput,
    image_refusal: Option<&str>,
) -> Result<(), Error> {
    if row.options.is_some() && function != super::RequestFunction::Choose {
        return Err(Error::usage("item options apply only to choose"));
    }
    if (row.examples.is_some() || row.seed_spans.is_some())
        && function != super::RequestFunction::Recognize
    {
        return Err(Error::usage(
            "item recognition controls apply only to recognize",
        ));
    }
    let images = match input {
        QuestionInput::Record(record) => record.images(),
        QuestionInput::Images(images) => images.images(),
        QuestionInput::Text(_) => &[],
    };
    if !images.is_empty() && !function.images() {
        return Err(Error::usage(format!(
            "{} accepts text only; images are unsupported",
            function.name()
        )));
    }
    if !images.is_empty()
        && let Some(message) = image_refusal
    {
        return Err(Error::usage(message.to_owned()));
    }
    Ok(())
}
