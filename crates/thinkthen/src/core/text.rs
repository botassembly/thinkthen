//! The text values a judgment carries, each one refused when it is blank.

use serde::Serialize;
use thiserror::Error;

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

text_value!(QuestionText, QuestionText, "question a judgment asks");
text_value!(Evidence, Evidence, "evidence a judgment reads");
text_value!(ModelName, ModelName, "model that answered");
text_value!(Url, Url, "URL a request is posted to");
text_value!(Meaning, Meaning, "text that says what yes or what no means");

#[cfg(test)]
mod tests {
    use super::{BlankTextError, Evidence, Meaning, ModelName, QuestionText, Url};

    #[test]
    fn new_keeps_the_text_it_was_given() {
        let question = QuestionText::new("asks for a refund").expect("not blank");
        assert_eq!(question.as_str(), "asks for a refund");
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
