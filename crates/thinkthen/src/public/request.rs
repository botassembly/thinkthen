//! One versioned edge contract for native and serialized requests.

mod admission;
mod definition;
mod input;
mod options;
#[cfg(test)]
mod tests;
use super::Error;
pub use admission::AdmittedRequest;
pub use definition::RequestDefinition;
pub use input::{
    RequestFraming, RequestImage, RequestInput, RequestItem, RequestOriginal, RequestSource,
};
pub use options::{RequestBatch, RequestOptions, RequestThreshold};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The admitted wire version; other versions are refused.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub enum RequestVersion {
    /// The first canonical request grammar.
    #[serde(rename = "thinkthen.request/1")]
    V1,
}

/// One closed request object. Native construction requires no JSON conversion.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Request {
    /// The explicit wire version.
    pub schema: RequestVersion,
    /// The requested judging function and its arguments.
    pub call: RequestCall,
}
impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("function", &self.call.function())
            .finish_non_exhaustive()
    }
}
impl Request {
    /// Construct the current request from typed arguments.
    #[must_use]
    pub const fn new(call: RequestCall) -> Self {
        Self {
            schema: RequestVersion::V1,
            call,
        }
    }
    /// Decode original canonical bytes without erasing repeated members.
    /// # Errors
    /// Refuses malformed, unknown, duplicate or null request controls.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        serde_json::from_str(text).map_err(|_| Error::usage("invalid canonical request"))
    }
    /// Validate the complete header without reading files or creating an engine.
    /// # Errors
    /// Refuses incompatible selectors, descriptors and controls.
    pub fn admit(self) -> Result<AdmittedRequest, Error> {
        admission::admit(self)
    }
}

/// Exactly the ten judging functions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum RequestFunction {
    /// A yes/no judgment.
    Decide,
    /// One ordered option.
    Choose,
    /// Every applicable label.
    Tag,
    /// A placement on ordered levels.
    Score,
    /// Keep passing records.
    Filter,
    /// Order records.
    Rank,
    /// Select a text unit.
    Find,
    /// Ask a named question set.
    Annotate,
    /// Identify named entities.
    Recognize,
    /// Judge entity relations.
    Relate,
}
impl RequestFunction {
    pub(super) const fn images(self) -> bool {
        matches!(self, Self::Decide | Self::Choose | Self::Score)
    }
}

/// The closed function tag selects one native invocation.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "function", rename_all = "lowercase", deny_unknown_fields)]
pub enum RequestCall {
    /// A yes/no judgment.
    Decide(RequestArguments),
    /// One ordered option.
    Choose(RequestArguments),
    /// Every applicable label.
    Tag(RequestArguments),
    /// A placement on ordered levels.
    Score(RequestArguments),
    /// Keep passing records.
    Filter(RequestArguments),
    /// Order records.
    Rank(RequestArguments),
    /// Select a text unit.
    Find(RequestArguments),
    /// Ask a named question set.
    Annotate(RequestArguments),
    /// Identify named entities.
    Recognize(RequestArguments),
    /// Judge entity relations.
    Relate(RequestArguments),
}
impl RequestCall {
    /// The function selected by the variant.
    #[must_use]
    pub const fn function(&self) -> RequestFunction {
        match self {
            Self::Decide(_) => RequestFunction::Decide,
            Self::Choose(_) => RequestFunction::Choose,
            Self::Tag(_) => RequestFunction::Tag,
            Self::Score(_) => RequestFunction::Score,
            Self::Filter(_) => RequestFunction::Filter,
            Self::Rank(_) => RequestFunction::Rank,
            Self::Find(_) => RequestFunction::Find,
            Self::Annotate(_) => RequestFunction::Annotate,
            Self::Recognize(_) => RequestFunction::Recognize,
            Self::Relate(_) => RequestFunction::Relate,
        }
    }
    /// Borrow the typed arguments regardless of the function.
    #[must_use]
    pub const fn arguments(&self) -> &RequestArguments {
        match self {
            Self::Decide(a)
            | Self::Choose(a)
            | Self::Tag(a)
            | Self::Score(a)
            | Self::Filter(a)
            | Self::Rank(a)
            | Self::Find(a)
            | Self::Annotate(a)
            | Self::Recognize(a)
            | Self::Relate(a) => a,
        }
    }
}

/// Typed question, input and optional caller controls.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RequestArguments {
    /// Explicit question selector; literal @ text remains literal.
    pub question: RequestQuestion,
    /// Explicit evidence selector.
    pub input: RequestInput,
    /// Absent controls retain existing defaults.
    #[serde(default)]
    pub options: RequestOptions,
}

/// Closed question selectors keep filesystem authority at the edge.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub enum RequestQuestion {
    /// Literal question wording.
    Text {
        /// Exact text.
        text: String,
    },
    /// Existing authored grammar or a directly converted native preparation.
    Definition {
        /// The typed prepared definition.
        value: RequestDefinition,
    },
    /// Explicit question-file path.
    File {
        /// User-selected path.
        path: PathBuf,
    },
    /// Explicit saved question name.
    Name {
        /// Native question-name grammar.
        name: String,
    },
    /// Explicit native reference lookup.
    Reference {
        /// Native reference with path precedence.
        reference: String,
    },
}
impl std::fmt::Debug for RequestQuestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestQuestion(<withheld>)")
    }
}

pub(super) fn present<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(de).map(Some)
}

impl std::fmt::Debug for RequestCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RequestCall")
            .field(&self.function())
            .finish()
    }
}
impl std::fmt::Debug for RequestArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestArguments(<withheld>)")
    }
}
