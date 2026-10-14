// Generated from the compiler-derived C header; do not edit.
#pragma once
#include <cstdint>
#include <string>
#ifndef THINKTHEN_CPP_ENUMS
#define THINKTHEN_CPP_ENUMS
namespace tt {
    enum class UsagePersistenceState : uint32_t {
        disabled = 1,
        failed = 4,
        pending = 2,
        written = 3,
    };
    enum class NativeFailureKind {
        backend = 2,
        cancelled = 5,
        deadline = 3,
        defect = 6,
        local = 4,
        usage = 1,
    };
    inline NativeFailureKind failure_kind(const std::string& name) {
        if(name=="backend") return NativeFailureKind::backend;
        if(name=="cancelled") return NativeFailureKind::cancelled;
        if(name=="deadline") return NativeFailureKind::deadline;
        if(name=="defect") return NativeFailureKind::defect;
        if(name=="local") return NativeFailureKind::local;
        if(name=="usage") return NativeFailureKind::usage;
        return NativeFailureKind::defect;
    }
}
#endif
// Generated from the shared Rust result graph; do not edit.
#pragma once
#include "result_value.hpp"
#include <memory>
#include <atomic>
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
    class ImmediateFacts;
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
    class FindQuestionDocument;
    class NameOdds;
    class PairOdds;
    class PieceOdds;
    class Place;
    class Plan;
    class ProfileWarning;
    class QuestionDocument;
    class QuestionDocumentChoose;
    class QuestionDocumentDecide;
    class QuestionDocumentScore;
    class QuestionDocumentTag;
    class Recognize;
    class RecognizeAnswer;
    class RecognizeQuestionDocument;
    class RecognizeEntities;
    class RecognizeModeProposals;
    class RelateFields;
    class RelateQuestionDocument;
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
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, AnnotationMember>>>>> cached_answers_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        AnnotationMember>>>>>();
        public: const Presence<std::vector<std::pair<std::string, AnnotationMember>>>& answers() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_file_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& file() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_first_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& first_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Position>>> cached_position_ = std::make_shared<std::shared_ptr<const Presence<Position>>>();
        public: const Presence<Position>& position() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnnotatedRow>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<AnnotatedRow>>>();
        public: const Presence<AnnotatedRow>& value() const;
    };
    class AnnotationMember : public Node {
        public: using Node::Node;
        std::optional<AnnotationMemberAnswerId> as_AnnotationMemberAnswerId() const;
        std::optional<AnnotationMemberFailureId> as_AnnotationMemberFailureId() const;
    };
    class AnnotationMemberAnswerId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answer>>>();
        public: const Presence<Answer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Observation>>>> cached_observations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Observation>>>>();
        public: const Presence<std::vector<Observation>>& observations() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<QuestionSource>>>> cached_question_sources_ = std::make_shared<std::shared_ptr<const Presence<std::vector<QuestionSource>>>>();
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_request_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& request() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Usage>>> cached_usage_ = std::make_shared<std::shared_ptr<const Presence<Usage>>>();
        public: const Presence<Usage>& usage() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Value>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<Value>>>();
        public: const Presence<Value>& value() const;
    };
    class AnnotationMemberFailureId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Failure>>> cached_failure_ = std::make_shared<std::shared_ptr<const Presence<Failure>>>();
        public: const Presence<Failure>& failure() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<FailureId>>> cached_failure_id_ = std::make_shared<std::shared_ptr<const Presence<FailureId>>>();
        public: const Presence<FailureId>& failure_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Observation>>>> cached_observations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Observation>>>>();
        public: const Presence<std::vector<Observation>>& observations() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<QuestionSource>>>> cached_question_sources_ = std::make_shared<std::shared_ptr<const Presence<std::vector<QuestionSource>>>>();
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_request_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& request() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Usage>>> cached_usage_ = std::make_shared<std::shared_ptr<const Presence<Usage>>>();
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
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& value() const;
    };
    class AnnotationValueDecision : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& value() const;
    };
    class AnnotationValueFailed : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Failure>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<Failure>>>();
        public: const Presence<Failure>& value() const;
    };
    class AnnotationValueScore : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& value() const;
    };
    class AnnotationValueTags : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& value() const;
    };
    class AnswerId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class Answers : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RelationMember>>>> cached_questions_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RelationMember>>>>();
        public: const Presence<std::vector<RelationMember>>& questions() const;
    };
    class AtomicArrayOfString : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answer>>>();
        public: const Presence<Answer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Image>>>> cached_images_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Image>>>>();
        public: const Presence<std::vector<Image>>& images() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RankMember>>>> cached_members_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RankMember>>>>();
        public: const Presence<std::vector<RankMember>>& members() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_question_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& question_name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& value() const;
    };
    class AtomicDecideValue : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answer>>>();
        public: const Presence<Answer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Image>>>> cached_images_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Image>>>>();
        public: const Presence<std::vector<Image>>& images() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RankMember>>>> cached_members_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RankMember>>>>();
        public: const Presence<std::vector<RankMember>>& members() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_question_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& question_name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<DecideValue>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<DecideValue>>>();
        public: const Presence<DecideValue>& value() const;
    };
    class AtomicNonZeroUsize : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answer>>>();
        public: const Presence<Answer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Image>>>> cached_images_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Image>>>>();
        public: const Presence<std::vector<Image>>& images() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RankMember>>>> cached_members_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RankMember>>>>();
        public: const Presence<std::vector<RankMember>>& members() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_question_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& question_name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& value() const;
    };
    class AtomicNullableString : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answer>>>();
        public: const Presence<Answer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Image>>>> cached_images_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Image>>>>();
        public: const Presence<std::vector<Image>>& images() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RankMember>>>> cached_members_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RankMember>>>>();
        public: const Presence<std::vector<RankMember>>& members() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_question_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& question_name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& value() const;
    };
    class AtomicBoolean : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answer>>>();
        public: const Presence<Answer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Image>>>> cached_images_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Image>>>>();
        public: const Presence<std::vector<Image>>& images() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RankMember>>>> cached_members_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RankMember>>>>();
        public: const Presence<std::vector<RankMember>>& members() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_question_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& question_name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& value() const;
    };
    class AtomicDouble : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answer>>>();
        public: const Presence<Answer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Image>>>> cached_images_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Image>>>>();
        public: const Presence<std::vector<Image>>& images() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RankMember>>>> cached_members_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RankMember>>>>();
        public: const Presence<std::vector<RankMember>>& members() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_question_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& question_name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& value() const;
    };
    class Attempt : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_ordinal_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& ordinal() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AttemptOutcome>>> cached_outcome_ = std::make_shared<std::shared_ptr<const Presence<AttemptOutcome>>>();
        public: const Presence<AttemptOutcome>& outcome() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_request_id_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& request_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_request_sha256_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& request_sha256() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<SdkRequestId>>> cached_sdk_request_id_ = std::make_shared<std::shared_ptr<const Presence<SdkRequestId>>>();
        public: const Presence<SdkRequestId>& sdk_request_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_server_ms_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& server_ms() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_status_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& status() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_wall_ms_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& wall_ms() const;
    };
    class Batch : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::variant<uint64_t, std::string>>> cached_value_ = std::make_shared<std::shared_ptr<const std::variant<uint64_t,
        std::string>>>();
        public: const std::variant<uint64_t, std::string>& value() const;
    };
    class BoundaryMode : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class BoundaryOdds : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<PieceOdds>>>> cached_pieces_ = std::make_shared<std::shared_ptr<const Presence<std::vector<PieceOdds>>>>();
        public: const Presence<std::vector<PieceOdds>>& pieces() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<BoundaryProposal>>>> cached_proposals_ = std::make_shared<std::shared_ptr<const Presence<std::vector<BoundaryProposal>>>>();
        public: const Presence<std::vector<BoundaryProposal>>& proposals() const;
    };
    class BoundaryProposal : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_end_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& end() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_length_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& length() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_start_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& start() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_text_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& text() const;
    };
    class CallError : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Error>>> cached_error_ = std::make_shared<std::shared_ptr<const Presence<Error>>>();
        public: const Presence<Error>& error() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Facts>>> cached_facts_ = std::make_shared<std::shared_ptr<const Presence<Facts>>>();
        public: const Presence<Facts>& facts() const;
    };
    class CallId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class DecideValue : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Data>> cached_value_ = std::make_shared<std::shared_ptr<const Data>>();
        public: const Data& value() const;
    };
    class EntityDocument : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
    };
    class Error : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<EstimatedInputDenial>>> cached_estimated_input_denial_ = std::make_shared<std::shared_ptr<const Presence<EstimatedInputDenial>>>();
        public: const Presence<EstimatedInputDenial>& estimated_input_denial() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<FailureKind>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<FailureKind>>>();
        public: const Presence<FailureKind>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_message_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& message() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_retryable_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& retryable() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<SendBudgetDenial>>> cached_send_budget_denial_ = std::make_shared<std::shared_ptr<const Presence<SendBudgetDenial>>>();
        public: const Presence<SendBudgetDenial>& send_budget_denial() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Stopped>>> cached_stopped_ = std::make_shared<std::shared_ptr<const Presence<Stopped>>>();
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
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_limit_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& limit() const;
    };
    class EstimatedInputDenialInitialRequest : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_limit_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& limit() const;
    };
    class EstimatedInputDenialRetry : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_status_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_status() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_limit_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& limit() const;
    };
    class Facts : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Attempt>>>> cached_attempts_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Attempt>>>>();
        public: const Presence<std::vector<Attempt>>& attempts() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_cache_answers_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& cache_answers() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<CallId>>> cached_call_id_ = std::make_shared<std::shared_ptr<const Presence<CallId>>>();
        public: const Presence<CallId>& call_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_estimated_cost_usd_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& estimated_cost_usd() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_held_model_mismatch_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& held_model_mismatch() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_input_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& input_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_largest_request_bytes_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& largest_request_bytes() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_largest_request_estimated_input_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& largest_request_estimated_input_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_output_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& output_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_records_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& records() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_requests_sent_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& requests_sent() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_seconds_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& seconds() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_token_estimate_method_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& token_estimate_method() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PersistenceObservation>>> cached_usage_persistence_ = std::make_shared<std::shared_ptr<const Presence<PersistenceObservation>>>();
        public: const Presence<PersistenceObservation>& usage_persistence() const;
    };
    class FailureId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class Find : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<FindAnswer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<FindAnswer>>>();
        public: const Presence<FindAnswer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<FindCandidate>>>> cached_candidates_ = std::make_shared<std::shared_ptr<const Presence<std::vector<FindCandidate>>>>();
        public: const Presence<std::vector<FindCandidate>>& candidates() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_file_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& file() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_first_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& first_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Position>>> cached_position_ = std::make_shared<std::shared_ptr<const Presence<Position>>>();
        public: const Presence<Position>& position() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<FindQuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<FindQuestionDocument>>>();
        public: const Presence<FindQuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::nullptr_t>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<std::nullptr_t>>>();
        public: const Presence<std::nullptr_t>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& value() const;
    };
    class FindCandidate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
    };
    class Image : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_base64_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& base64() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint32_t>>> cached_height_ = std::make_shared<std::shared_ptr<const Presence<uint32_t>>>();
        public: const Presence<uint32_t>& height() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<ImageMedia>>> cached_media_ = std::make_shared<std::shared_ptr<const Presence<ImageMedia>>>();
        public: const Presence<ImageMedia>& media() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint32_t>>> cached_width_ = std::make_shared<std::shared_ptr<const Presence<uint32_t>>>();
        public: const Presence<uint32_t>& width() const;
    };
    class ImageMedia : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class ImmediateFacts : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Attempt>>>> cached_attempts_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Attempt>>>>();
        public: const Presence<std::vector<Attempt>>& attempts() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_cache_answers_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& cache_answers() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<CallId>>> cached_call_id_ = std::make_shared<std::shared_ptr<const Presence<CallId>>>();
        public: const Presence<CallId>& call_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_estimated_cost_usd_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& estimated_cost_usd() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_held_model_mismatch_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& held_model_mismatch() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_input_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& input_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_largest_request_bytes_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& largest_request_bytes() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_largest_request_estimated_input_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& largest_request_estimated_input_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_output_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& output_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_records_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& records() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_requests_sent_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& requests_sent() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_seconds_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& seconds() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_token_estimate_method_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& token_estimate_method() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PersistenceObservation>>> cached_usage_persistence_ = std::make_shared<std::shared_ptr<const Presence<PersistenceObservation>>>();
        public: const Presence<PersistenceObservation>& usage_persistence() const;
    };
    class InputDeclaration : public Node {
        public: using Node::Node;
        std::optional<InputDeclarationStringRoot> as_InputDeclarationStringRoot() const;
        std::optional<InputDeclarationObjectRoot> as_InputDeclarationObjectRoot() const;
    };
    class InputDeclarationObjectRoot : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, InputPropertyType>>>>> cached_properties_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        InputPropertyType>>>>>();
        public: const Presence<std::vector<std::pair<std::string, InputPropertyType>>>& properties() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_required_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& required() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<ObjectType>>> cached_type_ = std::make_shared<std::shared_ptr<const Presence<ObjectType>>>();
        public: const Presence<ObjectType>& type() const;
    };
    class InputDeclarationStringRoot : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<StringType>>> cached_type_ = std::make_shared<std::shared_ptr<const Presence<StringType>>>();
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
        private: std::shared_ptr<std::shared_ptr<const Presence<StringRoot>>> cached_items_ = std::make_shared<std::shared_ptr<const Presence<StringRoot>>>();
        public: const Presence<StringRoot>& items() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_type_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& type() const;
    };
    class InputPropertyTypeBoolean : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_type_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& type() const;
    };
    class InputPropertyTypeNumber : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_type_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& type() const;
    };
    class InputPropertyTypeString : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_type_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& type() const;
    };
    class Label : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_description_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& description() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
    };
    class Meta : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_answered_by_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& answered_by() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Attempt>>>> cached_attempts_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Attempt>>>>();
        public: const Presence<std::vector<Attempt>>& attempts() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<BatchSetting>>> cached_batch_setting_ = std::make_shared<std::shared_ptr<const Presence<BatchSetting>>>();
        public: const Presence<BatchSetting>& batch_setting() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<BatchWarning>>> cached_batch_warning_ = std::make_shared<std::shared_ptr<const Presence<BatchWarning>>>();
        public: const Presence<BatchWarning>& batch_warning() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_cached_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& cached() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_context_sha256_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& context_sha256() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_failed_questions_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& failed_questions() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Observation>>>> cached_observations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Observation>>>>();
        public: const Presence<std::vector<Observation>>& observations() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Origin>>> cached_origin_ = std::make_shared<std::shared_ptr<const Presence<Origin>>>();
        public: const Presence<Origin>& origin() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<ProfileWarning>>> cached_profile_warning_ = std::make_shared<std::shared_ptr<const Presence<ProfileWarning>>>();
        public: const Presence<ProfileWarning>& profile_warning() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_question_sha256_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& question_sha256() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<QuestionSource>>>> cached_question_sources_ = std::make_shared<std::shared_ptr<const Presence<std::vector<QuestionSource>>>>();
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_questions_sha256_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& questions_sha256() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_requests_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& requests() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_requests_sent_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& requests_sent() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_tool_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& tool() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_url_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& url() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Usage>>> cached_usage_ = std::make_shared<std::shared_ptr<const Presence<Usage>>>();
        public: const Presence<Usage>& usage() const;
    };
    class ObjectRoot : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, InputPropertyType>>>>> cached_properties_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        InputPropertyType>>>>>();
        public: const Presence<std::vector<std::pair<std::string, InputPropertyType>>>& properties() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_required_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& required() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<ObjectType>>> cached_type_ = std::make_shared<std::shared_ptr<const Presence<ObjectType>>>();
        public: const Presence<ObjectType>& type() const;
    };
    class ObjectType : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class Observation : public Node {
        public: using Node::Node;
        std::optional<ObservationObservationId> as_ObservationObservationId() const;
        std::optional<ObservationFailureId> as_ObservationFailureId() const;
    };
    class ObservationId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class ObservationFailureId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<FailureId>>> cached_failure_id_ = std::make_shared<std::shared_ptr<const Presence<FailureId>>>();
        public: const Presence<FailureId>& failure_id() const;
    };
    class ObservationObservationId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<ObservationId>>> cached_observation_id_ = std::make_shared<std::shared_ptr<const Presence<ObservationId>>>();
        public: const Presence<ObservationId>& observation_id() const;
    };
    class Origin : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class PersistenceObservation : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_advice_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& advice() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_observed_at_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& observed_at() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<UsagePersistence>>> cached_state_ = std::make_shared<std::shared_ptr<const Presence<UsagePersistence>>>();
        public: const Presence<UsagePersistence>& state() const;
    };
    class PhysicalSource : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_file_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& file() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_first_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& first_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_line() const;
    };
    class Position : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_file_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& file() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_first_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& first() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_images_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& images() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last() const;
    };
    class QuestionName : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class QuestionSource : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_answered_by_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& answered_by() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint32_t>>> cached_batch_size_ = std::make_shared<std::shared_ptr<const Presence<uint32_t>>>();
        public: const Presence<uint32_t>& batch_size() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Origin>>> cached_origin_ = std::make_shared<std::shared_ptr<const Presence<Origin>>>();
        public: const Presence<Origin>& origin() const;
    };
    class RankMember : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RankMemberResult>>> cached_result_ = std::make_shared<std::shared_ptr<const Presence<RankMemberResult>>>();
        public: const Presence<RankMemberResult>& result() const;
    };
    class RankMemberResult : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answer>>>();
        public: const Presence<Answer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Image>>>> cached_images_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Image>>>>();
        public: const Presence<std::vector<Image>>& images() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::nullptr_t>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<std::nullptr_t>>>();
        public: const Presence<std::nullptr_t>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& value() const;
    };
    class Recognition : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<RecognitionOdds>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<RecognitionOdds>>>();
        public: const Presence<RecognitionOdds>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_file_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& file() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_first_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& first_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Position>>> cached_position_ = std::make_shared<std::shared_ptr<const Presence<Position>>>();
        public: const Presence<Position>& position() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RecognizeQuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<RecognizeQuestionDocument>>>();
        public: const Presence<RecognizeQuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Recognize>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<Recognize>>>();
        public: const Presence<Recognize>& value() const;
    };
    class RecognitionEdgeDocument : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_either_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& either() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_relation_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& relation() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Entity>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<Entity>>>();
        public: const Presence<Entity>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Entity>>> cached_target_ = std::make_shared<std::shared_ptr<const Presence<Entity>>>();
        public: const Presence<Entity>& target() const;
    };
    class RecognitionMode : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class RecognitionOdds : public Node {
        public: using Node::Node;
        std::optional<RecognitionOddsRecognizeAnswer> as_RecognitionOddsRecognizeAnswer() const;
        std::optional<RecognitionOddsBoundaryOdds> as_RecognitionOddsBoundaryOdds() const;
    };
    class RecognitionOddsBoundaryOdds : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<PieceOdds>>>> cached_pieces_ = std::make_shared<std::shared_ptr<const Presence<std::vector<PieceOdds>>>>();
        public: const Presence<std::vector<PieceOdds>>& pieces() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<BoundaryProposal>>>> cached_proposals_ = std::make_shared<std::shared_ptr<const Presence<std::vector<BoundaryProposal>>>>();
        public: const Presence<std::vector<BoundaryProposal>>& proposals() const;
    };
    class RecognitionOddsRecognizeAnswer : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<NameOdds>>>> cached_names_ = std::make_shared<std::shared_ptr<const Presence<std::vector<NameOdds>>>>();
        public: const Presence<std::vector<NameOdds>>& names() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<PairOdds>>>> cached_pairs_ = std::make_shared<std::shared_ptr<const Presence<std::vector<PairOdds>>>>();
        public: const Presence<std::vector<PairOdds>>& pairs() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<PieceOdds>>>> cached_pieces_ = std::make_shared<std::shared_ptr<const Presence<std::vector<PieceOdds>>>>();
        public: const Presence<std::vector<PieceOdds>>& pieces() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RecognitionProposal>>>> cached_proposals_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RecognitionProposal>>>>();
        public: const Presence<std::vector<RecognitionProposal>>& proposals() const;
    };
    class RecognitionProposal : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_end_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& end() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_kept_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& kept() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Place>>> cached_selected_ = std::make_shared<std::shared_ptr<const Presence<Place>>>();
        public: const Presence<Place>& selected() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_span_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& span_probability() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_start_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& start() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_strength_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& strength() const;
    };
    class RecognitionStageContext : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_boundary_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& boundary() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_edge_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind_edge() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_relation_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& relation() const;
    };
    class Relation : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answers>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answers>>>();
        public: const Presence<Answers>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_file_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& file() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_first_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& first_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<SessionInputSource>>>> cached_input_sources_ = std::make_shared<std::shared_ptr<const Presence<std::vector<SessionInputSource>>>>();
        public: const Presence<std::vector<SessionInputSource>>& input_sources() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Meta>>> cached_meta_ = std::make_shared<std::shared_ptr<const Presence<Meta>>>();
        public: const Presence<Meta>& meta() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Position>>> cached_position_ = std::make_shared<std::shared_ptr<const Presence<Position>>>();
        public: const Presence<Position>& position() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelateQuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<RelateQuestionDocument>>>();
        public: const Presence<RelateQuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Version>>> cached_schema_ = std::make_shared<std::shared_ptr<const Presence<Version>>>();
        public: const Presence<Version>& schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RelatedEntityEdge>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RelatedEntityEdge>>>>();
        public: const Presence<std::vector<RelatedEntityEdge>>& value() const;
    };
    class RelationDirection : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class RelationEndpoint : public Node {
        public: using Node::Node;
        std::optional<RelationEndpointRelatedEntity> as_RelationEndpointRelatedEntity() const;
        std::optional<RelationEndpointSource> as_RelationEndpointSource() const;
    };
    class RelationEndpointRelatedEntity : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
    };
    class RelationEndpointSource : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_file_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& file() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_first_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& first_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_ordinal_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& ordinal() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_record_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& record() const;
    };
    class RelationMember : public Node {
        public: using Node::Node;
        std::optional<RelationMemberAnswerId> as_RelationMemberAnswerId() const;
        std::optional<RelationMemberFailureId> as_RelationMemberFailureId() const;
    };
    class RelationMemberAnswerId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelationDirection>>> cached_direction_ = std::make_shared<std::shared_ptr<const Presence<RelationDirection>>>();
        public: const Presence<RelationDirection>& direction() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelationMethod>>> cached_method_ = std::make_shared<std::shared_ptr<const Presence<RelationMethod>>>();
        public: const Presence<RelationMethod>& method() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Observation>>>> cached_observations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Observation>>>>();
        public: const Presence<std::vector<Observation>>& observations() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<QuestionSource>>>> cached_question_sources_ = std::make_shared<std::shared_ptr<const Presence<std::vector<QuestionSource>>>>();
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_reads_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& reads() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_relation_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& relation() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_request_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& request() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelatedEntity>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<RelatedEntity>>>();
        public: const Presence<RelatedEntity>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelatedEntity>>> cached_target_ = std::make_shared<std::shared_ptr<const Presence<RelatedEntity>>>();
        public: const Presence<RelatedEntity>& target() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Usage>>> cached_usage_ = std::make_shared<std::shared_ptr<const Presence<Usage>>>();
        public: const Presence<Usage>& usage() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_accepted_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& accepted() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Answer>>> cached_answer_ = std::make_shared<std::shared_ptr<const Presence<Answer>>>();
        public: const Presence<Answer>& answer() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
        std::string source_name() const;
    };
    class RelationMemberFailureId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelationDirection>>> cached_direction_ = std::make_shared<std::shared_ptr<const Presence<RelationDirection>>>();
        public: const Presence<RelationDirection>& direction() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelationMethod>>> cached_method_ = std::make_shared<std::shared_ptr<const Presence<RelationMethod>>>();
        public: const Presence<RelationMethod>& method() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Observation>>>> cached_observations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Observation>>>>();
        public: const Presence<std::vector<Observation>>& observations() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<QuestionSource>>>> cached_question_sources_ = std::make_shared<std::shared_ptr<const Presence<std::vector<QuestionSource>>>>();
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_reads_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& reads() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_relation_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& relation() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_request_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& request() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelatedEntity>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<RelatedEntity>>>();
        public: const Presence<RelatedEntity>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelatedEntity>>> cached_target_ = std::make_shared<std::shared_ptr<const Presence<RelatedEntity>>>();
        public: const Presence<RelatedEntity>& target() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Usage>>> cached_usage_ = std::make_shared<std::shared_ptr<const Presence<Usage>>>();
        public: const Presence<Usage>& usage() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Failure>>> cached_failure_ = std::make_shared<std::shared_ptr<const Presence<Failure>>>();
        public: const Presence<Failure>& failure() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<FailureId>>> cached_failure_id_ = std::make_shared<std::shared_ptr<const Presence<FailureId>>>();
        public: const Presence<FailureId>& failure_id() const;
        std::string source_name() const;
    };
    class RelationMethod : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class RequestFunction : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class SdkRequestId : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
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
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
    };
    class SendBudgetDenialBeforeFirstSend : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
    };
    class SendBudgetDenialBeforeRetry : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_status_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_status() const;
    };
    class StopCause : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class Stopped : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_at_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& at() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<StopCause>>> cached_cause_ = std::make_shared<std::shared_ptr<const Presence<StopCause>>>();
        public: const Presence<StopCause>& cause() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_retryable_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& retryable() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_status_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& status() const;
    };
    class StringRoot : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<StringType>>> cached_type_ = std::make_shared<std::shared_ptr<const Presence<StringType>>>();
        public: const Presence<StringType>& type() const;
    };
    class StringType : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class Usage : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_input_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& input_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_output_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& output_tokens() const;
    };
    class UsagePersistence : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class Verb : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class Version : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class WordingVersion : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const uint32_t>> cached_value_ = std::make_shared<std::shared_ptr<const uint32_t>>();
        public: const uint32_t& value() const;
    };
    class AnnotatedField : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::variant<bool, std::nullptr_t, std::string,
        std::vector<std::string>, double, Failed>>> cached_value_ = std::make_shared<std::shared_ptr<const std::variant<bool,
        std::nullptr_t, std::string, std::vector<std::string>, double, Failed>>>();
        public: const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double,
        Failed>& value() const;
    };
    class AnnotatedRow : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::vector<std::pair<std::string, AnnotatedField>>>> cached_value_ = std::make_shared<std::shared_ptr<const std::vector<std::pair<std::string,
        AnnotatedField>>>>();
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
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_confidence_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& confidence() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_pick_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& pick() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>>> cached_probabilities_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        double>>>>>();
        public: const Presence<std::vector<std::pair<std::string, double>>>& probabilities() const;
    };
    class AnswerScore : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_confidence_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& confidence() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_level_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& level() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>>> cached_probabilities_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        double>>>>>();
        public: const Presence<std::vector<std::pair<std::string, double>>>& probabilities() const;
    };
    class AnswerTag : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>>> cached_probabilities_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        double>>>>>();
        public: const Presence<std::vector<std::pair<std::string, double>>>& probabilities() const;
    };
    class AnswerYesNo : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
    };
    class AttemptOutcome : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class BatchSetting : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::variant<uint64_t, std::string>>> cached_value_ = std::make_shared<std::shared_ptr<const std::variant<uint64_t,
        std::string>>>();
        public: const std::variant<uint64_t, std::string>& value() const;
    };
    class BatchWarning : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<BatchSetting>>> cached_running_ = std::make_shared<std::shared_ptr<const Presence<BatchSetting>>>();
        public: const Presence<BatchSetting>& running() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<BatchSetting>>> cached_tuned_for_ = std::make_shared<std::shared_ptr<const Presence<BatchSetting>>>();
        public: const Presence<BatchSetting>& tuned_for() const;
    };
    class Entity : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_end_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& end() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_file_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& file() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_first_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& first_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_length_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& length() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_start_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& start() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_strength_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& strength() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_text_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& text() const;
    };
    class EntityEdge : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_either_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& either() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_relation_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& relation() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Entity>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<Entity>>>();
        public: const Presence<Entity>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Entity>>> cached_target_ = std::make_shared<std::shared_ptr<const Presence<Entity>>>();
        public: const Presence<Entity>& target() const;
    };
    class Failed : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Failure>>> cached_failed_ = std::make_shared<std::shared_ptr<const Presence<Failure>>>();
        public: const Presence<Failure>& failed() const;
    };
    class Failure : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<FailureCause>>> cached_cause_ = std::make_shared<std::shared_ptr<const Presence<FailureCause>>>();
        public: const Presence<FailureCause>& cause() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
    };
    class FailureCause : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class FailureKind : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::string>> cached_value_ = std::make_shared<std::shared_ptr<const std::string>>();
        public: const std::string& value() const;
    };
    class FindAnswer : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_confidence_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& confidence() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_pick_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& pick() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>>> cached_probabilities_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        double>>>>>();
        public: const Presence<std::vector<std::pair<std::string, double>>>& probabilities() const;
    };
    class FindQuestionDocument : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Batch>>> cached_batch_ = std::make_shared<std::shared_ptr<const Presence<Batch>>>();
        public: const Presence<Batch>& batch() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_context_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& context_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_item_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& item_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Label>>>> cached_label_details_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Label>>>>();
        public: const Presence<std::vector<Label>>& label_details() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionName>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<QuestionName>>>();
        public: const Presence<QuestionName>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_none_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& none() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_on_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& on() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_profile_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& profile() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_text_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& text() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_verb_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& verb() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<WordingVersion>>> cached_wording_version_ = std::make_shared<std::shared_ptr<const Presence<WordingVersion>>>();
        public: const Presence<WordingVersion>& wording_version() const;
    };
    class NameOdds : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>>> cached_edges_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        double>>>>>();
        public: const Presence<std::vector<std::pair<std::string, double>>>& edges() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_end_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& end() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>>> cached_kinds_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        double>>>>>();
        public: const Presence<std::vector<std::pair<std::string, double>>>& kinds() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_start_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& start() const;
    };
    class PairOdds : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_relation_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& relation() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Place>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<Place>>>();
        public: const Presence<Place>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Place>>> cached_target_ = std::make_shared<std::shared_ptr<const Presence<Place>>>();
        public: const Presence<Place>& target() const;
    };
    class PieceOdds : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_end_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& end() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_start_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& start() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, double>>>>> cached_tags_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        double>>>>>();
        public: const Presence<std::vector<std::pair<std::string, double>>>& tags() const;
    };
    class Place : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_end_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& end() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_start_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& start() const;
    };
    class Plan : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_estimated_bytes_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& estimated_bytes() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<TokenBand>>> cached_estimated_input_tokens_ = std::make_shared<std::shared_ptr<const Presence<TokenBand>>>();
        public: const Presence<TokenBand>& estimated_input_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_first_body_utf8_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& first_body_utf8() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_largest_request_bytes_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& largest_request_bytes() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_largest_request_estimated_input_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& largest_request_estimated_input_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_records_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& records() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_requests_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& requests() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_token_estimate_method_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& token_estimate_method() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_upper_bound_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& upper_bound() const;
    };
    class ProfileWarning : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_running_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& running() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_tuned_for_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& tuned_for() const;
    };
    class QuestionDocument : public Node {
        public: using Node::Node;
        std::optional<QuestionDocumentDecide> as_QuestionDocumentDecide() const;
        std::optional<QuestionDocumentChoose> as_QuestionDocumentChoose() const;
        std::optional<QuestionDocumentTag> as_QuestionDocumentTag() const;
        std::optional<QuestionDocumentScore> as_QuestionDocumentScore() const;
    };
    class QuestionDocumentChoose : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Batch>>> cached_batch_ = std::make_shared<std::shared_ptr<const Presence<Batch>>>();
        public: const Presence<Batch>& batch() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_context_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& context_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_item_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& item_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Label>>>> cached_label_details_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Label>>>>();
        public: const Presence<std::vector<Label>>& label_details() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionName>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<QuestionName>>>();
        public: const Presence<QuestionName>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_on_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& on() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_profile_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& profile() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<WordingVersion>>> cached_wording_version_ = std::make_shared<std::shared_ptr<const Presence<WordingVersion>>>();
        public: const Presence<WordingVersion>& wording_version() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_options_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& options() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_text_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& text() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_verb_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& verb() const;
    };
    class QuestionDocumentDecide : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Batch>>> cached_batch_ = std::make_shared<std::shared_ptr<const Presence<Batch>>>();
        public: const Presence<Batch>& batch() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_context_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& context_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_item_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& item_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Label>>>> cached_label_details_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Label>>>>();
        public: const Presence<std::vector<Label>>& label_details() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionName>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<QuestionName>>>();
        public: const Presence<QuestionName>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_on_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& on() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_profile_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& profile() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<WordingVersion>>> cached_wording_version_ = std::make_shared<std::shared_ptr<const Presence<WordingVersion>>>();
        public: const Presence<WordingVersion>& wording_version() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_false__ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& false_() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_text_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& text() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_true__ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& true_() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_verb_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& verb() const;
    };
    class QuestionDocumentScore : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Batch>>> cached_batch_ = std::make_shared<std::shared_ptr<const Presence<Batch>>>();
        public: const Presence<Batch>& batch() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_context_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& context_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_item_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& item_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Label>>>> cached_label_details_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Label>>>>();
        public: const Presence<std::vector<Label>>& label_details() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionName>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<QuestionName>>>();
        public: const Presence<QuestionName>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_on_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& on() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_profile_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& profile() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<WordingVersion>>> cached_wording_version_ = std::make_shared<std::shared_ptr<const Presence<WordingVersion>>>();
        public: const Presence<WordingVersion>& wording_version() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_levels_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& levels() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_text_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& text() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_verb_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& verb() const;
    };
    class QuestionDocumentTag : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Batch>>> cached_batch_ = std::make_shared<std::shared_ptr<const Presence<Batch>>>();
        public: const Presence<Batch>& batch() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_context_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& context_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_item_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& item_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Label>>>> cached_label_details_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Label>>>>();
        public: const Presence<std::vector<Label>>& label_details() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionName>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<QuestionName>>>();
        public: const Presence<QuestionName>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_on_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& on() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_profile_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& profile() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<WordingVersion>>> cached_wording_version_ = std::make_shared<std::shared_ptr<const Presence<WordingVersion>>>();
        public: const Presence<WordingVersion>& wording_version() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_labels_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& labels() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_text_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& text() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_verb_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& verb() const;
    };
    class Recognize : public Node {
        public: using Node::Node;
        std::optional<RecognizeEntities> as_RecognizeEntities() const;
        std::optional<RecognizeModeProposals> as_RecognizeModeProposals() const;
    };
    class RecognizeAnswer : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<NameOdds>>>> cached_names_ = std::make_shared<std::shared_ptr<const Presence<std::vector<NameOdds>>>>();
        public: const Presence<std::vector<NameOdds>>& names() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<PairOdds>>>> cached_pairs_ = std::make_shared<std::shared_ptr<const Presence<std::vector<PairOdds>>>>();
        public: const Presence<std::vector<PairOdds>>& pairs() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<PieceOdds>>>> cached_pieces_ = std::make_shared<std::shared_ptr<const Presence<std::vector<PieceOdds>>>>();
        public: const Presence<std::vector<PieceOdds>>& pieces() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RecognitionProposal>>>> cached_proposals_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RecognitionProposal>>>>();
        public: const Presence<std::vector<RecognitionProposal>>& proposals() const;
    };
    class RecognizeQuestionDocument : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Batch>>> cached_batch_ = std::make_shared<std::shared_ptr<const Presence<Batch>>>();
        public: const Presence<Batch>& batch() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_context_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& context_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_entity_definition_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& entity_definition() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_instructions_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& instructions() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_item_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& item_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::pair<std::string, Data>>>>> cached_kinds_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::pair<std::string,
        Data>>>>>();
        public: const Presence<std::vector<std::pair<std::string, Data>>>& kinds() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Label>>>> cached_label_details_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Label>>>>();
        public: const Presence<std::vector<Label>>& label_details() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RecognitionMode>>> cached_mode_ = std::make_shared<std::shared_ptr<const Presence<RecognitionMode>>>();
        public: const Presence<RecognitionMode>& mode() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionName>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<QuestionName>>>();
        public: const Presence<QuestionName>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_on_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& on() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_profile_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& profile() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_relation_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& relation_threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RelationRule>>>> cached_relations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RelationRule>>>>();
        public: const Presence<std::vector<RelationRule>>& relations() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint32_t>>> cached_snippet_pieces_ = std::make_shared<std::shared_ptr<const Presence<uint32_t>>>();
        public: const Presence<uint32_t>& snippet_pieces() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RecognitionStageContext>>> cached_stage_context_ = std::make_shared<std::shared_ptr<const Presence<RecognitionStageContext>>>();
        public: const Presence<RecognitionStageContext>& stage_context() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Verb>>> cached_verb_ = std::make_shared<std::shared_ptr<const Presence<Verb>>>();
        public: const Presence<Verb>& verb() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<WordingVersion>>> cached_wording_version_ = std::make_shared<std::shared_ptr<const Presence<WordingVersion>>>();
        public: const Presence<WordingVersion>& wording_version() const;
    };
    class RecognizeEntities : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Entity>>>> cached_entities_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Entity>>>>();
        public: const Presence<std::vector<Entity>>& entities() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<EntityEdge>>>> cached_relations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<EntityEdge>>>>();
        public: const Presence<std::vector<EntityEdge>>& relations() const;
    };
    class RecognizeModeProposals : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<BoundaryMode>>> cached_mode_ = std::make_shared<std::shared_ptr<const Presence<BoundaryMode>>>();
        public: const Presence<BoundaryMode>& mode() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<BoundaryProposal>>>> cached_proposals_ = std::make_shared<std::shared_ptr<const Presence<std::vector<BoundaryProposal>>>>();
        public: const Presence<std::vector<BoundaryProposal>>& proposals() const;
    };
    class RelateFields : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
    };
    class RelateQuestionDocument : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Batch>>> cached_batch_ = std::make_shared<std::shared_ptr<const Presence<Batch>>>();
        public: const Presence<Batch>& batch() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_context_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& context_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelateFields>>> cached_fields_ = std::make_shared<std::shared_ptr<const Presence<RelateFields>>>();
        public: const Presence<RelateFields>& fields() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<InputDeclaration>>> cached_item_schema_ = std::make_shared<std::shared_ptr<const Presence<InputDeclaration>>>();
        public: const Presence<InputDeclaration>& item_schema() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Label>>>> cached_label_details_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Label>>>>();
        public: const Presence<std::vector<Label>>& label_details() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionName>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<QuestionName>>>();
        public: const Presence<QuestionName>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_on_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& on() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_profile_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& profile() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RelationRule>>>> cached_relations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RelationRule>>>>();
        public: const Presence<std::vector<RelationRule>>& relations() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_verb_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& verb() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<WordingVersion>>> cached_wording_version_ = std::make_shared<std::shared_ptr<const Presence<WordingVersion>>>();
        public: const Presence<WordingVersion>& wording_version() const;
    };
    class RelatedEntity : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
    };
    class RelatedEntityEdge : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_either_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& either() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_relation_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& relation() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelationEndpoint>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<RelationEndpoint>>>();
        public: const Presence<RelationEndpoint>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RelationEndpoint>>> cached_target_ = std::make_shared<std::shared_ptr<const Presence<RelationEndpoint>>>();
        public: const Presence<RelationEndpoint>& target() const;
        std::string source_name() const;
        std::string target_name() const;
    };
    class RelationRule : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_either_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& either() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_reads_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& reads() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_single_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& single() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_target_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& target() const;
    };
    class SessionAnnotation : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnnotationValue>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<AnnotationValue>>>();
        public: const Presence<AnnotationValue>& value() const;
    };
    class SessionInputSource : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
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
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& value() const;
    };
    class SessionJudgmentDecision : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& value() const;
    };
    class SessionJudgmentScore : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& value() const;
    };
    class SessionJudgmentTags : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& value() const;
    };
    class SessionNamedProbability : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
    };
    class SessionObservation : public Node {
        public: using Node::Node;
        std::optional<SessionObservationQuestion> as_SessionObservationQuestion() const;
        std::optional<SessionObservationRow> as_SessionObservationRow() const;
    };
    class SessionObservationQuestion : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<SessionQuestionDetail>>> cached_detail_ = std::make_shared<std::shared_ptr<const Presence<SessionQuestionDetail>>>();
        public: const Presence<SessionQuestionDetail>& detail() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_member_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& member() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_position_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& position() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_stage_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& stage() const;
    };
    class SessionObservationRow : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_index_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& index() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<SessionObservedRow>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<SessionObservedRow>>>();
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
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<SessionAnnotation>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<SessionAnnotation>>>>();
        public: const Presence<std::vector<SessionAnnotation>>& value() const;
    };
    class SessionObservedRowFind : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& value() const;
    };
    class SessionObservedRowJudgment : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<SessionJudgment>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<SessionJudgment>>>();
        public: const Presence<SessionJudgment>& value() const;
    };
    class SessionObservedRowRecognized : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<SessionRecognition>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<SessionRecognition>>>();
        public: const Presence<SessionRecognition>& value() const;
    };
    class SessionObservedRowRelations : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<SessionRelationEdge>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<SessionRelationEdge>>>>();
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
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Annotation>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Annotation>>>>();
        public: const Presence<std::vector<Annotation>>& value() const;
    };
    class SessionPacketAnnotateRow : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Annotation>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<Annotation>>>();
        public: const Presence<Annotation>& value() const;
    };
    class SessionPacketChooseAggregate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<AtomicNullableString>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<AtomicNullableString>>>>();
        public: const Presence<std::vector<AtomicNullableString>>& value() const;
    };
    class SessionPacketChooseRow : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AtomicNullableString>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<AtomicNullableString>>>();
        public: const Presence<AtomicNullableString>& value() const;
    };
    class SessionPacketDecideAggregate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<AtomicDecideValue>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<AtomicDecideValue>>>>();
        public: const Presence<std::vector<AtomicDecideValue>>& value() const;
    };
    class SessionPacketDecideRow : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AtomicDecideValue>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<AtomicDecideValue>>>();
        public: const Presence<AtomicDecideValue>& value() const;
    };
    class SessionPacketFilterAggregate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<AtomicBoolean>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<AtomicBoolean>>>>();
        public: const Presence<std::vector<AtomicBoolean>>& value() const;
    };
    class SessionPacketFilterRow : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AtomicBoolean>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<AtomicBoolean>>>();
        public: const Presence<AtomicBoolean>& value() const;
    };
    class SessionPacketFindAggregate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Find>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<Find>>>();
        public: const Presence<Find>& value() const;
    };
    class SessionPacketObservation : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<RequestFunction>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<RequestFunction>>>();
        public: const Presence<RequestFunction>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<SessionObservation>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<SessionObservation>>>();
        public: const Presence<SessionObservation>& value() const;
    };
    class SessionPacketRankAggregate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<AtomicNonZeroUsize>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<AtomicNonZeroUsize>>>>();
        public: const Presence<std::vector<AtomicNonZeroUsize>>& value() const;
    };
    class SessionPacketRecognizeAggregate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Recognition>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Recognition>>>>();
        public: const Presence<std::vector<Recognition>>& value() const;
    };
    class SessionPacketRelateAggregate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Relation>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<Relation>>>();
        public: const Presence<Relation>& value() const;
    };
    class SessionPacketScoreAggregate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<AtomicDouble>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<AtomicDouble>>>>();
        public: const Presence<std::vector<AtomicDouble>>& value() const;
    };
    class SessionPacketScoreRow : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AtomicDouble>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<AtomicDouble>>>();
        public: const Presence<AtomicDouble>& value() const;
    };
    class SessionPacketTagAggregate : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<AtomicArrayOfString>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<AtomicArrayOfString>>>>();
        public: const Presence<std::vector<AtomicArrayOfString>>& value() const;
    };
    class SessionPacketTagRow : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_function_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& function() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<AtomicArrayOfString>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<AtomicArrayOfString>>>();
        public: const Presence<AtomicArrayOfString>& value() const;
    };
    class SessionPacketTerminal : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<Facts>>> cached_facts_ = std::make_shared<std::shared_ptr<const Presence<Facts>>>();
        public: const Presence<Facts>& facts() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<CallError>>> cached_failure_ = std::make_shared<std::shared_ptr<const Presence<CallError>>>();
        public: const Presence<CallError>& failure() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
    };
    class SessionProbabilities : public Node {
        public: using Node::Node;
        std::optional<SessionProbabilitiesYesNo> as_SessionProbabilitiesYesNo() const;
        std::optional<SessionProbabilitiesNamed> as_SessionProbabilitiesNamed() const;
    };
    class SessionProbabilitiesNamed : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<SessionNamedProbability>>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<std::vector<SessionNamedProbability>>>>();
        public: const Presence<std::vector<SessionNamedProbability>>& value() const;
    };
    class SessionProbabilitiesYesNo : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& value() const;
    };
    class SessionQuestionDetail : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<AnswerId>>> cached_answer_id_ = std::make_shared<std::shared_ptr<const Presence<AnswerId>>>();
        public: const Presence<AnswerId>& answer_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_cached_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& cached() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_confidence_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& confidence() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_failed_questions_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& failed_questions() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Failure>>> cached_failure_ = std::make_shared<std::shared_ptr<const Presence<Failure>>>();
        public: const Presence<Failure>& failure() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<FailureId>>> cached_failure_id_ = std::make_shared<std::shared_ptr<const Presence<FailureId>>>();
        public: const Presence<FailureId>& failure_id() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_input_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& input() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<PhysicalSource>>> cached_input_source_ = std::make_shared<std::shared_ptr<const Presence<PhysicalSource>>>();
        public: const Presence<PhysicalSource>& input_source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<SessionInputSource>>>> cached_input_sources_ = std::make_shared<std::shared_ptr<const Presence<std::vector<SessionInputSource>>>>();
        public: const Presence<std::vector<SessionInputSource>>& input_sources() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Data>>>> cached_inputs_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Data>>>>();
        public: const Presence<std::vector<Data>>& inputs() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_model_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& model() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Observation>>>> cached_observations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Observation>>>>();
        public: const Presence<std::vector<Observation>>& observations() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<SessionProbabilities>>> cached_probabilities_ = std::make_shared<std::shared_ptr<const Presence<SessionProbabilities>>>();
        public: const Presence<SessionProbabilities>& probabilities() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<QuestionDocument>>> cached_question_ = std::make_shared<std::shared_ptr<const Presence<QuestionDocument>>>();
        public: const Presence<QuestionDocument>& question() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_question_sha256_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& question_sha256() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<QuestionSource>>>> cached_question_sources_ = std::make_shared<std::shared_ptr<const Presence<std::vector<QuestionSource>>>>();
        public: const Presence<std::vector<QuestionSource>>& question_sources() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_raw_pick_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& raw_pick() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Usage>>> cached_reported_usage_ = std::make_shared<std::shared_ptr<const Presence<Usage>>>();
        public: const Presence<Usage>& reported_usage() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<std::string>>>> cached_requests_ = std::make_shared<std::shared_ptr<const Presence<std::vector<std::string>>>>();
        public: const Presence<std::vector<std::string>>& requests() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_requests_sent_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& requests_sent() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Threshold>>> cached_threshold_ = std::make_shared<std::shared_ptr<const Presence<Threshold>>>();
        public: const Presence<Threshold>& threshold() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_url_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& url() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<TokenUsage>>> cached_usage_ = std::make_shared<std::shared_ptr<const Presence<TokenUsage>>>();
        public: const Presence<TokenUsage>& usage() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Value>>> cached_value_ = std::make_shared<std::shared_ptr<const Presence<Value>>>();
        public: const Presence<Value>& value() const;
    };
    class SessionRecognition : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<Entity>>>> cached_entities_ = std::make_shared<std::shared_ptr<const Presence<std::vector<Entity>>>>();
        public: const Presence<std::vector<Entity>>& entities() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<RecognitionMode>>> cached_mode_ = std::make_shared<std::shared_ptr<const Presence<RecognitionMode>>>();
        public: const Presence<RecognitionMode>& mode() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<BoundaryProposal>>>> cached_proposals_ = std::make_shared<std::shared_ptr<const Presence<std::vector<BoundaryProposal>>>>();
        public: const Presence<std::vector<BoundaryProposal>>& proposals() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::vector<RecognitionEdgeDocument>>>> cached_relations_ = std::make_shared<std::shared_ptr<const Presence<std::vector<RecognitionEdgeDocument>>>>();
        public: const Presence<std::vector<RecognitionEdgeDocument>>& relations() const;
    };
    class SessionRelationEdge : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<bool>>> cached_either_ = std::make_shared<std::shared_ptr<const Presence<bool>>>();
        public: const Presence<bool>& either() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<double>>> cached_probability_ = std::make_shared<std::shared_ptr<const Presence<double>>>();
        public: const Presence<double>& probability() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_relation_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& relation() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<EntityDocument>>> cached_source_ = std::make_shared<std::shared_ptr<const Presence<EntityDocument>>>();
        public: const Presence<EntityDocument>& source() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<EntityDocument>>> cached_target_ = std::make_shared<std::shared_ptr<const Presence<EntityDocument>>>();
        public: const Presence<EntityDocument>& target() const;
        std::string source_name() const;
        std::string target_name() const;
    };
    class SourceRelationEndpoint : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_file_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& file() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_first_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& first_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_kind_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& kind() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_last_line_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& last_line() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<std::string>>> cached_name_ = std::make_shared<std::shared_ptr<const Presence<std::string>>>();
        public: const Presence<std::string>& name() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_ordinal_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& ordinal() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<Data>>> cached_record_ = std::make_shared<std::shared_ptr<const Presence<Data>>>();
        public: const Presence<Data>& record() const;
    };
    class Threshold : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::variant<double, std::string>>> cached_value_ = std::make_shared<std::shared_ptr<const std::variant<double,
        std::string>>>();
        public: const std::variant<double, std::string>& value() const;
    };
    class TokenBand : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_lower_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& lower() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_upper_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& upper() const;
    };
    class TokenUsage : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_input_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& input_tokens() const;
        private: std::shared_ptr<std::shared_ptr<const Presence<uint64_t>>> cached_output_tokens_ = std::make_shared<std::shared_ptr<const Presence<uint64_t>>>();
        public: const Presence<uint64_t>& output_tokens() const;
    };
    class Value : public Node {
        public: using Node::Node;
        private: std::shared_ptr<std::shared_ptr<const std::variant<bool, std::nullptr_t, std::string,
        std::vector<std::string>, double>>> cached_value_ = std::make_shared<std::shared_ptr<const std::variant<bool,
        std::nullptr_t, std::string, std::vector<std::string>, double>>>();
        public: const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double>& value() const;
    };
    inline const Presence<AnswerId>& Annotation::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, AnnotationMember>>>& Annotation::answers() const {
        auto stored = std::atomic_load(cached_answers_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, AnnotationMember>>>>(detail::member_value<std::vector<std::pair<std::string,
            AnnotationMember>>>(value_, "answers"));
            if (std::atomic_compare_exchange_strong(cached_answers_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Annotation::file() const {
        auto stored = std::atomic_load(cached_file_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "file"));
            if (std::atomic_compare_exchange_strong(cached_file_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Annotation::first_line() const {
        auto stored = std::atomic_load(cached_first_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "first_line"));
            if (std::atomic_compare_exchange_strong(cached_first_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Annotation::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& Annotation::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Annotation::last_line() const {
        auto stored = std::atomic_load(cached_last_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_line"));
            if (std::atomic_compare_exchange_strong(cached_last_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& Annotation::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Position>& Annotation::position() const {
        auto stored = std::atomic_load(cached_position_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Position>>(detail::member_value<Position>(value_,
            "position"));
            if (std::atomic_compare_exchange_strong(cached_position_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& Annotation::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& Annotation::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnnotatedRow>& Annotation::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnnotatedRow>>(detail::member_value<AnnotatedRow>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& AnnotationMemberAnswerId::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Observation>>& AnnotationMemberAnswerId::observations() const {
        auto stored = std::atomic_load(cached_observations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
            "observations"));
            if (std::atomic_compare_exchange_strong(cached_observations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& AnnotationMemberAnswerId::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<QuestionSource>>& AnnotationMemberAnswerId::question_sources() const {
        auto stored = std::atomic_load(cached_question_sources_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
            "question_sources"));
            if (std::atomic_compare_exchange_strong(cached_question_sources_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnnotationMemberAnswerId::request() const {
        auto stored = std::atomic_load(cached_request_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "request"));
            if (std::atomic_compare_exchange_strong(cached_request_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& AnnotationMemberAnswerId::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Usage>& AnnotationMemberAnswerId::usage() const {
        auto stored = std::atomic_load(cached_usage_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
            "usage"));
            if (std::atomic_compare_exchange_strong(cached_usage_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Value>& AnnotationMemberAnswerId::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Value>>(detail::member_value<Value>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Failure>& AnnotationMemberFailureId::failure() const {
        auto stored = std::atomic_load(cached_failure_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
            "failure"));
            if (std::atomic_compare_exchange_strong(cached_failure_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<FailureId>& AnnotationMemberFailureId::failure_id() const {
        auto stored = std::atomic_load(cached_failure_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<FailureId>>(detail::member_value<FailureId>(value_,
            "failure_id"));
            if (std::atomic_compare_exchange_strong(cached_failure_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Observation>>& AnnotationMemberFailureId::observations() const {
        auto stored = std::atomic_load(cached_observations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
            "observations"));
            if (std::atomic_compare_exchange_strong(cached_observations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& AnnotationMemberFailureId::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<QuestionSource>>& AnnotationMemberFailureId::question_sources() const {
        auto stored = std::atomic_load(cached_question_sources_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
            "question_sources"));
            if (std::atomic_compare_exchange_strong(cached_question_sources_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnnotationMemberFailureId::request() const {
        auto stored = std::atomic_load(cached_request_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "request"));
            if (std::atomic_compare_exchange_strong(cached_request_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& AnnotationMemberFailureId::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Usage>& AnnotationMemberFailureId::usage() const {
        auto stored = std::atomic_load(cached_usage_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
            "usage"));
            if (std::atomic_compare_exchange_strong(cached_usage_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnnotationValueChoice::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnnotationValueDecision::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& AnnotationValueDecision::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnnotationValueFailed::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Failure>& AnnotationValueFailed::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnnotationValueScore::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& AnnotationValueScore::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnnotationValueTags::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& AnnotationValueTags::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& AnswerId::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RelationMember>>& Answers::questions() const {
        auto stored = std::atomic_load(cached_questions_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RelationMember>>>(detail::member_value<std::vector<RelationMember>>(value_,
            "questions"));
            if (std::atomic_compare_exchange_strong(cached_questions_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Answer>& AtomicArrayOfString::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& AtomicArrayOfString::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Image>>& AtomicArrayOfString::images() const {
        auto stored = std::atomic_load(cached_images_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
            "images"));
            if (std::atomic_compare_exchange_strong(cached_images_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& AtomicArrayOfString::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& AtomicArrayOfString::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RankMember>>& AtomicArrayOfString::members() const {
        auto stored = std::atomic_load(cached_members_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
            "members"));
            if (std::atomic_compare_exchange_strong(cached_members_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& AtomicArrayOfString::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& AtomicArrayOfString::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AtomicArrayOfString::question_name() const {
        auto stored = std::atomic_load(cached_question_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "question_name"));
            if (std::atomic_compare_exchange_strong(cached_question_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& AtomicArrayOfString::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& AtomicArrayOfString::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& AtomicArrayOfString::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& AtomicArrayOfString::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Answer>& AtomicDecideValue::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& AtomicDecideValue::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Image>>& AtomicDecideValue::images() const {
        auto stored = std::atomic_load(cached_images_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
            "images"));
            if (std::atomic_compare_exchange_strong(cached_images_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& AtomicDecideValue::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& AtomicDecideValue::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RankMember>>& AtomicDecideValue::members() const {
        auto stored = std::atomic_load(cached_members_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
            "members"));
            if (std::atomic_compare_exchange_strong(cached_members_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& AtomicDecideValue::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& AtomicDecideValue::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AtomicDecideValue::question_name() const {
        auto stored = std::atomic_load(cached_question_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "question_name"));
            if (std::atomic_compare_exchange_strong(cached_question_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& AtomicDecideValue::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& AtomicDecideValue::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& AtomicDecideValue::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<DecideValue>& AtomicDecideValue::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<DecideValue>>(detail::member_value<DecideValue>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Answer>& AtomicNonZeroUsize::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& AtomicNonZeroUsize::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Image>>& AtomicNonZeroUsize::images() const {
        auto stored = std::atomic_load(cached_images_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
            "images"));
            if (std::atomic_compare_exchange_strong(cached_images_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& AtomicNonZeroUsize::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& AtomicNonZeroUsize::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RankMember>>& AtomicNonZeroUsize::members() const {
        auto stored = std::atomic_load(cached_members_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
            "members"));
            if (std::atomic_compare_exchange_strong(cached_members_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& AtomicNonZeroUsize::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& AtomicNonZeroUsize::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AtomicNonZeroUsize::question_name() const {
        auto stored = std::atomic_load(cached_question_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "question_name"));
            if (std::atomic_compare_exchange_strong(cached_question_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& AtomicNonZeroUsize::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& AtomicNonZeroUsize::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& AtomicNonZeroUsize::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& AtomicNonZeroUsize::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Answer>& AtomicNullableString::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& AtomicNullableString::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Image>>& AtomicNullableString::images() const {
        auto stored = std::atomic_load(cached_images_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
            "images"));
            if (std::atomic_compare_exchange_strong(cached_images_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& AtomicNullableString::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& AtomicNullableString::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RankMember>>& AtomicNullableString::members() const {
        auto stored = std::atomic_load(cached_members_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
            "members"));
            if (std::atomic_compare_exchange_strong(cached_members_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& AtomicNullableString::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& AtomicNullableString::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AtomicNullableString::question_name() const {
        auto stored = std::atomic_load(cached_question_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "question_name"));
            if (std::atomic_compare_exchange_strong(cached_question_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& AtomicNullableString::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& AtomicNullableString::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& AtomicNullableString::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AtomicNullableString::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Answer>& AtomicBoolean::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& AtomicBoolean::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Image>>& AtomicBoolean::images() const {
        auto stored = std::atomic_load(cached_images_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
            "images"));
            if (std::atomic_compare_exchange_strong(cached_images_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& AtomicBoolean::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& AtomicBoolean::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RankMember>>& AtomicBoolean::members() const {
        auto stored = std::atomic_load(cached_members_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
            "members"));
            if (std::atomic_compare_exchange_strong(cached_members_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& AtomicBoolean::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& AtomicBoolean::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AtomicBoolean::question_name() const {
        auto stored = std::atomic_load(cached_question_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "question_name"));
            if (std::atomic_compare_exchange_strong(cached_question_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& AtomicBoolean::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& AtomicBoolean::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& AtomicBoolean::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& AtomicBoolean::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Answer>& AtomicDouble::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& AtomicDouble::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Image>>& AtomicDouble::images() const {
        auto stored = std::atomic_load(cached_images_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
            "images"));
            if (std::atomic_compare_exchange_strong(cached_images_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& AtomicDouble::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& AtomicDouble::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RankMember>>& AtomicDouble::members() const {
        auto stored = std::atomic_load(cached_members_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RankMember>>>(detail::member_value<std::vector<RankMember>>(value_,
            "members"));
            if (std::atomic_compare_exchange_strong(cached_members_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& AtomicDouble::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& AtomicDouble::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AtomicDouble::question_name() const {
        auto stored = std::atomic_load(cached_question_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "question_name"));
            if (std::atomic_compare_exchange_strong(cached_question_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& AtomicDouble::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& AtomicDouble::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& AtomicDouble::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& AtomicDouble::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Attempt::ordinal() const {
        auto stored = std::atomic_load(cached_ordinal_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "ordinal"));
            if (std::atomic_compare_exchange_strong(cached_ordinal_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AttemptOutcome>& Attempt::outcome() const {
        auto stored = std::atomic_load(cached_outcome_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AttemptOutcome>>(detail::member_value<AttemptOutcome>(value_,
            "outcome"));
            if (std::atomic_compare_exchange_strong(cached_outcome_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Attempt::request_id() const {
        auto stored = std::atomic_load(cached_request_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "request_id"));
            if (std::atomic_compare_exchange_strong(cached_request_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Attempt::request_sha256() const {
        auto stored = std::atomic_load(cached_request_sha256_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "request_sha256"));
            if (std::atomic_compare_exchange_strong(cached_request_sha256_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<SdkRequestId>& Attempt::sdk_request_id() const {
        auto stored = std::atomic_load(cached_sdk_request_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<SdkRequestId>>(detail::member_value<SdkRequestId>(value_,
            "sdk_request_id"));
            if (std::atomic_compare_exchange_strong(cached_sdk_request_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Attempt::server_ms() const {
        auto stored = std::atomic_load(cached_server_ms_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "server_ms"));
            if (std::atomic_compare_exchange_strong(cached_server_ms_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Attempt::status() const {
        auto stored = std::atomic_load(cached_status_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "status"));
            if (std::atomic_compare_exchange_strong(cached_status_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Attempt::wall_ms() const {
        auto stored = std::atomic_load(cached_wall_ms_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "wall_ms"));
            if (std::atomic_compare_exchange_strong(cached_wall_ms_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::variant<uint64_t, std::string>& Batch::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::variant<uint64_t, std::string>>(detail::decode<std::variant<uint64_t,
            std::string>>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& BoundaryMode::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<PieceOdds>>& BoundaryOdds::pieces() const {
        auto stored = std::atomic_load(cached_pieces_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<PieceOdds>>>(detail::member_value<std::vector<PieceOdds>>(value_,
            "pieces"));
            if (std::atomic_compare_exchange_strong(cached_pieces_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<BoundaryProposal>>& BoundaryOdds::proposals() const {
        auto stored = std::atomic_load(cached_proposals_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<BoundaryProposal>>>(detail::member_value<std::vector<BoundaryProposal>>(value_,
            "proposals"));
            if (std::atomic_compare_exchange_strong(cached_proposals_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& BoundaryProposal::end() const {
        auto stored = std::atomic_load(cached_end_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "end"));
            if (std::atomic_compare_exchange_strong(cached_end_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& BoundaryProposal::length() const {
        auto stored = std::atomic_load(cached_length_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "length"));
            if (std::atomic_compare_exchange_strong(cached_length_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& BoundaryProposal::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& BoundaryProposal::start() const {
        auto stored = std::atomic_load(cached_start_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "start"));
            if (std::atomic_compare_exchange_strong(cached_start_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& BoundaryProposal::text() const {
        auto stored = std::atomic_load(cached_text_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "text"));
            if (std::atomic_compare_exchange_strong(cached_text_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Error>& CallError::error() const {
        auto stored = std::atomic_load(cached_error_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Error>>(detail::member_value<Error>(value_,
            "error"));
            if (std::atomic_compare_exchange_strong(cached_error_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Facts>& CallError::facts() const {
        auto stored = std::atomic_load(cached_facts_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Facts>>(detail::member_value<Facts>(value_,
            "facts"));
            if (std::atomic_compare_exchange_strong(cached_facts_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& CallId::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Data& DecideValue::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Data>(detail::decode<Data>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& EntityDocument::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& EntityDocument::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<EstimatedInputDenial>& Error::estimated_input_denial() const {
        auto stored = std::atomic_load(cached_estimated_input_denial_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<EstimatedInputDenial>>(detail::member_value<EstimatedInputDenial>(value_,
            "estimated_input_denial"));
            if (std::atomic_compare_exchange_strong(cached_estimated_input_denial_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<FailureKind>& Error::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<FailureKind>>(detail::member_value<FailureKind>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Error::message() const {
        auto stored = std::atomic_load(cached_message_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "message"));
            if (std::atomic_compare_exchange_strong(cached_message_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& Error::retryable() const {
        auto stored = std::atomic_load(cached_retryable_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "retryable"));
            if (std::atomic_compare_exchange_strong(cached_retryable_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<SendBudgetDenial>& Error::send_budget_denial() const {
        auto stored = std::atomic_load(cached_send_budget_denial_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<SendBudgetDenial>>(detail::member_value<SendBudgetDenial>(value_,
            "send_budget_denial"));
            if (std::atomic_compare_exchange_strong(cached_send_budget_denial_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Stopped>& Error::stopped() const {
        auto stored = std::atomic_load(cached_stopped_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Stopped>>(detail::member_value<Stopped>(value_,
            "stopped"));
            if (std::atomic_compare_exchange_strong(cached_stopped_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& EstimatedInputDenialAdditionalRequest::limit() const {
        auto stored = std::atomic_load(cached_limit_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "limit"));
            if (std::atomic_compare_exchange_strong(cached_limit_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& EstimatedInputDenialInitialRequest::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& EstimatedInputDenialInitialRequest::limit() const {
        auto stored = std::atomic_load(cached_limit_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "limit"));
            if (std::atomic_compare_exchange_strong(cached_limit_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& EstimatedInputDenialRetry::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& EstimatedInputDenialRetry::last_status() const {
        auto stored = std::atomic_load(cached_last_status_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_status"));
            if (std::atomic_compare_exchange_strong(cached_last_status_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& EstimatedInputDenialRetry::limit() const {
        auto stored = std::atomic_load(cached_limit_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "limit"));
            if (std::atomic_compare_exchange_strong(cached_limit_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Attempt>>& Facts::attempts() const {
        auto stored = std::atomic_load(cached_attempts_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Attempt>>>(detail::member_value<std::vector<Attempt>>(value_,
            "attempts"));
            if (std::atomic_compare_exchange_strong(cached_attempts_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Facts::cache_answers() const {
        auto stored = std::atomic_load(cached_cache_answers_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "cache_answers"));
            if (std::atomic_compare_exchange_strong(cached_cache_answers_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<CallId>& Facts::call_id() const {
        auto stored = std::atomic_load(cached_call_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<CallId>>(detail::member_value<CallId>(value_,
            "call_id"));
            if (std::atomic_compare_exchange_strong(cached_call_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Facts::estimated_cost_usd() const {
        auto stored = std::atomic_load(cached_estimated_cost_usd_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "estimated_cost_usd"));
            if (std::atomic_compare_exchange_strong(cached_estimated_cost_usd_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& Facts::held_model_mismatch() const {
        auto stored = std::atomic_load(cached_held_model_mismatch_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "held_model_mismatch"));
            if (std::atomic_compare_exchange_strong(cached_held_model_mismatch_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Facts::input_tokens() const {
        auto stored = std::atomic_load(cached_input_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "input_tokens"));
            if (std::atomic_compare_exchange_strong(cached_input_tokens_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Facts::largest_request_bytes() const {
        auto stored = std::atomic_load(cached_largest_request_bytes_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "largest_request_bytes"));
            if (std::atomic_compare_exchange_strong(cached_largest_request_bytes_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Facts::largest_request_estimated_input_tokens() const {
        auto stored = std::atomic_load(cached_largest_request_estimated_input_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "largest_request_estimated_input_tokens"));
            if (std::atomic_compare_exchange_strong(cached_largest_request_estimated_input_tokens_.get(),
            &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Facts::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Facts::output_tokens() const {
        auto stored = std::atomic_load(cached_output_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "output_tokens"));
            if (std::atomic_compare_exchange_strong(cached_output_tokens_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Facts::records() const {
        auto stored = std::atomic_load(cached_records_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "records"));
            if (std::atomic_compare_exchange_strong(cached_records_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Facts::requests_sent() const {
        auto stored = std::atomic_load(cached_requests_sent_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "requests_sent"));
            if (std::atomic_compare_exchange_strong(cached_requests_sent_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& Facts::seconds() const {
        auto stored = std::atomic_load(cached_seconds_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "seconds"));
            if (std::atomic_compare_exchange_strong(cached_seconds_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Facts::token_estimate_method() const {
        auto stored = std::atomic_load(cached_token_estimate_method_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "token_estimate_method"));
            if (std::atomic_compare_exchange_strong(cached_token_estimate_method_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PersistenceObservation>& Facts::usage_persistence() const {
        auto stored = std::atomic_load(cached_usage_persistence_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PersistenceObservation>>(detail::member_value<PersistenceObservation>(value_,
            "usage_persistence"));
            if (std::atomic_compare_exchange_strong(cached_usage_persistence_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& FailureId::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<FindAnswer>& Find::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<FindAnswer>>(detail::member_value<FindAnswer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& Find::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<FindCandidate>>& Find::candidates() const {
        auto stored = std::atomic_load(cached_candidates_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<FindCandidate>>>(detail::member_value<std::vector<FindCandidate>>(value_,
            "candidates"));
            if (std::atomic_compare_exchange_strong(cached_candidates_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Find::file() const {
        auto stored = std::atomic_load(cached_file_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "file"));
            if (std::atomic_compare_exchange_strong(cached_file_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Find::first_line() const {
        auto stored = std::atomic_load(cached_first_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "first_line"));
            if (std::atomic_compare_exchange_strong(cached_first_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Find::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Find::last_line() const {
        auto stored = std::atomic_load(cached_last_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_line"));
            if (std::atomic_compare_exchange_strong(cached_last_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& Find::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Position>& Find::position() const {
        auto stored = std::atomic_load(cached_position_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Position>>(detail::member_value<Position>(value_,
            "position"));
            if (std::atomic_compare_exchange_strong(cached_position_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<FindQuestionDocument>& Find::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<FindQuestionDocument>>(detail::member_value<FindQuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& Find::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::nullptr_t>& Find::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::nullptr_t>>(detail::member_value<std::nullptr_t>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& Find::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& FindCandidate::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& FindCandidate::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& FindCandidate::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& FindCandidate::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Image::base64() const {
        auto stored = std::atomic_load(cached_base64_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "base64"));
            if (std::atomic_compare_exchange_strong(cached_base64_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint32_t>& Image::height() const {
        auto stored = std::atomic_load(cached_height_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint32_t>>(detail::member_value<uint32_t>(value_,
            "height"));
            if (std::atomic_compare_exchange_strong(cached_height_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<ImageMedia>& Image::media() const {
        auto stored = std::atomic_load(cached_media_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<ImageMedia>>(detail::member_value<ImageMedia>(value_,
            "media"));
            if (std::atomic_compare_exchange_strong(cached_media_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint32_t>& Image::width() const {
        auto stored = std::atomic_load(cached_width_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint32_t>>(detail::member_value<uint32_t>(value_,
            "width"));
            if (std::atomic_compare_exchange_strong(cached_width_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& ImageMedia::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Attempt>>& ImmediateFacts::attempts() const {
        auto stored = std::atomic_load(cached_attempts_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Attempt>>>(detail::member_value<std::vector<Attempt>>(value_,
            "attempts"));
            if (std::atomic_compare_exchange_strong(cached_attempts_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& ImmediateFacts::cache_answers() const {
        auto stored = std::atomic_load(cached_cache_answers_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "cache_answers"));
            if (std::atomic_compare_exchange_strong(cached_cache_answers_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<CallId>& ImmediateFacts::call_id() const {
        auto stored = std::atomic_load(cached_call_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<CallId>>(detail::member_value<CallId>(value_,
            "call_id"));
            if (std::atomic_compare_exchange_strong(cached_call_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& ImmediateFacts::estimated_cost_usd() const {
        auto stored = std::atomic_load(cached_estimated_cost_usd_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "estimated_cost_usd"));
            if (std::atomic_compare_exchange_strong(cached_estimated_cost_usd_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& ImmediateFacts::held_model_mismatch() const {
        auto stored = std::atomic_load(cached_held_model_mismatch_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "held_model_mismatch"));
            if (std::atomic_compare_exchange_strong(cached_held_model_mismatch_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& ImmediateFacts::input_tokens() const {
        auto stored = std::atomic_load(cached_input_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "input_tokens"));
            if (std::atomic_compare_exchange_strong(cached_input_tokens_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& ImmediateFacts::largest_request_bytes() const {
        auto stored = std::atomic_load(cached_largest_request_bytes_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "largest_request_bytes"));
            if (std::atomic_compare_exchange_strong(cached_largest_request_bytes_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& ImmediateFacts::largest_request_estimated_input_tokens() const {
        auto stored = std::atomic_load(cached_largest_request_estimated_input_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "largest_request_estimated_input_tokens"));
            if (std::atomic_compare_exchange_strong(cached_largest_request_estimated_input_tokens_.get(),
            &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& ImmediateFacts::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& ImmediateFacts::output_tokens() const {
        auto stored = std::atomic_load(cached_output_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "output_tokens"));
            if (std::atomic_compare_exchange_strong(cached_output_tokens_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& ImmediateFacts::records() const {
        auto stored = std::atomic_load(cached_records_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "records"));
            if (std::atomic_compare_exchange_strong(cached_records_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& ImmediateFacts::requests_sent() const {
        auto stored = std::atomic_load(cached_requests_sent_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "requests_sent"));
            if (std::atomic_compare_exchange_strong(cached_requests_sent_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& ImmediateFacts::seconds() const {
        auto stored = std::atomic_load(cached_seconds_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "seconds"));
            if (std::atomic_compare_exchange_strong(cached_seconds_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& ImmediateFacts::token_estimate_method() const {
        auto stored = std::atomic_load(cached_token_estimate_method_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "token_estimate_method"));
            if (std::atomic_compare_exchange_strong(cached_token_estimate_method_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PersistenceObservation>& ImmediateFacts::usage_persistence() const {
        auto stored = std::atomic_load(cached_usage_persistence_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PersistenceObservation>>(detail::member_value<PersistenceObservation>(value_,
            "usage_persistence"));
            if (std::atomic_compare_exchange_strong(cached_usage_persistence_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_properties_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, InputPropertyType>>>>(detail::member_value<std::vector<std::pair<std::string,
            InputPropertyType>>>(value_, "properties"));
            if (std::atomic_compare_exchange_strong(cached_properties_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& InputDeclarationObjectRoot::required() const {
        auto stored = std::atomic_load(cached_required_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "required"));
            if (std::atomic_compare_exchange_strong(cached_required_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<ObjectType>& InputDeclarationObjectRoot::type() const {
        auto stored = std::atomic_load(cached_type_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<ObjectType>>(detail::member_value<ObjectType>(value_,
            "type"));
            if (std::atomic_compare_exchange_strong(cached_type_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<StringType>& InputDeclarationStringRoot::type() const {
        auto stored = std::atomic_load(cached_type_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<StringType>>(detail::member_value<StringType>(value_,
            "type"));
            if (std::atomic_compare_exchange_strong(cached_type_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_items_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<StringRoot>>(detail::member_value<StringRoot>(value_,
            "items"));
            if (std::atomic_compare_exchange_strong(cached_items_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& InputPropertyTypeArray::type() const {
        auto stored = std::atomic_load(cached_type_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "type"));
            if (std::atomic_compare_exchange_strong(cached_type_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& InputPropertyTypeBoolean::type() const {
        auto stored = std::atomic_load(cached_type_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "type"));
            if (std::atomic_compare_exchange_strong(cached_type_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& InputPropertyTypeNumber::type() const {
        auto stored = std::atomic_load(cached_type_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "type"));
            if (std::atomic_compare_exchange_strong(cached_type_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& InputPropertyTypeString::type() const {
        auto stored = std::atomic_load(cached_type_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "type"));
            if (std::atomic_compare_exchange_strong(cached_type_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& Label::description() const {
        auto stored = std::atomic_load(cached_description_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "description"));
            if (std::atomic_compare_exchange_strong(cached_description_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Label::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Meta::answered_by() const {
        auto stored = std::atomic_load(cached_answered_by_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "answered_by"));
            if (std::atomic_compare_exchange_strong(cached_answered_by_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Attempt>>& Meta::attempts() const {
        auto stored = std::atomic_load(cached_attempts_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Attempt>>>(detail::member_value<std::vector<Attempt>>(value_,
            "attempts"));
            if (std::atomic_compare_exchange_strong(cached_attempts_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<BatchSetting>& Meta::batch_setting() const {
        auto stored = std::atomic_load(cached_batch_setting_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<BatchSetting>>(detail::member_value<BatchSetting>(value_,
            "batch_setting"));
            if (std::atomic_compare_exchange_strong(cached_batch_setting_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<BatchWarning>& Meta::batch_warning() const {
        auto stored = std::atomic_load(cached_batch_warning_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<BatchWarning>>(detail::member_value<BatchWarning>(value_,
            "batch_warning"));
            if (std::atomic_compare_exchange_strong(cached_batch_warning_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& Meta::cached() const {
        auto stored = std::atomic_load(cached_cached_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "cached"));
            if (std::atomic_compare_exchange_strong(cached_cached_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Meta::context_sha256() const {
        auto stored = std::atomic_load(cached_context_sha256_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "context_sha256"));
            if (std::atomic_compare_exchange_strong(cached_context_sha256_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Meta::failed_questions() const {
        auto stored = std::atomic_load(cached_failed_questions_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "failed_questions"));
            if (std::atomic_compare_exchange_strong(cached_failed_questions_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Meta::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Observation>>& Meta::observations() const {
        auto stored = std::atomic_load(cached_observations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
            "observations"));
            if (std::atomic_compare_exchange_strong(cached_observations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Origin>& Meta::origin() const {
        auto stored = std::atomic_load(cached_origin_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Origin>>(detail::member_value<Origin>(value_,
            "origin"));
            if (std::atomic_compare_exchange_strong(cached_origin_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<ProfileWarning>& Meta::profile_warning() const {
        auto stored = std::atomic_load(cached_profile_warning_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<ProfileWarning>>(detail::member_value<ProfileWarning>(value_,
            "profile_warning"));
            if (std::atomic_compare_exchange_strong(cached_profile_warning_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Meta::question_sha256() const {
        auto stored = std::atomic_load(cached_question_sha256_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "question_sha256"));
            if (std::atomic_compare_exchange_strong(cached_question_sha256_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<QuestionSource>>& Meta::question_sources() const {
        auto stored = std::atomic_load(cached_question_sources_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
            "question_sources"));
            if (std::atomic_compare_exchange_strong(cached_question_sources_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Meta::questions_sha256() const {
        auto stored = std::atomic_load(cached_questions_sha256_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "questions_sha256"));
            if (std::atomic_compare_exchange_strong(cached_questions_sha256_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& Meta::requests() const {
        auto stored = std::atomic_load(cached_requests_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "requests"));
            if (std::atomic_compare_exchange_strong(cached_requests_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Meta::requests_sent() const {
        auto stored = std::atomic_load(cached_requests_sent_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "requests_sent"));
            if (std::atomic_compare_exchange_strong(cached_requests_sent_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Meta::tool() const {
        auto stored = std::atomic_load(cached_tool_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "tool"));
            if (std::atomic_compare_exchange_strong(cached_tool_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Meta::url() const {
        auto stored = std::atomic_load(cached_url_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "url"));
            if (std::atomic_compare_exchange_strong(cached_url_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Usage>& Meta::usage() const {
        auto stored = std::atomic_load(cached_usage_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
            "usage"));
            if (std::atomic_compare_exchange_strong(cached_usage_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, InputPropertyType>>>& ObjectRoot::properties() const {
        auto stored = std::atomic_load(cached_properties_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, InputPropertyType>>>>(detail::member_value<std::vector<std::pair<std::string,
            InputPropertyType>>>(value_, "properties"));
            if (std::atomic_compare_exchange_strong(cached_properties_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& ObjectRoot::required() const {
        auto stored = std::atomic_load(cached_required_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "required"));
            if (std::atomic_compare_exchange_strong(cached_required_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<ObjectType>& ObjectRoot::type() const {
        auto stored = std::atomic_load(cached_type_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<ObjectType>>(detail::member_value<ObjectType>(value_,
            "type"));
            if (std::atomic_compare_exchange_strong(cached_type_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& ObjectType::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<FailureId>& ObservationFailureId::failure_id() const {
        auto stored = std::atomic_load(cached_failure_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<FailureId>>(detail::member_value<FailureId>(value_,
            "failure_id"));
            if (std::atomic_compare_exchange_strong(cached_failure_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<ObservationId>& ObservationObservationId::observation_id() const {
        auto stored = std::atomic_load(cached_observation_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<ObservationId>>(detail::member_value<ObservationId>(value_,
            "observation_id"));
            if (std::atomic_compare_exchange_strong(cached_observation_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& Origin::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& PersistenceObservation::advice() const {
        auto stored = std::atomic_load(cached_advice_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "advice"));
            if (std::atomic_compare_exchange_strong(cached_advice_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& PersistenceObservation::observed_at() const {
        auto stored = std::atomic_load(cached_observed_at_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "observed_at"));
            if (std::atomic_compare_exchange_strong(cached_observed_at_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<UsagePersistence>& PersistenceObservation::state() const {
        auto stored = std::atomic_load(cached_state_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<UsagePersistence>>(detail::member_value<UsagePersistence>(value_,
            "state"));
            if (std::atomic_compare_exchange_strong(cached_state_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& PhysicalSource::file() const {
        auto stored = std::atomic_load(cached_file_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "file"));
            if (std::atomic_compare_exchange_strong(cached_file_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& PhysicalSource::first_line() const {
        auto stored = std::atomic_load(cached_first_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "first_line"));
            if (std::atomic_compare_exchange_strong(cached_first_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& PhysicalSource::last_line() const {
        auto stored = std::atomic_load(cached_last_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_line"));
            if (std::atomic_compare_exchange_strong(cached_last_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Position::file() const {
        auto stored = std::atomic_load(cached_file_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "file"));
            if (std::atomic_compare_exchange_strong(cached_file_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Position::first() const {
        auto stored = std::atomic_load(cached_first_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "first"));
            if (std::atomic_compare_exchange_strong(cached_first_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& Position::images() const {
        auto stored = std::atomic_load(cached_images_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "images"));
            if (std::atomic_compare_exchange_strong(cached_images_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Position::last() const {
        auto stored = std::atomic_load(cached_last_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last"));
            if (std::atomic_compare_exchange_strong(cached_last_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& QuestionName::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionSource::answered_by() const {
        auto stored = std::atomic_load(cached_answered_by_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "answered_by"));
            if (std::atomic_compare_exchange_strong(cached_answered_by_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint32_t>& QuestionSource::batch_size() const {
        auto stored = std::atomic_load(cached_batch_size_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint32_t>>(detail::member_value<uint32_t>(value_,
            "batch_size"));
            if (std::atomic_compare_exchange_strong(cached_batch_size_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Origin>& QuestionSource::origin() const {
        auto stored = std::atomic_load(cached_origin_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Origin>>(detail::member_value<Origin>(value_,
            "origin"));
            if (std::atomic_compare_exchange_strong(cached_origin_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RankMember::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RankMemberResult>& RankMember::result() const {
        auto stored = std::atomic_load(cached_result_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RankMemberResult>>(detail::member_value<RankMemberResult>(value_,
            "result"));
            if (std::atomic_compare_exchange_strong(cached_result_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Answer>& RankMemberResult::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& RankMemberResult::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Image>>& RankMemberResult::images() const {
        auto stored = std::atomic_load(cached_images_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Image>>>(detail::member_value<std::vector<Image>>(value_,
            "images"));
            if (std::atomic_compare_exchange_strong(cached_images_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& RankMemberResult::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& RankMemberResult::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& RankMemberResult::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& RankMemberResult::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::nullptr_t>& RankMemberResult::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::nullptr_t>>(detail::member_value<std::nullptr_t>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& RankMemberResult::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RecognitionOdds>& Recognition::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RecognitionOdds>>(detail::member_value<RecognitionOdds>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& Recognition::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Recognition::file() const {
        auto stored = std::atomic_load(cached_file_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "file"));
            if (std::atomic_compare_exchange_strong(cached_file_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Recognition::first_line() const {
        auto stored = std::atomic_load(cached_first_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "first_line"));
            if (std::atomic_compare_exchange_strong(cached_first_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Recognition::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& Recognition::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Recognition::last_line() const {
        auto stored = std::atomic_load(cached_last_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_line"));
            if (std::atomic_compare_exchange_strong(cached_last_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& Recognition::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Position>& Recognition::position() const {
        auto stored = std::atomic_load(cached_position_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Position>>(detail::member_value<Position>(value_,
            "position"));
            if (std::atomic_compare_exchange_strong(cached_position_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RecognizeQuestionDocument>& Recognition::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RecognizeQuestionDocument>>(detail::member_value<RecognizeQuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& Recognition::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& Recognition::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Recognize>& Recognition::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Recognize>>(detail::member_value<Recognize>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& RecognitionEdgeDocument::either() const {
        auto stored = std::atomic_load(cached_either_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "either"));
            if (std::atomic_compare_exchange_strong(cached_either_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& RecognitionEdgeDocument::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RecognitionEdgeDocument::relation() const {
        auto stored = std::atomic_load(cached_relation_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "relation"));
            if (std::atomic_compare_exchange_strong(cached_relation_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Entity>& RecognitionEdgeDocument::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Entity>>(detail::member_value<Entity>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Entity>& RecognitionEdgeDocument::target() const {
        auto stored = std::atomic_load(cached_target_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Entity>>(detail::member_value<Entity>(value_,
            "target"));
            if (std::atomic_compare_exchange_strong(cached_target_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& RecognitionMode::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_pieces_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<PieceOdds>>>(detail::member_value<std::vector<PieceOdds>>(value_,
            "pieces"));
            if (std::atomic_compare_exchange_strong(cached_pieces_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<BoundaryProposal>>& RecognitionOddsBoundaryOdds::proposals() const {
        auto stored = std::atomic_load(cached_proposals_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<BoundaryProposal>>>(detail::member_value<std::vector<BoundaryProposal>>(value_,
            "proposals"));
            if (std::atomic_compare_exchange_strong(cached_proposals_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<NameOdds>>& RecognitionOddsRecognizeAnswer::names() const {
        auto stored = std::atomic_load(cached_names_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<NameOdds>>>(detail::member_value<std::vector<NameOdds>>(value_,
            "names"));
            if (std::atomic_compare_exchange_strong(cached_names_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<PairOdds>>& RecognitionOddsRecognizeAnswer::pairs() const {
        auto stored = std::atomic_load(cached_pairs_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<PairOdds>>>(detail::member_value<std::vector<PairOdds>>(value_,
            "pairs"));
            if (std::atomic_compare_exchange_strong(cached_pairs_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<PieceOdds>>& RecognitionOddsRecognizeAnswer::pieces() const {
        auto stored = std::atomic_load(cached_pieces_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<PieceOdds>>>(detail::member_value<std::vector<PieceOdds>>(value_,
            "pieces"));
            if (std::atomic_compare_exchange_strong(cached_pieces_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RecognitionProposal>>& RecognitionOddsRecognizeAnswer::proposals() const {
        auto stored = std::atomic_load(cached_proposals_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RecognitionProposal>>>(detail::member_value<std::vector<RecognitionProposal>>(value_,
            "proposals"));
            if (std::atomic_compare_exchange_strong(cached_proposals_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& RecognitionProposal::end() const {
        auto stored = std::atomic_load(cached_end_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "end"));
            if (std::atomic_compare_exchange_strong(cached_end_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& RecognitionProposal::kept() const {
        auto stored = std::atomic_load(cached_kept_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "kept"));
            if (std::atomic_compare_exchange_strong(cached_kept_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RecognitionProposal::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Place>& RecognitionProposal::selected() const {
        auto stored = std::atomic_load(cached_selected_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Place>>(detail::member_value<Place>(value_,
            "selected"));
            if (std::atomic_compare_exchange_strong(cached_selected_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& RecognitionProposal::span_probability() const {
        auto stored = std::atomic_load(cached_span_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "span_probability"));
            if (std::atomic_compare_exchange_strong(cached_span_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& RecognitionProposal::start() const {
        auto stored = std::atomic_load(cached_start_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "start"));
            if (std::atomic_compare_exchange_strong(cached_start_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& RecognitionProposal::strength() const {
        auto stored = std::atomic_load(cached_strength_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "strength"));
            if (std::atomic_compare_exchange_strong(cached_strength_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RecognitionStageContext::boundary() const {
        auto stored = std::atomic_load(cached_boundary_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "boundary"));
            if (std::atomic_compare_exchange_strong(cached_boundary_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RecognitionStageContext::kind_edge() const {
        auto stored = std::atomic_load(cached_kind_edge_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind_edge"));
            if (std::atomic_compare_exchange_strong(cached_kind_edge_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RecognitionStageContext::relation() const {
        auto stored = std::atomic_load(cached_relation_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "relation"));
            if (std::atomic_compare_exchange_strong(cached_relation_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Answers>& Relation::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answers>>(detail::member_value<Answers>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& Relation::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Relation::file() const {
        auto stored = std::atomic_load(cached_file_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "file"));
            if (std::atomic_compare_exchange_strong(cached_file_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Relation::first_line() const {
        auto stored = std::atomic_load(cached_first_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "first_line"));
            if (std::atomic_compare_exchange_strong(cached_first_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Relation::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& Relation::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<SessionInputSource>>& Relation::input_sources() const {
        auto stored = std::atomic_load(cached_input_sources_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<SessionInputSource>>>(detail::member_value<std::vector<SessionInputSource>>(value_,
            "input_sources"));
            if (std::atomic_compare_exchange_strong(cached_input_sources_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Relation::last_line() const {
        auto stored = std::atomic_load(cached_last_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_line"));
            if (std::atomic_compare_exchange_strong(cached_last_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Meta>& Relation::meta() const {
        auto stored = std::atomic_load(cached_meta_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Meta>>(detail::member_value<Meta>(value_,
            "meta"));
            if (std::atomic_compare_exchange_strong(cached_meta_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Position>& Relation::position() const {
        auto stored = std::atomic_load(cached_position_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Position>>(detail::member_value<Position>(value_,
            "position"));
            if (std::atomic_compare_exchange_strong(cached_position_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelateQuestionDocument>& Relation::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelateQuestionDocument>>(detail::member_value<RelateQuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Version>& Relation::schema() const {
        auto stored = std::atomic_load(cached_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Version>>(detail::member_value<Version>(value_,
            "schema"));
            if (std::atomic_compare_exchange_strong(cached_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RelatedEntityEdge>>& Relation::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RelatedEntityEdge>>>(detail::member_value<std::vector<RelatedEntityEdge>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& RelationDirection::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationEndpointRelatedEntity::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationEndpointSource::file() const {
        auto stored = std::atomic_load(cached_file_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "file"));
            if (std::atomic_compare_exchange_strong(cached_file_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& RelationEndpointSource::first_line() const {
        auto stored = std::atomic_load(cached_first_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "first_line"));
            if (std::atomic_compare_exchange_strong(cached_first_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationEndpointSource::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& RelationEndpointSource::last_line() const {
        auto stored = std::atomic_load(cached_last_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_line"));
            if (std::atomic_compare_exchange_strong(cached_last_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationEndpointSource::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& RelationEndpointSource::ordinal() const {
        auto stored = std::atomic_load(cached_ordinal_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "ordinal"));
            if (std::atomic_compare_exchange_strong(cached_ordinal_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& RelationEndpointSource::record() const {
        auto stored = std::atomic_load(cached_record_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "record"));
            if (std::atomic_compare_exchange_strong(cached_record_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_direction_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelationDirection>>(detail::member_value<RelationDirection>(value_,
            "direction"));
            if (std::atomic_compare_exchange_strong(cached_direction_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelationMethod>& RelationMemberAnswerId::method() const {
        auto stored = std::atomic_load(cached_method_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelationMethod>>(detail::member_value<RelationMethod>(value_,
            "method"));
            if (std::atomic_compare_exchange_strong(cached_method_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Observation>>& RelationMemberAnswerId::observations() const {
        auto stored = std::atomic_load(cached_observations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
            "observations"));
            if (std::atomic_compare_exchange_strong(cached_observations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& RelationMemberAnswerId::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<QuestionSource>>& RelationMemberAnswerId::question_sources() const {
        auto stored = std::atomic_load(cached_question_sources_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
            "question_sources"));
            if (std::atomic_compare_exchange_strong(cached_question_sources_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationMemberAnswerId::reads() const {
        auto stored = std::atomic_load(cached_reads_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "reads"));
            if (std::atomic_compare_exchange_strong(cached_reads_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationMemberAnswerId::relation() const {
        auto stored = std::atomic_load(cached_relation_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "relation"));
            if (std::atomic_compare_exchange_strong(cached_relation_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationMemberAnswerId::request() const {
        auto stored = std::atomic_load(cached_request_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "request"));
            if (std::atomic_compare_exchange_strong(cached_request_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelatedEntity>& RelationMemberAnswerId::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelatedEntity>>(detail::member_value<RelatedEntity>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelatedEntity>& RelationMemberAnswerId::target() const {
        auto stored = std::atomic_load(cached_target_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelatedEntity>>(detail::member_value<RelatedEntity>(value_,
            "target"));
            if (std::atomic_compare_exchange_strong(cached_target_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& RelationMemberAnswerId::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Usage>& RelationMemberAnswerId::usage() const {
        auto stored = std::atomic_load(cached_usage_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
            "usage"));
            if (std::atomic_compare_exchange_strong(cached_usage_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& RelationMemberAnswerId::accepted() const {
        auto stored = std::atomic_load(cached_accepted_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "accepted"));
            if (std::atomic_compare_exchange_strong(cached_accepted_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Answer>& RelationMemberAnswerId::answer() const {
        auto stored = std::atomic_load(cached_answer_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Answer>>(detail::member_value<Answer>(value_,
            "answer"));
            if (std::atomic_compare_exchange_strong(cached_answer_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& RelationMemberAnswerId::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& RelationMemberAnswerId::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline std::string RelationMemberAnswerId::source_name() const {
        return value_.at("source").at("name").get<std::string>();
    }
    inline const Presence<RelationDirection>& RelationMemberFailureId::direction() const {
        auto stored = std::atomic_load(cached_direction_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelationDirection>>(detail::member_value<RelationDirection>(value_,
            "direction"));
            if (std::atomic_compare_exchange_strong(cached_direction_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelationMethod>& RelationMemberFailureId::method() const {
        auto stored = std::atomic_load(cached_method_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelationMethod>>(detail::member_value<RelationMethod>(value_,
            "method"));
            if (std::atomic_compare_exchange_strong(cached_method_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Observation>>& RelationMemberFailureId::observations() const {
        auto stored = std::atomic_load(cached_observations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
            "observations"));
            if (std::atomic_compare_exchange_strong(cached_observations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& RelationMemberFailureId::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<QuestionSource>>& RelationMemberFailureId::question_sources() const {
        auto stored = std::atomic_load(cached_question_sources_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
            "question_sources"));
            if (std::atomic_compare_exchange_strong(cached_question_sources_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationMemberFailureId::reads() const {
        auto stored = std::atomic_load(cached_reads_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "reads"));
            if (std::atomic_compare_exchange_strong(cached_reads_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationMemberFailureId::relation() const {
        auto stored = std::atomic_load(cached_relation_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "relation"));
            if (std::atomic_compare_exchange_strong(cached_relation_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationMemberFailureId::request() const {
        auto stored = std::atomic_load(cached_request_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "request"));
            if (std::atomic_compare_exchange_strong(cached_request_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelatedEntity>& RelationMemberFailureId::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelatedEntity>>(detail::member_value<RelatedEntity>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelatedEntity>& RelationMemberFailureId::target() const {
        auto stored = std::atomic_load(cached_target_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelatedEntity>>(detail::member_value<RelatedEntity>(value_,
            "target"));
            if (std::atomic_compare_exchange_strong(cached_target_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& RelationMemberFailureId::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Usage>& RelationMemberFailureId::usage() const {
        auto stored = std::atomic_load(cached_usage_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
            "usage"));
            if (std::atomic_compare_exchange_strong(cached_usage_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Failure>& RelationMemberFailureId::failure() const {
        auto stored = std::atomic_load(cached_failure_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
            "failure"));
            if (std::atomic_compare_exchange_strong(cached_failure_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<FailureId>& RelationMemberFailureId::failure_id() const {
        auto stored = std::atomic_load(cached_failure_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<FailureId>>(detail::member_value<FailureId>(value_,
            "failure_id"));
            if (std::atomic_compare_exchange_strong(cached_failure_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline std::string RelationMemberFailureId::source_name() const {
        return value_.at("source").at("name").get<std::string>();
    }
    inline const std::string& RelationMethod::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& RequestFunction::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& SdkRequestId::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SendBudgetDenialBeforeFirstSend::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SendBudgetDenialBeforeRetry::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SendBudgetDenialBeforeRetry::last_status() const {
        auto stored = std::atomic_load(cached_last_status_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_status"));
            if (std::atomic_compare_exchange_strong(cached_last_status_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& StopCause::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Stopped::at() const {
        auto stored = std::atomic_load(cached_at_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "at"));
            if (std::atomic_compare_exchange_strong(cached_at_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<StopCause>& Stopped::cause() const {
        auto stored = std::atomic_load(cached_cause_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<StopCause>>(detail::member_value<StopCause>(value_,
            "cause"));
            if (std::atomic_compare_exchange_strong(cached_cause_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& Stopped::retryable() const {
        auto stored = std::atomic_load(cached_retryable_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "retryable"));
            if (std::atomic_compare_exchange_strong(cached_retryable_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Stopped::status() const {
        auto stored = std::atomic_load(cached_status_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "status"));
            if (std::atomic_compare_exchange_strong(cached_status_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<StringType>& StringRoot::type() const {
        auto stored = std::atomic_load(cached_type_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<StringType>>(detail::member_value<StringType>(value_,
            "type"));
            if (std::atomic_compare_exchange_strong(cached_type_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& StringType::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Usage::input_tokens() const {
        auto stored = std::atomic_load(cached_input_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "input_tokens"));
            if (std::atomic_compare_exchange_strong(cached_input_tokens_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Usage::output_tokens() const {
        auto stored = std::atomic_load(cached_output_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "output_tokens"));
            if (std::atomic_compare_exchange_strong(cached_output_tokens_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& UsagePersistence::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& Verb::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& Version::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const uint32_t& WordingVersion::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const uint32_t>(detail::decode<uint32_t>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double,
    Failed>& AnnotatedField::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>,
            double, Failed>>(detail::decode<std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>,
            double, Failed>>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::vector<std::pair<std::string, AnnotatedField>>& AnnotatedRow::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::vector<std::pair<std::string, AnnotatedField>>>(detail::decode<std::vector<std::pair<std::string,
            AnnotatedField>>>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_confidence_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "confidence"));
            if (std::atomic_compare_exchange_strong(cached_confidence_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnswerChoice::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnswerChoice::pick() const {
        auto stored = std::atomic_load(cached_pick_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "pick"));
            if (std::atomic_compare_exchange_strong(cached_pick_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& AnswerChoice::probabilities() const {
        auto stored = std::atomic_load(cached_probabilities_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, double>>>>(detail::member_value<std::vector<std::pair<std::string,
            double>>>(value_, "probabilities"));
            if (std::atomic_compare_exchange_strong(cached_probabilities_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& AnswerScore::confidence() const {
        auto stored = std::atomic_load(cached_confidence_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "confidence"));
            if (std::atomic_compare_exchange_strong(cached_confidence_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnswerScore::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnswerScore::level() const {
        auto stored = std::atomic_load(cached_level_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "level"));
            if (std::atomic_compare_exchange_strong(cached_level_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& AnswerScore::probabilities() const {
        auto stored = std::atomic_load(cached_probabilities_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, double>>>>(detail::member_value<std::vector<std::pair<std::string,
            double>>>(value_, "probabilities"));
            if (std::atomic_compare_exchange_strong(cached_probabilities_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnswerTag::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& AnswerTag::probabilities() const {
        auto stored = std::atomic_load(cached_probabilities_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, double>>>>(detail::member_value<std::vector<std::pair<std::string,
            double>>>(value_, "probabilities"));
            if (std::atomic_compare_exchange_strong(cached_probabilities_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& AnswerYesNo::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& AnswerYesNo::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& AttemptOutcome::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::variant<uint64_t, std::string>& BatchSetting::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::variant<uint64_t, std::string>>(detail::decode<std::variant<uint64_t,
            std::string>>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<BatchSetting>& BatchWarning::running() const {
        auto stored = std::atomic_load(cached_running_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<BatchSetting>>(detail::member_value<BatchSetting>(value_,
            "running"));
            if (std::atomic_compare_exchange_strong(cached_running_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<BatchSetting>& BatchWarning::tuned_for() const {
        auto stored = std::atomic_load(cached_tuned_for_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<BatchSetting>>(detail::member_value<BatchSetting>(value_,
            "tuned_for"));
            if (std::atomic_compare_exchange_strong(cached_tuned_for_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Entity::end() const {
        auto stored = std::atomic_load(cached_end_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "end"));
            if (std::atomic_compare_exchange_strong(cached_end_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Entity::file() const {
        auto stored = std::atomic_load(cached_file_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "file"));
            if (std::atomic_compare_exchange_strong(cached_file_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Entity::first_line() const {
        auto stored = std::atomic_load(cached_first_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "first_line"));
            if (std::atomic_compare_exchange_strong(cached_first_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Entity::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Entity::last_line() const {
        auto stored = std::atomic_load(cached_last_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_line"));
            if (std::atomic_compare_exchange_strong(cached_last_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Entity::length() const {
        auto stored = std::atomic_load(cached_length_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "length"));
            if (std::atomic_compare_exchange_strong(cached_length_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Entity::start() const {
        auto stored = std::atomic_load(cached_start_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "start"));
            if (std::atomic_compare_exchange_strong(cached_start_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& Entity::strength() const {
        auto stored = std::atomic_load(cached_strength_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "strength"));
            if (std::atomic_compare_exchange_strong(cached_strength_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Entity::text() const {
        auto stored = std::atomic_load(cached_text_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "text"));
            if (std::atomic_compare_exchange_strong(cached_text_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& EntityEdge::either() const {
        auto stored = std::atomic_load(cached_either_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "either"));
            if (std::atomic_compare_exchange_strong(cached_either_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& EntityEdge::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& EntityEdge::relation() const {
        auto stored = std::atomic_load(cached_relation_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "relation"));
            if (std::atomic_compare_exchange_strong(cached_relation_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Entity>& EntityEdge::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Entity>>(detail::member_value<Entity>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Entity>& EntityEdge::target() const {
        auto stored = std::atomic_load(cached_target_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Entity>>(detail::member_value<Entity>(value_,
            "target"));
            if (std::atomic_compare_exchange_strong(cached_target_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Failure>& Failed::failed() const {
        auto stored = std::atomic_load(cached_failed_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
            "failed"));
            if (std::atomic_compare_exchange_strong(cached_failed_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<FailureCause>& Failure::cause() const {
        auto stored = std::atomic_load(cached_cause_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<FailureCause>>(detail::member_value<FailureCause>(value_,
            "cause"));
            if (std::atomic_compare_exchange_strong(cached_cause_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Failure::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& FailureCause::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::string& FailureKind::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::string>(detail::decode<std::string>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& FindAnswer::confidence() const {
        auto stored = std::atomic_load(cached_confidence_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "confidence"));
            if (std::atomic_compare_exchange_strong(cached_confidence_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& FindAnswer::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& FindAnswer::pick() const {
        auto stored = std::atomic_load(cached_pick_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "pick"));
            if (std::atomic_compare_exchange_strong(cached_pick_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& FindAnswer::probabilities() const {
        auto stored = std::atomic_load(cached_probabilities_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, double>>>>(detail::member_value<std::vector<std::pair<std::string,
            double>>>(value_, "probabilities"));
            if (std::atomic_compare_exchange_strong(cached_probabilities_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Batch>& FindQuestionDocument::batch() const {
        auto stored = std::atomic_load(cached_batch_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
            "batch"));
            if (std::atomic_compare_exchange_strong(cached_batch_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& FindQuestionDocument::context_schema() const {
        auto stored = std::atomic_load(cached_context_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "context_schema"));
            if (std::atomic_compare_exchange_strong(cached_context_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& FindQuestionDocument::item_schema() const {
        auto stored = std::atomic_load(cached_item_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "item_schema"));
            if (std::atomic_compare_exchange_strong(cached_item_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Label>>& FindQuestionDocument::label_details() const {
        auto stored = std::atomic_load(cached_label_details_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
            "label_details"));
            if (std::atomic_compare_exchange_strong(cached_label_details_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& FindQuestionDocument::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionName>& FindQuestionDocument::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& FindQuestionDocument::none() const {
        auto stored = std::atomic_load(cached_none_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "none"));
            if (std::atomic_compare_exchange_strong(cached_none_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& FindQuestionDocument::on() const {
        auto stored = std::atomic_load(cached_on_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "on"));
            if (std::atomic_compare_exchange_strong(cached_on_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& FindQuestionDocument::profile() const {
        auto stored = std::atomic_load(cached_profile_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "profile"));
            if (std::atomic_compare_exchange_strong(cached_profile_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& FindQuestionDocument::text() const {
        auto stored = std::atomic_load(cached_text_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "text"));
            if (std::atomic_compare_exchange_strong(cached_text_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& FindQuestionDocument::verb() const {
        auto stored = std::atomic_load(cached_verb_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "verb"));
            if (std::atomic_compare_exchange_strong(cached_verb_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<WordingVersion>& FindQuestionDocument::wording_version() const {
        auto stored = std::atomic_load(cached_wording_version_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
            "wording_version"));
            if (std::atomic_compare_exchange_strong(cached_wording_version_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& NameOdds::edges() const {
        auto stored = std::atomic_load(cached_edges_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, double>>>>(detail::member_value<std::vector<std::pair<std::string,
            double>>>(value_, "edges"));
            if (std::atomic_compare_exchange_strong(cached_edges_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& NameOdds::end() const {
        auto stored = std::atomic_load(cached_end_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "end"));
            if (std::atomic_compare_exchange_strong(cached_end_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& NameOdds::kinds() const {
        auto stored = std::atomic_load(cached_kinds_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, double>>>>(detail::member_value<std::vector<std::pair<std::string,
            double>>>(value_, "kinds"));
            if (std::atomic_compare_exchange_strong(cached_kinds_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& NameOdds::start() const {
        auto stored = std::atomic_load(cached_start_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "start"));
            if (std::atomic_compare_exchange_strong(cached_start_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& PairOdds::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& PairOdds::relation() const {
        auto stored = std::atomic_load(cached_relation_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "relation"));
            if (std::atomic_compare_exchange_strong(cached_relation_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Place>& PairOdds::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Place>>(detail::member_value<Place>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Place>& PairOdds::target() const {
        auto stored = std::atomic_load(cached_target_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Place>>(detail::member_value<Place>(value_,
            "target"));
            if (std::atomic_compare_exchange_strong(cached_target_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& PieceOdds::end() const {
        auto stored = std::atomic_load(cached_end_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "end"));
            if (std::atomic_compare_exchange_strong(cached_end_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& PieceOdds::start() const {
        auto stored = std::atomic_load(cached_start_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "start"));
            if (std::atomic_compare_exchange_strong(cached_start_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, double>>>& PieceOdds::tags() const {
        auto stored = std::atomic_load(cached_tags_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, double>>>>(detail::member_value<std::vector<std::pair<std::string,
            double>>>(value_, "tags"));
            if (std::atomic_compare_exchange_strong(cached_tags_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Place::end() const {
        auto stored = std::atomic_load(cached_end_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "end"));
            if (std::atomic_compare_exchange_strong(cached_end_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Place::start() const {
        auto stored = std::atomic_load(cached_start_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "start"));
            if (std::atomic_compare_exchange_strong(cached_start_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Plan::estimated_bytes() const {
        auto stored = std::atomic_load(cached_estimated_bytes_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "estimated_bytes"));
            if (std::atomic_compare_exchange_strong(cached_estimated_bytes_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<TokenBand>& Plan::estimated_input_tokens() const {
        auto stored = std::atomic_load(cached_estimated_input_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<TokenBand>>(detail::member_value<TokenBand>(value_,
            "estimated_input_tokens"));
            if (std::atomic_compare_exchange_strong(cached_estimated_input_tokens_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Plan::first_body_utf8() const {
        auto stored = std::atomic_load(cached_first_body_utf8_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "first_body_utf8"));
            if (std::atomic_compare_exchange_strong(cached_first_body_utf8_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Plan::largest_request_bytes() const {
        auto stored = std::atomic_load(cached_largest_request_bytes_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "largest_request_bytes"));
            if (std::atomic_compare_exchange_strong(cached_largest_request_bytes_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Plan::largest_request_estimated_input_tokens() const {
        auto stored = std::atomic_load(cached_largest_request_estimated_input_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "largest_request_estimated_input_tokens"));
            if (std::atomic_compare_exchange_strong(cached_largest_request_estimated_input_tokens_.get(),
            &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Plan::records() const {
        auto stored = std::atomic_load(cached_records_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "records"));
            if (std::atomic_compare_exchange_strong(cached_records_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& Plan::requests() const {
        auto stored = std::atomic_load(cached_requests_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "requests"));
            if (std::atomic_compare_exchange_strong(cached_requests_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& Plan::token_estimate_method() const {
        auto stored = std::atomic_load(cached_token_estimate_method_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "token_estimate_method"));
            if (std::atomic_compare_exchange_strong(cached_token_estimate_method_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& Plan::upper_bound() const {
        auto stored = std::atomic_load(cached_upper_bound_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "upper_bound"));
            if (std::atomic_compare_exchange_strong(cached_upper_bound_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& ProfileWarning::running() const {
        auto stored = std::atomic_load(cached_running_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "running"));
            if (std::atomic_compare_exchange_strong(cached_running_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& ProfileWarning::tuned_for() const {
        auto stored = std::atomic_load(cached_tuned_for_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "tuned_for"));
            if (std::atomic_compare_exchange_strong(cached_tuned_for_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline std::optional<QuestionDocumentDecide> QuestionDocument::as_QuestionDocumentDecide() const {
        if (detail::literal(value_, "verb", "decide")) return QuestionDocumentDecide(value_);
        return std::nullopt;
    }
    inline std::optional<QuestionDocumentChoose> QuestionDocument::as_QuestionDocumentChoose() const {
        if (detail::literal(value_, "verb", "choose")) return QuestionDocumentChoose(value_);
        return std::nullopt;
    }
    inline std::optional<QuestionDocumentTag> QuestionDocument::as_QuestionDocumentTag() const {
        if (detail::literal(value_, "verb", "tag")) return QuestionDocumentTag(value_);
        return std::nullopt;
    }
    inline std::optional<QuestionDocumentScore> QuestionDocument::as_QuestionDocumentScore() const {
        if (detail::literal(value_, "verb", "score")) return QuestionDocumentScore(value_);
        return std::nullopt;
    }
    inline const Presence<Batch>& QuestionDocumentChoose::batch() const {
        auto stored = std::atomic_load(cached_batch_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
            "batch"));
            if (std::atomic_compare_exchange_strong(cached_batch_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& QuestionDocumentChoose::context_schema() const {
        auto stored = std::atomic_load(cached_context_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "context_schema"));
            if (std::atomic_compare_exchange_strong(cached_context_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& QuestionDocumentChoose::item_schema() const {
        auto stored = std::atomic_load(cached_item_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "item_schema"));
            if (std::atomic_compare_exchange_strong(cached_item_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Label>>& QuestionDocumentChoose::label_details() const {
        auto stored = std::atomic_load(cached_label_details_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
            "label_details"));
            if (std::atomic_compare_exchange_strong(cached_label_details_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentChoose::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionName>& QuestionDocumentChoose::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& QuestionDocumentChoose::on() const {
        auto stored = std::atomic_load(cached_on_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "on"));
            if (std::atomic_compare_exchange_strong(cached_on_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentChoose::profile() const {
        auto stored = std::atomic_load(cached_profile_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "profile"));
            if (std::atomic_compare_exchange_strong(cached_profile_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<WordingVersion>& QuestionDocumentChoose::wording_version() const {
        auto stored = std::atomic_load(cached_wording_version_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
            "wording_version"));
            if (std::atomic_compare_exchange_strong(cached_wording_version_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& QuestionDocumentChoose::options() const {
        auto stored = std::atomic_load(cached_options_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "options"));
            if (std::atomic_compare_exchange_strong(cached_options_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& QuestionDocumentChoose::text() const {
        auto stored = std::atomic_load(cached_text_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "text"));
            if (std::atomic_compare_exchange_strong(cached_text_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentChoose::verb() const {
        auto stored = std::atomic_load(cached_verb_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "verb"));
            if (std::atomic_compare_exchange_strong(cached_verb_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Batch>& QuestionDocumentDecide::batch() const {
        auto stored = std::atomic_load(cached_batch_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
            "batch"));
            if (std::atomic_compare_exchange_strong(cached_batch_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& QuestionDocumentDecide::context_schema() const {
        auto stored = std::atomic_load(cached_context_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "context_schema"));
            if (std::atomic_compare_exchange_strong(cached_context_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& QuestionDocumentDecide::item_schema() const {
        auto stored = std::atomic_load(cached_item_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "item_schema"));
            if (std::atomic_compare_exchange_strong(cached_item_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Label>>& QuestionDocumentDecide::label_details() const {
        auto stored = std::atomic_load(cached_label_details_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
            "label_details"));
            if (std::atomic_compare_exchange_strong(cached_label_details_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentDecide::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionName>& QuestionDocumentDecide::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& QuestionDocumentDecide::on() const {
        auto stored = std::atomic_load(cached_on_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "on"));
            if (std::atomic_compare_exchange_strong(cached_on_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentDecide::profile() const {
        auto stored = std::atomic_load(cached_profile_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "profile"));
            if (std::atomic_compare_exchange_strong(cached_profile_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<WordingVersion>& QuestionDocumentDecide::wording_version() const {
        auto stored = std::atomic_load(cached_wording_version_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
            "wording_version"));
            if (std::atomic_compare_exchange_strong(cached_wording_version_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& QuestionDocumentDecide::false_() const {
        auto stored = std::atomic_load(cached_false__.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "false"));
            if (std::atomic_compare_exchange_strong(cached_false__.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& QuestionDocumentDecide::text() const {
        auto stored = std::atomic_load(cached_text_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "text"));
            if (std::atomic_compare_exchange_strong(cached_text_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& QuestionDocumentDecide::true_() const {
        auto stored = std::atomic_load(cached_true__.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "true"));
            if (std::atomic_compare_exchange_strong(cached_true__.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentDecide::verb() const {
        auto stored = std::atomic_load(cached_verb_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "verb"));
            if (std::atomic_compare_exchange_strong(cached_verb_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Batch>& QuestionDocumentScore::batch() const {
        auto stored = std::atomic_load(cached_batch_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
            "batch"));
            if (std::atomic_compare_exchange_strong(cached_batch_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& QuestionDocumentScore::context_schema() const {
        auto stored = std::atomic_load(cached_context_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "context_schema"));
            if (std::atomic_compare_exchange_strong(cached_context_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& QuestionDocumentScore::item_schema() const {
        auto stored = std::atomic_load(cached_item_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "item_schema"));
            if (std::atomic_compare_exchange_strong(cached_item_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Label>>& QuestionDocumentScore::label_details() const {
        auto stored = std::atomic_load(cached_label_details_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
            "label_details"));
            if (std::atomic_compare_exchange_strong(cached_label_details_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentScore::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionName>& QuestionDocumentScore::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& QuestionDocumentScore::on() const {
        auto stored = std::atomic_load(cached_on_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "on"));
            if (std::atomic_compare_exchange_strong(cached_on_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentScore::profile() const {
        auto stored = std::atomic_load(cached_profile_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "profile"));
            if (std::atomic_compare_exchange_strong(cached_profile_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<WordingVersion>& QuestionDocumentScore::wording_version() const {
        auto stored = std::atomic_load(cached_wording_version_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
            "wording_version"));
            if (std::atomic_compare_exchange_strong(cached_wording_version_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& QuestionDocumentScore::levels() const {
        auto stored = std::atomic_load(cached_levels_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "levels"));
            if (std::atomic_compare_exchange_strong(cached_levels_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& QuestionDocumentScore::text() const {
        auto stored = std::atomic_load(cached_text_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "text"));
            if (std::atomic_compare_exchange_strong(cached_text_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentScore::verb() const {
        auto stored = std::atomic_load(cached_verb_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "verb"));
            if (std::atomic_compare_exchange_strong(cached_verb_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Batch>& QuestionDocumentTag::batch() const {
        auto stored = std::atomic_load(cached_batch_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
            "batch"));
            if (std::atomic_compare_exchange_strong(cached_batch_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& QuestionDocumentTag::context_schema() const {
        auto stored = std::atomic_load(cached_context_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "context_schema"));
            if (std::atomic_compare_exchange_strong(cached_context_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& QuestionDocumentTag::item_schema() const {
        auto stored = std::atomic_load(cached_item_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "item_schema"));
            if (std::atomic_compare_exchange_strong(cached_item_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Label>>& QuestionDocumentTag::label_details() const {
        auto stored = std::atomic_load(cached_label_details_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
            "label_details"));
            if (std::atomic_compare_exchange_strong(cached_label_details_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentTag::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionName>& QuestionDocumentTag::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& QuestionDocumentTag::on() const {
        auto stored = std::atomic_load(cached_on_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "on"));
            if (std::atomic_compare_exchange_strong(cached_on_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentTag::profile() const {
        auto stored = std::atomic_load(cached_profile_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "profile"));
            if (std::atomic_compare_exchange_strong(cached_profile_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<WordingVersion>& QuestionDocumentTag::wording_version() const {
        auto stored = std::atomic_load(cached_wording_version_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
            "wording_version"));
            if (std::atomic_compare_exchange_strong(cached_wording_version_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& QuestionDocumentTag::labels() const {
        auto stored = std::atomic_load(cached_labels_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "labels"));
            if (std::atomic_compare_exchange_strong(cached_labels_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& QuestionDocumentTag::text() const {
        auto stored = std::atomic_load(cached_text_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "text"));
            if (std::atomic_compare_exchange_strong(cached_text_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& QuestionDocumentTag::verb() const {
        auto stored = std::atomic_load(cached_verb_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "verb"));
            if (std::atomic_compare_exchange_strong(cached_verb_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_names_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<NameOdds>>>(detail::member_value<std::vector<NameOdds>>(value_,
            "names"));
            if (std::atomic_compare_exchange_strong(cached_names_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<PairOdds>>& RecognizeAnswer::pairs() const {
        auto stored = std::atomic_load(cached_pairs_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<PairOdds>>>(detail::member_value<std::vector<PairOdds>>(value_,
            "pairs"));
            if (std::atomic_compare_exchange_strong(cached_pairs_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<PieceOdds>>& RecognizeAnswer::pieces() const {
        auto stored = std::atomic_load(cached_pieces_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<PieceOdds>>>(detail::member_value<std::vector<PieceOdds>>(value_,
            "pieces"));
            if (std::atomic_compare_exchange_strong(cached_pieces_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RecognitionProposal>>& RecognizeAnswer::proposals() const {
        auto stored = std::atomic_load(cached_proposals_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RecognitionProposal>>>(detail::member_value<std::vector<RecognitionProposal>>(value_,
            "proposals"));
            if (std::atomic_compare_exchange_strong(cached_proposals_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Batch>& RecognizeQuestionDocument::batch() const {
        auto stored = std::atomic_load(cached_batch_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
            "batch"));
            if (std::atomic_compare_exchange_strong(cached_batch_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& RecognizeQuestionDocument::context_schema() const {
        auto stored = std::atomic_load(cached_context_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "context_schema"));
            if (std::atomic_compare_exchange_strong(cached_context_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& RecognizeQuestionDocument::entity_definition() const {
        auto stored = std::atomic_load(cached_entity_definition_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "entity_definition"));
            if (std::atomic_compare_exchange_strong(cached_entity_definition_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& RecognizeQuestionDocument::instructions() const {
        auto stored = std::atomic_load(cached_instructions_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "instructions"));
            if (std::atomic_compare_exchange_strong(cached_instructions_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& RecognizeQuestionDocument::item_schema() const {
        auto stored = std::atomic_load(cached_item_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "item_schema"));
            if (std::atomic_compare_exchange_strong(cached_item_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::pair<std::string, Data>>>& RecognizeQuestionDocument::kinds() const {
        auto stored = std::atomic_load(cached_kinds_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::pair<std::string, Data>>>>(detail::member_value<std::vector<std::pair<std::string,
            Data>>>(value_, "kinds"));
            if (std::atomic_compare_exchange_strong(cached_kinds_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Label>>& RecognizeQuestionDocument::label_details() const {
        auto stored = std::atomic_load(cached_label_details_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
            "label_details"));
            if (std::atomic_compare_exchange_strong(cached_label_details_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RecognitionMode>& RecognizeQuestionDocument::mode() const {
        auto stored = std::atomic_load(cached_mode_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RecognitionMode>>(detail::member_value<RecognitionMode>(value_,
            "mode"));
            if (std::atomic_compare_exchange_strong(cached_mode_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RecognizeQuestionDocument::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionName>& RecognizeQuestionDocument::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& RecognizeQuestionDocument::on() const {
        auto stored = std::atomic_load(cached_on_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "on"));
            if (std::atomic_compare_exchange_strong(cached_on_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RecognizeQuestionDocument::profile() const {
        auto stored = std::atomic_load(cached_profile_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "profile"));
            if (std::atomic_compare_exchange_strong(cached_profile_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& RecognizeQuestionDocument::relation_threshold() const {
        auto stored = std::atomic_load(cached_relation_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "relation_threshold"));
            if (std::atomic_compare_exchange_strong(cached_relation_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RelationRule>>& RecognizeQuestionDocument::relations() const {
        auto stored = std::atomic_load(cached_relations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RelationRule>>>(detail::member_value<std::vector<RelationRule>>(value_,
            "relations"));
            if (std::atomic_compare_exchange_strong(cached_relations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint32_t>& RecognizeQuestionDocument::snippet_pieces() const {
        auto stored = std::atomic_load(cached_snippet_pieces_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint32_t>>(detail::member_value<uint32_t>(value_,
            "snippet_pieces"));
            if (std::atomic_compare_exchange_strong(cached_snippet_pieces_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RecognitionStageContext>& RecognizeQuestionDocument::stage_context() const {
        auto stored = std::atomic_load(cached_stage_context_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RecognitionStageContext>>(detail::member_value<RecognitionStageContext>(value_,
            "stage_context"));
            if (std::atomic_compare_exchange_strong(cached_stage_context_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& RecognizeQuestionDocument::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Verb>& RecognizeQuestionDocument::verb() const {
        auto stored = std::atomic_load(cached_verb_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Verb>>(detail::member_value<Verb>(value_,
            "verb"));
            if (std::atomic_compare_exchange_strong(cached_verb_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<WordingVersion>& RecognizeQuestionDocument::wording_version() const {
        auto stored = std::atomic_load(cached_wording_version_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
            "wording_version"));
            if (std::atomic_compare_exchange_strong(cached_wording_version_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Entity>>& RecognizeEntities::entities() const {
        auto stored = std::atomic_load(cached_entities_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Entity>>>(detail::member_value<std::vector<Entity>>(value_,
            "entities"));
            if (std::atomic_compare_exchange_strong(cached_entities_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<EntityEdge>>& RecognizeEntities::relations() const {
        auto stored = std::atomic_load(cached_relations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<EntityEdge>>>(detail::member_value<std::vector<EntityEdge>>(value_,
            "relations"));
            if (std::atomic_compare_exchange_strong(cached_relations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<BoundaryMode>& RecognizeModeProposals::mode() const {
        auto stored = std::atomic_load(cached_mode_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<BoundaryMode>>(detail::member_value<BoundaryMode>(value_,
            "mode"));
            if (std::atomic_compare_exchange_strong(cached_mode_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<BoundaryProposal>>& RecognizeModeProposals::proposals() const {
        auto stored = std::atomic_load(cached_proposals_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<BoundaryProposal>>>(detail::member_value<std::vector<BoundaryProposal>>(value_,
            "proposals"));
            if (std::atomic_compare_exchange_strong(cached_proposals_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelateFields::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelateFields::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Batch>& RelateQuestionDocument::batch() const {
        auto stored = std::atomic_load(cached_batch_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Batch>>(detail::member_value<Batch>(value_,
            "batch"));
            if (std::atomic_compare_exchange_strong(cached_batch_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& RelateQuestionDocument::context_schema() const {
        auto stored = std::atomic_load(cached_context_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "context_schema"));
            if (std::atomic_compare_exchange_strong(cached_context_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelateFields>& RelateQuestionDocument::fields() const {
        auto stored = std::atomic_load(cached_fields_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelateFields>>(detail::member_value<RelateFields>(value_,
            "fields"));
            if (std::atomic_compare_exchange_strong(cached_fields_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<InputDeclaration>& RelateQuestionDocument::item_schema() const {
        auto stored = std::atomic_load(cached_item_schema_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<InputDeclaration>>(detail::member_value<InputDeclaration>(value_,
            "item_schema"));
            if (std::atomic_compare_exchange_strong(cached_item_schema_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Label>>& RelateQuestionDocument::label_details() const {
        auto stored = std::atomic_load(cached_label_details_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Label>>>(detail::member_value<std::vector<Label>>(value_,
            "label_details"));
            if (std::atomic_compare_exchange_strong(cached_label_details_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelateQuestionDocument::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionName>& RelateQuestionDocument::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionName>>(detail::member_value<QuestionName>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& RelateQuestionDocument::on() const {
        auto stored = std::atomic_load(cached_on_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "on"));
            if (std::atomic_compare_exchange_strong(cached_on_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelateQuestionDocument::profile() const {
        auto stored = std::atomic_load(cached_profile_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "profile"));
            if (std::atomic_compare_exchange_strong(cached_profile_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RelationRule>>& RelateQuestionDocument::relations() const {
        auto stored = std::atomic_load(cached_relations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RelationRule>>>(detail::member_value<std::vector<RelationRule>>(value_,
            "relations"));
            if (std::atomic_compare_exchange_strong(cached_relations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& RelateQuestionDocument::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelateQuestionDocument::verb() const {
        auto stored = std::atomic_load(cached_verb_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "verb"));
            if (std::atomic_compare_exchange_strong(cached_verb_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<WordingVersion>& RelateQuestionDocument::wording_version() const {
        auto stored = std::atomic_load(cached_wording_version_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<WordingVersion>>(detail::member_value<WordingVersion>(value_,
            "wording_version"));
            if (std::atomic_compare_exchange_strong(cached_wording_version_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelatedEntity::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelatedEntity::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& RelatedEntityEdge::either() const {
        auto stored = std::atomic_load(cached_either_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "either"));
            if (std::atomic_compare_exchange_strong(cached_either_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& RelatedEntityEdge::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelatedEntityEdge::relation() const {
        auto stored = std::atomic_load(cached_relation_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "relation"));
            if (std::atomic_compare_exchange_strong(cached_relation_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelationEndpoint>& RelatedEntityEdge::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelationEndpoint>>(detail::member_value<RelationEndpoint>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RelationEndpoint>& RelatedEntityEdge::target() const {
        auto stored = std::atomic_load(cached_target_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RelationEndpoint>>(detail::member_value<RelationEndpoint>(value_,
            "target"));
            if (std::atomic_compare_exchange_strong(cached_target_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline std::string RelatedEntityEdge::source_name() const {
        return value_.at("source").at("name").get<std::string>();
    }
    inline std::string RelatedEntityEdge::target_name() const {
        return value_.at("target").at("name").get<std::string>();
    }
    inline const Presence<bool>& RelationRule::either() const {
        auto stored = std::atomic_load(cached_either_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "either"));
            if (std::atomic_compare_exchange_strong(cached_either_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationRule::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationRule::reads() const {
        auto stored = std::atomic_load(cached_reads_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "reads"));
            if (std::atomic_compare_exchange_strong(cached_reads_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& RelationRule::single() const {
        auto stored = std::atomic_load(cached_single_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "single"));
            if (std::atomic_compare_exchange_strong(cached_single_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationRule::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& RelationRule::target() const {
        auto stored = std::atomic_load(cached_target_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "target"));
            if (std::atomic_compare_exchange_strong(cached_target_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionAnnotation::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnnotationValue>& SessionAnnotation::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnnotationValue>>(detail::member_value<AnnotationValue>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SessionInputSource::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& SessionInputSource::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionJudgmentChoice::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionJudgmentDecision::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& SessionJudgmentDecision::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionJudgmentScore::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& SessionJudgmentScore::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionJudgmentTags::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& SessionJudgmentTags::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionNamedProbability::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& SessionNamedProbability::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_detail_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<SessionQuestionDetail>>(detail::member_value<SessionQuestionDetail>(value_,
            "detail"));
            if (std::atomic_compare_exchange_strong(cached_detail_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SessionObservationQuestion::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionObservationQuestion::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionObservationQuestion::member() const {
        auto stored = std::atomic_load(cached_member_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "member"));
            if (std::atomic_compare_exchange_strong(cached_member_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SessionObservationQuestion::position() const {
        auto stored = std::atomic_load(cached_position_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "position"));
            if (std::atomic_compare_exchange_strong(cached_position_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionObservationQuestion::stage() const {
        auto stored = std::atomic_load(cached_stage_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "stage"));
            if (std::atomic_compare_exchange_strong(cached_stage_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SessionObservationRow::index() const {
        auto stored = std::atomic_load(cached_index_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "index"));
            if (std::atomic_compare_exchange_strong(cached_index_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionObservationRow::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<SessionObservedRow>& SessionObservationRow::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<SessionObservedRow>>(detail::member_value<SessionObservedRow>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<SessionAnnotation>>& SessionObservedRowAnnotated::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<SessionAnnotation>>>(detail::member_value<std::vector<SessionAnnotation>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionObservedRowFind::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SessionObservedRowFind::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionObservedRowJudgment::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<SessionJudgment>& SessionObservedRowJudgment::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<SessionJudgment>>(detail::member_value<SessionJudgment>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionObservedRowRecognized::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<SessionRecognition>& SessionObservedRowRecognized::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<SessionRecognition>>(detail::member_value<SessionRecognition>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionObservedRowRelations::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<SessionRelationEdge>>& SessionObservedRowRelations::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<SessionRelationEdge>>>(detail::member_value<std::vector<SessionRelationEdge>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketAnnotateAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Annotation>>& SessionPacketAnnotateAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Annotation>>>(detail::member_value<std::vector<Annotation>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketAnnotateRow::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketAnnotateRow::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Annotation>& SessionPacketAnnotateRow::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Annotation>>(detail::member_value<Annotation>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketChooseAggregate::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketChooseAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<AtomicNullableString>>& SessionPacketChooseAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<AtomicNullableString>>>(detail::member_value<std::vector<AtomicNullableString>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketChooseRow::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketChooseRow::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AtomicNullableString>& SessionPacketChooseRow::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AtomicNullableString>>(detail::member_value<AtomicNullableString>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketDecideAggregate::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketDecideAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<AtomicDecideValue>>& SessionPacketDecideAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<AtomicDecideValue>>>(detail::member_value<std::vector<AtomicDecideValue>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketDecideRow::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketDecideRow::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AtomicDecideValue>& SessionPacketDecideRow::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AtomicDecideValue>>(detail::member_value<AtomicDecideValue>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketFilterAggregate::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketFilterAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<AtomicBoolean>>& SessionPacketFilterAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<AtomicBoolean>>>(detail::member_value<std::vector<AtomicBoolean>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketFilterRow::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketFilterRow::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AtomicBoolean>& SessionPacketFilterRow::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AtomicBoolean>>(detail::member_value<AtomicBoolean>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketFindAggregate::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketFindAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Find>& SessionPacketFindAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Find>>(detail::member_value<Find>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RequestFunction>& SessionPacketObservation::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RequestFunction>>(detail::member_value<RequestFunction>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketObservation::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<SessionObservation>& SessionPacketObservation::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<SessionObservation>>(detail::member_value<SessionObservation>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketRankAggregate::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketRankAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<AtomicNonZeroUsize>>& SessionPacketRankAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<AtomicNonZeroUsize>>>(detail::member_value<std::vector<AtomicNonZeroUsize>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketRecognizeAggregate::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketRecognizeAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Recognition>>& SessionPacketRecognizeAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Recognition>>>(detail::member_value<std::vector<Recognition>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketRelateAggregate::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketRelateAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Relation>& SessionPacketRelateAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Relation>>(detail::member_value<Relation>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketScoreAggregate::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketScoreAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<AtomicDouble>>& SessionPacketScoreAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<AtomicDouble>>>(detail::member_value<std::vector<AtomicDouble>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketScoreRow::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketScoreRow::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AtomicDouble>& SessionPacketScoreRow::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AtomicDouble>>(detail::member_value<AtomicDouble>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketTagAggregate::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketTagAggregate::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<AtomicArrayOfString>>& SessionPacketTagAggregate::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<AtomicArrayOfString>>>(detail::member_value<std::vector<AtomicArrayOfString>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketTagRow::function() const {
        auto stored = std::atomic_load(cached_function_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "function"));
            if (std::atomic_compare_exchange_strong(cached_function_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketTagRow::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AtomicArrayOfString>& SessionPacketTagRow::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AtomicArrayOfString>>(detail::member_value<AtomicArrayOfString>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Facts>& SessionPacketTerminal::facts() const {
        auto stored = std::atomic_load(cached_facts_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Facts>>(detail::member_value<Facts>(value_,
            "facts"));
            if (std::atomic_compare_exchange_strong(cached_facts_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<CallError>& SessionPacketTerminal::failure() const {
        auto stored = std::atomic_load(cached_failure_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<CallError>>(detail::member_value<CallError>(value_,
            "failure"));
            if (std::atomic_compare_exchange_strong(cached_failure_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionPacketTerminal::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
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
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<SessionNamedProbability>>& SessionProbabilitiesNamed::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<SessionNamedProbability>>>(detail::member_value<std::vector<SessionNamedProbability>>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionProbabilitiesYesNo::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& SessionProbabilitiesYesNo::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<AnswerId>& SessionQuestionDetail::answer_id() const {
        auto stored = std::atomic_load(cached_answer_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<AnswerId>>(detail::member_value<AnswerId>(value_,
            "answer_id"));
            if (std::atomic_compare_exchange_strong(cached_answer_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& SessionQuestionDetail::cached() const {
        auto stored = std::atomic_load(cached_cached_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "cached"));
            if (std::atomic_compare_exchange_strong(cached_cached_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& SessionQuestionDetail::confidence() const {
        auto stored = std::atomic_load(cached_confidence_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "confidence"));
            if (std::atomic_compare_exchange_strong(cached_confidence_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SessionQuestionDetail::failed_questions() const {
        auto stored = std::atomic_load(cached_failed_questions_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "failed_questions"));
            if (std::atomic_compare_exchange_strong(cached_failed_questions_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Failure>& SessionQuestionDetail::failure() const {
        auto stored = std::atomic_load(cached_failure_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Failure>>(detail::member_value<Failure>(value_,
            "failure"));
            if (std::atomic_compare_exchange_strong(cached_failure_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<FailureId>& SessionQuestionDetail::failure_id() const {
        auto stored = std::atomic_load(cached_failure_id_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<FailureId>>(detail::member_value<FailureId>(value_,
            "failure_id"));
            if (std::atomic_compare_exchange_strong(cached_failure_id_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& SessionQuestionDetail::input() const {
        auto stored = std::atomic_load(cached_input_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "input"));
            if (std::atomic_compare_exchange_strong(cached_input_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<PhysicalSource>& SessionQuestionDetail::input_source() const {
        auto stored = std::atomic_load(cached_input_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<PhysicalSource>>(detail::member_value<PhysicalSource>(value_,
            "input_source"));
            if (std::atomic_compare_exchange_strong(cached_input_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<SessionInputSource>>& SessionQuestionDetail::input_sources() const {
        auto stored = std::atomic_load(cached_input_sources_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<SessionInputSource>>>(detail::member_value<std::vector<SessionInputSource>>(value_,
            "input_sources"));
            if (std::atomic_compare_exchange_strong(cached_input_sources_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Data>>& SessionQuestionDetail::inputs() const {
        auto stored = std::atomic_load(cached_inputs_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Data>>>(detail::member_value<std::vector<Data>>(value_,
            "inputs"));
            if (std::atomic_compare_exchange_strong(cached_inputs_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionQuestionDetail::model() const {
        auto stored = std::atomic_load(cached_model_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "model"));
            if (std::atomic_compare_exchange_strong(cached_model_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Observation>>& SessionQuestionDetail::observations() const {
        auto stored = std::atomic_load(cached_observations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Observation>>>(detail::member_value<std::vector<Observation>>(value_,
            "observations"));
            if (std::atomic_compare_exchange_strong(cached_observations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<SessionProbabilities>& SessionQuestionDetail::probabilities() const {
        auto stored = std::atomic_load(cached_probabilities_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<SessionProbabilities>>(detail::member_value<SessionProbabilities>(value_,
            "probabilities"));
            if (std::atomic_compare_exchange_strong(cached_probabilities_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<QuestionDocument>& SessionQuestionDetail::question() const {
        auto stored = std::atomic_load(cached_question_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<QuestionDocument>>(detail::member_value<QuestionDocument>(value_,
            "question"));
            if (std::atomic_compare_exchange_strong(cached_question_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionQuestionDetail::question_sha256() const {
        auto stored = std::atomic_load(cached_question_sha256_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "question_sha256"));
            if (std::atomic_compare_exchange_strong(cached_question_sha256_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<QuestionSource>>& SessionQuestionDetail::question_sources() const {
        auto stored = std::atomic_load(cached_question_sources_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<QuestionSource>>>(detail::member_value<std::vector<QuestionSource>>(value_,
            "question_sources"));
            if (std::atomic_compare_exchange_strong(cached_question_sources_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionQuestionDetail::raw_pick() const {
        auto stored = std::atomic_load(cached_raw_pick_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "raw_pick"));
            if (std::atomic_compare_exchange_strong(cached_raw_pick_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Usage>& SessionQuestionDetail::reported_usage() const {
        auto stored = std::atomic_load(cached_reported_usage_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Usage>>(detail::member_value<Usage>(value_,
            "reported_usage"));
            if (std::atomic_compare_exchange_strong(cached_reported_usage_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<std::string>>& SessionQuestionDetail::requests() const {
        auto stored = std::atomic_load(cached_requests_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<std::string>>>(detail::member_value<std::vector<std::string>>(value_,
            "requests"));
            if (std::atomic_compare_exchange_strong(cached_requests_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SessionQuestionDetail::requests_sent() const {
        auto stored = std::atomic_load(cached_requests_sent_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "requests_sent"));
            if (std::atomic_compare_exchange_strong(cached_requests_sent_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Threshold>& SessionQuestionDetail::threshold() const {
        auto stored = std::atomic_load(cached_threshold_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Threshold>>(detail::member_value<Threshold>(value_,
            "threshold"));
            if (std::atomic_compare_exchange_strong(cached_threshold_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionQuestionDetail::url() const {
        auto stored = std::atomic_load(cached_url_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "url"));
            if (std::atomic_compare_exchange_strong(cached_url_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<TokenUsage>& SessionQuestionDetail::usage() const {
        auto stored = std::atomic_load(cached_usage_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<TokenUsage>>(detail::member_value<TokenUsage>(value_,
            "usage"));
            if (std::atomic_compare_exchange_strong(cached_usage_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Value>& SessionQuestionDetail::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Value>>(detail::member_value<Value>(value_,
            "value"));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<Entity>>& SessionRecognition::entities() const {
        auto stored = std::atomic_load(cached_entities_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<Entity>>>(detail::member_value<std::vector<Entity>>(value_,
            "entities"));
            if (std::atomic_compare_exchange_strong(cached_entities_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<RecognitionMode>& SessionRecognition::mode() const {
        auto stored = std::atomic_load(cached_mode_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<RecognitionMode>>(detail::member_value<RecognitionMode>(value_,
            "mode"));
            if (std::atomic_compare_exchange_strong(cached_mode_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<BoundaryProposal>>& SessionRecognition::proposals() const {
        auto stored = std::atomic_load(cached_proposals_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<BoundaryProposal>>>(detail::member_value<std::vector<BoundaryProposal>>(value_,
            "proposals"));
            if (std::atomic_compare_exchange_strong(cached_proposals_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::vector<RecognitionEdgeDocument>>& SessionRecognition::relations() const {
        auto stored = std::atomic_load(cached_relations_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::vector<RecognitionEdgeDocument>>>(detail::member_value<std::vector<RecognitionEdgeDocument>>(value_,
            "relations"));
            if (std::atomic_compare_exchange_strong(cached_relations_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<bool>& SessionRelationEdge::either() const {
        auto stored = std::atomic_load(cached_either_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<bool>>(detail::member_value<bool>(value_,
            "either"));
            if (std::atomic_compare_exchange_strong(cached_either_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<double>& SessionRelationEdge::probability() const {
        auto stored = std::atomic_load(cached_probability_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<double>>(detail::member_value<double>(value_,
            "probability"));
            if (std::atomic_compare_exchange_strong(cached_probability_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SessionRelationEdge::relation() const {
        auto stored = std::atomic_load(cached_relation_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "relation"));
            if (std::atomic_compare_exchange_strong(cached_relation_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<EntityDocument>& SessionRelationEdge::source() const {
        auto stored = std::atomic_load(cached_source_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<EntityDocument>>(detail::member_value<EntityDocument>(value_,
            "source"));
            if (std::atomic_compare_exchange_strong(cached_source_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<EntityDocument>& SessionRelationEdge::target() const {
        auto stored = std::atomic_load(cached_target_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<EntityDocument>>(detail::member_value<EntityDocument>(value_,
            "target"));
            if (std::atomic_compare_exchange_strong(cached_target_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline std::string SessionRelationEdge::source_name() const {
        return value_.at("source").at("name").get<std::string>();
    }
    inline std::string SessionRelationEdge::target_name() const {
        return value_.at("target").at("name").get<std::string>();
    }
    inline const Presence<std::string>& SourceRelationEndpoint::file() const {
        auto stored = std::atomic_load(cached_file_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "file"));
            if (std::atomic_compare_exchange_strong(cached_file_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SourceRelationEndpoint::first_line() const {
        auto stored = std::atomic_load(cached_first_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "first_line"));
            if (std::atomic_compare_exchange_strong(cached_first_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SourceRelationEndpoint::kind() const {
        auto stored = std::atomic_load(cached_kind_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "kind"));
            if (std::atomic_compare_exchange_strong(cached_kind_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SourceRelationEndpoint::last_line() const {
        auto stored = std::atomic_load(cached_last_line_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "last_line"));
            if (std::atomic_compare_exchange_strong(cached_last_line_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<std::string>& SourceRelationEndpoint::name() const {
        auto stored = std::atomic_load(cached_name_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<std::string>>(detail::member_value<std::string>(value_,
            "name"));
            if (std::atomic_compare_exchange_strong(cached_name_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& SourceRelationEndpoint::ordinal() const {
        auto stored = std::atomic_load(cached_ordinal_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "ordinal"));
            if (std::atomic_compare_exchange_strong(cached_ordinal_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<Data>& SourceRelationEndpoint::record() const {
        auto stored = std::atomic_load(cached_record_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<Data>>(detail::member_value<Data>(value_,
            "record"));
            if (std::atomic_compare_exchange_strong(cached_record_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::variant<double, std::string>& Threshold::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::variant<double, std::string>>(detail::decode<std::variant<double,
            std::string>>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& TokenBand::lower() const {
        auto stored = std::atomic_load(cached_lower_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "lower"));
            if (std::atomic_compare_exchange_strong(cached_lower_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& TokenBand::upper() const {
        auto stored = std::atomic_load(cached_upper_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "upper"));
            if (std::atomic_compare_exchange_strong(cached_upper_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& TokenUsage::input_tokens() const {
        auto stored = std::atomic_load(cached_input_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "input_tokens"));
            if (std::atomic_compare_exchange_strong(cached_input_tokens_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const Presence<uint64_t>& TokenUsage::output_tokens() const {
        auto stored = std::atomic_load(cached_output_tokens_.get());
        if (!stored) {
            auto candidate = std::make_shared<const Presence<uint64_t>>(detail::member_value<uint64_t>(value_,
            "output_tokens"));
            if (std::atomic_compare_exchange_strong(cached_output_tokens_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
    inline const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>, double>& Value::value() const {
        auto stored = std::atomic_load(cached_value_.get());
        if (!stored) {
            auto candidate = std::make_shared<const std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>,
            double>>(detail::decode<std::variant<bool, std::nullptr_t, std::string, std::vector<std::string>,
            double>>(value_));
            if (std::atomic_compare_exchange_strong(cached_value_.get(), &stored, candidate)) stored = std::move(candidate);
        } return *stored;
    }
}
namespace tt {
    class Failure : public std::runtime_error {
        public:
        NativeFailureKind kind;
        bool retryable=false;
        std::optional<results::ImmediateFacts> facts;
        std::optional<results::CallError> failure;
        std::optional<results::SessionPacketTerminal> terminal;
        std::vector<results::SessionPacket> packets;
        Failure(int code, const std::string& message);
        Failure(const char* message, const std::vector<results::SessionPacket>& values);
        explicit Failure(results::SessionPacketTerminal value);
    };
}
namespace tt::results {
    inline void successful_answer(const std::vector<SessionPacket>& packets) {
        size_t terminals=0;
        for(const auto& packet:packets) if(detail::literal(::tt::results::detail::document(packet),
        "kind", "terminal")) {
            ++terminals;
            if(::tt::results::detail::document(packet).contains("failure")) throw ::tt::Failure("terminal contains a failure",
            packets);
        }
        if(terminals!=1 || packets.empty() || !detail::literal(::tt::results::detail::document(packets.back()),
        "kind","terminal")) throw ::tt::Failure("expected one final successful terminal",packets);
    }
    inline std::optional<bool> read_decide(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<std::optional<bool>> answer;
        for(const auto& packet:packets) if(detail::literal(::tt::results::detail::document(packet),
        "kind", "observation") && detail::literal(::tt::results::detail::document(packet).at("value"),
        "kind", "row") && detail::literal(::tt::results::detail::document(packet).at("value").at("value"),
        "kind", "judgment")) {
            const auto& judgment=::tt::results::detail::document(packet).at("value").at("value").at("value");
            if(!detail::literal(judgment,"kind","decision")) continue;
            if(answer) throw ::tt::Failure("multiple matching judgments",packets);
            answer.emplace(judgment.at("value").is_null() ? std::optional<bool>{
            } : std::optional<bool>(detail::decode<bool>(judgment.at("value"))));
        }
        if(!answer) throw ::tt::Failure("no matching judgment",packets);
        return std::move(*answer);
    }
    inline std::optional<std::string> read_choose(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<std::optional<std::string>> answer;
        for(const auto& packet:packets) if(detail::literal(::tt::results::detail::document(packet),
        "kind", "observation") && detail::literal(::tt::results::detail::document(packet).at("value"),
        "kind", "row") && detail::literal(::tt::results::detail::document(packet).at("value").at("value"),
        "kind", "judgment")) {
            const auto& judgment=::tt::results::detail::document(packet).at("value").at("value").at("value");
            if(!detail::literal(judgment,"kind","choice")) continue;
            if(answer) throw ::tt::Failure("multiple matching judgments",packets);
            answer.emplace(judgment.at("value").is_null() ? std::optional<std::string>{
            } : std::optional<std::string>(detail::decode<std::string>(judgment.at("value"))));
        }
        if(!answer) throw ::tt::Failure("no matching judgment",packets);
        return std::move(*answer);
    }
    inline double read_score(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<double> answer;
        for(const auto& packet:packets) if(detail::literal(::tt::results::detail::document(packet),
        "kind", "observation") && detail::literal(::tt::results::detail::document(packet).at("value"),
        "kind", "row") && detail::literal(::tt::results::detail::document(packet).at("value").at("value"),
        "kind", "judgment")) {
            const auto& judgment=::tt::results::detail::document(packet).at("value").at("value").at("value");
            if(!detail::literal(judgment,"kind","score")) continue;
            if(answer) throw ::tt::Failure("multiple matching judgments",packets);
            answer.emplace(detail::decode<double>(judgment.at("value")));
        }
        if(!answer) throw ::tt::Failure("no matching judgment",packets);
        return std::move(*answer);
    }
    inline std::vector<std::string> read_tag(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<std::vector<std::string>> answer;
        for(const auto& packet:packets) if(detail::literal(::tt::results::detail::document(packet),
        "kind", "observation") && detail::literal(::tt::results::detail::document(packet).at("value"),
        "kind", "row") && detail::literal(::tt::results::detail::document(packet).at("value").at("value"),
        "kind", "judgment")) {
            const auto& judgment=::tt::results::detail::document(packet).at("value").at("value").at("value");
            if(!detail::literal(judgment,"kind","tags")) continue;
            if(answer) throw ::tt::Failure("multiple matching judgments",packets);
            answer.emplace(detail::decode<std::vector<std::string>>(judgment.at("value")));
        }
        if(!answer) throw ::tt::Failure("no matching judgment",packets);
        return std::move(*answer);
    }
    inline std::vector<Data> read_filter(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::vector<Data> answer;
        for(const auto& packet:packets) {
            if(detail::literal(::tt::results::detail::document(packet),"function","filter")) {
                const auto& value=::tt::results::detail::document(packet).at("value");
                if(detail::literal(::tt::results::detail::document(packet),"kind","row")) answer.push_back(detail::decode<Data>(value.at("input")));
                else if(detail::literal(::tt::results::detail::document(packet),"kind","aggregate")) {
                    if(!answer.empty()) continue;
                    for(const auto& row:value) answer.push_back(detail::decode<Data>(row.at("input")));
                }
            }
        }
        return answer;
    }
    inline std::vector<uint64_t> read_rank(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::vector<uint64_t> answer;
        for(const auto& packet:packets) {
            if(detail::literal(::tt::results::detail::document(packet),"function","rank")) {
                const auto& value=::tt::results::detail::document(packet).at("value");
                if(detail::literal(::tt::results::detail::document(packet),"kind","row")) answer.push_back(detail::decode<uint64_t>(value.at("index")));
                else if(detail::literal(::tt::results::detail::document(packet),"kind","aggregate")) {
                    if(!answer.empty()) continue;
                    for(const auto& row:value) answer.push_back(detail::decode<uint64_t>(row.at("index")));
                }
            }
        }
        return answer;
    }
    inline Find read_find(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<Find> answer;
        for(const auto& packet:packets) if(auto aggregate=packet.as_SessionPacketFindAggregate()) {
            if(answer) throw ::tt::Failure("multiple matching aggregates",packets);
            answer.emplace(*aggregate->value().value);
        }
        if(!answer) throw ::tt::Failure("no matching aggregate",packets);
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
            if(answer) throw ::tt::Failure("multiple matching aggregates",packets);
            answer.emplace(*aggregate->value().value);
        }
        if(!answer) throw ::tt::Failure("no matching aggregate",packets);
        return std::move(*answer);
    }
    inline Relation read_relate(const std::vector<SessionPacket>& packets) {
        successful_answer(packets);
        std::optional<Relation> answer;
        for(const auto& packet:packets) if(auto aggregate=packet.as_SessionPacketRelateAggregate()) {
            if(answer) throw ::tt::Failure("multiple matching aggregates",packets);
            answer.emplace(*aggregate->value().value);
        }
        if(!answer) throw ::tt::Failure("no matching aggregate",packets);
        return std::move(*answer);
    }
}
#ifdef THINKTHEN_CPP_IMPLEMENTATION
#include <thinkthen/thinkthen.h>
namespace tt::results::detail {
    inline Data native_value(const thinkthen_complete_attempt_list_v1& value);
    inline Data native_value(const thinkthen_complete_attempt_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_call_id_presence_v1& value);
    inline Data native_value(const thinkthen_complete_utf8_presence_v1& value);
    inline Data native_value(const thinkthen_complete_u32_presence_v1& value);
    inline Data native_value(const thinkthen_complete_u64_presence_v1& value);
    inline Data native_value(const thinkthen_complete_persistence_observation_presence_v1& value);
    inline Data native_value(const thinkthen_complete_immediate_facts_v1& value);
    inline Data native_value(const thinkthen_complete_persistence_observation_v1& value);
    inline Data native_value(const thinkthen_complete_usage_persistence_v1& value);
    inline Data native_value(const thinkthen_complete_call_id_v1& value);
    inline Data native_value(const thinkthen_complete_attempt_v1& value);
    inline Data native_value(const thinkthen_complete_sdk_request_id_v1& value);
    inline Data native_value(const thinkthen_complete_attempt_outcome_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_v1& value);
    inline Data native_value(const thinkthen_complete_facts_presence_v1& value);
    inline Data native_value(const thinkthen_complete_call_error_presence_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_terminal_v1& value);
    inline Data native_value(const thinkthen_complete_call_error_v1& value);
    inline Data native_value(const thinkthen_complete_facts_v1& value);
    inline Data native_value(const thinkthen_complete_input_denial_presence_v1& value);
    inline Data native_value(const thinkthen_complete_send_denial_presence_v1& value);
    inline Data native_value(const thinkthen_complete_error_v1& value);
    inline Data native_value(const thinkthen_complete_stopped_v1& value);
    inline Data native_value(const thinkthen_complete_stop_cause_v1& value);
    inline Data native_value(const thinkthen_complete_send_denial_v1& value);
    inline Data native_value(const thinkthen_complete_send_denial_before_retry_v1& value);
    inline Data native_value(const thinkthen_complete_send_denial_before_additional_send_v1& value);
    inline Data native_value(const thinkthen_complete_send_denial_before_first_send_v1& value);
    inline Data native_value(const thinkthen_complete_failure_kind_v1& value);
    inline Data native_value(const thinkthen_complete_input_denial_v1& value);
    inline Data native_value(const thinkthen_complete_input_denial_retry_v1& value);
    inline Data native_value(const thinkthen_complete_input_denial_additional_request_v1& value);
    inline Data native_value(const thinkthen_complete_input_denial_initial_request_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_observation_v1& value);
    inline Data native_value(const thinkthen_complete_session_observation_v1& value);
    inline Data native_value(const thinkthen_complete_session_observation_row_v1& value);
    inline Data native_value(const thinkthen_complete_session_observed_row_v1& value);
    inline Data native_value(const thinkthen_complete_session_relation_edge_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_observed_row_relations_v1& value);
    inline Data native_value(const thinkthen_complete_session_relation_edge_v1& value);
    inline Data native_value(const thinkthen_complete_entity_document_v1& value);
    inline Data native_value(const thinkthen_complete_session_observed_row_find_v1& value);
    inline Data native_value(const thinkthen_complete_session_observed_row_recognized_v1& value);
    inline Data native_value(const thinkthen_complete_entity_list_v1& value);
    inline Data native_value(const thinkthen_complete_boundary_proposal_list_v1& value);
    inline Data native_value(const thinkthen_complete_boundary_proposal_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_edge_list_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_edge_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_session_recognition_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_edge_v1& value);
    inline Data native_value(const thinkthen_complete_entity_v1& value);
    inline Data native_value(const thinkthen_complete_boundary_proposal_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_mode_v1& value);
    inline Data native_value(const thinkthen_complete_session_annotation_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_observed_row_annotated_v1& value);
    inline Data native_value(const thinkthen_complete_session_annotation_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_value_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_value_failed_v1& value);
    inline Data native_value(const thinkthen_complete_failure_field_kind_v1& value);
    inline Data native_value(const thinkthen_complete_failure_v1& value);
    inline Data native_value(const thinkthen_complete_failure_cause_v1& value);
    inline Data native_value(const thinkthen_complete_string_list_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_value_tags_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_value_score_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_value_choice_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_value_decision_v1& value);
    inline Data native_value(const thinkthen_complete_session_observed_row_judgment_v1& value);
    inline Data native_value(const thinkthen_complete_session_judgment_v1& value);
    inline Data native_value(const thinkthen_complete_session_judgment_tags_v1& value);
    inline Data native_value(const thinkthen_complete_session_judgment_score_v1& value);
    inline Data native_value(const thinkthen_complete_session_judgment_choice_v1& value);
    inline Data native_value(const thinkthen_complete_session_judgment_decision_v1& value);
    inline Data native_value(const thinkthen_complete_session_observation_question_v1& value);
    inline Data native_value(const thinkthen_complete_answer_id_presence_v1& value);
    inline Data native_value(const thinkthen_complete_f64_presence_v1& value);
    inline Data native_value(const thinkthen_complete_failure_presence_v1& value);
    inline Data native_value(const thinkthen_complete_failure_id_presence_v1& value);
    inline Data native_value(const thinkthen_complete_json_presence_v1& value);
    inline Data native_value(const thinkthen_complete_physical_source_presence_v1& value);
    inline Data native_value(const thinkthen_complete_session_input_source_list_v1& value);
    inline Data native_value(const thinkthen_complete_json_list_v1& value);
    inline Data native_value(const thinkthen_complete_observation_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_probabilities_presence_v1& value);
    inline Data native_value(const thinkthen_complete_question_source_list_v1& value);
    inline Data native_value(const thinkthen_complete_usage_presence_v1& value);
    inline Data native_value(const thinkthen_complete_threshold_presence_v1& value);
    inline Data native_value(const thinkthen_complete_token_usage_presence_v1& value);
    inline Data native_value(const thinkthen_complete_value_presence_v1& value);
    inline Data native_value(const thinkthen_complete_session_question_detail_v1& value);
    inline Data native_value(const thinkthen_complete_value_v1& value);
    inline Data native_value(const thinkthen_complete_token_usage_v1& value);
    inline Data native_value(const thinkthen_complete_threshold_v1& value);
    inline Data native_value(const thinkthen_complete_usage_v1& value);
    inline Data native_value(const thinkthen_complete_question_source_v1& value);
    inline Data native_value(const thinkthen_complete_origin_v1& value);
    inline Data native_value(const thinkthen_complete_question_document_v1& value);
    inline Data native_value(const thinkthen_complete_batch_presence_v1& value);
    inline Data native_value(const thinkthen_complete_input_declaration_presence_v1& value);
    inline Data native_value(const thinkthen_complete_label_list_v1& value);
    inline Data native_value(const thinkthen_complete_label_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_question_name_presence_v1& value);
    inline Data native_value(const thinkthen_complete_string_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_wording_version_presence_v1& value);
    inline Data native_value(const thinkthen_complete_question_document_score_v1& value);
    inline Data native_value(const thinkthen_complete_wording_version_v1& value);
    inline Data native_value(const thinkthen_complete_question_name_v1& value);
    inline Data native_value(const thinkthen_complete_label_v1& value);
    inline Data native_value(const thinkthen_complete_input_declaration_v1& value);
    inline Data native_value(const thinkthen_complete_input_property_type_map_entry_v1& value);
    inline Data native_value(const thinkthen_complete_input_property_type_map_v1& value);
    inline Data native_value(const thinkthen_complete_input_declaration_object_root_v1& value);
    inline Data native_value(const thinkthen_complete_object_type_v1& value);
    inline Data native_value(const thinkthen_complete_input_property_type_v1& value);
    inline Data native_value(const thinkthen_complete_input_property_type_array_v1& value);
    inline Data native_value(const thinkthen_complete_string_root_v1& value);
    inline Data native_value(const thinkthen_complete_string_type_v1& value);
    inline Data native_value(const thinkthen_complete_input_property_type_boolean_v1& value);
    inline Data native_value(const thinkthen_complete_input_property_type_number_v1& value);
    inline Data native_value(const thinkthen_complete_input_property_type_string_v1& value);
    inline Data native_value(const thinkthen_complete_input_declaration_string_root_v1& value);
    inline Data native_value(const thinkthen_complete_batch_string_v1& value);
    inline Data native_value(const thinkthen_complete_batch_v1& value);
    inline Data native_value(const thinkthen_complete_question_document_tag_v1& value);
    inline Data native_value(const thinkthen_complete_question_document_choose_v1& value);
    inline Data native_value(const thinkthen_complete_question_document_decide_v1& value);
    inline Data native_value(const thinkthen_complete_session_probabilities_v1& value);
    inline Data native_value(const thinkthen_complete_session_named_probability_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_probabilities_named_v1& value);
    inline Data native_value(const thinkthen_complete_session_named_probability_v1& value);
    inline Data native_value(const thinkthen_complete_session_probabilities_yes_no_v1& value);
    inline Data native_value(const thinkthen_complete_observation_v1& value);
    inline Data native_value(const thinkthen_complete_observation_failure_id_v1& value);
    inline Data native_value(const thinkthen_complete_failure_id_v1& value);
    inline Data native_value(const thinkthen_complete_observation_observation_id_v1& value);
    inline Data native_value(const thinkthen_complete_observation_id_v1& value);
    inline Data native_value(const thinkthen_complete_session_input_source_v1& value);
    inline Data native_value(const thinkthen_complete_physical_source_v1& value);
    inline Data native_value(const thinkthen_complete_answer_id_v1& value);
    inline Data native_value(const thinkthen_complete_request_function_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_relate_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_session_input_source_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_position_presence_v1& value);
    inline Data native_value(const thinkthen_complete_related_entity_edge_list_v1& value);
    inline Data native_value(const thinkthen_complete_relation_v1& value);
    inline Data native_value(const thinkthen_complete_related_entity_edge_v1& value);
    inline Data native_value(const thinkthen_complete_relation_endpoint_v1& value);
    inline Data native_value(const thinkthen_complete_relation_endpoint_source_v1& value);
    inline Data native_value(const thinkthen_complete_relation_endpoint_related_entity_v1& value);
    inline Data native_value(const thinkthen_complete_version_v1& value);
    inline Data native_value(const thinkthen_complete_relate_fields_presence_v1& value);
    inline Data native_value(const thinkthen_complete_relation_rule_list_v1& value);
    inline Data native_value(const thinkthen_complete_relate_question_document_v1& value);
    inline Data native_value(const thinkthen_complete_relation_rule_v1& value);
    inline Data native_value(const thinkthen_complete_relate_fields_v1& value);
    inline Data native_value(const thinkthen_complete_position_v1& value);
    inline Data native_value(const thinkthen_complete_batch_setting_presence_v1& value);
    inline Data native_value(const thinkthen_complete_batch_warning_presence_v1& value);
    inline Data native_value(const thinkthen_complete_origin_presence_v1& value);
    inline Data native_value(const thinkthen_complete_profile_warning_presence_v1& value);
    inline Data native_value(const thinkthen_complete_meta_v1& value);
    inline Data native_value(const thinkthen_complete_profile_warning_v1& value);
    inline Data native_value(const thinkthen_complete_batch_warning_v1& value);
    inline Data native_value(const thinkthen_complete_batch_setting_v1& value);
    inline Data native_value(const thinkthen_complete_relation_member_list_v1& value);
    inline Data native_value(const thinkthen_complete_answers_v1& value);
    inline Data native_value(const thinkthen_complete_relation_member_v1& value);
    inline Data native_value(const thinkthen_complete_related_entity_presence_v1& value);
    inline Data native_value(const thinkthen_complete_relation_member_failure_id_v1& value);
    inline Data native_value(const thinkthen_complete_related_entity_v1& value);
    inline Data native_value(const thinkthen_complete_relation_method_v1& value);
    inline Data native_value(const thinkthen_complete_relation_direction_v1& value);
    inline Data native_value(const thinkthen_complete_relation_member_answer_id_v1& value);
    inline Data native_value(const thinkthen_complete_answer_v1& value);
    inline Data native_value(const thinkthen_complete_number_map_entry_v1& value);
    inline Data native_value(const thinkthen_complete_number_map_v1& value);
    inline Data native_value(const thinkthen_complete_answer_score_v1& value);
    inline Data native_value(const thinkthen_complete_answer_tag_v1& value);
    inline Data native_value(const thinkthen_complete_answer_choice_v1& value);
    inline Data native_value(const thinkthen_complete_answer_yes_no_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_recognize_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_v1& value);
    inline Data native_value(const thinkthen_complete_recognize_v1& value);
    inline Data native_value(const thinkthen_complete_recognize_mode_proposals_v1& value);
    inline Data native_value(const thinkthen_complete_boundary_mode_v1& value);
    inline Data native_value(const thinkthen_complete_entity_edge_list_v1& value);
    inline Data native_value(const thinkthen_complete_entity_edge_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_recognize_entities_v1& value);
    inline Data native_value(const thinkthen_complete_entity_edge_v1& value);
    inline Data native_value(const thinkthen_complete_json_map_entry_v1& value);
    inline Data native_value(const thinkthen_complete_json_map_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_mode_presence_v1& value);
    inline Data native_value(const thinkthen_complete_relation_rule_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_stage_context_presence_v1& value);
    inline Data native_value(const thinkthen_complete_recognize_question_document_v1& value);
    inline Data native_value(const thinkthen_complete_verb_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_stage_context_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_odds_v1& value);
    inline Data native_value(const thinkthen_complete_piece_odds_list_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_odds_boundary_odds_v1& value);
    inline Data native_value(const thinkthen_complete_piece_odds_v1& value);
    inline Data native_value(const thinkthen_complete_name_odds_list_v1& value);
    inline Data native_value(const thinkthen_complete_pair_odds_list_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_proposal_list_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_odds_recognize_answer_v1& value);
    inline Data native_value(const thinkthen_complete_place_presence_v1& value);
    inline Data native_value(const thinkthen_complete_recognition_proposal_v1& value);
    inline Data native_value(const thinkthen_complete_place_v1& value);
    inline Data native_value(const thinkthen_complete_pair_odds_v1& value);
    inline Data native_value(const thinkthen_complete_number_map_presence_v1& value);
    inline Data native_value(const thinkthen_complete_name_odds_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_annotate_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_member_map_entry_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_member_map_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_v1& value);
    inline Data native_value(const thinkthen_complete_annotated_field_map_entry_v1& value);
    inline Data native_value(const thinkthen_complete_annotated_field_map_v1& value);
    inline Data native_value(const thinkthen_complete_annotated_row_v1& value);
    inline Data native_value(const thinkthen_complete_annotated_field_v1& value);
    inline Data native_value(const thinkthen_complete_failed_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_member_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_member_failure_id_v1& value);
    inline Data native_value(const thinkthen_complete_annotation_member_answer_id_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_find_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_find_candidate_list_v1& value);
    inline Data native_value(const thinkthen_complete_find_candidate_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_find_v1& value);
    inline Data native_value(const thinkthen_complete_find_question_document_v1& value);
    inline Data native_value(const thinkthen_complete_find_candidate_v1& value);
    inline Data native_value(const thinkthen_complete_find_answer_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_non_zero_usize_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_rank_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_image_list_v1& value);
    inline Data native_value(const thinkthen_complete_image_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_rank_member_list_v1& value);
    inline Data native_value(const thinkthen_complete_rank_member_list_presence_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_non_zero_usize_v1& value);
    inline Data native_value(const thinkthen_complete_rank_member_v1& value);
    inline Data native_value(const thinkthen_complete_rank_member_result_v1& value);
    inline Data native_value(const thinkthen_complete_image_v1& value);
    inline Data native_value(const thinkthen_complete_image_media_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_boolean_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_filter_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_boolean_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_double_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_score_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_double_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_array_of_string_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_tag_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_array_of_string_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_nullable_string_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_choose_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_nullable_string_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_decide_value_list_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_decide_aggregate_v1& value);
    inline Data native_value(const thinkthen_complete_decide_value_presence_v1& value);
    inline Data native_value(const thinkthen_complete_atomic_decide_value_v1& value);
    inline Data native_value(const thinkthen_complete_decide_value_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_annotate_row_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_filter_row_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_score_row_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_tag_row_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_choose_row_v1& value);
    inline Data native_value(const thinkthen_complete_session_packet_decide_row_v1& value);
    inline Data native_value(const thinkthen_complete_json_v1& value);
    inline Data native_value(const thinkthen_complete_utf8_v1& value) {
        return Data(std::string(value.data ? value.data : "", value.len));
    }
    template<class T> Data native_value(const T* value) {
        if(!value) throw std::invalid_argument("missing native value");
        return native_value(*value);
    }
    template<class T, std::enable_if_t<std::is_arithmetic_v<T>, int> = 0> Data native_value(T value) {
        return Data(value);
    }
    inline Data native_value(const thinkthen_complete_json_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_JSON_NULL_V1: return Data(nullptr);
            case THINKTHEN_COMPLETE_JSON_BOOLEAN_V1: return Data(bool(value.data.boolean));
            case THINKTHEN_COMPLETE_JSON_STRING_V1: return native_value(value.data.string);
            case THINKTHEN_COMPLETE_JSON_NUMBER_V1: {
                auto text=native_value(value.data.number).get<std::string>();
                if(text.find_first_of(".eE")==std::string::npos) {
                    if(!text.empty() && text[0]=='-') {
                        int64_t number{
                        };
                        auto parsed=std::from_chars(text.data(),text.data()+text.size(),number);
                        if(parsed.ec==std::errc{
                        }) return Data(number);
                    } else {
                        uint64_t number{
                        };
                        auto parsed=std::from_chars(text.data(),text.data()+text.size(),number);
                        if(parsed.ec==std::errc{
                        }) return Data(number);
                    }
                }
                double number{
                };
                auto parsed=std::from_chars(text.data(),text.data()+text.size(),number);
                auto exponent=text.find_first_of("eE");
                if(parsed.ec==std::errc::result_out_of_range && exponent!=std::string::npos && exponent+1<text.size() && text[exponent+1]=="-"[0]) return Data(text[0]=="-"[0] ? -0.0 : 0.0);
                if(parsed.ec!=std::errc{
                }) throw std::invalid_argument("invalid native number");
                return Data(number);
            }
            case THINKTHEN_COMPLETE_JSON_ARRAY_V1: {
                Data::Array items;
                for(size_t i=0;
                i<value.data.array.len;
                ++i) items.push_back(native_value(value.data.array.data[i]));
                return Data(std::move(items));
            }
            case THINKTHEN_COMPLETE_JSON_OBJECT_V1: {
                Data::Object items;
                for(size_t i=0;
                i<value.data.object.len;
                ++i) {
                    auto& item=value.data.object.data[i];
                    items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
                } return Data(std::move(items));
            }
            default: throw std::invalid_argument("invalid native value kind");
        }
    }
    inline Data native_value(const thinkthen_complete_attempt_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_attempt_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_call_id_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_utf8_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_u32_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_u64_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_persistence_observation_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_immediate_facts_v1& value) {
        Data::Object items;
        if(value.attempts.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("attempts",
        Data(nullptr));
        else if(value.attempts.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("attempts",
        native_value(value.attempts.value));
        items.emplace_back("cache_answers",native_value(value.cache_answers));
        if(value.call_id.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("call_id",
        Data(nullptr));
        else if(value.call_id.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("call_id",
        native_value(value.call_id.value));
        if(value.estimated_cost_usd.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("estimated_cost_usd",
        Data(nullptr));
        else if(value.estimated_cost_usd.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("estimated_cost_usd",
        native_value(value.estimated_cost_usd.value));
        if(value.held_model_mismatch.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("held_model_mismatch",
        Data(nullptr));
        else if(value.held_model_mismatch.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("held_model_mismatch",
        Data(bool(value.held_model_mismatch.value)));
        if(value.input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input_tokens",
        Data(nullptr));
        else if(value.input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input_tokens",
        native_value(value.input_tokens.value));
        items.emplace_back("largest_request_bytes",native_value(value.largest_request_bytes));
        if(value.largest_request_estimated_input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("largest_request_estimated_input_tokens",
        Data(nullptr));
        else if(value.largest_request_estimated_input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("largest_request_estimated_input_tokens",
        native_value(value.largest_request_estimated_input_tokens.value));
        if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("model",Data(nullptr));
        else if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("model",
        native_value(value.model.value));
        if(value.output_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("output_tokens",
        Data(nullptr));
        else if(value.output_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("output_tokens",
        native_value(value.output_tokens.value));
        items.emplace_back("records",native_value(value.records));
        items.emplace_back("requests_sent",native_value(value.requests_sent));
        items.emplace_back("seconds",native_value(value.seconds));
        items.emplace_back("token_estimate_method",native_value(value.token_estimate_method));
        if(value.usage_persistence.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("usage_persistence",
        Data(nullptr));
        else if(value.usage_persistence.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("usage_persistence",
        native_value(value.usage_persistence.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_persistence_observation_v1& value) {
        Data::Object items;
        if(value.advice.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("advice",
        Data(nullptr));
        else if(value.advice.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("advice",
        native_value(value.advice.value));
        items.emplace_back("observed_at",native_value(value.observed_at));
        items.emplace_back("state",native_value(value.state));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_usage_persistence_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1: return Data("disabled");
            case THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1: return Data("pending");
            case THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1: return Data("written");
            case THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1: return Data("failed");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_call_id_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_attempt_v1& value) {
        Data::Object items;
        items.emplace_back("ordinal",native_value(value.ordinal));
        items.emplace_back("outcome",native_value(value.outcome));
        if(value.request_id.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("request_id",
        Data(nullptr));
        else if(value.request_id.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("request_id",
        native_value(value.request_id.value));
        items.emplace_back("request_sha256",native_value(value.request_sha256));
        items.emplace_back("sdk_request_id",native_value(value.sdk_request_id));
        if(value.server_ms.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("server_ms",
        Data(nullptr));
        else if(value.server_ms.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("server_ms",
        native_value(value.server_ms.value));
        if(value.status.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("status",
        Data(nullptr));
        else if(value.status.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("status",
        native_value(value.status.value));
        items.emplace_back("wall_ms",native_value(value.wall_ms));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_sdk_request_id_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_attempt_outcome_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK_V1: return Data("ok");
            case THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS_V1: return Data("status");
            case THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT_V1: return Data("transport");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_session_packet_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1: return native_value(value.data.decide_row);
            case THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1: return native_value(value.data.choose_row);
            case THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1: return native_value(value.data.tag_row);
            case THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1: return native_value(value.data.score_row);
            case THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1: return native_value(value.data.filter_row);
            case THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1: return native_value(value.data.annotate_row);
            case THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1: return native_value(value.data.decide_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1: return native_value(value.data.choose_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1: return native_value(value.data.tag_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1: return native_value(value.data.score_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1: return native_value(value.data.filter_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1: return native_value(value.data.rank_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1: return native_value(value.data.find_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1: return native_value(value.data.annotate_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1: return native_value(value.data.recognize_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1: return native_value(value.data.relate_aggregate);
            case THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1: return native_value(value.data.observation);
            case THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1: return native_value(value.data.terminal);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_facts_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_call_error_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_session_packet_terminal_v1& value) {
        Data::Object items;
        if(value.facts.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("facts",Data(nullptr));
        else if(value.facts.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("facts",
        native_value(value.facts.value));
        if(value.failure.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("failure",
        Data(nullptr));
        else if(value.failure.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("failure",
        native_value(value.failure.value));
        items.emplace_back("kind",native_value(value.kind));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_call_error_v1& value) {
        Data::Object items;
        items.emplace_back("error",native_value(value.error));
        if(value.facts.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("facts",Data(nullptr));
        else if(value.facts.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("facts",
        native_value(value.facts.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_facts_v1& value) {
        Data::Object items;
        if(value.attempts.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("attempts",
        Data(nullptr));
        else if(value.attempts.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("attempts",
        native_value(value.attempts.value));
        items.emplace_back("cache_answers",native_value(value.cache_answers));
        items.emplace_back("call_id",native_value(value.call_id));
        if(value.estimated_cost_usd.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("estimated_cost_usd",
        Data(nullptr));
        else if(value.estimated_cost_usd.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("estimated_cost_usd",
        native_value(value.estimated_cost_usd.value));
        if(value.held_model_mismatch.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("held_model_mismatch",
        Data(nullptr));
        else if(value.held_model_mismatch.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("held_model_mismatch",
        Data(bool(value.held_model_mismatch.value)));
        if(value.input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input_tokens",
        Data(nullptr));
        else if(value.input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input_tokens",
        native_value(value.input_tokens.value));
        items.emplace_back("largest_request_bytes",native_value(value.largest_request_bytes));
        if(value.largest_request_estimated_input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("largest_request_estimated_input_tokens",
        Data(nullptr));
        else if(value.largest_request_estimated_input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("largest_request_estimated_input_tokens",
        native_value(value.largest_request_estimated_input_tokens.value));
        if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("model",Data(nullptr));
        else if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("model",
        native_value(value.model.value));
        if(value.output_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("output_tokens",
        Data(nullptr));
        else if(value.output_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("output_tokens",
        native_value(value.output_tokens.value));
        items.emplace_back("records",native_value(value.records));
        items.emplace_back("requests_sent",native_value(value.requests_sent));
        items.emplace_back("seconds",native_value(value.seconds));
        items.emplace_back("token_estimate_method",native_value(value.token_estimate_method));
        if(value.usage_persistence.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("usage_persistence",
        Data(nullptr));
        else if(value.usage_persistence.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("usage_persistence",
        native_value(value.usage_persistence.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_input_denial_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_send_denial_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_error_v1& value) {
        Data::Object items;
        if(value.estimated_input_denial.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("estimated_input_denial",
        Data(nullptr));
        else if(value.estimated_input_denial.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("estimated_input_denial",
        native_value(value.estimated_input_denial.value));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("message",native_value(value.message));
        items.emplace_back("retryable",Data(bool(value.retryable)));
        if(value.send_budget_denial.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("send_budget_denial",
        Data(nullptr));
        else if(value.send_budget_denial.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("send_budget_denial",
        native_value(value.send_budget_denial.value));
        items.emplace_back("stopped",native_value(value.stopped));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_stopped_v1& value) {
        Data::Object items;
        if(value.at.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("at",Data(nullptr));
        else if(value.at.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("at",native_value(value.at.value));
        items.emplace_back("cause",native_value(value.cause));
        items.emplace_back("retryable",Data(bool(value.retryable)));
        if(value.status.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("status",
        Data(nullptr));
        else if(value.status.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("status",
        native_value(value.status.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_stop_cause_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_STOP_CAUSE_USAGE_V1: return Data("usage");
            case THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL_V1: return Data("local");
            case THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY_V1: return Data("no_key");
            case THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT_V1: return Data("transport");
            case THINKTHEN_COMPLETE_STOP_CAUSE_STATUS_V1: return Data("status");
            case THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE_V1: return Data("too_large");
            case THINKTHEN_COMPLETE_STOP_CAUSE_REPLY_V1: return Data("reply");
            case THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND_V1: return Data("backend");
            case THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED_V1: return Data("cancelled");
            case THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE_V1: return Data("deadline");
            case THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT_V1: return Data("defect");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_send_denial_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND_V1: return native_value(value.data.before_first_send);
            case THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND_V1: return native_value(value.data.before_additional_send);
            case THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY_V1: return native_value(value.data.before_retry);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_send_denial_before_retry_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("last_status",native_value(value.last_status));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_send_denial_before_additional_send_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_send_denial_before_first_send_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_failure_kind_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_FAILURE_KIND_USAGE_V1: return Data("usage");
            case THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1: return Data("backend");
            case THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1: return Data("local");
            case THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED_V1: return Data("cancelled");
            case THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE_V1: return Data("deadline");
            case THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT_V1: return Data("defect");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_input_denial_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST_V1: return native_value(value.data.initial_request);
            case THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST_V1: return native_value(value.data.additional_request);
            case THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY_V1: return native_value(value.data.retry);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_input_denial_retry_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("last_status",native_value(value.last_status));
        items.emplace_back("limit",native_value(value.limit));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_input_denial_additional_request_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("limit",native_value(value.limit));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_input_denial_initial_request_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("limit",native_value(value.limit));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_observation_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_observation_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION_V1: return native_value(value.data.question);
            case THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW_V1: return native_value(value.data.row);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_session_observation_row_v1& value) {
        Data::Object items;
        items.emplace_back("index",native_value(value.index));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_observed_row_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT_V1: return native_value(value.data.judgment);
            case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED_V1: return native_value(value.data.annotated);
            case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED_V1: return native_value(value.data.recognized);
            case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND_V1: return native_value(value.data.find);
            case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS_V1: return native_value(value.data.relations);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_session_relation_edge_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_observed_row_relations_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_relation_edge_v1& value) {
        Data::Object items;
        items.emplace_back("either",Data(bool(value.either)));
        items.emplace_back("probability",native_value(value.probability));
        items.emplace_back("relation",native_value(value.relation));
        items.emplace_back("source",native_value(value.source));
        items.emplace_back("target",native_value(value.target));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_entity_document_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("name",native_value(value.name));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_observed_row_find_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        native_value(value.value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_observed_row_recognized_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_entity_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_boundary_proposal_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_boundary_proposal_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_recognition_edge_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_edge_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_session_recognition_v1& value) {
        Data::Object items;
        items.emplace_back("entities",native_value(value.entities));
        items.emplace_back("mode",native_value(value.mode));
        if(value.proposals.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("proposals",
        Data(nullptr));
        else if(value.proposals.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("proposals",
        native_value(value.proposals.value));
        if(value.relations.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("relations",
        Data(nullptr));
        else if(value.relations.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("relations",
        native_value(value.relations.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_edge_v1& value) {
        Data::Object items;
        items.emplace_back("either",Data(bool(value.either)));
        items.emplace_back("probability",native_value(value.probability));
        items.emplace_back("relation",native_value(value.relation));
        items.emplace_back("source",native_value(value.source));
        items.emplace_back("target",native_value(value.target));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_entity_v1& value) {
        Data::Object items;
        items.emplace_back("end",native_value(value.end));
        if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("file",Data(nullptr));
        else if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("file",
        native_value(value.file.value));
        if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("first_line",
        Data(nullptr));
        else if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("first_line",
        native_value(value.first_line.value));
        items.emplace_back("kind",native_value(value.kind));
        if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("last_line",
        Data(nullptr));
        else if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("last_line",
        native_value(value.last_line.value));
        items.emplace_back("length",native_value(value.length));
        items.emplace_back("start",native_value(value.start));
        items.emplace_back("strength",native_value(value.strength));
        items.emplace_back("text",native_value(value.text));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_boundary_proposal_v1& value) {
        Data::Object items;
        items.emplace_back("end",native_value(value.end));
        items.emplace_back("length",native_value(value.length));
        items.emplace_back("probability",native_value(value.probability));
        items.emplace_back("start",native_value(value.start));
        items.emplace_back("text",native_value(value.text));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_mode_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE_V1: return Data("whole");
            case THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY_V1: return Data("boundary_only");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_session_annotation_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_observed_row_annotated_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_annotation_v1& value) {
        Data::Object items;
        items.emplace_back("name",native_value(value.name));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_value_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION_V1: return native_value(value.data.decision);
            case THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE_V1: return native_value(value.data.choice);
            case THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE_V1: return native_value(value.data.score);
            case THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS_V1: return native_value(value.data.tags);
            case THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED_V1: return native_value(value.data.failed);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_annotation_value_failed_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_failure_field_kind_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND_V1: return Data("backend");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_failure_v1& value) {
        Data::Object items;
        items.emplace_back("cause",native_value(value.cause));
        items.emplace_back("kind",native_value(value.kind));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_failure_cause_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER_V1: return Data("missing_answer");
            case THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND_V1: return Data("wrong_kind");
            case THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY_V1: return Data("missing_probability");
            case THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY_V1: return Data("invalid_probability");
            case THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION_V1: return Data("invalid_distribution");
            case THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY_V1: return Data("unexpected_probability");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_string_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_value_tags_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_value_score_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_value_choice_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        native_value(value.value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_value_decision_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        Data(bool(value.value.value)));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_observed_row_judgment_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_judgment_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION_V1: return native_value(value.data.decision);
            case THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE_V1: return native_value(value.data.choice);
            case THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE_V1: return native_value(value.data.score);
            case THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS_V1: return native_value(value.data.tags);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_session_judgment_tags_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_judgment_score_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_judgment_choice_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        native_value(value.value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_judgment_decision_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        Data(bool(value.value.value)));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_observation_question_v1& value) {
        Data::Object items;
        items.emplace_back("detail",native_value(value.detail));
        items.emplace_back("index",native_value(value.index));
        items.emplace_back("kind",native_value(value.kind));
        if(value.member.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("member",
        Data(nullptr));
        else if(value.member.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("member",
        native_value(value.member.value));
        items.emplace_back("position",native_value(value.position));
        if(value.stage.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("stage",Data(nullptr));
        else if(value.stage.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("stage",
        native_value(value.stage.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_answer_id_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_f64_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_failure_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_failure_id_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_json_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_physical_source_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_session_input_source_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_json_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_observation_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_probabilities_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_question_source_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_usage_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_threshold_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_token_usage_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_value_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_session_question_detail_v1& value) {
        Data::Object items;
        if(value.answer_id.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("answer_id",
        Data(nullptr));
        else if(value.answer_id.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("answer_id",
        native_value(value.answer_id.value));
        items.emplace_back("cached",Data(bool(value.cached)));
        if(value.confidence.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("confidence",
        Data(nullptr));
        else if(value.confidence.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("confidence",
        native_value(value.confidence.value));
        items.emplace_back("failed_questions",native_value(value.failed_questions));
        if(value.failure.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("failure",
        Data(nullptr));
        else if(value.failure.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("failure",
        native_value(value.failure.value));
        if(value.failure_id.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("failure_id",
        Data(nullptr));
        else if(value.failure_id.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("failure_id",
        native_value(value.failure_id.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.input_source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input_source",
        Data(nullptr));
        else if(value.input_source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input_source",
        native_value(value.input_source.value));
        items.emplace_back("input_sources",native_value(value.input_sources));
        items.emplace_back("inputs",native_value(value.inputs));
        items.emplace_back("model",native_value(value.model));
        items.emplace_back("observations",native_value(value.observations));
        if(value.probabilities.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("probabilities",
        Data(nullptr));
        else if(value.probabilities.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("probabilities",
        native_value(value.probabilities.value));
        items.emplace_back("question",native_value(value.question));
        items.emplace_back("question_sha256",native_value(value.question_sha256));
        items.emplace_back("question_sources",native_value(value.question_sources));
        if(value.raw_pick.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("raw_pick",
        Data(nullptr));
        else if(value.raw_pick.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("raw_pick",
        native_value(value.raw_pick.value));
        if(value.reported_usage.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("reported_usage",
        Data(nullptr));
        else if(value.reported_usage.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("reported_usage",
        native_value(value.reported_usage.value));
        items.emplace_back("requests",native_value(value.requests));
        items.emplace_back("requests_sent",native_value(value.requests_sent));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        items.emplace_back("url",native_value(value.url));
        if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("usage",Data(nullptr));
        else if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("usage",
        native_value(value.usage.value));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        native_value(value.value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_value_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_VALUE_BOOLEAN_V1: return Data(bool(value.data.boolean));
            case THINKTHEN_COMPLETE_VALUE_NULL_V1: return Data(nullptr);
            case THINKTHEN_COMPLETE_VALUE_STRING_V1: return native_value(value.data.string);
            case THINKTHEN_COMPLETE_VALUE_ARRAY_V1: return native_value(value.data.array);
            case THINKTHEN_COMPLETE_VALUE_NUMBER_V1: return native_value(value.data.number);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_token_usage_v1& value) {
        Data::Object items;
        items.emplace_back("input_tokens",native_value(value.input_tokens));
        items.emplace_back("output_tokens",native_value(value.output_tokens));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_threshold_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_THRESHOLD_NUMBER_V1: return native_value(value.data.number);
            case THINKTHEN_COMPLETE_THRESHOLD_STRING_V1: return native_value(value.data.string);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_usage_v1& value) {
        Data::Object items;
        if(value.input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input_tokens",
        Data(nullptr));
        else if(value.input_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input_tokens",
        native_value(value.input_tokens.value));
        if(value.output_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("output_tokens",
        Data(nullptr));
        else if(value.output_tokens.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("output_tokens",
        native_value(value.output_tokens.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_question_source_v1& value) {
        Data::Object items;
        items.emplace_back("answered_by",native_value(value.answered_by));
        if(value.batch_size.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch_size",
        Data(nullptr));
        else if(value.batch_size.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch_size",
        native_value(value.batch_size.value));
        items.emplace_back("origin",native_value(value.origin));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_origin_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_ORIGIN_LIVE_V1: return Data("live");
            case THINKTHEN_COMPLETE_ORIGIN_CACHE_V1: return Data("cache");
            case THINKTHEN_COMPLETE_ORIGIN_REPLAY_V1: return Data("replay");
            case THINKTHEN_COMPLETE_ORIGIN_PROXY_V1: return Data("proxy");
            case THINKTHEN_COMPLETE_ORIGIN_MEMORY_V1: return Data("memory");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_question_document_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE_V1: return native_value(value.data.decide);
            case THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE_V1: return native_value(value.data.choose);
            case THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG_V1: return native_value(value.data.tag);
            case THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE_V1: return native_value(value.data.score);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_batch_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_input_declaration_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_label_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_label_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_question_name_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_string_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_wording_version_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_question_document_score_v1& value) {
        Data::Object items;
        if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch",Data(nullptr));
        else if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch",
        native_value(value.batch.value));
        if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("context_schema",
        Data(nullptr));
        else if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("context_schema",
        native_value(value.context_schema.value));
        if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("item_schema",
        Data(nullptr));
        else if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("item_schema",
        native_value(value.item_schema.value));
        if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("label_details",
        Data(nullptr));
        else if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("label_details",
        native_value(value.label_details.value));
        if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("model",Data(nullptr));
        else if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("model",
        native_value(value.model.value));
        if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("name",Data(nullptr));
        else if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("name",
        native_value(value.name.value));
        if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("on",Data(nullptr));
        else if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("on",native_value(value.on.value));
        if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("profile",
        Data(nullptr));
        else if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("profile",
        native_value(value.profile.value));
        if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("wording_version",
        Data(nullptr));
        else if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("wording_version",
        native_value(value.wording_version.value));
        items.emplace_back("levels",native_value(value.levels));
        if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("text",Data(nullptr));
        else if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("text",
        native_value(value.text.value));
        items.emplace_back("verb",native_value(value.verb));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_wording_version_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_question_name_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_label_v1& value) {
        Data::Object items;
        if(value.description.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("description",
        Data(nullptr));
        else if(value.description.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("description",
        native_value(value.description.value));
        items.emplace_back("name",native_value(value.name));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_input_declaration_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT_V1: return native_value(value.data.string_root);
            case THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT_V1: return native_value(value.data.object_root);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_input_property_type_map_entry_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_input_property_type_map_v1& value) {
        Data::Object items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            auto& item=value.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_input_declaration_object_root_v1& value) {
        Data::Object items;
        items.emplace_back("properties",native_value(value.properties));
        if(value.required.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("required",
        Data(nullptr));
        else if(value.required.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("required",
        native_value(value.required.value));
        items.emplace_back("type",native_value(value.type));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_object_type_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT_V1: return Data("object");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_input_property_type_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING_V1: return native_value(value.data.string);
            case THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER_V1: return native_value(value.data.number);
            case THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN_V1: return Data(bool(value.data.boolean));
            case THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY_V1: return native_value(value.data.array);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_input_property_type_array_v1& value) {
        Data::Object items;
        items.emplace_back("items",native_value(value.items));
        items.emplace_back("type",native_value(value.type));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_string_root_v1& value) {
        Data::Object items;
        items.emplace_back("type",native_value(value.type));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_string_type_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_STRING_TYPE_STRING_V1: return Data("string");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_input_property_type_boolean_v1& value) {
        Data::Object items;
        items.emplace_back("type",native_value(value.type));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_input_property_type_number_v1& value) {
        Data::Object items;
        items.emplace_back("type",native_value(value.type));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_input_property_type_string_v1& value) {
        Data::Object items;
        items.emplace_back("type",native_value(value.type));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_input_declaration_string_root_v1& value) {
        Data::Object items;
        items.emplace_back("type",native_value(value.type));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_batch_string_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_BATCH_STRING_MAX_V1: return Data("max");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_batch_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_BATCH_INTEGER_V1: return native_value(value.data.integer);
            case THINKTHEN_COMPLETE_BATCH_STRING_V1: return native_value(value.data.string);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_question_document_tag_v1& value) {
        Data::Object items;
        if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch",Data(nullptr));
        else if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch",
        native_value(value.batch.value));
        if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("context_schema",
        Data(nullptr));
        else if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("context_schema",
        native_value(value.context_schema.value));
        if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("item_schema",
        Data(nullptr));
        else if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("item_schema",
        native_value(value.item_schema.value));
        if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("label_details",
        Data(nullptr));
        else if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("label_details",
        native_value(value.label_details.value));
        if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("model",Data(nullptr));
        else if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("model",
        native_value(value.model.value));
        if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("name",Data(nullptr));
        else if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("name",
        native_value(value.name.value));
        if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("on",Data(nullptr));
        else if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("on",native_value(value.on.value));
        if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("profile",
        Data(nullptr));
        else if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("profile",
        native_value(value.profile.value));
        if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("wording_version",
        Data(nullptr));
        else if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("wording_version",
        native_value(value.wording_version.value));
        items.emplace_back("labels",native_value(value.labels));
        if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("text",Data(nullptr));
        else if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("text",
        native_value(value.text.value));
        items.emplace_back("verb",native_value(value.verb));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_question_document_choose_v1& value) {
        Data::Object items;
        if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch",Data(nullptr));
        else if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch",
        native_value(value.batch.value));
        if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("context_schema",
        Data(nullptr));
        else if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("context_schema",
        native_value(value.context_schema.value));
        if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("item_schema",
        Data(nullptr));
        else if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("item_schema",
        native_value(value.item_schema.value));
        if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("label_details",
        Data(nullptr));
        else if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("label_details",
        native_value(value.label_details.value));
        if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("model",Data(nullptr));
        else if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("model",
        native_value(value.model.value));
        if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("name",Data(nullptr));
        else if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("name",
        native_value(value.name.value));
        if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("on",Data(nullptr));
        else if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("on",native_value(value.on.value));
        if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("profile",
        Data(nullptr));
        else if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("profile",
        native_value(value.profile.value));
        if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("wording_version",
        Data(nullptr));
        else if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("wording_version",
        native_value(value.wording_version.value));
        items.emplace_back("options",native_value(value.options));
        if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("text",Data(nullptr));
        else if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("text",
        native_value(value.text.value));
        items.emplace_back("verb",native_value(value.verb));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_question_document_decide_v1& value) {
        Data::Object items;
        if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch",Data(nullptr));
        else if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch",
        native_value(value.batch.value));
        if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("context_schema",
        Data(nullptr));
        else if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("context_schema",
        native_value(value.context_schema.value));
        if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("item_schema",
        Data(nullptr));
        else if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("item_schema",
        native_value(value.item_schema.value));
        if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("label_details",
        Data(nullptr));
        else if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("label_details",
        native_value(value.label_details.value));
        if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("model",Data(nullptr));
        else if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("model",
        native_value(value.model.value));
        if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("name",Data(nullptr));
        else if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("name",
        native_value(value.name.value));
        if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("on",Data(nullptr));
        else if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("on",native_value(value.on.value));
        if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("profile",
        Data(nullptr));
        else if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("profile",
        native_value(value.profile.value));
        if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("wording_version",
        Data(nullptr));
        else if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("wording_version",
        native_value(value.wording_version.value));
        if(value.false_.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("false",Data(nullptr));
        else if(value.false_.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("false",
        native_value(value.false_.value));
        if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("text",Data(nullptr));
        else if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("text",
        native_value(value.text.value));
        if(value.true_.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("true",Data(nullptr));
        else if(value.true_.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("true",
        native_value(value.true_.value));
        items.emplace_back("verb",native_value(value.verb));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_probabilities_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO_V1: return native_value(value.data.yes_no);
            case THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED_V1: return native_value(value.data.named);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_session_named_probability_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_probabilities_named_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_named_probability_v1& value) {
        Data::Object items;
        items.emplace_back("name",native_value(value.name));
        items.emplace_back("probability",native_value(value.probability));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_probabilities_yes_no_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_observation_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID_V1: return native_value(value.data.observation_id);
            case THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID_V1: return native_value(value.data.failure_id);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_observation_failure_id_v1& value) {
        Data::Object items;
        items.emplace_back("failure_id",native_value(value.failure_id));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_failure_id_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_observation_observation_id_v1& value) {
        Data::Object items;
        items.emplace_back("observation_id",native_value(value.observation_id));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_observation_id_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_session_input_source_v1& value) {
        Data::Object items;
        items.emplace_back("index",native_value(value.index));
        items.emplace_back("source",native_value(value.source));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_physical_source_v1& value) {
        Data::Object items;
        items.emplace_back("file",native_value(value.file));
        if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("first_line",
        Data(nullptr));
        else if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("first_line",
        native_value(value.first_line.value));
        if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("last_line",
        Data(nullptr));
        else if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("last_line",
        native_value(value.last_line.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_answer_id_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_request_function_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1: return Data("decide");
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1: return Data("choose");
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1: return Data("tag");
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1: return Data("score");
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1: return Data("filter");
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1: return Data("rank");
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1: return Data("find");
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1: return Data("annotate");
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1: return Data("recognize");
            case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1: return Data("relate");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_session_packet_relate_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_input_source_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_position_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_related_entity_edge_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_relation_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("file",Data(nullptr));
        else if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("file",
        native_value(value.file.value));
        if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("first_line",
        Data(nullptr));
        else if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("first_line",
        native_value(value.first_line.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.input_sources.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input_sources",
        Data(nullptr));
        else if(value.input_sources.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input_sources",
        native_value(value.input_sources.value));
        if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("last_line",
        Data(nullptr));
        else if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("last_line",
        native_value(value.last_line.value));
        items.emplace_back("meta",native_value(value.meta));
        if(value.position.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("position",
        Data(nullptr));
        else if(value.position.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("position",
        native_value(value.position.value));
        items.emplace_back("question",native_value(value.question));
        items.emplace_back("schema",native_value(value.schema));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_related_entity_edge_v1& value) {
        Data::Object items;
        if(value.either.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("either",
        Data(nullptr));
        else if(value.either.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("either",
        Data(bool(value.either.value)));
        items.emplace_back("probability",native_value(value.probability));
        items.emplace_back("relation",native_value(value.relation));
        items.emplace_back("source",native_value(value.source));
        items.emplace_back("target",native_value(value.target));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_relation_endpoint_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY_V1: return native_value(value.data.related_entity);
            case THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE_V1: return native_value(value.data.source);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_relation_endpoint_source_v1& value) {
        Data::Object items;
        if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("file",Data(nullptr));
        else if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("file",
        native_value(value.file.value));
        if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("first_line",
        Data(nullptr));
        else if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("first_line",
        native_value(value.first_line.value));
        items.emplace_back("kind",native_value(value.kind));
        if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("last_line",
        Data(nullptr));
        else if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("last_line",
        native_value(value.last_line.value));
        items.emplace_back("name",native_value(value.name));
        items.emplace_back("ordinal",native_value(value.ordinal));
        if(value.record.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("record",
        Data(nullptr));
        else if(value.record.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("record",
        native_value(value.record.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_relation_endpoint_related_entity_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("name",native_value(value.name));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_version_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_VERSION_THINKTHEN_RESULT_2_V1: return Data("thinkthen.result/2");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_relate_fields_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_relation_rule_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_relate_question_document_v1& value) {
        Data::Object items;
        if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch",Data(nullptr));
        else if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch",
        native_value(value.batch.value));
        if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("context_schema",
        Data(nullptr));
        else if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("context_schema",
        native_value(value.context_schema.value));
        if(value.fields.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("fields",
        Data(nullptr));
        else if(value.fields.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("fields",
        native_value(value.fields.value));
        if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("item_schema",
        Data(nullptr));
        else if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("item_schema",
        native_value(value.item_schema.value));
        if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("label_details",
        Data(nullptr));
        else if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("label_details",
        native_value(value.label_details.value));
        if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("model",Data(nullptr));
        else if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("model",
        native_value(value.model.value));
        if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("name",Data(nullptr));
        else if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("name",
        native_value(value.name.value));
        if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("on",Data(nullptr));
        else if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("on",native_value(value.on.value));
        if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("profile",
        Data(nullptr));
        else if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("profile",
        native_value(value.profile.value));
        items.emplace_back("relations",native_value(value.relations));
        items.emplace_back("threshold",native_value(value.threshold));
        items.emplace_back("verb",native_value(value.verb));
        if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("wording_version",
        Data(nullptr));
        else if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("wording_version",
        native_value(value.wording_version.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_relation_rule_v1& value) {
        Data::Object items;
        items.emplace_back("either",Data(bool(value.either)));
        items.emplace_back("name",native_value(value.name));
        items.emplace_back("reads",native_value(value.reads));
        if(value.single.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("single",
        Data(nullptr));
        else if(value.single.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("single",
        Data(bool(value.single.value)));
        items.emplace_back("source",native_value(value.source));
        items.emplace_back("target",native_value(value.target));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_relate_fields_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("name",native_value(value.name));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_position_v1& value) {
        Data::Object items;
        if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("file",Data(nullptr));
        else if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("file",
        native_value(value.file.value));
        if(value.first.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("first",Data(nullptr));
        else if(value.first.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("first",
        native_value(value.first.value));
        if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("images",
        Data(nullptr));
        else if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("images",
        native_value(value.images.value));
        if(value.last.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("last",Data(nullptr));
        else if(value.last.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("last",
        native_value(value.last.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_batch_setting_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_batch_warning_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_origin_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_profile_warning_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_meta_v1& value) {
        Data::Object items;
        if(value.answered_by.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("answered_by",
        Data(nullptr));
        else if(value.answered_by.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("answered_by",
        native_value(value.answered_by.value));
        if(value.attempts.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("attempts",
        Data(nullptr));
        else if(value.attempts.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("attempts",
        native_value(value.attempts.value));
        if(value.batch_setting.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch_setting",
        Data(nullptr));
        else if(value.batch_setting.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch_setting",
        native_value(value.batch_setting.value));
        if(value.batch_warning.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch_warning",
        Data(nullptr));
        else if(value.batch_warning.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch_warning",
        native_value(value.batch_warning.value));
        items.emplace_back("cached",Data(bool(value.cached)));
        if(value.context_sha256.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("context_sha256",
        Data(nullptr));
        else if(value.context_sha256.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("context_sha256",
        native_value(value.context_sha256.value));
        items.emplace_back("failed_questions",native_value(value.failed_questions));
        items.emplace_back("model",native_value(value.model));
        items.emplace_back("observations",native_value(value.observations));
        if(value.origin.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("origin",
        Data(nullptr));
        else if(value.origin.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("origin",
        native_value(value.origin.value));
        if(value.profile_warning.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("profile_warning",
        Data(nullptr));
        else if(value.profile_warning.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("profile_warning",
        native_value(value.profile_warning.value));
        if(value.question_sha256.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("question_sha256",
        Data(nullptr));
        else if(value.question_sha256.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("question_sha256",
        native_value(value.question_sha256.value));
        items.emplace_back("question_sources",native_value(value.question_sources));
        if(value.questions_sha256.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("questions_sha256",
        Data(nullptr));
        else if(value.questions_sha256.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("questions_sha256",
        native_value(value.questions_sha256.value));
        items.emplace_back("requests",native_value(value.requests));
        items.emplace_back("requests_sent",native_value(value.requests_sent));
        items.emplace_back("tool",native_value(value.tool));
        items.emplace_back("url",native_value(value.url));
        if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("usage",Data(nullptr));
        else if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("usage",
        native_value(value.usage.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_profile_warning_v1& value) {
        Data::Object items;
        items.emplace_back("running",native_value(value.running));
        items.emplace_back("tuned_for",native_value(value.tuned_for));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_batch_warning_v1& value) {
        Data::Object items;
        items.emplace_back("running",native_value(value.running));
        items.emplace_back("tuned_for",native_value(value.tuned_for));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_batch_setting_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER_V1: return native_value(value.data.integer);
            case THINKTHEN_COMPLETE_BATCH_SETTING_STRING_V1: return native_value(value.data.string);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_relation_member_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_answers_v1& value) {
        Data::Object items;
        items.emplace_back("questions",native_value(value.questions));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_relation_member_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID_V1: return native_value(value.data.answer_id);
            case THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID_V1: return native_value(value.data.failure_id);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_related_entity_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_relation_member_failure_id_v1& value) {
        Data::Object items;
        items.emplace_back("direction",native_value(value.direction));
        items.emplace_back("method",native_value(value.method));
        items.emplace_back("observations",native_value(value.observations));
        items.emplace_back("question",native_value(value.question));
        items.emplace_back("question_sources",native_value(value.question_sources));
        items.emplace_back("reads",native_value(value.reads));
        items.emplace_back("relation",native_value(value.relation));
        items.emplace_back("request",native_value(value.request));
        items.emplace_back("source",native_value(value.source));
        if(value.target.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("target",
        Data(nullptr));
        else if(value.target.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("target",
        native_value(value.target.value));
        items.emplace_back("threshold",native_value(value.threshold));
        if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("usage",Data(nullptr));
        else if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("usage",
        native_value(value.usage.value));
        items.emplace_back("failure",native_value(value.failure));
        items.emplace_back("failure_id",native_value(value.failure_id));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_related_entity_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("name",native_value(value.name));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_relation_method_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO_V1: return Data("yes_no");
            case THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE_V1: return Data("choice");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_relation_direction_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET_V1: return Data("source_to_target");
            case THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER_V1: return Data("either");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_relation_member_answer_id_v1& value) {
        Data::Object items;
        items.emplace_back("direction",native_value(value.direction));
        items.emplace_back("method",native_value(value.method));
        items.emplace_back("observations",native_value(value.observations));
        items.emplace_back("question",native_value(value.question));
        items.emplace_back("question_sources",native_value(value.question_sources));
        items.emplace_back("reads",native_value(value.reads));
        items.emplace_back("relation",native_value(value.relation));
        items.emplace_back("request",native_value(value.request));
        items.emplace_back("source",native_value(value.source));
        if(value.target.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("target",
        Data(nullptr));
        else if(value.target.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("target",
        native_value(value.target.value));
        items.emplace_back("threshold",native_value(value.threshold));
        if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("usage",Data(nullptr));
        else if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("usage",
        native_value(value.usage.value));
        items.emplace_back("accepted",Data(bool(value.accepted)));
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        items.emplace_back("probability",native_value(value.probability));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_answer_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_ANSWER_YES_NO_V1: return native_value(value.data.yes_no);
            case THINKTHEN_COMPLETE_ANSWER_CHOICE_V1: return native_value(value.data.choice);
            case THINKTHEN_COMPLETE_ANSWER_TAG_V1: return native_value(value.data.tag);
            case THINKTHEN_COMPLETE_ANSWER_SCORE_V1: return native_value(value.data.score);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_number_map_entry_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_number_map_v1& value) {
        Data::Object items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            auto& item=value.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_answer_score_v1& value) {
        Data::Object items;
        if(value.confidence.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("confidence",
        Data(nullptr));
        else if(value.confidence.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("confidence",
        native_value(value.confidence.value));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("level",native_value(value.level));
        items.emplace_back("probabilities",native_value(value.probabilities));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_answer_tag_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("probabilities",native_value(value.probabilities));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_answer_choice_v1& value) {
        Data::Object items;
        if(value.confidence.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("confidence",
        Data(nullptr));
        else if(value.confidence.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("confidence",
        native_value(value.confidence.value));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("pick",native_value(value.pick));
        items.emplace_back("probabilities",native_value(value.probabilities));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_answer_yes_no_v1& value) {
        Data::Object items;
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("probability",native_value(value.probability));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_recognize_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("file",Data(nullptr));
        else if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("file",
        native_value(value.file.value));
        if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("first_line",
        Data(nullptr));
        else if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("first_line",
        native_value(value.first_line.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("last_line",
        Data(nullptr));
        else if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("last_line",
        native_value(value.last_line.value));
        items.emplace_back("meta",native_value(value.meta));
        if(value.position.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("position",
        Data(nullptr));
        else if(value.position.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("position",
        native_value(value.position.value));
        items.emplace_back("question",native_value(value.question));
        items.emplace_back("schema",native_value(value.schema));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognize_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1: return native_value(value.data.entities);
            case THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS_V1: return native_value(value.data.mode_proposals);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_recognize_mode_proposals_v1& value) {
        Data::Object items;
        items.emplace_back("mode",native_value(value.mode));
        items.emplace_back("proposals",native_value(value.proposals));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_boundary_mode_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY_V1: return Data("boundary_only");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_entity_edge_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_entity_edge_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_recognize_entities_v1& value) {
        Data::Object items;
        items.emplace_back("entities",native_value(value.entities));
        if(value.relations.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("relations",
        Data(nullptr));
        else if(value.relations.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("relations",
        native_value(value.relations.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_entity_edge_v1& value) {
        Data::Object items;
        if(value.either.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("either",
        Data(nullptr));
        else if(value.either.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("either",
        Data(bool(value.either.value)));
        items.emplace_back("probability",native_value(value.probability));
        items.emplace_back("relation",native_value(value.relation));
        items.emplace_back("source",native_value(value.source));
        items.emplace_back("target",native_value(value.target));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_json_map_entry_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_json_map_v1& value) {
        Data::Object items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            auto& item=value.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_mode_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_relation_rule_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_recognition_stage_context_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_recognize_question_document_v1& value) {
        Data::Object items;
        if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch",Data(nullptr));
        else if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch",
        native_value(value.batch.value));
        if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("context_schema",
        Data(nullptr));
        else if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("context_schema",
        native_value(value.context_schema.value));
        if(value.entity_definition.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("entity_definition",
        Data(nullptr));
        else if(value.entity_definition.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("entity_definition",
        native_value(value.entity_definition.value));
        if(value.instructions.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("instructions",
        Data(nullptr));
        else if(value.instructions.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("instructions",
        native_value(value.instructions.value));
        if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("item_schema",
        Data(nullptr));
        else if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("item_schema",
        native_value(value.item_schema.value));
        items.emplace_back("kinds",native_value(value.kinds));
        if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("label_details",
        Data(nullptr));
        else if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("label_details",
        native_value(value.label_details.value));
        if(value.mode.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("mode",Data(nullptr));
        else if(value.mode.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("mode",
        native_value(value.mode.value));
        if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("model",Data(nullptr));
        else if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("model",
        native_value(value.model.value));
        if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("name",Data(nullptr));
        else if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("name",
        native_value(value.name.value));
        if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("on",Data(nullptr));
        else if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("on",native_value(value.on.value));
        if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("profile",
        Data(nullptr));
        else if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("profile",
        native_value(value.profile.value));
        if(value.relation_threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("relation_threshold",
        Data(nullptr));
        else if(value.relation_threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("relation_threshold",
        native_value(value.relation_threshold.value));
        if(value.relations.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("relations",
        Data(nullptr));
        else if(value.relations.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("relations",
        native_value(value.relations.value));
        if(value.snippet_pieces.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("snippet_pieces",
        Data(nullptr));
        else if(value.snippet_pieces.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("snippet_pieces",
        native_value(value.snippet_pieces.value));
        if(value.stage_context.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("stage_context",
        Data(nullptr));
        else if(value.stage_context.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("stage_context",
        native_value(value.stage_context.value));
        items.emplace_back("threshold",native_value(value.threshold));
        items.emplace_back("verb",native_value(value.verb));
        if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("wording_version",
        Data(nullptr));
        else if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("wording_version",
        native_value(value.wording_version.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_verb_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_VERB_RECOGNIZE_V1: return Data("recognize");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_recognition_stage_context_v1& value) {
        Data::Object items;
        if(value.boundary.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("boundary",
        Data(nullptr));
        else if(value.boundary.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("boundary",
        native_value(value.boundary.value));
        if(value.kind_edge.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("kind_edge",
        Data(nullptr));
        else if(value.kind_edge.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("kind_edge",
        native_value(value.kind_edge.value));
        if(value.relation.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("relation",
        Data(nullptr));
        else if(value.relation.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("relation",
        native_value(value.relation.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_odds_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER_V1: return native_value(value.data.recognize_answer);
            case THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS_V1: return native_value(value.data.boundary_odds);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_piece_odds_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_odds_boundary_odds_v1& value) {
        Data::Object items;
        items.emplace_back("pieces",native_value(value.pieces));
        items.emplace_back("proposals",native_value(value.proposals));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_piece_odds_v1& value) {
        Data::Object items;
        items.emplace_back("end",native_value(value.end));
        items.emplace_back("start",native_value(value.start));
        items.emplace_back("tags",native_value(value.tags));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_name_odds_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_pair_odds_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_proposal_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_recognition_odds_recognize_answer_v1& value) {
        Data::Object items;
        items.emplace_back("names",native_value(value.names));
        items.emplace_back("pairs",native_value(value.pairs));
        items.emplace_back("pieces",native_value(value.pieces));
        items.emplace_back("proposals",native_value(value.proposals));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_place_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_recognition_proposal_v1& value) {
        Data::Object items;
        items.emplace_back("end",native_value(value.end));
        items.emplace_back("kept",Data(bool(value.kept)));
        if(value.kind.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("kind",Data(nullptr));
        else if(value.kind.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("kind",
        native_value(value.kind.value));
        if(value.selected.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("selected",
        Data(nullptr));
        else if(value.selected.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("selected",
        native_value(value.selected.value));
        items.emplace_back("span_probability",native_value(value.span_probability));
        items.emplace_back("start",native_value(value.start));
        if(value.strength.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("strength",
        Data(nullptr));
        else if(value.strength.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("strength",
        native_value(value.strength.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_place_v1& value) {
        Data::Object items;
        items.emplace_back("end",native_value(value.end));
        items.emplace_back("start",native_value(value.start));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_pair_odds_v1& value) {
        Data::Object items;
        items.emplace_back("probability",native_value(value.probability));
        items.emplace_back("relation",native_value(value.relation));
        items.emplace_back("source",native_value(value.source));
        items.emplace_back("target",native_value(value.target));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_number_map_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_name_odds_v1& value) {
        Data::Object items;
        if(value.edges.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("edges",Data(nullptr));
        else if(value.edges.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("edges",
        native_value(value.edges.value));
        items.emplace_back("end",native_value(value.end));
        if(value.kinds.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("kinds",Data(nullptr));
        else if(value.kinds.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("kinds",
        native_value(value.kinds.value));
        items.emplace_back("start",native_value(value.start));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_annotate_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_member_map_entry_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_annotation_member_map_v1& value) {
        Data::Object items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            auto& item=value.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_v1& value) {
        Data::Object items;
        items.emplace_back("answer_id",native_value(value.answer_id));
        items.emplace_back("answers",native_value(value.answers));
        if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("file",Data(nullptr));
        else if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("file",
        native_value(value.file.value));
        if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("first_line",
        Data(nullptr));
        else if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("first_line",
        native_value(value.first_line.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("last_line",
        Data(nullptr));
        else if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("last_line",
        native_value(value.last_line.value));
        items.emplace_back("meta",native_value(value.meta));
        if(value.position.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("position",
        Data(nullptr));
        else if(value.position.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("position",
        native_value(value.position.value));
        items.emplace_back("schema",native_value(value.schema));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotated_field_map_entry_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_annotated_field_map_v1& value) {
        Data::Object items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            auto& item=value.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotated_row_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_annotated_field_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN_V1: return Data(bool(value.data.boolean));
            case THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1: return Data(nullptr);
            case THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1: return native_value(value.data.string);
            case THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1: return native_value(value.data.array);
            case THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER_V1: return native_value(value.data.number);
            case THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT_V1: return native_value(value.data.object);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_failed_v1& value) {
        Data::Object items;
        items.emplace_back("failed",native_value(value.failed));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_member_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID_V1: return native_value(value.data.answer_id);
            case THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID_V1: return native_value(value.data.failure_id);
            default: throw std::invalid_argument("invalid native alternative");
        }
    }
    inline Data native_value(const thinkthen_complete_annotation_member_failure_id_v1& value) {
        Data::Object items;
        items.emplace_back("failure",native_value(value.failure));
        items.emplace_back("failure_id",native_value(value.failure_id));
        items.emplace_back("observations",native_value(value.observations));
        items.emplace_back("question",native_value(value.question));
        items.emplace_back("question_sources",native_value(value.question_sources));
        items.emplace_back("request",native_value(value.request));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("usage",Data(nullptr));
        else if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("usage",
        native_value(value.usage.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_annotation_member_answer_id_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        items.emplace_back("observations",native_value(value.observations));
        items.emplace_back("question",native_value(value.question));
        items.emplace_back("question_sources",native_value(value.question_sources));
        items.emplace_back("request",native_value(value.request));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("usage",Data(nullptr));
        else if(value.usage.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("usage",
        native_value(value.usage.value));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        native_value(value.value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_find_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_find_candidate_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_find_candidate_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_find_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.candidates.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("candidates",
        Data(nullptr));
        else if(value.candidates.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("candidates",
        native_value(value.candidates.value));
        if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("file",Data(nullptr));
        else if(value.file.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("file",
        native_value(value.file.value));
        if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("first_line",
        Data(nullptr));
        else if(value.first_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("first_line",
        native_value(value.first_line.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("last_line",
        Data(nullptr));
        else if(value.last_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("last_line",
        native_value(value.last_line.value));
        items.emplace_back("meta",native_value(value.meta));
        if(value.position.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("position",
        Data(nullptr));
        else if(value.position.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("position",
        native_value(value.position.value));
        items.emplace_back("question",native_value(value.question));
        items.emplace_back("schema",native_value(value.schema));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        native_value(value.value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_find_question_document_v1& value) {
        Data::Object items;
        if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("batch",Data(nullptr));
        else if(value.batch.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("batch",
        native_value(value.batch.value));
        if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("context_schema",
        Data(nullptr));
        else if(value.context_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("context_schema",
        native_value(value.context_schema.value));
        if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("item_schema",
        Data(nullptr));
        else if(value.item_schema.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("item_schema",
        native_value(value.item_schema.value));
        if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("label_details",
        Data(nullptr));
        else if(value.label_details.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("label_details",
        native_value(value.label_details.value));
        if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("model",Data(nullptr));
        else if(value.model.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("model",
        native_value(value.model.value));
        if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("name",Data(nullptr));
        else if(value.name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("name",
        native_value(value.name.value));
        items.emplace_back("none",Data(bool(value.none)));
        if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("on",Data(nullptr));
        else if(value.on.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("on",native_value(value.on.value));
        if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("profile",
        Data(nullptr));
        else if(value.profile.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("profile",
        native_value(value.profile.value));
        if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("text",Data(nullptr));
        else if(value.text.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("text",
        native_value(value.text.value));
        items.emplace_back("verb",native_value(value.verb));
        if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("wording_version",
        Data(nullptr));
        else if(value.wording_version.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("wording_version",
        native_value(value.wording_version.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_find_candidate_v1& value) {
        Data::Object items;
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        items.emplace_back("probability",native_value(value.probability));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_find_answer_v1& value) {
        Data::Object items;
        if(value.confidence.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("confidence",
        Data(nullptr));
        else if(value.confidence.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("confidence",
        native_value(value.confidence.value));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("pick",native_value(value.pick));
        items.emplace_back("probabilities",native_value(value.probabilities));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_atomic_non_zero_usize_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_rank_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_image_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_image_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_rank_member_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_rank_member_list_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_atomic_non_zero_usize_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("images",
        Data(nullptr));
        else if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("images",
        native_value(value.images.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("members",
        Data(nullptr));
        else if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("members",
        native_value(value.members.value));
        items.emplace_back("meta",native_value(value.meta));
        items.emplace_back("question",native_value(value.question));
        if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("question_name",
        Data(nullptr));
        else if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("question_name",
        native_value(value.question_name.value));
        items.emplace_back("schema",native_value(value.schema));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_rank_member_v1& value) {
        Data::Object items;
        items.emplace_back("name",native_value(value.name));
        items.emplace_back("result",native_value(value.result));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_rank_member_result_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("images",
        Data(nullptr));
        else if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("images",
        native_value(value.images.value));
        items.emplace_back("meta",native_value(value.meta));
        items.emplace_back("question",native_value(value.question));
        items.emplace_back("schema",native_value(value.schema));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_image_v1& value) {
        Data::Object items;
        items.emplace_back("base64",native_value(value.base64));
        items.emplace_back("height",native_value(value.height));
        items.emplace_back("media",native_value(value.media));
        items.emplace_back("width",native_value(value.width));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_image_media_v1& value) {
        switch(value.kind) {
            case THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG_V1: return Data("image/jpeg");
            case THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG_V1: return Data("image/png");
            default: throw std::invalid_argument("invalid native enum");
        }
    }
    inline Data native_value(const thinkthen_complete_atomic_boolean_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_filter_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_atomic_boolean_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("images",
        Data(nullptr));
        else if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("images",
        native_value(value.images.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("members",
        Data(nullptr));
        else if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("members",
        native_value(value.members.value));
        items.emplace_back("meta",native_value(value.meta));
        items.emplace_back("question",native_value(value.question));
        if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("question_name",
        Data(nullptr));
        else if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("question_name",
        native_value(value.question_name.value));
        items.emplace_back("schema",native_value(value.schema));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        items.emplace_back("value",Data(bool(value.value)));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_atomic_double_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_score_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_atomic_double_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("images",
        Data(nullptr));
        else if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("images",
        native_value(value.images.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("members",
        Data(nullptr));
        else if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("members",
        native_value(value.members.value));
        items.emplace_back("meta",native_value(value.meta));
        items.emplace_back("question",native_value(value.question));
        if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("question_name",
        Data(nullptr));
        else if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("question_name",
        native_value(value.question_name.value));
        items.emplace_back("schema",native_value(value.schema));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_atomic_array_of_string_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_tag_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_atomic_array_of_string_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("images",
        Data(nullptr));
        else if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("images",
        native_value(value.images.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("members",
        Data(nullptr));
        else if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("members",
        native_value(value.members.value));
        items.emplace_back("meta",native_value(value.meta));
        items.emplace_back("question",native_value(value.question));
        if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("question_name",
        Data(nullptr));
        else if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("question_name",
        native_value(value.question_name.value));
        items.emplace_back("schema",native_value(value.schema));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_atomic_nullable_string_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_choose_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_atomic_nullable_string_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("images",
        Data(nullptr));
        else if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("images",
        native_value(value.images.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("members",
        Data(nullptr));
        else if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("members",
        native_value(value.members.value));
        items.emplace_back("meta",native_value(value.meta));
        items.emplace_back("question",native_value(value.question));
        if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("question_name",
        Data(nullptr));
        else if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("question_name",
        native_value(value.question_name.value));
        items.emplace_back("schema",native_value(value.schema));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        native_value(value.value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_atomic_decide_value_list_v1& value) {
        Data::Array items;
        for(size_t i=0;
        i<value.len;
        ++i) {
            items.push_back(native_value(value.data[i]));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_decide_aggregate_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_decide_value_presence_v1& value) {
        return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);
    }
    inline Data native_value(const thinkthen_complete_atomic_decide_value_v1& value) {
        Data::Object items;
        items.emplace_back("answer",native_value(value.answer));
        items.emplace_back("answer_id",native_value(value.answer_id));
        if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("images",
        Data(nullptr));
        else if(value.images.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("images",
        native_value(value.images.value));
        if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("index",Data(nullptr));
        else if(value.index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("index",
        native_value(value.index.value));
        if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("input",Data(nullptr));
        else if(value.input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("input",
        native_value(value.input.value));
        if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("members",
        Data(nullptr));
        else if(value.members.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("members",
        native_value(value.members.value));
        items.emplace_back("meta",native_value(value.meta));
        items.emplace_back("question",native_value(value.question));
        if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("question_name",
        Data(nullptr));
        else if(value.question_name.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("question_name",
        native_value(value.question_name.value));
        items.emplace_back("schema",native_value(value.schema));
        if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("source",
        Data(nullptr));
        else if(value.source.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("source",
        native_value(value.source.value));
        if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("threshold",
        Data(nullptr));
        else if(value.threshold.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("threshold",
        native_value(value.threshold.value));
        if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back("value",Data(nullptr));
        else if(value.value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back("value",
        native_value(value.value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_decide_value_v1& value) {
        return native_value(value.value);
    }
    inline Data native_value(const thinkthen_complete_session_packet_annotate_row_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_filter_row_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_score_row_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_tag_row_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_choose_row_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
    inline Data native_value(const thinkthen_complete_session_packet_decide_row_v1& value) {
        Data::Object items;
        items.emplace_back("function",native_value(value.function));
        items.emplace_back("kind",native_value(value.kind));
        items.emplace_back("value",native_value(value.value));
        for(size_t i=0;
        i<value.extensions.len;
        ++i) {
            auto& item=value.extensions.data[i];
            items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));
        }
        return Data(std::move(items));
    }
}
#endif
// Generated from the compiler-derived C header; do not edit.
#include <cstdint>
#include <string>
#ifndef THINKTHEN_CPP_ENUMS
#define THINKTHEN_CPP_ENUMS
namespace tt {
    enum class UsagePersistenceState : uint32_t {
        disabled = 1,
        failed = 4,
        pending = 2,
        written = 3,
    };
    enum class NativeFailureKind {
        backend = 2,
        cancelled = 5,
        deadline = 3,
        defect = 6,
        local = 4,
        usage = 1,
    };
    inline NativeFailureKind failure_kind(const std::string& name) {
        if(name=="backend") return NativeFailureKind::backend;
        if(name=="cancelled") return NativeFailureKind::cancelled;
        if(name=="deadline") return NativeFailureKind::deadline;
        if(name=="defect") return NativeFailureKind::defect;
        if(name=="local") return NativeFailureKind::local;
        if(name=="usage") return NativeFailureKind::usage;
        return NativeFailureKind::defect;
    }
}
#endif
