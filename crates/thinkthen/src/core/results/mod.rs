//! Per-function rows keep answers and detail typed through serialization.
#![allow(
    clippy::disallowed_types,
    reason = "schema derives construct metadata; result values are typed"
)]
mod functions;
mod values;
use super::{
    Error, Probability,
    questions::{self, FunctionName},
    records::RecordId,
};
use functions::{labels, levels};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
pub use values::*;

pub(crate) mod sealed {
    pub trait Sealed {}
}
pub(crate) trait Decode: Function {
    fn answer(value: crate::core::Answer) -> Result<Self::Answer, Error>;
    fn detail(value: crate::core::Detail) -> Result<Option<Self::Detail>, Error>;
}
/// A sealed question with one answer and detail type.
///
/// Execution machinery is private even to generic consumers.
/// ```compile_fail
/// use thinkthen::results::{AnyRow, Function};
/// fn forge<F: Function>(row: AnyRow) { let _ = F::row(row); }
/// ```
pub trait Function:
    sealed::Sealed
    + Clone
    + std::fmt::Debug
    + Into<questions::Question>
    + Send
    + Sync
    + schemars::JsonSchema
    + 'static
{
    /// The answer returned by this function.
    type Answer: Clone
        + std::fmt::Debug
        + Serialize
        + serde::de::DeserializeOwned
        + schemars::JsonSchema
        + Send
        + 'static;
    /// The function's optional backend detail.
    type Detail: Clone
        + std::fmt::Debug
        + Serialize
        + serde::de::DeserializeOwned
        + schemars::JsonSchema
        + Send
        + 'static;
    /// Its stable function discriminator.
    const NAME: FunctionName;
}
super::abi::discriminators! {
    /// The result status, independent of the backend.
    #[non_exhaustive]
    #[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize,schemars::JsonSchema)]
    #[serde(rename_all="lowercase")]
    pub enum Status {
        /// An accepted answer.
        Answered=1,
        /// A result inside the threshold band.
        Unsure=2,
        /// One record failed.
        Failed=3,
    }
}
/// A status-specific answer with no untyped payload.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "status",
    rename_all = "lowercase",
    deny_unknown_fields,
    bound(deserialize = "A:Deserialize<'de>, D:Deserialize<'de>")
)]
#[schemars(bound = "A:schemars::JsonSchema,D:schemars::JsonSchema")]
pub enum Outcome<A, D> {
    /// An accepted typed answer.
    Answered {
        /// The answer, including explicit null when find returns no match.
        #[serde(deserialize_with = "required_answer")]
        #[schemars(required, schema_with = "answer_schema::<A>")]
        answer: A,
        /// A defined aggregate probability, when the function has one.
        #[serde(
            default,
            deserialize_with = "optional_value",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "Probability")]
        probability: Option<Probability>,
        /// The function's complete detail.
        #[serde(
            default,
            deserialize_with = "optional_value",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "D")]
        detail: Option<D>,
    },
    /// A result inside a threshold band, without an answer key.
    Unsure {
        /// The probability behind the result.
        probability: Probability,
        /// The function's complete detail.
        #[serde(
            default,
            deserialize_with = "optional_value",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "D")]
        detail: Option<D>,
    },
    /// A failure belonging to this record.
    Failed {
        /// The typed failure.
        #[serde(deserialize_with = "record_error")]
        #[schemars(schema_with = "record_error_schema")]
        error: Error,
    },
}
impl<A, D> Outcome<A, D> {
    /// The status of this outcome.
    #[must_use]
    pub const fn status(&self) -> Status {
        match self {
            Self::Answered { .. } => Status::Answered,
            Self::Unsure { .. } => Status::Unsure,
            Self::Failed { .. } => Status::Failed,
        }
    }
}
/// The execution question and attempted route, including on failed rows.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Source {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    pub(crate) question_identity: String,
    pub(crate) backend: crate::core::BackendName,
    pub(crate) model: String,
}
impl Source {
    /// The caller's execution name, independent of toolbox selection.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    /// The stable caller-visible question identity.
    #[must_use]
    pub fn question_identity(&self) -> &str {
        &self.question_identity
    }
    /// The attempted backend.
    #[must_use]
    pub const fn backend(&self) -> crate::core::BackendName {
        self.backend
    }
    /// The resolved model.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }
}
#[derive(Debug, Clone)]
struct FunctionTag<F>(std::marker::PhantomData<F>);
impl<F: Function> Serialize for FunctionTag<F> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        F::NAME.serialize(s)
    }
}
impl<'de, F: Function> Deserialize<'de> for FunctionTag<F> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let name = FunctionName::deserialize(d)?;
        if name != F::NAME {
            return Err(serde::de::Error::custom(
                "row function does not match its type",
            ));
        }
        Ok(Self(std::marker::PhantomData))
    }
}
impl<F: Function> schemars::JsonSchema for FunctionTag<F> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("{:?}Function", F::NAME).into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"string","const":F::NAME})
    }
}
/// One result with its original input position and optional exact id.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
#[serde(bound(serialize = ""))]
#[schemars(bound = "", extend("unevaluatedProperties" = false))]
pub struct Row<F: Function> {
    function: FunctionTag<F>,
    position: NonZeroU64,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<RecordId>,
    source: Source,
    #[serde(flatten)]
    outcome: Outcome<F::Answer, F::Detail>,
}
impl<F: Function> Row<F> {
    pub(crate) fn new(
        position: NonZeroU64,
        id: Option<RecordId>,
        source: Source,
        outcome: Outcome<F::Answer, F::Detail>,
    ) -> Self {
        Self {
            function: FunctionTag(std::marker::PhantomData),
            position,
            id,
            source,
            outcome,
        }
    }
    /// The stable function discriminator.
    #[must_use]
    pub const fn function(&self) -> FunctionName {
        F::NAME
    }
    /// The one-based original input ordinal, independent of ranking.
    #[must_use]
    pub const fn position(&self) -> u64 {
        self.position.get()
    }
    /// The caller's exact identity, when present.
    #[must_use]
    pub const fn id(&self) -> Option<&RecordId> {
        self.id.as_ref()
    }
    /// The attempted question and route.
    #[must_use]
    pub const fn source(&self) -> &Source {
        &self.source
    }
    /// The typed status-specific result.
    #[must_use]
    pub const fn outcome(&self) -> &Outcome<F::Answer, F::Detail> {
        &self.outcome
    }
    /// The result status.
    #[must_use]
    pub const fn status(&self) -> Status {
        self.outcome.status()
    }
    /// The function's defined aggregate probability, when present.
    #[must_use]
    pub fn probability(&self) -> Option<Probability> {
        match &self.outcome {
            Outcome::Answered { probability, .. } => *probability,
            Outcome::Unsure { probability, .. } => Some(*probability),
            Outcome::Failed { .. } => None,
        }
    }
    /// The error belonging to this failed record.
    #[must_use]
    pub fn error(&self) -> Option<&Error> {
        match &self.outcome {
            Outcome::Failed { error } => Some(error),
            _ => None,
        }
    }
    /// Choose to return a record failure as an error. Unsure remains no answer.
    /// # Errors
    /// Returns this record's error for a failed outcome.
    pub fn into_result(self) -> Result<Option<F::Answer>, Error> {
        match self.outcome {
            Outcome::Answered { answer, .. } => Ok(Some(answer)),
            Outcome::Unsure { .. } => Ok(None),
            Outcome::Failed { error } => Err(error),
        }
    }
    pub(crate) fn from_outcome(
        position: NonZeroU64,
        id: Option<RecordId>,
        source: Source,
        outcome: crate::core::Outcome,
    ) -> Result<Self, Error>
    where
        F: Decode,
    {
        let outcome = admit_outcome::<F>(outcome)?;
        Ok(Self::new(position, id, source, outcome))
    }
}
impl Row<questions::Filter> {
    /// Only an answered true result keeps the original item.
    #[must_use]
    pub fn kept(&self) -> bool {
        matches!(self.outcome, Outcome::Answered { answer: true, .. })
    }
}

pub(crate) fn admit_outcome<F: Function + Decode>(
    outcome: crate::core::Outcome,
) -> Result<Outcome<F::Answer, F::Detail>, Error> {
    let detail =
        |value: Option<crate::core::Detail>| value.map(F::detail).transpose().map(Option::flatten);
    let outcome = match outcome {
        crate::core::Outcome::Answered {
            answer,
            p,
            detail: raw,
        } => Outcome::Answered {
            answer: F::answer(answer)?,
            probability: p,
            detail: detail(raw)?,
        },
        crate::core::Outcome::Unsure { p, detail: raw } => Outcome::Unsure {
            probability: p,
            detail: detail(raw)?,
        },
        crate::core::Outcome::Failed { error } => {
            if !error.code().is_record() {
                return Err(error);
            }
            Outcome::Failed { error }
        }
    };
    Ok(outcome)
}

pub(crate) fn mismatch() -> Error {
    Error::from_contract(
        super::Code::InternalBug,
        "the execution result does not match its admitted function",
    )
}

mod registry;
pub(crate) use registry::function_set;
crate::core::questions::result_contract!();
mod admission;
use admission::RowWire;

fn record_error<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Error, D::Error> {
    let error = Error::deserialize(d)?;
    if !error.code().is_record() {
        return Err(serde::de::Error::custom(
            "a call-level error cannot be a failed row",
        ));
    }
    Ok(error)
}
fn record_error_schema(g: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut schema = <Error as schemars::JsonSchema>::json_schema(g);
    if let Some(properties) = schema
        .as_object_mut()
        .and_then(|fields| fields.get_mut("properties"))
        .and_then(|value| value.as_object_mut())
    {
        properties.insert("code".into(),serde_json::json!({"type":"string","enum":super::Code::ALL.iter().filter(|code|code.is_record()).map(|code|code.as_str()).collect::<Vec<_>>()}));
    }
    schema
}

fn required_answer<'de, T: Deserialize<'de>, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<T, D::Error> {
    T::deserialize(d)
}
fn optional_value<'de, T: Deserialize<'de>, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}

fn answer_schema<A: schemars::JsonSchema>(g: &mut schemars::SchemaGenerator) -> schemars::Schema {
    A::json_schema(g)
}
