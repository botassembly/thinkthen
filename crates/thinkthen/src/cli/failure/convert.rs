//! The conversions that carry each lower error into one command failure.

use super::Failure;
use crate::core::adapters::built_in::DecodeError;
use crate::core::{
    BackendError, QuestionFileError, QuestionSetError, ReadingError, RecordError, RenderError,
};
use crate::engine::error::Error as EngineError;

impl From<crate::config::ConfigError> for Failure {
    fn from(error: crate::config::ConfigError) -> Self {
        if let Some(message) = error.price {
            Self::Usage(message)
        } else {
            Self::Configuration(error.message.into_owned())
        }
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
            EngineError::SendBudgetFirst => Self::Usage("the process send budget was spent"),
            EngineError::SendBudgetAdditional => {
                Self::Usage("the process send budget was spent before another request")
            }
            EngineError::SendBudgetRetry(_) => {
                Self::Usage("the process send budget was spent before a retry")
            }
            EngineError::EstimatedInput(reason) => Self::EstimatedInput(reason),
            EngineError::ImageEstimatedInput(reason) => Self::Image(reason.to_string().replace(
                "encoded-body-bytes-908-v1",
                "text-bytes-908-plus-image-tiles-v1",
            )),
            EngineError::TokenLimit => Self::TokenLimit,
            EngineError::ReplyTooLarge(limit) => Self::ReplyTooLarge(limit),
            EngineError::Reply(error) => Self::Reply(error),
            EngineError::QuestionMiss(key) => Self::QuestionMiss { key, context: None },
            EngineError::StoreAmbiguous => Self::StoreAmbiguous,
            EngineError::StoreHotJournal => Self::StoreHotJournal,
            EngineError::Entry(name, message) => Self::Entry(name, message),
            EngineError::RecordingStorage => Self::RecordingStorage,
            EngineError::RecordingPathIsFile => Self::RecordingPathIsFile,
            EngineError::DefaultCachePrivate => Self::DefaultCachePrivate,
            EngineError::UsageUnreadable(sentence) => Self::UsageUnreadable(sentence),
            EngineError::CacheEntry => Self::CacheEntry,
            EngineError::Defect(message) => Self::Defect(message),
            EngineError::Usage(message) => Self::Usage(message),
            EngineError::ProfileLimit(limit) => Self::ProfileLimit(limit),
            EngineError::Cancelled => Self::Cancelled,
            EngineError::Deadline(_) => Self::Defect("an unavailable deadline reached the command"),
            EngineError::NoKey(variable) => Self::NoKey(variable),
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
            EngineError::TextTooLong { bytes, limit } => {
                Self::Recognize(super::recognize::Error::TextTooLong { bytes, limit })
            }
            EngineError::RecognizeRelationNames { count, limit } => {
                Self::Recognize(super::recognize::Error::RelationLimit(
                    crate::engine::error::relation_names_message(count, limit),
                ))
            }
            EngineError::RecognizeRelationQuestions {
                names,
                count,
                limit,
            } => Self::Recognize(super::recognize::Error::RelationLimit(
                crate::engine::error::relation_questions_message(names, count, limit),
            )),
        }
    }
}
