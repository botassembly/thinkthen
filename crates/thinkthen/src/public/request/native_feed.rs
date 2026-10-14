//! Native composed feeds retain their originals while sharing request admission.
use super::execution::{FeedContents, RequestFeed};
use super::{AdmittedRequest, RequestFraming, RequestFunction, RequestInput};
use crate::{Error, QuestionInput};

pub(super) fn records<'a>(
    request: &'a AdmittedRequest,
    contents: FeedContents<'a>,
) -> Result<super::composition::Inputs<'a>, Error> {
    header(request)?;
    let FeedContents::Records(records) = contents else {
        return Err(Error::defect("native feed lost its records"));
    };
    if !request.request.call.arguments().options.files_only {
        return Ok(records);
    }
    Ok(Box::new(records.map(|row| {
        row.and_then(|row| {
            if !matches!(&row.original, QuestionInput::Record(record) if record.location().is_some()) {
                return Err(Error::usage("files_only requires filter source input"));
            }
            Ok(row)
        })
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

pub(super) fn projection(
    request: &AdmittedRequest,
    feed: Option<&RequestFeed<'_>>,
) -> Result<bool, Error> {
    let args = request.request.call.arguments();
    if let RequestInput::Feed { name, .. } = &args.input
        && feed.is_none_or(|feed| &feed.name != name)
    {
        return Err(Error::usage(
            "the request requires its named caller-supplied feed",
        ));
    }
    let all = feed.is_some_and(|feed| feed.all_filter_results);
    // Feed location authority belongs to the attached owner, not the header.
    if args.options.files_only
        && matches!(args.input, RequestInput::Feed { .. })
        && feed.is_none_or(|feed| {
            !matches!(
                feed.contents,
                FeedContents::Records(_) | FeedContents::Session(_)
            )
        })
    {
        return Err(Error::usage("files_only requires filter source input"));
    }
    if all
        && (request.request.call.function() != RequestFunction::Filter
            || !matches!(args.input, RequestInput::Feed { .. })
            || feed.is_none_or(|feed| {
                !matches!(
                    feed.contents,
                    FeedContents::Records(_) | FeedContents::Session(_)
                )
            }))
    {
        return Err(Error::usage(
            "all filter results require a native composed filter feed",
        ));
    }
    if feed.is_some_and(|feed| feed.record_reading.is_some())
        && (!matches!(args.input, RequestInput::Feed { .. })
            || feed.is_none_or(|feed| !matches!(feed.contents, FeedContents::Session(_)))
            || super::composition::explicit_projection(&args.options))
    {
        return Err(Error::usage(
            "owned record reading requires a session feed without request projections",
        ));
    }
    Ok(all)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::request::RequestFunction;

    #[test]
    fn native_records_keep_function_specific_recognition_control_refusals() {
        for (function, examples, expected) in [
            (
                RequestFunction::Find,
                true,
                "find takes one whole-set call context and no per-record controls",
            ),
            (
                RequestFunction::Find,
                false,
                "find takes one whole-set call context and no per-record controls",
            ),
            (
                RequestFunction::Relate,
                true,
                "relate takes one whole-set call context and no per-record controls",
            ),
            (
                RequestFunction::Relate,
                false,
                "relate takes one whole-set call context and no per-record controls",
            ),
            (
                RequestFunction::Decide,
                true,
                "record examples are admitted only for recognize",
            ),
            (
                RequestFunction::Decide,
                false,
                "record seed spans are admitted only for recognize",
            ),
        ] {
            let row = crate::RecordInput {
                original: "First.",
                context: None,
                options: None,
                examples: examples.then(Vec::new),
                seed_spans: (!examples).then(Vec::new),
            };
            let input = QuestionInput::Text("First.".into());
            let error = validate(function, &row, &input, None, true).expect_err("native controls");
            assert_eq!(error.kind(), crate::ErrorKind::Usage);
            assert_eq!(error.detail().message(), expected);
            validate(RequestFunction::Recognize, &row, &input, None, true)
                .expect("recognition controls");
        }
    }
}
