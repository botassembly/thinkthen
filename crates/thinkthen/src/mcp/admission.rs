//! Strict transport arguments; source authority and validation stay native.

use super::tools::Tool;
use crate::{CallOptions, CancelToken, Error, InputReaderOptions, ReaderMedia, ReaderOptions};
use serde::Deserialize;
use serde_json::{Value, value::RawValue};
use std::path::PathBuf;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CallParams {
    pub(super) name: Tool,
    pub(super) arguments: Arguments,
    #[serde(rename = "_meta", default)]
    _meta: Option<Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Arguments {
    #[serde(default, deserialize_with = "raw_question")]
    pub(super) question: Option<Box<RawValue>>,
    #[serde(default, deserialize_with = "present")]
    pub(super) question_file: Option<PathBuf>,
    #[serde(default, deserialize_with = "present")]
    pub(super) evidence: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) records: Option<Vec<Box<RawValue>>>,
    #[serde(default, deserialize_with = "present")]
    pub(super) source: Option<Source>,
    #[serde(default, deserialize_with = "attachments")]
    pub(super) images: Vec<PathBuf>,
    #[serde(default)]
    pub(super) options: Options,
}

impl std::fmt::Debug for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arguments").finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Source {
    pub(super) paths: Vec<PathBuf>,
    #[serde(flatten)]
    pub(super) reading: ReaderOptions,
    #[serde(default)]
    pub(super) media: ReaderMedia,
}

impl Source {
    pub(super) fn options(&self) -> InputReaderOptions {
        InputReaderOptions {
            reading: self.reading,
            media: self.media,
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Options {
    #[serde(default, deserialize_with = "present")]
    pub(super) deadline_ms: Option<i64>,
    #[serde(default, deserialize_with = "present")]
    pub(super) max_requests_total: Option<u64>,
    #[serde(default, deserialize_with = "present")]
    pub(super) context: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) field: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) context_field: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) options_field: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) model: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) batch: Option<usize>,
    #[serde(default, deserialize_with = "present")]
    pub(super) top: Option<usize>,
    #[serde(default)]
    pub(super) files_only: bool,
    #[serde(default)]
    pub(super) none: bool,
}

pub(super) struct Invocation {
    pub(super) tool: Tool,
    pub(super) arguments: Arguments,
}
impl std::fmt::Debug for Invocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Invocation")
            .field("tool", &self.tool)
            .finish_non_exhaustive()
    }
}

impl Arguments {
    fn validate_paths(&self) -> Result<(), Error> {
        if self
            .question_file
            .as_ref()
            .is_some_and(|path| path.as_os_str().is_empty())
            || self.images.iter().any(|path| path.as_os_str().is_empty())
            || self
                .source
                .as_ref()
                .is_some_and(|source| source.paths.iter().any(|path| path.as_os_str().is_empty()))
        {
            return Err(Error::usage("explicit file paths must not be empty"));
        }
        Ok(())
    }
}

impl Invocation {
    /// Admission performs no file reads and constructs no engine.
    pub(super) fn admit(params: CallParams) -> Result<Self, Error> {
        let arguments = params.arguments;
        if arguments.question.is_some() == arguments.question_file.is_some() {
            return Err(Error::usage(
                "give exactly one of question or question_file",
            ));
        }
        let inputs = usize::from(arguments.evidence.is_some())
            + usize::from(arguments.records.is_some())
            + usize::from(arguments.source.is_some());
        if inputs > 1 || (inputs == 0 && arguments.images.is_empty()) {
            return Err(Error::usage(
                "give one of evidence, records or source, or explicit images",
            ));
        }
        if !arguments.images.is_empty()
            && (arguments.records.is_some() || arguments.source.is_some())
        {
            return Err(Error::usage(
                "attachments cannot accompany records or source",
            ));
        }
        if arguments.images.len() > crate::MAX_IMAGES {
            return Err(Error::usage("image evidence requires 1 to 8 images"));
        }
        let images = !arguments.images.is_empty()
            || arguments
                .source
                .as_ref()
                .is_some_and(|s| s.media == ReaderMedia::Image);
        if images && !params.name.images() {
            return Err(Error::usage("this function takes text only"));
        }
        if let Some(source) = &arguments.source {
            if source.paths.is_empty() {
                return Err(Error::usage("source requires at least one path"));
            }
            source.options().validate()?;
        }
        arguments.validate_paths()?;
        let options = &arguments.options;
        if let Some(model) = &options.model {
            crate::core::ModelName::new(model).map_err(|_| Error::usage("invalid model name"))?;
        }
        if images
            && (options.field.is_some()
                || options.context_field.is_some()
                || options.options_field.is_some())
        {
            return Err(Error::usage("image input cannot accompany field pointers"));
        }
        if options.none && params.name != Tool::Find {
            return Err(Error::usage("none applies only to find"));
        }
        if options.top.is_some() && params.name != Tool::Rank {
            return Err(Error::usage("top applies only to rank"));
        }
        if options.files_only && (params.name != Tool::Filter || arguments.source.is_none()) {
            return Err(Error::usage("files_only requires filter source input"));
        }
        if options.batch == Some(0) {
            return Err(Error::usage("batch requires a positive whole number"));
        }
        for pointer in [
            &options.field,
            &options.context_field,
            &options.options_field,
        ]
        .into_iter()
        .flatten()
        {
            crate::core::Pointer::new(pointer)
                .map_err(|_| Error::usage("invalid field pointer"))?;
        }
        if let Some(ms) = options.deadline_ms {
            CallOptions::new().deadline_ms(ms)?;
        }
        Ok(Self {
            tool: params.name,
            arguments,
        })
    }

    /// Borrow the request token; native execution remains responsible for all
    /// request reservation, observed attempts, scheduling and started failures.
    pub(super) fn controls<'a>(&'a self, token: &'a CancelToken) -> Result<CallOptions<'a>, Error> {
        let options = &self.arguments.options;
        let mut controls = CallOptions::new()
            .cancel(token)
            .max_requests_total(options.max_requests_total);
        if let Some(ms) = options.deadline_ms {
            controls = controls.deadline_ms(ms)?;
        }
        if let Some(context) = &options.context {
            controls = controls.context(context);
        }
        if let Some(batch) = options.batch.and_then(std::num::NonZeroUsize::new) {
            controls = controls.batch(crate::BatchSetting::Records(batch));
        }
        Ok(controls)
    }
}

fn present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

fn raw_question<'de, D>(deserializer: D) -> Result<Option<Box<RawValue>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = Box::<RawValue>::deserialize(deserializer)?;
    let value: Value = serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
    if !value.is_string() && !value.is_object() {
        return Err(serde::de::Error::custom(
            "question must be literal text or an ordinary JSON question object",
        ));
    }
    Ok(Some(raw))
}

fn attachments<'de, D>(deserializer: D) -> Result<Vec<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let paths = Vec::<PathBuf>::deserialize(deserializer)?;
    if paths.is_empty() || paths.len() > crate::MAX_IMAGES {
        return Err(serde::de::Error::custom(
            "image evidence requires 1 to 8 images",
        ));
    }
    Ok(paths)
}
