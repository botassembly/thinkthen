//! Per-function rows keep answers and detail typed through serialization.
#![allow(
    clippy::disallowed_types,
    reason = "schema derives construct metadata; result values are typed"
)]
mod answer;
mod entry;
pub use entry::Entry;
mod domains;
mod functions;
pub(crate) mod offsets;
mod shape;
mod values;
use super::{
    Error, Probability,
    questions::{self, FunctionName},
    records::RecordId,
};
pub use answer::{Annotations, NoAnswer};
use functions::{labels, levels};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
pub use values::*;

pub(crate) mod sealed {
    pub trait Sealed {
        fn project(
            row: super::AnyRow,
            token: super::Projection,
        ) -> Result<super::Row<Self>, super::Error>
        where
            Self: super::Function + Sized;
        fn entry(
            entry: &super::AnnotationEntry,
            _: super::Projection,
        ) -> Result<super::Outcome<Self::Answer, Self::Detail>, super::Error>
        where
            Self: super::Function + Sized,
        {
            entry
                .outcome_any()
                .downcast_ref::<super::Outcome<Self::Answer, Self::Detail>>()
                .cloned()
                .ok_or_else(super::mismatch)
        }
    }
    pub(super) trait Failures {
        fn failure(&self) -> Option<&super::Error> {
            None
        }
    }
    /// The plain text a row prints for its answer.
    pub trait Text {
        fn text(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
}
/// An unconstructible capability keeping execution projection inside the engine.
#[doc(hidden)]
#[derive(Debug)]
pub struct Projection {
    _private: (),
}
impl Projection {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }
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
        + PartialEq
        + Serialize
        + serde::de::DeserializeOwned
        + schemars::JsonSchema
        + Send
        + 'static;
    /// The plain owned value returned when taking an answer.
    type Owned;
    /// Take the public value from its validated stored representation.
    #[doc(hidden)]
    fn take(answer: Self::Answer) -> Self::Owned;
    /// The plain value or borrow returned by an answer read.
    type Read<'a>
    where
        Self: 'a;
    /// Read the plain value without changing its owned representation.
    #[doc(hidden)]
    fn read(answer: &Self::Answer) -> Self::Read<'_>;
    /// Format this function's answer as the command does.
    #[doc(hidden)]
    fn format(answer: &Self::Answer, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    /// The first failed child of a form, if any.
    #[doc(hidden)]
    fn answer_failure(_: &Self::Answer) -> Option<&Error> {
        None
    }
    /// The function's optional backend detail.
    type Detail: Clone
        + std::fmt::Debug
        + PartialEq
        + Serialize
        + serde::de::DeserializeOwned
        + schemars::JsonSchema
        + Send
        + 'static;
    /// Its stable function discriminator.
    const NAME: FunctionName;
}
/// A question supported by one-record and ordered many-record calls.
/// Filter and rank use their collection operations instead.
/// ```compile_fail
/// # let engine = thinkthen::Settings::default().engine(|_| None).unwrap();
/// let filter = thinkthen::Filter::new("Keep?").unwrap();
/// engine.run(&filter, "evidence");
/// ```
/// ```compile_fail
/// # let engine = thinkthen::Settings::default().engine(|_| None).unwrap();
/// let rank = thinkthen::Rank::with_strings("How urgent?", ["low", "high"]).unwrap();
/// engine.rows(&rank, ["evidence"]);
/// ```
pub trait SingleFunction: Function {}
super::abi::discriminators! {
    /// The result status, independent of the backend.
    ///
    /// The set is closed, so a `match` with one arm for each status compiles
    /// without a catch-all arm, and the compiler flags a missed not-sure case.
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
///
/// The set is closed: an exhaustive `match` needs no catch-all arm.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
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
impl<A, D> Outcome<A, D> {
    pub(crate) fn map_answer<B>(
        self,
        map: impl FnOnce(A) -> Result<B, Error>,
    ) -> Result<Outcome<B, D>, Error> {
        Ok(match self {
            Self::Answered {
                answer,
                probability,
                detail,
            } => Outcome::Answered {
                answer: map(answer)?,
                probability,
                detail,
            },
            Self::Unsure {
                probability,
                detail,
            } => Outcome::Unsure {
                probability,
                detail,
            },
            Self::Failed { error } => Outcome::Failed { error },
        })
    }
}
/// The execution question and attempted route, including on failed rows.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Source {
    #[serde(
        default,
        deserialize_with = "optional_value",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
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
#[derive(Clone)]
struct FunctionTag<F>(std::marker::PhantomData<F>);
impl<F: Function> std::fmt::Debug for FunctionTag<F> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&F::NAME, formatter)
    }
}
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
#[schemars(bound = "", extend("unevaluatedProperties" = false), transform = shape::saved_item)]
pub struct Row<F: Function, T = ()> {
    #[serde(skip)]
    #[schemars(skip)]
    item: T,
    function: FunctionTag<F>,
    position: NonZeroU64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "RecordId")]
    id: Option<RecordId>,
    source: Source,
    #[serde(skip)]
    #[schemars(skip)]
    original: Option<Box<super::records::Record>>,
    #[serde(flatten)]
    #[schemars(schema_with = "shape::outcome_schema::<F>")]
    outcome: Outcome<F::Answer, F::Detail>,
}
impl Row<questions::Rank> {
    /// The owned original input retained with this ranked result.
    /// Deserialized rows do not carry an original input.
    #[must_use]
    pub fn record(&self) -> Option<&super::records::Record> {
        self.original.as_deref()
    }
}
impl<F: Function> Row<F> {
    pub(crate) fn new(
        position: NonZeroU64,
        id: Option<RecordId>,
        source: Source,
        outcome: Outcome<F::Answer, F::Detail>,
    ) -> Self {
        Self {
            item: (),
            function: FunctionTag(std::marker::PhantomData),
            position,
            id,
            source,
            original: None,
            outcome,
        }
    }
    pub(crate) fn with_original(mut self, original: Option<super::records::Record>) -> Self {
        self.original = original.map(Box::new);
        self
    }
    pub(crate) fn into_record_result(mut self) -> Result<Row<F, super::records::Record>, Error> {
        let record = self.original.take().ok_or_else(mismatch)?;
        Ok(self.with_item(*record))
    }
    pub(crate) fn original_record(&self) -> Option<&super::records::Record> {
        self.original.as_deref()
    }
    pub(crate) fn map_answer<G: Function<Detail = F::Detail>>(
        self,
        map: impl FnOnce(F::Answer) -> Result<G::Answer, Error>,
    ) -> Result<Row<G>, Error> {
        let outcome = self.outcome.map_answer(map)?;
        Ok(Row {
            item: self.item,
            function: FunctionTag(std::marker::PhantomData),
            position: self.position,
            id: self.id,
            source: self.source,
            original: self.original,
            outcome,
        })
    }
    pub(crate) fn with_item<T>(self, item: T) -> Row<F, T> {
        Row {
            item,
            function: self.function,
            position: self.position,
            id: self.id,
            source: self.source,
            original: None,
            outcome: self.outcome,
        }
    }
}
impl<F: Function, T> Row<F, T> {
    /// The caller's original item retained by a many-record result.
    #[must_use]
    pub const fn item(&self) -> &T {
        &self.item
    }
    /// Take back the caller's original item.
    #[must_use]
    pub fn into_item(self) -> T {
        self.item
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
    /// Whether this result has an answer.
    #[must_use]
    pub fn is_answered(&self) -> bool {
        self.status() == Status::Answered
    }
    /// Whether the model could not choose an answer.
    #[must_use]
    pub fn is_unsure(&self) -> bool {
        self.status() == Status::Unsure
    }
    /// Whether this record failed.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        self.status() == Status::Failed
    }
    /// The function's detail, including detail on an unsure result.
    #[must_use]
    pub fn detail(&self) -> Option<&F::Detail> {
        self.outcome.detail()
    }
    /// The function's defined aggregate probability, when present.
    #[must_use]
    pub fn probability(&self) -> Option<Probability> {
        self.outcome.probability()
    }
    /// The error belonging to this failed record.
    #[must_use]
    pub fn error(&self) -> Option<&Error> {
        self.outcome.error()
    }
    /// Report a failed record or a failed entry of an answered form.
    /// Not-sure entries are not failures.
    #[must_use]
    pub fn failure(&self) -> Option<&Error> {
        self.outcome.failure::<F>()
    }
    /// Return this result unchanged unless it or one of its form entries failed.
    /// # Errors
    /// Returns the record error, or the first failed entry in question-name order.
    pub fn err_if_failed(self) -> Result<Self, Error> {
        if let Some(error) = self.failure() {
            return Err(error.clone());
        }
        Ok(self)
    }
}
impl<F: Function> Row<F> {
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
impl<F: Function, T, U> PartialEq<Row<F, U>> for Row<F, T> {
    fn eq(&self, other: &Row<F, U>) -> bool {
        self.position == other.position
            && self.id == other.id
            && self.source == other.source
            && self.outcome == other.outcome
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
pub(crate) fn optional_value<'de, T: Deserialize<'de>, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}

fn answer_schema<A: schemars::JsonSchema>(g: &mut schemars::SchemaGenerator) -> schemars::Schema {
    A::json_schema(g)
}
