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
    Tls,
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
#[derive(Clone, Debug)]
#[allow(
    dead_code,
    reason = "deterministic conformance injections exercise kinds absent from the current command"
)]
pub(crate) enum Error {
    Transport(TransportKind),
    Status(u16),
    /// A process send total refused the first live attempt.
    SendBudgetFirst,
    /// A process send total refused another ordinary or split request.
    SendBudgetAdditional,
    /// A process send total refused retrying this status.
    SendBudgetRetry(u16),
    /// The estimated input admission refused one final body.
    EstimatedInput(crate::core::EstimatedInputDenial),
    ImageEstimatedInput(crate::core::EstimatedInputDenial),
    /// Status 400 whose body named `max_tokens_exceeded`. It keeps no body byte.
    TokenLimit,
    /// The reply passed its request's limit of this many bytes and was not kept.
    ReplyTooLarge(u64),
    Reply(DecodeError),
    /// Strict replay found no stored answer under this question key.
    QuestionMiss(String),
    /// A replay folder holds both the fixture and the live store.
    StoreAmbiguous,
    /// A read-only replay met a write that did not finish.
    StoreHotJournal,
    Entry(String, String),
    RecordingStorage,
    RecordingPathIsFile,
    DefaultCachePrivate,
    /// The usage folder exists and cannot be read, so a send would go
    /// uncounted (ticket 0360). It carries the whole sentence, which names
    /// only a generated file name.
    UsageUnreadable(String),
    CacheEntry,
    Defect(&'static str),
    Usage(&'static str),
    ProfileLimit(ProfileLimit),
    Cancelled,
    Deadline(Budget),
    /// The key variable holds nothing, so no key can be sent.
    NoKey(String),
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
    /// A recognize text passed its byte limit, so no request was sent.
    TextTooLong {
        bytes: usize,
        limit: usize,
    },
    /// A nonempty recognition relation plan admitted too many distinct names.
    RecognizeRelationNames {
        count: usize,
        limit: usize,
    },
    /// A nonempty recognition relation plan would ask too many pair questions.
    /// `None` means the checked total overflowed.
    RecognizeRelationQuestions {
        names: usize,
        count: Option<usize>,
        limit: usize,
    },
}

/// The statuses a backend is asked again after.
const RETRIED: [u16; 11] = [429, 500, 502, 503, 504, 520, 521, 522, 523, 524, 529];

pub(crate) fn retried_status(status: u16) -> bool {
    RETRIED.contains(&status)
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
    /// A refusal that a smaller batch may answer.
    pub(crate) const fn too_large(&self) -> bool {
        matches!(self, Self::TokenLimit | Self::Status(413))
    }

    /// A refusal by the caller's send budget or estimated input cap.
    pub(crate) const fn spent(&self) -> bool {
        matches!(
            self,
            Self::EstimatedInput(_)
                | Self::ImageEstimatedInput(_)
                | Self::SendBudgetFirst
                | Self::SendBudgetAdditional
                | Self::SendBudgetRetry(_)
        )
    }

    pub(crate) const fn kind(&self) -> Kind {
        match self {
            Self::Transport(_)
            | Self::Status(_)
            | Self::SendBudgetRetry(_)
            | Self::TokenLimit
            | Self::ReplyTooLarge(_)
            | Self::Reply(_)
            | Self::ModelsDiffer(_)
            | Self::UsageOverflow
            | Self::RecognizeLogical => Kind::Backend,
            Self::QuestionMiss(_)
            | Self::StoreAmbiguous
            | Self::StoreHotJournal
            | Self::Entry(_, _)
            | Self::RecordingStorage
            | Self::RecordingPathIsFile
            | Self::DefaultCachePrivate
            | Self::UsageUnreadable(_) => Kind::Local,
            Self::CacheEntry => Kind::Local,
            Self::Defect(_) => Kind::Defect,
            Self::Usage(_)
            | Self::EstimatedInput(_)
            | Self::ImageEstimatedInput(_)
            | Self::SendBudgetFirst
            | Self::SendBudgetAdditional
            | Self::ProfileLimit(_)
            | Self::NoKey(_)
            | Self::WidthActive(_)
            | Self::RecognizeKinds
            | Self::TextTooLong { .. }
            | Self::RecognizeRelationNames { .. }
            | Self::RecognizeRelationQuestions { .. } => Kind::Usage,
            Self::Cancelled => Kind::Cancelled,
            Self::Deadline(_) => Kind::Deadline,
        }
    }

    /// Whether the same call may succeed when sent again: only a busy or
    /// failing backend status. A transport failure may already have reached
    /// the backend, so it is never retried.
    pub(crate) fn retryable(&self) -> bool {
        matches!(self, Self::Status(status) if retried_status(*status))
    }
}

/// Numeric relation refusals shared by command and public error envelopes.
pub(crate) fn relation_names_message(count: usize, limit: usize) -> String {
    format!(
        "{count} distinct relation-eligible names exceed the limit of {limit}; reduce names or split the input"
    )
}

pub(crate) fn relation_questions_message(
    names: usize,
    count: Option<usize>,
    limit: usize,
) -> String {
    let asked = count.map_or_else(
        || format!("at least {}", limit + 1),
        |count| count.to_string(),
    );
    format!(
        "{names} distinct relation-eligible names would ask {asked} relation questions, over the limit of {limit}; reduce names or relation rules, or split the input"
    )
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

/// The sentence a reply past its request's limit earns, in the command and the library.
pub(crate) fn reply_too_large(limit: u64) -> String {
    format!(
        "the backend's reply passed this request's limit of {limit} bytes, so the answer was not kept; the request was not sent again"
    )
}

impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Self {
        Self::Reply(error)
    }
}
