//! Strict transport arguments; source authority and validation stay native.

use super::tools::Tool;
use crate::{CallOptions, CancelToken, Error, ReaderMedia, ReaderOptions};
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

impl Invocation {
    /// Translate the closed transport carriers, then admit through Request without reads.
    pub(super) fn admit(params: CallParams) -> Result<Self, Error> {
        let invocation = Self {
            tool: params.name,
            arguments: params.arguments,
        };
        invocation.request()?;
        Ok(invocation)
    }

    pub(super) fn call_token(&self, notification: &CancelToken) -> CancelToken {
        if self.arguments.options.cancelled {
            let initial = CancelToken::new();
            initial.cancel();
            initial
        } else {
            notification.clone()
        }
    }

    /// Borrow the call token; native execution remains responsible for all
    /// request reservation, observed attempts, scheduling and started failures.
    pub(super) fn controls<'a>(&'a self, token: &'a CancelToken) -> Result<CallOptions<'a>, Error> {
        let mut controls = CallOptions::new()
            .cancel(token)
            .surface(crate::Surface::Mcp);
        if self.arguments.options.proxy.is_some() {
            controls = controls.proxy(&crate::ProxyActivation::Empty);
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

pub(super) type Reading = crate::RequestThreshold;

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

pub(super) type Batch = crate::RequestBatch;
