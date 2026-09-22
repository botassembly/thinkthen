//! The text values a judgment carries, each one refused when it is blank.

use std::fmt;

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::core::json::Json;

/// Which text value arrived blank.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub(crate) enum BlankTextError {
    /// A question states a fact about the evidence, so it carries text.
    #[error("a question is text, not white space")]
    QuestionText,
    /// Evidence is what a judgment reads, so it carries text.
    #[error("the evidence is empty or blank")]
    Evidence,
    /// A model name reports what answered, so it carries text.
    #[error("a model name is text, not white space")]
    ModelName,
    /// A URL names where the request is posted, so it carries text.
    #[error("a URL is text, not white space")]
    Url,
    /// What yes or no means is read by the model, so it carries text.
    #[error("what yes or no means is text, not white space")]
    Meaning,
    /// A description is read by the model, so a string one carries text.
    #[error("a description is text, not white space")]
    Description,
}

/// Declare one text value that is not blank, its accessor, and its conversions.
macro_rules! text_value {
    ($name:ident, $variant:ident, $what:literal) => {
        #[doc = concat!("The ", $what, ", as text that is not blank.")]
        #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
        #[serde(into = "String")]
        pub(crate) struct $name(String);

        impl $name {
            #[doc = concat!("Take text that is not blank as the ", $what, ".")]
            ///
            /// # Errors
            ///
            /// Returns [`BlankTextError`] when the text is empty or holds only
            /// white space.
            pub(crate) fn new(text: impl Into<String>) -> Result<Self, BlankTextError> {
                let text = text.into();
                if text.trim().is_empty() {
                    return Err(BlankTextError::$variant);
                }
                Ok(Self(text))
            }

            #[doc = concat!("Read the ", $what, " back as text.")]
            #[must_use]
            pub(crate) fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

text_value!(Evidence, Evidence, "evidence a judgment reads");
text_value!(ModelName, ModelName, "model that answered");
text_value!(Url, Url, "URL a request is posted to");

/// The question a judgment asks, as text that is not blank, an object, or a
/// list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct QuestionText(Json);

impl QuestionText {
    /// Take text that is not blank as the question a judgment asks.
    ///
    /// # Errors
    ///
    /// Returns [`BlankTextError`] when the text is empty or holds only
    /// white space.
    pub(crate) fn new(text: impl Into<String>) -> Result<Self, BlankTextError> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(BlankTextError::QuestionText);
        }
        Ok(Self(Json::String(text)))
    }

    /// Take an object or a list as the question a judgment asks.
    pub(crate) fn structured(value: &Json) -> Option<Self> {
        match value {
            Json::Array(_) | Json::Object(_) => Some(Self(value.clone())),
            _ => None,
        }
    }

    /// Read the question back as the JSON it holds.
    #[must_use]
    pub(crate) const fn as_json(&self) -> &Json {
        &self.0
    }
}

impl Serialize for QuestionText {
    /// Write the question as the JSON it holds, so a string stays a string.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/// The text that says what yes or what no means, as text that is not blank,
/// an object, a list, or null.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Meaning(Json);

impl Meaning {
    /// Take text that is not blank as the text that says what a meaning is.
    ///
    /// # Errors
    ///
    /// Returns [`BlankTextError`] when the text is empty or holds only
    /// white space.
    pub(crate) fn new(text: impl Into<String>) -> Result<Self, BlankTextError> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(BlankTextError::Meaning);
        }
        Ok(Self(Json::String(text)))
    }

    /// Take an object, a list, or null as what a meaning is.
    pub(crate) fn structured(value: &Json) -> Option<Self> {
        match value {
            Json::Array(_) | Json::Null | Json::Object(_) => Some(Self(value.clone())),
            _ => None,
        }
    }

    /// Read the meaning back as the JSON it holds.
    #[must_use]
    pub(crate) const fn as_json(&self) -> &Json {
        &self.0
    }
}

impl fmt::Debug for Meaning {
    /// A criterion may hold evidence, so debug text withholds what it says.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Meaning(..)")
    }
}

impl Serialize for Meaning {
    /// Write the meaning as the JSON it holds, so a string stays a string.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/// The description a label or a level carries, as text, an object, a list, or
/// null.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Description(Json);

impl Description {
    /// Take text as a description; a blank one reads as absent.
    #[must_use]
    pub(crate) fn text(value: impl Into<String>) -> Self {
        Self(Json::String(value.into()))
    }

    /// Take a JSON value as a description when it is a string, an object, a
    /// list, or null.
    pub(crate) fn of_json(value: &Json) -> Option<Self> {
        match value {
            Json::Array(_) | Json::Null | Json::Object(_) | Json::String(_) => {
                Some(Self(value.clone()))
            }
            Json::Bool(_) | Json::Number(_) => None,
        }
    }

    /// True when the description is a blank string, which reads as absent.
    #[must_use]
    pub(crate) fn blank(&self) -> bool {
        matches!(&self.0, Json::String(text) if text.trim().is_empty())
    }

    /// Read the description back as the JSON it holds.
    #[must_use]
    pub(crate) const fn as_json(&self) -> &Json {
        &self.0
    }
}

impl fmt::Debug for Description {
    /// A description may hold evidence, so debug text withholds what it says.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Description(..)")
    }
}

impl Serialize for Description {
    /// Write the description as the JSON it holds, so a string stays a string.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::{BlankTextError, Evidence, Meaning, ModelName, QuestionText, Url};

    #[test]
    fn new_keeps_the_text_it_was_given() {
        let question = QuestionText::new("asks for a refund").expect("not blank");
        assert_eq!(question.as_json().as_str(), Some("asks for a refund"));
        let evidence = Evidence::new(" leading space is kept ").expect("not blank");
        assert_eq!(evidence.as_str(), " leading space is kept ");
        let model = ModelName::new("jev-1.13.0").expect("not blank");
        assert_eq!(model.as_str(), "jev-1.13.0");
    }

    #[test]
    fn new_refuses_text_that_is_empty_or_only_white_space() {
        type Make = fn(&str) -> Result<(), BlankTextError>;
        let makers: [(Make, BlankTextError); 5] = [
            (
                |text| Meaning::new(text).map(|_| ()),
                BlankTextError::Meaning,
            ),
            (|text| Url::new(text).map(|_| ()), BlankTextError::Url),
            (
                |text| QuestionText::new(text).map(|_| ()),
                BlankTextError::QuestionText,
            ),
            (
                |text| Evidence::new(text).map(|_| ()),
                BlankTextError::Evidence,
            ),
            (
                |text| ModelName::new(text).map(|_| ()),
                BlankTextError::ModelName,
            ),
        ];
        for (make, expected) in makers {
            for case in ["", " ", "\t", "\n", "  \t\r\n ", "\u{a0}"] {
                assert_eq!(make(case), Err(expected), "{expected} refuses {case:?}");
            }
        }
    }
}
