//! One versioned edge contract for native and serialized requests.

mod admission;
mod column;
mod question_parts;
pub use column::{RequestColumn, RequestColumnValues, RequestColumnView};
mod composition;
mod decoding;
mod definition;
pub use decoding::RequestDecodeCause;
mod execution;
#[cfg(test)]
pub(in crate::public) use execution::native_atomic as atomic;
pub(in crate::public) use execution::{native, restore, restore_found, scalar};
mod framing;
mod inline;
mod input;
mod native_feed;
mod options;
mod preview;
pub(crate) mod pull;
pub use pull::RequestRows;
mod result;
mod source;
pub use source::RequestSourceReader;
mod session;
mod session_queue;
mod session_result;
#[cfg(test)]
pub(crate) use session_result::SessionPacketDocument;
#[cfg(test)]
mod tests;
pub(crate) mod transport;
use super::{Error, LoadedQuestion};
pub use admission::AdmittedRequest;
pub use definition::RequestDefinition;
pub use execution::{RequestEnvironment, RequestFeed};
pub use input::{
    RequestFraming, RequestImage, RequestInput, RequestItem, RequestOriginal, RequestSource,
};
pub use options::{RequestBatch, RequestOptions, RequestThreshold};
pub use result::{RequestOutcome, RequestValue};
use serde::{Deserialize, Serialize};
pub use session::{
    RequestReaderFailure, RequestSession, RequestSessionDescriptor, RequestSessionFeedOptions,
    RequestSessionPush, RequestSessionPushStatus, RequestSessionRead, RequestSessionReady,
    RequestSessionWaitInterest,
};
pub use session_result::{RequestSessionResult, RequestSessionRow, RequestSessionTerminal};
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
    pub fn new(mut call: RequestCall) -> Self {
        let function = call.function();
        native_controls(call.arguments_mut(), function);
        Self {
            schema: RequestVersion::V1,
            call,
        }
    }
    /// Construct a named call and select the collection kind for its function.
    /// # Errors
    /// Refuses an unknown function name.
    pub fn named(function: &str, arguments: RequestArguments) -> Result<Self, Error> {
        Ok(Self::for_function(function.parse()?, arguments))
    }
    /// Construct a named call from the existing canonical component grammars.
    /// # Errors
    /// Refuses unknown names, malformed components and unsupported declarations.
    pub fn named_from_json(
        function: &str,
        question: &str,
        input: &str,
        options: &str,
    ) -> Result<Self, Error> {
        let function = function.parse()?;
        Ok(Self::for_function(
            function,
            RequestArguments {
                question: RequestQuestion::from_json(question)?,
                input: RequestInput::from_json(input)?,
                options: decoding::decode(options, |_| Ok(()))?,
            },
        ))
    }
    /// Construct a typed call, deriving collection kinds from the function.
    #[must_use]
    pub fn for_function(function: RequestFunction, mut arguments: RequestArguments) -> Self {
        if let RequestInput::Records { items } = arguments.input {
            arguments.input = match function {
                RequestFunction::Find => RequestInput::Units { items },
                RequestFunction::Relate => RequestInput::Entities { items },
                _ => RequestInput::Records { items },
            };
        }
        let call = RequestCall::new(function, arguments);
        Self::new(call)
    }
    /// Construct a call from the kind of a prepared question definition.
    #[must_use]
    pub fn for_definition(
        definition: RequestDefinition,
        input: RequestInput,
        options: RequestOptions,
    ) -> Self {
        let function = RequestFunction::for_definition(&definition);
        Self::for_function(
            function,
            RequestArguments {
                question: RequestQuestion::Definition { value: definition },
                input,
                options,
            },
        )
    }
    #[cfg(feature = "cli")]
    pub(crate) fn admit_for_transport(
        self,
        limit: transport::TransportAttachmentLimit,
    ) -> Result<AdmittedRequest, Error> {
        admission::admit(self, Some(limit))
    }
    /// Decode original canonical bytes without erasing repeated members.
    /// # Errors
    /// Refuses malformed, unknown, duplicate or null request controls.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        decoding::decode(text, |text| {
            definition::validate_wire_definition(text)
                .and_then(|()| decoding::validate_request_contexts(text))
        })
    }

    /// Validate the complete header without reading files or creating an engine.
    /// # Errors
    /// Refuses incompatible selectors, descriptors and controls.
    pub fn admit(self) -> Result<AdmittedRequest, Error> {
        admission::admit(self, None)
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
impl std::str::FromStr for RequestFunction {
    type Err = Error;
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "decide" => Ok(Self::Decide),
            "choose" => Ok(Self::Choose),
            "tag" => Ok(Self::Tag),
            "score" => Ok(Self::Score),
            "filter" => Ok(Self::Filter),
            "rank" => Ok(Self::Rank),
            "find" => Ok(Self::Find),
            "annotate" => Ok(Self::Annotate),
            "recognize" => Ok(Self::Recognize),
            "relate" => Ok(Self::Relate),
            _ => Err(Error::usage("unknown function")),
        }
    }
}
impl RequestFunction {
    /// Select the function of a prepared question definition.
    #[must_use]
    pub fn for_definition(definition: &RequestDefinition) -> Self {
        match definition {
            RequestDefinition::Atomic(LoadedQuestion::Banded(_)) => Self::Decide,
            RequestDefinition::Atomic(LoadedQuestion::Question(q)) => match q.kind() {
                crate::QuestionKind::Decide => Self::Decide,
                crate::QuestionKind::Choose => Self::Choose,
                crate::QuestionKind::Tag => Self::Tag,
                crate::QuestionKind::Score => Self::Score,
                crate::QuestionKind::Rank => Self::Rank,
                crate::QuestionKind::Find => Self::Find,
            },
            RequestDefinition::Annotate(_) | RequestDefinition::DecodedSet { .. } => Self::Annotate,
            RequestDefinition::Rank(_) | RequestDefinition::RankSet(_) => Self::Rank,
            RequestDefinition::Find(_) => Self::Find,
            RequestDefinition::Recognize(_) | RequestDefinition::Recognition(_) => Self::Recognize,
            RequestDefinition::Relate(_) => Self::Relate,
            RequestDefinition::DynamicChoose(_) => Self::Choose,
        }
    }
    pub(crate) const fn streams(self) -> bool {
        matches!(
            self,
            Self::Decide | Self::Choose | Self::Tag | Self::Score | Self::Filter | Self::Annotate
        )
    }

    pub(crate) const fn input_function(self) -> super::InputFunction {
        match self {
            Self::Decide => super::InputFunction::Decide,
            Self::Choose => super::InputFunction::Choose,
            Self::Tag => super::InputFunction::Tag,
            Self::Score => super::InputFunction::Score,
            Self::Filter => super::InputFunction::Filter,
            Self::Rank => super::InputFunction::Rank,
            Self::Find => super::InputFunction::Find,
            Self::Annotate => super::InputFunction::Annotate,
            Self::Recognize => super::InputFunction::Recognize,
            Self::Relate => super::InputFunction::Relate,
        }
    }
    pub(crate) const fn from_input(function: super::InputFunction) -> Self {
        match function {
            super::InputFunction::Decide => Self::Decide,
            super::InputFunction::Choose => Self::Choose,
            super::InputFunction::Score => Self::Score,
            super::InputFunction::Tag => Self::Tag,
            super::InputFunction::Filter => Self::Filter,
            super::InputFunction::Rank => Self::Rank,
            super::InputFunction::Annotate => Self::Annotate,
            super::InputFunction::Find => Self::Find,
            super::InputFunction::Recognize => Self::Recognize,
            super::InputFunction::Relate => Self::Relate,
        }
    }
    #[cfg(feature = "cli")]
    pub(crate) const ALL: [Self; 10] = [
        Self::Decide,
        Self::Choose,
        Self::Tag,
        Self::Score,
        Self::Filter,
        Self::Rank,
        Self::Find,
        Self::Annotate,
        Self::Recognize,
        Self::Relate,
    ];
    pub(crate) fn allows_option(self, name: &str) -> bool {
        match name {
            "threshold" => matches!(
                self,
                Self::Decide
                    | Self::Choose
                    | Self::Tag
                    | Self::Filter
                    | Self::Rank
                    | Self::Recognize
            ),
            "options_field" => self == Self::Choose,
            "examples" | "examples_field" | "seed_spans" | "seed_spans_field" | "stage_context"
            | "snippet_pieces" => self == Self::Recognize,
            "none" => self == Self::Find,
            "top" => self == Self::Rank,
            "files_only" => self == Self::Filter,
            "details" => matches!(self, Self::Decide | Self::Choose | Self::Tag | Self::Score),
            _ => true,
        }
    }
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Decide => "decide",
            Self::Choose => "choose",
            Self::Tag => "tag",
            Self::Score => "score",
            Self::Filter => "filter",
            Self::Rank => "rank",
            Self::Find => "find",
            Self::Annotate => "annotate",
            Self::Recognize => "recognize",
            Self::Relate => "relate",
        }
    }
    pub(crate) const fn images(self) -> bool {
        matches!(self, Self::Decide | Self::Choose | Self::Score)
    }
}

/// One closed function tag with the same argument object for every function.
#[derive(Clone, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RequestCall {
    /// The requested judging function.
    pub function: RequestFunction,
    /// The shared typed question, input and caller controls.
    #[serde(flatten)]
    pub arguments: RequestArguments,
}
impl RequestCall {
    /// Construct one typed invocation.
    #[must_use]
    pub const fn new(function: RequestFunction, arguments: RequestArguments) -> Self {
        Self {
            function,
            arguments,
        }
    }
    /// The function selected by the call.
    #[must_use]
    pub const fn function(&self) -> RequestFunction {
        self.function
    }
    pub(crate) fn arguments_mut(&mut self) -> &mut RequestArguments {
        &mut self.arguments
    }
    /// Borrow the shared typed arguments.
    #[must_use]
    pub const fn arguments(&self) -> &RequestArguments {
        &self.arguments
    }
}
impl<'de> Deserialize<'de> for RequestCall {
    fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        // A flat closed decoder refuses unknown fields; flatten cannot enforce it.
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Document {
            function: RequestFunction,
            question: RequestQuestion,
            input: RequestInput,
            #[serde(default)]
            options: RequestOptions,
        }
        struct Object;
        impl<'de> serde::de::Visitor<'de> for Object {
            type Value = Document;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a request call object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Document, A::Error> {
                Document::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            }
        }
        let document = decoder.deserialize_map(Object)?;
        Ok(Self::new(
            document.function,
            RequestArguments {
                question: document.question,
                input: document.input,
                options: document.options,
            },
        ))
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
#[expect(
    clippy::large_enum_variant,
    reason = "native preparations enter directly through the public typed definition variant"
)]
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

fn native_controls(arguments: &mut RequestArguments, function: RequestFunction) {
    let RequestQuestion::Definition { value } = &arguments.question else {
        return;
    };
    if let RequestDefinition::Recognition(q) = value
        && arguments.options.examples.is_none()
        && !q.examples().is_empty()
    {
        arguments.options.examples = Some(q.examples().to_vec());
    }
    if let RequestDefinition::DynamicChoose(q) = value
        && !q.authored_threshold
        && arguments.options.threshold.is_none()
    {
        arguments.options.threshold = q
            .threshold
            .map(|rule| RequestThreshold::Cut(rule.bounds().0));
    }
    let RequestDefinition::Atomic(LoadedQuestion::Question(q)) = value else {
        return;
    };
    if q.kind == crate::public::NativeQuestionKind::FindNone {
        arguments.options.none = true;
    }
    if function != RequestFunction::Rank
        && !q.authored_threshold
        && arguments.options.threshold.is_none()
    {
        arguments.options.threshold = q.threshold.map(|rule| {
            if rule.is_cut() {
                RequestThreshold::Cut(rule.bounds().0)
            } else {
                RequestThreshold::Rule(rule.to_string())
            }
        });
    }
}

/// Admit a borrowed native call without collecting its caller-owned originals.
pub(crate) fn admit_native(
    function: RequestFunction,
    definition: RequestDefinition,
) -> Result<AdmittedRequest, crate::Error> {
    if let RequestDefinition::Rank(q) = &definition {
        crate::public::engine::only(
            q,
            &[
                crate::public::NativeQuestionKind::Rank,
                crate::public::NativeQuestionKind::Score,
            ],
            "rank",
        )?;
    }
    if let RequestDefinition::Rank(q) = &definition
        && (q.threshold.is_some_and(|rule| !rule.is_cut())
            || (q.kind() == crate::QuestionKind::Score && q.threshold.is_some()))
    {
        return Err(Error::usage(
            "rank cutoff requires a decide probability and never a band",
        ));
    }
    let request = Request::for_function(
        function,
        RequestArguments {
            question: RequestQuestion::Definition { value: definition },
            input: RequestInput::Feed {
                name: "native".into(),
                framing: RequestFraming::Document,
                reading: crate::ReaderOptions::default(),
                images: vec![],
            },
            options: RequestOptions::default(),
        },
    );
    admission::admit_native(request)
}
