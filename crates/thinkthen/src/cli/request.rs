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
pub(super) fn admit(command: &Command) -> Result<Option<crate::AdmittedRequest>, Failure> {
    let mut atomic = None;
    let call = match command {
        Command::Decide(a) => {
            let mut common = source_common(&a.common, &a.extra)?;
            let options = options(&common, Some(&a.batching), a.threshold.as_ref());
            let question = selector(&a.question, || {
                atomic_definition(asked::decide(a, None)?, &mut atomic)
            })?;
            RequestCall::Decide(arguments(&mut common, question, options)?)
        }
        Command::Choose(a) => {
            let question = selector(&a.question, || {
                atomic_definition(asked::choose(a, None)?, &mut atomic)
            })?;
            let mut options = options(&a.common, Some(&a.batching), a.threshold.as_ref());
            options.options_field = a.options_pointer.clone();
            RequestCall::Choose(arguments(&mut a.common.clone(), question, options)?)
        }
        Command::Tag(a) => {
            let question = selector(&a.question, || {
                atomic_definition(asked::tag(a, None)?, &mut atomic)
            })?;
            RequestCall::Tag(arguments(
                &mut a.common.clone(),
                question,
                options(&a.common, Some(&a.batching), a.threshold.as_ref()),
            )?)
        }
        Command::Score(a) => {
            let question = selector(&a.question, || {
                atomic_definition(asked::score(a, None)?, &mut atomic)
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
                atomic_definition(asked::filter(a, None)?, &mut atomic)
            })?;
            let mut options = options(&common, Some(&a.batching), a.threshold.as_ref());
            options.details = false;
            options.files_only = a.files_only;
            RequestCall::Filter(arguments(&mut common, question, options)?)
        }
        Command::Rank(a) => {
            let mut common = source_common(&a.common, &a.extra)?;
            let question = selector(&a.question, || {
                let (resolved, tier, rank_set) = asked::rank(a, None)?;
                let prepared = crate::cli_atomic::Prepared {
                    resolved,
                    batch: tier.batch,
                    tuned: tier.tuned,
                    rank_set,
                };
                let definition = crate::cli_atomic::definition(&prepared).map_err(native)?;
                atomic = Some(prepared);
                Ok(definition)
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
            options.mode = a
                .mode
                .as_deref()
                .map(|mode| {
                    crate::RecognitionMode::parse(mode)
                        .ok_or(Failure::Usage(crate::RecognitionMode::USAGE))
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
        _ => return Ok(None),
    };
    Request::new(call)
        .admit()
        .map(|admitted| Some(admitted.retain_cli_atomic(atomic)))
        .map_err(native)
}
fn atomic_definition(
    (resolved, tier): (crate::core::Resolved, asked::FileTier),
    retained: &mut Option<crate::cli_atomic::Prepared>,
) -> Result<RequestDefinition, Failure> {
    let prepared = crate::cli_atomic::Prepared {
        resolved,
        batch: tier.batch,
        tuned: tier.tuned,
        rank_set: None,
    };
    let definition = crate::cli_atomic::definition(&prepared).map_err(native)?;
    *retained = Some(prepared);
    Ok(definition)
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

fn relate_config(error: crate::core::RelateConfigError) -> Failure {
    Failure::Relate(crate::failure::relate::Error::Config { file: false, error })
}
