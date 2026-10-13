// Generated from the shared Rust result graph; do not edit.
#pragma once
#include "result_value.hpp"
#include <memory>
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
    class InputDeclarationObjectRoot;
    class InputDeclarationStringRoot;
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
    class RecognitionOddsBoundaryOdds;
    class RecognitionOddsRecognizeAnswer;
    class RecognitionProposal;
    class RecognitionStageContext;
    class Relation;
    class RelationDirection;
    class RelationEndpoint;
    class RelationEndpointRelatedEntity;
    class RelationEndpointSource;
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
    class Plan;
    class ProfileWarning;
    class Recognize;
    class RecognizeAnswer;
    class RecognizeEntities;
    class RecognizeModeProposals;
    class RelateFields;
    class RelatedEntity;
    class RelatedEntityEdge;
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
    class TokenBand;
    class TokenUsage;
    class Value;
    class Annotation : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, AnnotationMember>>>> cached_answers_;
        public: const Presence<std::vector<std::pair<std::string, AnnotationMember>>>& answers() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_file_;
        public: const Presence<std::string>& file() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_first_line_;
        public: const Presence<uint64_t>& first_line() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_line_;
        public: const Presence<uint64_t>& last_line() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<Position>> cached_position_;
        public: const Presence<Position>& position() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
        private: mutable std::shared_ptr<const Presence<AnnotatedRow>> cached_value_;
        public: const Presence<AnnotatedRow>& value() const;
    };
    class AnnotationMember : public Node {
        public: using Node::Node;
        std::optional<AnnotationMemberAnswerId> as_AnnotationMemberAnswerId() const;
        std::optional<AnnotationMemberFailureId> as_AnnotationMemberFailureId() const;
    };
    class AnnotationMemberAnswerId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Answer>> cached_answer_;
        public: const Presence<Answer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Observation>>> cached_observations_;
        public: const Presence<std::vector<Observation>>& observations() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::vector<QuestionSource>>> cached_question_sources_;
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_request_;
        public: const Presence<std::string>& request() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<Usage>> cached_usage_;
        public: const Presence<Usage>& usage() const;
        private: mutable std::shared_ptr<const Presence<Value>> cached_value_;
        public: const Presence<Value>& value() const;
    };
    class AnnotationMemberFailureId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Failure>> cached_failure_;
        public: const Presence<Failure>& failure() const;
        private: mutable std::shared_ptr<const Presence<FailureId>> cached_failure_id_;
        public: const Presence<FailureId>& failure_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Observation>>> cached_observations_;
        public: const Presence<std::vector<Observation>>& observations() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::vector<QuestionSource>>> cached_question_sources_;
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_request_;
        public: const Presence<std::string>& request() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<Usage>> cached_usage_;
        public: const Presence<Usage>& usage() const;
    };
    class AnnotationValue : public Node {
        public: using Node::Node;
        std::optional<AnnotationValueDecision> as_AnnotationValueDecision() const;
        std::optional<AnnotationValueChoice> as_AnnotationValueChoice() const;
        std::optional<AnnotationValueScore> as_AnnotationValueScore() const;
        std::optional<AnnotationValueTags> as_AnnotationValueTags() const;
        std::optional<AnnotationValueFailed> as_AnnotationValueFailed() const;
    };
    class AnnotationValueChoice : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_value_;
        public: const Presence<std::string>& value() const;
    };
    class AnnotationValueDecision : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_value_;
        public: const Presence<bool>& value() const;
    };
    class AnnotationValueFailed : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<Failure>> cached_value_;
        public: const Presence<Failure>& value() const;
    };
    class AnnotationValueScore : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_value_;
        public: const Presence<double>& value() const;
    };
    class AnnotationValueTags : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_value_;
        public: const Presence<std::vector<std::string>>& value() const;
    };
    class AnswerId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class Answers : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<RelationMember>>> cached_questions_;
        public: const Presence<std::vector<RelationMember>>& questions() const;
    };
    class AtomicArrayOfString : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Answer>> cached_answer_;
        public: const Presence<Answer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Image>>> cached_images_;
        public: const Presence<std::vector<Image>>& images() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RankMember>>> cached_members_;
        public: const Presence<std::vector<RankMember>>& members() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_question_name_;
        public: const Presence<std::string>& question_name() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_value_;
        public: const Presence<std::vector<std::string>>& value() const;
    };
    class AtomicDecideValue : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Answer>> cached_answer_;
        public: const Presence<Answer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Image>>> cached_images_;
        public: const Presence<std::vector<Image>>& images() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RankMember>>> cached_members_;
        public: const Presence<std::vector<RankMember>>& members() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_question_name_;
        public: const Presence<std::string>& question_name() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<DecideValue>> cached_value_;
        public: const Presence<DecideValue>& value() const;
    };
    class AtomicNonZeroUsize : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Answer>> cached_answer_;
        public: const Presence<Answer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Image>>> cached_images_;
        public: const Presence<std::vector<Image>>& images() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RankMember>>> cached_members_;
        public: const Presence<std::vector<RankMember>>& members() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_question_name_;
        public: const Presence<std::string>& question_name() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_value_;
        public: const Presence<uint64_t>& value() const;
    };
    class AtomicNullableString : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Answer>> cached_answer_;
        public: const Presence<Answer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Image>>> cached_images_;
        public: const Presence<std::vector<Image>>& images() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RankMember>>> cached_members_;
        public: const Presence<std::vector<RankMember>>& members() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_question_name_;
        public: const Presence<std::string>& question_name() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_value_;
        public: const Presence<std::string>& value() const;
    };
    class AtomicBoolean : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Answer>> cached_answer_;
        public: const Presence<Answer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Image>>> cached_images_;
        public: const Presence<std::vector<Image>>& images() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RankMember>>> cached_members_;
        public: const Presence<std::vector<RankMember>>& members() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_question_name_;
        public: const Presence<std::string>& question_name() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_value_;
        public: const Presence<bool>& value() const;
    };
    class AtomicDouble : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Answer>> cached_answer_;
        public: const Presence<Answer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Image>>> cached_images_;
        public: const Presence<std::vector<Image>>& images() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RankMember>>> cached_members_;
        public: const Presence<std::vector<RankMember>>& members() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_question_name_;
        public: const Presence<std::string>& question_name() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_value_;
        public: const Presence<double>& value() const;
    };
    class Attempt : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_ordinal_;
        public: const Presence<uint64_t>& ordinal() const;
        private: mutable std::shared_ptr<const Presence<AttemptOutcome>> cached_outcome_;
        public: const Presence<AttemptOutcome>& outcome() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_request_id_;
        public: const Presence<std::string>& request_id() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_request_sha256_;
        public: const Presence<std::string>& request_sha256() const;
        private: mutable std::shared_ptr<const Presence<SdkRequestId>> cached_sdk_request_id_;
        public: const Presence<SdkRequestId>& sdk_request_id() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_server_ms_;
        public: const Presence<uint64_t>& server_ms() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_status_;
        public: const Presence<uint64_t>& status() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_wall_ms_;
        public: const Presence<uint64_t>& wall_ms() const;
    };
    class Batch : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::variant<uint64_t, std::string>> cached_value_;
        public: const std::variant<uint64_t, std::string>& value() const;
    };
    class BoundaryMode : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class BoundaryOdds : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<PieceOdds>>> cached_pieces_;
        public: const Presence<std::vector<PieceOdds>>& pieces() const;
        private: mutable std::shared_ptr<const Presence<std::vector<BoundaryProposal>>> cached_proposals_;
        public: const Presence<std::vector<BoundaryProposal>>& proposals() const;
    };
    class BoundaryProposal : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_end_;
        public: const Presence<uint64_t>& end() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_length_;
        public: const Presence<uint64_t>& length() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_start_;
        public: const Presence<uint64_t>& start() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_text_;
        public: const Presence<std::string>& text() const;
    };
    class CallError : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Error>> cached_error_;
        public: const Presence<Error>& error() const;
        private: mutable std::shared_ptr<const Presence<Facts>> cached_facts_;
        public: const Presence<Facts>& facts() const;
    };
    class CallId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class DecideValue : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Json> cached_value_;
        public: const Json& value() const;
    };
    class EntityDocument : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
    };
    class Error : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<EstimatedInputDenial>> cached_estimated_input_denial_;
        public: const Presence<EstimatedInputDenial>& estimated_input_denial() const;
        private: mutable std::shared_ptr<const Presence<FailureKind>> cached_kind_;
        public: const Presence<FailureKind>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_message_;
        public: const Presence<std::string>& message() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_retryable_;
        public: const Presence<bool>& retryable() const;
        private: mutable std::shared_ptr<const Presence<SendBudgetDenial>> cached_send_budget_denial_;
        public: const Presence<SendBudgetDenial>& send_budget_denial() const;
        private: mutable std::shared_ptr<const Presence<Stopped>> cached_stopped_;
        public: const Presence<Stopped>& stopped() const;
    };
    class EstimatedInputDenial : public Node {
        public: using Node::Node;
        std::optional<EstimatedInputDenialInitialRequest> as_EstimatedInputDenialInitialRequest() const;
        std::optional<EstimatedInputDenialAdditionalRequest> as_EstimatedInputDenialAdditionalRequest() const;
        std::optional<EstimatedInputDenialRetry> as_EstimatedInputDenialRetry() const;
    };
    class EstimatedInputDenialAdditionalRequest : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_limit_;
        public: const Presence<uint64_t>& limit() const;
    };
    class EstimatedInputDenialInitialRequest : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_limit_;
        public: const Presence<uint64_t>& limit() const;
    };
    class EstimatedInputDenialRetry : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_status_;
        public: const Presence<uint64_t>& last_status() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_limit_;
        public: const Presence<uint64_t>& limit() const;
    };
    class Facts : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<Attempt>>> cached_attempts_;
        public: const Presence<std::vector<Attempt>>& attempts() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_cache_answers_;
        public: const Presence<uint64_t>& cache_answers() const;
        private: mutable std::shared_ptr<const Presence<CallId>> cached_call_id_;
        public: const Presence<CallId>& call_id() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_estimated_cost_usd_;
        public: const Presence<std::string>& estimated_cost_usd() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_held_model_mismatch_;
        public: const Presence<bool>& held_model_mismatch() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_input_tokens_;
        public: const Presence<uint64_t>& input_tokens() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_largest_request_bytes_;
        public: const Presence<uint64_t>& largest_request_bytes() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_largest_request_estimated_input_tokens_;
        public: const Presence<uint64_t>& largest_request_estimated_input_tokens() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_output_tokens_;
        public: const Presence<uint64_t>& output_tokens() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_records_;
        public: const Presence<uint64_t>& records() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_requests_sent_;
        public: const Presence<uint64_t>& requests_sent() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_seconds_;
        public: const Presence<double>& seconds() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_token_estimate_method_;
        public: const Presence<std::string>& token_estimate_method() const;
        private: mutable std::shared_ptr<const Presence<PersistenceObservation>> cached_usage_persistence_;
        public: const Presence<PersistenceObservation>& usage_persistence() const;
    };
    class FailureId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class Find : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<FindAnswer>> cached_answer_;
        public: const Presence<FindAnswer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<FindCandidate>>> cached_candidates_;
        public: const Presence<std::vector<FindCandidate>>& candidates() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_file_;
        public: const Presence<std::string>& file() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_first_line_;
        public: const Presence<uint64_t>& first_line() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_line_;
        public: const Presence<uint64_t>& last_line() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<Position>> cached_position_;
        public: const Presence<Position>& position() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion2>> cached_question_;
        public: const Presence<ReadableQuestion2>& question() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<std::nullptr_t>> cached_threshold_;
        public: const Presence<std::nullptr_t>& threshold() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_value_;
        public: const Presence<Json>& value() const;
    };
    class FindCandidate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
    };
    class Image : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_base64_;
        public: const Presence<std::string>& base64() const;
        private: mutable std::shared_ptr<const Presence<uint32_t>> cached_height_;
        public: const Presence<uint32_t>& height() const;
        private: mutable std::shared_ptr<const Presence<ImageMedia>> cached_media_;
        public: const Presence<ImageMedia>& media() const;
        private: mutable std::shared_ptr<const Presence<uint32_t>> cached_width_;
        public: const Presence<uint32_t>& width() const;
    };
    class ImageMedia : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class InputDeclaration : public Node {
        public: using Node::Node;
        std::optional<InputDeclarationStringRoot> as_InputDeclarationStringRoot() const;
        std::optional<InputDeclarationObjectRoot> as_InputDeclarationObjectRoot() const;
    };
    class InputDeclarationObjectRoot : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, InputPropertyType>>>> cached_properties_;
        public: const Presence<std::vector<std::pair<std::string, InputPropertyType>>>& properties() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_required_;
        public: const Presence<std::vector<std::string>>& required() const;
        private: mutable std::shared_ptr<const Presence<ObjectType>> cached_type_;
        public: const Presence<ObjectType>& type() const;
    };
    class InputDeclarationStringRoot : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<StringType>> cached_type_;
        public: const Presence<StringType>& type() const;
    };
    class InputPropertyType : public Node {
        public: using Node::Node;
        std::optional<InputPropertyTypeString> as_InputPropertyTypeString() const;
        std::optional<InputPropertyTypeNumber> as_InputPropertyTypeNumber() const;
        std::optional<InputPropertyTypeBoolean> as_InputPropertyTypeBoolean() const;
        std::optional<InputPropertyTypeArray> as_InputPropertyTypeArray() const;
    };
    class InputPropertyTypeArray : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<StringRoot>> cached_items_;
        public: const Presence<StringRoot>& items() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_type_;
        public: const Presence<std::string>& type() const;
    };
    class InputPropertyTypeBoolean : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_type_;
        public: const Presence<std::string>& type() const;
    };
    class InputPropertyTypeNumber : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_type_;
        public: const Presence<std::string>& type() const;
    };
    class InputPropertyTypeString : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_type_;
        public: const Presence<std::string>& type() const;
    };
    class Label : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Json>> cached_description_;
        public: const Presence<Json>& description() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
    };
    class Meta : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_answered_by_;
        public: const Presence<std::string>& answered_by() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Attempt>>> cached_attempts_;
        public: const Presence<std::vector<Attempt>>& attempts() const;
        private: mutable std::shared_ptr<const Presence<BatchSetting>> cached_batch_setting_;
        public: const Presence<BatchSetting>& batch_setting() const;
        private: mutable std::shared_ptr<const Presence<BatchWarning>> cached_batch_warning_;
        public: const Presence<BatchWarning>& batch_warning() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_cached_;
        public: const Presence<bool>& cached() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_context_sha256_;
        public: const Presence<std::string>& context_sha256() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_failed_questions_;
        public: const Presence<uint64_t>& failed_questions() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Observation>>> cached_observations_;
        public: const Presence<std::vector<Observation>>& observations() const;
        private: mutable std::shared_ptr<const Presence<Origin>> cached_origin_;
        public: const Presence<Origin>& origin() const;
        private: mutable std::shared_ptr<const Presence<ProfileWarning>> cached_profile_warning_;
        public: const Presence<ProfileWarning>& profile_warning() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_question_sha256_;
        public: const Presence<std::string>& question_sha256() const;
        private: mutable std::shared_ptr<const Presence<std::vector<QuestionSource>>> cached_question_sources_;
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_questions_sha256_;
        public: const Presence<std::string>& questions_sha256() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_requests_;
        public: const Presence<std::vector<std::string>>& requests() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_requests_sent_;
        public: const Presence<uint64_t>& requests_sent() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_tool_;
        public: const Presence<std::string>& tool() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_url_;
        public: const Presence<std::string>& url() const;
        private: mutable std::shared_ptr<const Presence<Usage>> cached_usage_;
        public: const Presence<Usage>& usage() const;
    };
    class ObjectRoot : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, InputPropertyType>>>> cached_properties_;
        public: const Presence<std::vector<std::pair<std::string, InputPropertyType>>>& properties() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_required_;
        public: const Presence<std::vector<std::string>>& required() const;
        private: mutable std::shared_ptr<const Presence<ObjectType>> cached_type_;
        public: const Presence<ObjectType>& type() const;
    };
    class ObjectType : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class Observation : public Node {
        public: using Node::Node;
        std::optional<ObservationObservationId> as_ObservationObservationId() const;
        std::optional<ObservationFailureId> as_ObservationFailureId() const;
    };
    class ObservationId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class ObservationFailureId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<FailureId>> cached_failure_id_;
        public: const Presence<FailureId>& failure_id() const;
    };
    class ObservationObservationId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<ObservationId>> cached_observation_id_;
        public: const Presence<ObservationId>& observation_id() const;
    };
    class Origin : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class PersistenceObservation : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_advice_;
        public: const Presence<std::string>& advice() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_observed_at_;
        public: const Presence<std::string>& observed_at() const;
        private: mutable std::shared_ptr<const Presence<UsagePersistence>> cached_state_;
        public: const Presence<UsagePersistence>& state() const;
    };
    class PhysicalSource : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_file_;
        public: const Presence<std::string>& file() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_first_line_;
        public: const Presence<uint64_t>& first_line() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_line_;
        public: const Presence<uint64_t>& last_line() const;
    };
    class Position : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_file_;
        public: const Presence<std::string>& file() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_first_;
        public: const Presence<uint64_t>& first() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_images_;
        public: const Presence<std::vector<std::string>>& images() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_;
        public: const Presence<uint64_t>& last() const;
    };
    class QuestionName : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class QuestionSource : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_answered_by_;
        public: const Presence<std::string>& answered_by() const;
        private: mutable std::shared_ptr<const Presence<uint32_t>> cached_batch_size_;
        public: const Presence<uint32_t>& batch_size() const;
        private: mutable std::shared_ptr<const Presence<Origin>> cached_origin_;
        public: const Presence<Origin>& origin() const;
    };
    class RankMember : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
        private: mutable std::shared_ptr<const Presence<RankMemberResult>> cached_result_;
        public: const Presence<RankMemberResult>& result() const;
    };
    class RankMemberResult : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Answer>> cached_answer_;
        public: const Presence<Answer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Image>>> cached_images_;
        public: const Presence<std::vector<Image>>& images() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
        private: mutable std::shared_ptr<const Presence<std::nullptr_t>> cached_threshold_;
        public: const Presence<std::nullptr_t>& threshold() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_value_;
        public: const Presence<uint64_t>& value() const;
    };
    class ReadableQuestion : public Node {
        public: using Node::Node;
        std::optional<ReadableQuestionDecide> as_ReadableQuestionDecide() const;
        std::optional<ReadableQuestionChoose> as_ReadableQuestionChoose() const;
        std::optional<ReadableQuestionTag> as_ReadableQuestionTag() const;
        std::optional<ReadableQuestionScore> as_ReadableQuestionScore() const;
    };
    class ReadableQuestion2 : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Batch>> cached_batch_;
        public: const Presence<Batch>& batch() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_context_schema_;
        public: const Presence<InputDeclaration>& context_schema() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_item_schema_;
        public: const Presence<InputDeclaration>& item_schema() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Label>>> cached_label_details_;
        public: const Presence<std::vector<Label>>& label_details() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<QuestionName>> cached_name_;
        public: const Presence<QuestionName>& name() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_none_;
        public: const Presence<bool>& none() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_on_;
        public: const Presence<std::vector<std::string>>& on() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_profile_;
        public: const Presence<std::string>& profile() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_text_;
        public: const Presence<Json>& text() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_verb_;
        public: const Presence<std::string>& verb() const;
        private: mutable std::shared_ptr<const Presence<WordingVersion>> cached_wording_version_;
        public: const Presence<WordingVersion>& wording_version() const;
    };
    class ReadableQuestion3 : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Batch>> cached_batch_;
        public: const Presence<Batch>& batch() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_context_schema_;
        public: const Presence<InputDeclaration>& context_schema() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_entity_definition_;
        public: const Presence<Json>& entity_definition() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_instructions_;
        public: const Presence<Json>& instructions() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_item_schema_;
        public: const Presence<InputDeclaration>& item_schema() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, Json>>>> cached_kinds_;
        public: const Presence<std::vector<std::pair<std::string, Json>>>& kinds() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Label>>> cached_label_details_;
        public: const Presence<std::vector<Label>>& label_details() const;
        private: mutable std::shared_ptr<const Presence<RecognitionMode>> cached_mode_;
        public: const Presence<RecognitionMode>& mode() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<QuestionName>> cached_name_;
        public: const Presence<QuestionName>& name() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_on_;
        public: const Presence<std::vector<std::string>>& on() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_profile_;
        public: const Presence<std::string>& profile() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_relation_threshold_;
        public: const Presence<Threshold>& relation_threshold() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RelationRule>>> cached_relations_;
        public: const Presence<std::vector<RelationRule>>& relations() const;
        private: mutable std::shared_ptr<const Presence<uint32_t>> cached_snippet_pieces_;
        public: const Presence<uint32_t>& snippet_pieces() const;
        private: mutable std::shared_ptr<const Presence<RecognitionStageContext>> cached_stage_context_;
        public: const Presence<RecognitionStageContext>& stage_context() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<Verb>> cached_verb_;
        public: const Presence<Verb>& verb() const;
        private: mutable std::shared_ptr<const Presence<WordingVersion>> cached_wording_version_;
        public: const Presence<WordingVersion>& wording_version() const;
    };
    class ReadableQuestion4 : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Batch>> cached_batch_;
        public: const Presence<Batch>& batch() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_context_schema_;
        public: const Presence<InputDeclaration>& context_schema() const;
        private: mutable std::shared_ptr<const Presence<RelateFields>> cached_fields_;
        public: const Presence<RelateFields>& fields() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_item_schema_;
        public: const Presence<InputDeclaration>& item_schema() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Label>>> cached_label_details_;
        public: const Presence<std::vector<Label>>& label_details() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<QuestionName>> cached_name_;
        public: const Presence<QuestionName>& name() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_on_;
        public: const Presence<std::vector<std::string>>& on() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_profile_;
        public: const Presence<std::string>& profile() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RelationRule>>> cached_relations_;
        public: const Presence<std::vector<RelationRule>>& relations() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_verb_;
        public: const Presence<std::string>& verb() const;
        private: mutable std::shared_ptr<const Presence<WordingVersion>> cached_wording_version_;
        public: const Presence<WordingVersion>& wording_version() const;
    };
    class ReadableQuestionChoose : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Batch>> cached_batch_;
        public: const Presence<Batch>& batch() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_context_schema_;
        public: const Presence<InputDeclaration>& context_schema() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_item_schema_;
        public: const Presence<InputDeclaration>& item_schema() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Label>>> cached_label_details_;
        public: const Presence<std::vector<Label>>& label_details() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<QuestionName>> cached_name_;
        public: const Presence<QuestionName>& name() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_on_;
        public: const Presence<std::vector<std::string>>& on() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_profile_;
        public: const Presence<std::string>& profile() const;
        private: mutable std::shared_ptr<const Presence<WordingVersion>> cached_wording_version_;
        public: const Presence<WordingVersion>& wording_version() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_options_;
        public: const Presence<std::vector<std::string>>& options() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_text_;
        public: const Presence<Json>& text() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_verb_;
        public: const Presence<std::string>& verb() const;
    };
    class ReadableQuestionDecide : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Batch>> cached_batch_;
        public: const Presence<Batch>& batch() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_context_schema_;
        public: const Presence<InputDeclaration>& context_schema() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_item_schema_;
        public: const Presence<InputDeclaration>& item_schema() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Label>>> cached_label_details_;
        public: const Presence<std::vector<Label>>& label_details() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<QuestionName>> cached_name_;
        public: const Presence<QuestionName>& name() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_on_;
        public: const Presence<std::vector<std::string>>& on() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_profile_;
        public: const Presence<std::string>& profile() const;
        private: mutable std::shared_ptr<const Presence<WordingVersion>> cached_wording_version_;
        public: const Presence<WordingVersion>& wording_version() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_false__;
        public: const Presence<Json>& false_() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_text_;
        public: const Presence<Json>& text() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_true__;
        public: const Presence<Json>& true_() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_verb_;
        public: const Presence<std::string>& verb() const;
    };
    class ReadableQuestionScore : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Batch>> cached_batch_;
        public: const Presence<Batch>& batch() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_context_schema_;
        public: const Presence<InputDeclaration>& context_schema() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_item_schema_;
        public: const Presence<InputDeclaration>& item_schema() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Label>>> cached_label_details_;
        public: const Presence<std::vector<Label>>& label_details() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<QuestionName>> cached_name_;
        public: const Presence<QuestionName>& name() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_on_;
        public: const Presence<std::vector<std::string>>& on() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_profile_;
        public: const Presence<std::string>& profile() const;
        private: mutable std::shared_ptr<const Presence<WordingVersion>> cached_wording_version_;
        public: const Presence<WordingVersion>& wording_version() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_levels_;
        public: const Presence<std::vector<std::string>>& levels() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_text_;
        public: const Presence<Json>& text() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_verb_;
        public: const Presence<std::string>& verb() const;
    };
    class ReadableQuestionTag : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Batch>> cached_batch_;
        public: const Presence<Batch>& batch() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_context_schema_;
        public: const Presence<InputDeclaration>& context_schema() const;
        private: mutable std::shared_ptr<const Presence<InputDeclaration>> cached_item_schema_;
        public: const Presence<InputDeclaration>& item_schema() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Label>>> cached_label_details_;
        public: const Presence<std::vector<Label>>& label_details() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<QuestionName>> cached_name_;
        public: const Presence<QuestionName>& name() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_on_;
        public: const Presence<std::vector<std::string>>& on() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_profile_;
        public: const Presence<std::string>& profile() const;
        private: mutable std::shared_ptr<const Presence<WordingVersion>> cached_wording_version_;
        public: const Presence<WordingVersion>& wording_version() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_labels_;
        public: const Presence<std::vector<std::string>>& labels() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_text_;
        public: const Presence<Json>& text() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_verb_;
        public: const Presence<std::string>& verb() const;
    };
    class Recognition : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<RecognitionOdds>> cached_answer_;
        public: const Presence<RecognitionOdds>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_file_;
        public: const Presence<std::string>& file() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_first_line_;
        public: const Presence<uint64_t>& first_line() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_line_;
        public: const Presence<uint64_t>& last_line() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<Position>> cached_position_;
        public: const Presence<Position>& position() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion3>> cached_question_;
        public: const Presence<ReadableQuestion3>& question() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
        private: mutable std::shared_ptr<const Presence<Recognize>> cached_value_;
        public: const Presence<Recognize>& value() const;
    };
    class RecognitionEdgeDocument : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<bool>> cached_either_;
        public: const Presence<bool>& either() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_relation_;
        public: const Presence<std::string>& relation() const;
        private: mutable std::shared_ptr<const Presence<Entity>> cached_source_;
        public: const Presence<Entity>& source() const;
        private: mutable std::shared_ptr<const Presence<Entity>> cached_target_;
        public: const Presence<Entity>& target() const;
    };
    class RecognitionMode : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class RecognitionOdds : public Node {
        public: using Node::Node;
        std::optional<RecognitionOddsRecognizeAnswer> as_RecognitionOddsRecognizeAnswer() const;
        std::optional<RecognitionOddsBoundaryOdds> as_RecognitionOddsBoundaryOdds() const;
    };
    class RecognitionOddsBoundaryOdds : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<PieceOdds>>> cached_pieces_;
        public: const Presence<std::vector<PieceOdds>>& pieces() const;
        private: mutable std::shared_ptr<const Presence<std::vector<BoundaryProposal>>> cached_proposals_;
        public: const Presence<std::vector<BoundaryProposal>>& proposals() const;
    };
    class RecognitionOddsRecognizeAnswer : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<NameOdds>>> cached_names_;
        public: const Presence<std::vector<NameOdds>>& names() const;
        private: mutable std::shared_ptr<const Presence<std::vector<PairOdds>>> cached_pairs_;
        public: const Presence<std::vector<PairOdds>>& pairs() const;
        private: mutable std::shared_ptr<const Presence<std::vector<PieceOdds>>> cached_pieces_;
        public: const Presence<std::vector<PieceOdds>>& pieces() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RecognitionProposal>>> cached_proposals_;
        public: const Presence<std::vector<RecognitionProposal>>& proposals() const;
    };
    class RecognitionProposal : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_end_;
        public: const Presence<uint64_t>& end() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_kept_;
        public: const Presence<bool>& kept() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<Place>> cached_selected_;
        public: const Presence<Place>& selected() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_span_probability_;
        public: const Presence<double>& span_probability() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_start_;
        public: const Presence<uint64_t>& start() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_strength_;
        public: const Presence<double>& strength() const;
    };
    class RecognitionStageContext : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_boundary_;
        public: const Presence<std::string>& boundary() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_edge_;
        public: const Presence<std::string>& kind_edge() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_relation_;
        public: const Presence<std::string>& relation() const;
    };
    class Relation : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Answers>> cached_answer_;
        public: const Presence<Answers>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_file_;
        public: const Presence<std::string>& file() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_first_line_;
        public: const Presence<uint64_t>& first_line() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<std::vector<SessionInputSource>>> cached_input_sources_;
        public: const Presence<std::vector<SessionInputSource>>& input_sources() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_line_;
        public: const Presence<uint64_t>& last_line() const;
        private: mutable std::shared_ptr<const Presence<Meta>> cached_meta_;
        public: const Presence<Meta>& meta() const;
        private: mutable std::shared_ptr<const Presence<Position>> cached_position_;
        public: const Presence<Position>& position() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion4>> cached_question_;
        public: const Presence<ReadableQuestion4>& question() const;
        private: mutable std::shared_ptr<const Presence<Version>> cached_schema_;
        public: const Presence<Version>& schema() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RelatedEntityEdge>>> cached_value_;
        public: const Presence<std::vector<RelatedEntityEdge>>& value() const;
    };
    class RelationDirection : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class RelationEndpoint : public Node {
        public: using Node::Node;
        std::optional<RelationEndpointRelatedEntity> as_RelationEndpointRelatedEntity() const;
        std::optional<RelationEndpointSource> as_RelationEndpointSource() const;
    };
    class RelationEndpointRelatedEntity : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
    };
    class RelationEndpointSource : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_file_;
        public: const Presence<std::string>& file() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_first_line_;
        public: const Presence<uint64_t>& first_line() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_line_;
        public: const Presence<uint64_t>& last_line() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_ordinal_;
        public: const Presence<uint64_t>& ordinal() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_record_;
        public: const Presence<Json>& record() const;
    };
    class RelationMember : public Node {
        public: using Node::Node;
        std::optional<RelationMemberAnswerId> as_RelationMemberAnswerId() const;
        std::optional<RelationMemberFailureId> as_RelationMemberFailureId() const;
    };
    class RelationMemberAnswerId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<RelationDirection>> cached_direction_;
        public: const Presence<RelationDirection>& direction() const;
        private: mutable std::shared_ptr<const Presence<RelationMethod>> cached_method_;
        public: const Presence<RelationMethod>& method() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Observation>>> cached_observations_;
        public: const Presence<std::vector<Observation>>& observations() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::vector<QuestionSource>>> cached_question_sources_;
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_reads_;
        public: const Presence<std::string>& reads() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_relation_;
        public: const Presence<std::string>& relation() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_request_;
        public: const Presence<std::string>& request() const;
        private: mutable std::shared_ptr<const Presence<RelatedEntity>> cached_source_;
        public: const Presence<RelatedEntity>& source() const;
        private: mutable std::shared_ptr<const Presence<RelatedEntity>> cached_target_;
        public: const Presence<RelatedEntity>& target() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<Usage>> cached_usage_;
        public: const Presence<Usage>& usage() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_accepted_;
        public: const Presence<bool>& accepted() const;
        private: mutable std::shared_ptr<const Presence<Answer>> cached_answer_;
        public: const Presence<Answer>& answer() const;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
        std::string source_name() const;
    };
    class RelationMemberFailureId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<RelationDirection>> cached_direction_;
        public: const Presence<RelationDirection>& direction() const;
        private: mutable std::shared_ptr<const Presence<RelationMethod>> cached_method_;
        public: const Presence<RelationMethod>& method() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Observation>>> cached_observations_;
        public: const Presence<std::vector<Observation>>& observations() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::vector<QuestionSource>>> cached_question_sources_;
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_reads_;
        public: const Presence<std::string>& reads() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_relation_;
        public: const Presence<std::string>& relation() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_request_;
        public: const Presence<std::string>& request() const;
        private: mutable std::shared_ptr<const Presence<RelatedEntity>> cached_source_;
        public: const Presence<RelatedEntity>& source() const;
        private: mutable std::shared_ptr<const Presence<RelatedEntity>> cached_target_;
        public: const Presence<RelatedEntity>& target() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<Usage>> cached_usage_;
        public: const Presence<Usage>& usage() const;
        private: mutable std::shared_ptr<const Presence<Failure>> cached_failure_;
        public: const Presence<Failure>& failure() const;
        private: mutable std::shared_ptr<const Presence<FailureId>> cached_failure_id_;
        public: const Presence<FailureId>& failure_id() const;
        std::string source_name() const;
    };
    class RelationMethod : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class RequestFunction : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class SdkRequestId : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class SendBudgetDenial : public Node {
        public: using Node::Node;
        std::optional<SendBudgetDenialBeforeFirstSend> as_SendBudgetDenialBeforeFirstSend() const;
        std::optional<SendBudgetDenialBeforeAdditionalSend> as_SendBudgetDenialBeforeAdditionalSend() const;
        std::optional<SendBudgetDenialBeforeRetry> as_SendBudgetDenialBeforeRetry() const;
    };
    class SendBudgetDenialBeforeAdditionalSend : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
    };
    class SendBudgetDenialBeforeFirstSend : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
    };
    class SendBudgetDenialBeforeRetry : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_status_;
        public: const Presence<uint64_t>& last_status() const;
    };
    class StopCause : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class Stopped : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_at_;
        public: const Presence<uint64_t>& at() const;
        private: mutable std::shared_ptr<const Presence<StopCause>> cached_cause_;
        public: const Presence<StopCause>& cause() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_retryable_;
        public: const Presence<bool>& retryable() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_status_;
        public: const Presence<uint64_t>& status() const;
    };
    class StringRoot : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<StringType>> cached_type_;
        public: const Presence<StringType>& type() const;
    };
    class StringType : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class Usage : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_input_tokens_;
        public: const Presence<uint64_t>& input_tokens() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_output_tokens_;
        public: const Presence<uint64_t>& output_tokens() const;
    };
    class UsagePersistence : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class Verb : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class Version : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class WordingVersion : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const uint32_t> cached_value_;
        public: const uint32_t& value() const;
    };
    class AnnotatedField : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>,
        double, Failed>> cached_value_;
        public: const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double,
        Failed>& value() const;
    };
    class AnnotatedRow : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::vector<std::pair<std::string, AnnotatedField>>> cached_value_;
        public: const std::vector<std::pair<std::string, AnnotatedField>>& value() const;
    };
    class Answer : public Node {
        public: using Node::Node;
        std::optional<AnswerYesNo> as_AnswerYesNo() const;
        std::optional<AnswerChoice> as_AnswerChoice() const;
        std::optional<AnswerTag> as_AnswerTag() const;
        std::optional<AnswerScore> as_AnswerScore() const;
    };
    class AnswerChoice : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<double>> cached_confidence_;
        public: const Presence<double>& confidence() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_pick_;
        public: const Presence<std::string>& pick() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>> cached_probabilities_;
        public: const Presence<std::vector<std::pair<std::string, double>>>& probabilities() const;
    };
    class AnswerScore : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<double>> cached_confidence_;
        public: const Presence<double>& confidence() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_level_;
        public: const Presence<std::string>& level() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>> cached_probabilities_;
        public: const Presence<std::vector<std::pair<std::string, double>>>& probabilities() const;
    };
    class AnswerTag : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>> cached_probabilities_;
        public: const Presence<std::vector<std::pair<std::string, double>>>& probabilities() const;
    };
    class AnswerYesNo : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
    };
    class AttemptOutcome : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class BatchSetting : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::variant<uint64_t, std::string>> cached_value_;
        public: const std::variant<uint64_t, std::string>& value() const;
    };
    class BatchWarning : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<BatchSetting>> cached_running_;
        public: const Presence<BatchSetting>& running() const;
        private: mutable std::shared_ptr<const Presence<BatchSetting>> cached_tuned_for_;
        public: const Presence<BatchSetting>& tuned_for() const;
    };
    class Entity : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_end_;
        public: const Presence<uint64_t>& end() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_file_;
        public: const Presence<std::string>& file() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_first_line_;
        public: const Presence<uint64_t>& first_line() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_line_;
        public: const Presence<uint64_t>& last_line() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_length_;
        public: const Presence<uint64_t>& length() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_start_;
        public: const Presence<uint64_t>& start() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_strength_;
        public: const Presence<double>& strength() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_text_;
        public: const Presence<std::string>& text() const;
    };
    class EntityEdge : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<bool>> cached_either_;
        public: const Presence<bool>& either() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_relation_;
        public: const Presence<std::string>& relation() const;
        private: mutable std::shared_ptr<const Presence<Entity>> cached_source_;
        public: const Presence<Entity>& source() const;
        private: mutable std::shared_ptr<const Presence<Entity>> cached_target_;
        public: const Presence<Entity>& target() const;
    };
    class Failed : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Failure>> cached_failed_;
        public: const Presence<Failure>& failed() const;
    };
    class Failure : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<FailureCause>> cached_cause_;
        public: const Presence<FailureCause>& cause() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
    };
    class FailureCause : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class FailureKind : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::string> cached_value_;
        public: const std::string& value() const;
    };
    class FindAnswer : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<double>> cached_confidence_;
        public: const Presence<double>& confidence() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_pick_;
        public: const Presence<std::string>& pick() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>> cached_probabilities_;
        public: const Presence<std::vector<std::pair<std::string, double>>>& probabilities() const;
    };
    class NameOdds : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>> cached_edges_;
        public: const Presence<std::vector<std::pair<std::string, double>>>& edges() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_end_;
        public: const Presence<uint64_t>& end() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>> cached_kinds_;
        public: const Presence<std::vector<std::pair<std::string, double>>>& kinds() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_start_;
        public: const Presence<uint64_t>& start() const;
    };
    class PairOdds : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_relation_;
        public: const Presence<std::string>& relation() const;
        private: mutable std::shared_ptr<const Presence<Place>> cached_source_;
        public: const Presence<Place>& source() const;
        private: mutable std::shared_ptr<const Presence<Place>> cached_target_;
        public: const Presence<Place>& target() const;
    };
    class PieceOdds : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_end_;
        public: const Presence<uint64_t>& end() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_start_;
        public: const Presence<uint64_t>& start() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>> cached_tags_;
        public: const Presence<std::vector<std::pair<std::string, double>>>& tags() const;
    };
    class Place : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_end_;
        public: const Presence<uint64_t>& end() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_start_;
        public: const Presence<uint64_t>& start() const;
    };
    class Plan : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_estimated_bytes_;
        public: const Presence<uint64_t>& estimated_bytes() const;
        private: mutable std::shared_ptr<const Presence<TokenBand>> cached_estimated_input_tokens_;
        public: const Presence<TokenBand>& estimated_input_tokens() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_first_body_utf8_;
        public: const Presence<std::string>& first_body_utf8() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_largest_request_bytes_;
        public: const Presence<uint64_t>& largest_request_bytes() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_largest_request_estimated_input_tokens_;
        public: const Presence<uint64_t>& largest_request_estimated_input_tokens() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_records_;
        public: const Presence<uint64_t>& records() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_requests_;
        public: const Presence<uint64_t>& requests() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_token_estimate_method_;
        public: const Presence<std::string>& token_estimate_method() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_upper_bound_;
        public: const Presence<bool>& upper_bound() const;
    };
    class ProfileWarning : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_running_;
        public: const Presence<std::string>& running() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_tuned_for_;
        public: const Presence<std::string>& tuned_for() const;
    };
    class Recognize : public Node {
        public: using Node::Node;
        std::optional<RecognizeEntities> as_RecognizeEntities() const;
        std::optional<RecognizeModeProposals> as_RecognizeModeProposals() const;
    };
    class RecognizeAnswer : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<NameOdds>>> cached_names_;
        public: const Presence<std::vector<NameOdds>>& names() const;
        private: mutable std::shared_ptr<const Presence<std::vector<PairOdds>>> cached_pairs_;
        public: const Presence<std::vector<PairOdds>>& pairs() const;
        private: mutable std::shared_ptr<const Presence<std::vector<PieceOdds>>> cached_pieces_;
        public: const Presence<std::vector<PieceOdds>>& pieces() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RecognitionProposal>>> cached_proposals_;
        public: const Presence<std::vector<RecognitionProposal>>& proposals() const;
    };
    class RecognizeEntities : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<Entity>>> cached_entities_;
        public: const Presence<std::vector<Entity>>& entities() const;
        private: mutable std::shared_ptr<const Presence<std::vector<EntityEdge>>> cached_relations_;
        public: const Presence<std::vector<EntityEdge>>& relations() const;
    };
    class RecognizeModeProposals : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<BoundaryMode>> cached_mode_;
        public: const Presence<BoundaryMode>& mode() const;
        private: mutable std::shared_ptr<const Presence<std::vector<BoundaryProposal>>> cached_proposals_;
        public: const Presence<std::vector<BoundaryProposal>>& proposals() const;
    };
    class RelateFields : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
    };
    class RelatedEntity : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
    };
    class RelatedEntityEdge : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<bool>> cached_either_;
        public: const Presence<bool>& either() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_relation_;
        public: const Presence<std::string>& relation() const;
        private: mutable std::shared_ptr<const Presence<RelationEndpoint>> cached_source_;
        public: const Presence<RelationEndpoint>& source() const;
        private: mutable std::shared_ptr<const Presence<RelationEndpoint>> cached_target_;
        public: const Presence<RelationEndpoint>& target() const;
        std::string source_name() const;
        std::string target_name() const;
    };
    class RelationRule : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<bool>> cached_either_;
        public: const Presence<bool>& either() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_reads_;
        public: const Presence<std::string>& reads() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_single_;
        public: const Presence<bool>& single() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_source_;
        public: const Presence<std::string>& source() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_target_;
        public: const Presence<std::string>& target() const;
    };
    class SessionAnnotation : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
        private: mutable std::shared_ptr<const Presence<AnnotationValue>> cached_value_;
        public: const Presence<AnnotationValue>& value() const;
    };
    class SessionInputSource : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_source_;
        public: const Presence<PhysicalSource>& source() const;
    };
    class SessionJudgment : public Node {
        public: using Node::Node;
        std::optional<SessionJudgmentDecision> as_SessionJudgmentDecision() const;
        std::optional<SessionJudgmentChoice> as_SessionJudgmentChoice() const;
        std::optional<SessionJudgmentScore> as_SessionJudgmentScore() const;
        std::optional<SessionJudgmentTags> as_SessionJudgmentTags() const;
    };
    class SessionJudgmentChoice : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_value_;
        public: const Presence<std::string>& value() const;
    };
    class SessionJudgmentDecision : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_value_;
        public: const Presence<bool>& value() const;
    };
    class SessionJudgmentScore : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_value_;
        public: const Presence<double>& value() const;
    };
    class SessionJudgmentTags : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_value_;
        public: const Presence<std::vector<std::string>>& value() const;
    };
    class SessionNamedProbability : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
    };
    class SessionObservation : public Node {
        public: using Node::Node;
        std::optional<SessionObservationQuestion> as_SessionObservationQuestion() const;
        std::optional<SessionObservationRow> as_SessionObservationRow() const;
    };
    class SessionObservationQuestion : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<SessionQuestionDetail>> cached_detail_;
        public: const Presence<SessionQuestionDetail>& detail() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_member_;
        public: const Presence<std::string>& member() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_position_;
        public: const Presence<uint64_t>& position() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_stage_;
        public: const Presence<std::string>& stage() const;
    };
    class SessionObservationRow : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_index_;
        public: const Presence<uint64_t>& index() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<SessionObservedRow>> cached_value_;
        public: const Presence<SessionObservedRow>& value() const;
    };
    class SessionObservedRow : public Node {
        public: using Node::Node;
        std::optional<SessionObservedRowJudgment> as_SessionObservedRowJudgment() const;
        std::optional<SessionObservedRowAnnotated> as_SessionObservedRowAnnotated() const;
        std::optional<SessionObservedRowRecognized> as_SessionObservedRowRecognized() const;
        std::optional<SessionObservedRowFind> as_SessionObservedRowFind() const;
        std::optional<SessionObservedRowRelations> as_SessionObservedRowRelations() const;
    };
    class SessionObservedRowAnnotated : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<SessionAnnotation>>> cached_value_;
        public: const Presence<std::vector<SessionAnnotation>>& value() const;
    };
    class SessionObservedRowFind : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_value_;
        public: const Presence<uint64_t>& value() const;
    };
    class SessionObservedRowJudgment : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<SessionJudgment>> cached_value_;
        public: const Presence<SessionJudgment>& value() const;
    };
    class SessionObservedRowRecognized : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<SessionRecognition>> cached_value_;
        public: const Presence<SessionRecognition>& value() const;
    };
    class SessionObservedRowRelations : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<SessionRelationEdge>>> cached_value_;
        public: const Presence<std::vector<SessionRelationEdge>>& value() const;
    };
    class SessionPacket : public Node {
        public: using Node::Node;
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
    class SessionPacketAnnotateAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Annotation>>> cached_value_;
        public: const Presence<std::vector<Annotation>>& value() const;
    };
    class SessionPacketAnnotateRow : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<Annotation>> cached_value_;
        public: const Presence<Annotation>& value() const;
    };
    class SessionPacketChooseAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<AtomicNullableString>>> cached_value_;
        public: const Presence<std::vector<AtomicNullableString>>& value() const;
    };
    class SessionPacketChooseRow : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<AtomicNullableString>> cached_value_;
        public: const Presence<AtomicNullableString>& value() const;
    };
    class SessionPacketDecideAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<AtomicDecideValue>>> cached_value_;
        public: const Presence<std::vector<AtomicDecideValue>>& value() const;
    };
    class SessionPacketDecideRow : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<AtomicDecideValue>> cached_value_;
        public: const Presence<AtomicDecideValue>& value() const;
    };
    class SessionPacketFilterAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<AtomicBoolean>>> cached_value_;
        public: const Presence<std::vector<AtomicBoolean>>& value() const;
    };
    class SessionPacketFilterRow : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<AtomicBoolean>> cached_value_;
        public: const Presence<AtomicBoolean>& value() const;
    };
    class SessionPacketFindAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<Find>> cached_value_;
        public: const Presence<Find>& value() const;
    };
    class SessionPacketObservation : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<RequestFunction>> cached_function_;
        public: const Presence<RequestFunction>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<SessionObservation>> cached_value_;
        public: const Presence<SessionObservation>& value() const;
    };
    class SessionPacketRankAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<AtomicNonZeroUsize>>> cached_value_;
        public: const Presence<std::vector<AtomicNonZeroUsize>>& value() const;
    };
    class SessionPacketRecognizeAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Recognition>>> cached_value_;
        public: const Presence<std::vector<Recognition>>& value() const;
    };
    class SessionPacketRelateAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<Relation>> cached_value_;
        public: const Presence<Relation>& value() const;
    };
    class SessionPacketScoreAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<AtomicDouble>>> cached_value_;
        public: const Presence<std::vector<AtomicDouble>>& value() const;
    };
    class SessionPacketScoreRow : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<AtomicDouble>> cached_value_;
        public: const Presence<AtomicDouble>& value() const;
    };
    class SessionPacketTagAggregate : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<AtomicArrayOfString>>> cached_value_;
        public: const Presence<std::vector<AtomicArrayOfString>>& value() const;
    };
    class SessionPacketTagRow : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_function_;
        public: const Presence<std::string>& function() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<AtomicArrayOfString>> cached_value_;
        public: const Presence<AtomicArrayOfString>& value() const;
    };
    class SessionPacketTerminal : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<Facts>> cached_facts_;
        public: const Presence<Facts>& facts() const;
        private: mutable std::shared_ptr<const Presence<CallError>> cached_failure_;
        public: const Presence<CallError>& failure() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
    };
    class SessionProbabilities : public Node {
        public: using Node::Node;
        std::optional<SessionProbabilitiesYesNo> as_SessionProbabilitiesYesNo() const;
        std::optional<SessionProbabilitiesNamed> as_SessionProbabilitiesNamed() const;
    };
    class SessionProbabilitiesNamed : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<std::vector<SessionNamedProbability>>> cached_value_;
        public: const Presence<std::vector<SessionNamedProbability>>& value() const;
    };
    class SessionProbabilitiesYesNo : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_value_;
        public: const Presence<double>& value() const;
    };
    class SessionQuestionDetail : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<AnswerId>> cached_answer_id_;
        public: const Presence<AnswerId>& answer_id() const;
        private: mutable std::shared_ptr<const Presence<bool>> cached_cached_;
        public: const Presence<bool>& cached() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_confidence_;
        public: const Presence<double>& confidence() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_failed_questions_;
        public: const Presence<uint64_t>& failed_questions() const;
        private: mutable std::shared_ptr<const Presence<Failure>> cached_failure_;
        public: const Presence<Failure>& failure() const;
        private: mutable std::shared_ptr<const Presence<FailureId>> cached_failure_id_;
        public: const Presence<FailureId>& failure_id() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_input_;
        public: const Presence<Json>& input() const;
        private: mutable std::shared_ptr<const Presence<PhysicalSource>> cached_input_source_;
        public: const Presence<PhysicalSource>& input_source() const;
        private: mutable std::shared_ptr<const Presence<std::vector<SessionInputSource>>> cached_input_sources_;
        public: const Presence<std::vector<SessionInputSource>>& input_sources() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Json>>> cached_inputs_;
        public: const Presence<std::vector<Json>>& inputs() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_model_;
        public: const Presence<std::string>& model() const;
        private: mutable std::shared_ptr<const Presence<std::vector<Observation>>> cached_observations_;
        public: const Presence<std::vector<Observation>>& observations() const;
        private: mutable std::shared_ptr<const Presence<SessionProbabilities>> cached_probabilities_;
        public: const Presence<SessionProbabilities>& probabilities() const;
        private: mutable std::shared_ptr<const Presence<ReadableQuestion>> cached_question_;
        public: const Presence<ReadableQuestion>& question() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_question_sha256_;
        public: const Presence<std::string>& question_sha256() const;
        private: mutable std::shared_ptr<const Presence<std::vector<QuestionSource>>> cached_question_sources_;
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_raw_pick_;
        public: const Presence<std::string>& raw_pick() const;
        private: mutable std::shared_ptr<const Presence<Usage>> cached_reported_usage_;
        public: const Presence<Usage>& reported_usage() const;
        private: mutable std::shared_ptr<const Presence<std::vector<std::string>>> cached_requests_;
        public: const Presence<std::vector<std::string>>& requests() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_requests_sent_;
        public: const Presence<uint64_t>& requests_sent() const;
        private: mutable std::shared_ptr<const Presence<Threshold>> cached_threshold_;
        public: const Presence<Threshold>& threshold() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_url_;
        public: const Presence<std::string>& url() const;
        private: mutable std::shared_ptr<const Presence<TokenUsage>> cached_usage_;
        public: const Presence<TokenUsage>& usage() const;
        private: mutable std::shared_ptr<const Presence<Value>> cached_value_;
        public: const Presence<Value>& value() const;
    };
    class SessionRecognition : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::vector<Entity>>> cached_entities_;
        public: const Presence<std::vector<Entity>>& entities() const;
        private: mutable std::shared_ptr<const Presence<RecognitionMode>> cached_mode_;
        public: const Presence<RecognitionMode>& mode() const;
        private: mutable std::shared_ptr<const Presence<std::vector<BoundaryProposal>>> cached_proposals_;
        public: const Presence<std::vector<BoundaryProposal>>& proposals() const;
        private: mutable std::shared_ptr<const Presence<std::vector<RecognitionEdgeDocument>>> cached_relations_;
        public: const Presence<std::vector<RecognitionEdgeDocument>>& relations() const;
    };
    class SessionRelationEdge : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<bool>> cached_either_;
        public: const Presence<bool>& either() const;
        private: mutable std::shared_ptr<const Presence<double>> cached_probability_;
        public: const Presence<double>& probability() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_relation_;
        public: const Presence<std::string>& relation() const;
        private: mutable std::shared_ptr<const Presence<EntityDocument>> cached_source_;
        public: const Presence<EntityDocument>& source() const;
        private: mutable std::shared_ptr<const Presence<EntityDocument>> cached_target_;
        public: const Presence<EntityDocument>& target() const;
        std::string source_name() const;
        std::string target_name() const;
    };
    class SourceRelationEndpoint : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_file_;
        public: const Presence<std::string>& file() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_first_line_;
        public: const Presence<uint64_t>& first_line() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_kind_;
        public: const Presence<std::string>& kind() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_last_line_;
        public: const Presence<uint64_t>& last_line() const;
        private: mutable std::shared_ptr<const Presence<std::string>> cached_name_;
        public: const Presence<std::string>& name() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_ordinal_;
        public: const Presence<uint64_t>& ordinal() const;
        private: mutable std::shared_ptr<const Presence<Json>> cached_record_;
        public: const Presence<Json>& record() const;
    };
    class Threshold : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::variant<double, std::string>> cached_value_;
        public: const std::variant<double, std::string>& value() const;
    };
    class TokenBand : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_lower_;
        public: const Presence<uint64_t>& lower() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_upper_;
        public: const Presence<uint64_t>& upper() const;
    };
    class TokenUsage : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_input_tokens_;
        public: const Presence<uint64_t>& input_tokens() const;
        private: mutable std::shared_ptr<const Presence<uint64_t>> cached_output_tokens_;
        public: const Presence<uint64_t>& output_tokens() const;
    };
    class Value : public Node {
        public: using Node::Node;
        private: mutable std::shared_ptr<const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>,
        double>> cached_value_;
        public: const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double>& value() const;
    };
    inline const Presence<AnswerId>& Annotation::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<std::pair<std::string, AnnotationMember>>>& Annotation::answers() const {
        if (!cached_answers_) cached_answers_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        AnnotationMember>>>>(detail::member_value<std::vector<std::pair<std::string, AnnotationMember>>>(value_,
        "answers"));
        return *cached_answers_;
    }
    inline const Presence<std::string>& Annotation::file() const {
        if (!cached_file_) cached_file_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "file"));
        return *cached_file_;
    }
    inline const Presence<uint64_t>& Annotation::first_line() const {
        if (!cached_first_line_) cached_first_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "first_line"));
        return *cached_first_line_;
    }
    inline const Presence<uint64_t>& Annotation::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& Annotation::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<uint64_t>& Annotation::last_line() const {
        if (!cached_last_line_) cached_last_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_line"));
        return *cached_last_line_;
    }
    inline const Presence<Meta>& Annotation::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<Position>& Annotation::position() const {
        if (!cached_position_) cached_position_ = std::make_shared<const Presence<Position>>(detail::member_value<Position>(value_,
        "position"));
        return *cached_position_;
    }
    inline const Presence<Version>& Annotation::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<PhysicalSource>& Annotation::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<AnnotatedRow>& Annotation::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<AnnotatedRow>>(detail::member_value<AnnotatedRow>(value_,
        "value"));
        return *cached_value_;
    }
    inline std::optional<AnnotationMemberAnswerId> AnnotationMember::as_AnnotationMemberAnswerId() const {
        if (value_.contains("answer_id")) return AnnotationMemberAnswerId(value_);
        return std::nullopt;
    }
    inline std::optional<AnnotationMemberFailureId> AnnotationMember::as_AnnotationMemberFailureId() const {
        if (value_.contains("failure_id")) return AnnotationMemberFailureId(value_);
        return std::nullopt;
    }
    inline const Presence<Answer>& AnnotationMemberAnswerId::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& AnnotationMemberAnswerId::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<Observation>>& AnnotationMemberAnswerId::observations() const {
        if (!cached_observations_) cached_observations_ = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
        "observations"));
        return *cached_observations_;
    }
    inline const Presence<ReadableQuestion>& AnnotationMemberAnswerId::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::vector<QuestionSource>>& AnnotationMemberAnswerId::question_sources() const {
        if (!cached_question_sources_) cached_question_sources_ = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
        "question_sources"));
        return *cached_question_sources_;
    }
    inline const Presence<std::string>& AnnotationMemberAnswerId::request() const {
        if (!cached_request_) cached_request_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "request"));
        return *cached_request_;
    }
    inline const Presence<Threshold>& AnnotationMemberAnswerId::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<Usage>& AnnotationMemberAnswerId::usage() const {
        if (!cached_usage_) cached_usage_ = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
        "usage"));
        return *cached_usage_;
    }
    inline const Presence<Value>& AnnotationMemberAnswerId::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<Value>>(detail::member_value<Value>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<Failure>& AnnotationMemberFailureId::failure() const {
        if (!cached_failure_) cached_failure_ = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
        "failure"));
        return *cached_failure_;
    }
    inline const Presence<FailureId>& AnnotationMemberFailureId::failure_id() const {
        if (!cached_failure_id_) cached_failure_id_ = std::make_shared<const Presence<FailureId>>(detail::member_value<FailureId>(value_,
        "failure_id"));
        return *cached_failure_id_;
    }
    inline const Presence<std::vector<Observation>>& AnnotationMemberFailureId::observations() const {
        if (!cached_observations_) cached_observations_ = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
        "observations"));
        return *cached_observations_;
    }
    inline const Presence<ReadableQuestion>& AnnotationMemberFailureId::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::vector<QuestionSource>>& AnnotationMemberFailureId::question_sources() const {
        if (!cached_question_sources_) cached_question_sources_ = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
        "question_sources"));
        return *cached_question_sources_;
    }
    inline const Presence<std::string>& AnnotationMemberFailureId::request() const {
        if (!cached_request_) cached_request_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "request"));
        return *cached_request_;
    }
    inline const Presence<Threshold>& AnnotationMemberFailureId::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<Usage>& AnnotationMemberFailureId::usage() const {
        if (!cached_usage_) cached_usage_ = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
        "usage"));
        return *cached_usage_;
    }
    inline std::optional<AnnotationValueDecision> AnnotationValue::as_AnnotationValueDecision() const {
        if (detail::literal(value_, "kind", "decision")) return AnnotationValueDecision(value_);
        return std::nullopt;
    }
    inline std::optional<AnnotationValueChoice> AnnotationValue::as_AnnotationValueChoice() const {
        if (detail::literal(value_, "kind", "choice")) return AnnotationValueChoice(value_);
        return std::nullopt;
    }
    inline std::optional<AnnotationValueScore> AnnotationValue::as_AnnotationValueScore() const {
        if (detail::literal(value_, "kind", "score")) return AnnotationValueScore(value_);
        return std::nullopt;
    }
    inline std::optional<AnnotationValueTags> AnnotationValue::as_AnnotationValueTags() const {
        if (detail::literal(value_, "kind", "tags")) return AnnotationValueTags(value_);
        return std::nullopt;
    }
    inline std::optional<AnnotationValueFailed> AnnotationValue::as_AnnotationValueFailed() const {
        if (detail::literal(value_, "kind", "failed")) return AnnotationValueFailed(value_);
        return std::nullopt;
    }
    inline const Presence<std::string>& AnnotationValueChoice::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& AnnotationValueChoice::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& AnnotationValueDecision::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<bool>& AnnotationValueDecision::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& AnnotationValueFailed::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<Failure>& AnnotationValueFailed::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& AnnotationValueScore::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<double>& AnnotationValueScore::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& AnnotationValueTags::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<std::string>>& AnnotationValueTags::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const std::string& AnswerId::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Presence<std::vector<RelationMember>>& Answers::questions() const {
        if (!cached_questions_) cached_questions_ = std::make_shared<const Presence<std::vector<RelationMember>>>(detail::member_value<std::vector<RelationMember>>(value_,
        "questions"));
        return *cached_questions_;
    }
    inline const Presence<Answer>& AtomicArrayOfString::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& AtomicArrayOfString::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<Image>>& AtomicArrayOfString::images() const {
        if (!cached_images_) cached_images_ = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
        "images"));
        return *cached_images_;
    }
    inline const Presence<uint64_t>& AtomicArrayOfString::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& AtomicArrayOfString::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<std::vector<RankMember>>& AtomicArrayOfString::members() const {
        if (!cached_members_) cached_members_ = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
        "members"));
        return *cached_members_;
    }
    inline const Presence<Meta>& AtomicArrayOfString::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<ReadableQuestion>& AtomicArrayOfString::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::string>& AtomicArrayOfString::question_name() const {
        if (!cached_question_name_) cached_question_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "question_name"));
        return *cached_question_name_;
    }
    inline const Presence<Version>& AtomicArrayOfString::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<PhysicalSource>& AtomicArrayOfString::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Threshold>& AtomicArrayOfString::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<std::vector<std::string>>& AtomicArrayOfString::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<Answer>& AtomicDecideValue::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& AtomicDecideValue::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<Image>>& AtomicDecideValue::images() const {
        if (!cached_images_) cached_images_ = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
        "images"));
        return *cached_images_;
    }
    inline const Presence<uint64_t>& AtomicDecideValue::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& AtomicDecideValue::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<std::vector<RankMember>>& AtomicDecideValue::members() const {
        if (!cached_members_) cached_members_ = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
        "members"));
        return *cached_members_;
    }
    inline const Presence<Meta>& AtomicDecideValue::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<ReadableQuestion>& AtomicDecideValue::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::string>& AtomicDecideValue::question_name() const {
        if (!cached_question_name_) cached_question_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "question_name"));
        return *cached_question_name_;
    }
    inline const Presence<Version>& AtomicDecideValue::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<PhysicalSource>& AtomicDecideValue::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Threshold>& AtomicDecideValue::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<DecideValue>& AtomicDecideValue::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<DecideValue>>(detail::member_value<DecideValue>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<Answer>& AtomicNonZeroUsize::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& AtomicNonZeroUsize::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<Image>>& AtomicNonZeroUsize::images() const {
        if (!cached_images_) cached_images_ = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
        "images"));
        return *cached_images_;
    }
    inline const Presence<uint64_t>& AtomicNonZeroUsize::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& AtomicNonZeroUsize::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<std::vector<RankMember>>& AtomicNonZeroUsize::members() const {
        if (!cached_members_) cached_members_ = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
        "members"));
        return *cached_members_;
    }
    inline const Presence<Meta>& AtomicNonZeroUsize::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<ReadableQuestion>& AtomicNonZeroUsize::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::string>& AtomicNonZeroUsize::question_name() const {
        if (!cached_question_name_) cached_question_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "question_name"));
        return *cached_question_name_;
    }
    inline const Presence<Version>& AtomicNonZeroUsize::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<PhysicalSource>& AtomicNonZeroUsize::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Threshold>& AtomicNonZeroUsize::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<uint64_t>& AtomicNonZeroUsize::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<Answer>& AtomicNullableString::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& AtomicNullableString::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<Image>>& AtomicNullableString::images() const {
        if (!cached_images_) cached_images_ = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
        "images"));
        return *cached_images_;
    }
    inline const Presence<uint64_t>& AtomicNullableString::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& AtomicNullableString::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<std::vector<RankMember>>& AtomicNullableString::members() const {
        if (!cached_members_) cached_members_ = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
        "members"));
        return *cached_members_;
    }
    inline const Presence<Meta>& AtomicNullableString::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<ReadableQuestion>& AtomicNullableString::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::string>& AtomicNullableString::question_name() const {
        if (!cached_question_name_) cached_question_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "question_name"));
        return *cached_question_name_;
    }
    inline const Presence<Version>& AtomicNullableString::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<PhysicalSource>& AtomicNullableString::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Threshold>& AtomicNullableString::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<std::string>& AtomicNullableString::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<Answer>& AtomicBoolean::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& AtomicBoolean::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<Image>>& AtomicBoolean::images() const {
        if (!cached_images_) cached_images_ = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
        "images"));
        return *cached_images_;
    }
    inline const Presence<uint64_t>& AtomicBoolean::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& AtomicBoolean::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<std::vector<RankMember>>& AtomicBoolean::members() const {
        if (!cached_members_) cached_members_ = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
        "members"));
        return *cached_members_;
    }
    inline const Presence<Meta>& AtomicBoolean::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<ReadableQuestion>& AtomicBoolean::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::string>& AtomicBoolean::question_name() const {
        if (!cached_question_name_) cached_question_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "question_name"));
        return *cached_question_name_;
    }
    inline const Presence<Version>& AtomicBoolean::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<PhysicalSource>& AtomicBoolean::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Threshold>& AtomicBoolean::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<bool>& AtomicBoolean::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<Answer>& AtomicDouble::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& AtomicDouble::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<Image>>& AtomicDouble::images() const {
        if (!cached_images_) cached_images_ = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
        "images"));
        return *cached_images_;
    }
    inline const Presence<uint64_t>& AtomicDouble::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& AtomicDouble::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<std::vector<RankMember>>& AtomicDouble::members() const {
        if (!cached_members_) cached_members_ = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
        "members"));
        return *cached_members_;
    }
    inline const Presence<Meta>& AtomicDouble::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<ReadableQuestion>& AtomicDouble::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::string>& AtomicDouble::question_name() const {
        if (!cached_question_name_) cached_question_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "question_name"));
        return *cached_question_name_;
    }
    inline const Presence<Version>& AtomicDouble::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<PhysicalSource>& AtomicDouble::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Threshold>& AtomicDouble::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<double>& AtomicDouble::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<uint64_t>& Attempt::ordinal() const {
        if (!cached_ordinal_) cached_ordinal_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "ordinal"));
        return *cached_ordinal_;
    }
    inline const Presence<AttemptOutcome>& Attempt::outcome() const {
        if (!cached_outcome_) cached_outcome_ = std::make_shared<const Presence<AttemptOutcome>>(detail::member_value<AttemptOutcome>(value_,
        "outcome"));
        return *cached_outcome_;
    }
    inline const Presence<std::string>& Attempt::request_id() const {
        if (!cached_request_id_) cached_request_id_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "request_id"));
        return *cached_request_id_;
    }
    inline const Presence<std::string>& Attempt::request_sha256() const {
        if (!cached_request_sha256_) cached_request_sha256_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "request_sha256"));
        return *cached_request_sha256_;
    }
    inline const Presence<SdkRequestId>& Attempt::sdk_request_id() const {
        if (!cached_sdk_request_id_) cached_sdk_request_id_ = std::make_shared<const Presence<SdkRequestId>>(detail::member_value<SdkRequestId>(value_,
        "sdk_request_id"));
        return *cached_sdk_request_id_;
    }
    inline const Presence<uint64_t>& Attempt::server_ms() const {
        if (!cached_server_ms_) cached_server_ms_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "server_ms"));
        return *cached_server_ms_;
    }
    inline const Presence<uint64_t>& Attempt::status() const {
        if (!cached_status_) cached_status_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "status"));
        return *cached_status_;
    }
    inline const Presence<uint64_t>& Attempt::wall_ms() const {
        if (!cached_wall_ms_) cached_wall_ms_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "wall_ms"));
        return *cached_wall_ms_;
    }
    inline const std::variant<uint64_t, std::string>& Batch::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::variant<uint64_t, std::string>>(decode<std::variant<uint64_t,
        std::string>>(value_));
        return *cached_value_;
    }
    inline const std::string& BoundaryMode::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Presence<std::vector<PieceOdds>>& BoundaryOdds::pieces() const {
        if (!cached_pieces_) cached_pieces_ = std::make_shared<const Presence<std::vector<PieceOdds>>>(detail::member_value<std::vector<PieceOdds>>(value_,
        "pieces"));
        return *cached_pieces_;
    }
    inline const Presence<std::vector<BoundaryProposal>>& BoundaryOdds::proposals() const {
        if (!cached_proposals_) cached_proposals_ = std::make_shared<const Presence<std::vector<BoundaryProposal>>>(detail::member_value<std::vector<BoundaryProposal>>(value_,
        "proposals"));
        return *cached_proposals_;
    }
    inline const Presence<uint64_t>& BoundaryProposal::end() const {
        if (!cached_end_) cached_end_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "end"));
        return *cached_end_;
    }
    inline const Presence<uint64_t>& BoundaryProposal::length() const {
        if (!cached_length_) cached_length_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "length"));
        return *cached_length_;
    }
    inline const Presence<double>& BoundaryProposal::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline const Presence<uint64_t>& BoundaryProposal::start() const {
        if (!cached_start_) cached_start_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "start"));
        return *cached_start_;
    }
    inline const Presence<std::string>& BoundaryProposal::text() const {
        if (!cached_text_) cached_text_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "text"));
        return *cached_text_;
    }
    inline const Presence<Error>& CallError::error() const {
        if (!cached_error_) cached_error_ = std::make_shared<const Presence<Error>>(detail::member_value<Error>(value_,
        "error"));
        return *cached_error_;
    }
    inline const Presence<Facts>& CallError::facts() const {
        if (!cached_facts_) cached_facts_ = std::make_shared<const Presence<Facts>>(detail::member_value<Facts>(value_,
        "facts"));
        return *cached_facts_;
    }
    inline const std::string& CallId::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Json& DecideValue::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Json>(decode<Json>(value_));
        return *cached_value_;
    }
    inline const Presence<std::string>& EntityDocument::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& EntityDocument::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<EstimatedInputDenial>& Error::estimated_input_denial() const {
        if (!cached_estimated_input_denial_) cached_estimated_input_denial_ = std::make_shared<const Presence<EstimatedInputDenial>>(detail::member_value<EstimatedInputDenial>(value_,
        "estimated_input_denial"));
        return *cached_estimated_input_denial_;
    }
    inline const Presence<FailureKind>& Error::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<FailureKind>>(detail::member_value<FailureKind>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& Error::message() const {
        if (!cached_message_) cached_message_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "message"));
        return *cached_message_;
    }
    inline const Presence<bool>& Error::retryable() const {
        if (!cached_retryable_) cached_retryable_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "retryable"));
        return *cached_retryable_;
    }
    inline const Presence<SendBudgetDenial>& Error::send_budget_denial() const {
        if (!cached_send_budget_denial_) cached_send_budget_denial_ = std::make_shared<const Presence<SendBudgetDenial>>(detail::member_value<SendBudgetDenial>(value_,
        "send_budget_denial"));
        return *cached_send_budget_denial_;
    }
    inline const Presence<Stopped>& Error::stopped() const {
        if (!cached_stopped_) cached_stopped_ = std::make_shared<const Presence<Stopped>>(detail::member_value<Stopped>(value_,
        "stopped"));
        return *cached_stopped_;
    }
    inline std::optional<EstimatedInputDenialInitialRequest> EstimatedInputDenial::as_EstimatedInputDenialInitialRequest() const {
        if (detail::literal(value_, "kind", "initial_request")) return EstimatedInputDenialInitialRequest(value_);
        return std::nullopt;
    }
    inline std::optional<EstimatedInputDenialAdditionalRequest> EstimatedInputDenial::as_EstimatedInputDenialAdditionalRequest() const {
        if (detail::literal(value_, "kind", "additional_request")) return EstimatedInputDenialAdditionalRequest(value_);
        return std::nullopt;
    }
    inline std::optional<EstimatedInputDenialRetry> EstimatedInputDenial::as_EstimatedInputDenialRetry() const {
        if (detail::literal(value_, "kind", "retry")) return EstimatedInputDenialRetry(value_);
        return std::nullopt;
    }
    inline const Presence<std::string>& EstimatedInputDenialAdditionalRequest::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<uint64_t>& EstimatedInputDenialAdditionalRequest::limit() const {
        if (!cached_limit_) cached_limit_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "limit"));
        return *cached_limit_;
    }
    inline const Presence<std::string>& EstimatedInputDenialInitialRequest::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<uint64_t>& EstimatedInputDenialInitialRequest::limit() const {
        if (!cached_limit_) cached_limit_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "limit"));
        return *cached_limit_;
    }
    inline const Presence<std::string>& EstimatedInputDenialRetry::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<uint64_t>& EstimatedInputDenialRetry::last_status() const {
        if (!cached_last_status_) cached_last_status_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_status"));
        return *cached_last_status_;
    }
    inline const Presence<uint64_t>& EstimatedInputDenialRetry::limit() const {
        if (!cached_limit_) cached_limit_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "limit"));
        return *cached_limit_;
    }
    inline const Presence<std::vector<Attempt>>& Facts::attempts() const {
        if (!cached_attempts_) cached_attempts_ = std::make_shared<const Presence<std::vector<Attempt>>>(detail::member_value<std::vector<Attempt>>(value_,
        "attempts"));
        return *cached_attempts_;
    }
    inline const Presence<uint64_t>& Facts::cache_answers() const {
        if (!cached_cache_answers_) cached_cache_answers_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "cache_answers"));
        return *cached_cache_answers_;
    }
    inline const Presence<CallId>& Facts::call_id() const {
        if (!cached_call_id_) cached_call_id_ = std::make_shared<const Presence<CallId>>(detail::member_value<CallId>(value_,
        "call_id"));
        return *cached_call_id_;
    }
    inline const Presence<std::string>& Facts::estimated_cost_usd() const {
        if (!cached_estimated_cost_usd_) cached_estimated_cost_usd_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "estimated_cost_usd"));
        return *cached_estimated_cost_usd_;
    }
    inline const Presence<bool>& Facts::held_model_mismatch() const {
        if (!cached_held_model_mismatch_) cached_held_model_mismatch_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "held_model_mismatch"));
        return *cached_held_model_mismatch_;
    }
    inline const Presence<uint64_t>& Facts::input_tokens() const {
        if (!cached_input_tokens_) cached_input_tokens_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "input_tokens"));
        return *cached_input_tokens_;
    }
    inline const Presence<uint64_t>& Facts::largest_request_bytes() const {
        if (!cached_largest_request_bytes_) cached_largest_request_bytes_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "largest_request_bytes"));
        return *cached_largest_request_bytes_;
    }
    inline const Presence<uint64_t>& Facts::largest_request_estimated_input_tokens() const {
        if (!cached_largest_request_estimated_input_tokens_) cached_largest_request_estimated_input_tokens_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "largest_request_estimated_input_tokens"));
        return *cached_largest_request_estimated_input_tokens_;
    }
    inline const Presence<std::string>& Facts::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<uint64_t>& Facts::output_tokens() const {
        if (!cached_output_tokens_) cached_output_tokens_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "output_tokens"));
        return *cached_output_tokens_;
    }
    inline const Presence<uint64_t>& Facts::records() const {
        if (!cached_records_) cached_records_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "records"));
        return *cached_records_;
    }
    inline const Presence<uint64_t>& Facts::requests_sent() const {
        if (!cached_requests_sent_) cached_requests_sent_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "requests_sent"));
        return *cached_requests_sent_;
    }
    inline const Presence<double>& Facts::seconds() const {
        if (!cached_seconds_) cached_seconds_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "seconds"));
        return *cached_seconds_;
    }
    inline const Presence<std::string>& Facts::token_estimate_method() const {
        if (!cached_token_estimate_method_) cached_token_estimate_method_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "token_estimate_method"));
        return *cached_token_estimate_method_;
    }
    inline const Presence<PersistenceObservation>& Facts::usage_persistence() const {
        if (!cached_usage_persistence_) cached_usage_persistence_ = std::make_shared<const Presence<PersistenceObservation>>(detail::member_value<PersistenceObservation>(value_,
        "usage_persistence"));
        return *cached_usage_persistence_;
    }
    inline const std::string& FailureId::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Presence<FindAnswer>& Find::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<FindAnswer>>(detail::member_value<FindAnswer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& Find::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<FindCandidate>>& Find::candidates() const {
        if (!cached_candidates_) cached_candidates_ = std::make_shared<const Presence<std::vector<FindCandidate>>>(detail::member_value<std::vector<FindCandidate>>(value_,
        "candidates"));
        return *cached_candidates_;
    }
    inline const Presence<std::string>& Find::file() const {
        if (!cached_file_) cached_file_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "file"));
        return *cached_file_;
    }
    inline const Presence<uint64_t>& Find::first_line() const {
        if (!cached_first_line_) cached_first_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "first_line"));
        return *cached_first_line_;
    }
    inline const Presence<uint64_t>& Find::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<uint64_t>& Find::last_line() const {
        if (!cached_last_line_) cached_last_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_line"));
        return *cached_last_line_;
    }
    inline const Presence<Meta>& Find::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<Position>& Find::position() const {
        if (!cached_position_) cached_position_ = std::make_shared<const Presence<Position>>(detail::member_value<Position>(value_,
        "position"));
        return *cached_position_;
    }
    inline const Presence<ReadableQuestion2>& Find::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion2>>(detail::member_value<ReadableQuestion2>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<Version>& Find::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<std::nullptr_t>& Find::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<std::nullptr_t>>(detail::member_value<std::nullptr_t>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<Json>& Find::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<uint64_t>& FindCandidate::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& FindCandidate::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<double>& FindCandidate::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline const Presence<PhysicalSource>& FindCandidate::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<std::string>& Image::base64() const {
        if (!cached_base64_) cached_base64_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "base64"));
        return *cached_base64_;
    }
    inline const Presence<uint32_t>& Image::height() const {
        if (!cached_height_) cached_height_ = std::make_shared<const Presence<uint32_t>>(detail::member_value<uint32_t>(value_,
        "height"));
        return *cached_height_;
    }
    inline const Presence<ImageMedia>& Image::media() const {
        if (!cached_media_) cached_media_ = std::make_shared<const Presence<ImageMedia>>(detail::member_value<ImageMedia>(value_,
        "media"));
        return *cached_media_;
    }
    inline const Presence<uint32_t>& Image::width() const {
        if (!cached_width_) cached_width_ = std::make_shared<const Presence<uint32_t>>(detail::member_value<uint32_t>(value_,
        "width"));
        return *cached_width_;
    }
    inline const std::string& ImageMedia::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline std::optional<InputDeclarationStringRoot> InputDeclaration::as_InputDeclarationStringRoot() const {
        if (detail::literal(value_, "type", "string")) return InputDeclarationStringRoot(value_);
        return std::nullopt;
    }
    inline std::optional<InputDeclarationObjectRoot> InputDeclaration::as_InputDeclarationObjectRoot() const {
        if (detail::literal(value_, "type", "object")) return InputDeclarationObjectRoot(value_);
        return std::nullopt;
    }
    inline const Presence<std::vector<std::pair<std::string, InputPropertyType>>>& InputDeclarationObjectRoot::properties() const {
        if (!cached_properties_) cached_properties_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        InputPropertyType>>>>(detail::member_value<std::vector<std::pair<std::string, InputPropertyType>>>(value_,
        "properties"));
        return *cached_properties_;
    }
    inline const Presence<std::vector<std::string>>& InputDeclarationObjectRoot::required() const {
        if (!cached_required_) cached_required_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "required"));
        return *cached_required_;
    }
    inline const Presence<ObjectType>& InputDeclarationObjectRoot::type() const {
        if (!cached_type_) cached_type_ = std::make_shared<const Presence<ObjectType>>(detail::member_value<ObjectType>(value_,
        "type"));
        return *cached_type_;
    }
    inline const Presence<StringType>& InputDeclarationStringRoot::type() const {
        if (!cached_type_) cached_type_ = std::make_shared<const Presence<StringType>>(detail::member_value<StringType>(value_,
        "type"));
        return *cached_type_;
    }
    inline std::optional<InputPropertyTypeString> InputPropertyType::as_InputPropertyTypeString() const {
        if (detail::literal(value_, "type", "string")) return InputPropertyTypeString(value_);
        return std::nullopt;
    }
    inline std::optional<InputPropertyTypeNumber> InputPropertyType::as_InputPropertyTypeNumber() const {
        if (detail::literal(value_, "type", "number")) return InputPropertyTypeNumber(value_);
        return std::nullopt;
    }
    inline std::optional<InputPropertyTypeBoolean> InputPropertyType::as_InputPropertyTypeBoolean() const {
        if (detail::literal(value_, "type", "boolean")) return InputPropertyTypeBoolean(value_);
        return std::nullopt;
    }
    inline std::optional<InputPropertyTypeArray> InputPropertyType::as_InputPropertyTypeArray() const {
        if (detail::literal(value_, "type", "array")) return InputPropertyTypeArray(value_);
        return std::nullopt;
    }
    inline const Presence<StringRoot>& InputPropertyTypeArray::items() const {
        if (!cached_items_) cached_items_ = std::make_shared<const Presence<StringRoot>>(detail::member_value<StringRoot>(value_,
        "items"));
        return *cached_items_;
    }
    inline const Presence<std::string>& InputPropertyTypeArray::type() const {
        if (!cached_type_) cached_type_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "type"));
        return *cached_type_;
    }
    inline const Presence<std::string>& InputPropertyTypeBoolean::type() const {
        if (!cached_type_) cached_type_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "type"));
        return *cached_type_;
    }
    inline const Presence<std::string>& InputPropertyTypeNumber::type() const {
        if (!cached_type_) cached_type_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "type"));
        return *cached_type_;
    }
    inline const Presence<std::string>& InputPropertyTypeString::type() const {
        if (!cached_type_) cached_type_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "type"));
        return *cached_type_;
    }
    inline const Presence<Json>& Label::description() const {
        if (!cached_description_) cached_description_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "description"));
        return *cached_description_;
    }
    inline const Presence<std::string>& Label::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::string>& Meta::answered_by() const {
        if (!cached_answered_by_) cached_answered_by_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "answered_by"));
        return *cached_answered_by_;
    }
    inline const Presence<std::vector<Attempt>>& Meta::attempts() const {
        if (!cached_attempts_) cached_attempts_ = std::make_shared<const Presence<std::vector<Attempt>>>(detail::member_value<std::vector<Attempt>>(value_,
        "attempts"));
        return *cached_attempts_;
    }
    inline const Presence<BatchSetting>& Meta::batch_setting() const {
        if (!cached_batch_setting_) cached_batch_setting_ = std::make_shared<const Presence<BatchSetting>>(detail::member_value<BatchSetting>(value_,
        "batch_setting"));
        return *cached_batch_setting_;
    }
    inline const Presence<BatchWarning>& Meta::batch_warning() const {
        if (!cached_batch_warning_) cached_batch_warning_ = std::make_shared<const Presence<BatchWarning>>(detail::member_value<BatchWarning>(value_,
        "batch_warning"));
        return *cached_batch_warning_;
    }
    inline const Presence<bool>& Meta::cached() const {
        if (!cached_cached_) cached_cached_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "cached"));
        return *cached_cached_;
    }
    inline const Presence<std::string>& Meta::context_sha256() const {
        if (!cached_context_sha256_) cached_context_sha256_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "context_sha256"));
        return *cached_context_sha256_;
    }
    inline const Presence<uint64_t>& Meta::failed_questions() const {
        if (!cached_failed_questions_) cached_failed_questions_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "failed_questions"));
        return *cached_failed_questions_;
    }
    inline const Presence<std::string>& Meta::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<std::vector<Observation>>& Meta::observations() const {
        if (!cached_observations_) cached_observations_ = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
        "observations"));
        return *cached_observations_;
    }
    inline const Presence<Origin>& Meta::origin() const {
        if (!cached_origin_) cached_origin_ = std::make_shared<const Presence<Origin>>(detail::member_value<Origin>(value_,
        "origin"));
        return *cached_origin_;
    }
    inline const Presence<ProfileWarning>& Meta::profile_warning() const {
        if (!cached_profile_warning_) cached_profile_warning_ = std::make_shared<const Presence<ProfileWarning>>(detail::member_value<ProfileWarning>(value_,
        "profile_warning"));
        return *cached_profile_warning_;
    }
    inline const Presence<std::string>& Meta::question_sha256() const {
        if (!cached_question_sha256_) cached_question_sha256_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "question_sha256"));
        return *cached_question_sha256_;
    }
    inline const Presence<std::vector<QuestionSource>>& Meta::question_sources() const {
        if (!cached_question_sources_) cached_question_sources_ = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
        "question_sources"));
        return *cached_question_sources_;
    }
    inline const Presence<std::string>& Meta::questions_sha256() const {
        if (!cached_questions_sha256_) cached_questions_sha256_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "questions_sha256"));
        return *cached_questions_sha256_;
    }
    inline const Presence<std::vector<std::string>>& Meta::requests() const {
        if (!cached_requests_) cached_requests_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "requests"));
        return *cached_requests_;
    }
    inline const Presence<uint64_t>& Meta::requests_sent() const {
        if (!cached_requests_sent_) cached_requests_sent_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "requests_sent"));
        return *cached_requests_sent_;
    }
    inline const Presence<std::string>& Meta::tool() const {
        if (!cached_tool_) cached_tool_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "tool"));
        return *cached_tool_;
    }
    inline const Presence<std::string>& Meta::url() const {
        if (!cached_url_) cached_url_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "url"));
        return *cached_url_;
    }
    inline const Presence<Usage>& Meta::usage() const {
        if (!cached_usage_) cached_usage_ = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
        "usage"));
        return *cached_usage_;
    }
    inline const Presence<std::vector<std::pair<std::string, InputPropertyType>>>& ObjectRoot::properties() const {
        if (!cached_properties_) cached_properties_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        InputPropertyType>>>>(detail::member_value<std::vector<std::pair<std::string, InputPropertyType>>>(value_,
        "properties"));
        return *cached_properties_;
    }
    inline const Presence<std::vector<std::string>>& ObjectRoot::required() const {
        if (!cached_required_) cached_required_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "required"));
        return *cached_required_;
    }
    inline const Presence<ObjectType>& ObjectRoot::type() const {
        if (!cached_type_) cached_type_ = std::make_shared<const Presence<ObjectType>>(detail::member_value<ObjectType>(value_,
        "type"));
        return *cached_type_;
    }
    inline const std::string& ObjectType::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline std::optional<ObservationObservationId> Observation::as_ObservationObservationId() const {
        if (value_.contains("observation_id")) return ObservationObservationId(value_);
        return std::nullopt;
    }
    inline std::optional<ObservationFailureId> Observation::as_ObservationFailureId() const {
        if (value_.contains("failure_id")) return ObservationFailureId(value_);
        return std::nullopt;
    }
    inline const std::string& ObservationId::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Presence<FailureId>& ObservationFailureId::failure_id() const {
        if (!cached_failure_id_) cached_failure_id_ = std::make_shared<const Presence<FailureId>>(detail::member_value<FailureId>(value_,
        "failure_id"));
        return *cached_failure_id_;
    }
    inline const Presence<ObservationId>& ObservationObservationId::observation_id() const {
        if (!cached_observation_id_) cached_observation_id_ = std::make_shared<const Presence<ObservationId>>(detail::member_value<ObservationId>(value_,
        "observation_id"));
        return *cached_observation_id_;
    }
    inline const std::string& Origin::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Presence<std::string>& PersistenceObservation::advice() const {
        if (!cached_advice_) cached_advice_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "advice"));
        return *cached_advice_;
    }
    inline const Presence<std::string>& PersistenceObservation::observed_at() const {
        if (!cached_observed_at_) cached_observed_at_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "observed_at"));
        return *cached_observed_at_;
    }
    inline const Presence<UsagePersistence>& PersistenceObservation::state() const {
        if (!cached_state_) cached_state_ = std::make_shared<const Presence<UsagePersistence>>(detail::member_value<UsagePersistence>(value_,
        "state"));
        return *cached_state_;
    }
    inline const Presence<std::string>& PhysicalSource::file() const {
        if (!cached_file_) cached_file_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "file"));
        return *cached_file_;
    }
    inline const Presence<uint64_t>& PhysicalSource::first_line() const {
        if (!cached_first_line_) cached_first_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "first_line"));
        return *cached_first_line_;
    }
    inline const Presence<uint64_t>& PhysicalSource::last_line() const {
        if (!cached_last_line_) cached_last_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_line"));
        return *cached_last_line_;
    }
    inline const Presence<std::string>& Position::file() const {
        if (!cached_file_) cached_file_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "file"));
        return *cached_file_;
    }
    inline const Presence<uint64_t>& Position::first() const {
        if (!cached_first_) cached_first_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "first"));
        return *cached_first_;
    }
    inline const Presence<std::vector<std::string>>& Position::images() const {
        if (!cached_images_) cached_images_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "images"));
        return *cached_images_;
    }
    inline const Presence<uint64_t>& Position::last() const {
        if (!cached_last_) cached_last_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last"));
        return *cached_last_;
    }
    inline const std::string& QuestionName::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Presence<std::string>& QuestionSource::answered_by() const {
        if (!cached_answered_by_) cached_answered_by_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "answered_by"));
        return *cached_answered_by_;
    }
    inline const Presence<uint32_t>& QuestionSource::batch_size() const {
        if (!cached_batch_size_) cached_batch_size_ = std::make_shared<const Presence<uint32_t>>(detail::member_value<uint32_t>(value_,
        "batch_size"));
        return *cached_batch_size_;
    }
    inline const Presence<Origin>& QuestionSource::origin() const {
        if (!cached_origin_) cached_origin_ = std::make_shared<const Presence<Origin>>(detail::member_value<Origin>(value_,
        "origin"));
        return *cached_origin_;
    }
    inline const Presence<std::string>& RankMember::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<RankMemberResult>& RankMember::result() const {
        if (!cached_result_) cached_result_ = std::make_shared<const Presence<RankMemberResult>>(detail::member_value<RankMemberResult>(value_,
        "result"));
        return *cached_result_;
    }
    inline const Presence<Answer>& RankMemberResult::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& RankMemberResult::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::vector<Image>>& RankMemberResult::images() const {
        if (!cached_images_) cached_images_ = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
        "images"));
        return *cached_images_;
    }
    inline const Presence<Meta>& RankMemberResult::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<ReadableQuestion>& RankMemberResult::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<Version>& RankMemberResult::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<PhysicalSource>& RankMemberResult::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<std::nullptr_t>& RankMemberResult::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<std::nullptr_t>>(detail::member_value<std::nullptr_t>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<uint64_t>& RankMemberResult::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "value"));
        return *cached_value_;
    }
    inline std::optional<ReadableQuestionDecide> ReadableQuestion::as_ReadableQuestionDecide() const {
        if (detail::literal(value_, "verb", "decide")) return ReadableQuestionDecide(value_);
        return std::nullopt;
    }
    inline std::optional<ReadableQuestionChoose> ReadableQuestion::as_ReadableQuestionChoose() const {
        if (detail::literal(value_, "verb", "choose")) return ReadableQuestionChoose(value_);
        return std::nullopt;
    }
    inline std::optional<ReadableQuestionTag> ReadableQuestion::as_ReadableQuestionTag() const {
        if (detail::literal(value_, "verb", "tag")) return ReadableQuestionTag(value_);
        return std::nullopt;
    }
    inline std::optional<ReadableQuestionScore> ReadableQuestion::as_ReadableQuestionScore() const {
        if (detail::literal(value_, "verb", "score")) return ReadableQuestionScore(value_);
        return std::nullopt;
    }
    inline const Presence<Batch>& ReadableQuestion2::batch() const {
        if (!cached_batch_) cached_batch_ = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
        "batch"));
        return *cached_batch_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestion2::context_schema() const {
        if (!cached_context_schema_) cached_context_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "context_schema"));
        return *cached_context_schema_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestion2::item_schema() const {
        if (!cached_item_schema_) cached_item_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "item_schema"));
        return *cached_item_schema_;
    }
    inline const Presence<std::vector<Label>>& ReadableQuestion2::label_details() const {
        if (!cached_label_details_) cached_label_details_ = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
        "label_details"));
        return *cached_label_details_;
    }
    inline const Presence<std::string>& ReadableQuestion2::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<QuestionName>& ReadableQuestion2::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<bool>& ReadableQuestion2::none() const {
        if (!cached_none_) cached_none_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "none"));
        return *cached_none_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestion2::on() const {
        if (!cached_on_) cached_on_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "on"));
        return *cached_on_;
    }
    inline const Presence<std::string>& ReadableQuestion2::profile() const {
        if (!cached_profile_) cached_profile_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "profile"));
        return *cached_profile_;
    }
    inline const Presence<Json>& ReadableQuestion2::text() const {
        if (!cached_text_) cached_text_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "text"));
        return *cached_text_;
    }
    inline const Presence<std::string>& ReadableQuestion2::verb() const {
        if (!cached_verb_) cached_verb_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "verb"));
        return *cached_verb_;
    }
    inline const Presence<WordingVersion>& ReadableQuestion2::wording_version() const {
        if (!cached_wording_version_) cached_wording_version_ = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
        "wording_version"));
        return *cached_wording_version_;
    }
    inline const Presence<Batch>& ReadableQuestion3::batch() const {
        if (!cached_batch_) cached_batch_ = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
        "batch"));
        return *cached_batch_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestion3::context_schema() const {
        if (!cached_context_schema_) cached_context_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "context_schema"));
        return *cached_context_schema_;
    }
    inline const Presence<Json>& ReadableQuestion3::entity_definition() const {
        if (!cached_entity_definition_) cached_entity_definition_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "entity_definition"));
        return *cached_entity_definition_;
    }
    inline const Presence<Json>& ReadableQuestion3::instructions() const {
        if (!cached_instructions_) cached_instructions_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "instructions"));
        return *cached_instructions_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestion3::item_schema() const {
        if (!cached_item_schema_) cached_item_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "item_schema"));
        return *cached_item_schema_;
    }
    inline const Presence<std::vector<std::pair<std::string, Json>>>& ReadableQuestion3::kinds() const {
        if (!cached_kinds_) cached_kinds_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        Json>>>>(detail::member_value<std::vector<std::pair<std::string, Json>>>(value_, "kinds"));
        return *cached_kinds_;
    }
    inline const Presence<std::vector<Label>>& ReadableQuestion3::label_details() const {
        if (!cached_label_details_) cached_label_details_ = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
        "label_details"));
        return *cached_label_details_;
    }
    inline const Presence<RecognitionMode>& ReadableQuestion3::mode() const {
        if (!cached_mode_) cached_mode_ = std::make_shared<const Presence<RecognitionMode>>(detail::member_value<RecognitionMode>(value_,
        "mode"));
        return *cached_mode_;
    }
    inline const Presence<std::string>& ReadableQuestion3::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<QuestionName>& ReadableQuestion3::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestion3::on() const {
        if (!cached_on_) cached_on_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "on"));
        return *cached_on_;
    }
    inline const Presence<std::string>& ReadableQuestion3::profile() const {
        if (!cached_profile_) cached_profile_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "profile"));
        return *cached_profile_;
    }
    inline const Presence<Threshold>& ReadableQuestion3::relation_threshold() const {
        if (!cached_relation_threshold_) cached_relation_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "relation_threshold"));
        return *cached_relation_threshold_;
    }
    inline const Presence<std::vector<RelationRule>>& ReadableQuestion3::relations() const {
        if (!cached_relations_) cached_relations_ = std::make_shared<const Presence<std::vector<RelationRule>>>(detail::member_value<std::vector<RelationRule>>(value_,
        "relations"));
        return *cached_relations_;
    }
    inline const Presence<uint32_t>& ReadableQuestion3::snippet_pieces() const {
        if (!cached_snippet_pieces_) cached_snippet_pieces_ = std::make_shared<const Presence<uint32_t>>(detail::member_value<uint32_t>(value_,
        "snippet_pieces"));
        return *cached_snippet_pieces_;
    }
    inline const Presence<RecognitionStageContext>& ReadableQuestion3::stage_context() const {
        if (!cached_stage_context_) cached_stage_context_ = std::make_shared<const Presence<RecognitionStageContext>>(detail::member_value<RecognitionStageContext>(value_,
        "stage_context"));
        return *cached_stage_context_;
    }
    inline const Presence<Threshold>& ReadableQuestion3::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<Verb>& ReadableQuestion3::verb() const {
        if (!cached_verb_) cached_verb_ = std::make_shared<const Presence<Verb>>(detail::member_value<Verb>(value_,
        "verb"));
        return *cached_verb_;
    }
    inline const Presence<WordingVersion>& ReadableQuestion3::wording_version() const {
        if (!cached_wording_version_) cached_wording_version_ = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
        "wording_version"));
        return *cached_wording_version_;
    }
    inline const Presence<Batch>& ReadableQuestion4::batch() const {
        if (!cached_batch_) cached_batch_ = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
        "batch"));
        return *cached_batch_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestion4::context_schema() const {
        if (!cached_context_schema_) cached_context_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "context_schema"));
        return *cached_context_schema_;
    }
    inline const Presence<RelateFields>& ReadableQuestion4::fields() const {
        if (!cached_fields_) cached_fields_ = std::make_shared<const Presence<RelateFields>>(detail::member_value<RelateFields>(value_,
        "fields"));
        return *cached_fields_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestion4::item_schema() const {
        if (!cached_item_schema_) cached_item_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "item_schema"));
        return *cached_item_schema_;
    }
    inline const Presence<std::vector<Label>>& ReadableQuestion4::label_details() const {
        if (!cached_label_details_) cached_label_details_ = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
        "label_details"));
        return *cached_label_details_;
    }
    inline const Presence<std::string>& ReadableQuestion4::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<QuestionName>& ReadableQuestion4::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestion4::on() const {
        if (!cached_on_) cached_on_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "on"));
        return *cached_on_;
    }
    inline const Presence<std::string>& ReadableQuestion4::profile() const {
        if (!cached_profile_) cached_profile_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "profile"));
        return *cached_profile_;
    }
    inline const Presence<std::vector<RelationRule>>& ReadableQuestion4::relations() const {
        if (!cached_relations_) cached_relations_ = std::make_shared<const Presence<std::vector<RelationRule>>>(detail::member_value<std::vector<RelationRule>>(value_,
        "relations"));
        return *cached_relations_;
    }
    inline const Presence<Threshold>& ReadableQuestion4::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<std::string>& ReadableQuestion4::verb() const {
        if (!cached_verb_) cached_verb_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "verb"));
        return *cached_verb_;
    }
    inline const Presence<WordingVersion>& ReadableQuestion4::wording_version() const {
        if (!cached_wording_version_) cached_wording_version_ = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
        "wording_version"));
        return *cached_wording_version_;
    }
    inline const Presence<Batch>& ReadableQuestionChoose::batch() const {
        if (!cached_batch_) cached_batch_ = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
        "batch"));
        return *cached_batch_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestionChoose::context_schema() const {
        if (!cached_context_schema_) cached_context_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "context_schema"));
        return *cached_context_schema_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestionChoose::item_schema() const {
        if (!cached_item_schema_) cached_item_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "item_schema"));
        return *cached_item_schema_;
    }
    inline const Presence<std::vector<Label>>& ReadableQuestionChoose::label_details() const {
        if (!cached_label_details_) cached_label_details_ = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
        "label_details"));
        return *cached_label_details_;
    }
    inline const Presence<std::string>& ReadableQuestionChoose::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<QuestionName>& ReadableQuestionChoose::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestionChoose::on() const {
        if (!cached_on_) cached_on_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "on"));
        return *cached_on_;
    }
    inline const Presence<std::string>& ReadableQuestionChoose::profile() const {
        if (!cached_profile_) cached_profile_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "profile"));
        return *cached_profile_;
    }
    inline const Presence<WordingVersion>& ReadableQuestionChoose::wording_version() const {
        if (!cached_wording_version_) cached_wording_version_ = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
        "wording_version"));
        return *cached_wording_version_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestionChoose::options() const {
        if (!cached_options_) cached_options_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "options"));
        return *cached_options_;
    }
    inline const Presence<Json>& ReadableQuestionChoose::text() const {
        if (!cached_text_) cached_text_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "text"));
        return *cached_text_;
    }
    inline const Presence<std::string>& ReadableQuestionChoose::verb() const {
        if (!cached_verb_) cached_verb_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "verb"));
        return *cached_verb_;
    }
    inline const Presence<Batch>& ReadableQuestionDecide::batch() const {
        if (!cached_batch_) cached_batch_ = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
        "batch"));
        return *cached_batch_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestionDecide::context_schema() const {
        if (!cached_context_schema_) cached_context_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "context_schema"));
        return *cached_context_schema_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestionDecide::item_schema() const {
        if (!cached_item_schema_) cached_item_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "item_schema"));
        return *cached_item_schema_;
    }
    inline const Presence<std::vector<Label>>& ReadableQuestionDecide::label_details() const {
        if (!cached_label_details_) cached_label_details_ = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
        "label_details"));
        return *cached_label_details_;
    }
    inline const Presence<std::string>& ReadableQuestionDecide::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<QuestionName>& ReadableQuestionDecide::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestionDecide::on() const {
        if (!cached_on_) cached_on_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "on"));
        return *cached_on_;
    }
    inline const Presence<std::string>& ReadableQuestionDecide::profile() const {
        if (!cached_profile_) cached_profile_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "profile"));
        return *cached_profile_;
    }
    inline const Presence<WordingVersion>& ReadableQuestionDecide::wording_version() const {
        if (!cached_wording_version_) cached_wording_version_ = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
        "wording_version"));
        return *cached_wording_version_;
    }
    inline const Presence<Json>& ReadableQuestionDecide::false_() const {
        if (!cached_false__) cached_false__ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "false"));
        return *cached_false__;
    }
    inline const Presence<Json>& ReadableQuestionDecide::text() const {
        if (!cached_text_) cached_text_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "text"));
        return *cached_text_;
    }
    inline const Presence<Json>& ReadableQuestionDecide::true_() const {
        if (!cached_true__) cached_true__ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "true"));
        return *cached_true__;
    }
    inline const Presence<std::string>& ReadableQuestionDecide::verb() const {
        if (!cached_verb_) cached_verb_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "verb"));
        return *cached_verb_;
    }
    inline const Presence<Batch>& ReadableQuestionScore::batch() const {
        if (!cached_batch_) cached_batch_ = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
        "batch"));
        return *cached_batch_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestionScore::context_schema() const {
        if (!cached_context_schema_) cached_context_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "context_schema"));
        return *cached_context_schema_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestionScore::item_schema() const {
        if (!cached_item_schema_) cached_item_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "item_schema"));
        return *cached_item_schema_;
    }
    inline const Presence<std::vector<Label>>& ReadableQuestionScore::label_details() const {
        if (!cached_label_details_) cached_label_details_ = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
        "label_details"));
        return *cached_label_details_;
    }
    inline const Presence<std::string>& ReadableQuestionScore::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<QuestionName>& ReadableQuestionScore::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestionScore::on() const {
        if (!cached_on_) cached_on_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "on"));
        return *cached_on_;
    }
    inline const Presence<std::string>& ReadableQuestionScore::profile() const {
        if (!cached_profile_) cached_profile_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "profile"));
        return *cached_profile_;
    }
    inline const Presence<WordingVersion>& ReadableQuestionScore::wording_version() const {
        if (!cached_wording_version_) cached_wording_version_ = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
        "wording_version"));
        return *cached_wording_version_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestionScore::levels() const {
        if (!cached_levels_) cached_levels_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "levels"));
        return *cached_levels_;
    }
    inline const Presence<Json>& ReadableQuestionScore::text() const {
        if (!cached_text_) cached_text_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "text"));
        return *cached_text_;
    }
    inline const Presence<std::string>& ReadableQuestionScore::verb() const {
        if (!cached_verb_) cached_verb_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "verb"));
        return *cached_verb_;
    }
    inline const Presence<Batch>& ReadableQuestionTag::batch() const {
        if (!cached_batch_) cached_batch_ = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
        "batch"));
        return *cached_batch_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestionTag::context_schema() const {
        if (!cached_context_schema_) cached_context_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "context_schema"));
        return *cached_context_schema_;
    }
    inline const Presence<InputDeclaration>& ReadableQuestionTag::item_schema() const {
        if (!cached_item_schema_) cached_item_schema_ = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
        "item_schema"));
        return *cached_item_schema_;
    }
    inline const Presence<std::vector<Label>>& ReadableQuestionTag::label_details() const {
        if (!cached_label_details_) cached_label_details_ = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
        "label_details"));
        return *cached_label_details_;
    }
    inline const Presence<std::string>& ReadableQuestionTag::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<QuestionName>& ReadableQuestionTag::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestionTag::on() const {
        if (!cached_on_) cached_on_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "on"));
        return *cached_on_;
    }
    inline const Presence<std::string>& ReadableQuestionTag::profile() const {
        if (!cached_profile_) cached_profile_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "profile"));
        return *cached_profile_;
    }
    inline const Presence<WordingVersion>& ReadableQuestionTag::wording_version() const {
        if (!cached_wording_version_) cached_wording_version_ = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
        "wording_version"));
        return *cached_wording_version_;
    }
    inline const Presence<std::vector<std::string>>& ReadableQuestionTag::labels() const {
        if (!cached_labels_) cached_labels_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "labels"));
        return *cached_labels_;
    }
    inline const Presence<Json>& ReadableQuestionTag::text() const {
        if (!cached_text_) cached_text_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "text"));
        return *cached_text_;
    }
    inline const Presence<std::string>& ReadableQuestionTag::verb() const {
        if (!cached_verb_) cached_verb_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "verb"));
        return *cached_verb_;
    }
    inline const Presence<RecognitionOdds>& Recognition::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<RecognitionOdds>>(detail::member_value<RecognitionOdds>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& Recognition::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::string>& Recognition::file() const {
        if (!cached_file_) cached_file_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "file"));
        return *cached_file_;
    }
    inline const Presence<uint64_t>& Recognition::first_line() const {
        if (!cached_first_line_) cached_first_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "first_line"));
        return *cached_first_line_;
    }
    inline const Presence<uint64_t>& Recognition::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& Recognition::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<uint64_t>& Recognition::last_line() const {
        if (!cached_last_line_) cached_last_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_line"));
        return *cached_last_line_;
    }
    inline const Presence<Meta>& Recognition::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<Position>& Recognition::position() const {
        if (!cached_position_) cached_position_ = std::make_shared<const Presence<Position>>(detail::member_value<Position>(value_,
        "position"));
        return *cached_position_;
    }
    inline const Presence<ReadableQuestion3>& Recognition::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion3>>(detail::member_value<ReadableQuestion3>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<Version>& Recognition::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<PhysicalSource>& Recognition::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Recognize>& Recognition::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<Recognize>>(detail::member_value<Recognize>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<bool>& RecognitionEdgeDocument::either() const {
        if (!cached_either_) cached_either_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "either"));
        return *cached_either_;
    }
    inline const Presence<double>& RecognitionEdgeDocument::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline const Presence<std::string>& RecognitionEdgeDocument::relation() const {
        if (!cached_relation_) cached_relation_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "relation"));
        return *cached_relation_;
    }
    inline const Presence<Entity>& RecognitionEdgeDocument::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<Entity>>(detail::member_value<Entity>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Entity>& RecognitionEdgeDocument::target() const {
        if (!cached_target_) cached_target_ = std::make_shared<const Presence<Entity>>(detail::member_value<Entity>(value_,
        "target"));
        return *cached_target_;
    }
    inline const std::string& RecognitionMode::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline std::optional<RecognitionOddsRecognizeAnswer> RecognitionOdds::as_RecognitionOddsRecognizeAnswer() const {
        if (value_.contains("names") && value_.contains("pairs") && value_.contains("pieces") && value_.contains("proposals")) return RecognitionOddsRecognizeAnswer(value_);
        return std::nullopt;
    }
    inline std::optional<RecognitionOddsBoundaryOdds> RecognitionOdds::as_RecognitionOddsBoundaryOdds() const {
        if (value_.contains("pieces") && value_.contains("proposals") && !value_.contains("names") && !value_.contains("pairs")) return RecognitionOddsBoundaryOdds(value_);
        return std::nullopt;
    }
    inline const Presence<std::vector<PieceOdds>>& RecognitionOddsBoundaryOdds::pieces() const {
        if (!cached_pieces_) cached_pieces_ = std::make_shared<const Presence<std::vector<PieceOdds>>>(detail::member_value<std::vector<PieceOdds>>(value_,
        "pieces"));
        return *cached_pieces_;
    }
    inline const Presence<std::vector<BoundaryProposal>>& RecognitionOddsBoundaryOdds::proposals() const {
        if (!cached_proposals_) cached_proposals_ = std::make_shared<const Presence<std::vector<BoundaryProposal>>>(detail::member_value<std::vector<BoundaryProposal>>(value_,
        "proposals"));
        return *cached_proposals_;
    }
    inline const Presence<std::vector<NameOdds>>& RecognitionOddsRecognizeAnswer::names() const {
        if (!cached_names_) cached_names_ = std::make_shared<const Presence<std::vector<NameOdds>>>(detail::member_value<std::vector<NameOdds>>(value_,
        "names"));
        return *cached_names_;
    }
    inline const Presence<std::vector<PairOdds>>& RecognitionOddsRecognizeAnswer::pairs() const {
        if (!cached_pairs_) cached_pairs_ = std::make_shared<const Presence<std::vector<PairOdds>>>(detail::member_value<std::vector<PairOdds>>(value_,
        "pairs"));
        return *cached_pairs_;
    }
    inline const Presence<std::vector<PieceOdds>>& RecognitionOddsRecognizeAnswer::pieces() const {
        if (!cached_pieces_) cached_pieces_ = std::make_shared<const Presence<std::vector<PieceOdds>>>(detail::member_value<std::vector<PieceOdds>>(value_,
        "pieces"));
        return *cached_pieces_;
    }
    inline const Presence<std::vector<RecognitionProposal>>& RecognitionOddsRecognizeAnswer::proposals() const {
        if (!cached_proposals_) cached_proposals_ = std::make_shared<const Presence<std::vector<RecognitionProposal>>>(detail::member_value<std::vector<RecognitionProposal>>(value_,
        "proposals"));
        return *cached_proposals_;
    }
    inline const Presence<uint64_t>& RecognitionProposal::end() const {
        if (!cached_end_) cached_end_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "end"));
        return *cached_end_;
    }
    inline const Presence<bool>& RecognitionProposal::kept() const {
        if (!cached_kept_) cached_kept_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "kept"));
        return *cached_kept_;
    }
    inline const Presence<std::string>& RecognitionProposal::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<Place>& RecognitionProposal::selected() const {
        if (!cached_selected_) cached_selected_ = std::make_shared<const Presence<Place>>(detail::member_value<Place>(value_,
        "selected"));
        return *cached_selected_;
    }
    inline const Presence<double>& RecognitionProposal::span_probability() const {
        if (!cached_span_probability_) cached_span_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "span_probability"));
        return *cached_span_probability_;
    }
    inline const Presence<uint64_t>& RecognitionProposal::start() const {
        if (!cached_start_) cached_start_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "start"));
        return *cached_start_;
    }
    inline const Presence<double>& RecognitionProposal::strength() const {
        if (!cached_strength_) cached_strength_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "strength"));
        return *cached_strength_;
    }
    inline const Presence<std::string>& RecognitionStageContext::boundary() const {
        if (!cached_boundary_) cached_boundary_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "boundary"));
        return *cached_boundary_;
    }
    inline const Presence<std::string>& RecognitionStageContext::kind_edge() const {
        if (!cached_kind_edge_) cached_kind_edge_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind_edge"));
        return *cached_kind_edge_;
    }
    inline const Presence<std::string>& RecognitionStageContext::relation() const {
        if (!cached_relation_) cached_relation_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "relation"));
        return *cached_relation_;
    }
    inline const Presence<Answers>& Relation::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answers>>(detail::member_value<Answers>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& Relation::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<std::string>& Relation::file() const {
        if (!cached_file_) cached_file_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "file"));
        return *cached_file_;
    }
    inline const Presence<uint64_t>& Relation::first_line() const {
        if (!cached_first_line_) cached_first_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "first_line"));
        return *cached_first_line_;
    }
    inline const Presence<uint64_t>& Relation::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<Json>& Relation::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<std::vector<SessionInputSource>>& Relation::input_sources() const {
        if (!cached_input_sources_) cached_input_sources_ = std::make_shared<const Presence<std::vector<SessionInputSource>>>(detail::member_value<std::vector<SessionInputSource>>(value_,
        "input_sources"));
        return *cached_input_sources_;
    }
    inline const Presence<uint64_t>& Relation::last_line() const {
        if (!cached_last_line_) cached_last_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_line"));
        return *cached_last_line_;
    }
    inline const Presence<Meta>& Relation::meta() const {
        if (!cached_meta_) cached_meta_ = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
        "meta"));
        return *cached_meta_;
    }
    inline const Presence<Position>& Relation::position() const {
        if (!cached_position_) cached_position_ = std::make_shared<const Presence<Position>>(detail::member_value<Position>(value_,
        "position"));
        return *cached_position_;
    }
    inline const Presence<ReadableQuestion4>& Relation::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion4>>(detail::member_value<ReadableQuestion4>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<Version>& Relation::schema() const {
        if (!cached_schema_) cached_schema_ = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
        "schema"));
        return *cached_schema_;
    }
    inline const Presence<std::vector<RelatedEntityEdge>>& Relation::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<RelatedEntityEdge>>>(detail::member_value<std::vector<RelatedEntityEdge>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const std::string& RelationDirection::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline std::optional<RelationEndpointRelatedEntity> RelationEndpoint::as_RelationEndpointRelatedEntity() const {
        if (value_.contains("kind") && value_.contains("name") && !value_.contains("file") && !value_.contains("ordinal") && !value_.contains("record")) return RelationEndpointRelatedEntity(value_);
        return std::nullopt;
    }
    inline std::optional<RelationEndpointSource> RelationEndpoint::as_RelationEndpointSource() const {
        if (value_.contains("file") && value_.contains("kind") && value_.contains("name") && value_.contains("ordinal") && value_.contains("record")) return RelationEndpointSource(value_);
        return std::nullopt;
    }
    inline const Presence<std::string>& RelationEndpointRelatedEntity::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& RelationEndpointRelatedEntity::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::string>& RelationEndpointSource::file() const {
        if (!cached_file_) cached_file_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "file"));
        return *cached_file_;
    }
    inline const Presence<uint64_t>& RelationEndpointSource::first_line() const {
        if (!cached_first_line_) cached_first_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "first_line"));
        return *cached_first_line_;
    }
    inline const Presence<std::string>& RelationEndpointSource::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<uint64_t>& RelationEndpointSource::last_line() const {
        if (!cached_last_line_) cached_last_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_line"));
        return *cached_last_line_;
    }
    inline const Presence<std::string>& RelationEndpointSource::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<uint64_t>& RelationEndpointSource::ordinal() const {
        if (!cached_ordinal_) cached_ordinal_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "ordinal"));
        return *cached_ordinal_;
    }
    inline const Presence<Json>& RelationEndpointSource::record() const {
        if (!cached_record_) cached_record_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "record"));
        return *cached_record_;
    }
    inline std::optional<RelationMemberAnswerId> RelationMember::as_RelationMemberAnswerId() const {
        if (value_.contains("answer_id")) return RelationMemberAnswerId(value_);
        return std::nullopt;
    }
    inline std::optional<RelationMemberFailureId> RelationMember::as_RelationMemberFailureId() const {
        if (value_.contains("failure_id")) return RelationMemberFailureId(value_);
        return std::nullopt;
    }
    inline const Presence<RelationDirection>& RelationMemberAnswerId::direction() const {
        if (!cached_direction_) cached_direction_ = std::make_shared<const Presence<RelationDirection>>(detail::member_value<RelationDirection>(value_,
        "direction"));
        return *cached_direction_;
    }
    inline const Presence<RelationMethod>& RelationMemberAnswerId::method() const {
        if (!cached_method_) cached_method_ = std::make_shared<const Presence<RelationMethod>>(detail::member_value<RelationMethod>(value_,
        "method"));
        return *cached_method_;
    }
    inline const Presence<std::vector<Observation>>& RelationMemberAnswerId::observations() const {
        if (!cached_observations_) cached_observations_ = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
        "observations"));
        return *cached_observations_;
    }
    inline const Presence<ReadableQuestion>& RelationMemberAnswerId::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::vector<QuestionSource>>& RelationMemberAnswerId::question_sources() const {
        if (!cached_question_sources_) cached_question_sources_ = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
        "question_sources"));
        return *cached_question_sources_;
    }
    inline const Presence<std::string>& RelationMemberAnswerId::reads() const {
        if (!cached_reads_) cached_reads_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "reads"));
        return *cached_reads_;
    }
    inline const Presence<std::string>& RelationMemberAnswerId::relation() const {
        if (!cached_relation_) cached_relation_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "relation"));
        return *cached_relation_;
    }
    inline const Presence<std::string>& RelationMemberAnswerId::request() const {
        if (!cached_request_) cached_request_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "request"));
        return *cached_request_;
    }
    inline const Presence<RelatedEntity>& RelationMemberAnswerId::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<RelatedEntity>>(detail::member_value<RelatedEntity>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<RelatedEntity>& RelationMemberAnswerId::target() const {
        if (!cached_target_) cached_target_ = std::make_shared<const Presence<RelatedEntity>>(detail::member_value<RelatedEntity>(value_,
        "target"));
        return *cached_target_;
    }
    inline const Presence<Threshold>& RelationMemberAnswerId::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<Usage>& RelationMemberAnswerId::usage() const {
        if (!cached_usage_) cached_usage_ = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
        "usage"));
        return *cached_usage_;
    }
    inline const Presence<bool>& RelationMemberAnswerId::accepted() const {
        if (!cached_accepted_) cached_accepted_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "accepted"));
        return *cached_accepted_;
    }
    inline const Presence<Answer>& RelationMemberAnswerId::answer() const {
        if (!cached_answer_) cached_answer_ = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
        "answer"));
        return *cached_answer_;
    }
    inline const Presence<AnswerId>& RelationMemberAnswerId::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<double>& RelationMemberAnswerId::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline std::string RelationMemberAnswerId::source_name() const {
        return value_.at("source").at("name").get<std::string>();
    }
    inline const Presence<RelationDirection>& RelationMemberFailureId::direction() const {
        if (!cached_direction_) cached_direction_ = std::make_shared<const Presence<RelationDirection>>(detail::member_value<RelationDirection>(value_,
        "direction"));
        return *cached_direction_;
    }
    inline const Presence<RelationMethod>& RelationMemberFailureId::method() const {
        if (!cached_method_) cached_method_ = std::make_shared<const Presence<RelationMethod>>(detail::member_value<RelationMethod>(value_,
        "method"));
        return *cached_method_;
    }
    inline const Presence<std::vector<Observation>>& RelationMemberFailureId::observations() const {
        if (!cached_observations_) cached_observations_ = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
        "observations"));
        return *cached_observations_;
    }
    inline const Presence<ReadableQuestion>& RelationMemberFailureId::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::vector<QuestionSource>>& RelationMemberFailureId::question_sources() const {
        if (!cached_question_sources_) cached_question_sources_ = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
        "question_sources"));
        return *cached_question_sources_;
    }
    inline const Presence<std::string>& RelationMemberFailureId::reads() const {
        if (!cached_reads_) cached_reads_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "reads"));
        return *cached_reads_;
    }
    inline const Presence<std::string>& RelationMemberFailureId::relation() const {
        if (!cached_relation_) cached_relation_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "relation"));
        return *cached_relation_;
    }
    inline const Presence<std::string>& RelationMemberFailureId::request() const {
        if (!cached_request_) cached_request_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "request"));
        return *cached_request_;
    }
    inline const Presence<RelatedEntity>& RelationMemberFailureId::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<RelatedEntity>>(detail::member_value<RelatedEntity>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<RelatedEntity>& RelationMemberFailureId::target() const {
        if (!cached_target_) cached_target_ = std::make_shared<const Presence<RelatedEntity>>(detail::member_value<RelatedEntity>(value_,
        "target"));
        return *cached_target_;
    }
    inline const Presence<Threshold>& RelationMemberFailureId::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<Usage>& RelationMemberFailureId::usage() const {
        if (!cached_usage_) cached_usage_ = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
        "usage"));
        return *cached_usage_;
    }
    inline const Presence<Failure>& RelationMemberFailureId::failure() const {
        if (!cached_failure_) cached_failure_ = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
        "failure"));
        return *cached_failure_;
    }
    inline const Presence<FailureId>& RelationMemberFailureId::failure_id() const {
        if (!cached_failure_id_) cached_failure_id_ = std::make_shared<const Presence<FailureId>>(detail::member_value<FailureId>(value_,
        "failure_id"));
        return *cached_failure_id_;
    }
    inline std::string RelationMemberFailureId::source_name() const {
        return value_.at("source").at("name").get<std::string>();
    }
    inline const std::string& RelationMethod::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const std::string& RequestFunction::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const std::string& SdkRequestId::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline std::optional<SendBudgetDenialBeforeFirstSend> SendBudgetDenial::as_SendBudgetDenialBeforeFirstSend() const {
        if (detail::literal(value_, "kind", "before_first_send")) return SendBudgetDenialBeforeFirstSend(value_);
        return std::nullopt;
    }
    inline std::optional<SendBudgetDenialBeforeAdditionalSend> SendBudgetDenial::as_SendBudgetDenialBeforeAdditionalSend() const {
        if (detail::literal(value_, "kind", "before_additional_send")) return SendBudgetDenialBeforeAdditionalSend(value_);
        return std::nullopt;
    }
    inline std::optional<SendBudgetDenialBeforeRetry> SendBudgetDenial::as_SendBudgetDenialBeforeRetry() const {
        if (detail::literal(value_, "kind", "before_retry")) return SendBudgetDenialBeforeRetry(value_);
        return std::nullopt;
    }
    inline const Presence<std::string>& SendBudgetDenialBeforeAdditionalSend::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& SendBudgetDenialBeforeFirstSend::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& SendBudgetDenialBeforeRetry::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<uint64_t>& SendBudgetDenialBeforeRetry::last_status() const {
        if (!cached_last_status_) cached_last_status_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_status"));
        return *cached_last_status_;
    }
    inline const std::string& StopCause::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Presence<uint64_t>& Stopped::at() const {
        if (!cached_at_) cached_at_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "at"));
        return *cached_at_;
    }
    inline const Presence<StopCause>& Stopped::cause() const {
        if (!cached_cause_) cached_cause_ = std::make_shared<const Presence<StopCause>>(detail::member_value<StopCause>(value_,
        "cause"));
        return *cached_cause_;
    }
    inline const Presence<bool>& Stopped::retryable() const {
        if (!cached_retryable_) cached_retryable_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "retryable"));
        return *cached_retryable_;
    }
    inline const Presence<uint64_t>& Stopped::status() const {
        if (!cached_status_) cached_status_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "status"));
        return *cached_status_;
    }
    inline const Presence<StringType>& StringRoot::type() const {
        if (!cached_type_) cached_type_ = std::make_shared<const Presence<StringType>>(detail::member_value<StringType>(value_,
        "type"));
        return *cached_type_;
    }
    inline const std::string& StringType::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Presence<uint64_t>& Usage::input_tokens() const {
        if (!cached_input_tokens_) cached_input_tokens_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "input_tokens"));
        return *cached_input_tokens_;
    }
    inline const Presence<uint64_t>& Usage::output_tokens() const {
        if (!cached_output_tokens_) cached_output_tokens_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "output_tokens"));
        return *cached_output_tokens_;
    }
    inline const std::string& UsagePersistence::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const std::string& Verb::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const std::string& Version::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const uint32_t& WordingVersion::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const uint32_t>(decode<uint32_t>(value_));
        return *cached_value_;
    }
    inline const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double,
    Failed>& AnnotatedField::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::variant<bool, std::nullptr_t,
        std::string, std::vector<std::string>, double, Failed>>(decode<std::variant<bool, std::nullptr_t,
        std::string, std::vector<std::string>, double, Failed>>(value_));
        return *cached_value_;
    }
    inline const std::vector<std::pair<std::string, AnnotatedField>>& AnnotatedRow::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::vector<std::pair<std::string,
        AnnotatedField>>>(decode<std::vector<std::pair<std::string, AnnotatedField>>>(value_));
        return *cached_value_;
    }
    inline std::optional<AnswerYesNo> Answer::as_AnswerYesNo() const {
        if (detail::literal(value_, "kind", "yes_no")) return AnswerYesNo(value_);
        return std::nullopt;
    }
    inline std::optional<AnswerChoice> Answer::as_AnswerChoice() const {
        if (detail::literal(value_, "kind", "choice")) return AnswerChoice(value_);
        return std::nullopt;
    }
    inline std::optional<AnswerTag> Answer::as_AnswerTag() const {
        if (detail::literal(value_, "kind", "tag")) return AnswerTag(value_);
        return std::nullopt;
    }
    inline std::optional<AnswerScore> Answer::as_AnswerScore() const {
        if (detail::literal(value_, "kind", "score")) return AnswerScore(value_);
        return std::nullopt;
    }
    inline const Presence<double>& AnswerChoice::confidence() const {
        if (!cached_confidence_) cached_confidence_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "confidence"));
        return *cached_confidence_;
    }
    inline const Presence<std::string>& AnswerChoice::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& AnswerChoice::pick() const {
        if (!cached_pick_) cached_pick_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "pick"));
        return *cached_pick_;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& AnswerChoice::probabilities() const {
        if (!cached_probabilities_) cached_probabilities_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        double>>>>(detail::member_value<std::vector<std::pair<std::string, double>>>(value_, "probabilities"));
        return *cached_probabilities_;
    }
    inline const Presence<double>& AnswerScore::confidence() const {
        if (!cached_confidence_) cached_confidence_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "confidence"));
        return *cached_confidence_;
    }
    inline const Presence<std::string>& AnswerScore::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& AnswerScore::level() const {
        if (!cached_level_) cached_level_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "level"));
        return *cached_level_;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& AnswerScore::probabilities() const {
        if (!cached_probabilities_) cached_probabilities_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        double>>>>(detail::member_value<std::vector<std::pair<std::string, double>>>(value_, "probabilities"));
        return *cached_probabilities_;
    }
    inline const Presence<std::string>& AnswerTag::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& AnswerTag::probabilities() const {
        if (!cached_probabilities_) cached_probabilities_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        double>>>>(detail::member_value<std::vector<std::pair<std::string, double>>>(value_, "probabilities"));
        return *cached_probabilities_;
    }
    inline const Presence<std::string>& AnswerYesNo::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<double>& AnswerYesNo::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline const std::string& AttemptOutcome::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const std::variant<uint64_t, std::string>& BatchSetting::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::variant<uint64_t, std::string>>(decode<std::variant<uint64_t,
        std::string>>(value_));
        return *cached_value_;
    }
    inline const Presence<BatchSetting>& BatchWarning::running() const {
        if (!cached_running_) cached_running_ = std::make_shared<const Presence<BatchSetting>>(detail::member_value<BatchSetting>(value_,
        "running"));
        return *cached_running_;
    }
    inline const Presence<BatchSetting>& BatchWarning::tuned_for() const {
        if (!cached_tuned_for_) cached_tuned_for_ = std::make_shared<const Presence<BatchSetting>>(detail::member_value<BatchSetting>(value_,
        "tuned_for"));
        return *cached_tuned_for_;
    }
    inline const Presence<uint64_t>& Entity::end() const {
        if (!cached_end_) cached_end_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "end"));
        return *cached_end_;
    }
    inline const Presence<std::string>& Entity::file() const {
        if (!cached_file_) cached_file_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "file"));
        return *cached_file_;
    }
    inline const Presence<uint64_t>& Entity::first_line() const {
        if (!cached_first_line_) cached_first_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "first_line"));
        return *cached_first_line_;
    }
    inline const Presence<std::string>& Entity::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<uint64_t>& Entity::last_line() const {
        if (!cached_last_line_) cached_last_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_line"));
        return *cached_last_line_;
    }
    inline const Presence<uint64_t>& Entity::length() const {
        if (!cached_length_) cached_length_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "length"));
        return *cached_length_;
    }
    inline const Presence<uint64_t>& Entity::start() const {
        if (!cached_start_) cached_start_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "start"));
        return *cached_start_;
    }
    inline const Presence<double>& Entity::strength() const {
        if (!cached_strength_) cached_strength_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "strength"));
        return *cached_strength_;
    }
    inline const Presence<std::string>& Entity::text() const {
        if (!cached_text_) cached_text_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "text"));
        return *cached_text_;
    }
    inline const Presence<bool>& EntityEdge::either() const {
        if (!cached_either_) cached_either_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "either"));
        return *cached_either_;
    }
    inline const Presence<double>& EntityEdge::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline const Presence<std::string>& EntityEdge::relation() const {
        if (!cached_relation_) cached_relation_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "relation"));
        return *cached_relation_;
    }
    inline const Presence<Entity>& EntityEdge::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<Entity>>(detail::member_value<Entity>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Entity>& EntityEdge::target() const {
        if (!cached_target_) cached_target_ = std::make_shared<const Presence<Entity>>(detail::member_value<Entity>(value_,
        "target"));
        return *cached_target_;
    }
    inline const Presence<Failure>& Failed::failed() const {
        if (!cached_failed_) cached_failed_ = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
        "failed"));
        return *cached_failed_;
    }
    inline const Presence<FailureCause>& Failure::cause() const {
        if (!cached_cause_) cached_cause_ = std::make_shared<const Presence<FailureCause>>(detail::member_value<FailureCause>(value_,
        "cause"));
        return *cached_cause_;
    }
    inline const Presence<std::string>& Failure::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const std::string& FailureCause::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const std::string& FailureKind::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::string>(decode<std::string>(value_));
        return *cached_value_;
    }
    inline const Presence<double>& FindAnswer::confidence() const {
        if (!cached_confidence_) cached_confidence_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "confidence"));
        return *cached_confidence_;
    }
    inline const Presence<std::string>& FindAnswer::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& FindAnswer::pick() const {
        if (!cached_pick_) cached_pick_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "pick"));
        return *cached_pick_;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& FindAnswer::probabilities() const {
        if (!cached_probabilities_) cached_probabilities_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        double>>>>(detail::member_value<std::vector<std::pair<std::string, double>>>(value_, "probabilities"));
        return *cached_probabilities_;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& NameOdds::edges() const {
        if (!cached_edges_) cached_edges_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        double>>>>(detail::member_value<std::vector<std::pair<std::string, double>>>(value_, "edges"));
        return *cached_edges_;
    }
    inline const Presence<uint64_t>& NameOdds::end() const {
        if (!cached_end_) cached_end_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "end"));
        return *cached_end_;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& NameOdds::kinds() const {
        if (!cached_kinds_) cached_kinds_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        double>>>>(detail::member_value<std::vector<std::pair<std::string, double>>>(value_, "kinds"));
        return *cached_kinds_;
    }
    inline const Presence<uint64_t>& NameOdds::start() const {
        if (!cached_start_) cached_start_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "start"));
        return *cached_start_;
    }
    inline const Presence<double>& PairOdds::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline const Presence<std::string>& PairOdds::relation() const {
        if (!cached_relation_) cached_relation_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "relation"));
        return *cached_relation_;
    }
    inline const Presence<Place>& PairOdds::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<Place>>(detail::member_value<Place>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<Place>& PairOdds::target() const {
        if (!cached_target_) cached_target_ = std::make_shared<const Presence<Place>>(detail::member_value<Place>(value_,
        "target"));
        return *cached_target_;
    }
    inline const Presence<uint64_t>& PieceOdds::end() const {
        if (!cached_end_) cached_end_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "end"));
        return *cached_end_;
    }
    inline const Presence<uint64_t>& PieceOdds::start() const {
        if (!cached_start_) cached_start_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "start"));
        return *cached_start_;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& PieceOdds::tags() const {
        if (!cached_tags_) cached_tags_ = std::make_shared<const Presence<std::vector<std::pair<std::string,
        double>>>>(detail::member_value<std::vector<std::pair<std::string, double>>>(value_, "tags"));
        return *cached_tags_;
    }
    inline const Presence<uint64_t>& Place::end() const {
        if (!cached_end_) cached_end_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "end"));
        return *cached_end_;
    }
    inline const Presence<uint64_t>& Place::start() const {
        if (!cached_start_) cached_start_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "start"));
        return *cached_start_;
    }
    inline const Presence<uint64_t>& Plan::estimated_bytes() const {
        if (!cached_estimated_bytes_) cached_estimated_bytes_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "estimated_bytes"));
        return *cached_estimated_bytes_;
    }
    inline const Presence<TokenBand>& Plan::estimated_input_tokens() const {
        if (!cached_estimated_input_tokens_) cached_estimated_input_tokens_ = std::make_shared<const Presence<TokenBand>>(detail::member_value<TokenBand>(value_,
        "estimated_input_tokens"));
        return *cached_estimated_input_tokens_;
    }
    inline const Presence<std::string>& Plan::first_body_utf8() const {
        if (!cached_first_body_utf8_) cached_first_body_utf8_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "first_body_utf8"));
        return *cached_first_body_utf8_;
    }
    inline const Presence<uint64_t>& Plan::largest_request_bytes() const {
        if (!cached_largest_request_bytes_) cached_largest_request_bytes_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "largest_request_bytes"));
        return *cached_largest_request_bytes_;
    }
    inline const Presence<uint64_t>& Plan::largest_request_estimated_input_tokens() const {
        if (!cached_largest_request_estimated_input_tokens_) cached_largest_request_estimated_input_tokens_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "largest_request_estimated_input_tokens"));
        return *cached_largest_request_estimated_input_tokens_;
    }
    inline const Presence<uint64_t>& Plan::records() const {
        if (!cached_records_) cached_records_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "records"));
        return *cached_records_;
    }
    inline const Presence<uint64_t>& Plan::requests() const {
        if (!cached_requests_) cached_requests_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "requests"));
        return *cached_requests_;
    }
    inline const Presence<std::string>& Plan::token_estimate_method() const {
        if (!cached_token_estimate_method_) cached_token_estimate_method_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "token_estimate_method"));
        return *cached_token_estimate_method_;
    }
    inline const Presence<bool>& Plan::upper_bound() const {
        if (!cached_upper_bound_) cached_upper_bound_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "upper_bound"));
        return *cached_upper_bound_;
    }
    inline const Presence<std::string>& ProfileWarning::running() const {
        if (!cached_running_) cached_running_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "running"));
        return *cached_running_;
    }
    inline const Presence<std::string>& ProfileWarning::tuned_for() const {
        if (!cached_tuned_for_) cached_tuned_for_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "tuned_for"));
        return *cached_tuned_for_;
    }
    inline std::optional<RecognizeEntities> Recognize::as_RecognizeEntities() const {
        if (value_.contains("entities") && !value_.contains("mode") && !value_.contains("proposals")) return RecognizeEntities(value_);
        return std::nullopt;
    }
    inline std::optional<RecognizeModeProposals> Recognize::as_RecognizeModeProposals() const {
        if (value_.contains("mode") && value_.contains("proposals") && !value_.contains("entities")) return RecognizeModeProposals(value_);
        return std::nullopt;
    }
    inline const Presence<std::vector<NameOdds>>& RecognizeAnswer::names() const {
        if (!cached_names_) cached_names_ = std::make_shared<const Presence<std::vector<NameOdds>>>(detail::member_value<std::vector<NameOdds>>(value_,
        "names"));
        return *cached_names_;
    }
    inline const Presence<std::vector<PairOdds>>& RecognizeAnswer::pairs() const {
        if (!cached_pairs_) cached_pairs_ = std::make_shared<const Presence<std::vector<PairOdds>>>(detail::member_value<std::vector<PairOdds>>(value_,
        "pairs"));
        return *cached_pairs_;
    }
    inline const Presence<std::vector<PieceOdds>>& RecognizeAnswer::pieces() const {
        if (!cached_pieces_) cached_pieces_ = std::make_shared<const Presence<std::vector<PieceOdds>>>(detail::member_value<std::vector<PieceOdds>>(value_,
        "pieces"));
        return *cached_pieces_;
    }
    inline const Presence<std::vector<RecognitionProposal>>& RecognizeAnswer::proposals() const {
        if (!cached_proposals_) cached_proposals_ = std::make_shared<const Presence<std::vector<RecognitionProposal>>>(detail::member_value<std::vector<RecognitionProposal>>(value_,
        "proposals"));
        return *cached_proposals_;
    }
    inline const Presence<std::vector<Entity>>& RecognizeEntities::entities() const {
        if (!cached_entities_) cached_entities_ = std::make_shared<const Presence<std::vector<Entity>>>(detail::member_value<std::vector<Entity>>(value_,
        "entities"));
        return *cached_entities_;
    }
    inline const Presence<std::vector<EntityEdge>>& RecognizeEntities::relations() const {
        if (!cached_relations_) cached_relations_ = std::make_shared<const Presence<std::vector<EntityEdge>>>(detail::member_value<std::vector<EntityEdge>>(value_,
        "relations"));
        return *cached_relations_;
    }
    inline const Presence<BoundaryMode>& RecognizeModeProposals::mode() const {
        if (!cached_mode_) cached_mode_ = std::make_shared<const Presence<BoundaryMode>>(detail::member_value<BoundaryMode>(value_,
        "mode"));
        return *cached_mode_;
    }
    inline const Presence<std::vector<BoundaryProposal>>& RecognizeModeProposals::proposals() const {
        if (!cached_proposals_) cached_proposals_ = std::make_shared<const Presence<std::vector<BoundaryProposal>>>(detail::member_value<std::vector<BoundaryProposal>>(value_,
        "proposals"));
        return *cached_proposals_;
    }
    inline const Presence<std::string>& RelateFields::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& RelateFields::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::string>& RelatedEntity::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& RelatedEntity::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<bool>& RelatedEntityEdge::either() const {
        if (!cached_either_) cached_either_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "either"));
        return *cached_either_;
    }
    inline const Presence<double>& RelatedEntityEdge::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline const Presence<std::string>& RelatedEntityEdge::relation() const {
        if (!cached_relation_) cached_relation_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "relation"));
        return *cached_relation_;
    }
    inline const Presence<RelationEndpoint>& RelatedEntityEdge::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<RelationEndpoint>>(detail::member_value<RelationEndpoint>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<RelationEndpoint>& RelatedEntityEdge::target() const {
        if (!cached_target_) cached_target_ = std::make_shared<const Presence<RelationEndpoint>>(detail::member_value<RelationEndpoint>(value_,
        "target"));
        return *cached_target_;
    }
    inline std::string RelatedEntityEdge::source_name() const {
        return value_.at("source").at("name").get<std::string>();
    }
    inline std::string RelatedEntityEdge::target_name() const {
        return value_.at("target").at("name").get<std::string>();
    }
    inline const Presence<bool>& RelationRule::either() const {
        if (!cached_either_) cached_either_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "either"));
        return *cached_either_;
    }
    inline const Presence<std::string>& RelationRule::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<std::string>& RelationRule::reads() const {
        if (!cached_reads_) cached_reads_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "reads"));
        return *cached_reads_;
    }
    inline const Presence<bool>& RelationRule::single() const {
        if (!cached_single_) cached_single_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "single"));
        return *cached_single_;
    }
    inline const Presence<std::string>& RelationRule::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<std::string>& RelationRule::target() const {
        if (!cached_target_) cached_target_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "target"));
        return *cached_target_;
    }
    inline const Presence<std::string>& SessionAnnotation::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<AnnotationValue>& SessionAnnotation::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<AnnotationValue>>(detail::member_value<AnnotationValue>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<uint64_t>& SessionInputSource::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<PhysicalSource>& SessionInputSource::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "source"));
        return *cached_source_;
    }
    inline std::optional<SessionJudgmentDecision> SessionJudgment::as_SessionJudgmentDecision() const {
        if (detail::literal(value_, "kind", "decision")) return SessionJudgmentDecision(value_);
        return std::nullopt;
    }
    inline std::optional<SessionJudgmentChoice> SessionJudgment::as_SessionJudgmentChoice() const {
        if (detail::literal(value_, "kind", "choice")) return SessionJudgmentChoice(value_);
        return std::nullopt;
    }
    inline std::optional<SessionJudgmentScore> SessionJudgment::as_SessionJudgmentScore() const {
        if (detail::literal(value_, "kind", "score")) return SessionJudgmentScore(value_);
        return std::nullopt;
    }
    inline std::optional<SessionJudgmentTags> SessionJudgment::as_SessionJudgmentTags() const {
        if (detail::literal(value_, "kind", "tags")) return SessionJudgmentTags(value_);
        return std::nullopt;
    }
    inline const Presence<std::string>& SessionJudgmentChoice::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& SessionJudgmentChoice::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionJudgmentDecision::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<bool>& SessionJudgmentDecision::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionJudgmentScore::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<double>& SessionJudgmentScore::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionJudgmentTags::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<std::string>>& SessionJudgmentTags::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionNamedProbability::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<double>& SessionNamedProbability::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline std::optional<SessionObservationQuestion> SessionObservation::as_SessionObservationQuestion() const {
        if (detail::literal(value_, "kind", "question")) return SessionObservationQuestion(value_);
        return std::nullopt;
    }
    inline std::optional<SessionObservationRow> SessionObservation::as_SessionObservationRow() const {
        if (detail::literal(value_, "kind", "row")) return SessionObservationRow(value_);
        return std::nullopt;
    }
    inline const Presence<SessionQuestionDetail>& SessionObservationQuestion::detail() const {
        if (!cached_detail_) cached_detail_ = std::make_shared<const Presence<SessionQuestionDetail>>(detail::member_value<SessionQuestionDetail>(value_,
        "detail"));
        return *cached_detail_;
    }
    inline const Presence<uint64_t>& SessionObservationQuestion::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<std::string>& SessionObservationQuestion::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::string>& SessionObservationQuestion::member() const {
        if (!cached_member_) cached_member_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "member"));
        return *cached_member_;
    }
    inline const Presence<uint64_t>& SessionObservationQuestion::position() const {
        if (!cached_position_) cached_position_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "position"));
        return *cached_position_;
    }
    inline const Presence<std::string>& SessionObservationQuestion::stage() const {
        if (!cached_stage_) cached_stage_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "stage"));
        return *cached_stage_;
    }
    inline const Presence<uint64_t>& SessionObservationRow::index() const {
        if (!cached_index_) cached_index_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "index"));
        return *cached_index_;
    }
    inline const Presence<std::string>& SessionObservationRow::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<SessionObservedRow>& SessionObservationRow::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<SessionObservedRow>>(detail::member_value<SessionObservedRow>(value_,
        "value"));
        return *cached_value_;
    }
    inline std::optional<SessionObservedRowJudgment> SessionObservedRow::as_SessionObservedRowJudgment() const {
        if (detail::literal(value_, "kind", "judgment")) return SessionObservedRowJudgment(value_);
        return std::nullopt;
    }
    inline std::optional<SessionObservedRowAnnotated> SessionObservedRow::as_SessionObservedRowAnnotated() const {
        if (detail::literal(value_, "kind", "annotated")) return SessionObservedRowAnnotated(value_);
        return std::nullopt;
    }
    inline std::optional<SessionObservedRowRecognized> SessionObservedRow::as_SessionObservedRowRecognized() const {
        if (detail::literal(value_, "kind", "recognized")) return SessionObservedRowRecognized(value_);
        return std::nullopt;
    }
    inline std::optional<SessionObservedRowFind> SessionObservedRow::as_SessionObservedRowFind() const {
        if (detail::literal(value_, "kind", "find")) return SessionObservedRowFind(value_);
        return std::nullopt;
    }
    inline std::optional<SessionObservedRowRelations> SessionObservedRow::as_SessionObservedRowRelations() const {
        if (detail::literal(value_, "kind", "relations")) return SessionObservedRowRelations(value_);
        return std::nullopt;
    }
    inline const Presence<std::string>& SessionObservedRowAnnotated::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<SessionAnnotation>>& SessionObservedRowAnnotated::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<SessionAnnotation>>>(detail::member_value<std::vector<SessionAnnotation>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionObservedRowFind::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<uint64_t>& SessionObservedRowFind::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionObservedRowJudgment::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<SessionJudgment>& SessionObservedRowJudgment::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<SessionJudgment>>(detail::member_value<SessionJudgment>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionObservedRowRecognized::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<SessionRecognition>& SessionObservedRowRecognized::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<SessionRecognition>>(detail::member_value<SessionRecognition>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionObservedRowRelations::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<SessionRelationEdge>>& SessionObservedRowRelations::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<SessionRelationEdge>>>(detail::member_value<std::vector<SessionRelationEdge>>(value_,
        "value"));
        return *cached_value_;
    }
    inline std::optional<SessionPacketDecideRow> SessionPacket::as_SessionPacketDecideRow() const {
        if (detail::literal(value_, "function", "decide") && detail::literal(value_, "kind", "row")) return SessionPacketDecideRow(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketChooseRow> SessionPacket::as_SessionPacketChooseRow() const {
        if (detail::literal(value_, "function", "choose") && detail::literal(value_, "kind", "row")) return SessionPacketChooseRow(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketTagRow> SessionPacket::as_SessionPacketTagRow() const {
        if (detail::literal(value_, "function", "tag") && detail::literal(value_, "kind", "row")) return SessionPacketTagRow(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketScoreRow> SessionPacket::as_SessionPacketScoreRow() const {
        if (detail::literal(value_, "function", "score") && detail::literal(value_, "kind", "row")) return SessionPacketScoreRow(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketFilterRow> SessionPacket::as_SessionPacketFilterRow() const {
        if (detail::literal(value_, "function", "filter") && detail::literal(value_, "kind", "row")) return SessionPacketFilterRow(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketAnnotateRow> SessionPacket::as_SessionPacketAnnotateRow() const {
        if (detail::literal(value_, "function", "annotate") && detail::literal(value_, "kind", "row")) return SessionPacketAnnotateRow(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketDecideAggregate> SessionPacket::as_SessionPacketDecideAggregate() const {
        if (detail::literal(value_, "function", "decide") && detail::literal(value_, "kind", "aggregate")) return SessionPacketDecideAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketChooseAggregate> SessionPacket::as_SessionPacketChooseAggregate() const {
        if (detail::literal(value_, "function", "choose") && detail::literal(value_, "kind", "aggregate")) return SessionPacketChooseAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketTagAggregate> SessionPacket::as_SessionPacketTagAggregate() const {
        if (detail::literal(value_, "function", "tag") && detail::literal(value_, "kind", "aggregate")) return SessionPacketTagAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketScoreAggregate> SessionPacket::as_SessionPacketScoreAggregate() const {
        if (detail::literal(value_, "function", "score") && detail::literal(value_, "kind", "aggregate")) return SessionPacketScoreAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketFilterAggregate> SessionPacket::as_SessionPacketFilterAggregate() const {
        if (detail::literal(value_, "function", "filter") && detail::literal(value_, "kind", "aggregate")) return SessionPacketFilterAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketRankAggregate> SessionPacket::as_SessionPacketRankAggregate() const {
        if (detail::literal(value_, "function", "rank") && detail::literal(value_, "kind", "aggregate")) return SessionPacketRankAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketFindAggregate> SessionPacket::as_SessionPacketFindAggregate() const {
        if (detail::literal(value_, "function", "find") && detail::literal(value_, "kind", "aggregate")) return SessionPacketFindAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketAnnotateAggregate> SessionPacket::as_SessionPacketAnnotateAggregate() const {
        if (detail::literal(value_, "function", "annotate") && detail::literal(value_, "kind", "aggregate")) return SessionPacketAnnotateAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketRecognizeAggregate> SessionPacket::as_SessionPacketRecognizeAggregate() const {
        if (detail::literal(value_, "function", "recognize") && detail::literal(value_, "kind", "aggregate")) return SessionPacketRecognizeAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketRelateAggregate> SessionPacket::as_SessionPacketRelateAggregate() const {
        if (detail::literal(value_, "function", "relate") && detail::literal(value_, "kind", "aggregate")) return SessionPacketRelateAggregate(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketObservation> SessionPacket::as_SessionPacketObservation() const {
        if (detail::literal(value_, "kind", "observation")) return SessionPacketObservation(value_);
        return std::nullopt;
    }
    inline std::optional<SessionPacketTerminal> SessionPacket::as_SessionPacketTerminal() const {
        if (detail::literal(value_, "kind", "terminal")) return SessionPacketTerminal(value_);
        return std::nullopt;
    }
    inline const Presence<std::string>& SessionPacketAnnotateAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketAnnotateAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<Annotation>>& SessionPacketAnnotateAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<Annotation>>>(detail::member_value<std::vector<Annotation>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketAnnotateRow::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketAnnotateRow::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<Annotation>& SessionPacketAnnotateRow::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<Annotation>>(detail::member_value<Annotation>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketChooseAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketChooseAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<AtomicNullableString>>& SessionPacketChooseAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<AtomicNullableString>>>(detail::member_value<std::vector<AtomicNullableString>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketChooseRow::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketChooseRow::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<AtomicNullableString>& SessionPacketChooseRow::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<AtomicNullableString>>(detail::member_value<AtomicNullableString>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketDecideAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketDecideAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<AtomicDecideValue>>& SessionPacketDecideAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<AtomicDecideValue>>>(detail::member_value<std::vector<AtomicDecideValue>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketDecideRow::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketDecideRow::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<AtomicDecideValue>& SessionPacketDecideRow::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<AtomicDecideValue>>(detail::member_value<AtomicDecideValue>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketFilterAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketFilterAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<AtomicBoolean>>& SessionPacketFilterAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<AtomicBoolean>>>(detail::member_value<std::vector<AtomicBoolean>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketFilterRow::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketFilterRow::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<AtomicBoolean>& SessionPacketFilterRow::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<AtomicBoolean>>(detail::member_value<AtomicBoolean>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketFindAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketFindAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<Find>& SessionPacketFindAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<Find>>(detail::member_value<Find>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<RequestFunction>& SessionPacketObservation::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<RequestFunction>>(detail::member_value<RequestFunction>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketObservation::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<SessionObservation>& SessionPacketObservation::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<SessionObservation>>(detail::member_value<SessionObservation>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketRankAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketRankAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<AtomicNonZeroUsize>>& SessionPacketRankAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<AtomicNonZeroUsize>>>(detail::member_value<std::vector<AtomicNonZeroUsize>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketRecognizeAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketRecognizeAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<Recognition>>& SessionPacketRecognizeAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<Recognition>>>(detail::member_value<std::vector<Recognition>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketRelateAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketRelateAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<Relation>& SessionPacketRelateAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<Relation>>(detail::member_value<Relation>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketScoreAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketScoreAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<AtomicDouble>>& SessionPacketScoreAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<AtomicDouble>>>(detail::member_value<std::vector<AtomicDouble>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketScoreRow::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketScoreRow::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<AtomicDouble>& SessionPacketScoreRow::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<AtomicDouble>>(detail::member_value<AtomicDouble>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketTagAggregate::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketTagAggregate::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<AtomicArrayOfString>>& SessionPacketTagAggregate::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<AtomicArrayOfString>>>(detail::member_value<std::vector<AtomicArrayOfString>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionPacketTagRow::function() const {
        if (!cached_function_) cached_function_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "function"));
        return *cached_function_;
    }
    inline const Presence<std::string>& SessionPacketTagRow::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<AtomicArrayOfString>& SessionPacketTagRow::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<AtomicArrayOfString>>(detail::member_value<AtomicArrayOfString>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<Facts>& SessionPacketTerminal::facts() const {
        if (!cached_facts_) cached_facts_ = std::make_shared<const Presence<Facts>>(detail::member_value<Facts>(value_,
        "facts"));
        return *cached_facts_;
    }
    inline const Presence<CallError>& SessionPacketTerminal::failure() const {
        if (!cached_failure_) cached_failure_ = std::make_shared<const Presence<CallError>>(detail::member_value<CallError>(value_,
        "failure"));
        return *cached_failure_;
    }
    inline const Presence<std::string>& SessionPacketTerminal::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline std::optional<SessionProbabilitiesYesNo> SessionProbabilities::as_SessionProbabilitiesYesNo() const {
        if (detail::literal(value_, "kind", "yes_no")) return SessionProbabilitiesYesNo(value_);
        return std::nullopt;
    }
    inline std::optional<SessionProbabilitiesNamed> SessionProbabilities::as_SessionProbabilitiesNamed() const {
        if (detail::literal(value_, "kind", "named")) return SessionProbabilitiesNamed(value_);
        return std::nullopt;
    }
    inline const Presence<std::string>& SessionProbabilitiesNamed::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<std::vector<SessionNamedProbability>>& SessionProbabilitiesNamed::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<std::vector<SessionNamedProbability>>>(detail::member_value<std::vector<SessionNamedProbability>>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::string>& SessionProbabilitiesYesNo::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<double>& SessionProbabilitiesYesNo::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<AnswerId>& SessionQuestionDetail::answer_id() const {
        if (!cached_answer_id_) cached_answer_id_ = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
        "answer_id"));
        return *cached_answer_id_;
    }
    inline const Presence<bool>& SessionQuestionDetail::cached() const {
        if (!cached_cached_) cached_cached_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "cached"));
        return *cached_cached_;
    }
    inline const Presence<double>& SessionQuestionDetail::confidence() const {
        if (!cached_confidence_) cached_confidence_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "confidence"));
        return *cached_confidence_;
    }
    inline const Presence<uint64_t>& SessionQuestionDetail::failed_questions() const {
        if (!cached_failed_questions_) cached_failed_questions_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "failed_questions"));
        return *cached_failed_questions_;
    }
    inline const Presence<Failure>& SessionQuestionDetail::failure() const {
        if (!cached_failure_) cached_failure_ = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
        "failure"));
        return *cached_failure_;
    }
    inline const Presence<FailureId>& SessionQuestionDetail::failure_id() const {
        if (!cached_failure_id_) cached_failure_id_ = std::make_shared<const Presence<FailureId>>(detail::member_value<FailureId>(value_,
        "failure_id"));
        return *cached_failure_id_;
    }
    inline const Presence<Json>& SessionQuestionDetail::input() const {
        if (!cached_input_) cached_input_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "input"));
        return *cached_input_;
    }
    inline const Presence<PhysicalSource>& SessionQuestionDetail::input_source() const {
        if (!cached_input_source_) cached_input_source_ = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
        "input_source"));
        return *cached_input_source_;
    }
    inline const Presence<std::vector<SessionInputSource>>& SessionQuestionDetail::input_sources() const {
        if (!cached_input_sources_) cached_input_sources_ = std::make_shared<const Presence<std::vector<SessionInputSource>>>(detail::member_value<std::vector<SessionInputSource>>(value_,
        "input_sources"));
        return *cached_input_sources_;
    }
    inline const Presence<std::vector<Json>>& SessionQuestionDetail::inputs() const {
        if (!cached_inputs_) cached_inputs_ = std::make_shared<const Presence<std::vector<Json>>>(detail::member_value<std::vector<Json>>(value_,
        "inputs"));
        return *cached_inputs_;
    }
    inline const Presence<std::string>& SessionQuestionDetail::model() const {
        if (!cached_model_) cached_model_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "model"));
        return *cached_model_;
    }
    inline const Presence<std::vector<Observation>>& SessionQuestionDetail::observations() const {
        if (!cached_observations_) cached_observations_ = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
        "observations"));
        return *cached_observations_;
    }
    inline const Presence<SessionProbabilities>& SessionQuestionDetail::probabilities() const {
        if (!cached_probabilities_) cached_probabilities_ = std::make_shared<const Presence<SessionProbabilities>>(detail::member_value<SessionProbabilities>(value_,
        "probabilities"));
        return *cached_probabilities_;
    }
    inline const Presence<ReadableQuestion>& SessionQuestionDetail::question() const {
        if (!cached_question_) cached_question_ = std::make_shared<const Presence<ReadableQuestion>>(detail::member_value<ReadableQuestion>(value_,
        "question"));
        return *cached_question_;
    }
    inline const Presence<std::string>& SessionQuestionDetail::question_sha256() const {
        if (!cached_question_sha256_) cached_question_sha256_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "question_sha256"));
        return *cached_question_sha256_;
    }
    inline const Presence<std::vector<QuestionSource>>& SessionQuestionDetail::question_sources() const {
        if (!cached_question_sources_) cached_question_sources_ = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
        "question_sources"));
        return *cached_question_sources_;
    }
    inline const Presence<std::string>& SessionQuestionDetail::raw_pick() const {
        if (!cached_raw_pick_) cached_raw_pick_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "raw_pick"));
        return *cached_raw_pick_;
    }
    inline const Presence<Usage>& SessionQuestionDetail::reported_usage() const {
        if (!cached_reported_usage_) cached_reported_usage_ = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
        "reported_usage"));
        return *cached_reported_usage_;
    }
    inline const Presence<std::vector<std::string>>& SessionQuestionDetail::requests() const {
        if (!cached_requests_) cached_requests_ = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
        "requests"));
        return *cached_requests_;
    }
    inline const Presence<uint64_t>& SessionQuestionDetail::requests_sent() const {
        if (!cached_requests_sent_) cached_requests_sent_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "requests_sent"));
        return *cached_requests_sent_;
    }
    inline const Presence<Threshold>& SessionQuestionDetail::threshold() const {
        if (!cached_threshold_) cached_threshold_ = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
        "threshold"));
        return *cached_threshold_;
    }
    inline const Presence<std::string>& SessionQuestionDetail::url() const {
        if (!cached_url_) cached_url_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "url"));
        return *cached_url_;
    }
    inline const Presence<TokenUsage>& SessionQuestionDetail::usage() const {
        if (!cached_usage_) cached_usage_ = std::make_shared<const Presence<TokenUsage>>(detail::member_value<TokenUsage>(value_,
        "usage"));
        return *cached_usage_;
    }
    inline const Presence<Value>& SessionQuestionDetail::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const Presence<Value>>(detail::member_value<Value>(value_,
        "value"));
        return *cached_value_;
    }
    inline const Presence<std::vector<Entity>>& SessionRecognition::entities() const {
        if (!cached_entities_) cached_entities_ = std::make_shared<const Presence<std::vector<Entity>>>(detail::member_value<std::vector<Entity>>(value_,
        "entities"));
        return *cached_entities_;
    }
    inline const Presence<RecognitionMode>& SessionRecognition::mode() const {
        if (!cached_mode_) cached_mode_ = std::make_shared<const Presence<RecognitionMode>>(detail::member_value<RecognitionMode>(value_,
        "mode"));
        return *cached_mode_;
    }
    inline const Presence<std::vector<BoundaryProposal>>& SessionRecognition::proposals() const {
        if (!cached_proposals_) cached_proposals_ = std::make_shared<const Presence<std::vector<BoundaryProposal>>>(detail::member_value<std::vector<BoundaryProposal>>(value_,
        "proposals"));
        return *cached_proposals_;
    }
    inline const Presence<std::vector<RecognitionEdgeDocument>>& SessionRecognition::relations() const {
        if (!cached_relations_) cached_relations_ = std::make_shared<const Presence<std::vector<RecognitionEdgeDocument>>>(detail::member_value<std::vector<RecognitionEdgeDocument>>(value_,
        "relations"));
        return *cached_relations_;
    }
    inline const Presence<bool>& SessionRelationEdge::either() const {
        if (!cached_either_) cached_either_ = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
        "either"));
        return *cached_either_;
    }
    inline const Presence<double>& SessionRelationEdge::probability() const {
        if (!cached_probability_) cached_probability_ = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
        "probability"));
        return *cached_probability_;
    }
    inline const Presence<std::string>& SessionRelationEdge::relation() const {
        if (!cached_relation_) cached_relation_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "relation"));
        return *cached_relation_;
    }
    inline const Presence<EntityDocument>& SessionRelationEdge::source() const {
        if (!cached_source_) cached_source_ = std::make_shared<const Presence<EntityDocument>>(detail::member_value<EntityDocument>(value_,
        "source"));
        return *cached_source_;
    }
    inline const Presence<EntityDocument>& SessionRelationEdge::target() const {
        if (!cached_target_) cached_target_ = std::make_shared<const Presence<EntityDocument>>(detail::member_value<EntityDocument>(value_,
        "target"));
        return *cached_target_;
    }
    inline std::string SessionRelationEdge::source_name() const {
        return value_.at("source").at("name").get<std::string>();
    }
    inline std::string SessionRelationEdge::target_name() const {
        return value_.at("target").at("name").get<std::string>();
    }
    inline const Presence<std::string>& SourceRelationEndpoint::file() const {
        if (!cached_file_) cached_file_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "file"));
        return *cached_file_;
    }
    inline const Presence<uint64_t>& SourceRelationEndpoint::first_line() const {
        if (!cached_first_line_) cached_first_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "first_line"));
        return *cached_first_line_;
    }
    inline const Presence<std::string>& SourceRelationEndpoint::kind() const {
        if (!cached_kind_) cached_kind_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "kind"));
        return *cached_kind_;
    }
    inline const Presence<uint64_t>& SourceRelationEndpoint::last_line() const {
        if (!cached_last_line_) cached_last_line_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "last_line"));
        return *cached_last_line_;
    }
    inline const Presence<std::string>& SourceRelationEndpoint::name() const {
        if (!cached_name_) cached_name_ = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
        "name"));
        return *cached_name_;
    }
    inline const Presence<uint64_t>& SourceRelationEndpoint::ordinal() const {
        if (!cached_ordinal_) cached_ordinal_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "ordinal"));
        return *cached_ordinal_;
    }
    inline const Presence<Json>& SourceRelationEndpoint::record() const {
        if (!cached_record_) cached_record_ = std::make_shared<const Presence<Json>>(detail::member_value<Json>(value_,
        "record"));
        return *cached_record_;
    }
    inline const std::variant<double, std::string>& Threshold::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::variant<double, std::string>>(decode<std::variant<double,
        std::string>>(value_));
        return *cached_value_;
    }
    inline const Presence<uint64_t>& TokenBand::lower() const {
        if (!cached_lower_) cached_lower_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "lower"));
        return *cached_lower_;
    }
    inline const Presence<uint64_t>& TokenBand::upper() const {
        if (!cached_upper_) cached_upper_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "upper"));
        return *cached_upper_;
    }
    inline const Presence<uint64_t>& TokenUsage::input_tokens() const {
        if (!cached_input_tokens_) cached_input_tokens_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "input_tokens"));
        return *cached_input_tokens_;
    }
    inline const Presence<uint64_t>& TokenUsage::output_tokens() const {
        if (!cached_output_tokens_) cached_output_tokens_ = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
        "output_tokens"));
        return *cached_output_tokens_;
    }
    inline const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double>& Value::value() const {
        if (!cached_value_) cached_value_ = std::make_shared<const std::variant<bool, std::nullptr_t,
        std::string, std::vector<std::string>, double>>(decode<std::variant<bool, std::nullptr_t, std::string,
        std::vector<std::string>, double>>(value_));
        return *cached_value_;
    }
}
namespace tt::results {
    class AnswerCardinalityFailure : public std::runtime_error {
        public:
        std::vector<SessionPacket> packets;
        AnswerCardinalityFailure(const char* message, const std::vector<SessionPacket>& values):std::runtime_error(message),
        packets(values) {
        }
    };
    inline void successful_answer(const std::vector<SessionPacket>& packets) {
        size_t terminals=0;
        for(const auto& packet:packets) if(detail::literal(packet.document(), "kind", "terminal")) {
            ++terminals;
            if(packet.document().contains("failure")) throw AnswerCardinalityFailure("terminal contains a failure",
            packets);
        }
        if(terminals!=1 || packets.empty() || !detail::literal(packets.back().document(),"kind","terminal")) throw AnswerCardinalityFailure("expected one final successful terminal",
        packets);
    }
    inline std::optional<bool> read_decide(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<std::optional<bool>> answer;
        for(const auto& packet:packets) if(detail::literal(packet.document(), "kind", "observation") && detail::literal(packet.document().at("value"),
        "kind", "row") && detail::literal(packet.document().at("value").at("value"), "kind", "judgment")) {
            const auto& judgment=packet.document().at("value").at("value").at("value");
            if(!detail::literal(judgment,"kind","decision")) continue;
            if(answer) throw AnswerCardinalityFailure("multiple matching judgments",packets);
            answer.emplace(judgment.at("value").is_null() ? std::optional<bool>{
            } : std::optional<bool>(decode<bool>(judgment.at("value"))));
        }
        if(!answer) throw AnswerCardinalityFailure("no matching judgment",packets);
        return std::move(*answer);
    }
    inline std::optional<std::string> read_choose(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<std::optional<std::string>> answer;
        for(const auto& packet:packets) if(detail::literal(packet.document(), "kind", "observation") && detail::literal(packet.document().at("value"),
        "kind", "row") && detail::literal(packet.document().at("value").at("value"), "kind", "judgment")) {
            const auto& judgment=packet.document().at("value").at("value").at("value");
            if(!detail::literal(judgment,"kind","choice")) continue;
            if(answer) throw AnswerCardinalityFailure("multiple matching judgments",packets);
            answer.emplace(judgment.at("value").is_null() ? std::optional<std::string>{
            } : std::optional<std::string>(decode<std::string>(judgment.at("value"))));
        }
        if(!answer) throw AnswerCardinalityFailure("no matching judgment",packets);
        return std::move(*answer);
    }
    inline double read_score(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<double> answer;
        for(const auto& packet:packets) if(detail::literal(packet.document(), "kind", "observation") && detail::literal(packet.document().at("value"),
        "kind", "row") && detail::literal(packet.document().at("value").at("value"), "kind", "judgment")) {
            const auto& judgment=packet.document().at("value").at("value").at("value");
            if(!detail::literal(judgment,"kind","score")) continue;
            if(answer) throw AnswerCardinalityFailure("multiple matching judgments",packets);
            answer.emplace(decode<double>(judgment.at("value")));
        }
        if(!answer) throw AnswerCardinalityFailure("no matching judgment",packets);
        return std::move(*answer);
    }
    inline std::vector<std::string> read_tag(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<std::vector<std::string>> answer;
        for(const auto& packet:packets) if(detail::literal(packet.document(), "kind", "observation") && detail::literal(packet.document().at("value"),
        "kind", "row") && detail::literal(packet.document().at("value").at("value"), "kind", "judgment")) {
            const auto& judgment=packet.document().at("value").at("value").at("value");
            if(!detail::literal(judgment,"kind","tags")) continue;
            if(answer) throw AnswerCardinalityFailure("multiple matching judgments",packets);
            answer.emplace(decode<std::vector<std::string>>(judgment.at("value")));
        }
        if(!answer) throw AnswerCardinalityFailure("no matching judgment",packets);
        return std::move(*answer);
    }
    inline std::vector<AtomicBoolean> read_filter(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::vector<AtomicBoolean> answer;
        for(const auto& packet:packets) if(auto row=packet.as_SessionPacketFilterRow()) answer.push_back(*row->value().value);
        return answer;
    }
    inline std::vector<AtomicNonZeroUsize> read_rank(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<std::vector<AtomicNonZeroUsize>> answer;
        for(const auto& packet:packets) if(auto aggregate=packet.as_SessionPacketRankAggregate()) {
            if(answer) throw AnswerCardinalityFailure("multiple matching aggregates",packets);
            answer.emplace(*aggregate->value().value);
        }
        if(!answer) throw AnswerCardinalityFailure("no matching aggregate",packets);
        return std::move(*answer);
    }
    inline Find read_find(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<Find> answer;
        for(const auto& packet:packets) if(auto aggregate=packet.as_SessionPacketFindAggregate()) {
            if(answer) throw AnswerCardinalityFailure("multiple matching aggregates",packets);
            answer.emplace(*aggregate->value().value);
        }
        if(!answer) throw AnswerCardinalityFailure("no matching aggregate",packets);
        return std::move(*answer);
    }
    inline std::vector<Annotation> read_annotate(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::vector<Annotation> answer;
        for(const auto& packet:packets) if(auto row=packet.as_SessionPacketAnnotateRow()) answer.push_back(*row->value().value);
        return answer;
    }
    inline std::vector<Recognition> read_recognize(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<std::vector<Recognition>> answer;
        for(const auto& packet:packets) if(auto aggregate=packet.as_SessionPacketRecognizeAggregate()) {
            if(answer) throw AnswerCardinalityFailure("multiple matching aggregates",packets);
            answer.emplace(*aggregate->value().value);
        }
        if(!answer) throw AnswerCardinalityFailure("no matching aggregate",packets);
        return std::move(*answer);
    }
    inline Relation read_relate(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<Relation> answer;
        for(const auto& packet:packets) if(auto aggregate=packet.as_SessionPacketRelateAggregate()) {
            if(answer) throw AnswerCardinalityFailure("multiple matching aggregates",packets);
            answer.emplace(*aggregate->value().value);
        }
        if(!answer) throw AnswerCardinalityFailure("no matching aggregate",packets);
        return std::move(*answer);
    }
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
    enum class NativeFailureKind {
        backend = THINKTHEN_EBACKEND,
        cancelled = THINKTHEN_ECANCELLED,
        deadline = THINKTHEN_EDEADLINE,
        defect = THINKTHEN_EDEFECT,
        local = THINKTHEN_ELOCAL,
        usage = THINKTHEN_EUSAGE,
    };
}
