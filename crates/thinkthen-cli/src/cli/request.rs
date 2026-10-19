//! CLI flags construct the native request before question or evidence reads.
use super::{
    args::{Batching, Command, Common},
    failure::Failure,
};
use crate::RequestFunction;
use crate::{
    Request, RequestArguments, RequestBatch, RequestFraming, RequestImage,
    RequestInput, RequestOptions, RequestQuestion, RequestSource,
};
use std::path::PathBuf;

mod atomic;
mod relations;

#[expect(
    clippy::too_many_lines,
    reason = "one ten-function adapter makes CLI transport choices reviewable together"
)]
pub(super) fn admit(command: &Command) -> Result<Option<crate::AdmittedRequest>, Failure> {
    let (function, arguments) = match command {
        Command::Decide(a) => {
            let mut common = source_common(&a.common, &a.extra)?;
            let mut options = options(&common, Some(&a.batching), a.threshold.as_ref());
            atomic::meanings(&mut options, &a.meanings)?;
            let question = selector(&a.question);
            (
                RequestFunction::Decide,
                arguments(&mut common, question, options)?,
            )
        }
        Command::Choose(a) => {
            let mut options = options(&a.common, Some(&a.batching), a.threshold.as_ref());
            options.labels = atomic::choice_options(a)?;
            let question = atomic::choose_selector(a)?;
            options.options_field = a.options_pointer.clone();
            (
                RequestFunction::Choose,
                arguments(&mut a.common.clone(), question, options)?,
            )
        }
        Command::Tag(a) => {
            let question = selector(&a.question);
            let mut options = options(&a.common, Some(&a.batching), a.threshold.as_ref());
            options.labels = atomic::tag_options(a)?;
            (
                RequestFunction::Tag,
                arguments(
                    &mut a.common.clone(),
                    question,
                    options,
                )?,
            )
        }
        Command::Score(a) => {
            let question = selector(&a.question);
            let mut options = options(&a.common, Some(&a.batching), a.threshold.as_ref());
            options.levels = (!a.levels.is_empty()).then(|| atomic::bare(&a.levels));
            (
                RequestFunction::Score,
                arguments(
                    &mut a.common.clone(),
                    question,
                    options,
                )?,
            )
        }
        Command::Filter(a) => {
            let mut common = source_common(&a.common, &a.extra)?;
            let question = selector(&a.question);
            let mut options = options(&common, Some(&a.batching), a.threshold.as_ref());
            atomic::meanings(&mut options, &a.meanings)?;
            options.files_only = a.files_only;
            (
                RequestFunction::Filter,
                arguments(&mut common, question, options)?,
            )
        }
        Command::Rank(a) => {
            let mut common = source_common(&a.common, &a.extra)?;
            let question = selector(&a.question);
            let mut options = options(&common, Some(&a.batching), a.threshold.as_ref());
            atomic::meanings(&mut options, &a.meanings)?;
            options.top = a
                .top
                .as_ref()
                .map(|v| v.parse::<usize>().map_err(|_| Failure::TopIsZero))
                .transpose()?;
            (
                RequestFunction::Rank,
                arguments(&mut common, question, options)?,
            )
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
            options.none = a.none;
            (
                RequestFunction::Find,
                arguments(&mut common, question, options)?,
            )
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
            let options = options(&common, Some(&a.batching), a.threshold.as_ref());
            (
                RequestFunction::Annotate,
                arguments(&mut common, question, options)?,
            )
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
            options.mode = a
                .mode
                .as_deref()
                .map(|mode| {
                    match mode {
                        "whole" => Ok(crate::RecognitionMode::Whole),
                        "boundary_only" => Ok(crate::RecognitionMode::BoundaryOnly),
                        _ => Err(Failure::Usage("recognize mode is whole or boundary_only")),
                    }
                })
                .transpose()?;
            options.threshold = a
                .threshold
                .as_ref()
                .map(|rule| crate::RequestThreshold::Rule(rule.clone()));
            options.relation_threshold = a
                .relation_threshold
                .as_ref()
                .map(|rule| crate::RequestThreshold::Rule(rule.clone()));
            options.context_field = a.context_field.clone();
            options.examples_field = a.examples_field.clone();
            options.seed_spans_field = a.seed_spans_field.clone();
            options.snippet_pieces = a.snippet_pieces;
            options.stage_context = Some(crate::RecognitionStageContext {
                boundary: a.boundary_context.clone(),
                kind_edge: a.kind_edge_context.clone(),
                relation: a.relation_context.clone(),
            })
            .filter(|c| !c.is_empty());
            (
                RequestFunction::Recognize,
                arguments(&mut a.common.clone(), question, options)?,
            )
        }
        Command::Relate(a) => {
            let question = if let [reference] = a.relations.as_slice()
                && reference.starts_with('@')
            {
                RequestQuestion::Reference {
                    reference: reference.clone(),
                }
            } else {
                RequestQuestion::Definition {
                    value: relations::inline(a)?.into(),
                }
            };
            let mut options = options(&a.common, None, None);
            options.field = None;
            (
                RequestFunction::Relate,
                arguments(&mut a.common.clone(), question, options)?,
            )
        }
        _ => return Ok(None),
    };
    Request::for_function(function, arguments)
        .admit()
        .map(Some)
        .map_err(native)
}
fn selector(text: &str) -> RequestQuestion {
    if text.starts_with('@') {
        RequestQuestion::Reference { reference: text.to_owned() }
    } else {
        RequestQuestion::Text { text: text.to_owned() }
    }
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
                && common.image.is_empty()
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
                framing: None,
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
    error.into()
}

impl crate::Engine {
    pub(crate) fn execute_cli_request<'a>(
        &self,
        request: &'a crate::AdmittedRequest,
        environment: crate::RequestEnvironment<'a>,
        sink: &dyn Fn(crate::RequestValue),
        release: &dyn Fn(usize),
        recover: crate::public::AnnotationRecovery<'a>,
    ) -> Result<crate::RequestOutcome, crate::Error> {
        self.execute_request_sink(request, environment, Some(sink), Some(release), recover)
    }
}
