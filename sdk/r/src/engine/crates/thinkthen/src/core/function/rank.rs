//! Positive bounds for canonical ranked rows.

use crate::core::{Error, ErrorKind};

/// A positive bound on the number of retained ranked records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RankLimit(std::num::NonZeroUsize);

impl RankLimit {
    pub(crate) const MAX: usize = i32::MAX as usize;

    /// Admit the caller's top limit.
    ///
    /// # Errors
    /// Returns `usage.question` outside the portable signed count range.
    pub(crate) fn new(value: usize) -> Result<Self, Error> {
        if value > Self::MAX {
            return Err(Self::invalid());
        }
        std::num::NonZeroUsize::new(value)
            .map(Self)
            .ok_or_else(Self::invalid)
    }

    /// The number of records to retain.
    #[must_use]
    pub(crate) const fn get(self) -> usize {
        self.0.get()
    }

    fn invalid() -> Error {
        Error::new(
            ErrorKind::Question,
            crate::core::diagnostic::requirement(
                "top",
                &format!("a whole number from 1 to {}", Self::MAX),
                "an out-of-range count",
            ),
        )
    }
}

impl std::str::FromStr for RankLimit {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Error> {
        text.parse::<usize>()
            .map_err(|_| Self::invalid())
            .and_then(Self::new)
    }
}
