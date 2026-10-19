//! The `thinkthen` command and local stdio server.

#![cfg_attr(not(windows), forbid(unsafe_code))]
#![cfg_attr(windows, deny(unsafe_code))]

use std::process::ExitCode;
use thinkthen::{BatchSetting, Engine, QuestionSet, RawRecord, RequestFraming, ResolvedSettings, RequestSessionRead, RequestSessionResult, PlanEstimate, CompleteAnnotated, InputDeclaration, AdmittedRequest, QuestionKind, RequestDefinition, CompleteChoice, CompleteDecision, CompleteFilter, CompleteRank, CompleteScore, CompleteTags, ResultMetadata, RequestEnvironment, RequestInput, EngineBuilder, CacheSettings, Facts, Tally, ImageEvidence, InputFileReader, InputReaderOptions, ReaderMedia, ReaderOptions, SourceItem, SourceItems, SourceUnit, FileReader, TableFormat, TableReader, QuestionFileError, read_question_file, Error, RecordEvidence, RecordInput, RecordReading, Recognize, SourceRecord, RecognitionValue, Recognized, RecognizedEntity, Relate, CompleteRelated, RequestFunction, Request, RequestArguments, RequestBatch, RequestImage, RequestOptions, RequestQuestion, RequestSource, Description, Kind, RelationRule, Threshold, CallOptions, CancelToken, Call, RequestValue, ImageMedia, RequestDescriptor, RequestFeed, RequestItem, RequestOriginal, Probability, Framing, Record, CompleteRecord, QuestionInput, Origin, ErrorDiagnostic, Surface, RequestSessionFeedOptions, RequestSessionRow, RequestSessionPushStatus, RequestSessionWaitInterest, RequestSessionReady, RequestSessionDescriptor, SourceLocation, Setting, QuestionContent, Question, validate_pointer, RankSet, QuestionFileReference, QuestionFileRole, RecordChooseQuestion, RecordContext, storage_writable_by_another, OwnedRecordObservation, cache_unused, convert_cache, CachePruneOptions, preview_cache_prune, prune_cache, AttemptOutcome, Environment, Input, RequestOutcome, ErrorKind, CallId, Counters, UsageFailure, UsagePersistence, FindQuestionFile, LoadedQuestion, CompleteFound, FindSelection, Candidate, find_candidate_count_range, read_inputs, ImageInput, MAX_IMAGE_BYTES, enumerate_files, RecognizeQuestionFile, RecognitionExample, RecognitionMode, CompleteRecognized, RequestThreshold, RelationMethod, SourceRelationEndpoint, RecognitionStageContext, Answer, QuestionSource, current_usage_month, read_usage, cache_counts, UsageFailureCategory, JsonFailure, ProxyActivation, CompleteError, SettledAnswer, CompleteFacts, RecordOptions, request_schema, MAX_IMAGES};

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
