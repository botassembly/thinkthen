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
    /// The key variable holds nothing, so no key can be sent.
    NoKey(&'static str),
    /// A later explicit width differs from the one this process selected.
    WidthActive(crate::engine::WidthActive),
    /// Replies for one logical result named different model versions.
    ModelsDiffer(Option<(String, String)>),
    /// Reply token counts cannot be represented as one total.
    UsageOverflow,
    /// Recognition asked for more kinds than one kind question can carry.
    RecognizeKinds,
    /// The backend failed one question recognition requires.
    RecognizeLogical,
    /// No logical relation question has a usable answer.
    RelateLogical,
}

/// The statuses a backend is asked again after.
const RETRIED: [u16; 6] = [429, 500, 502, 503, 504, 529];

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
            Self::Transport(_)
            | Self::Status(_)
            | Self::Reply(_)
            | Self::ModelsDiffer(_)
            | Self::UsageOverflow
            | Self::RecognizeLogical
            | Self::RelateLogical => Kind::Backend,
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
            Self::Usage(_)
            | Self::ProfileLimit(_)
            | Self::NoKey(_)
            | Self::WidthActive(_)
            | Self::RecognizeKinds => Kind::Usage,
            Self::Cancelled => Kind::Cancelled,
            Self::Deadline(_) => Kind::Deadline,
        }
    }

    /// Whether the same call may succeed when sent again: only a busy or
    /// failing backend status. A transport failure may already have reached
    /// the backend, so it is never retried.
    pub(crate) fn retryable(&self) -> bool {
        matches!(self, Self::Status(status) if RETRIED.contains(status))
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
