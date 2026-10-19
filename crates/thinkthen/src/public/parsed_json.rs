//! Read-only native ordered JSON, independent of execution record admission.
use crate::{Error, QuestionContent, core};
use std::{fmt, sync::Arc};

/// Parsed ordered JSON for read-only consumers such as offline comparison.
///
/// Unlike [`crate::RawRecord`], this has no execution byte ceiling. It cannot
/// become an execution record or request input. A clone shares the parsed value;
/// explicit borrowed content access can expose the caller's original evidence.
#[derive(Clone)]
pub struct ParsedJson(Arc<core::Json>);

impl ParsedJson {
    /// Parse any JSON shape using the native ordered parser, without a byte ceiling.
    /// # Errors
    /// Refuses malformed, duplicate-name, nonfinite or overdeep JSON through the
    /// ordinary safe parser diagnostic. No parser body becomes an error source.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        core::Json::parse(text)
            .map(|value| Self(Arc::new(value)))
            .map_err(Error::refused)
    }

    /// Borrow ordered content. This explicit view can expose original evidence.
    #[must_use]
    pub fn content(&self) -> QuestionContent<'_> {
        QuestionContent(self.0.as_ref())
    }
}

impl fmt::Debug for ParsedJson {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ParsedJson(<withheld>)")
    }
}
