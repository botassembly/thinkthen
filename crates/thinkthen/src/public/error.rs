//! The one public error: six kinds, a safe message, and the retry signal.

use crate::engine::error::{Error as EngineError, Kind, TransportKind};

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

    pub(crate) fn of(kind: ErrorKind, message: impl Into<String>) -> Self {
        let detail = ErrorDetail {
            message: message.into(),
            retryable: false,
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
            | Self::Defect(detail) => detail.retryable = error.retryable(),
        }
        public
    }
}

/// One library sentence for each engine failure. The command words its own,
/// because its sentences name command-line options.
fn message(error: &EngineError) -> String {
    let fixed = match error {
        EngineError::Transport(kind) => transport(*kind),
        EngineError::Status(status) => return format!("the backend answered with status {status}"),
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
            "the recording folder belongs to another backend address"
        }
        EngineError::RecordingFolderLegacy => "the recording folder uses a retired layout",
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
            return format!("no key is set; set {variable} or call EngineBuilder::api_key");
        }
        EngineError::WidthActive(active) => return active.to_string(),
        EngineError::ModelsDiffer(_) => {
            "the backend returned different model versions for one call; pin the model and use a cache"
        }
        EngineError::UsageOverflow => "the backend reported token counts whose total is too large",
        EngineError::RecognizeKinds => "recognize takes 1 to 20 distinct, nonblank kinds",
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
        TransportKind::Other => "the backend could not be reached",
    }
}
