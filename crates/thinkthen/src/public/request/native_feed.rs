//! Native composed feeds retain their originals while sharing request admission.
use super::{AdmittedRequest, RequestFraming, RequestInput};
use crate::{Error, QuestionInput};

pub(super) fn records<'a>(
    request: &'a AdmittedRequest,
    contents: super::execution::FeedContents<'a>,
) -> Result<super::composition::Inputs<'a>, Error> {
    header(request)?;
    let super::execution::FeedContents::Records(records) = contents else {
        return Err(Error::defect("native feed lost its records"));
    };
    Ok(records)
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
    native: bool,
) -> Result<(), Error> {
    if function == super::RequestFunction::Find {
        if row.context.is_some() || row.options.is_some() {
            return Err(Error::usage(
                "find takes one whole-set call context and no per-record controls",
            ));
        }
        crate::public::images::guard(crate::InputFunction::Find, input)?;
    }
    if row.options.is_some() && function != super::RequestFunction::Choose {
        return Err(Error::usage("item options apply only to choose"));
    }
    let (examples, seeds) = match (native, function) {
        (true, super::RequestFunction::Find) => (
            "find takes one whole-set call context and no per-record controls",
            "find takes one whole-set call context and no per-record controls",
        ),
        (true, super::RequestFunction::Relate) => (
            "relate takes one whole-set call context and no per-record controls",
            "relate takes one whole-set call context and no per-record controls",
        ),
        (true, _) => (
            "record examples are admitted only for recognize",
            "record seed spans are admitted only for recognize",
        ),
        (false, _) => (
            "item recognition controls apply only to recognize",
            "item recognition controls apply only to recognize",
        ),
    };
    row.admit_recognition_controls(function, examples, seeds)?;
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
