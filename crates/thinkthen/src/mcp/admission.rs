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
    pub(super) question_name: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) question_reference: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) evidence: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) records: Option<Vec<Box<RawValue>>>,
    #[serde(default, deserialize_with = "present")]
    pub(super) source: Option<Source>,
    #[serde(default, deserialize_with = "present")]
    pub(super) inputs: Option<Vec<super::inputs::Descriptor>>,
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
    #[serde(default)]
    pub(super) cancelled: bool,
    #[serde(default, deserialize_with = "present")]
    pub(super) deadline_ms: Option<i64>,
    #[serde(default, deserialize_with = "present")]
    pub(super) max_requests_total: Option<u64>,
    #[serde(default, deserialize_with = "present")]
    pub(super) context: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) field: Option<Fields>,
    #[serde(default, deserialize_with = "present")]
    pub(super) context_field: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) options_field: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) model: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub(super) threshold: Option<Reading>,
    #[serde(default)]
    pub(super) attempts: bool,
    #[serde(default, deserialize_with = "present")]
    pub(super) proxy: Option<Box<RawValue>>,
    #[serde(default, deserialize_with = "present")]
    pub(super) batch: Option<Batch>,
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
    fn validate_descriptors(&self, tool: Tool) -> Result<(), Error> {
        let inputs = usize::from(self.evidence.is_some())
            + usize::from(self.records.is_some())
            + usize::from(self.source.is_some())
            + usize::from(self.inputs.is_some());

        if self.inputs.is_some() && !self.images.is_empty() {
            return Err(Error::usage("inputs cannot accompany top-level images"));
        }
        if let Some(descriptors) = &self.inputs {
            for descriptor in descriptors {
                descriptor.admit(tool, &self.options)?;
            }
        }
        if inputs > 1 || (inputs == 0 && self.images.is_empty()) {
            return Err(Error::usage(
                "give one of evidence, records, source or inputs, or explicit images",
            ));
        }
        Ok(())
    }
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
        if usize::from(arguments.question.is_some())
            + usize::from(arguments.question_file.is_some())
            + usize::from(arguments.question_name.is_some())
            + usize::from(arguments.question_reference.is_some())
            != 1
        {
            return Err(Error::usage(
                "give exactly one question, file, name or explicit reference",
            ));
        }
        arguments.validate_descriptors(params.name)?;
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
        if let Some(name) = &arguments.question_name {
            crate::QuestionName::new(name)?;
        }
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
        options.validate_reading(params.name)?;
        if options.none && params.name != Tool::Find {
            return Err(Error::usage("none applies only to find"));
        }
        if options.top.is_some() && params.name != Tool::Rank {
            return Err(Error::usage("top applies only to rank"));
        }
        if options.files_only && (params.name != Tool::Filter || arguments.source.is_none()) {
            return Err(Error::usage("files_only requires filter source input"));
        }
        if let Some(batch) = &options.batch {
            batch.native()?;
        }
        for pointer in options.field.iter().flat_map(Fields::values).chain(
            [&options.context_field, &options.options_field]
                .into_iter()
                .flatten()
                .map(String::as_str),
        ) {
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
            .surface(crate::Surface::Mcp)
            .attempts(options.attempts)
            .max_requests_total(options.max_requests_total);
        if options.proxy.is_some() {
            controls = controls.proxy(&crate::ProxyActivation::Empty);
        }
        if let Some(ms) = options.deadline_ms {
            controls = controls.deadline_ms(ms)?;
        }
        if let Some(context) = &options.context {
            controls = controls.context(context);
        }
        if let Some(batch) = &options.batch {
            controls = controls.batch(batch.native()?);
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
    if paths.is_empty() {
        return Err(serde::de::Error::custom(
            "image evidence requires 1 to 8 images",
        ));
    }
    Ok(paths)
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum Reading {
    Cut(f64),
    Rule(String),
}
impl Reading {
    pub(super) fn native(&self) -> Result<crate::core::Threshold, Error> {
        match self {
            Self::Cut(value) => crate::core::Threshold::cut(*value),
            Self::Rule(value) => value.parse(),
        }
        .map_err(Error::refused)
    }
}

impl Options {
    fn validate_reading(&self, tool: Tool) -> Result<(), Error> {
        if let Some(reading) = &self.threshold {
            let rule = reading.native()?;
            if !matches!(tool, Tool::Decide | Tool::Choose | Tool::Tag | Tool::Filter)
                || (tool != Tool::Decide && !rule.is_cut())
            {
                return Err(Error::usage("this function does not accept this threshold"));
            }
        }
        if self.options_field.is_some() && tool != Tool::Choose {
            return Err(Error::usage("options_field applies only to choose"));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum Fields {
    One(String),
    Many(Vec<String>),
}
impl Fields {
    pub(super) fn values(&self) -> impl Iterator<Item = &str> {
        let fields = match self {
            Self::One(field) => std::slice::from_ref(field),
            Self::Many(fields) => fields.as_slice(),
        };
        fields.iter().map(String::as_str)
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum Batch {
    Count(usize),
    Named(String),
}
impl Batch {
    pub(super) fn native(&self) -> Result<crate::BatchSetting, Error> {
        let setting = match self {
            Self::Count(count) => crate::core::Setting::parse(&count.to_string()),
            Self::Named(name) if name == "max" => Some(crate::core::Setting::Max),
            Self::Named(_) => None,
        };
        match setting {
            Some(crate::core::Setting::Max) => Ok(crate::BatchSetting::Max),
            Some(crate::core::Setting::Records(count)) => Ok(crate::BatchSetting::Records(count)),
            None => Err(Error::usage(
                "batch requires a positive whole number or max",
            )),
        }
    }
}
