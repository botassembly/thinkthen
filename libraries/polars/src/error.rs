//! The door's one error: the engine's error, or the door's own refusal.

use std::fmt;

use thinkthen::ErrorKind;

/// What stopped a door call.
///
/// `Debug` and `Display` name a column, a dtype, or a question name, and
/// never a text of the column.
#[derive(Debug)]
pub enum Error {
    /// The engine refused or failed the call.
    Engine(thinkthen::Error),
    /// The door refused the call before any request.
    Usage(String),
    /// A fault inside the door. Report it.
    Defect(String),
}

impl Error {
    /// The engine's kind, or `Usage` or `Defect` for the door's own refusals.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        match self {
            Self::Engine(error) => error.kind(),
            Self::Usage(_) => ErrorKind::Usage,
            Self::Defect(_) => ErrorKind::Defect,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engine(error) => error.fmt(formatter),
            Self::Usage(message) | Self::Defect(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Engine(error) => Some(error),
            Self::Usage(_) | Self::Defect(_) => None,
        }
    }
}

impl From<thinkthen::Error> for Error {
    fn from(error: thinkthen::Error) -> Self {
        Self::Engine(error)
    }
}
