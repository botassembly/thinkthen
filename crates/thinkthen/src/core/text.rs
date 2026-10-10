//! The text values a judgment carries, each one refused when it is blank.

use std::borrow::Cow;
use std::fmt;

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::core::json::Json;
use crate::core::render::{RenderError, json_line};

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
    /// A model name travels in the request, the recording, and the result,
    /// so it carries no line break, other control character, or white space
    /// but a plain space.
    #[error("a model name holds no control character or white space but a plain space")]
    ModelControl,
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

/// The evidence a judgment reads: text that is not blank, or the JSON object
/// or list a pointer selection made.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Evidence(Shape);

/// Which of the two shapes the evidence holds.
#[derive(Clone, Eq, PartialEq)]
enum Shape {
    /// Text the record held, or a scalar selection's compact spelling.
    Text(String),
    /// The object or list a pointer selection made.
    Structured(Json),
}

/// A value offered as structured evidence is not an object or a list.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("the structured evidence is not an object or a list")]
pub(crate) struct EvidenceShapeError;

impl Evidence {
    /// Empty ancillary text is valid only beside validated images.
    pub(crate) fn image_text(text: String) -> Self {
        Self(Shape::Text(text))
    }

    /// Take text that is not blank as the evidence a judgment reads.
    ///
    /// # Errors
    ///
    /// Returns [`BlankTextError`] when the text is empty or holds only white
    /// space.
    pub(crate) fn new(text: impl Into<String>) -> Result<Self, BlankTextError> {
        let text = text.into();
        Self::validate_text(&text)?;
        Ok(Self(Shape::Text(text)))
    }

    /// Check text before a caller copies borrowed evidence.
    pub(crate) fn validate_text(text: &str) -> Result<(), BlankTextError> {
        if text.trim().is_empty() {
            return Err(BlankTextError::Evidence);
        }
        Ok(())
    }

    /// Take the object or list a pointer selection made as the evidence.
    ///
    /// # Errors
    ///
    /// Returns [`EvidenceShapeError`] when the value is a string, a number,
    /// `true`, `false`, or `null`.
    pub(crate) fn structured(value: Json) -> Result<Self, EvidenceShapeError> {
        match value {
            Json::Array(_) | Json::Object(_) => Ok(Self(Shape::Structured(value))),
            _ => Err(EvidenceShapeError),
        }
    }

    /// The text form: the text itself, or the compact spelling of the object
    /// or list. An evidence limit counts its bytes, `find` writes it into a
    /// unit, and a nested `on` reads a selection back out of it.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError`] when the compact spelling cannot be written.
    pub(crate) fn as_text(&self) -> Result<Cow<'_, str>, RenderError> {
        match &self.0 {
            Shape::Text(text) => Ok(Cow::Borrowed(text.as_str())),
            Shape::Structured(value) => Ok(Cow::Owned(json_line(value)?)),
        }
    }

    /// The value a request's `state` field carries: the object or list a
    /// selection made, and the text as a string otherwise.
    #[must_use]
    pub(crate) fn as_json(&self) -> Json {
        match &self.0 {
            Shape::Text(text) => Json::String(text.clone()),
            Shape::Structured(value) => value.clone(),
        }
    }
}

impl fmt::Debug for Evidence {
    /// Evidence is what a judgment reads, so debug text withholds it.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Evidence(..)")
    }
}

/// The model that answered, as text that is not blank.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(into = "String")]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(inline))]
pub(crate) struct ModelName(String);

impl ModelName {
    /// Take the model a request names, with the white space around it
    /// dropped, as a base address drops its own.
    ///
    /// # Errors
    ///
    /// Returns [`BlankTextError`] when the text is empty, holds only white
    /// space, or holds a control character or white space but a plain space.
    pub(crate) fn new(text: impl Into<String>) -> Result<Self, BlankTextError> {
        let text = text.into();
        let name = text.trim();
        if name.is_empty() {
            return Err(BlankTextError::ModelName);
        }
        if name
            .chars()
            .any(|c| c.is_control() || (c.is_whitespace() && c != ' '))
        {
            return Err(BlankTextError::ModelControl);
        }
        Ok(Self(name.to_owned()))
    }

    /// Take the model a reply or a saved result names, as it wrote it.
    ///
    /// A backend's name is reported, not chosen, so it keeps its bytes and a
    /// diagnostic withholds it when it is not safe to print.
    ///
    /// # Errors
    ///
    /// Returns [`BlankTextError`] when the text is empty or holds only white
    /// space.
    pub(crate) fn reported(text: impl Into<String>) -> Result<Self, BlankTextError> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(BlankTextError::ModelName);
        }
        Ok(Self(text))
    }

    /// Read the model that answered back as text.
    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<ModelName> for String {
    fn from(value: ModelName) -> Self {
        value.0
    }
}

/// The URL a request is posted to. Debug never prints the raw address.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(into = "String")]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(inline))]
pub(crate) struct Url(String);

impl Url {
    /// Take a nonblank posting URL.
    pub(crate) fn new(text: impl Into<String>) -> Result<Self, BlankTextError> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(BlankTextError::Url);
        }
        Ok(Self(text))
    }

    /// Read the original posting URL for wire and result serialization.
    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Url> for String {
    fn from(value: Url) -> Self {
        value.0
    }
}

impl fmt::Debug for Url {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Url(<withheld>)")
    }
}

/// The question a judgment asks, as text that is not blank, an object, or a
/// list.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(inline, with = "Json"))]
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
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(inline, with = "Json"))]
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

/// How many bytes a `Debug` line left out, which it prints in their place.
pub(crate) struct Withheld(pub(crate) usize);

impl fmt::Debug for Withheld {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "<{} bytes withheld>", self.0)
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
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(inline, with = "Json"))]
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
    use super::{
        BlankTextError, Evidence, EvidenceShapeError, Meaning, ModelName, QuestionText, Url,
    };
    use crate::core::json::Json;

    #[test]
    fn raw_url_debug_withholds_the_address() {
        let url = Url::new("http://localhost/url-marker-0210").expect("URL");
        assert_eq!(url.as_str(), "http://localhost/url-marker-0210");
        let shown = format!("{url:?}");
        assert!(shown.contains("<withheld>"));
        assert!(!shown.contains("url-marker-0210"));
    }

    #[test]
    fn new_keeps_the_text_it_was_given() {
        let question = QuestionText::new("asks for a refund").expect("not blank");
        assert_eq!(question.as_json().as_str(), Some("asks for a refund"));
        let evidence = Evidence::new(" leading space is kept ").expect("not blank");
        assert_eq!(
            evidence.as_text().expect("text evidence").as_ref(),
            " leading space is kept "
        );
        let model = ModelName::new("jev-1.13.0").expect("not blank");
        assert_eq!(model.as_str(), "jev-1.13.0");
    }

    #[test]
    fn a_model_name_drops_surrounding_space_and_refuses_a_control_character() {
        for (text, kept) in [
            (" jev-1.13.0 ", Ok("jev-1.13.0")),
            ("jev-1.13.0\n", Ok("jev-1.13.0")),
            ("\tlocal model\r\n", Ok("local model")),
            ("jev\n1.13.0", Err(BlankTextError::ModelControl)),
            ("jev\u{0}", Err(BlankTextError::ModelControl)),
            ("jev\u{7f}", Err(BlankTextError::ModelControl)),
            ("jev\u{85}x", Err(BlankTextError::ModelControl)),
            ("jev\u{2028}x", Err(BlankTextError::ModelControl)),
        ] {
            assert_eq!(
                ModelName::new(text).as_ref().map(ModelName::as_str),
                kept.as_ref().map(|name| *name),
                "{text:?}"
            );
        }
    }

    #[test]
    fn structured_evidence_takes_only_an_object_or_a_list() {
        for text in ["{}", "[]", r#"{"a":[1]}"#, r#"[{"a":1}]"#] {
            let value = Json::parse(text).expect("JSON");
            assert!(Evidence::structured(value).is_ok(), "{text}");
        }
        for text in [r#""str""#, "3", "false", "null"] {
            let value = Json::parse(text).expect("JSON");
            assert_eq!(
                Evidence::structured(value),
                Err(EvidenceShapeError),
                "{text}"
            );
        }
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
