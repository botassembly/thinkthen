//! Borrowed ordered-content inspection for command-owned offline reports.
use super::QuestionContent;
use crate::{Error, core};

impl<'a> QuestionContent<'a> {
    /// One object member, preserving absence separately from explicit null.
    #[must_use]
    pub fn member(self, name: &str) -> Option<Self> {
        self.0.member(name).map(Self)
    }

    /// One zero-based array element.
    #[must_use]
    pub fn element(self, at: usize) -> Option<Self> {
        self.0.element(at).map(Self)
    }

    /// Object members in their original authored order.
    #[must_use]
    pub fn fields(self) -> Option<impl ExactSizeIterator<Item = (&'a str, Self)> + 'a> {
        let core::Json::Object(fields) = self.0 else {
            return None;
        };
        Some(
            fields
                .iter()
                .map(|(name, value)| (name.as_str(), Self(value))),
        )
    }

    /// Array elements in their original authored order.
    #[must_use]
    pub fn elements(self) -> Option<impl ExactSizeIterator<Item = Self> + 'a> {
        let core::Json::Array(values) = self.0 else {
            return None;
        };
        Some(values.iter().map(Self))
    }

    /// An exact admitted JSON number, retaining integer and decimal distinctions.
    #[must_use]
    pub fn number(self) -> Option<&'a serde_json::Number> {
        match self.0 {
            core::Json::Number(number) => Some(number),
            _ => None,
        }
    }

    /// An authored boolean, preserving false as a value.
    #[must_use]
    pub const fn boolean(self) -> Option<bool> {
        match self.0 {
            core::Json::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Resolve an RFC 6901 selector through the existing native pointer parser.
    /// Absent members and paths traversing noncontainers return None.
    /// # Errors
    /// Refuses a selector outside the existing native pointer grammar.
    pub fn pointer(self, selector: &str) -> Result<Option<Self>, Error> {
        core::Pointer::new(selector)
            .map(|pointer| pointer.resolve(self.0).map(Self))
            .map_err(Error::refused)
    }
}
