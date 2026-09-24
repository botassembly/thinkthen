//! The conversions that carry each lower error into one command failure.

use super::Failure;
use crate::core::adapters::built_in::DecodeError;
use crate::core::{
    BackendError, QuestionFileError, QuestionSetError, ReadingError, RecordError, RenderError,
};
use crate::engine::error::Error as EngineError;

impl From<crate::config::ConfigError> for Failure {
    fn from(error: crate::config::ConfigError) -> Self {
        Self::Configuration(error.message)
    }
}

impl From<BackendError> for Failure {
    fn from(error: BackendError) -> Self {
        Self::Backend(error)
    }
}

impl From<QuestionFileError> for Failure {
    fn from(error: QuestionFileError) -> Self {
        Self::Question(error)
    }
}

impl From<QuestionSetError> for Failure {
    fn from(error: QuestionSetError) -> Self {
        Self::QuestionSet(error)
    }
}

impl From<DecodeError> for Failure {
    fn from(error: DecodeError) -> Self {
        Self::Reply(error)
    }
}

impl From<ReadingError> for Failure {
    fn from(error: ReadingError) -> Self {
        Self::Reading(error)
    }
}

impl From<RecordError> for Failure {
    fn from(error: RecordError) -> Self {
        Self::Record(error)
    }
}

impl From<RenderError> for Failure {
    fn from(error: RenderError) -> Self {
        Self::Render(error)
    }
}

impl From<EngineError> for Failure {
    fn from(error: EngineError) -> Self {
        match error {
            EngineError::Transport(message) => Self::Transport(message),
            EngineError::Status(status) => Self::Status(status),
            EngineError::Reply(error) => Self::Reply(error),
            EngineError::ReplayMiss(name) => Self::ReplayMiss(name),
            EngineError::Entry(name, message) => Self::Entry(name, message),
            EngineError::RecordingConflict(name) => Self::RecordingConflict(name),
            EngineError::RecordingStorage => Self::RecordingStorage,
            EngineError::RecordingPathIsFile => Self::RecordingPathIsFile,
            EngineError::RecordingBackendMismatch => Self::RecordingBackendMismatch,
            EngineError::RecordingFolderLegacy => Self::RecordingFolderLegacy,
            EngineError::DefaultCachePrivate => Self::DefaultCachePrivate,
            EngineError::CacheEntry => Self::CacheEntry,
            EngineError::Defect(message) => Self::Defect(message),
            EngineError::Usage(message) => Self::Usage(message),
            EngineError::ProfileLimit(limit) => Self::ProfileLimit(limit),
            EngineError::Cancelled => Self::Cancelled,
            EngineError::Deadline(_) => Self::Defect("an unavailable deadline reached the command"),
            EngineError::NoKey(variable) => Self::NoKey(variable.to_owned()),
            EngineError::WidthActive(active) => Self::WidthActive(active),
            EngineError::ModelsDiffer(models) => Self::ModelsDiffer(models),
            EngineError::UsageOverflow => Self::UsageOverflow,
            EngineError::RecognizeKinds => Self::Recognize(super::recognize::Error::Config {
                file: false,
                error: crate::core::RecognizeConfigError::Kinds,
            }),
            EngineError::RecognizeLogical => {
                Self::Recognize(super::recognize::Error::LogicalQuestion)
            }
        }
    }
}
