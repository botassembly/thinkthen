//! Label types connect Rust enums to the existing ordered string contract.
use super::{AnswerOption, Error, question_error};

/// A value represented by one exact label in a question's domain.
pub trait Choice:
    Clone
    + std::fmt::Debug
    + PartialEq
    + Send
    + Sync
    + serde::Serialize
    + serde::de::DeserializeOwned
    + schemars::JsonSchema
    + 'static
{
    /// The copied enum member or borrowed string returned by an answer read.
    type Read<'a>
    where
        Self: 'a;
    /// Read the member or text.
    fn read(&self) -> Self::Read<'_>;
    /// The exact text read by the model.
    fn label(&self) -> &str;
    /// Decode one returned label.
    /// # Errors
    /// Refuses labels outside this type's declared domain.
    fn from_label(label: String) -> Result<Self, Error>;
    /// The declared domain, when this type has a finite set of members.
    fn declared() -> Option<&'static [Self]> {
        None
    }
}
/// An enum whose declaration supplies the complete ordered domain.
pub trait Choices: Choice + Copy {
    /// Every member, in declaration order.
    const ALL: &'static [Self];
}
impl Choice for String {
    type Read<'a> = &'a str;
    fn read(&self) -> &str {
        self
    }
    fn label(&self) -> &str {
        self
    }
    fn from_label(label: String) -> Result<Self, Error> {
        Ok(label)
    }
}
pub(super) fn validate<C: Choice>(options: &[AnswerOption]) -> Result<(), Error> {
    if let Some(domain) = C::declared()
        && !options
            .iter()
            .map(AnswerOption::label)
            .eq(domain.iter().map(Choice::label))
    {
        return Err(question_error(
            "the saved domain does not match the enum's declared labels and order",
        ));
    }
    Ok(())
}
pub(super) fn options<C: Choices>() -> Result<Vec<AnswerOption>, Error> {
    C::ALL
        .iter()
        .map(|member| AnswerOption::new(member.label()))
        .collect()
}
pub(super) fn descriptions<C: Choices>(
    options: &mut [AnswerOption],
    values: impl IntoIterator<Item = (C, impl Into<String>)>,
) {
    for (member, text) in values {
        if let Some(option) = options
            .iter_mut()
            .find(|option| option.label() == member.label())
        {
            *option = option.clone().description(text);
        }
    }
}

/// Declare ordinary enum choices whose text is shared by questions and answers.
#[macro_export]
macro_rules! choices {
    ($(#[$attr:meta])* $vis:vis enum $name:ident { $($(#[$member_attr:meta])* $member:ident = $label:literal),+ $(,)? }) => {
        $(#[$attr])* #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $name { $($(#[$member_attr])* #[doc=$label] $member),+ }
        impl $crate::questions::Choice for $name {
            type Read<'a> = Self;
            fn read(&self) -> Self { *self }
            fn label(&self) -> &str { match self { $(Self::$member => $label),+ } }
            fn from_label(label: ::std::string::String) -> ::std::result::Result<Self, $crate::Error> {
                match label.as_str() { $($label => ::std::result::Result::Ok(Self::$member),)+ _ => ::std::result::Result::Err($crate::questions::__choice::unknown_label()) }
            }
            fn declared() -> ::std::option::Option<&'static [Self]> { ::std::option::Option::Some(<Self as $crate::questions::Choices>::ALL) }
        }
        impl $crate::questions::Choices for $name { const ALL: &'static [Self] = &[$(Self::$member),+]; }
        impl ::std::fmt::Display for $name {
            fn fmt(&self, out: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                out.write_str(<Self as $crate::questions::Choice>::label(self))
            }
        }
        impl $crate::questions::__choice::serde::Serialize for $name {
            fn serialize<S: $crate::questions::__choice::serde::Serializer>(&self, serializer:S) -> ::std::result::Result<S::Ok,S::Error> {
                serializer.serialize_str(<Self as $crate::questions::Choice>::label(self))
            }
        }
        impl<'de> $crate::questions::__choice::serde::Deserialize<'de> for $name {
            fn deserialize<D: $crate::questions::__choice::serde::Deserializer<'de>>(deserializer:D) -> ::std::result::Result<Self,D::Error> {
                let label = <::std::string::String as $crate::questions::__choice::serde::Deserialize>::deserialize(deserializer)?;
                <Self as $crate::questions::Choice>::from_label(label).map_err($crate::questions::__choice::serde::de::Error::custom)
            }
        }
        impl $crate::questions::__choice::schemars::JsonSchema for $name {
            fn schema_name() -> ::std::borrow::Cow<'static,str> { stringify!($name).into() }
            fn json_schema(_: &mut $crate::questions::__choice::schemars::SchemaGenerator) -> $crate::questions::__choice::schemars::Schema {
                $crate::questions::__choice::enum_schema(&[$($label),+])
            }
        }
    };
}
