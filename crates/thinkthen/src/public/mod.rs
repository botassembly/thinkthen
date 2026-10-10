//! The public Rust API over the private engine facade.
//!
//! Callers own an explicit [`Engine`] and its typed native results.

use std::sync::OnceLock;

mod images;
mod request;
pub use images::{
    ImageAdmission, ImageEvidence, ImageInput, ImageMedia, InputEvidence, InputFunction,
    MAX_IMAGE_BYTES, MAX_IMAGES, QuestionInput,
};
pub use request::*;
mod input_files;
pub use input_files::{
    ImageSourceRecord, InputFileReader, InputReaderOptions, ReaderMedia, SourceItem, SourceItems,
    read_inputs,
};
mod files;
mod table;
pub use table::{TableFormat, TableReader};
mod find_question;
pub use files::{
    FileReader, ReaderOptions, SourceRecord, SourceRecords, SourceUnit, enumerate_files, read_files,
};
pub use find_question::FindQuestionFile;

mod annotated;
mod asking;
pub(crate) mod batch;
mod builders;
mod bulk;
mod rank_question;
mod rank_set;
pub use rank_set::RankSet;
mod choice;
pub(crate) mod complete;
mod engine;
pub(crate) mod error;
#[cfg(feature = "polars")]
mod frame;
mod native_batch;
mod options;
pub(crate) use options::cli_reader;
mod panic;
mod plan;
pub(crate) use plan::SourceBudget;
mod proxy;
mod pull;
mod question;
pub(crate) use question::Kind as NativeQuestionKind;
mod question_file;
mod record_choose;
mod record_composition;
pub use record_choose::RecordChooseQuestion;
mod record_context;
mod record_input;
pub use crate::core::{Surface, SurfaceError};
pub use proxy::{
    CodeThreshold, ProxyActivation, ProxyId, ProxyMetadata, ProxyOverride, ProxyRequest,
};
pub use record_composition::{RawRecord, RecordEvidence, RecordReading, SourceLocation};
pub use record_context::{ObjectContext, RecordContext};
pub use record_input::{RecordInput, RecordOption, RecordOptions};
mod recognize;
pub use crate::core::{
    RecognitionExample, RecognitionExampleEntity, RecognitionExampleText, RecognitionMode,
    RecognitionSeedSpan, RecognitionStageContext,
};
mod recognize_question;
pub use recognize_question::RecognizeQuestionFile;
mod relate;
mod results;
pub use results::UsagePersistence;
mod set;
mod settings;

pub use crate::core::settings::{For, Settings, SettingsError};
pub use crate::core::{AnswerId, CallId, FailureId, IdentityError, ObservationId, SdkRequestId};
pub use crate::core::{MemberIdentity, Observation, Origin, QuestionSource, ResultIdentity};
pub use crate::core::{RelationDirection, RelationMethod, ReportedUsage};
/// The channel every ThinkThen binding waits on, so a forked child on macOS
/// can wait too (ticket 0365). It is for the bindings, not a documented API.
#[doc(hidden)]
pub use crate::engine::fork_safe;
pub use annotated::{Annotated, AnnotatedRecord, Failed, FailureCause, NamedAnnotation};
pub use batch::Batch;
pub use builders::{ChooseBuilder, DecideBuilder, LabelBuilder, ScoreBuilder, TagBuilder};
pub use choice::Choice;
pub use engine::{DecisionQuestion, DetailQuestion, Engine, Evidence};
pub use error::{Error, ErrorDetail, ErrorKind, StopCause, Stopped};
#[cfg(feature = "polars")]
pub use frame::{PolarsCallOptions, PolarsEngine, PolarsExprOptions};
pub use native_batch::RecoverableDetails;
pub use options::{
    BatchSetting, CallOptions, CancelToken, EstimatedInputDenial, SendBudget, SendBudgetDenial,
    process_requests_sent,
};
pub use panic::{contained, uncontained};
pub use plan::PlanEstimate;
pub use question::{
    BandedQuestion, ChooseQuestion, Description, DescriptionBuilder, LoadedQuestion, Question,
    QuestionKind, TagQuestion,
};
pub use question_file::{QuestionFileError, read_question_file};
pub use recognize::{
    BoundaryProposal, Kind, RecognitionValue, Recognize, RecognizeBuilder, Recognized,
    RecognizedEntity, Relation, RelationRule,
};
pub use relate::{Edge, Entity, Relate, RelateBuilder};
pub use results::FindSelection;
#[cfg(test)]
pub(crate) use results::QuestionJson;
pub use results::{
    Answer, AttemptObservation, AttemptOutcome, BatchMismatch, Call, Candidate, CompleteAnnotated,
    CompleteAnnotationMember, CompleteAttempt, CompleteChoice, CompleteDecision, CompleteFacts,
    CompleteFilter, CompleteFound, CompleteRank, CompleteRankMember, CompleteRecord, CompleteScore,
    CompleteSetRank, CompleteTags, Counters, Details, DoorReply, Facts, Found, Judgment,
    NamedProbability, ObservedRow, Picked, Probabilities, ProfileMismatch, QuestionDetail, Ranked,
    RankedRow, RecordObservation, ResultMetadata, Row, SetRanked, Tally, TallyStart, Usage,
    complete_call_schema,
};
pub use results::{
    CompleteRecognized, NameProbabilities, PairProbability, PieceProbabilities,
    RecognitionProbabilities, RecognitionProposal,
};
pub use results::{
    CompleteRelated, CompleteRelationMember, SourceRelationEdge, SourceRelationEndpoint,
};
pub use results::{OwnedObservedRow, OwnedQuestionDetail, OwnedRecordObservation};
pub use set::{QuestionSet, QuestionSetBuilder};
pub use settings::EngineBuilder;

/// The Polars crate the door takes, so a caller names the exact version.
#[cfg(feature = "polars")]
pub use ::polars;

/// Temporary support for language adapters awaiting their owned-engine migration.
/// New Rust callers build and retain their own [`Engine`].
///
/// # Errors
///
/// As [`Engine::from_env`].
#[doc(hidden)]
pub fn default_engine() -> Result<&'static Engine, Error> {
    static ENGINE: OnceLock<Engine> = OnceLock::new();
    if let Some(engine) = ENGINE.get() {
        return Ok(engine);
    }
    // Two threads that both find the cell empty each build; one engine is
    // dropped unused. `from_env` registers no throttle, so the race costs one
    // extra build and nothing else.
    let built = Engine::from_env()?;
    Ok(ENGINE.get_or_init(|| built))
}

/// The [`default_engine`]'s totals.
///
/// # Errors
///
/// As [`default_engine`].
#[doc(hidden)]
pub fn usage() -> Result<Counters, Error> {
    Ok(default_engine()?.usage())
}

pub use results::{
    FindReading, QuestionContent, ResolvedOption, ResolvedQuestion, ResolvedThreshold,
};

pub use results::{RecognitionReading, RelationReading, ResolvedRelationRule};

pub use results::{CompleteCall, CompleteError, ErrorSnapshot};

mod declarations;
pub use crate::core::{
    InputDeclaration, InputProperty, InputPropertyType, ObjectDeclaration, QuestionName,
    WordingVersion,
};

mod question_metadata;
mod question_preparation;

pub(crate) mod named_question;
pub use crate::core::QuestionRole as QuestionFileRole;
pub use named_question::QuestionFileReference;

pub use results::{
    SourceBoundaryProposal, SourceRecognition, SourceRecognizedEntity, SourceRecognizedRelation,
};
