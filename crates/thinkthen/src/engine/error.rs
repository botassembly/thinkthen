//! Structured failures produced below the command boundary.

use std::fmt;
use std::time::Duration;

use crate::core::ProfileLimit;
use crate::core::adapters::built_in::DecodeError;

/// A safe, stable description of why a backend connection failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TransportKind {
    Timeout,
    NameLookup,
    Refused,
    PrematureClose,
    Other,
}

/// The stable class a host-facing error will use in a later ticket.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(
    dead_code,
    reason = "the private bridge fixes all six accepted kinds before public engine errors land"
)]
pub(crate) enum Kind {
    Usage,
    Local,
    Backend,
    Cancelled,
    Deadline,
    Defect,
}

/// One exact engine failure. The command maps it to its existing diagnostics.
#[derive(Debug)]
#[allow(
    dead_code,
    reason = "deterministic conformance injections exercise kinds absent from the current command"
)]
pub(crate) enum Error {
    Transport(TransportKind),
    Status(u16),
    Reply(DecodeError),
    ReplayMiss(String),
    Entry(String, String),
    RecordingConflict(String),
    RecordingStorage,
    RecordingPathIsFile,
    RecordingBackendMismatch,
    RecordingFolderLegacy,
    DefaultCachePrivate,
    CacheEntry,
    Defect(&'static str),
    Usage(&'static str),
    ProfileLimit(ProfileLimit),
    Cancelled,
    Deadline(Budget),
}

/// The whole-call budget a spent deadline was made from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Budget(pub(crate) Duration);

impl fmt::Display for Budget {
    /// Name the budget in the largest whole unit, in integers alone.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let budget = self.0;
        if budget.subsec_nanos() == 0 {
            write!(formatter, "the deadline of {} s", budget.as_secs())?;
        } else if budget.subsec_nanos().is_multiple_of(1_000_000) {
            write!(formatter, "the deadline of {} ms", budget.as_millis())?;
        } else {
            write!(formatter, "the deadline of {} ns", budget.as_nanos())?;
        }
        formatter.write_str(" passed before the call answered")
    }
}

#[allow(
    dead_code,
    reason = "the command runner reads kinds while normal command paths preserve exact causes"
)]
impl Error {
    pub(crate) const fn kind(&self) -> Kind {
        match self {
            Self::Transport(_) | Self::Status(_) | Self::Reply(_) => Kind::Backend,
            Self::ReplayMiss(_)
            | Self::Entry(_, _)
            | Self::RecordingConflict(_)
            | Self::RecordingStorage
            | Self::RecordingPathIsFile
            | Self::RecordingBackendMismatch
            | Self::RecordingFolderLegacy
            | Self::DefaultCachePrivate => Kind::Local,
            Self::CacheEntry => Kind::Local,
            Self::Defect(_) => Kind::Defect,
            Self::Usage(_) | Self::ProfileLimit(_) => Kind::Usage,
            Self::Cancelled => Kind::Cancelled,
            Self::Deadline(_) => Kind::Deadline,
        }
    }
}

#[allow(
    dead_code,
    reason = "the offline command runner compares the accepted kind spellings"
)]
impl Kind {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Usage => "usage",
            Self::Local => "local",
            Self::Backend => "backend",
            Self::Cancelled => "cancelled",
            Self::Deadline => "deadline",
            Self::Defect => "defect",
        }
    }
}

impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Self {
        Self::Reply(error)
    }
}
