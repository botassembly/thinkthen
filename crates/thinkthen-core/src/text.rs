//! The text values a judgment carries, each one refused when it is empty.

use serde::Serialize;
use thiserror::Error;

/// Which text value arrived empty.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum EmptyTextError {
    /// A condition states a fact about the evidence, so it carries text.
    #[error("a condition is not empty")]
    Condition,
    /// A backend name picks a profile, so it carries text.
    #[error("a backend name is not empty")]
    BackendName,
    /// A model name reports what answered, so it carries text.
    #[error("a model name is not empty")]
    ModelName,
}

/// Declare one non-empty text value, its accessor, and its serde conversions.
macro_rules! non_empty_text {
    ($name:ident, $variant:ident, $what:literal) => {
        #[doc = concat!("The ", $what, ", as text that is not empty.")]
        #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
        #[serde(into = "String")]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Take text that is not empty as the ", $what, ".")]
            ///
            /// # Errors
            ///
            /// Returns [`EmptyTextError`] when the text holds no characters.
            pub fn new(text: impl Into<String>) -> Result<Self, EmptyTextError> {
                let text = text.into();
                if text.is_empty() {
                    return Err(EmptyTextError::$variant);
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

non_empty_text!(Condition, Condition, "condition a question asks about");
non_empty_text!(BackendName, BackendName, "backend that answered");
non_empty_text!(ModelName, ModelName, "model that answered");

#[cfg(test)]
mod tests {
    use super::{BackendName, Condition, EmptyTextError, ModelName};

    #[test]
    fn new_keeps_the_text_it_was_given() {
        let condition = Condition::new("asks for a refund").expect("not empty");
        assert_eq!(condition.as_str(), "asks for a refund");
        let backend = BackendName::new("jev").expect("not empty");
        assert_eq!(backend.as_str(), "jev");
        let model = ModelName::new("jev-1.13.0").expect("not empty");
        assert_eq!(model.as_str(), "jev-1.13.0");
    }

    #[test]
    fn new_refuses_empty_text() {
        assert_eq!(Condition::new(""), Err(EmptyTextError::Condition));
        assert_eq!(BackendName::new(""), Err(EmptyTextError::BackendName));
        assert_eq!(ModelName::new(""), Err(EmptyTextError::ModelName));
    }
}
