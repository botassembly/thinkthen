//! CLI flags construct the native request before question or evidence reads.
use super::{
    args::{Batching, Command, Common},
    asked,
    failure::Failure,
};
use crate::{
    Request, RequestArguments, RequestBatch, RequestCall, RequestDefinition, RequestFraming,
    RequestImage, RequestInput, RequestOptions, RequestQuestion, RequestSource,
};
use std::path::PathBuf;

#[expect(
    clippy::too_many_lines,
    reason = "one ten-function adapter makes CLI transport choices reviewable together"
)]
pub(super) fn admit(command: &Command) -> Result<(), Failure> {
    let call = match command {
        Command::Decide(a) => {
            let mut common = source_common(&a.common, &a.extra)?;
            let options = options(&common, Some(&a.batching), a.threshold.as_ref());
            let question = selector(&a.question, || {
                definition(
                    asked::decide(a)?.0,
                    crate::public::NativeQuestionKind::Decide,
                )
            })?;
            RequestCall::Decide(arguments(&mut common, question, options)?)
        }
        Command::Choose(a) => {
            let question = selector(&a.question, || {
                let resolved = asked::choose(a)?.0;
                if resolved.question().is_none() {
                    return Ok(crate::Question::choose_records(&a.question)
                        .map_err(native)?
                        .into());
                }
                definition(resolved, crate::public::NativeQuestionKind::Choose)
            })?;
            let mut options = options(&a.common, Some(&a.batching), a.threshold.as_ref());
            options.options_field = a.options_pointer.clone();
            RequestCall::Choose(arguments(&mut a.common.clone(), question, options)?)
        }
        Command::Tag(a) => {
            let question = selector(&a.question, || {
                definition(asked::tag(a)?.0, crate::public::NativeQuestionKind::Tag)
            })?;
            RequestCall::Tag(arguments(
                &mut a.common.clone(),
                question,
                options(&a.common, Some(&a.batching), a.threshold.as_ref()),
            )?)
        }
        Command::Score(a) => {
            let question = selector(&a.question, || {
                definition(asked::score(a)?.0, crate::public::NativeQuestionKind::Score)
            })?;
            RequestCall::Score(arguments(
                &mut a.common.clone(),
                question,
                options(&a.common, Some(&a.batching), a.threshold.as_ref()),
            )?)
        }
        Command::Filter(a) => {
            let mut common = source_common(&a.common, &a.extra)?;
            let question = selector(&a.question, || {
                definition(
                    asked::filter(a)?.0,
                    crate::public::NativeQuestionKind::Decide,
                )
            })?;
            let mut options = options(&common, Some(&a.batching), a.threshold.as_ref());
            options.details = false;
            options.files_only = a.files_only;
            RequestCall::Filter(arguments(&mut common, question, options)?)
        }
        Command::Rank(a) => {
            let mut common = source_common(&a.common, &a.extra)?;
            let question = selector(&a.question, || {
                definition(asked::rank(a)?.0, crate::public::NativeQuestionKind::Rank)
            })?;
            let mut options = options(&common, Some(&a.batching), a.threshold.as_ref());
            options.details = false;
            options.top = a
                .top
                .as_ref()
                .map(|v| v.parse::<usize>().map_err(|_| Failure::TopIsZero))
                .transpose()?;
            RequestCall::Rank(arguments(&mut common, question, options)?)
        }
        Command::Find(a) => {
            let mut common = a.common.as_common();
            let question = if a.question.starts_with('@') {
                RequestQuestion::Reference {
                    reference: a.question.clone(),
                }
            } else {
                RequestQuestion::Text {
                    text: a.question.clone(),
                }
            };
            let mut options = options(&common, None, None);
            options.details = false;
            options.none = a.none;
            RequestCall::Find(arguments(&mut common, question, options)?)
        }
        Command::Annotate(a) => {
            let mut common = a.common.clone();
            if !a.extra_input.is_empty() {
                if !common.input.is_empty() {
                    return Err(Failure::Usage(
                        "positional input files cannot accompany --input",
                    ));
                }
                common.input = a.extra_input.clone();
            }
            let question = RequestQuestion::File {
                path: a.questions.clone(),
            };
            let mut options = options(&common, Some(&a.batching), a.threshold.as_ref());
            options.details = false;
            RequestCall::Annotate(arguments(&mut common, question, options)?)
        }
        Command::Recognize(a) => {
            let question =
                if let Some(reference) = a.kinds.first().filter(|value| value.starts_with('@')) {
                    RequestQuestion::Reference {
                        reference: reference.clone(),
                    }
                } else {
                    RequestQuestion::Definition {
                        value: super::recognize::request_definition(a)?.into(),
                    }
                };
            let mut options = options(&a.common, None, None);
            options.details = false;
            options.context_field = a.context_field.clone();
            options.examples_field = a.examples_field.clone();
            options.seed_spans_field = a.seed_spans_field.clone();
            options.stage_context = Some(crate::RecognitionStageContext {
                boundary: a.boundary_context.clone(),
                kind_edge: a.kind_edge_context.clone(),
                relation: a.relation_context.clone(),
            })
            .filter(|c| !c.is_empty());
            RequestCall::Recognize(arguments(&mut a.common.clone(), question, options)?)
        }
        Command::Relate(a) => {
            let question = if let [reference] = a.relations.as_slice()
                && reference.starts_with('@')
            {
                RequestQuestion::Reference {
                    reference: reference.clone(),
                }
            } else {
                let mut spec = crate::core::RelateSpec::inline(&a.relations, a.either)
                    .map_err(relate_config)?;
                if let Some(threshold) = &a.threshold {
                    spec.override_threshold(threshold).map_err(relate_config)?;
                }
                spec.override_fields(
                    a.common.field.first().map(String::as_str),
                    a.kind_field.as_deref(),
                )
                .map_err(relate_config)?;
                RequestQuestion::Definition {
                    value: crate::Relate(spec).into(),
                }
            };
            let mut options = options(&a.common, None, None);
            options.details = false;
            options.field = None;
            RequestCall::Relate(arguments(&mut a.common.clone(), question, options)?)
        }
        _ => return Ok(()),
    };
    let request = Request::new(call);
    request
        .clone()
        .admit()
        .map_err(|error| diagnostic(error, &request))?;
    Ok(())
}
fn selector(
    text: &str,
    inline: impl FnOnce() -> Result<RequestDefinition, Failure>,
) -> Result<RequestQuestion, Failure> {
    if text.starts_with('@') {
        Ok(RequestQuestion::Reference {
            reference: text.to_owned(),
        })
    } else {
        inline().map(|value| RequestQuestion::Definition { value })
    }
}
fn definition(
    resolved: crate::core::Resolved,
    mut kind: crate::public::NativeQuestionKind,
) -> Result<RequestDefinition, Failure> {
    let core = resolved
        .question()
        .cloned()
        .ok_or(Failure::Defect("inline preparation has no question"))?;
    let threshold = resolved.threshold();
    if kind == crate::public::NativeQuestionKind::Decide && threshold.is_some_and(|v| !v.is_cut()) {
        kind = crate::public::NativeQuestionKind::Banded;
    }
    let q = crate::Question {
        metadata: resolved.metadata().clone(),
        core,
        threshold,
        authored_threshold: false,
        model: (!resolved.sources().model_is_default()).then(|| resolved.model().clone()),
        profile: resolved.profile().cloned(),
        batch: None,
        kind,
    };
    Ok(if kind == crate::public::NativeQuestionKind::Banded {
        RequestDefinition::Atomic(crate::LoadedQuestion::Banded(crate::BandedQuestion(q)))
    } else {
        q.into()
    })
}
fn source_common(common: &Common, extra: &[std::ffi::OsString]) -> Result<Common, Failure> {
    let mut common = common.clone();
    if !extra.is_empty() {
        if !common.input.is_empty() {
            return Err(Failure::Usage(
                "positional input files cannot accompany --input",
            ));
        }
        common.input = extra.iter().map(PathBuf::from).collect();
    }
    Ok(common)
}
fn options(
    common: &Common,
    batch: Option<&Batching>,
    threshold: Option<&String>,
) -> RequestOptions {
    RequestOptions {
        model: common.model.clone(),
        threshold: threshold.cloned().map(crate::RequestThreshold::Rule),
        field: (!common.field.is_empty()).then(|| common.field.clone()),
        context_field: batch.and_then(|b| b.context_field.clone()),
        batch: batch.and_then(|b| b.batch.clone()).map(|value| {
            value
                .parse::<usize>()
                .map_or_else(|_| RequestBatch::Named(value), RequestBatch::Count)
        }),
        max_requests_total: common.max_requests_total,
        details: common.details,
        ..RequestOptions::default()
    }
}
fn arguments(
    common: &mut Common,
    question: RequestQuestion,
    options: RequestOptions,
) -> Result<RequestArguments, Failure> {
    common.check_plan_name()?;
    let images = common
        .image
        .iter()
        .map(|path| RequestImage::File {
            path: path.clone(),
            media: common.image_media.as_deref().map(|media| {
                if media == "image/jpeg" {
                    crate::ImageMedia::Jpeg
                } else {
                    crate::ImageMedia::Png
                }
            }),
        })
        .collect();
    let reading = crate::ReaderOptions {
        unit: if common.window.is_some() {
            crate::SourceUnit::Window
        } else if common.unit.as_deref() == Some("file")
            || (common.unit.is_none()
                && common.window.is_none()
                && !common.input.is_empty()
                && !common.lines
                && !common.jsonl
                && !common.csv
                && !common.tsv)
        {
            crate::SourceUnit::File
        } else {
            crate::SourceUnit::Line
        },
        window: common
            .window
            .as_ref()
            .map(|v| {
                v.parse::<usize>()
                    .map_err(|_| Failure::Usage("--window takes a positive whole number"))
            })
            .transpose()?,
    };
    let input = if common.input.is_empty()
        || !common.image.is_empty()
        || common.jsonl
        || common.csv
        || common.tsv
    {
        RequestInput::Feed {
            name: "cli-input".to_owned(),
            framing: if common.csv {
                RequestFraming::Csv
            } else if common.tsv {
                RequestFraming::Tsv
            } else if common.jsonl {
                RequestFraming::Jsonl
            } else if common.lines {
                RequestFraming::Lines
            } else {
                RequestFraming::Document
            },
            reading,
            images,
        }
    } else {
        RequestInput::Source {
            source: RequestSource {
                paths: common.input.clone(),
                reading,
                media: if common.media.as_deref() == Some("image") {
                    crate::ReaderMedia::Image
                } else {
                    crate::ReaderMedia::Text
                },
            },
        }
    };
    Ok(RequestArguments {
        question,
        input,
        options,
    })
}
fn native(error: crate::Error) -> Failure {
    Failure::Image(error.detail().message().to_owned())
}

// Reuse the established CLI formatter only after shared admission has refused.
fn diagnostic(error: crate::Error, request: &Request) -> Failure {
    let options = &request.call.arguments().options;
    match error.detail().message() {
        "invalid model name" => {
            if let Some(model) = &options.model
                && let Err(failure) = super::edge::model_flag(model)
            {
                return failure;
            }
        }
        "invalid field pointer" => {
            let pointers = options
                .field
                .iter()
                .flatten()
                .map(|p| ("--field", p))
                .chain(options.options_field.iter().map(|p| ("--options", p)))
                .chain(options.context_field.iter().map(|p| ("--context-field", p)));
            for (option, pointer) in pointers {
                if let Err(cause) = crate::core::Pointer::new(pointer) {
                    return Failure::Pointer(option, crate::core::safe_key(pointer), cause);
                }
            }
        }
        "batch requires a positive whole number or max" => {
            return Failure::Usage("--batch takes max or a whole number of at least 1");
        }
        "this function does not accept this threshold"
            if request.call.function() == crate::RequestFunction::Annotate =>
        {
            return Failure::Usage(
                "--threshold belongs to each question in the question set; `annotate` takes no command-level threshold",
            );
        }
        _ => {}
    }
    native(error)
}

fn relate_config(error: crate::core::RelateConfigError) -> Failure {
    Failure::Relate(crate::failure::relate::Error::Config { file: false, error })
}
