//! The `thinkthen` command and local stdio server.

#![cfg_attr(not(windows), forbid(unsafe_code))]
#![cfg_attr(windows, deny(unsafe_code))]

use std::process::ExitCode;
use thinkthen::{
    AdmittedRequest, Answer, AttemptOutcome, BatchSetting, CachePruneOptions, CacheSettings, Call,
    CallId, CallOptions, CancelToken, Candidate, CompleteAnnotated, CompleteChoice,
    CompleteDecision, CompleteError, CompleteFacts, CompleteFilter, CompleteFound, CompleteRank,
    CompleteRecognized, CompleteRecord, CompleteRelated, CompleteScore, CompleteTags, Counters,
    Description, Engine, EngineBuilder, Error, ErrorDiagnostic, ErrorKind, Facts, FileReader,
    FindQuestionFile, FindSelection, ImageEvidence, ImageInput, ImageMedia, InputDeclaration,
    InputFileReader, InputReaderOptions, JsonFailure, Kind, LoadedQuestion, MAX_IMAGE_BYTES,
    MAX_IMAGES, Origin, OwnedRecordObservation, PlanEstimate, Probability, ProxyActivation,
    Question, QuestionContent, QuestionFileError, QuestionFileReference, QuestionFileRole,
    QuestionInput, QuestionKind, QuestionSet, QuestionSource, RankSet, RawRecord, ReaderMedia,
    ReaderOptions, RecognitionExample, RecognitionMode, RecognitionStageContext, RecognitionValue,
    Recognize, RecognizeQuestionFile, Recognized, RecognizedEntity, RecordChooseQuestion,
    RecordContext, RecordEvidence, RecordInput, RecordOptions, RecordReading, Relate,
    RelationMethod, RelationRule, Request, RequestArguments, RequestBatch, RequestDefinition,
    RequestDescriptor, RequestEnvironment, RequestFeed, RequestFraming, RequestFunction,
    RequestImage, RequestInput, RequestItem, RequestOptions, RequestOriginal, RequestOutcome,
    RequestQuestion, RequestSessionDescriptor, RequestSessionFeedOptions, RequestSessionPushStatus,
    RequestSessionRead, RequestSessionResult, RequestSessionRow,
    RequestSource, RequestThreshold, RequestValue, ResolvedSettings,
    ResultMetadata, SettledAnswer, SourceItem, SourceItems, SourceLocation, SourceRecord,
    SourceRelationEndpoint, SourceUnit, Surface, TableFormat, TableReader, Tally, Threshold,
    UsageFailure, UsageFailureCategory, UsagePersistence, cache_counts, cache_unused,
    convert_cache, current_usage_month, enumerate_files, find_candidate_count_range,
    preview_cache_prune, prune_cache, read_inputs, read_question_file, read_usage, request_schema,
    storage_writable_by_another, validate_pointer,
};

#[cfg(test)]
mod schema_tests;
#[cfg(test)]
mod test_deadline;

mod cli;
mod mcp;
mod plan_document;
mod probes;
mod render;
mod saved_json;
mod statistics;
#[cfg(windows)]
mod windows;

use cli::{args, asking, edge, failure, judge, profile, schedule};

fn main() -> ExitCode {
    cli::entry()
}
