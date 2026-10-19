//! Public diagnostic facts become command-owned wording and failure shapes.

use super::Failure;
use thinkthen::{ConfigurationFailure, Error, ErrorDiagnostic as D, ErrorKind};

impl From<crate::render::RenderError> for Failure {
    fn from(error: crate::render::RenderError) -> Self {
        Self::Render(error)
    }
}

impl From<Error> for Failure {
    fn from(error: Error) -> Self {
        if let Some(failure) = super::source::take(&error) {
            return failure;
        }
        match error.diagnostic() {
            Some(D::Json(_) | D::Profile { .. }) => {
                Self::Native(error.kind(), error.to_string())
            }
            Some(diagnostic) => diagnostic_failure(diagnostic, error.retryable()),
            None if error.kind() == ErrorKind::Cancelled => Self::Cancelled,
            None => Self::Native(error.kind(), error.to_string()),
        }
    }
}

#[expect(clippy::too_many_lines, reason = "one exhaustive public diagnostic projection keeps command failure mappings together")]
fn diagnostic_failure(diagnostic: D<'_>, retryable: bool) -> Failure {
    use Failure as F;
    match diagnostic {
        D::Transport(kind) => F::Transport(kind),
        D::Status(status) => F::Status(status, retryable),
        D::Provider { cause, provider } => F::Provider {
            cause: Box::new(diagnostic_failure(*cause, retryable)),
            diagnostic: super::provider::Diagnostic::from(provider),
        },
        D::SendBudget(reason) => F::Usage(match reason {
            thinkthen::SendBudgetDenial::BeforeFirstSend => "the process send budget was spent",
            thinkthen::SendBudgetDenial::BeforeAdditionalSend => "the process send budget was spent before another request",
            thinkthen::SendBudgetDenial::BeforeRetry { .. } => "the process send budget was spent before a retry",
        }),
        D::EstimatedInput { reason, images: false } => F::EstimatedInput(*reason),
        D::EstimatedInput { reason, images: true } => F::Image(reason.to_string().replace(
            "encoded-body-bytes-908-v1", "text-bytes-908-plus-image-tiles-v1",
        )),
        D::TokenLimit => F::TokenLimit,
        D::ReplyTooLarge(limit) => F::ReplyTooLarge(limit),
        D::Reply(message) => F::Reply(message),
        D::ReplayMiss(key) => F::QuestionMiss { key: key.to_owned(), context: None },
        D::StoreAmbiguous => F::StoreAmbiguous,
        D::StoreHotJournal => F::StoreHotJournal,
        D::StoreVersion(file) => F::StoreVersion(file.to_owned()),
        D::Entry { name, reason } => F::Entry(name.to_owned(), reason.to_owned()),
        D::RecordingStorage => F::RecordingStorage,
        D::RecordingForbidden => F::RecordingForbidden,
        D::RecordingPathIsFile => F::RecordingPathIsFile,
        D::DefaultCachePrivate => F::DefaultCachePrivate,
        D::UsageUnreadable(message) => F::UsageUnreadable(message.to_owned()),
        D::CacheEntry => F::CacheEntry,
        D::Defect(message) => F::Defect(message),
        D::Usage(message) => F::Usage(message),
        D::ProfileLimit { profile, dimension, limit, actual } => F::ProfileLimit {
            profile: profile.to_owned(), dimension, limit, actual,
        },
        D::Cancelled => F::Cancelled,
        D::Deadline(_) => F::Defect("an unavailable deadline reached the command"),
        D::NoKey(variable) => F::NoKey(variable.to_owned()),
        D::WidthActive(message) => F::WidthActive(message),
        D::ModelsDiffer(models) => F::ModelsDiffer(models.map(|(a,b)| (a.to_owned(), b.to_owned()))),
        D::UsageOverflow => F::UsageOverflow,
        D::RecognitionKinds => F::Recognize(super::recognize::Error::Config {
            file: false,
            error: "recognize takes distinct, nonblank kinds".to_owned(),
        }),
        D::RecognitionExamples(error) => F::Recognize(super::recognize::Error::Examples { record: None, error }),
        D::RecognitionLogical => F::Recognize(super::recognize::Error::LogicalQuestion),
        D::TextTooLong { bytes, limit } => F::Recognize(super::recognize::Error::TextTooLong { bytes, limit }),
        D::RecognitionRelationLimit(message) => F::Recognize(super::recognize::Error::RelationLimit(message)),
        D::Range { first, last, cause } => {
            let cause = diagnostic_failure(*cause, retryable);
            match cause {
                F::Transport(_) | F::Status(..) | F::Provider { .. } | F::TokenLimit | F::Reply(_) if last > first =>
                    F::BatchFailed { last: last.saturating_add(1), cause: Box::new(cause) },
                other => other,
            }
        }
        D::PartialReply { first, last, .. } => F::PartialReply {
            first: first.saturating_add(1), last: last.saturating_add(1),
        },
        D::Context { initial, dimension, limit, actual, profile } => F::Context(super::context::Error::OverLimit {
            initial, dimension, limit, actual, profile: profile.map(str::to_owned),
        }),
        D::RelationEntityCount(count) => F::Relate(super::relate::Error::TooMany { count }),
        D::ModelControl => F::Usage("--model holds no control character or white space but a plain space"),
        D::ModelBlank => F::Usage("--model is text, not white space"),
        D::Pointer { option, selector, reason } => F::Pointer(option, selector.to_owned(), reason),
        D::Batch => F::Usage("--batch takes max or a whole number of at least 1"),
        D::AnnotationThreshold => F::Usage("--threshold belongs to each question in the question set; `annotate` takes no command-level threshold"),
        D::QuestionUnreadable(message) => F::OpenQuestionFile(std::io::Error::other(message)),
        D::QuestionTooLarge => F::QuestionFileTooLarge,
        D::QuestionNotUtf8 => F::OpenQuestionFile(std::io::Error::new(
            std::io::ErrorKind::InvalidData, "stream did not contain valid UTF-8",
        )),
        D::Question { message, from_file } => F::Question { message, from_file },
        D::QuestionSet(message) => F::QuestionSet(message.into()),
        D::Reading(message) => F::Reading(message),
        D::Record(message) => F::Record(message),
        D::MissingPointer { pointer } => F::Record(format!("the record holds nothing at `{pointer}`")),
        D::InvalidUtf8 => F::InvalidUtf8 { record: false },
        D::Rendering(message) => F::Native(ErrorKind::Defect, format!("defect: {message}")),
        D::RecognitionConfiguration { message, from_file } => F::Recognize(super::recognize::Error::Config { file: from_file, error: message }),
        D::RelationConfiguration { message, from_file } => F::Relate(super::relate::Error::Config { file: from_file, error: message }),
        D::EntitySet(message) => F::Relate(super::relate::Error::Entities(message)),
        D::Json(failure) | D::RecordJson(failure) => F::RecordJson(failure),
        D::InputJson { line, column } => F::Record(format!(
            "the input is not valid JSON: the JSON at line {line} column {column} is not one"
        )),
        D::QuestionJson { failure, from_file } => F::Question {
            message: super::json::question(failure), from_file,
        },
        D::Threshold(error) => F::Native(ErrorKind::Usage, format!("--threshold: {error}")),
        D::Profile { failure, from_file } => F::Native(
            if from_file { ErrorKind::Local } else { ErrorKind::Usage },
            format!("the profile JSON {}", super::profile::reason(failure)),
        ),
        D::Configuration { failure, message } => F::Native(
            if failure == ConfigurationFailure::Prices { ErrorKind::Usage } else { ErrorKind::Local },
            message.to_owned(),
        ),
    }
}

#[cfg(test)]
mod conformance_tests;
