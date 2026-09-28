//! The one public error: six kinds, a safe message, and the retry signal.

use crate::engine::error::{Error as EngineError, Kind, TransportKind, reply_too_large};
use crate::public::SendBudgetDenial;
use crate::public::results::Facts;

/// What stopped a call, as one of six stable kinds.
///
/// Match the variant or read [`Error::kind`]. Every message is safe to log:
/// none carries a key, evidence, or a label a record supplied.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The call or a setting cannot act as given.
    #[error("{}", .0.message)]
    Usage(ErrorDetail),
    /// The backend failed or refused the call.
    #[error("{}", .0.message)]
    Backend(ErrorDetail),
    /// A local file, folder, or cache failed.
    #[error("{}", .0.message)]
    Local(ErrorDetail),
    /// The call's cancel token fired, or its interrupt check answered `true`.
    #[error("{}", .0.message)]
    Cancelled(ErrorDetail),
    /// The call's deadline passed.
    #[error("{}", .0.message)]
    Deadline(ErrorDetail),
    /// A fault inside `thinkthen`. Report it.
    #[error("{}", .0.message)]
    Defect(ErrorDetail),
}

/// The six kinds an [`Error`] takes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// See [`Error::Usage`].
    Usage,
    /// See [`Error::Backend`].
    Backend,
    /// See [`Error::Local`].
    Local,
    /// See [`Error::Cancelled`].
    Cancelled,
    /// See [`Error::Deadline`].
    Deadline,
    /// See [`Error::Defect`].
    Defect,
}

impl ErrorKind {
    /// The conformance word: `usage`, `backend`, `local`, `cancelled`,
    /// `deadline`, or `defect`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Usage => "usage",
            Self::Backend => "backend",
            Self::Local => "local",
            Self::Cancelled => "cancelled",
            Self::Deadline => "deadline",
            Self::Defect => "defect",
        }
    }
}

/// The safe message behind one [`Error`] and whether the same call may succeed later.
#[derive(Debug)]
pub struct ErrorDetail {
    message: String,
    retryable: bool,
    send_budget_denial: Option<SendBudgetDenial>,
    facts: Option<Box<Facts>>,
}

impl ErrorDetail {
    /// The message, which names no key and no evidence.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Error {
    /// The kind of this error.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        match self {
            Self::Usage(_) => ErrorKind::Usage,
            Self::Backend(_) => ErrorKind::Backend,
            Self::Local(_) => ErrorKind::Local,
            Self::Cancelled(_) => ErrorKind::Cancelled,
            Self::Deadline(_) => ErrorKind::Deadline,
            Self::Defect(_) => ErrorKind::Defect,
        }
    }

    /// True only for a busy or failing backend status, which the same call
    /// may pass later. A transport failure may already have reached the
    /// backend, so it is never retryable.
    #[must_use]
    pub const fn retryable(&self) -> bool {
        self.detail().retryable
    }

    /// The message and retry signal behind this error.
    #[must_use]
    pub const fn detail(&self) -> &ErrorDetail {
        match self {
            Self::Usage(detail)
            | Self::Backend(detail)
            | Self::Local(detail)
            | Self::Cancelled(detail)
            | Self::Deadline(detail)
            | Self::Defect(detail) => detail,
        }
    }

    /// The typed reason a process send budget refused this live attempt.
    #[must_use]
    pub const fn send_budget_denial(&self) -> Option<SendBudgetDenial> {
        self.detail().send_budget_denial
    }

    /// Final facts for a started call, including one that sent nothing.
    #[must_use]
    pub fn facts(&self) -> Option<&Facts> {
        self.detail().facts.as_deref()
    }

    pub(crate) fn with_facts(mut self, facts: Facts) -> Self {
        match &mut self {
            Self::Usage(detail)
            | Self::Backend(detail)
            | Self::Local(detail)
            | Self::Cancelled(detail)
            | Self::Deadline(detail)
            | Self::Defect(detail) => detail.facts = Some(Box::new(facts)),
        }
        self
    }

    pub(crate) fn of(kind: ErrorKind, message: impl Into<String>) -> Self {
        let detail = ErrorDetail {
            message: message.into(),
            retryable: false,
            send_budget_denial: None,
            facts: None,
        };
        match kind {
            ErrorKind::Usage => Self::Usage(detail),
            ErrorKind::Backend => Self::Backend(detail),
            ErrorKind::Local => Self::Local(detail),
            ErrorKind::Cancelled => Self::Cancelled(detail),
            ErrorKind::Deadline => Self::Deadline(detail),
            ErrorKind::Defect => Self::Defect(detail),
        }
    }

    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Usage, message)
    }

    /// A usage error whose message is the refusal the core gave.
    pub(crate) fn refused(error: impl std::fmt::Display) -> Self {
        Self::usage(error.to_string())
    }

    pub(crate) fn local(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Local, message)
    }

    pub(crate) fn defect(message: &str) -> Self {
        Self::of(ErrorKind::Defect, format!("defect: {message}"))
    }

    pub(crate) fn cancelled() -> Self {
        Self::of(ErrorKind::Cancelled, "the call was cancelled")
    }
}

const fn kind_of(kind: Kind) -> ErrorKind {
    match kind {
        Kind::Usage => ErrorKind::Usage,
        Kind::Local => ErrorKind::Local,
        Kind::Backend => ErrorKind::Backend,
        Kind::Cancelled => ErrorKind::Cancelled,
        Kind::Deadline => ErrorKind::Deadline,
        Kind::Defect => ErrorKind::Defect,
    }
}

impl From<EngineError> for Error {
    fn from(error: EngineError) -> Self {
        let mut public = Self::of(kind_of(error.kind()), message(&error));
        match &mut public {
            Self::Usage(detail)
            | Self::Backend(detail)
            | Self::Local(detail)
            | Self::Cancelled(detail)
            | Self::Deadline(detail)
            | Self::Defect(detail) => {
                detail.retryable = error.retryable();
                detail.send_budget_denial = match error {
                    EngineError::SendBudgetFirst => Some(SendBudgetDenial::BeforeFirstSend),
                    EngineError::SendBudgetRetry(last_status) => {
                        Some(SendBudgetDenial::BeforeRetry { last_status })
                    }
                    _ => None,
                };
            }
        }
        public
    }
}

/// One library sentence for each engine failure. The command words its own,
/// because its sentences name command-line options.
fn message(error: &EngineError) -> String {
    let fixed = match error {
        EngineError::Transport(kind) => transport(*kind),
        EngineError::Status(302) => {
            return "the backend answered with status 302: the redirect was not followed"
                .to_owned();
        }
        EngineError::Status(status) => return format!("the backend answered with status {status}"),
        EngineError::SendBudgetFirst => "the process send budget was spent before a request",
        EngineError::SendBudgetRetry(status) => {
            return format!("the process send budget was spent before retrying status {status}");
        }
        EngineError::TokenLimit => "the backend answered with status 400",
        EngineError::ReplyTooLarge(limit) => return reply_too_large(*limit),
        EngineError::Reply(decode) => return format!("the reply was refused: {decode}"),
        EngineError::ReplayMiss(_) => "the replay folder holds no reply for this request",
        EngineError::Entry(..) | EngineError::CacheEntry => {
            "the cache or recording folder holds a malformed entry"
        }
        EngineError::RecordingConflict(_) => {
            "the recording folder already holds another reply for this request"
        }
        EngineError::RecordingStorage => "the recording folder could not be written",
        EngineError::RecordingPathIsFile => "the recording folder names a file",
        EngineError::RecordingBackendMismatch(..) => {
            "the recording folder belongs to another backend address; restore its backend settings or choose another folder"
        }
        EngineError::RecordingFolderLegacy => {
            "the recording folder predates backend binding; replay it read-only or choose a new folder"
        }
        EngineError::DefaultCachePrivate => {
            "the default cache folder is not private; set its permissions to 0700 or use no_cache"
        }
        EngineError::Defect(what) => return format!("defect: {what}"),
        EngineError::Usage(what) => what,
        EngineError::ProfileLimit(limit) => {
            return format!(
                "profile {} allows at most {} {}; this request has {}",
                limit.name.as_str(),
                limit.limit,
                limit.kind.words(),
                limit.actual
            );
        }
        EngineError::Cancelled => "the call was cancelled",
        EngineError::Deadline(budget) => return budget.to_string(),
        EngineError::NoKey(variable) => {
            return format!("no key is set; configure an API key for the engine ({variable})");
        }
        EngineError::WidthActive(active) => return active.to_string(),
        EngineError::ModelsDiffer(_) => {
            "the replies for one call named different model versions; a cache may hold answers from the other version, so turn the cache off or prune it with thinkthen cache prune DIR --answered-by-other-than VERSION, naming the version a call with the cache off returns"
        }
        EngineError::UsageOverflow => "the backend reported token counts whose total is too large",
        EngineError::RecognizeKinds => "recognize takes 0 to 20 distinct, nonblank kinds",
        EngineError::RecognizeRelationNames { count, limit } => {
            return crate::engine::error::relation_names_message(*count, *limit);
        }
        EngineError::RecognizeRelationQuestions {
            names,
            count,
            limit,
        } => {
            return crate::engine::error::relation_questions_message(*names, *count, *limit);
        }
        EngineError::TextTooLong { bytes, limit } => {
            return format!("the text is {bytes} bytes, over recognize's limit of {limit}");
        }
        EngineError::RecognizeLogical => "the backend failed a question recognition requires",
    };
    fixed.to_owned()
}

const fn transport(kind: TransportKind) -> &'static str {
    match kind {
        TransportKind::Timeout => "the backend timed out",
        TransportKind::NameLookup => "the backend's host could not be found",
        TransportKind::Refused => "the backend refused the connection",
        TransportKind::PrematureClose => {
            "the backend closed the connection before a reply and may have received the request; it was not sent again"
        }
        TransportKind::Tls => {
            "the TLS connection or certificate check failed; check the backend's certificate trust"
        }
        TransportKind::Other => "the backend could not be reached",
    }
}
