//! The text values a judgment carries, each one refused when it is blank.

use serde::Serialize;
use thiserror::Error;

/// Which text value arrived blank.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BlankTextError {
    /// A condition states a fact about the evidence, so it carries text.
    #[error("a condition is text, not white space")]
    Condition,
    /// Evidence is what a judgment reads, so it carries text.
    #[error("evidence is text, not white space")]
    Evidence,
    /// A backend name picks a profile, so it carries text.
    #[error("a backend name is text, not white space")]
    BackendName,
    /// A model name reports what answered, so it carries text.
    #[error("a model name is text, not white space")]
    ModelName,
    /// A URL names where the request is posted, so it carries text.
    #[error("a URL is text, not white space")]
    Url,
    /// A key variable names an environment variable, so it carries text.
    #[error("a key variable name is text, not white space")]
    KeyVar,
}

/// Declare one text value that is not blank, its accessor, and its conversions.
macro_rules! text_value {
    ($name:ident, $variant:ident, $what:literal) => {
        #[doc = concat!("The ", $what, ", as text that is not blank.")]
        #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
        #[serde(into = "String")]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Take text that is not blank as the ", $what, ".")]
            ///
            /// # Errors
            ///
            /// Returns [`BlankTextError`] when the text is empty or holds only
            /// white space.
            pub fn new(text: impl Into<String>) -> Result<Self, BlankTextError> {
                let text = text.into();
                if text.trim().is_empty() {
                    return Err(BlankTextError::$variant);
                }
                Ok(Self(text))
            }

            #[doc = concat!("Read the ", $what, " back as text.")]
            #[must_use]
            pub fn as_str(&self) -> &str {
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

text_value!(Condition, Condition, "condition a question asks about");
text_value!(Evidence, Evidence, "evidence a judgment reads");
text_value!(BackendName, BackendName, "backend that answered");
text_value!(ModelName, ModelName, "model that answered");
text_value!(Url, Url, "URL a request is posted to");
text_value!(KeyVar, KeyVar, "environment variable that holds the key");

#[cfg(test)]
mod tests {
    use super::{BackendName, BlankTextError, Condition, Evidence, KeyVar, ModelName, Url};

    #[test]
    fn new_keeps_the_text_it_was_given() {
        let condition = Condition::new("asks for a refund").expect("not blank");
        assert_eq!(condition.as_str(), "asks for a refund");
        let evidence = Evidence::new(" leading space is kept ").expect("not blank");
        assert_eq!(evidence.as_str(), " leading space is kept ");
        let backend = BackendName::new("jev").expect("not blank");
        assert_eq!(backend.as_str(), "jev");
        let model = ModelName::new("jev-1.13.0").expect("not blank");
        assert_eq!(model.as_str(), "jev-1.13.0");
    }

    #[test]
    fn new_refuses_text_that_is_empty_or_only_white_space() {
        type Make = fn(&str) -> Result<(), BlankTextError>;
        let makers: [(Make, BlankTextError); 6] = [
            (|text| Url::new(text).map(|_| ()), BlankTextError::Url),
            (|text| KeyVar::new(text).map(|_| ()), BlankTextError::KeyVar),
            (
                |text| Condition::new(text).map(|_| ()),
                BlankTextError::Condition,
            ),
            (
                |text| Evidence::new(text).map(|_| ()),
                BlankTextError::Evidence,
            ),
            (
                |text| BackendName::new(text).map(|_| ()),
                BlankTextError::BackendName,
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
