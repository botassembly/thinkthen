//! Typed facts needed by command diagnostics, without command rendering.
use super::{Error, diagnostic::Diagnostic};
use crate::{ErrorKind, engine::error::Error as Native};

mod classification;
pub use classification::{ConfigurationFailure, JsonFailure, ProfileFailure};

/// A validated subtype of backend connection failure.
pub use crate::engine::error::TransportKind as TransportFailure;

/// Known provider identifiers already admitted for existing diagnostics.
/// This view never contains a provider response body or its free-text message.
#[derive(Clone, Copy, Debug)]
pub struct ProviderFailure<'a> {
    /// The selected adapter's backend name.
    pub backend: &'static str,
    /// Whether an admitted provider identifier means exhausted credit.
    pub quota: bool,
    /// Whether the response has the existing alternate-backend hint.
    pub alternate_backend_hint: bool,
    /// Admitted provider error type, when present.
    pub error_type: Option<&'a str>,
    /// Admitted provider error code, when present.
    pub code: Option<&'a str>,
}

/// Typed safe facts for a caller's existing diagnostic presentation.
/// These are projections of retained causes, not a second error type.
/// They contain no provider bodies, original records or credential values.
#[derive(Debug)]
pub enum ErrorDiagnostic<'a> {
    /// Pure native JSON admission, without quoted input or duplicate paths.
    Json(JsonFailure),
    /// Syntax refusal while parsing one whole input document as JSON.
    /// The distinction from record JSON preserves the native producer's framing.
    InputJson {
        /// One-based source line.
        line: usize,
        /// One-based source column, or zero for empty input.
        column: usize,
    },
    /// A record's native JSON admission, preserving safe location facts.
    RecordJson(JsonFailure),
    /// A saved question's native JSON admission.
    QuestionJson {
        /// Native parser classification and numeric location.
        failure: JsonFailure,
        /// Whether the saved grammar reports a file as its source.
        from_file: bool,
    },
    /// Native threshold grammar or numeric admission.
    Threshold(crate::ThresholdError),
    /// Native backend-profile grammar, with no parser prose or authored values.
    Profile {
        /// Closed native refusal classification.
        failure: ProfileFailure,
        /// Whether the caller selected a profile file.
        from_file: bool,
    },
    /// Captured configuration admission with its existing value-free sentence.
    Configuration {
        /// Closed refusal classification.
        failure: ConfigurationFailure,
        /// Existing producer sentence; contains no paths, values or parser bodies.
        message: &'a str,
    },
    /// Backend connection classification.
    Transport(TransportFailure),
    /// Actual HTTP status.
    Status(u16),
    /// Known provider identifiers around an existing cause.
    Provider {
        /// The underlying retained diagnostic.
        cause: Box<Self>,
        /// Admitted provider identifiers only.
        provider: ProviderFailure<'a>,
    },
    /// A process send reservation was refused.
    SendBudget(crate::SendBudgetDenial),
    /// Final-body estimated-input admission was refused.
    EstimatedInput {
        /// Existing typed denial.
        reason: &'a crate::EstimatedInputDenial,
        /// Whether image token-estimate wording applies.
        images: bool,
    },
    /// The existing recognized input-token limit response.
    TokenLimit,
    /// A response exceeded the configured byte ceiling.
    ReplyTooLarge(u64),
    /// A fixed safe response-decoding sentence, never raw parser input.
    Reply(String),
    /// Strict replay could not find this generated question key.
    ReplayMiss(&'a str),
    /// Fixture and live store coexist in strict replay.
    StoreAmbiguous,
    /// A read-only store has an unfinished write.
    StoreHotJournal,
    /// A generated store filename has an old format.
    StoreVersion(&'a str),
    /// An identified local store entry failed the existing validation.
    Entry {
        /// Existing local entry name.
        name: &'a str,
        /// Existing safe validation reason.
        reason: &'a str,
    },
    /// The named recording could not be read or written.
    RecordingStorage,
    /// A reply forbids requested storage.
    RecordingForbidden,
    /// A directory selection names a regular file.
    RecordingPathIsFile,
    /// The private platform cache failed its ownership rules.
    DefaultCachePrivate,
    /// The count-only usage store failed its safe read check.
    UsageUnreadable(&'a str),
    /// A final cache entry is malformed.
    CacheEntry,
    /// An internal defect's fixed description.
    Defect(&'static str),
    /// A fixed native admission description.
    Usage(&'static str),
    /// A configured profile refused one request.
    ProfileLimit {
        /// The validated profile name.
        profile: &'a str,
        /// The existing dimension name.
        dimension: &'static str,
        /// The admitted ceiling.
        limit: usize,
        /// The measured request value.
        actual: usize,
    },
    /// Caller cancellation.
    Cancelled,
    /// The fixed native deadline description.
    Deadline(String),
    /// The selected variable lacked a usable credential.
    NoKey(&'a str),
    /// An existing process throttle conflicts with requested settings.
    WidthActive(String),
    /// Reported model versions differ within one result.
    ModelsDiffer(Option<(&'a str, &'a str)>),
    /// Reported token counts overflowed their aggregate.
    UsageOverflow,
    /// Recognition kind admission failed.
    RecognitionKinds,
    /// Existing safe recognition example admission sentence.
    RecognitionExamples(String),
    /// A required recognition question failed.
    RecognitionLogical,
    /// Recognition text exceeded its admitted byte ceiling.
    TextTooLong {
        /// Actual original byte count.
        bytes: usize,
        /// Admitted byte ceiling.
        limit: usize,
    },
    /// Existing safe recognition relation limit description.
    RecognitionRelationLimit(String),
    /// A native request range wraps an existing engine failure.
    Range {
        /// Zero-based first record in the request.
        first: usize,
        /// Zero-based last record in the request.
        last: usize,
        /// The original engine failure.
        cause: Box<Self>,
    },
    /// A partial reply omitted required rows in a known request range.
    PartialReply {
        /// Zero-based first request record.
        first: usize,
        /// Zero-based last request record.
        last: usize,
        /// Fixed safe decoder sentence, when a concrete decoder cause exists.
        cause: Option<String>,
    },
    /// Shared context admission exceeded its applicable limit.
    Context {
        /// Whether context alone caused the initial refusal.
        initial: bool,
        /// Existing dimension name.
        dimension: &'static str,
        /// Admitted ceiling.
        limit: usize,
        /// Measured body value.
        actual: usize,
        /// Validated configured profile name, if any.
        profile: Option<&'a str>,
    },
    /// Complete relation input exceeded its entity ceiling.
    RelationEntityCount(usize),
    /// A model selector has forbidden control characters.
    ModelControl,
    /// A model selector is blank.
    ModelBlank,
    /// The caller's pointer selector failed its closed grammar.
    Pointer {
        /// The fixed option name owning the selector.
        option: &'static str,
        /// The authored selector, never an input record or parser body.
        selector: &'a str,
        /// Fixed safe grammar description.
        reason: String,
    },
    /// Batch size admission failed.
    Batch,
    /// Annotation thresholds belong to individual questions.
    AnnotationThreshold,
    /// A selected question file failed its ordinary read.
    QuestionUnreadable(String),
    /// A selected question file exceeded its byte ceiling.
    QuestionTooLarge,
    /// A selected question file is not UTF-8.
    QuestionNotUtf8,
    /// A native question-file grammar failure.
    Question {
        /// Fixed safe parser description with existing authored field names.
        message: String,
        /// Whether its source was a saved question file.
        from_file: bool,
    },
    /// Existing safe annotation-set grammar description.
    QuestionSet(String),
    /// Existing safe framing grammar description.
    Reading(String),
    /// Existing safe record-admission description.
    Record(String),
    /// Input bytes cannot form UTF-8 text.
    InvalidUtf8,
    /// Native result rendering failed.
    Rendering(String),
    /// Existing safe recognition configuration description.
    RecognitionConfiguration {
        /// Safe grammar sentence.
        message: String,
        /// Whether the failing selector names a file.
        from_file: bool,
    },
    /// Existing safe relation configuration description.
    RelationConfiguration {
        /// Safe grammar sentence.
        message: String,
        /// Whether file-local error classification applies.
        from_file: bool,
    },
    /// Existing safe complete entity-set admission description.
    EntitySet(String),
}

impl Error {
    /// Inspect safe typed diagnostic facts without parsing the error's message.
    /// Command rendering and option advice belong to the caller. Native parser
    /// bodies, evidence and credentials never enter this view or its Debug output.
    #[must_use]
    pub fn diagnostic(&self) -> Option<ErrorDiagnostic<'_>> {
        Some(match self.detail().diagnostic.as_deref()? {
            Diagnostic::Configuration(cause) => ErrorDiagnostic::Configuration {
                failure: if cause.unreadable { ConfigurationFailure::Unreadable }
                    else if cause.price.is_some() { ConfigurationFailure::Prices }
                    else { ConfigurationFailure::Grammar },
                message: self.detail().message(),
            },
            Diagnostic::Engine(error) => engine(error),
            Diagnostic::EngineRange { cause, first, last } => ErrorDiagnostic::Range {
                first: *first, last: *last, cause: Box::new(engine(cause)),
            },
            Diagnostic::Context { initial, kind, limit, actual, profile } => ErrorDiagnostic::Context {
                initial: *initial, dimension: kind.words(), limit: *limit, actual: *actual,
                profile: profile.as_ref().map(|name| name.as_str()),
            },
            Diagnostic::PartialReply { cause, first, last } => ErrorDiagnostic::PartialReply {
                first: *first, last: *last, cause: cause.as_ref().map(ToString::to_string),
            },
            Diagnostic::RelationEntityCount(count) => ErrorDiagnostic::RelationEntityCount(*count),
            Diagnostic::Model(crate::core::BlankTextError::ModelControl) => ErrorDiagnostic::ModelControl,
            Diagnostic::Model(_) => ErrorDiagnostic::ModelBlank,
            Diagnostic::Pointer(option, selector, cause) => ErrorDiagnostic::Pointer {
                option, selector, reason: cause.to_string(),
            },
            Diagnostic::Batch => ErrorDiagnostic::Batch,
            Diagnostic::ThresholdFunction(crate::RequestFunction::Annotate) => ErrorDiagnostic::AnnotationThreshold,
            Diagnostic::QuestionRead(crate::QuestionFileError::Unreadable(cause)) => ErrorDiagnostic::QuestionUnreadable(cause.to_string()),
            Diagnostic::QuestionRead(crate::QuestionFileError::TooLarge) => ErrorDiagnostic::QuestionTooLarge,
            Diagnostic::QuestionRead(crate::QuestionFileError::NotUtf8) => ErrorDiagnostic::QuestionNotUtf8,
            Diagnostic::Refusal(cause) => return refusal(cause.as_ref(), self.kind()),
            _ => return None,
        })
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one exhaustive native-cause projection keeps diagnostic facts together"
)]
fn engine(error: &Native) -> ErrorDiagnostic<'_> {
    use ErrorDiagnostic as D;
    match error {
        Native::Transport(kind) => D::Transport(*kind),
        Native::Status(status) => D::Status(*status),
        Native::Provider { cause, diagnostic } => D::Provider {
            cause: Box::new(engine(cause)),
            provider: ProviderFailure {
                backend: diagnostic.backend, quota: diagnostic.quota,
                alternate_backend_hint: diagnostic.ollama,
                error_type: diagnostic.error_type.as_deref(), code: diagnostic.code.as_deref(),
            },
        },
        Native::SendBudgetFirst => D::SendBudget(crate::SendBudgetDenial::BeforeFirstSend),
        Native::SendBudgetAdditional => D::SendBudget(crate::SendBudgetDenial::BeforeAdditionalSend),
        Native::SendBudgetRetry(status) => D::SendBudget(crate::SendBudgetDenial::BeforeRetry { last_status: *status }),
        Native::EstimatedInput(reason) => D::EstimatedInput { reason, images: false },
        Native::ImageEstimatedInput(reason) => D::EstimatedInput { reason, images: true },
        Native::TokenLimit => D::TokenLimit,
        Native::ReplyTooLarge(limit) => D::ReplyTooLarge(*limit),
        Native::Reply(cause) => D::Reply(cause.to_string()),
        Native::QuestionMiss(key) => D::ReplayMiss(key),
        Native::StoreAmbiguous => D::StoreAmbiguous,
        Native::StoreHotJournal => D::StoreHotJournal,
        Native::StoreVersion(file) => D::StoreVersion(file),
        Native::Entry(name, reason) => D::Entry { name, reason },
        Native::RecordingStorage => D::RecordingStorage,
        Native::RecordingForbidden => D::RecordingForbidden,
        Native::RecordingPathIsFile => D::RecordingPathIsFile,
        Native::DefaultCachePrivate => D::DefaultCachePrivate,
        Native::UsageUnreadable(message) => D::UsageUnreadable(message),
        Native::CacheEntry => D::CacheEntry,
        Native::Defect(message) => D::Defect(message),
        Native::Usage(message) => D::Usage(message),
        Native::ProfileLimit(limit) => D::ProfileLimit {
            profile: limit.name.as_str(), dimension: limit.kind.words(),
            limit: limit.limit, actual: limit.actual,
        },
        Native::Cancelled => D::Cancelled,
        Native::Deadline(budget) => D::Deadline(budget.to_string()),
        Native::NoKey(variable) => D::NoKey(variable),
        Native::WidthActive(active) => D::WidthActive(active.to_string()),
        Native::ModelsDiffer(models) => D::ModelsDiffer(models.as_ref().map(|(a, b)| (a.as_str(), b.as_str()))),
        Native::UsageOverflow => D::UsageOverflow,
        Native::RecognizeKinds => D::RecognitionKinds,
        Native::RecognitionExamples(error) => D::RecognitionExamples(error.to_string()),
        Native::RecognizeLogical => D::RecognitionLogical,
        Native::TextTooLong { bytes, limit } => D::TextTooLong { bytes: *bytes, limit: *limit },
        Native::RecognizeRelationNames { count, limit } => D::RecognitionRelationLimit(crate::engine::error::relation_names_message(*count, *limit)),
        Native::RecognizeRelationQuestions { names, count, limit } => D::RecognitionRelationLimit(crate::engine::error::relation_questions_message(*names, *count, *limit)),
    }
}

fn refusal(cause: &(dyn std::any::Any + Send + Sync), kind: ErrorKind) -> Option<ErrorDiagnostic<'_>> {
    use ErrorDiagnostic as D;
    if let Some(error) = cause.downcast_ref::<crate::core::BlankTextError>() {
        match error {
            crate::core::BlankTextError::ModelControl => return Some(D::ModelControl),
            crate::core::BlankTextError::ModelName => return Some(D::ModelBlank),
            _ => {}
        }
    }
    if let Some(error) = cause.downcast_ref::<crate::core::JsonError>() {
        return Some(D::Json(classification::json(error)));
    }
    if let Some(error) = cause.downcast_ref::<crate::ThresholdError>() {
        return Some(D::Threshold(*error));
    }
    if let Some(error) = cause.downcast_ref::<crate::core::ProfileError>() {
        return Some(D::Profile { failure: classification::profile(error), from_file: kind == ErrorKind::Local });
    }
    if let Some(error) = cause.downcast_ref::<crate::core::ExamplesError>() {
        return Some(D::RecognitionExamples(error.to_string()));
    }
    if let Some(error) = cause.downcast_ref::<crate::core::RelateConfigError>() {
        return Some(D::RelationConfiguration {
            message: error.to_string(), from_file: kind == ErrorKind::Local && *error != crate::core::RelateConfigError::WrongVerb,
        });
    }
    if let Some(error) = cause.downcast_ref::<crate::core::RecognizeConfigError>() {
        return Some(D::RecognitionConfiguration { message: error.to_string(), from_file: kind == ErrorKind::Local });
    }
    if let Some(error) = cause.downcast_ref::<crate::core::EntitySetError>() {
        return Some(D::EntitySet(error.to_string()));
    }
    if let Some(error) = cause.downcast_ref::<crate::core::QuestionFileError>() {
        if let crate::core::QuestionFileError::NotJson(cause) = error {
            return Some(D::QuestionJson { failure: classification::json(cause), from_file: error.origin() == crate::core::Source::File });
        }
        return Some(D::Question { message: error.to_string(), from_file: error.origin() == crate::core::Source::File });
    }
    if let Some(error) = cause.downcast_ref::<crate::core::QuestionSetError>() {
        return Some(D::QuestionSet(error.to_string()));
    }
    if let Some(error) = cause.downcast_ref::<crate::core::ReadingError>() {
        return Some(D::Reading(error.to_string()));
    }
    if let Some(error) = cause.downcast_ref::<crate::core::RecordError>() {
        return Some(match error {
            crate::core::RecordError::NotUtf8 => D::InvalidUtf8,
            crate::core::RecordError::Json(cause) => D::RecordJson(classification::json(cause)),
            crate::core::RecordError::InputJson { line, column } => D::InputJson { line: *line, column: *column },
            crate::core::RecordError::Render(cause) => D::Rendering(cause.to_string()),
            _ => D::Record(error.to_string()),
        });
    }
    None
}

#[cfg(test)]
mod input_json_tests {
    use super::*;

    #[test]
    fn native_document_and_record_readers_keep_distinct_safe_syntax_projections() {
        let marker = "private-whole-document-marker";
        for text in [String::new(), format!("{{\n\"{marker}\":?}}") ] {
            for (framing, document) in [(crate::core::Framing::Document, true), (crate::core::Framing::Jsonl, false)] {
                let reading = crate::core::Reading::new(framing,
                    vec![crate::core::Pointer::new("/evidence").expect("pointer")]).expect("reading");
                let cause = reading.record(text.as_bytes()).expect_err("malformed JSON");
                let error = Error::refused(cause);
                let diagnostic = error.diagnostic().expect("retained native cause");
                let (line, column) = match diagnostic {
                    ErrorDiagnostic::InputJson { line, column } if document => (line, column),
                    ErrorDiagnostic::RecordJson(JsonFailure::Syntax { line, column }) if !document => (line, column),
                    other => panic!("unexpected diagnostic: {other:?}"),
                };
                assert_eq!(line, if text.is_empty() { 1 } else { 2 });
                assert_eq!(column == 0, text.is_empty());
                for shown in [format!("{error:?}"), error.to_string(), format!("{:?}", error.diagnostic()),
                    serde_json::to_string(&error.complete()).expect("safe error document")] {
                    assert!(!shown.contains(marker));
                }
                assert!(std::error::Error::source(&error).is_none());
            }
        }
    }
}
