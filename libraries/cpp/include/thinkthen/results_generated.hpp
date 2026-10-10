// Generated from the shared Rust result graph; do not edit.
#pragma once
#include "result_value.hpp"
namespace tt::results {
inline constexpr const char* request_version = "thinkthen.request/1";
class Annotation;
class AnnotationMember;
class AnnotationMemberAnswerId;
class AnnotationMemberFailureId;
class AnnotationValue;
class AnnotationValueChoice;
class AnnotationValueDecision;
class AnnotationValueFailed;
class AnnotationValueScore;
class AnnotationValueTags;
class AnswerId;
class Answers;
class AtomicArrayOfString;
class AtomicDecideValue;
class AtomicNonZeroUsize;
class AtomicNullableString;
class AtomicBoolean;
class AtomicDouble;
class Attempt;
class Batch;
class BoundaryMode;
class BoundaryOdds;
class BoundaryProposal;
class CallError;
class CallId;
class DecideValue;
class EntityDocument;
class Error;
class EstimatedInputDenial;
class EstimatedInputDenialAdditionalRequest;
class EstimatedInputDenialInitialRequest;
class EstimatedInputDenialRetry;
class Facts;
class FailureId;
class Find;
class FindCandidate;
class Image;
class ImageMedia;
class InputDeclaration;
class InputDeclarationObject;
class InputDeclarationString;
class InputPropertyType;
class InputPropertyTypeArray;
class InputPropertyTypeBoolean;
class InputPropertyTypeNumber;
class InputPropertyTypeString;
class Label;
class Meta;
class ObjectRoot;
class ObjectType;
class Observation;
class ObservationId;
class ObservationFailureId;
class ObservationObservationId;
class Origin;
class PersistenceObservation;
class PhysicalSource;
class Position;
class QuestionName;
class QuestionSource;
class RankMember;
class RankMemberResult;
class ReadableQuestion;
class ReadableQuestion2;
class ReadableQuestion3;
class ReadableQuestion4;
class ReadableQuestionChoose;
class ReadableQuestionDecide;
class ReadableQuestionScore;
class ReadableQuestionTag;
class Recognition;
class RecognitionEdgeDocument;
class RecognitionMode;
class RecognitionOdds;
class RecognitionOddsFieldsNamesPairsPiecesProposals;
class RecognitionOddsFieldsPiecesProposals;
class RecognitionProposal;
class RecognitionStageContext;
class Relation;
class RelationDirection;
class RelationMember;
class RelationMemberAnswerId;
class RelationMemberFailureId;
class RelationMethod;
class RequestFunction;
class SdkRequestId;
class SendBudgetDenial;
class SendBudgetDenialBeforeAdditionalSend;
class SendBudgetDenialBeforeFirstSend;
class SendBudgetDenialBeforeRetry;
class StopCause;
class Stopped;
class StringRoot;
class StringType;
class Usage;
class UsagePersistence;
class Verb;
class Version;
class WordingVersion;
class AnnotatedField;
class AnnotatedRow;
class Answer;
class AnswerChoice;
class AnswerScore;
class AnswerTag;
class AnswerYesNo;
class AttemptOutcome;
class BatchSetting;
class BatchWarning;
class Entity;
class EntityEdge;
class Failed;
class Failure;
class FailureCause;
class FailureKind;
class FindAnswer;
class NameOdds;
class PairOdds;
class PieceOdds;
class Place;
class ProfileWarning;
class Recognize;
class RecognizeAnswer;
class RecognizeFieldsEntities;
class RecognizeFieldsModeProposals;
class RelateFields;
class RelatedEntity;
class RelatedEntityEdge;
class RelatedEntityEdgePropertiesSource;
class RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord;
class RelatedEntityEdgePropertiesSourceFieldsKindName;
class RelationRule;
class SessionAnnotation;
class SessionInputSource;
class SessionJudgment;
class SessionJudgmentChoice;
class SessionJudgmentDecision;
class SessionJudgmentScore;
class SessionJudgmentTags;
class SessionNamedProbability;
class SessionObservation;
class SessionObservationQuestion;
class SessionObservationRow;
class SessionObservedRow;
class SessionObservedRowAnnotated;
class SessionObservedRowFind;
class SessionObservedRowJudgment;
class SessionObservedRowRecognized;
class SessionObservedRowRelations;
class SessionPacket;
class SessionPacketAnnotateAggregate;
class SessionPacketAnnotateRow;
class SessionPacketChooseAggregate;
class SessionPacketChooseRow;
class SessionPacketDecideAggregate;
class SessionPacketDecideRow;
class SessionPacketFilterAggregate;
class SessionPacketFilterRow;
class SessionPacketFindAggregate;
class SessionPacketObservation;
class SessionPacketRankAggregate;
class SessionPacketRecognizeAggregate;
class SessionPacketRelateAggregate;
class SessionPacketScoreAggregate;
class SessionPacketScoreRow;
class SessionPacketTagAggregate;
class SessionPacketTagRow;
class SessionPacketTerminal;
class SessionProbabilities;
class SessionProbabilitiesNamed;
class SessionProbabilitiesYesNo;
class SessionQuestionDetail;
class SessionRecognition;
class SessionRelationEdge;
class SourceRelationEndpoint;
class Threshold;
class TokenUsage;
class Value;
class Annotation : public Node { public: using Node::Node;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<std::pair<std::string, AnnotationMember>>> answers() const;
    Presence<std::string> file() const;
    Presence<uint64_t> first_line() const;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<uint64_t> last_line() const;
    Presence<Meta> meta() const;
    Presence<Position> position() const;
    Presence<Version> schema() const;
    Presence<PhysicalSource> source() const;
    Presence<AnnotatedRow> value() const;
};
class AnnotationMember : public Node { public: using Node::Node;
    std::optional<AnnotationMemberAnswerId> as_AnnotationMemberAnswerId() const;
    std::optional<AnnotationMemberFailureId> as_AnnotationMemberFailureId() const;
};
class AnnotationMemberAnswerId : public Node { public: using Node::Node;
    Presence<Answer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<Observation>> observations() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::vector<QuestionSource>> question_sources() const;
    Presence<std::string> request() const;
    Presence<Threshold> threshold() const;
    Presence<Usage> usage() const;
    Presence<Value> value() const;
};
class AnnotationMemberFailureId : public Node { public: using Node::Node;
    Presence<Failure> failure() const;
    Presence<FailureId> failure_id() const;
    Presence<std::vector<Observation>> observations() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::vector<QuestionSource>> question_sources() const;
    Presence<std::string> request() const;
    Presence<Threshold> threshold() const;
    Presence<Usage> usage() const;
};
class AnnotationValue : public Node { public: using Node::Node;
    std::optional<AnnotationValueDecision> as_AnnotationValueDecision() const;
    std::optional<AnnotationValueChoice> as_AnnotationValueChoice() const;
    std::optional<AnnotationValueScore> as_AnnotationValueScore() const;
    std::optional<AnnotationValueTags> as_AnnotationValueTags() const;
    std::optional<AnnotationValueFailed> as_AnnotationValueFailed() const;
};
class AnnotationValueChoice : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::string> value() const;
};
class AnnotationValueDecision : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<bool> value() const;
};
class AnnotationValueFailed : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<Failure> value() const;
};
class AnnotationValueScore : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<double> value() const;
};
class AnnotationValueTags : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::vector<std::string>> value() const;
};
class AnswerId : public Node { public: using Node::Node;
    std::string value() const;
};
class Answers : public Node { public: using Node::Node;
    Presence<std::vector<RelationMember>> questions() const;
};
class AtomicArrayOfString : public Node { public: using Node::Node;
    Presence<Answer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<Image>> images() const;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<std::vector<RankMember>> members() const;
    Presence<Meta> meta() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::string> question_name() const;
    Presence<Version> schema() const;
    Presence<PhysicalSource> source() const;
    Presence<Threshold> threshold() const;
    Presence<std::vector<std::string>> value() const;
};
class AtomicDecideValue : public Node { public: using Node::Node;
    Presence<Answer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<Image>> images() const;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<std::vector<RankMember>> members() const;
    Presence<Meta> meta() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::string> question_name() const;
    Presence<Version> schema() const;
    Presence<PhysicalSource> source() const;
    Presence<Threshold> threshold() const;
    Presence<DecideValue> value() const;
};
class AtomicNonZeroUsize : public Node { public: using Node::Node;
    Presence<Answer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<Image>> images() const;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<std::vector<RankMember>> members() const;
    Presence<Meta> meta() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::string> question_name() const;
    Presence<Version> schema() const;
    Presence<PhysicalSource> source() const;
    Presence<Threshold> threshold() const;
    Presence<uint64_t> value() const;
};
class AtomicNullableString : public Node { public: using Node::Node;
    Presence<Answer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<Image>> images() const;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<std::vector<RankMember>> members() const;
    Presence<Meta> meta() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::string> question_name() const;
    Presence<Version> schema() const;
    Presence<PhysicalSource> source() const;
    Presence<Threshold> threshold() const;
    Presence<std::string> value() const;
};
class AtomicBoolean : public Node { public: using Node::Node;
    Presence<Answer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<Image>> images() const;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<std::vector<RankMember>> members() const;
    Presence<Meta> meta() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::string> question_name() const;
    Presence<Version> schema() const;
    Presence<PhysicalSource> source() const;
    Presence<Threshold> threshold() const;
    Presence<bool> value() const;
};
class AtomicDouble : public Node { public: using Node::Node;
    Presence<Answer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<Image>> images() const;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<std::vector<RankMember>> members() const;
    Presence<Meta> meta() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::string> question_name() const;
    Presence<Version> schema() const;
    Presence<PhysicalSource> source() const;
    Presence<Threshold> threshold() const;
    Presence<double> value() const;
};
class Attempt : public Node { public: using Node::Node;
    Presence<uint64_t> ordinal() const;
    Presence<AttemptOutcome> outcome() const;
    Presence<std::string> request_id() const;
    Presence<std::string> request_sha256() const;
    Presence<SdkRequestId> sdk_request_id() const;
    Presence<uint64_t> server_ms() const;
    Presence<uint64_t> status() const;
    Presence<uint64_t> wall_ms() const;
};
class Batch : public Node { public: using Node::Node;
    std::variant<uint64_t, std::string> value() const;
};
class BoundaryMode : public Node { public: using Node::Node;
    std::string value() const;
};
class BoundaryOdds : public Node { public: using Node::Node;
    Presence<std::vector<PieceOdds>> pieces() const;
    Presence<std::vector<BoundaryProposal>> proposals() const;
};
class BoundaryProposal : public Node { public: using Node::Node;
    Presence<uint64_t> end() const;
    Presence<uint64_t> length() const;
    Presence<double> probability() const;
    Presence<uint64_t> start() const;
    Presence<std::string> text() const;
};
class CallError : public Node { public: using Node::Node;
    Presence<Error> error() const;
    Presence<Facts> facts() const;
};
class CallId : public Node { public: using Node::Node;
    std::string value() const;
};
class DecideValue : public Node { public: using Node::Node;
    Json value() const;
};
class EntityDocument : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::string> name() const;
};
class Error : public Node { public: using Node::Node;
    Presence<EstimatedInputDenial> estimated_input_denial() const;
    Presence<FailureKind> kind() const;
    Presence<std::string> message() const;
    Presence<bool> retryable() const;
    Presence<SendBudgetDenial> send_budget_denial() const;
    Presence<Stopped> stopped() const;
};
class EstimatedInputDenial : public Node { public: using Node::Node;
    std::optional<EstimatedInputDenialInitialRequest> as_EstimatedInputDenialInitialRequest() const;
    std::optional<EstimatedInputDenialAdditionalRequest> as_EstimatedInputDenialAdditionalRequest() const;
    std::optional<EstimatedInputDenialRetry> as_EstimatedInputDenialRetry() const;
};
class EstimatedInputDenialAdditionalRequest : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<uint64_t> limit() const;
};
class EstimatedInputDenialInitialRequest : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<uint64_t> limit() const;
};
class EstimatedInputDenialRetry : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<uint64_t> last_status() const;
    Presence<uint64_t> limit() const;
};
class Facts : public Node { public: using Node::Node;
    Presence<std::vector<Attempt>> attempts() const;
    Presence<uint64_t> cache_answers() const;
    Presence<CallId> call_id() const;
    Presence<std::string> estimated_cost_usd() const;
    Presence<bool> held_model_mismatch() const;
    Presence<uint64_t> input_tokens() const;
    Presence<uint64_t> largest_request_bytes() const;
    Presence<uint64_t> largest_request_estimated_input_tokens() const;
    Presence<std::string> model() const;
    Presence<uint64_t> output_tokens() const;
    Presence<uint64_t> records() const;
    Presence<uint64_t> requests_sent() const;
    Presence<double> seconds() const;
    Presence<std::string> token_estimate_method() const;
    Presence<PersistenceObservation> usage_persistence() const;
};
class FailureId : public Node { public: using Node::Node;
    std::string value() const;
};
class Find : public Node { public: using Node::Node;
    Presence<FindAnswer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<FindCandidate>> candidates() const;
    Presence<std::string> file() const;
    Presence<uint64_t> first_line() const;
    Presence<uint64_t> index() const;
    Presence<uint64_t> last_line() const;
    Presence<Meta> meta() const;
    Presence<Position> position() const;
    Presence<ReadableQuestion2> question() const;
    Presence<Version> schema() const;
    Presence<std::nullptr_t> threshold() const;
    Presence<Json> value() const;
};
class FindCandidate : public Node { public: using Node::Node;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<double> probability() const;
    Presence<PhysicalSource> source() const;
};
class Image : public Node { public: using Node::Node;
    Presence<std::string> base64() const;
    Presence<uint32_t> height() const;
    Presence<ImageMedia> media() const;
    Presence<uint32_t> width() const;
};
class ImageMedia : public Node { public: using Node::Node;
    std::string value() const;
};
class InputDeclaration : public Node { public: using Node::Node;
    std::optional<InputDeclarationString> as_InputDeclarationString() const;
    std::optional<InputDeclarationObject> as_InputDeclarationObject() const;
};
class InputDeclarationObject : public Node { public: using Node::Node;
    Presence<std::vector<std::pair<std::string, InputPropertyType>>> properties() const;
    Presence<std::vector<std::string>> required() const;
    Presence<ObjectType> type() const;
};
class InputDeclarationString : public Node { public: using Node::Node;
    Presence<StringType> type() const;
};
class InputPropertyType : public Node { public: using Node::Node;
    std::optional<InputPropertyTypeString> as_InputPropertyTypeString() const;
    std::optional<InputPropertyTypeNumber> as_InputPropertyTypeNumber() const;
    std::optional<InputPropertyTypeBoolean> as_InputPropertyTypeBoolean() const;
    std::optional<InputPropertyTypeArray> as_InputPropertyTypeArray() const;
};
class InputPropertyTypeArray : public Node { public: using Node::Node;
    Presence<StringRoot> items() const;
    Presence<std::string> type() const;
};
class InputPropertyTypeBoolean : public Node { public: using Node::Node;
    Presence<std::string> type() const;
};
class InputPropertyTypeNumber : public Node { public: using Node::Node;
    Presence<std::string> type() const;
};
class InputPropertyTypeString : public Node { public: using Node::Node;
    Presence<std::string> type() const;
};
class Label : public Node { public: using Node::Node;
    Presence<Json> description() const;
    Presence<std::string> name() const;
};
class Meta : public Node { public: using Node::Node;
    Presence<std::string> answered_by() const;
    Presence<std::vector<Attempt>> attempts() const;
    Presence<BatchSetting> batch_setting() const;
    Presence<BatchWarning> batch_warning() const;
    Presence<bool> cached() const;
    Presence<std::string> context_sha256() const;
    Presence<uint64_t> failed_questions() const;
    Presence<std::string> model() const;
    Presence<std::vector<Observation>> observations() const;
    Presence<Origin> origin() const;
    Presence<ProfileWarning> profile_warning() const;
    Presence<std::string> question_sha256() const;
    Presence<std::vector<QuestionSource>> question_sources() const;
    Presence<std::string> questions_sha256() const;
    Presence<std::vector<std::string>> requests() const;
    Presence<uint64_t> requests_sent() const;
    Presence<std::string> tool() const;
    Presence<std::string> url() const;
    Presence<Usage> usage() const;
};
class ObjectRoot : public Node { public: using Node::Node;
    Presence<std::vector<std::pair<std::string, InputPropertyType>>> properties() const;
    Presence<std::vector<std::string>> required() const;
    Presence<ObjectType> type() const;
};
class ObjectType : public Node { public: using Node::Node;
    std::string value() const;
};
class Observation : public Node { public: using Node::Node;
    std::optional<ObservationObservationId> as_ObservationObservationId() const;
    std::optional<ObservationFailureId> as_ObservationFailureId() const;
};
class ObservationId : public Node { public: using Node::Node;
    std::string value() const;
};
class ObservationFailureId : public Node { public: using Node::Node;
    Presence<FailureId> failure_id() const;
};
class ObservationObservationId : public Node { public: using Node::Node;
    Presence<ObservationId> observation_id() const;
};
class Origin : public Node { public: using Node::Node;
    std::string value() const;
};
class PersistenceObservation : public Node { public: using Node::Node;
    Presence<std::string> advice() const;
    Presence<std::string> observed_at() const;
    Presence<UsagePersistence> state() const;
};
class PhysicalSource : public Node { public: using Node::Node;
    Presence<std::string> file() const;
    Presence<uint64_t> first_line() const;
    Presence<uint64_t> last_line() const;
};
class Position : public Node { public: using Node::Node;
    Presence<std::string> file() const;
    Presence<uint64_t> first() const;
    Presence<std::vector<std::string>> images() const;
    Presence<uint64_t> last() const;
};
class QuestionName : public Node { public: using Node::Node;
    std::string value() const;
};
class QuestionSource : public Node { public: using Node::Node;
    Presence<std::string> answered_by() const;
    Presence<uint32_t> batch_size() const;
    Presence<Origin> origin() const;
};
class RankMember : public Node { public: using Node::Node;
    Presence<std::string> name() const;
    Presence<RankMemberResult> result() const;
};
class RankMemberResult : public Node { public: using Node::Node;
    Presence<Answer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::vector<Image>> images() const;
    Presence<Meta> meta() const;
    Presence<ReadableQuestion> question() const;
    Presence<Version> schema() const;
    Presence<PhysicalSource> source() const;
    Presence<std::nullptr_t> threshold() const;
    Presence<uint64_t> value() const;
};
class ReadableQuestion : public Node { public: using Node::Node;
    std::optional<ReadableQuestionDecide> as_ReadableQuestionDecide() const;
    std::optional<ReadableQuestionChoose> as_ReadableQuestionChoose() const;
    std::optional<ReadableQuestionTag> as_ReadableQuestionTag() const;
    std::optional<ReadableQuestionScore> as_ReadableQuestionScore() const;
};
class ReadableQuestion2 : public Node { public: using Node::Node;
    Presence<Batch> batch() const;
    Presence<InputDeclaration> context_schema() const;
    Presence<InputDeclaration> item_schema() const;
    Presence<std::vector<Label>> label_details() const;
    Presence<std::string> model() const;
    Presence<QuestionName> name() const;
    Presence<bool> none() const;
    Presence<std::vector<std::string>> on() const;
    Presence<std::string> profile() const;
    Presence<Json> text() const;
    Presence<std::string> verb() const;
    Presence<WordingVersion> wording_version() const;
};
class ReadableQuestion3 : public Node { public: using Node::Node;
    Presence<Batch> batch() const;
    Presence<InputDeclaration> context_schema() const;
    Presence<Json> entity_definition() const;
    Presence<Json> instructions() const;
    Presence<InputDeclaration> item_schema() const;
    Presence<std::vector<std::pair<std::string, Json>>> kinds() const;
    Presence<std::vector<Label>> label_details() const;
    Presence<RecognitionMode> mode() const;
    Presence<std::string> model() const;
    Presence<QuestionName> name() const;
    Presence<std::vector<std::string>> on() const;
    Presence<std::string> profile() const;
    Presence<Threshold> relation_threshold() const;
    Presence<std::vector<RelationRule>> relations() const;
    Presence<uint32_t> snippet_pieces() const;
    Presence<RecognitionStageContext> stage_context() const;
    Presence<Threshold> threshold() const;
    Presence<Verb> verb() const;
    Presence<WordingVersion> wording_version() const;
};
class ReadableQuestion4 : public Node { public: using Node::Node;
    Presence<Batch> batch() const;
    Presence<InputDeclaration> context_schema() const;
    Presence<RelateFields> fields() const;
    Presence<InputDeclaration> item_schema() const;
    Presence<std::vector<Label>> label_details() const;
    Presence<std::string> model() const;
    Presence<QuestionName> name() const;
    Presence<std::vector<std::string>> on() const;
    Presence<std::string> profile() const;
    Presence<std::vector<RelationRule>> relations() const;
    Presence<Threshold> threshold() const;
    Presence<std::string> verb() const;
    Presence<WordingVersion> wording_version() const;
};
class ReadableQuestionChoose : public Node { public: using Node::Node;
    Presence<Batch> batch() const;
    Presence<InputDeclaration> context_schema() const;
    Presence<InputDeclaration> item_schema() const;
    Presence<std::vector<Label>> label_details() const;
    Presence<std::string> model() const;
    Presence<QuestionName> name() const;
    Presence<std::vector<std::string>> on() const;
    Presence<std::string> profile() const;
    Presence<WordingVersion> wording_version() const;
    Presence<std::vector<std::string>> options() const;
    Presence<Json> text() const;
    Presence<std::string> verb() const;
};
class ReadableQuestionDecide : public Node { public: using Node::Node;
    Presence<Batch> batch() const;
    Presence<InputDeclaration> context_schema() const;
    Presence<InputDeclaration> item_schema() const;
    Presence<std::vector<Label>> label_details() const;
    Presence<std::string> model() const;
    Presence<QuestionName> name() const;
    Presence<std::vector<std::string>> on() const;
    Presence<std::string> profile() const;
    Presence<WordingVersion> wording_version() const;
    Presence<Json> false_() const;
    Presence<Json> text() const;
    Presence<Json> true_() const;
    Presence<std::string> verb() const;
};
class ReadableQuestionScore : public Node { public: using Node::Node;
    Presence<Batch> batch() const;
    Presence<InputDeclaration> context_schema() const;
    Presence<InputDeclaration> item_schema() const;
    Presence<std::vector<Label>> label_details() const;
    Presence<std::string> model() const;
    Presence<QuestionName> name() const;
    Presence<std::vector<std::string>> on() const;
    Presence<std::string> profile() const;
    Presence<WordingVersion> wording_version() const;
    Presence<std::vector<std::string>> levels() const;
    Presence<Json> text() const;
    Presence<std::string> verb() const;
};
class ReadableQuestionTag : public Node { public: using Node::Node;
    Presence<Batch> batch() const;
    Presence<InputDeclaration> context_schema() const;
    Presence<InputDeclaration> item_schema() const;
    Presence<std::vector<Label>> label_details() const;
    Presence<std::string> model() const;
    Presence<QuestionName> name() const;
    Presence<std::vector<std::string>> on() const;
    Presence<std::string> profile() const;
    Presence<WordingVersion> wording_version() const;
    Presence<std::vector<std::string>> labels() const;
    Presence<Json> text() const;
    Presence<std::string> verb() const;
};
class Recognition : public Node { public: using Node::Node;
    Presence<RecognitionOdds> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::string> file() const;
    Presence<uint64_t> first_line() const;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<uint64_t> last_line() const;
    Presence<Meta> meta() const;
    Presence<Position> position() const;
    Presence<ReadableQuestion3> question() const;
    Presence<Version> schema() const;
    Presence<PhysicalSource> source() const;
    Presence<Recognize> value() const;
};
class RecognitionEdgeDocument : public Node { public: using Node::Node;
    Presence<bool> either() const;
    Presence<double> probability() const;
    Presence<std::string> relation() const;
    Presence<Entity> source() const;
    Presence<Entity> target() const;
};
class RecognitionMode : public Node { public: using Node::Node;
    std::string value() const;
};
class RecognitionOdds : public Node { public: using Node::Node;
    std::optional<RecognitionOddsFieldsNamesPairsPiecesProposals> as_RecognitionOddsFieldsNamesPairsPiecesProposals() const;
    std::optional<RecognitionOddsFieldsPiecesProposals> as_RecognitionOddsFieldsPiecesProposals() const;
};
class RecognitionOddsFieldsNamesPairsPiecesProposals : public Node { public: using Node::Node;
    Presence<std::vector<NameOdds>> names() const;
    Presence<std::vector<PairOdds>> pairs() const;
    Presence<std::vector<PieceOdds>> pieces() const;
    Presence<std::vector<RecognitionProposal>> proposals() const;
};
class RecognitionOddsFieldsPiecesProposals : public Node { public: using Node::Node;
    Presence<std::vector<PieceOdds>> pieces() const;
    Presence<std::vector<BoundaryProposal>> proposals() const;
};
class RecognitionProposal : public Node { public: using Node::Node;
    Presence<uint64_t> end() const;
    Presence<bool> kept() const;
    Presence<std::string> kind() const;
    Presence<Place> selected() const;
    Presence<double> span_probability() const;
    Presence<uint64_t> start() const;
    Presence<double> strength() const;
};
class RecognitionStageContext : public Node { public: using Node::Node;
    Presence<std::string> boundary() const;
    Presence<std::string> kind_edge() const;
    Presence<std::string> relation() const;
};
class Relation : public Node { public: using Node::Node;
    Presence<Answers> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<std::string> file() const;
    Presence<uint64_t> first_line() const;
    Presence<uint64_t> index() const;
    Presence<Json> input() const;
    Presence<std::vector<SessionInputSource>> input_sources() const;
    Presence<uint64_t> last_line() const;
    Presence<Meta> meta() const;
    Presence<Position> position() const;
    Presence<ReadableQuestion4> question() const;
    Presence<Version> schema() const;
    Presence<std::vector<RelatedEntityEdge>> value() const;
};
class RelationDirection : public Node { public: using Node::Node;
    std::string value() const;
};
class RelationMember : public Node { public: using Node::Node;
    std::optional<RelationMemberAnswerId> as_RelationMemberAnswerId() const;
    std::optional<RelationMemberFailureId> as_RelationMemberFailureId() const;
};
class RelationMemberAnswerId : public Node { public: using Node::Node;
    Presence<RelationDirection> direction() const;
    Presence<RelationMethod> method() const;
    Presence<std::vector<Observation>> observations() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::vector<QuestionSource>> question_sources() const;
    Presence<std::string> reads() const;
    Presence<std::string> relation() const;
    Presence<std::string> request() const;
    Presence<RelatedEntity> source() const;
    Presence<RelatedEntity> target() const;
    Presence<Threshold> threshold() const;
    Presence<Usage> usage() const;
    Presence<bool> accepted() const;
    Presence<Answer> answer() const;
    Presence<AnswerId> answer_id() const;
    Presence<double> probability() const;
};
class RelationMemberFailureId : public Node { public: using Node::Node;
    Presence<RelationDirection> direction() const;
    Presence<RelationMethod> method() const;
    Presence<std::vector<Observation>> observations() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::vector<QuestionSource>> question_sources() const;
    Presence<std::string> reads() const;
    Presence<std::string> relation() const;
    Presence<std::string> request() const;
    Presence<RelatedEntity> source() const;
    Presence<RelatedEntity> target() const;
    Presence<Threshold> threshold() const;
    Presence<Usage> usage() const;
    Presence<Failure> failure() const;
    Presence<FailureId> failure_id() const;
};
class RelationMethod : public Node { public: using Node::Node;
    std::string value() const;
};
class RequestFunction : public Node { public: using Node::Node;
    std::string value() const;
};
class SdkRequestId : public Node { public: using Node::Node;
    std::string value() const;
};
class SendBudgetDenial : public Node { public: using Node::Node;
    std::optional<SendBudgetDenialBeforeFirstSend> as_SendBudgetDenialBeforeFirstSend() const;
    std::optional<SendBudgetDenialBeforeAdditionalSend> as_SendBudgetDenialBeforeAdditionalSend() const;
    std::optional<SendBudgetDenialBeforeRetry> as_SendBudgetDenialBeforeRetry() const;
};
class SendBudgetDenialBeforeAdditionalSend : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
};
class SendBudgetDenialBeforeFirstSend : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
};
class SendBudgetDenialBeforeRetry : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<uint64_t> last_status() const;
};
class StopCause : public Node { public: using Node::Node;
    std::string value() const;
};
class Stopped : public Node { public: using Node::Node;
    Presence<uint64_t> at() const;
    Presence<StopCause> cause() const;
    Presence<bool> retryable() const;
    Presence<uint64_t> status() const;
};
class StringRoot : public Node { public: using Node::Node;
    Presence<StringType> type() const;
};
class StringType : public Node { public: using Node::Node;
    std::string value() const;
};
class Usage : public Node { public: using Node::Node;
    Presence<uint64_t> input_tokens() const;
    Presence<uint64_t> output_tokens() const;
};
class UsagePersistence : public Node { public: using Node::Node;
    std::string value() const;
};
class Verb : public Node { public: using Node::Node;
    std::string value() const;
};
class Version : public Node { public: using Node::Node;
    std::string value() const;
};
class WordingVersion : public Node { public: using Node::Node;
    uint32_t value() const;
};
class AnnotatedField : public Node { public: using Node::Node;
    std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double, Failed> value() const;
};
class AnnotatedRow : public Node { public: using Node::Node;
    std::vector<std::pair<std::string, AnnotatedField>> value() const;
};
class Answer : public Node { public: using Node::Node;
    std::optional<AnswerYesNo> as_AnswerYesNo() const;
    std::optional<AnswerChoice> as_AnswerChoice() const;
    std::optional<AnswerTag> as_AnswerTag() const;
    std::optional<AnswerScore> as_AnswerScore() const;
};
class AnswerChoice : public Node { public: using Node::Node;
    Presence<double> confidence() const;
    Presence<std::string> kind() const;
    Presence<std::string> pick() const;
    Presence<std::vector<std::pair<std::string, double>>> probabilities() const;
};
class AnswerScore : public Node { public: using Node::Node;
    Presence<double> confidence() const;
    Presence<std::string> kind() const;
    Presence<std::string> level() const;
    Presence<std::vector<std::pair<std::string, double>>> probabilities() const;
};
class AnswerTag : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::vector<std::pair<std::string, double>>> probabilities() const;
};
class AnswerYesNo : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<double> probability() const;
};
class AttemptOutcome : public Node { public: using Node::Node;
    std::string value() const;
};
class BatchSetting : public Node { public: using Node::Node;
    std::variant<uint64_t, std::string> value() const;
};
class BatchWarning : public Node { public: using Node::Node;
    Presence<BatchSetting> running() const;
    Presence<BatchSetting> tuned_for() const;
};
class Entity : public Node { public: using Node::Node;
    Presence<uint64_t> end() const;
    Presence<std::string> file() const;
    Presence<uint64_t> first_line() const;
    Presence<std::string> kind() const;
    Presence<uint64_t> last_line() const;
    Presence<uint64_t> length() const;
    Presence<uint64_t> start() const;
    Presence<double> strength() const;
    Presence<std::string> text() const;
};
class EntityEdge : public Node { public: using Node::Node;
    Presence<bool> either() const;
    Presence<double> probability() const;
    Presence<std::string> relation() const;
    Presence<Entity> source() const;
    Presence<Entity> target() const;
};
class Failed : public Node { public: using Node::Node;
    Presence<Failure> failed() const;
};
class Failure : public Node { public: using Node::Node;
    Presence<FailureCause> cause() const;
    Presence<std::string> kind() const;
};
class FailureCause : public Node { public: using Node::Node;
    std::string value() const;
};
class FailureKind : public Node { public: using Node::Node;
    std::string value() const;
};
class FindAnswer : public Node { public: using Node::Node;
    Presence<double> confidence() const;
    Presence<std::string> kind() const;
    Presence<std::string> pick() const;
    Presence<std::vector<std::pair<std::string, double>>> probabilities() const;
};
class NameOdds : public Node { public: using Node::Node;
    Presence<std::vector<std::pair<std::string, double>>> edges() const;
    Presence<uint64_t> end() const;
    Presence<std::vector<std::pair<std::string, double>>> kinds() const;
    Presence<uint64_t> start() const;
};
class PairOdds : public Node { public: using Node::Node;
    Presence<double> probability() const;
    Presence<std::string> relation() const;
    Presence<Place> source() const;
    Presence<Place> target() const;
};
class PieceOdds : public Node { public: using Node::Node;
    Presence<uint64_t> end() const;
    Presence<uint64_t> start() const;
    Presence<std::vector<std::pair<std::string, double>>> tags() const;
};
class Place : public Node { public: using Node::Node;
    Presence<uint64_t> end() const;
    Presence<uint64_t> start() const;
};
class ProfileWarning : public Node { public: using Node::Node;
    Presence<std::string> running() const;
    Presence<std::string> tuned_for() const;
};
class Recognize : public Node { public: using Node::Node;
    std::optional<RecognizeFieldsEntities> as_RecognizeFieldsEntities() const;
    std::optional<RecognizeFieldsModeProposals> as_RecognizeFieldsModeProposals() const;
};
class RecognizeAnswer : public Node { public: using Node::Node;
    Presence<std::vector<NameOdds>> names() const;
    Presence<std::vector<PairOdds>> pairs() const;
    Presence<std::vector<PieceOdds>> pieces() const;
    Presence<std::vector<RecognitionProposal>> proposals() const;
};
class RecognizeFieldsEntities : public Node { public: using Node::Node;
    Presence<std::vector<Entity>> entities() const;
    Presence<std::vector<EntityEdge>> relations() const;
};
class RecognizeFieldsModeProposals : public Node { public: using Node::Node;
    Presence<BoundaryMode> mode() const;
    Presence<std::vector<BoundaryProposal>> proposals() const;
};
class RelateFields : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::string> name() const;
};
class RelatedEntity : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::string> name() const;
};
class RelatedEntityEdge : public Node { public: using Node::Node;
    Presence<bool> either() const;
    Presence<double> probability() const;
    Presence<std::string> relation() const;
    Presence<RelatedEntityEdgePropertiesSource> source() const;
    Presence<RelatedEntityEdgePropertiesSource> target() const;
};
class RelatedEntityEdgePropertiesSource : public Node { public: using Node::Node;
    std::optional<RelatedEntityEdgePropertiesSourceFieldsKindName> as_RelatedEntityEdgePropertiesSourceFieldsKindName() const;
    std::optional<RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord> as_RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord() const;
};
class RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord : public Node { public: using Node::Node;
    Presence<std::string> file() const;
    Presence<uint64_t> first_line() const;
    Presence<std::string> kind() const;
    Presence<uint64_t> last_line() const;
    Presence<std::string> name() const;
    Presence<uint64_t> ordinal() const;
    Presence<Json> record() const;
};
class RelatedEntityEdgePropertiesSourceFieldsKindName : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::string> name() const;
};
class RelationRule : public Node { public: using Node::Node;
    Presence<bool> either() const;
    Presence<std::string> name() const;
    Presence<std::string> reads() const;
    Presence<bool> single() const;
    Presence<std::string> source() const;
    Presence<std::string> target() const;
};
class SessionAnnotation : public Node { public: using Node::Node;
    Presence<std::string> name() const;
    Presence<AnnotationValue> value() const;
};
class SessionInputSource : public Node { public: using Node::Node;
    Presence<uint64_t> index() const;
    Presence<PhysicalSource> source() const;
};
class SessionJudgment : public Node { public: using Node::Node;
    std::optional<SessionJudgmentDecision> as_SessionJudgmentDecision() const;
    std::optional<SessionJudgmentChoice> as_SessionJudgmentChoice() const;
    std::optional<SessionJudgmentScore> as_SessionJudgmentScore() const;
    std::optional<SessionJudgmentTags> as_SessionJudgmentTags() const;
};
class SessionJudgmentChoice : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::string> value() const;
};
class SessionJudgmentDecision : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<bool> value() const;
};
class SessionJudgmentScore : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<double> value() const;
};
class SessionJudgmentTags : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::vector<std::string>> value() const;
};
class SessionNamedProbability : public Node { public: using Node::Node;
    Presence<std::string> name() const;
    Presence<double> probability() const;
};
class SessionObservation : public Node { public: using Node::Node;
    std::optional<SessionObservationQuestion> as_SessionObservationQuestion() const;
    std::optional<SessionObservationRow> as_SessionObservationRow() const;
};
class SessionObservationQuestion : public Node { public: using Node::Node;
    Presence<SessionQuestionDetail> detail() const;
    Presence<uint64_t> index() const;
    Presence<std::string> kind() const;
    Presence<std::string> member() const;
    Presence<uint64_t> position() const;
    Presence<std::string> stage() const;
};
class SessionObservationRow : public Node { public: using Node::Node;
    Presence<uint64_t> index() const;
    Presence<std::string> kind() const;
    Presence<SessionObservedRow> value() const;
};
class SessionObservedRow : public Node { public: using Node::Node;
    std::optional<SessionObservedRowJudgment> as_SessionObservedRowJudgment() const;
    std::optional<SessionObservedRowAnnotated> as_SessionObservedRowAnnotated() const;
    std::optional<SessionObservedRowRecognized> as_SessionObservedRowRecognized() const;
    std::optional<SessionObservedRowFind> as_SessionObservedRowFind() const;
    std::optional<SessionObservedRowRelations> as_SessionObservedRowRelations() const;
};
class SessionObservedRowAnnotated : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::vector<SessionAnnotation>> value() const;
};
class SessionObservedRowFind : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<uint64_t> value() const;
};
class SessionObservedRowJudgment : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<SessionJudgment> value() const;
};
class SessionObservedRowRecognized : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<SessionRecognition> value() const;
};
class SessionObservedRowRelations : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::vector<SessionRelationEdge>> value() const;
};
class SessionPacket : public Node { public: using Node::Node;
    std::optional<SessionPacketDecideRow> as_SessionPacketDecideRow() const;
    std::optional<SessionPacketChooseRow> as_SessionPacketChooseRow() const;
    std::optional<SessionPacketTagRow> as_SessionPacketTagRow() const;
    std::optional<SessionPacketScoreRow> as_SessionPacketScoreRow() const;
    std::optional<SessionPacketFilterRow> as_SessionPacketFilterRow() const;
    std::optional<SessionPacketAnnotateRow> as_SessionPacketAnnotateRow() const;
    std::optional<SessionPacketDecideAggregate> as_SessionPacketDecideAggregate() const;
    std::optional<SessionPacketChooseAggregate> as_SessionPacketChooseAggregate() const;
    std::optional<SessionPacketTagAggregate> as_SessionPacketTagAggregate() const;
    std::optional<SessionPacketScoreAggregate> as_SessionPacketScoreAggregate() const;
    std::optional<SessionPacketFilterAggregate> as_SessionPacketFilterAggregate() const;
    std::optional<SessionPacketRankAggregate> as_SessionPacketRankAggregate() const;
    std::optional<SessionPacketFindAggregate> as_SessionPacketFindAggregate() const;
    std::optional<SessionPacketAnnotateAggregate> as_SessionPacketAnnotateAggregate() const;
    std::optional<SessionPacketRecognizeAggregate> as_SessionPacketRecognizeAggregate() const;
    std::optional<SessionPacketRelateAggregate> as_SessionPacketRelateAggregate() const;
    std::optional<SessionPacketObservation> as_SessionPacketObservation() const;
    std::optional<SessionPacketTerminal> as_SessionPacketTerminal() const;
};
class SessionPacketAnnotateAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<std::vector<Annotation>> value() const;
};
class SessionPacketAnnotateRow : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<Annotation> value() const;
};
class SessionPacketChooseAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<std::vector<AtomicNullableString>> value() const;
};
class SessionPacketChooseRow : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<AtomicNullableString> value() const;
};
class SessionPacketDecideAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<std::vector<AtomicDecideValue>> value() const;
};
class SessionPacketDecideRow : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<AtomicDecideValue> value() const;
};
class SessionPacketFilterAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<std::vector<AtomicBoolean>> value() const;
};
class SessionPacketFilterRow : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<AtomicBoolean> value() const;
};
class SessionPacketFindAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<Find> value() const;
};
class SessionPacketObservation : public Node { public: using Node::Node;
    Presence<RequestFunction> function() const;
    Presence<std::string> kind() const;
    Presence<SessionObservation> value() const;
};
class SessionPacketRankAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<std::vector<AtomicNonZeroUsize>> value() const;
};
class SessionPacketRecognizeAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<std::vector<Recognition>> value() const;
};
class SessionPacketRelateAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<Relation> value() const;
};
class SessionPacketScoreAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<std::vector<AtomicDouble>> value() const;
};
class SessionPacketScoreRow : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<AtomicDouble> value() const;
};
class SessionPacketTagAggregate : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<std::vector<AtomicArrayOfString>> value() const;
};
class SessionPacketTagRow : public Node { public: using Node::Node;
    Presence<std::string> function() const;
    Presence<std::string> kind() const;
    Presence<AtomicArrayOfString> value() const;
};
class SessionPacketTerminal : public Node { public: using Node::Node;
    Presence<Facts> facts() const;
    Presence<CallError> failure() const;
    Presence<std::string> kind() const;
};
class SessionProbabilities : public Node { public: using Node::Node;
    std::optional<SessionProbabilitiesYesNo> as_SessionProbabilitiesYesNo() const;
    std::optional<SessionProbabilitiesNamed> as_SessionProbabilitiesNamed() const;
};
class SessionProbabilitiesNamed : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<std::vector<SessionNamedProbability>> value() const;
};
class SessionProbabilitiesYesNo : public Node { public: using Node::Node;
    Presence<std::string> kind() const;
    Presence<double> value() const;
};
class SessionQuestionDetail : public Node { public: using Node::Node;
    Presence<AnswerId> answer_id() const;
    Presence<bool> cached() const;
    Presence<double> confidence() const;
    Presence<uint64_t> failed_questions() const;
    Presence<Failure> failure() const;
    Presence<FailureId> failure_id() const;
    Presence<Json> input() const;
    Presence<PhysicalSource> input_source() const;
    Presence<std::vector<SessionInputSource>> input_sources() const;
    Presence<std::vector<Json>> inputs() const;
    Presence<std::string> model() const;
    Presence<std::vector<Observation>> observations() const;
    Presence<SessionProbabilities> probabilities() const;
    Presence<ReadableQuestion> question() const;
    Presence<std::string> question_sha256() const;
    Presence<std::vector<QuestionSource>> question_sources() const;
    Presence<std::string> raw_pick() const;
    Presence<Usage> reported_usage() const;
    Presence<std::vector<std::string>> requests() const;
    Presence<uint64_t> requests_sent() const;
    Presence<Threshold> threshold() const;
    Presence<std::string> url() const;
    Presence<TokenUsage> usage() const;
    Presence<Value> value() const;
};
class SessionRecognition : public Node { public: using Node::Node;
    Presence<std::vector<Entity>> entities() const;
    Presence<RecognitionMode> mode() const;
    Presence<std::vector<BoundaryProposal>> proposals() const;
    Presence<std::vector<RecognitionEdgeDocument>> relations() const;
};
class SessionRelationEdge : public Node { public: using Node::Node;
    Presence<bool> either() const;
    Presence<double> probability() const;
    Presence<std::string> relation() const;
    Presence<EntityDocument> source() const;
    Presence<EntityDocument> target() const;
};
class SourceRelationEndpoint : public Node { public: using Node::Node;
    Presence<std::string> file() const;
    Presence<uint64_t> first_line() const;
    Presence<std::string> kind() const;
    Presence<uint64_t> last_line() const;
    Presence<std::string> name() const;
    Presence<uint64_t> ordinal() const;
    Presence<Json> record() const;
};
class Threshold : public Node { public: using Node::Node;
    std::variant<double, std::string> value() const;
};
class TokenUsage : public Node { public: using Node::Node;
    Presence<uint64_t> input_tokens() const;
    Presence<uint64_t> output_tokens() const;
};
class Value : public Node { public: using Node::Node;
    std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double> value() const;
};
inline Presence<AnswerId> Annotation::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<std::pair<std::string, AnnotationMember>>> Annotation::answers() const { return member_value<std::vector<std::pair<std::string, AnnotationMember>>>(value_, "answers"); }
inline Presence<std::string> Annotation::file() const { return member_value<std::string>(value_, "file"); }
inline Presence<uint64_t> Annotation::first_line() const { return member_value<uint64_t>(value_, "first_line"); }
inline Presence<uint64_t> Annotation::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> Annotation::input() const { return member_value<Json>(value_, "input"); }
inline Presence<uint64_t> Annotation::last_line() const { return member_value<uint64_t>(value_, "last_line"); }
inline Presence<Meta> Annotation::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<Position> Annotation::position() const { return member_value<Position>(value_, "position"); }
inline Presence<Version> Annotation::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<PhysicalSource> Annotation::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<AnnotatedRow> Annotation::value() const { return member_value<AnnotatedRow>(value_, "value"); }
inline std::optional<AnnotationMemberAnswerId> AnnotationMember::as_AnnotationMemberAnswerId() const { if (value_.contains("answer_id")) return AnnotationMemberAnswerId(value_); return std::nullopt; }
inline std::optional<AnnotationMemberFailureId> AnnotationMember::as_AnnotationMemberFailureId() const { if (value_.contains("failure_id")) return AnnotationMemberFailureId(value_); return std::nullopt; }
inline Presence<Answer> AnnotationMemberAnswerId::answer() const { return member_value<Answer>(value_, "answer"); }
inline Presence<AnswerId> AnnotationMemberAnswerId::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<Observation>> AnnotationMemberAnswerId::observations() const { return member_value<std::vector<Observation>>(value_, "observations"); }
inline Presence<ReadableQuestion> AnnotationMemberAnswerId::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::vector<QuestionSource>> AnnotationMemberAnswerId::question_sources() const { return member_value<std::vector<QuestionSource>>(value_, "question_sources"); }
inline Presence<std::string> AnnotationMemberAnswerId::request() const { return member_value<std::string>(value_, "request"); }
inline Presence<Threshold> AnnotationMemberAnswerId::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<Usage> AnnotationMemberAnswerId::usage() const { return member_value<Usage>(value_, "usage"); }
inline Presence<Value> AnnotationMemberAnswerId::value() const { return member_value<Value>(value_, "value"); }
inline Presence<Failure> AnnotationMemberFailureId::failure() const { return member_value<Failure>(value_, "failure"); }
inline Presence<FailureId> AnnotationMemberFailureId::failure_id() const { return member_value<FailureId>(value_, "failure_id"); }
inline Presence<std::vector<Observation>> AnnotationMemberFailureId::observations() const { return member_value<std::vector<Observation>>(value_, "observations"); }
inline Presence<ReadableQuestion> AnnotationMemberFailureId::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::vector<QuestionSource>> AnnotationMemberFailureId::question_sources() const { return member_value<std::vector<QuestionSource>>(value_, "question_sources"); }
inline Presence<std::string> AnnotationMemberFailureId::request() const { return member_value<std::string>(value_, "request"); }
inline Presence<Threshold> AnnotationMemberFailureId::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<Usage> AnnotationMemberFailureId::usage() const { return member_value<Usage>(value_, "usage"); }
inline std::optional<AnnotationValueDecision> AnnotationValue::as_AnnotationValueDecision() const { if (literal(value_, "kind", "decision")) return AnnotationValueDecision(value_); return std::nullopt; }
inline std::optional<AnnotationValueChoice> AnnotationValue::as_AnnotationValueChoice() const { if (literal(value_, "kind", "choice")) return AnnotationValueChoice(value_); return std::nullopt; }
inline std::optional<AnnotationValueScore> AnnotationValue::as_AnnotationValueScore() const { if (literal(value_, "kind", "score")) return AnnotationValueScore(value_); return std::nullopt; }
inline std::optional<AnnotationValueTags> AnnotationValue::as_AnnotationValueTags() const { if (literal(value_, "kind", "tags")) return AnnotationValueTags(value_); return std::nullopt; }
inline std::optional<AnnotationValueFailed> AnnotationValue::as_AnnotationValueFailed() const { if (literal(value_, "kind", "failed")) return AnnotationValueFailed(value_); return std::nullopt; }
inline Presence<std::string> AnnotationValueChoice::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> AnnotationValueChoice::value() const { return member_value<std::string>(value_, "value"); }
inline Presence<std::string> AnnotationValueDecision::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<bool> AnnotationValueDecision::value() const { return member_value<bool>(value_, "value"); }
inline Presence<std::string> AnnotationValueFailed::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<Failure> AnnotationValueFailed::value() const { return member_value<Failure>(value_, "value"); }
inline Presence<std::string> AnnotationValueScore::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<double> AnnotationValueScore::value() const { return member_value<double>(value_, "value"); }
inline Presence<std::string> AnnotationValueTags::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<std::string>> AnnotationValueTags::value() const { return member_value<std::vector<std::string>>(value_, "value"); }
inline std::string AnswerId::value() const { return decode<std::string>(value_); }
inline Presence<std::vector<RelationMember>> Answers::questions() const { return member_value<std::vector<RelationMember>>(value_, "questions"); }
inline Presence<Answer> AtomicArrayOfString::answer() const { return member_value<Answer>(value_, "answer"); }
inline Presence<AnswerId> AtomicArrayOfString::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<Image>> AtomicArrayOfString::images() const { return member_value<std::vector<Image>>(value_, "images"); }
inline Presence<uint64_t> AtomicArrayOfString::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> AtomicArrayOfString::input() const { return member_value<Json>(value_, "input"); }
inline Presence<std::vector<RankMember>> AtomicArrayOfString::members() const { return member_value<std::vector<RankMember>>(value_, "members"); }
inline Presence<Meta> AtomicArrayOfString::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<ReadableQuestion> AtomicArrayOfString::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::string> AtomicArrayOfString::question_name() const { return member_value<std::string>(value_, "question_name"); }
inline Presence<Version> AtomicArrayOfString::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<PhysicalSource> AtomicArrayOfString::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<Threshold> AtomicArrayOfString::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<std::vector<std::string>> AtomicArrayOfString::value() const { return member_value<std::vector<std::string>>(value_, "value"); }
inline Presence<Answer> AtomicDecideValue::answer() const { return member_value<Answer>(value_, "answer"); }
inline Presence<AnswerId> AtomicDecideValue::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<Image>> AtomicDecideValue::images() const { return member_value<std::vector<Image>>(value_, "images"); }
inline Presence<uint64_t> AtomicDecideValue::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> AtomicDecideValue::input() const { return member_value<Json>(value_, "input"); }
inline Presence<std::vector<RankMember>> AtomicDecideValue::members() const { return member_value<std::vector<RankMember>>(value_, "members"); }
inline Presence<Meta> AtomicDecideValue::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<ReadableQuestion> AtomicDecideValue::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::string> AtomicDecideValue::question_name() const { return member_value<std::string>(value_, "question_name"); }
inline Presence<Version> AtomicDecideValue::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<PhysicalSource> AtomicDecideValue::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<Threshold> AtomicDecideValue::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<DecideValue> AtomicDecideValue::value() const { return member_value<DecideValue>(value_, "value"); }
inline Presence<Answer> AtomicNonZeroUsize::answer() const { return member_value<Answer>(value_, "answer"); }
inline Presence<AnswerId> AtomicNonZeroUsize::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<Image>> AtomicNonZeroUsize::images() const { return member_value<std::vector<Image>>(value_, "images"); }
inline Presence<uint64_t> AtomicNonZeroUsize::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> AtomicNonZeroUsize::input() const { return member_value<Json>(value_, "input"); }
inline Presence<std::vector<RankMember>> AtomicNonZeroUsize::members() const { return member_value<std::vector<RankMember>>(value_, "members"); }
inline Presence<Meta> AtomicNonZeroUsize::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<ReadableQuestion> AtomicNonZeroUsize::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::string> AtomicNonZeroUsize::question_name() const { return member_value<std::string>(value_, "question_name"); }
inline Presence<Version> AtomicNonZeroUsize::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<PhysicalSource> AtomicNonZeroUsize::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<Threshold> AtomicNonZeroUsize::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<uint64_t> AtomicNonZeroUsize::value() const { return member_value<uint64_t>(value_, "value"); }
inline Presence<Answer> AtomicNullableString::answer() const { return member_value<Answer>(value_, "answer"); }
inline Presence<AnswerId> AtomicNullableString::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<Image>> AtomicNullableString::images() const { return member_value<std::vector<Image>>(value_, "images"); }
inline Presence<uint64_t> AtomicNullableString::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> AtomicNullableString::input() const { return member_value<Json>(value_, "input"); }
inline Presence<std::vector<RankMember>> AtomicNullableString::members() const { return member_value<std::vector<RankMember>>(value_, "members"); }
inline Presence<Meta> AtomicNullableString::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<ReadableQuestion> AtomicNullableString::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::string> AtomicNullableString::question_name() const { return member_value<std::string>(value_, "question_name"); }
inline Presence<Version> AtomicNullableString::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<PhysicalSource> AtomicNullableString::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<Threshold> AtomicNullableString::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<std::string> AtomicNullableString::value() const { return member_value<std::string>(value_, "value"); }
inline Presence<Answer> AtomicBoolean::answer() const { return member_value<Answer>(value_, "answer"); }
inline Presence<AnswerId> AtomicBoolean::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<Image>> AtomicBoolean::images() const { return member_value<std::vector<Image>>(value_, "images"); }
inline Presence<uint64_t> AtomicBoolean::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> AtomicBoolean::input() const { return member_value<Json>(value_, "input"); }
inline Presence<std::vector<RankMember>> AtomicBoolean::members() const { return member_value<std::vector<RankMember>>(value_, "members"); }
inline Presence<Meta> AtomicBoolean::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<ReadableQuestion> AtomicBoolean::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::string> AtomicBoolean::question_name() const { return member_value<std::string>(value_, "question_name"); }
inline Presence<Version> AtomicBoolean::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<PhysicalSource> AtomicBoolean::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<Threshold> AtomicBoolean::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<bool> AtomicBoolean::value() const { return member_value<bool>(value_, "value"); }
inline Presence<Answer> AtomicDouble::answer() const { return member_value<Answer>(value_, "answer"); }
inline Presence<AnswerId> AtomicDouble::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<Image>> AtomicDouble::images() const { return member_value<std::vector<Image>>(value_, "images"); }
inline Presence<uint64_t> AtomicDouble::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> AtomicDouble::input() const { return member_value<Json>(value_, "input"); }
inline Presence<std::vector<RankMember>> AtomicDouble::members() const { return member_value<std::vector<RankMember>>(value_, "members"); }
inline Presence<Meta> AtomicDouble::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<ReadableQuestion> AtomicDouble::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::string> AtomicDouble::question_name() const { return member_value<std::string>(value_, "question_name"); }
inline Presence<Version> AtomicDouble::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<PhysicalSource> AtomicDouble::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<Threshold> AtomicDouble::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<double> AtomicDouble::value() const { return member_value<double>(value_, "value"); }
inline Presence<uint64_t> Attempt::ordinal() const { return member_value<uint64_t>(value_, "ordinal"); }
inline Presence<AttemptOutcome> Attempt::outcome() const { return member_value<AttemptOutcome>(value_, "outcome"); }
inline Presence<std::string> Attempt::request_id() const { return member_value<std::string>(value_, "request_id"); }
inline Presence<std::string> Attempt::request_sha256() const { return member_value<std::string>(value_, "request_sha256"); }
inline Presence<SdkRequestId> Attempt::sdk_request_id() const { return member_value<SdkRequestId>(value_, "sdk_request_id"); }
inline Presence<uint64_t> Attempt::server_ms() const { return member_value<uint64_t>(value_, "server_ms"); }
inline Presence<uint64_t> Attempt::status() const { return member_value<uint64_t>(value_, "status"); }
inline Presence<uint64_t> Attempt::wall_ms() const { return member_value<uint64_t>(value_, "wall_ms"); }
inline std::variant<uint64_t, std::string> Batch::value() const { return decode<std::variant<uint64_t, std::string>>(value_); }
inline std::string BoundaryMode::value() const { return decode<std::string>(value_); }
inline Presence<std::vector<PieceOdds>> BoundaryOdds::pieces() const { return member_value<std::vector<PieceOdds>>(value_, "pieces"); }
inline Presence<std::vector<BoundaryProposal>> BoundaryOdds::proposals() const { return member_value<std::vector<BoundaryProposal>>(value_, "proposals"); }
inline Presence<uint64_t> BoundaryProposal::end() const { return member_value<uint64_t>(value_, "end"); }
inline Presence<uint64_t> BoundaryProposal::length() const { return member_value<uint64_t>(value_, "length"); }
inline Presence<double> BoundaryProposal::probability() const { return member_value<double>(value_, "probability"); }
inline Presence<uint64_t> BoundaryProposal::start() const { return member_value<uint64_t>(value_, "start"); }
inline Presence<std::string> BoundaryProposal::text() const { return member_value<std::string>(value_, "text"); }
inline Presence<Error> CallError::error() const { return member_value<Error>(value_, "error"); }
inline Presence<Facts> CallError::facts() const { return member_value<Facts>(value_, "facts"); }
inline std::string CallId::value() const { return decode<std::string>(value_); }
inline Json DecideValue::value() const { return decode<Json>(value_); }
inline Presence<std::string> EntityDocument::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> EntityDocument::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<EstimatedInputDenial> Error::estimated_input_denial() const { return member_value<EstimatedInputDenial>(value_, "estimated_input_denial"); }
inline Presence<FailureKind> Error::kind() const { return member_value<FailureKind>(value_, "kind"); }
inline Presence<std::string> Error::message() const { return member_value<std::string>(value_, "message"); }
inline Presence<bool> Error::retryable() const { return member_value<bool>(value_, "retryable"); }
inline Presence<SendBudgetDenial> Error::send_budget_denial() const { return member_value<SendBudgetDenial>(value_, "send_budget_denial"); }
inline Presence<Stopped> Error::stopped() const { return member_value<Stopped>(value_, "stopped"); }
inline std::optional<EstimatedInputDenialInitialRequest> EstimatedInputDenial::as_EstimatedInputDenialInitialRequest() const { if (literal(value_, "kind", "initial_request")) return EstimatedInputDenialInitialRequest(value_); return std::nullopt; }
inline std::optional<EstimatedInputDenialAdditionalRequest> EstimatedInputDenial::as_EstimatedInputDenialAdditionalRequest() const { if (literal(value_, "kind", "additional_request")) return EstimatedInputDenialAdditionalRequest(value_); return std::nullopt; }
inline std::optional<EstimatedInputDenialRetry> EstimatedInputDenial::as_EstimatedInputDenialRetry() const { if (literal(value_, "kind", "retry")) return EstimatedInputDenialRetry(value_); return std::nullopt; }
inline Presence<std::string> EstimatedInputDenialAdditionalRequest::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<uint64_t> EstimatedInputDenialAdditionalRequest::limit() const { return member_value<uint64_t>(value_, "limit"); }
inline Presence<std::string> EstimatedInputDenialInitialRequest::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<uint64_t> EstimatedInputDenialInitialRequest::limit() const { return member_value<uint64_t>(value_, "limit"); }
inline Presence<std::string> EstimatedInputDenialRetry::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<uint64_t> EstimatedInputDenialRetry::last_status() const { return member_value<uint64_t>(value_, "last_status"); }
inline Presence<uint64_t> EstimatedInputDenialRetry::limit() const { return member_value<uint64_t>(value_, "limit"); }
inline Presence<std::vector<Attempt>> Facts::attempts() const { return member_value<std::vector<Attempt>>(value_, "attempts"); }
inline Presence<uint64_t> Facts::cache_answers() const { return member_value<uint64_t>(value_, "cache_answers"); }
inline Presence<CallId> Facts::call_id() const { return member_value<CallId>(value_, "call_id"); }
inline Presence<std::string> Facts::estimated_cost_usd() const { return member_value<std::string>(value_, "estimated_cost_usd"); }
inline Presence<bool> Facts::held_model_mismatch() const { return member_value<bool>(value_, "held_model_mismatch"); }
inline Presence<uint64_t> Facts::input_tokens() const { return member_value<uint64_t>(value_, "input_tokens"); }
inline Presence<uint64_t> Facts::largest_request_bytes() const { return member_value<uint64_t>(value_, "largest_request_bytes"); }
inline Presence<uint64_t> Facts::largest_request_estimated_input_tokens() const { return member_value<uint64_t>(value_, "largest_request_estimated_input_tokens"); }
inline Presence<std::string> Facts::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<uint64_t> Facts::output_tokens() const { return member_value<uint64_t>(value_, "output_tokens"); }
inline Presence<uint64_t> Facts::records() const { return member_value<uint64_t>(value_, "records"); }
inline Presence<uint64_t> Facts::requests_sent() const { return member_value<uint64_t>(value_, "requests_sent"); }
inline Presence<double> Facts::seconds() const { return member_value<double>(value_, "seconds"); }
inline Presence<std::string> Facts::token_estimate_method() const { return member_value<std::string>(value_, "token_estimate_method"); }
inline Presence<PersistenceObservation> Facts::usage_persistence() const { return member_value<PersistenceObservation>(value_, "usage_persistence"); }
inline std::string FailureId::value() const { return decode<std::string>(value_); }
inline Presence<FindAnswer> Find::answer() const { return member_value<FindAnswer>(value_, "answer"); }
inline Presence<AnswerId> Find::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<FindCandidate>> Find::candidates() const { return member_value<std::vector<FindCandidate>>(value_, "candidates"); }
inline Presence<std::string> Find::file() const { return member_value<std::string>(value_, "file"); }
inline Presence<uint64_t> Find::first_line() const { return member_value<uint64_t>(value_, "first_line"); }
inline Presence<uint64_t> Find::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<uint64_t> Find::last_line() const { return member_value<uint64_t>(value_, "last_line"); }
inline Presence<Meta> Find::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<Position> Find::position() const { return member_value<Position>(value_, "position"); }
inline Presence<ReadableQuestion2> Find::question() const { return member_value<ReadableQuestion2>(value_, "question"); }
inline Presence<Version> Find::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<std::nullptr_t> Find::threshold() const { return member_value<std::nullptr_t>(value_, "threshold"); }
inline Presence<Json> Find::value() const { return member_value<Json>(value_, "value"); }
inline Presence<uint64_t> FindCandidate::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> FindCandidate::input() const { return member_value<Json>(value_, "input"); }
inline Presence<double> FindCandidate::probability() const { return member_value<double>(value_, "probability"); }
inline Presence<PhysicalSource> FindCandidate::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<std::string> Image::base64() const { return member_value<std::string>(value_, "base64"); }
inline Presence<uint32_t> Image::height() const { return member_value<uint32_t>(value_, "height"); }
inline Presence<ImageMedia> Image::media() const { return member_value<ImageMedia>(value_, "media"); }
inline Presence<uint32_t> Image::width() const { return member_value<uint32_t>(value_, "width"); }
inline std::string ImageMedia::value() const { return decode<std::string>(value_); }
inline std::optional<InputDeclarationString> InputDeclaration::as_InputDeclarationString() const { if (literal(value_, "type", "string")) return InputDeclarationString(value_); return std::nullopt; }
inline std::optional<InputDeclarationObject> InputDeclaration::as_InputDeclarationObject() const { if (literal(value_, "type", "object")) return InputDeclarationObject(value_); return std::nullopt; }
inline Presence<std::vector<std::pair<std::string, InputPropertyType>>> InputDeclarationObject::properties() const { return member_value<std::vector<std::pair<std::string, InputPropertyType>>>(value_, "properties"); }
inline Presence<std::vector<std::string>> InputDeclarationObject::required() const { return member_value<std::vector<std::string>>(value_, "required"); }
inline Presence<ObjectType> InputDeclarationObject::type() const { return member_value<ObjectType>(value_, "type"); }
inline Presence<StringType> InputDeclarationString::type() const { return member_value<StringType>(value_, "type"); }
inline std::optional<InputPropertyTypeString> InputPropertyType::as_InputPropertyTypeString() const { if (literal(value_, "type", "string")) return InputPropertyTypeString(value_); return std::nullopt; }
inline std::optional<InputPropertyTypeNumber> InputPropertyType::as_InputPropertyTypeNumber() const { if (literal(value_, "type", "number")) return InputPropertyTypeNumber(value_); return std::nullopt; }
inline std::optional<InputPropertyTypeBoolean> InputPropertyType::as_InputPropertyTypeBoolean() const { if (literal(value_, "type", "boolean")) return InputPropertyTypeBoolean(value_); return std::nullopt; }
inline std::optional<InputPropertyTypeArray> InputPropertyType::as_InputPropertyTypeArray() const { if (literal(value_, "type", "array")) return InputPropertyTypeArray(value_); return std::nullopt; }
inline Presence<StringRoot> InputPropertyTypeArray::items() const { return member_value<StringRoot>(value_, "items"); }
inline Presence<std::string> InputPropertyTypeArray::type() const { return member_value<std::string>(value_, "type"); }
inline Presence<std::string> InputPropertyTypeBoolean::type() const { return member_value<std::string>(value_, "type"); }
inline Presence<std::string> InputPropertyTypeNumber::type() const { return member_value<std::string>(value_, "type"); }
inline Presence<std::string> InputPropertyTypeString::type() const { return member_value<std::string>(value_, "type"); }
inline Presence<Json> Label::description() const { return member_value<Json>(value_, "description"); }
inline Presence<std::string> Label::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<std::string> Meta::answered_by() const { return member_value<std::string>(value_, "answered_by"); }
inline Presence<std::vector<Attempt>> Meta::attempts() const { return member_value<std::vector<Attempt>>(value_, "attempts"); }
inline Presence<BatchSetting> Meta::batch_setting() const { return member_value<BatchSetting>(value_, "batch_setting"); }
inline Presence<BatchWarning> Meta::batch_warning() const { return member_value<BatchWarning>(value_, "batch_warning"); }
inline Presence<bool> Meta::cached() const { return member_value<bool>(value_, "cached"); }
inline Presence<std::string> Meta::context_sha256() const { return member_value<std::string>(value_, "context_sha256"); }
inline Presence<uint64_t> Meta::failed_questions() const { return member_value<uint64_t>(value_, "failed_questions"); }
inline Presence<std::string> Meta::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<std::vector<Observation>> Meta::observations() const { return member_value<std::vector<Observation>>(value_, "observations"); }
inline Presence<Origin> Meta::origin() const { return member_value<Origin>(value_, "origin"); }
inline Presence<ProfileWarning> Meta::profile_warning() const { return member_value<ProfileWarning>(value_, "profile_warning"); }
inline Presence<std::string> Meta::question_sha256() const { return member_value<std::string>(value_, "question_sha256"); }
inline Presence<std::vector<QuestionSource>> Meta::question_sources() const { return member_value<std::vector<QuestionSource>>(value_, "question_sources"); }
inline Presence<std::string> Meta::questions_sha256() const { return member_value<std::string>(value_, "questions_sha256"); }
inline Presence<std::vector<std::string>> Meta::requests() const { return member_value<std::vector<std::string>>(value_, "requests"); }
inline Presence<uint64_t> Meta::requests_sent() const { return member_value<uint64_t>(value_, "requests_sent"); }
inline Presence<std::string> Meta::tool() const { return member_value<std::string>(value_, "tool"); }
inline Presence<std::string> Meta::url() const { return member_value<std::string>(value_, "url"); }
inline Presence<Usage> Meta::usage() const { return member_value<Usage>(value_, "usage"); }
inline Presence<std::vector<std::pair<std::string, InputPropertyType>>> ObjectRoot::properties() const { return member_value<std::vector<std::pair<std::string, InputPropertyType>>>(value_, "properties"); }
inline Presence<std::vector<std::string>> ObjectRoot::required() const { return member_value<std::vector<std::string>>(value_, "required"); }
inline Presence<ObjectType> ObjectRoot::type() const { return member_value<ObjectType>(value_, "type"); }
inline std::string ObjectType::value() const { return decode<std::string>(value_); }
inline std::optional<ObservationObservationId> Observation::as_ObservationObservationId() const { if (value_.contains("observation_id")) return ObservationObservationId(value_); return std::nullopt; }
inline std::optional<ObservationFailureId> Observation::as_ObservationFailureId() const { if (value_.contains("failure_id")) return ObservationFailureId(value_); return std::nullopt; }
inline std::string ObservationId::value() const { return decode<std::string>(value_); }
inline Presence<FailureId> ObservationFailureId::failure_id() const { return member_value<FailureId>(value_, "failure_id"); }
inline Presence<ObservationId> ObservationObservationId::observation_id() const { return member_value<ObservationId>(value_, "observation_id"); }
inline std::string Origin::value() const { return decode<std::string>(value_); }
inline Presence<std::string> PersistenceObservation::advice() const { return member_value<std::string>(value_, "advice"); }
inline Presence<std::string> PersistenceObservation::observed_at() const { return member_value<std::string>(value_, "observed_at"); }
inline Presence<UsagePersistence> PersistenceObservation::state() const { return member_value<UsagePersistence>(value_, "state"); }
inline Presence<std::string> PhysicalSource::file() const { return member_value<std::string>(value_, "file"); }
inline Presence<uint64_t> PhysicalSource::first_line() const { return member_value<uint64_t>(value_, "first_line"); }
inline Presence<uint64_t> PhysicalSource::last_line() const { return member_value<uint64_t>(value_, "last_line"); }
inline Presence<std::string> Position::file() const { return member_value<std::string>(value_, "file"); }
inline Presence<uint64_t> Position::first() const { return member_value<uint64_t>(value_, "first"); }
inline Presence<std::vector<std::string>> Position::images() const { return member_value<std::vector<std::string>>(value_, "images"); }
inline Presence<uint64_t> Position::last() const { return member_value<uint64_t>(value_, "last"); }
inline std::string QuestionName::value() const { return decode<std::string>(value_); }
inline Presence<std::string> QuestionSource::answered_by() const { return member_value<std::string>(value_, "answered_by"); }
inline Presence<uint32_t> QuestionSource::batch_size() const { return member_value<uint32_t>(value_, "batch_size"); }
inline Presence<Origin> QuestionSource::origin() const { return member_value<Origin>(value_, "origin"); }
inline Presence<std::string> RankMember::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<RankMemberResult> RankMember::result() const { return member_value<RankMemberResult>(value_, "result"); }
inline Presence<Answer> RankMemberResult::answer() const { return member_value<Answer>(value_, "answer"); }
inline Presence<AnswerId> RankMemberResult::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::vector<Image>> RankMemberResult::images() const { return member_value<std::vector<Image>>(value_, "images"); }
inline Presence<Meta> RankMemberResult::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<ReadableQuestion> RankMemberResult::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<Version> RankMemberResult::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<PhysicalSource> RankMemberResult::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<std::nullptr_t> RankMemberResult::threshold() const { return member_value<std::nullptr_t>(value_, "threshold"); }
inline Presence<uint64_t> RankMemberResult::value() const { return member_value<uint64_t>(value_, "value"); }
inline std::optional<ReadableQuestionDecide> ReadableQuestion::as_ReadableQuestionDecide() const { if (literal(value_, "verb", "decide")) return ReadableQuestionDecide(value_); return std::nullopt; }
inline std::optional<ReadableQuestionChoose> ReadableQuestion::as_ReadableQuestionChoose() const { if (literal(value_, "verb", "choose")) return ReadableQuestionChoose(value_); return std::nullopt; }
inline std::optional<ReadableQuestionTag> ReadableQuestion::as_ReadableQuestionTag() const { if (literal(value_, "verb", "tag")) return ReadableQuestionTag(value_); return std::nullopt; }
inline std::optional<ReadableQuestionScore> ReadableQuestion::as_ReadableQuestionScore() const { if (literal(value_, "verb", "score")) return ReadableQuestionScore(value_); return std::nullopt; }
inline Presence<Batch> ReadableQuestion2::batch() const { return member_value<Batch>(value_, "batch"); }
inline Presence<InputDeclaration> ReadableQuestion2::context_schema() const { return member_value<InputDeclaration>(value_, "context_schema"); }
inline Presence<InputDeclaration> ReadableQuestion2::item_schema() const { return member_value<InputDeclaration>(value_, "item_schema"); }
inline Presence<std::vector<Label>> ReadableQuestion2::label_details() const { return member_value<std::vector<Label>>(value_, "label_details"); }
inline Presence<std::string> ReadableQuestion2::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<QuestionName> ReadableQuestion2::name() const { return member_value<QuestionName>(value_, "name"); }
inline Presence<bool> ReadableQuestion2::none() const { return member_value<bool>(value_, "none"); }
inline Presence<std::vector<std::string>> ReadableQuestion2::on() const { return member_value<std::vector<std::string>>(value_, "on"); }
inline Presence<std::string> ReadableQuestion2::profile() const { return member_value<std::string>(value_, "profile"); }
inline Presence<Json> ReadableQuestion2::text() const { return member_value<Json>(value_, "text"); }
inline Presence<std::string> ReadableQuestion2::verb() const { return member_value<std::string>(value_, "verb"); }
inline Presence<WordingVersion> ReadableQuestion2::wording_version() const { return member_value<WordingVersion>(value_, "wording_version"); }
inline Presence<Batch> ReadableQuestion3::batch() const { return member_value<Batch>(value_, "batch"); }
inline Presence<InputDeclaration> ReadableQuestion3::context_schema() const { return member_value<InputDeclaration>(value_, "context_schema"); }
inline Presence<Json> ReadableQuestion3::entity_definition() const { return member_value<Json>(value_, "entity_definition"); }
inline Presence<Json> ReadableQuestion3::instructions() const { return member_value<Json>(value_, "instructions"); }
inline Presence<InputDeclaration> ReadableQuestion3::item_schema() const { return member_value<InputDeclaration>(value_, "item_schema"); }
inline Presence<std::vector<std::pair<std::string, Json>>> ReadableQuestion3::kinds() const { return member_value<std::vector<std::pair<std::string, Json>>>(value_, "kinds"); }
inline Presence<std::vector<Label>> ReadableQuestion3::label_details() const { return member_value<std::vector<Label>>(value_, "label_details"); }
inline Presence<RecognitionMode> ReadableQuestion3::mode() const { return member_value<RecognitionMode>(value_, "mode"); }
inline Presence<std::string> ReadableQuestion3::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<QuestionName> ReadableQuestion3::name() const { return member_value<QuestionName>(value_, "name"); }
inline Presence<std::vector<std::string>> ReadableQuestion3::on() const { return member_value<std::vector<std::string>>(value_, "on"); }
inline Presence<std::string> ReadableQuestion3::profile() const { return member_value<std::string>(value_, "profile"); }
inline Presence<Threshold> ReadableQuestion3::relation_threshold() const { return member_value<Threshold>(value_, "relation_threshold"); }
inline Presence<std::vector<RelationRule>> ReadableQuestion3::relations() const { return member_value<std::vector<RelationRule>>(value_, "relations"); }
inline Presence<uint32_t> ReadableQuestion3::snippet_pieces() const { return member_value<uint32_t>(value_, "snippet_pieces"); }
inline Presence<RecognitionStageContext> ReadableQuestion3::stage_context() const { return member_value<RecognitionStageContext>(value_, "stage_context"); }
inline Presence<Threshold> ReadableQuestion3::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<Verb> ReadableQuestion3::verb() const { return member_value<Verb>(value_, "verb"); }
inline Presence<WordingVersion> ReadableQuestion3::wording_version() const { return member_value<WordingVersion>(value_, "wording_version"); }
inline Presence<Batch> ReadableQuestion4::batch() const { return member_value<Batch>(value_, "batch"); }
inline Presence<InputDeclaration> ReadableQuestion4::context_schema() const { return member_value<InputDeclaration>(value_, "context_schema"); }
inline Presence<RelateFields> ReadableQuestion4::fields() const { return member_value<RelateFields>(value_, "fields"); }
inline Presence<InputDeclaration> ReadableQuestion4::item_schema() const { return member_value<InputDeclaration>(value_, "item_schema"); }
inline Presence<std::vector<Label>> ReadableQuestion4::label_details() const { return member_value<std::vector<Label>>(value_, "label_details"); }
inline Presence<std::string> ReadableQuestion4::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<QuestionName> ReadableQuestion4::name() const { return member_value<QuestionName>(value_, "name"); }
inline Presence<std::vector<std::string>> ReadableQuestion4::on() const { return member_value<std::vector<std::string>>(value_, "on"); }
inline Presence<std::string> ReadableQuestion4::profile() const { return member_value<std::string>(value_, "profile"); }
inline Presence<std::vector<RelationRule>> ReadableQuestion4::relations() const { return member_value<std::vector<RelationRule>>(value_, "relations"); }
inline Presence<Threshold> ReadableQuestion4::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<std::string> ReadableQuestion4::verb() const { return member_value<std::string>(value_, "verb"); }
inline Presence<WordingVersion> ReadableQuestion4::wording_version() const { return member_value<WordingVersion>(value_, "wording_version"); }
inline Presence<Batch> ReadableQuestionChoose::batch() const { return member_value<Batch>(value_, "batch"); }
inline Presence<InputDeclaration> ReadableQuestionChoose::context_schema() const { return member_value<InputDeclaration>(value_, "context_schema"); }
inline Presence<InputDeclaration> ReadableQuestionChoose::item_schema() const { return member_value<InputDeclaration>(value_, "item_schema"); }
inline Presence<std::vector<Label>> ReadableQuestionChoose::label_details() const { return member_value<std::vector<Label>>(value_, "label_details"); }
inline Presence<std::string> ReadableQuestionChoose::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<QuestionName> ReadableQuestionChoose::name() const { return member_value<QuestionName>(value_, "name"); }
inline Presence<std::vector<std::string>> ReadableQuestionChoose::on() const { return member_value<std::vector<std::string>>(value_, "on"); }
inline Presence<std::string> ReadableQuestionChoose::profile() const { return member_value<std::string>(value_, "profile"); }
inline Presence<WordingVersion> ReadableQuestionChoose::wording_version() const { return member_value<WordingVersion>(value_, "wording_version"); }
inline Presence<std::vector<std::string>> ReadableQuestionChoose::options() const { return member_value<std::vector<std::string>>(value_, "options"); }
inline Presence<Json> ReadableQuestionChoose::text() const { return member_value<Json>(value_, "text"); }
inline Presence<std::string> ReadableQuestionChoose::verb() const { return member_value<std::string>(value_, "verb"); }
inline Presence<Batch> ReadableQuestionDecide::batch() const { return member_value<Batch>(value_, "batch"); }
inline Presence<InputDeclaration> ReadableQuestionDecide::context_schema() const { return member_value<InputDeclaration>(value_, "context_schema"); }
inline Presence<InputDeclaration> ReadableQuestionDecide::item_schema() const { return member_value<InputDeclaration>(value_, "item_schema"); }
inline Presence<std::vector<Label>> ReadableQuestionDecide::label_details() const { return member_value<std::vector<Label>>(value_, "label_details"); }
inline Presence<std::string> ReadableQuestionDecide::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<QuestionName> ReadableQuestionDecide::name() const { return member_value<QuestionName>(value_, "name"); }
inline Presence<std::vector<std::string>> ReadableQuestionDecide::on() const { return member_value<std::vector<std::string>>(value_, "on"); }
inline Presence<std::string> ReadableQuestionDecide::profile() const { return member_value<std::string>(value_, "profile"); }
inline Presence<WordingVersion> ReadableQuestionDecide::wording_version() const { return member_value<WordingVersion>(value_, "wording_version"); }
inline Presence<Json> ReadableQuestionDecide::false_() const { return member_value<Json>(value_, "false"); }
inline Presence<Json> ReadableQuestionDecide::text() const { return member_value<Json>(value_, "text"); }
inline Presence<Json> ReadableQuestionDecide::true_() const { return member_value<Json>(value_, "true"); }
inline Presence<std::string> ReadableQuestionDecide::verb() const { return member_value<std::string>(value_, "verb"); }
inline Presence<Batch> ReadableQuestionScore::batch() const { return member_value<Batch>(value_, "batch"); }
inline Presence<InputDeclaration> ReadableQuestionScore::context_schema() const { return member_value<InputDeclaration>(value_, "context_schema"); }
inline Presence<InputDeclaration> ReadableQuestionScore::item_schema() const { return member_value<InputDeclaration>(value_, "item_schema"); }
inline Presence<std::vector<Label>> ReadableQuestionScore::label_details() const { return member_value<std::vector<Label>>(value_, "label_details"); }
inline Presence<std::string> ReadableQuestionScore::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<QuestionName> ReadableQuestionScore::name() const { return member_value<QuestionName>(value_, "name"); }
inline Presence<std::vector<std::string>> ReadableQuestionScore::on() const { return member_value<std::vector<std::string>>(value_, "on"); }
inline Presence<std::string> ReadableQuestionScore::profile() const { return member_value<std::string>(value_, "profile"); }
inline Presence<WordingVersion> ReadableQuestionScore::wording_version() const { return member_value<WordingVersion>(value_, "wording_version"); }
inline Presence<std::vector<std::string>> ReadableQuestionScore::levels() const { return member_value<std::vector<std::string>>(value_, "levels"); }
inline Presence<Json> ReadableQuestionScore::text() const { return member_value<Json>(value_, "text"); }
inline Presence<std::string> ReadableQuestionScore::verb() const { return member_value<std::string>(value_, "verb"); }
inline Presence<Batch> ReadableQuestionTag::batch() const { return member_value<Batch>(value_, "batch"); }
inline Presence<InputDeclaration> ReadableQuestionTag::context_schema() const { return member_value<InputDeclaration>(value_, "context_schema"); }
inline Presence<InputDeclaration> ReadableQuestionTag::item_schema() const { return member_value<InputDeclaration>(value_, "item_schema"); }
inline Presence<std::vector<Label>> ReadableQuestionTag::label_details() const { return member_value<std::vector<Label>>(value_, "label_details"); }
inline Presence<std::string> ReadableQuestionTag::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<QuestionName> ReadableQuestionTag::name() const { return member_value<QuestionName>(value_, "name"); }
inline Presence<std::vector<std::string>> ReadableQuestionTag::on() const { return member_value<std::vector<std::string>>(value_, "on"); }
inline Presence<std::string> ReadableQuestionTag::profile() const { return member_value<std::string>(value_, "profile"); }
inline Presence<WordingVersion> ReadableQuestionTag::wording_version() const { return member_value<WordingVersion>(value_, "wording_version"); }
inline Presence<std::vector<std::string>> ReadableQuestionTag::labels() const { return member_value<std::vector<std::string>>(value_, "labels"); }
inline Presence<Json> ReadableQuestionTag::text() const { return member_value<Json>(value_, "text"); }
inline Presence<std::string> ReadableQuestionTag::verb() const { return member_value<std::string>(value_, "verb"); }
inline Presence<RecognitionOdds> Recognition::answer() const { return member_value<RecognitionOdds>(value_, "answer"); }
inline Presence<AnswerId> Recognition::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::string> Recognition::file() const { return member_value<std::string>(value_, "file"); }
inline Presence<uint64_t> Recognition::first_line() const { return member_value<uint64_t>(value_, "first_line"); }
inline Presence<uint64_t> Recognition::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> Recognition::input() const { return member_value<Json>(value_, "input"); }
inline Presence<uint64_t> Recognition::last_line() const { return member_value<uint64_t>(value_, "last_line"); }
inline Presence<Meta> Recognition::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<Position> Recognition::position() const { return member_value<Position>(value_, "position"); }
inline Presence<ReadableQuestion3> Recognition::question() const { return member_value<ReadableQuestion3>(value_, "question"); }
inline Presence<Version> Recognition::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<PhysicalSource> Recognition::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline Presence<Recognize> Recognition::value() const { return member_value<Recognize>(value_, "value"); }
inline Presence<bool> RecognitionEdgeDocument::either() const { return member_value<bool>(value_, "either"); }
inline Presence<double> RecognitionEdgeDocument::probability() const { return member_value<double>(value_, "probability"); }
inline Presence<std::string> RecognitionEdgeDocument::relation() const { return member_value<std::string>(value_, "relation"); }
inline Presence<Entity> RecognitionEdgeDocument::source() const { return member_value<Entity>(value_, "source"); }
inline Presence<Entity> RecognitionEdgeDocument::target() const { return member_value<Entity>(value_, "target"); }
inline std::string RecognitionMode::value() const { return decode<std::string>(value_); }
inline std::optional<RecognitionOddsFieldsNamesPairsPiecesProposals> RecognitionOdds::as_RecognitionOddsFieldsNamesPairsPiecesProposals() const { if (value_.contains("names") && value_.contains("pairs") && value_.contains("pieces") && value_.contains("proposals")) return RecognitionOddsFieldsNamesPairsPiecesProposals(value_); return std::nullopt; }
inline std::optional<RecognitionOddsFieldsPiecesProposals> RecognitionOdds::as_RecognitionOddsFieldsPiecesProposals() const { if (value_.contains("pieces") && value_.contains("proposals") && !value_.contains("names") && !value_.contains("pairs")) return RecognitionOddsFieldsPiecesProposals(value_); return std::nullopt; }
inline Presence<std::vector<NameOdds>> RecognitionOddsFieldsNamesPairsPiecesProposals::names() const { return member_value<std::vector<NameOdds>>(value_, "names"); }
inline Presence<std::vector<PairOdds>> RecognitionOddsFieldsNamesPairsPiecesProposals::pairs() const { return member_value<std::vector<PairOdds>>(value_, "pairs"); }
inline Presence<std::vector<PieceOdds>> RecognitionOddsFieldsNamesPairsPiecesProposals::pieces() const { return member_value<std::vector<PieceOdds>>(value_, "pieces"); }
inline Presence<std::vector<RecognitionProposal>> RecognitionOddsFieldsNamesPairsPiecesProposals::proposals() const { return member_value<std::vector<RecognitionProposal>>(value_, "proposals"); }
inline Presence<std::vector<PieceOdds>> RecognitionOddsFieldsPiecesProposals::pieces() const { return member_value<std::vector<PieceOdds>>(value_, "pieces"); }
inline Presence<std::vector<BoundaryProposal>> RecognitionOddsFieldsPiecesProposals::proposals() const { return member_value<std::vector<BoundaryProposal>>(value_, "proposals"); }
inline Presence<uint64_t> RecognitionProposal::end() const { return member_value<uint64_t>(value_, "end"); }
inline Presence<bool> RecognitionProposal::kept() const { return member_value<bool>(value_, "kept"); }
inline Presence<std::string> RecognitionProposal::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<Place> RecognitionProposal::selected() const { return member_value<Place>(value_, "selected"); }
inline Presence<double> RecognitionProposal::span_probability() const { return member_value<double>(value_, "span_probability"); }
inline Presence<uint64_t> RecognitionProposal::start() const { return member_value<uint64_t>(value_, "start"); }
inline Presence<double> RecognitionProposal::strength() const { return member_value<double>(value_, "strength"); }
inline Presence<std::string> RecognitionStageContext::boundary() const { return member_value<std::string>(value_, "boundary"); }
inline Presence<std::string> RecognitionStageContext::kind_edge() const { return member_value<std::string>(value_, "kind_edge"); }
inline Presence<std::string> RecognitionStageContext::relation() const { return member_value<std::string>(value_, "relation"); }
inline Presence<Answers> Relation::answer() const { return member_value<Answers>(value_, "answer"); }
inline Presence<AnswerId> Relation::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<std::string> Relation::file() const { return member_value<std::string>(value_, "file"); }
inline Presence<uint64_t> Relation::first_line() const { return member_value<uint64_t>(value_, "first_line"); }
inline Presence<uint64_t> Relation::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<Json> Relation::input() const { return member_value<Json>(value_, "input"); }
inline Presence<std::vector<SessionInputSource>> Relation::input_sources() const { return member_value<std::vector<SessionInputSource>>(value_, "input_sources"); }
inline Presence<uint64_t> Relation::last_line() const { return member_value<uint64_t>(value_, "last_line"); }
inline Presence<Meta> Relation::meta() const { return member_value<Meta>(value_, "meta"); }
inline Presence<Position> Relation::position() const { return member_value<Position>(value_, "position"); }
inline Presence<ReadableQuestion4> Relation::question() const { return member_value<ReadableQuestion4>(value_, "question"); }
inline Presence<Version> Relation::schema() const { return member_value<Version>(value_, "schema"); }
inline Presence<std::vector<RelatedEntityEdge>> Relation::value() const { return member_value<std::vector<RelatedEntityEdge>>(value_, "value"); }
inline std::string RelationDirection::value() const { return decode<std::string>(value_); }
inline std::optional<RelationMemberAnswerId> RelationMember::as_RelationMemberAnswerId() const { if (value_.contains("answer_id")) return RelationMemberAnswerId(value_); return std::nullopt; }
inline std::optional<RelationMemberFailureId> RelationMember::as_RelationMemberFailureId() const { if (value_.contains("failure_id")) return RelationMemberFailureId(value_); return std::nullopt; }
inline Presence<RelationDirection> RelationMemberAnswerId::direction() const { return member_value<RelationDirection>(value_, "direction"); }
inline Presence<RelationMethod> RelationMemberAnswerId::method() const { return member_value<RelationMethod>(value_, "method"); }
inline Presence<std::vector<Observation>> RelationMemberAnswerId::observations() const { return member_value<std::vector<Observation>>(value_, "observations"); }
inline Presence<ReadableQuestion> RelationMemberAnswerId::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::vector<QuestionSource>> RelationMemberAnswerId::question_sources() const { return member_value<std::vector<QuestionSource>>(value_, "question_sources"); }
inline Presence<std::string> RelationMemberAnswerId::reads() const { return member_value<std::string>(value_, "reads"); }
inline Presence<std::string> RelationMemberAnswerId::relation() const { return member_value<std::string>(value_, "relation"); }
inline Presence<std::string> RelationMemberAnswerId::request() const { return member_value<std::string>(value_, "request"); }
inline Presence<RelatedEntity> RelationMemberAnswerId::source() const { return member_value<RelatedEntity>(value_, "source"); }
inline Presence<RelatedEntity> RelationMemberAnswerId::target() const { return member_value<RelatedEntity>(value_, "target"); }
inline Presence<Threshold> RelationMemberAnswerId::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<Usage> RelationMemberAnswerId::usage() const { return member_value<Usage>(value_, "usage"); }
inline Presence<bool> RelationMemberAnswerId::accepted() const { return member_value<bool>(value_, "accepted"); }
inline Presence<Answer> RelationMemberAnswerId::answer() const { return member_value<Answer>(value_, "answer"); }
inline Presence<AnswerId> RelationMemberAnswerId::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<double> RelationMemberAnswerId::probability() const { return member_value<double>(value_, "probability"); }
inline Presence<RelationDirection> RelationMemberFailureId::direction() const { return member_value<RelationDirection>(value_, "direction"); }
inline Presence<RelationMethod> RelationMemberFailureId::method() const { return member_value<RelationMethod>(value_, "method"); }
inline Presence<std::vector<Observation>> RelationMemberFailureId::observations() const { return member_value<std::vector<Observation>>(value_, "observations"); }
inline Presence<ReadableQuestion> RelationMemberFailureId::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::vector<QuestionSource>> RelationMemberFailureId::question_sources() const { return member_value<std::vector<QuestionSource>>(value_, "question_sources"); }
inline Presence<std::string> RelationMemberFailureId::reads() const { return member_value<std::string>(value_, "reads"); }
inline Presence<std::string> RelationMemberFailureId::relation() const { return member_value<std::string>(value_, "relation"); }
inline Presence<std::string> RelationMemberFailureId::request() const { return member_value<std::string>(value_, "request"); }
inline Presence<RelatedEntity> RelationMemberFailureId::source() const { return member_value<RelatedEntity>(value_, "source"); }
inline Presence<RelatedEntity> RelationMemberFailureId::target() const { return member_value<RelatedEntity>(value_, "target"); }
inline Presence<Threshold> RelationMemberFailureId::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<Usage> RelationMemberFailureId::usage() const { return member_value<Usage>(value_, "usage"); }
inline Presence<Failure> RelationMemberFailureId::failure() const { return member_value<Failure>(value_, "failure"); }
inline Presence<FailureId> RelationMemberFailureId::failure_id() const { return member_value<FailureId>(value_, "failure_id"); }
inline std::string RelationMethod::value() const { return decode<std::string>(value_); }
inline std::string RequestFunction::value() const { return decode<std::string>(value_); }
inline std::string SdkRequestId::value() const { return decode<std::string>(value_); }
inline std::optional<SendBudgetDenialBeforeFirstSend> SendBudgetDenial::as_SendBudgetDenialBeforeFirstSend() const { if (literal(value_, "kind", "before_first_send")) return SendBudgetDenialBeforeFirstSend(value_); return std::nullopt; }
inline std::optional<SendBudgetDenialBeforeAdditionalSend> SendBudgetDenial::as_SendBudgetDenialBeforeAdditionalSend() const { if (literal(value_, "kind", "before_additional_send")) return SendBudgetDenialBeforeAdditionalSend(value_); return std::nullopt; }
inline std::optional<SendBudgetDenialBeforeRetry> SendBudgetDenial::as_SendBudgetDenialBeforeRetry() const { if (literal(value_, "kind", "before_retry")) return SendBudgetDenialBeforeRetry(value_); return std::nullopt; }
inline Presence<std::string> SendBudgetDenialBeforeAdditionalSend::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> SendBudgetDenialBeforeFirstSend::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> SendBudgetDenialBeforeRetry::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<uint64_t> SendBudgetDenialBeforeRetry::last_status() const { return member_value<uint64_t>(value_, "last_status"); }
inline std::string StopCause::value() const { return decode<std::string>(value_); }
inline Presence<uint64_t> Stopped::at() const { return member_value<uint64_t>(value_, "at"); }
inline Presence<StopCause> Stopped::cause() const { return member_value<StopCause>(value_, "cause"); }
inline Presence<bool> Stopped::retryable() const { return member_value<bool>(value_, "retryable"); }
inline Presence<uint64_t> Stopped::status() const { return member_value<uint64_t>(value_, "status"); }
inline Presence<StringType> StringRoot::type() const { return member_value<StringType>(value_, "type"); }
inline std::string StringType::value() const { return decode<std::string>(value_); }
inline Presence<uint64_t> Usage::input_tokens() const { return member_value<uint64_t>(value_, "input_tokens"); }
inline Presence<uint64_t> Usage::output_tokens() const { return member_value<uint64_t>(value_, "output_tokens"); }
inline std::string UsagePersistence::value() const { return decode<std::string>(value_); }
inline std::string Verb::value() const { return decode<std::string>(value_); }
inline std::string Version::value() const { return decode<std::string>(value_); }
inline uint32_t WordingVersion::value() const { return decode<uint32_t>(value_); }
inline std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double, Failed> AnnotatedField::value() const { return decode<std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double, Failed>>(value_); }
inline std::vector<std::pair<std::string, AnnotatedField>> AnnotatedRow::value() const { return decode<std::vector<std::pair<std::string, AnnotatedField>>>(value_); }
inline std::optional<AnswerYesNo> Answer::as_AnswerYesNo() const { if (literal(value_, "kind", "yes_no")) return AnswerYesNo(value_); return std::nullopt; }
inline std::optional<AnswerChoice> Answer::as_AnswerChoice() const { if (literal(value_, "kind", "choice")) return AnswerChoice(value_); return std::nullopt; }
inline std::optional<AnswerTag> Answer::as_AnswerTag() const { if (literal(value_, "kind", "tag")) return AnswerTag(value_); return std::nullopt; }
inline std::optional<AnswerScore> Answer::as_AnswerScore() const { if (literal(value_, "kind", "score")) return AnswerScore(value_); return std::nullopt; }
inline Presence<double> AnswerChoice::confidence() const { return member_value<double>(value_, "confidence"); }
inline Presence<std::string> AnswerChoice::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> AnswerChoice::pick() const { return member_value<std::string>(value_, "pick"); }
inline Presence<std::vector<std::pair<std::string, double>>> AnswerChoice::probabilities() const { return member_value<std::vector<std::pair<std::string, double>>>(value_, "probabilities"); }
inline Presence<double> AnswerScore::confidence() const { return member_value<double>(value_, "confidence"); }
inline Presence<std::string> AnswerScore::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> AnswerScore::level() const { return member_value<std::string>(value_, "level"); }
inline Presence<std::vector<std::pair<std::string, double>>> AnswerScore::probabilities() const { return member_value<std::vector<std::pair<std::string, double>>>(value_, "probabilities"); }
inline Presence<std::string> AnswerTag::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<std::pair<std::string, double>>> AnswerTag::probabilities() const { return member_value<std::vector<std::pair<std::string, double>>>(value_, "probabilities"); }
inline Presence<std::string> AnswerYesNo::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<double> AnswerYesNo::probability() const { return member_value<double>(value_, "probability"); }
inline std::string AttemptOutcome::value() const { return decode<std::string>(value_); }
inline std::variant<uint64_t, std::string> BatchSetting::value() const { return decode<std::variant<uint64_t, std::string>>(value_); }
inline Presence<BatchSetting> BatchWarning::running() const { return member_value<BatchSetting>(value_, "running"); }
inline Presence<BatchSetting> BatchWarning::tuned_for() const { return member_value<BatchSetting>(value_, "tuned_for"); }
inline Presence<uint64_t> Entity::end() const { return member_value<uint64_t>(value_, "end"); }
inline Presence<std::string> Entity::file() const { return member_value<std::string>(value_, "file"); }
inline Presence<uint64_t> Entity::first_line() const { return member_value<uint64_t>(value_, "first_line"); }
inline Presence<std::string> Entity::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<uint64_t> Entity::last_line() const { return member_value<uint64_t>(value_, "last_line"); }
inline Presence<uint64_t> Entity::length() const { return member_value<uint64_t>(value_, "length"); }
inline Presence<uint64_t> Entity::start() const { return member_value<uint64_t>(value_, "start"); }
inline Presence<double> Entity::strength() const { return member_value<double>(value_, "strength"); }
inline Presence<std::string> Entity::text() const { return member_value<std::string>(value_, "text"); }
inline Presence<bool> EntityEdge::either() const { return member_value<bool>(value_, "either"); }
inline Presence<double> EntityEdge::probability() const { return member_value<double>(value_, "probability"); }
inline Presence<std::string> EntityEdge::relation() const { return member_value<std::string>(value_, "relation"); }
inline Presence<Entity> EntityEdge::source() const { return member_value<Entity>(value_, "source"); }
inline Presence<Entity> EntityEdge::target() const { return member_value<Entity>(value_, "target"); }
inline Presence<Failure> Failed::failed() const { return member_value<Failure>(value_, "failed"); }
inline Presence<FailureCause> Failure::cause() const { return member_value<FailureCause>(value_, "cause"); }
inline Presence<std::string> Failure::kind() const { return member_value<std::string>(value_, "kind"); }
inline std::string FailureCause::value() const { return decode<std::string>(value_); }
inline std::string FailureKind::value() const { return decode<std::string>(value_); }
inline Presence<double> FindAnswer::confidence() const { return member_value<double>(value_, "confidence"); }
inline Presence<std::string> FindAnswer::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> FindAnswer::pick() const { return member_value<std::string>(value_, "pick"); }
inline Presence<std::vector<std::pair<std::string, double>>> FindAnswer::probabilities() const { return member_value<std::vector<std::pair<std::string, double>>>(value_, "probabilities"); }
inline Presence<std::vector<std::pair<std::string, double>>> NameOdds::edges() const { return member_value<std::vector<std::pair<std::string, double>>>(value_, "edges"); }
inline Presence<uint64_t> NameOdds::end() const { return member_value<uint64_t>(value_, "end"); }
inline Presence<std::vector<std::pair<std::string, double>>> NameOdds::kinds() const { return member_value<std::vector<std::pair<std::string, double>>>(value_, "kinds"); }
inline Presence<uint64_t> NameOdds::start() const { return member_value<uint64_t>(value_, "start"); }
inline Presence<double> PairOdds::probability() const { return member_value<double>(value_, "probability"); }
inline Presence<std::string> PairOdds::relation() const { return member_value<std::string>(value_, "relation"); }
inline Presence<Place> PairOdds::source() const { return member_value<Place>(value_, "source"); }
inline Presence<Place> PairOdds::target() const { return member_value<Place>(value_, "target"); }
inline Presence<uint64_t> PieceOdds::end() const { return member_value<uint64_t>(value_, "end"); }
inline Presence<uint64_t> PieceOdds::start() const { return member_value<uint64_t>(value_, "start"); }
inline Presence<std::vector<std::pair<std::string, double>>> PieceOdds::tags() const { return member_value<std::vector<std::pair<std::string, double>>>(value_, "tags"); }
inline Presence<uint64_t> Place::end() const { return member_value<uint64_t>(value_, "end"); }
inline Presence<uint64_t> Place::start() const { return member_value<uint64_t>(value_, "start"); }
inline Presence<std::string> ProfileWarning::running() const { return member_value<std::string>(value_, "running"); }
inline Presence<std::string> ProfileWarning::tuned_for() const { return member_value<std::string>(value_, "tuned_for"); }
inline std::optional<RecognizeFieldsEntities> Recognize::as_RecognizeFieldsEntities() const { if (value_.contains("entities") && !value_.contains("mode") && !value_.contains("proposals")) return RecognizeFieldsEntities(value_); return std::nullopt; }
inline std::optional<RecognizeFieldsModeProposals> Recognize::as_RecognizeFieldsModeProposals() const { if (value_.contains("mode") && value_.contains("proposals") && !value_.contains("entities")) return RecognizeFieldsModeProposals(value_); return std::nullopt; }
inline Presence<std::vector<NameOdds>> RecognizeAnswer::names() const { return member_value<std::vector<NameOdds>>(value_, "names"); }
inline Presence<std::vector<PairOdds>> RecognizeAnswer::pairs() const { return member_value<std::vector<PairOdds>>(value_, "pairs"); }
inline Presence<std::vector<PieceOdds>> RecognizeAnswer::pieces() const { return member_value<std::vector<PieceOdds>>(value_, "pieces"); }
inline Presence<std::vector<RecognitionProposal>> RecognizeAnswer::proposals() const { return member_value<std::vector<RecognitionProposal>>(value_, "proposals"); }
inline Presence<std::vector<Entity>> RecognizeFieldsEntities::entities() const { return member_value<std::vector<Entity>>(value_, "entities"); }
inline Presence<std::vector<EntityEdge>> RecognizeFieldsEntities::relations() const { return member_value<std::vector<EntityEdge>>(value_, "relations"); }
inline Presence<BoundaryMode> RecognizeFieldsModeProposals::mode() const { return member_value<BoundaryMode>(value_, "mode"); }
inline Presence<std::vector<BoundaryProposal>> RecognizeFieldsModeProposals::proposals() const { return member_value<std::vector<BoundaryProposal>>(value_, "proposals"); }
inline Presence<std::string> RelateFields::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> RelateFields::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<std::string> RelatedEntity::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> RelatedEntity::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<bool> RelatedEntityEdge::either() const { return member_value<bool>(value_, "either"); }
inline Presence<double> RelatedEntityEdge::probability() const { return member_value<double>(value_, "probability"); }
inline Presence<std::string> RelatedEntityEdge::relation() const { return member_value<std::string>(value_, "relation"); }
inline Presence<RelatedEntityEdgePropertiesSource> RelatedEntityEdge::source() const { return member_value<RelatedEntityEdgePropertiesSource>(value_, "source"); }
inline Presence<RelatedEntityEdgePropertiesSource> RelatedEntityEdge::target() const { return member_value<RelatedEntityEdgePropertiesSource>(value_, "target"); }
inline std::optional<RelatedEntityEdgePropertiesSourceFieldsKindName> RelatedEntityEdgePropertiesSource::as_RelatedEntityEdgePropertiesSourceFieldsKindName() const { if (value_.contains("kind") && value_.contains("name") && !value_.contains("file") && !value_.contains("ordinal") && !value_.contains("record")) return RelatedEntityEdgePropertiesSourceFieldsKindName(value_); return std::nullopt; }
inline std::optional<RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord> RelatedEntityEdgePropertiesSource::as_RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord() const { if (value_.contains("file") && value_.contains("kind") && value_.contains("name") && value_.contains("ordinal") && value_.contains("record")) return RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord(value_); return std::nullopt; }
inline Presence<std::string> RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord::file() const { return member_value<std::string>(value_, "file"); }
inline Presence<uint64_t> RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord::first_line() const { return member_value<uint64_t>(value_, "first_line"); }
inline Presence<std::string> RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<uint64_t> RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord::last_line() const { return member_value<uint64_t>(value_, "last_line"); }
inline Presence<std::string> RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<uint64_t> RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord::ordinal() const { return member_value<uint64_t>(value_, "ordinal"); }
inline Presence<Json> RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord::record() const { return member_value<Json>(value_, "record"); }
inline Presence<std::string> RelatedEntityEdgePropertiesSourceFieldsKindName::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> RelatedEntityEdgePropertiesSourceFieldsKindName::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<bool> RelationRule::either() const { return member_value<bool>(value_, "either"); }
inline Presence<std::string> RelationRule::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<std::string> RelationRule::reads() const { return member_value<std::string>(value_, "reads"); }
inline Presence<bool> RelationRule::single() const { return member_value<bool>(value_, "single"); }
inline Presence<std::string> RelationRule::source() const { return member_value<std::string>(value_, "source"); }
inline Presence<std::string> RelationRule::target() const { return member_value<std::string>(value_, "target"); }
inline Presence<std::string> SessionAnnotation::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<AnnotationValue> SessionAnnotation::value() const { return member_value<AnnotationValue>(value_, "value"); }
inline Presence<uint64_t> SessionInputSource::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<PhysicalSource> SessionInputSource::source() const { return member_value<PhysicalSource>(value_, "source"); }
inline std::optional<SessionJudgmentDecision> SessionJudgment::as_SessionJudgmentDecision() const { if (literal(value_, "kind", "decision")) return SessionJudgmentDecision(value_); return std::nullopt; }
inline std::optional<SessionJudgmentChoice> SessionJudgment::as_SessionJudgmentChoice() const { if (literal(value_, "kind", "choice")) return SessionJudgmentChoice(value_); return std::nullopt; }
inline std::optional<SessionJudgmentScore> SessionJudgment::as_SessionJudgmentScore() const { if (literal(value_, "kind", "score")) return SessionJudgmentScore(value_); return std::nullopt; }
inline std::optional<SessionJudgmentTags> SessionJudgment::as_SessionJudgmentTags() const { if (literal(value_, "kind", "tags")) return SessionJudgmentTags(value_); return std::nullopt; }
inline Presence<std::string> SessionJudgmentChoice::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> SessionJudgmentChoice::value() const { return member_value<std::string>(value_, "value"); }
inline Presence<std::string> SessionJudgmentDecision::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<bool> SessionJudgmentDecision::value() const { return member_value<bool>(value_, "value"); }
inline Presence<std::string> SessionJudgmentScore::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<double> SessionJudgmentScore::value() const { return member_value<double>(value_, "value"); }
inline Presence<std::string> SessionJudgmentTags::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<std::string>> SessionJudgmentTags::value() const { return member_value<std::vector<std::string>>(value_, "value"); }
inline Presence<std::string> SessionNamedProbability::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<double> SessionNamedProbability::probability() const { return member_value<double>(value_, "probability"); }
inline std::optional<SessionObservationQuestion> SessionObservation::as_SessionObservationQuestion() const { if (literal(value_, "kind", "question")) return SessionObservationQuestion(value_); return std::nullopt; }
inline std::optional<SessionObservationRow> SessionObservation::as_SessionObservationRow() const { if (literal(value_, "kind", "row")) return SessionObservationRow(value_); return std::nullopt; }
inline Presence<SessionQuestionDetail> SessionObservationQuestion::detail() const { return member_value<SessionQuestionDetail>(value_, "detail"); }
inline Presence<uint64_t> SessionObservationQuestion::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<std::string> SessionObservationQuestion::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::string> SessionObservationQuestion::member() const { return member_value<std::string>(value_, "member"); }
inline Presence<uint64_t> SessionObservationQuestion::position() const { return member_value<uint64_t>(value_, "position"); }
inline Presence<std::string> SessionObservationQuestion::stage() const { return member_value<std::string>(value_, "stage"); }
inline Presence<uint64_t> SessionObservationRow::index() const { return member_value<uint64_t>(value_, "index"); }
inline Presence<std::string> SessionObservationRow::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<SessionObservedRow> SessionObservationRow::value() const { return member_value<SessionObservedRow>(value_, "value"); }
inline std::optional<SessionObservedRowJudgment> SessionObservedRow::as_SessionObservedRowJudgment() const { if (literal(value_, "kind", "judgment")) return SessionObservedRowJudgment(value_); return std::nullopt; }
inline std::optional<SessionObservedRowAnnotated> SessionObservedRow::as_SessionObservedRowAnnotated() const { if (literal(value_, "kind", "annotated")) return SessionObservedRowAnnotated(value_); return std::nullopt; }
inline std::optional<SessionObservedRowRecognized> SessionObservedRow::as_SessionObservedRowRecognized() const { if (literal(value_, "kind", "recognized")) return SessionObservedRowRecognized(value_); return std::nullopt; }
inline std::optional<SessionObservedRowFind> SessionObservedRow::as_SessionObservedRowFind() const { if (literal(value_, "kind", "find")) return SessionObservedRowFind(value_); return std::nullopt; }
inline std::optional<SessionObservedRowRelations> SessionObservedRow::as_SessionObservedRowRelations() const { if (literal(value_, "kind", "relations")) return SessionObservedRowRelations(value_); return std::nullopt; }
inline Presence<std::string> SessionObservedRowAnnotated::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<SessionAnnotation>> SessionObservedRowAnnotated::value() const { return member_value<std::vector<SessionAnnotation>>(value_, "value"); }
inline Presence<std::string> SessionObservedRowFind::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<uint64_t> SessionObservedRowFind::value() const { return member_value<uint64_t>(value_, "value"); }
inline Presence<std::string> SessionObservedRowJudgment::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<SessionJudgment> SessionObservedRowJudgment::value() const { return member_value<SessionJudgment>(value_, "value"); }
inline Presence<std::string> SessionObservedRowRecognized::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<SessionRecognition> SessionObservedRowRecognized::value() const { return member_value<SessionRecognition>(value_, "value"); }
inline Presence<std::string> SessionObservedRowRelations::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<SessionRelationEdge>> SessionObservedRowRelations::value() const { return member_value<std::vector<SessionRelationEdge>>(value_, "value"); }
inline std::optional<SessionPacketDecideRow> SessionPacket::as_SessionPacketDecideRow() const { if (literal(value_, "function", "decide") && literal(value_, "kind", "row")) return SessionPacketDecideRow(value_); return std::nullopt; }
inline std::optional<SessionPacketChooseRow> SessionPacket::as_SessionPacketChooseRow() const { if (literal(value_, "function", "choose") && literal(value_, "kind", "row")) return SessionPacketChooseRow(value_); return std::nullopt; }
inline std::optional<SessionPacketTagRow> SessionPacket::as_SessionPacketTagRow() const { if (literal(value_, "function", "tag") && literal(value_, "kind", "row")) return SessionPacketTagRow(value_); return std::nullopt; }
inline std::optional<SessionPacketScoreRow> SessionPacket::as_SessionPacketScoreRow() const { if (literal(value_, "function", "score") && literal(value_, "kind", "row")) return SessionPacketScoreRow(value_); return std::nullopt; }
inline std::optional<SessionPacketFilterRow> SessionPacket::as_SessionPacketFilterRow() const { if (literal(value_, "function", "filter") && literal(value_, "kind", "row")) return SessionPacketFilterRow(value_); return std::nullopt; }
inline std::optional<SessionPacketAnnotateRow> SessionPacket::as_SessionPacketAnnotateRow() const { if (literal(value_, "function", "annotate") && literal(value_, "kind", "row")) return SessionPacketAnnotateRow(value_); return std::nullopt; }
inline std::optional<SessionPacketDecideAggregate> SessionPacket::as_SessionPacketDecideAggregate() const { if (literal(value_, "function", "decide") && literal(value_, "kind", "aggregate")) return SessionPacketDecideAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketChooseAggregate> SessionPacket::as_SessionPacketChooseAggregate() const { if (literal(value_, "function", "choose") && literal(value_, "kind", "aggregate")) return SessionPacketChooseAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketTagAggregate> SessionPacket::as_SessionPacketTagAggregate() const { if (literal(value_, "function", "tag") && literal(value_, "kind", "aggregate")) return SessionPacketTagAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketScoreAggregate> SessionPacket::as_SessionPacketScoreAggregate() const { if (literal(value_, "function", "score") && literal(value_, "kind", "aggregate")) return SessionPacketScoreAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketFilterAggregate> SessionPacket::as_SessionPacketFilterAggregate() const { if (literal(value_, "function", "filter") && literal(value_, "kind", "aggregate")) return SessionPacketFilterAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketRankAggregate> SessionPacket::as_SessionPacketRankAggregate() const { if (literal(value_, "function", "rank") && literal(value_, "kind", "aggregate")) return SessionPacketRankAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketFindAggregate> SessionPacket::as_SessionPacketFindAggregate() const { if (literal(value_, "function", "find") && literal(value_, "kind", "aggregate")) return SessionPacketFindAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketAnnotateAggregate> SessionPacket::as_SessionPacketAnnotateAggregate() const { if (literal(value_, "function", "annotate") && literal(value_, "kind", "aggregate")) return SessionPacketAnnotateAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketRecognizeAggregate> SessionPacket::as_SessionPacketRecognizeAggregate() const { if (literal(value_, "function", "recognize") && literal(value_, "kind", "aggregate")) return SessionPacketRecognizeAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketRelateAggregate> SessionPacket::as_SessionPacketRelateAggregate() const { if (literal(value_, "function", "relate") && literal(value_, "kind", "aggregate")) return SessionPacketRelateAggregate(value_); return std::nullopt; }
inline std::optional<SessionPacketObservation> SessionPacket::as_SessionPacketObservation() const { if (literal(value_, "kind", "observation")) return SessionPacketObservation(value_); return std::nullopt; }
inline std::optional<SessionPacketTerminal> SessionPacket::as_SessionPacketTerminal() const { if (literal(value_, "kind", "terminal")) return SessionPacketTerminal(value_); return std::nullopt; }
inline Presence<std::string> SessionPacketAnnotateAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketAnnotateAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<Annotation>> SessionPacketAnnotateAggregate::value() const { return member_value<std::vector<Annotation>>(value_, "value"); }
inline Presence<std::string> SessionPacketAnnotateRow::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketAnnotateRow::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<Annotation> SessionPacketAnnotateRow::value() const { return member_value<Annotation>(value_, "value"); }
inline Presence<std::string> SessionPacketChooseAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketChooseAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<AtomicNullableString>> SessionPacketChooseAggregate::value() const { return member_value<std::vector<AtomicNullableString>>(value_, "value"); }
inline Presence<std::string> SessionPacketChooseRow::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketChooseRow::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<AtomicNullableString> SessionPacketChooseRow::value() const { return member_value<AtomicNullableString>(value_, "value"); }
inline Presence<std::string> SessionPacketDecideAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketDecideAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<AtomicDecideValue>> SessionPacketDecideAggregate::value() const { return member_value<std::vector<AtomicDecideValue>>(value_, "value"); }
inline Presence<std::string> SessionPacketDecideRow::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketDecideRow::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<AtomicDecideValue> SessionPacketDecideRow::value() const { return member_value<AtomicDecideValue>(value_, "value"); }
inline Presence<std::string> SessionPacketFilterAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketFilterAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<AtomicBoolean>> SessionPacketFilterAggregate::value() const { return member_value<std::vector<AtomicBoolean>>(value_, "value"); }
inline Presence<std::string> SessionPacketFilterRow::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketFilterRow::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<AtomicBoolean> SessionPacketFilterRow::value() const { return member_value<AtomicBoolean>(value_, "value"); }
inline Presence<std::string> SessionPacketFindAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketFindAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<Find> SessionPacketFindAggregate::value() const { return member_value<Find>(value_, "value"); }
inline Presence<RequestFunction> SessionPacketObservation::function() const { return member_value<RequestFunction>(value_, "function"); }
inline Presence<std::string> SessionPacketObservation::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<SessionObservation> SessionPacketObservation::value() const { return member_value<SessionObservation>(value_, "value"); }
inline Presence<std::string> SessionPacketRankAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketRankAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<AtomicNonZeroUsize>> SessionPacketRankAggregate::value() const { return member_value<std::vector<AtomicNonZeroUsize>>(value_, "value"); }
inline Presence<std::string> SessionPacketRecognizeAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketRecognizeAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<Recognition>> SessionPacketRecognizeAggregate::value() const { return member_value<std::vector<Recognition>>(value_, "value"); }
inline Presence<std::string> SessionPacketRelateAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketRelateAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<Relation> SessionPacketRelateAggregate::value() const { return member_value<Relation>(value_, "value"); }
inline Presence<std::string> SessionPacketScoreAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketScoreAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<AtomicDouble>> SessionPacketScoreAggregate::value() const { return member_value<std::vector<AtomicDouble>>(value_, "value"); }
inline Presence<std::string> SessionPacketScoreRow::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketScoreRow::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<AtomicDouble> SessionPacketScoreRow::value() const { return member_value<AtomicDouble>(value_, "value"); }
inline Presence<std::string> SessionPacketTagAggregate::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketTagAggregate::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<AtomicArrayOfString>> SessionPacketTagAggregate::value() const { return member_value<std::vector<AtomicArrayOfString>>(value_, "value"); }
inline Presence<std::string> SessionPacketTagRow::function() const { return member_value<std::string>(value_, "function"); }
inline Presence<std::string> SessionPacketTagRow::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<AtomicArrayOfString> SessionPacketTagRow::value() const { return member_value<AtomicArrayOfString>(value_, "value"); }
inline Presence<Facts> SessionPacketTerminal::facts() const { return member_value<Facts>(value_, "facts"); }
inline Presence<CallError> SessionPacketTerminal::failure() const { return member_value<CallError>(value_, "failure"); }
inline Presence<std::string> SessionPacketTerminal::kind() const { return member_value<std::string>(value_, "kind"); }
inline std::optional<SessionProbabilitiesYesNo> SessionProbabilities::as_SessionProbabilitiesYesNo() const { if (literal(value_, "kind", "yes_no")) return SessionProbabilitiesYesNo(value_); return std::nullopt; }
inline std::optional<SessionProbabilitiesNamed> SessionProbabilities::as_SessionProbabilitiesNamed() const { if (literal(value_, "kind", "named")) return SessionProbabilitiesNamed(value_); return std::nullopt; }
inline Presence<std::string> SessionProbabilitiesNamed::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<std::vector<SessionNamedProbability>> SessionProbabilitiesNamed::value() const { return member_value<std::vector<SessionNamedProbability>>(value_, "value"); }
inline Presence<std::string> SessionProbabilitiesYesNo::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<double> SessionProbabilitiesYesNo::value() const { return member_value<double>(value_, "value"); }
inline Presence<AnswerId> SessionQuestionDetail::answer_id() const { return member_value<AnswerId>(value_, "answer_id"); }
inline Presence<bool> SessionQuestionDetail::cached() const { return member_value<bool>(value_, "cached"); }
inline Presence<double> SessionQuestionDetail::confidence() const { return member_value<double>(value_, "confidence"); }
inline Presence<uint64_t> SessionQuestionDetail::failed_questions() const { return member_value<uint64_t>(value_, "failed_questions"); }
inline Presence<Failure> SessionQuestionDetail::failure() const { return member_value<Failure>(value_, "failure"); }
inline Presence<FailureId> SessionQuestionDetail::failure_id() const { return member_value<FailureId>(value_, "failure_id"); }
inline Presence<Json> SessionQuestionDetail::input() const { return member_value<Json>(value_, "input"); }
inline Presence<PhysicalSource> SessionQuestionDetail::input_source() const { return member_value<PhysicalSource>(value_, "input_source"); }
inline Presence<std::vector<SessionInputSource>> SessionQuestionDetail::input_sources() const { return member_value<std::vector<SessionInputSource>>(value_, "input_sources"); }
inline Presence<std::vector<Json>> SessionQuestionDetail::inputs() const { return member_value<std::vector<Json>>(value_, "inputs"); }
inline Presence<std::string> SessionQuestionDetail::model() const { return member_value<std::string>(value_, "model"); }
inline Presence<std::vector<Observation>> SessionQuestionDetail::observations() const { return member_value<std::vector<Observation>>(value_, "observations"); }
inline Presence<SessionProbabilities> SessionQuestionDetail::probabilities() const { return member_value<SessionProbabilities>(value_, "probabilities"); }
inline Presence<ReadableQuestion> SessionQuestionDetail::question() const { return member_value<ReadableQuestion>(value_, "question"); }
inline Presence<std::string> SessionQuestionDetail::question_sha256() const { return member_value<std::string>(value_, "question_sha256"); }
inline Presence<std::vector<QuestionSource>> SessionQuestionDetail::question_sources() const { return member_value<std::vector<QuestionSource>>(value_, "question_sources"); }
inline Presence<std::string> SessionQuestionDetail::raw_pick() const { return member_value<std::string>(value_, "raw_pick"); }
inline Presence<Usage> SessionQuestionDetail::reported_usage() const { return member_value<Usage>(value_, "reported_usage"); }
inline Presence<std::vector<std::string>> SessionQuestionDetail::requests() const { return member_value<std::vector<std::string>>(value_, "requests"); }
inline Presence<uint64_t> SessionQuestionDetail::requests_sent() const { return member_value<uint64_t>(value_, "requests_sent"); }
inline Presence<Threshold> SessionQuestionDetail::threshold() const { return member_value<Threshold>(value_, "threshold"); }
inline Presence<std::string> SessionQuestionDetail::url() const { return member_value<std::string>(value_, "url"); }
inline Presence<TokenUsage> SessionQuestionDetail::usage() const { return member_value<TokenUsage>(value_, "usage"); }
inline Presence<Value> SessionQuestionDetail::value() const { return member_value<Value>(value_, "value"); }
inline Presence<std::vector<Entity>> SessionRecognition::entities() const { return member_value<std::vector<Entity>>(value_, "entities"); }
inline Presence<RecognitionMode> SessionRecognition::mode() const { return member_value<RecognitionMode>(value_, "mode"); }
inline Presence<std::vector<BoundaryProposal>> SessionRecognition::proposals() const { return member_value<std::vector<BoundaryProposal>>(value_, "proposals"); }
inline Presence<std::vector<RecognitionEdgeDocument>> SessionRecognition::relations() const { return member_value<std::vector<RecognitionEdgeDocument>>(value_, "relations"); }
inline Presence<bool> SessionRelationEdge::either() const { return member_value<bool>(value_, "either"); }
inline Presence<double> SessionRelationEdge::probability() const { return member_value<double>(value_, "probability"); }
inline Presence<std::string> SessionRelationEdge::relation() const { return member_value<std::string>(value_, "relation"); }
inline Presence<EntityDocument> SessionRelationEdge::source() const { return member_value<EntityDocument>(value_, "source"); }
inline Presence<EntityDocument> SessionRelationEdge::target() const { return member_value<EntityDocument>(value_, "target"); }
inline Presence<std::string> SourceRelationEndpoint::file() const { return member_value<std::string>(value_, "file"); }
inline Presence<uint64_t> SourceRelationEndpoint::first_line() const { return member_value<uint64_t>(value_, "first_line"); }
inline Presence<std::string> SourceRelationEndpoint::kind() const { return member_value<std::string>(value_, "kind"); }
inline Presence<uint64_t> SourceRelationEndpoint::last_line() const { return member_value<uint64_t>(value_, "last_line"); }
inline Presence<std::string> SourceRelationEndpoint::name() const { return member_value<std::string>(value_, "name"); }
inline Presence<uint64_t> SourceRelationEndpoint::ordinal() const { return member_value<uint64_t>(value_, "ordinal"); }
inline Presence<Json> SourceRelationEndpoint::record() const { return member_value<Json>(value_, "record"); }
inline std::variant<double, std::string> Threshold::value() const { return decode<std::variant<double, std::string>>(value_); }
inline Presence<uint64_t> TokenUsage::input_tokens() const { return member_value<uint64_t>(value_, "input_tokens"); }
inline Presence<uint64_t> TokenUsage::output_tokens() const { return member_value<uint64_t>(value_, "output_tokens"); }
inline std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double> Value::value() const { return decode<std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double>>(value_); }
}
// Generated from the compiler-derived C header; do not edit.
#include <thinkthen/thinkthen.h>
namespace tt {
enum class UsagePersistenceState : uint32_t {
disabled = THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1,
failed = THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1,
pending = THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1,
written = THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1,
};
}
