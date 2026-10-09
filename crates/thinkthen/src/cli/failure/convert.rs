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
            EngineError::RecordingForbidden => Self::RecordingForbidden,
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
            EngineError::RecognitionExamples(error) => {
                Self::Recognize(super::recognize::Error::Examples {
                    record: None,
                    error,
                })
            }
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

impl From<crate::Error> for Failure {
    fn from(mut error: crate::Error) -> Self {
        use crate::public::error::diagnostic::Diagnostic;
        match error.take_diagnostic() {
            Some(Diagnostic::Engine(cause)) => Self::from(cause),
            Some(Diagnostic::EngineRange { cause, first, last }) => {
                let cause = Self::from(cause);
                match cause {
                    Self::Transport(_) | Self::Status(_) | Self::TokenLimit | Self::Reply(_)
                        if last > first =>
                    {
                        Self::BatchFailed {
                            last: last.saturating_add(1),
                            cause: Box::new(cause),
                        }
                    }
                    other => other,
                }
            }
            Some(Diagnostic::PartialReply { first, last, .. }) => Self::PartialReply {
                first: first.saturating_add(1),
                last: last.saturating_add(1),
            },
            Some(Diagnostic::Context {
                initial,
                kind,
                limit,
                actual,
                profile,
            }) => Self::Context(super::context::Error::OverLimit {
                initial,
                kind,
                limit,
                actual,
                profile,
            }),
            Some(Diagnostic::CliInput(cause)) => *cause,
            Some(Diagnostic::Model(cause)) => Self::Usage(match cause {
                crate::core::BlankTextError::ModelControl => {
                    "--model holds no control character or white space but a plain space"
                }
                _ => "--model is text, not white space",
            }),
            Some(Diagnostic::Pointer(option, typed, cause)) => Self::Pointer(option, typed, cause),
            Some(Diagnostic::Batch) => {
                Self::Usage("--batch takes max or a whole number of at least 1")
            }
            Some(Diagnostic::ThresholdFunction(crate::RequestFunction::Annotate)) => Self::Usage(
                "--threshold belongs to each question in the question set; `annotate` takes no command-level threshold",
            ),
            Some(Diagnostic::QuestionRead(cause)) => match cause {
                crate::QuestionFileError::Unreadable(cause) => Self::OpenQuestionFile(cause),
                crate::QuestionFileError::TooLarge => Self::QuestionFileTooLarge,
                crate::QuestionFileError::NotUtf8 => Self::OpenQuestionFile(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "stream did not contain valid UTF-8",
                )),
            },
            Some(Diagnostic::Refusal(cause)) => refusal(cause, error),
            _ if error.kind() == crate::ErrorKind::Cancelled => Self::Cancelled,
            _ => Self::Native(error.kind(), error.detail().message().to_owned()),
        }
    }
}

fn refusal(mut cause: Box<dyn std::any::Any + Send + Sync>, error: crate::Error) -> Failure {
    macro_rules! take {
        ($kind:ty) => {
            cause = match cause.downcast::<$kind>() {
                Ok(cause) => return Failure::from(*cause),
                Err(cause) => cause,
            };
        };
    }
    take!(crate::core::QuestionFileError);
    take!(crate::core::QuestionSetError);
    take!(crate::core::ReadingError);
    take!(crate::core::RecordError);
    let _cause = cause;
    Failure::Native(error.kind(), error.detail().message().to_owned())
}
