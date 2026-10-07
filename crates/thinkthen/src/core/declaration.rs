//! Closed author declarations; deliberately separate from semantic identity.
use super::Json;
use serde::Serialize;
use serde::ser::Serializer;
mod reading;
mod wire;
pub(crate) use reading::AuthoredReading;
use std::fmt;
use thiserror::Error;

/// A validated author name, never a routing or cache identity.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(transparent)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct QuestionName(
    #[cfg_attr(test, schemars(regex(pattern = "^[a-z][a-z0-9_-]{0,63}$")))] String,
);
impl QuestionName {
    pub(crate) fn validated(value: &str) -> Result<Self, DeclarationError> {
        let bytes = value.as_bytes();
        if !(1..=64).contains(&bytes.len())
            || !bytes.first().is_some_and(u8::is_ascii_lowercase)
            || !bytes
                .iter()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'_' || *c == b'-')
        {
            return Err(DeclarationError);
        }
        Ok(Self(value.to_owned()))
    }
    /// The exact case-sensitive author name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An optional author wording version, with no inferred default.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct WordingVersion(#[cfg_attr(test, schemars(range(min = 1, max = 2147483647)))] u32);
impl WordingVersion {
    pub(crate) fn validated(value: u32) -> Result<Self, DeclarationError> {
        if !(1..=2_147_483_647).contains(&value) {
            return Err(DeclarationError);
        }
        Ok(Self(value))
    }
    /// The declared positive version number.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// The four admitted object property shapes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(with = "wire::Property"))]
pub enum InputPropertyType {
    /// A JSON string, including an empty string.
    String,
    /// A finite JSON number, without coercion.
    Number,
    /// A JSON boolean, including false.
    Boolean,
    /// An ordered array containing only strings.
    StringList,
}
impl InputPropertyType {
    fn accepts(self, value: &Json) -> bool {
        match (self, value) {
            (Self::String, Json::String(_))
            | (Self::Number, Json::Number(_))
            | (Self::Boolean, Json::Bool(_)) => true,
            (Self::StringList, Json::Array(items)) => {
                items.iter().all(|v| matches!(v, Json::String(_)))
            }
            _ => false,
        }
    }
}
impl Serialize for InputPropertyType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        wire::Property::from(*self).serialize(serializer)
    }
}

/// One validated literal property name and its shape.
#[derive(Clone, Eq, PartialEq)]
pub struct InputProperty {
    name: String,
    kind: InputPropertyType,
}
impl InputProperty {
    pub(crate) fn validated(name: &str, kind: InputPropertyType) -> Result<Self, DeclarationError> {
        if name.is_empty() || name.chars().any(char::is_control) {
            return Err(DeclarationError);
        }
        Ok(Self {
            name: name.to_owned(),
            kind,
        })
    }
    /// Literal property name, never a pointer.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The property's admitted shape.
    #[must_use]
    pub const fn kind(&self) -> InputPropertyType {
        self.kind
    }
}

/// A validated ordered object declaration. Extra input properties are retained.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "wire::ObjectRoot<'static>")
)]
pub struct ObjectDeclaration {
    properties: Vec<InputProperty>,
    required: Vec<String>,
}
impl ObjectDeclaration {
    pub(crate) fn validated(
        properties: Vec<InputProperty>,
        required: Vec<String>,
    ) -> Result<Self, DeclarationError> {
        for (at, property) in properties.iter().enumerate() {
            if properties.iter().take(at).any(|p| p.name == property.name) {
                return Err(DeclarationError);
            }
        }
        for (at, name) in required.iter().enumerate() {
            if required.iter().take(at).any(|held| held == name)
                || !properties.iter().any(|p| &p.name == name)
            {
                return Err(DeclarationError);
            }
        }
        Ok(Self {
            properties,
            required,
        })
    }
    /// Properties in author order.
    #[must_use]
    pub fn properties(&self) -> &[InputProperty] {
        &self.properties
    }
    /// Required literal names in author order.
    #[must_use]
    pub fn required(&self) -> &[String] {
        &self.required
    }
    fn accepts(&self, value: &Json) -> bool {
        matches!(value, Json::Object(_))
            && self
                .required
                .iter()
                .all(|name| value.member(name).is_some())
            && self
                .properties
                .iter()
                .all(|p| value.member(&p.name).is_none_or(|v| p.kind.accepts(v)))
    }
}
impl Serialize for ObjectDeclaration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        wire::ObjectRoot::from(self).serialize(serializer)
    }
}

/// The complete closed subset of admitted root input declarations.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "wire::Root<'static>")
)]
pub enum InputDeclaration {
    /// A typed string.
    String,
    /// A typed object with ordered declared properties.
    Object(ObjectDeclaration),
}
impl InputDeclaration {
    pub(crate) fn accepts(&self, value: &Json) -> bool {
        match self {
            Self::String => matches!(value, Json::String(_)),
            Self::Object(object) => object.accepts(value),
        }
    }
}
impl Serialize for InputDeclaration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        wire::Root::from(self).serialize(serializer)
    }
}

/// A safe refusal for malformed names, versions or declaration features.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("the question declaration uses an unsupported feature")]
pub(crate) struct DeclarationError;

/// Caller metadata, excluded from every semantic serializer and digest.
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(crate) struct QuestionMetadata {
    #[serde(skip)]
    pub(crate) reading: AuthoredReading,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<QuestionName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) wording_version: Option<WordingVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) item_schema: Option<InputDeclaration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) context_schema: Option<InputDeclaration>,
}

macro_rules! withheld {
    ($($name:ident),+) => { $(impl fmt::Debug for $name {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct(stringify!($name)).finish_non_exhaustive()
        }
    })+ };
}
withheld!(
    QuestionName,
    InputProperty,
    ObjectDeclaration,
    InputDeclaration
);

/// Presentation only: the semantic question serializer remains unchanged.
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(crate) struct ReadableQuestion<'a, Q: Serialize> {
    #[serde(flatten)]
    pub(crate) question: &'a Q,
    #[serde(flatten)]
    pub(crate) metadata: &'a QuestionMetadata,
}
mod parsing;
