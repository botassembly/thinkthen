// Generated from the shared Rust result graph; do not edit.
#import "TTResult.h"
NS_ASSUME_NONNULL_BEGIN
FOUNDATION_EXPORT NSInteger TTNativeErrorCode(NSString *kind);
@class TTAnnotation, TTAnnotationMember, TTAnnotationMemberAnswerId, TTAnnotationMemberFailureId,
TTAnnotationValue, TTAnnotationValueChoice, TTAnnotationValueDecision, TTAnnotationValueFailed,
TTAnnotationValueScore, TTAnnotationValueTags, TTAnswers, TTTagRecord, TTDecisionRecord, TTRankedRecord,
TTChoiceRecord, TTFilteredRecord, TTScoreRecord, TTAttempt, TTBatch, TTBoundaryOdds, TTBoundaryProposal,
TTCallError, TTEntityDocument, TTError, TTEstimatedInputDenial, TTEstimatedInputDenialAdditionalRequest,
TTEstimatedInputDenialInitialRequest, TTEstimatedInputDenialRetry, TTFacts, TTFind, TTFindCandidate,
TTImage, TTDeclarationBuilder, TTDeclarationObjectRootBuilder, TTDeclarationStringRootBuilder,
TTPropertyTypeBuilder, TTPropertyTypeArrayBuilder, TTPropertyTypeBooleanBuilder, TTPropertyTypeNumberBuilder,
TTPropertyTypeStringBuilder, TTLabel, TTMeta, TTObjectRoot, TTObservation, TTObservationFailureId,
TTObservationObservationId, TTPersistenceObservation, TTPhysicalSource, TTPosition, TTQuestionSource,
TTRankMember, TTRankMemberResult, TTRecognition, TTRecognitionEdgeDocument, TTRecognitionOdds,
TTRecognitionOddsBoundaryOdds, TTRecognitionOddsRecognizeAnswer, TTRecognitionProposal, TTRecognitionStageContext,
TTRelation, TTRelationEndpoint, TTRelationEndpointRelatedEntity, TTRelationEndpointSource,
TTRelationMember, TTRelationMemberAnswerId, TTRelationMemberFailureId, TTSendBudgetDenial,
TTSendBudgetDenialBeforeAdditionalSend, TTSendBudgetDenialBeforeFirstSend, TTSendBudgetDenialBeforeRetry,
TTStopped, TTStringRoot, TTUsage, TTAnnotatedField, TTAnswer, TTAnswerChoice, TTAnswerScore,
TTAnswerTag, TTAnswerYesNo, TTBatchSetting, TTBatchWarning, TTEntity, TTEntityEdge, TTFailed,
TTFailure, TTFindAnswer, TTFindQuestionDocument, TTNameOdds, TTPairOdds, TTPieceOdds, TTPlace,
TTPlan, TTProfileWarning, TTQuestionDocument, TTQuestionDocumentChoose, TTQuestionDocumentDecide,
TTQuestionDocumentScore, TTQuestionDocumentTag, TTRecognize, TTRecognizeAnswer, TTRecognizeQuestionDocument,
TTRecognizeEntities, TTRecognizeModeProposals, TTRelateFields, TTRelateQuestionDocument, TTRelatedEntity,
TTRelatedEntityEdge, TTRelationRule, TTSessionAnnotation, TTSessionInputSource, TTSessionJudgment,
TTSessionJudgmentChoice, TTSessionJudgmentDecision, TTSessionJudgmentScore, TTSessionJudgmentTags,
TTSessionNamedProbability, TTSessionObservation, TTSessionObservationQuestion, TTSessionObservationRow,
TTSessionObservedRow, TTSessionObservedRowAnnotated, TTSessionObservedRowFind, TTSessionObservedRowJudgment,
TTSessionObservedRowRecognized, TTSessionObservedRowRelations, TTSessionPacket, TTSessionPacketAnnotateAggregate,
TTSessionPacketAnnotateRow, TTSessionPacketChooseAggregate, TTSessionPacketChooseRow, TTSessionPacketDecideAggregate,
TTSessionPacketDecideRow, TTSessionPacketFilterAggregate, TTSessionPacketFilterRow, TTSessionPacketFindAggregate,
TTSessionPacketObservation, TTSessionPacketRankAggregate, TTSessionPacketRecognizeAggregate,
TTSessionPacketRelateAggregate, TTSessionPacketScoreAggregate, TTSessionPacketScoreRow, TTSessionPacketTagAggregate,
TTSessionPacketTagRow, TTSessionPacketTerminal, TTSessionProbabilities, TTSessionProbabilitiesNamed,
TTSessionProbabilitiesYesNo, TTSessionQuestionDetail, TTSessionRecognition, TTSessionRelationEdge,
TTSourceRelationEndpoint, TTThreshold, TTTokenBand, TTTokenUsage, TTValue, TTImmediateFacts;
@interface TTAnnotation : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, TTAnnotationMember *> *> *answers;
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTPosition *> *position;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, TTAnnotatedField *> *> *value;
@end
@interface TTAnnotationMember : TTResultNode
@end
@interface TTAnnotationValue : TTResultNode
@end
@interface TTAnswers : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTRelationMember *> *> *questions;
@end
@interface TTTagRecord : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *value;
@end
@interface TTDecisionRecord : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<id> *value;
@end
@interface TTRankedRecord : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTChoiceRecord : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSString *> *value;
@end
@interface TTFilteredRecord : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTScoreRecord : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTAttempt : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *ordinal;
@property(nonatomic, readonly) TTPresence<NSString *> *outcome;
@property(nonatomic, readonly) TTPresence<NSString *> *requestId;
@property(nonatomic, readonly) TTPresence<NSString *> *requestSha256;
@property(nonatomic, readonly) TTPresence<NSString *> *sdkRequestId;
@property(nonatomic, readonly) TTPresence<NSNumber *> *serverMs;
@property(nonatomic, readonly) TTPresence<NSNumber *> *status;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wallMs;
@end
@interface TTBatch : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
@end
@interface TTBoundaryOdds : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
@end
@interface TTBoundaryProposal : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSNumber *> *length;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
@property(nonatomic, readonly) TTPresence<NSString *> *text;
@end
@interface TTCallError : TTResultNode
@property(nonatomic, readonly) TTPresence<TTError *> *error;
@property(nonatomic, readonly) TTPresence<TTFacts *> *facts;
@end
@interface TTEntityDocument : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@end
@interface TTError : TTResultNode
@property(nonatomic, readonly) TTPresence<TTEstimatedInputDenial *> *estimatedInputDenial;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *message;
@property(nonatomic, readonly) TTPresence<NSNumber *> *retryable;
@property(nonatomic, readonly) TTPresence<TTSendBudgetDenial *> *sendBudgetDenial;
@property(nonatomic, readonly) TTPresence<TTStopped *> *stopped;
@end
@interface TTEstimatedInputDenial : TTResultNode
@end
@interface TTFacts : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTAttempt *> *> *attempts;
@property(nonatomic, readonly) TTPresence<NSNumber *> *cacheAnswers;
@property(nonatomic, readonly) TTPresence<NSString *> *callId;
@property(nonatomic, readonly) TTPresence<NSString *> *estimatedCostUsd;
@property(nonatomic, readonly) TTPresence<NSNumber *> *heldModelMismatch;
@property(nonatomic, readonly) TTPresence<NSNumber *> *inputTokens;
@property(nonatomic, readonly) TTPresence<NSNumber *> *largestRequestBytes;
@property(nonatomic, readonly) TTPresence<NSNumber *> *largestRequestEstimatedInputTokens;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSNumber *> *outputTokens;
@property(nonatomic, readonly) TTPresence<NSNumber *> *records;
@property(nonatomic, readonly) TTPresence<NSNumber *> *requestsSent;
@property(nonatomic, readonly) TTPresence<NSNumber *> *seconds;
@property(nonatomic, readonly) TTPresence<NSString *> *tokenEstimateMethod;
@property(nonatomic, readonly) TTPresence<TTPersistenceObservation *> *usagePersistence;
@end
@interface TTFind : TTResultNode
@property(nonatomic, readonly) TTPresence<TTFindAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTFindCandidate *> *> *candidates;
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTPosition *> *position;
@property(nonatomic, readonly) TTPresence<TTFindQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<id> *threshold;
@property(nonatomic, readonly) TTPresence<id> *value;
@end
@interface TTFindCandidate : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@end
@interface TTImage : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *base64;
@property(nonatomic, readonly) TTPresence<NSNumber *> *height;
@property(nonatomic, readonly) TTPresence<NSString *> *media;
@property(nonatomic, readonly) TTPresence<NSNumber *> *width;
@end
@interface TTDeclarationBuilder : TTResultNode
@end
@interface TTPropertyTypeBuilder : TTResultNode
@end
@interface TTLabel : TTResultNode
@property(nonatomic, readonly) TTPresence<id> *descriptionValue;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@end
@interface TTMeta : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *answeredBy;
@property(nonatomic, readonly) TTPresence<NSArray<TTAttempt *> *> *attempts;
@property(nonatomic, readonly) TTPresence<TTBatchSetting *> *batchSetting;
@property(nonatomic, readonly) TTPresence<TTBatchWarning *> *batchWarning;
@property(nonatomic, readonly) TTPresence<NSNumber *> *cached;
@property(nonatomic, readonly) TTPresence<NSString *> *contextSha256;
@property(nonatomic, readonly) TTPresence<NSNumber *> *failedQuestions;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<NSString *> *origin;
@property(nonatomic, readonly) TTPresence<TTProfileWarning *> *profileWarning;
@property(nonatomic, readonly) TTPresence<NSString *> *questionSha256;
@property(nonatomic, readonly) TTPresence<NSArray<TTQuestionSource *> *> *questionSources;
@property(nonatomic, readonly) TTPresence<NSString *> *questionsSha256;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *requests;
@property(nonatomic, readonly) TTPresence<NSNumber *> *requestsSent;
@property(nonatomic, readonly) TTPresence<NSString *> *tool;
@property(nonatomic, readonly) TTPresence<NSString *> *url;
@property(nonatomic, readonly) TTPresence<TTUsage *> *usage;
@end
@interface TTObjectRoot : TTResultNode
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, TTPropertyTypeBuilder *> *> *properties;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *required;
@property(nonatomic, readonly) TTPresence<NSString *> *type;
@end
@interface TTObservation : TTResultNode
@end
@interface TTPersistenceObservation : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *advice;
@property(nonatomic, readonly) TTPresence<NSString *> *observedAt;
@property(nonatomic, readonly) TTPresence<NSString *> *state;
@end
@interface TTPhysicalSource : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@end
@interface TTPosition : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *first;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *last;
@end
@interface TTQuestionSource : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *answeredBy;
@property(nonatomic, readonly) TTPresence<NSNumber *> *batchSize;
@property(nonatomic, readonly) TTPresence<NSString *> *origin;
@end
@interface TTRankMember : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<TTRankMemberResult *> *result;
@end
@interface TTRankMemberResult : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<id> *threshold;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTRecognition : TTResultNode
@property(nonatomic, readonly) TTPresence<TTRecognitionOdds *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTPosition *> *position;
@property(nonatomic, readonly) TTPresence<TTRecognizeQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTRecognize *> *value;
@end
@interface TTRecognitionEdgeDocument : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTEntity *> *source;
@property(nonatomic, readonly) TTPresence<TTEntity *> *target;
@end
@interface TTRecognitionOdds : TTResultNode
@end
@interface TTRecognitionProposal : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSNumber *> *kept;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTPlace *> *selected;
@property(nonatomic, readonly) TTPresence<NSNumber *> *spanProbability;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
@property(nonatomic, readonly) TTPresence<NSNumber *> *strength;
@end
@interface TTRecognitionStageContext : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *boundary;
@property(nonatomic, readonly) TTPresence<NSString *> *kindEdge;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@end
@interface TTRelation : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswers *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTSessionInputSource *> *> *inputSources;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTPosition *> *position;
@property(nonatomic, readonly) TTPresence<TTRelateQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<NSArray<TTRelatedEntityEdge *> *> *value;
@end
@interface TTRelationEndpoint : TTResultNode
@end
@interface TTRelationMember : TTResultNode
@end
@interface TTSendBudgetDenial : TTResultNode
@end
@interface TTStopped : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *at;
@property(nonatomic, readonly) TTPresence<NSString *> *cause;
@property(nonatomic, readonly) TTPresence<NSNumber *> *retryable;
@property(nonatomic, readonly) TTPresence<NSNumber *> *status;
@end
@interface TTStringRoot : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *type;
@end
@interface TTUsage : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *inputTokens;
@property(nonatomic, readonly) TTPresence<NSNumber *> *outputTokens;
@end
@interface TTAnnotatedField : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
@end
@interface TTAnswer : TTResultNode
@end
@interface TTBatchSetting : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
@end
@interface TTBatchWarning : TTResultNode
@property(nonatomic, readonly) TTPresence<TTBatchSetting *> *running;
@property(nonatomic, readonly) TTPresence<TTBatchSetting *> *tunedFor;
@end
@interface TTEntity : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<NSNumber *> *length;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
@property(nonatomic, readonly) TTPresence<NSNumber *> *strength;
@property(nonatomic, readonly) TTPresence<NSString *> *text;
@end
@interface TTEntityEdge : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTEntity *> *source;
@property(nonatomic, readonly) TTPresence<TTEntity *> *target;
@end
@interface TTFailed : TTResultNode
@property(nonatomic, readonly) TTPresence<TTFailure *> *failed;
@end
@interface TTFailure : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *cause;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@end
@interface TTFindAnswer : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *confidence;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *pick;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *probabilities;
@end
@interface TTFindQuestionDocument : TTResultNode
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSNumber *> *none;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<id> *text;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@end
@interface TTNameOdds : TTResultNode
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *edges;
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *kinds;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
@end
@interface TTPairOdds : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTPlace *> *source;
@property(nonatomic, readonly) TTPresence<TTPlace *> *target;
@end
@interface TTPieceOdds : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *tags;
@end
@interface TTPlace : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
@end
@interface TTPlan : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *estimatedBytes;
@property(nonatomic, readonly) TTPresence<TTTokenBand *> *estimatedInputTokens;
@property(nonatomic, readonly) TTPresence<NSString *> *firstBodyUtf8;
@property(nonatomic, readonly) TTPresence<NSNumber *> *largestRequestBytes;
@property(nonatomic, readonly) TTPresence<NSNumber *> *largestRequestEstimatedInputTokens;
@property(nonatomic, readonly) TTPresence<NSNumber *> *records;
@property(nonatomic, readonly) TTPresence<NSNumber *> *requests;
@property(nonatomic, readonly) TTPresence<NSString *> *tokenEstimateMethod;
@property(nonatomic, readonly) TTPresence<NSNumber *> *upperBound;
@end
@interface TTProfileWarning : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *running;
@property(nonatomic, readonly) TTPresence<NSString *> *tunedFor;
@end
@interface TTQuestionDocument : TTResultNode
@end
@interface TTRecognize : TTResultNode
@end
@interface TTRecognizeAnswer : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTNameOdds *> *> *names;
@property(nonatomic, readonly) TTPresence<NSArray<TTPairOdds *> *> *pairs;
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognitionProposal *> *> *proposals;
@end
@interface TTRecognizeQuestionDocument : TTResultNode
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *contextSchema;
@property(nonatomic, readonly) TTPresence<id> *entityDefinition;
@property(nonatomic, readonly) TTPresence<id> *instructions;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, id> *> *kinds;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *mode;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *relationThreshold;
@property(nonatomic, readonly) TTPresence<NSArray<TTRelationRule *> *> *relations;
@property(nonatomic, readonly) TTPresence<NSNumber *> *snippetPieces;
@property(nonatomic, readonly) TTPresence<TTRecognitionStageContext *> *stageContext;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@end
@interface TTRelateFields : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@end
@interface TTRelateQuestionDocument : TTResultNode
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTRelateFields *> *fields;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<NSArray<TTRelationRule *> *> *relations;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@end
@interface TTRelatedEntity : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@end
@interface TTRelatedEntityEdge : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTRelationEndpoint *> *source;
@property(nonatomic, readonly) TTPresence<TTRelationEndpoint *> *target;
@property(nonatomic, readonly) TTPresence<NSString *> *sourceName;
@property(nonatomic, readonly) TTPresence<NSString *> *targetName;
@end
@interface TTRelationRule : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSString *> *reads;
@property(nonatomic, readonly) TTPresence<NSNumber *> *single;
@property(nonatomic, readonly) TTPresence<NSString *> *source;
@property(nonatomic, readonly) TTPresence<NSString *> *target;
@end
@interface TTSessionAnnotation : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<TTAnnotationValue *> *value;
@end
@interface TTSessionInputSource : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@end
@interface TTSessionJudgment : TTResultNode
@end
@interface TTSessionNamedProbability : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@end
@interface TTSessionObservation : TTResultNode
@end
@interface TTSessionObservedRow : TTResultNode
@end
@interface TTSessionPacket : TTResultNode
@end
@interface TTSessionProbabilities : TTResultNode
@end
@interface TTSessionQuestionDetail : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSNumber *> *cached;
@property(nonatomic, readonly) TTPresence<NSNumber *> *confidence;
@property(nonatomic, readonly) TTPresence<NSNumber *> *failedQuestions;
@property(nonatomic, readonly) TTPresence<TTFailure *> *failure;
@property(nonatomic, readonly) TTPresence<NSString *> *failureId;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *inputSource;
@property(nonatomic, readonly) TTPresence<NSArray<TTSessionInputSource *> *> *inputSources;
@property(nonatomic, readonly) TTPresence<NSArray<id> *> *inputs;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<TTSessionProbabilities *> *probabilities;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionSha256;
@property(nonatomic, readonly) TTPresence<NSArray<TTQuestionSource *> *> *questionSources;
@property(nonatomic, readonly) TTPresence<NSString *> *rawPick;
@property(nonatomic, readonly) TTPresence<TTUsage *> *reportedUsage;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *requests;
@property(nonatomic, readonly) TTPresence<NSNumber *> *requestsSent;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSString *> *url;
@property(nonatomic, readonly) TTPresence<TTTokenUsage *> *usage;
@property(nonatomic, readonly) TTPresence<TTValue *> *value;
@end
@interface TTSessionRecognition : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTEntity *> *> *entities;
@property(nonatomic, readonly) TTPresence<NSString *> *mode;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognitionEdgeDocument *> *> *relations;
@end
@interface TTSessionRelationEdge : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTEntityDocument *> *source;
@property(nonatomic, readonly) TTPresence<TTEntityDocument *> *target;
@property(nonatomic, readonly) TTPresence<NSString *> *sourceName;
@property(nonatomic, readonly) TTPresence<NSString *> *targetName;
@end
@interface TTSourceRelationEndpoint : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSNumber *> *ordinal;
@property(nonatomic, readonly) TTPresence<id> *record;
@end
@interface TTThreshold : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
@end
@interface TTTokenBand : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *lower;
@property(nonatomic, readonly) TTPresence<NSNumber *> *upper;
@end
@interface TTTokenUsage : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *inputTokens;
@property(nonatomic, readonly) TTPresence<NSNumber *> *outputTokens;
@end
@interface TTValue : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
@end
@interface TTImmediateFacts : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTAttempt *> *> *attempts;
@property(nonatomic, readonly) TTPresence<NSNumber *> *cacheAnswers;
@property(nonatomic, readonly) TTPresence<NSString *> *callId;
@property(nonatomic, readonly) TTPresence<NSString *> *estimatedCostUsd;
@property(nonatomic, readonly) TTPresence<NSNumber *> *heldModelMismatch;
@property(nonatomic, readonly) TTPresence<NSNumber *> *inputTokens;
@property(nonatomic, readonly) TTPresence<NSNumber *> *largestRequestBytes;
@property(nonatomic, readonly) TTPresence<NSNumber *> *largestRequestEstimatedInputTokens;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSNumber *> *outputTokens;
@property(nonatomic, readonly) TTPresence<NSNumber *> *records;
@property(nonatomic, readonly) TTPresence<NSNumber *> *requestsSent;
@property(nonatomic, readonly) TTPresence<NSNumber *> *seconds;
@property(nonatomic, readonly) TTPresence<NSString *> *tokenEstimateMethod;
@property(nonatomic, readonly) TTPresence<TTPersistenceObservation *> *usagePersistence;
@end
@interface TTAnnotationMemberAnswerId : TTAnnotationMember
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSArray<TTQuestionSource *> *> *questionSources;
@property(nonatomic, readonly) TTPresence<NSString *> *request;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<TTUsage *> *usage;
@property(nonatomic, readonly) TTPresence<TTValue *> *value;
@end
@interface TTAnnotationMemberFailureId : TTAnnotationMember
@property(nonatomic, readonly) TTPresence<TTFailure *> *failure;
@property(nonatomic, readonly) TTPresence<NSString *> *failureId;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSArray<TTQuestionSource *> *> *questionSources;
@property(nonatomic, readonly) TTPresence<NSString *> *request;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<TTUsage *> *usage;
@end
@interface TTAnnotationValueChoice : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *value;
@end
@interface TTAnnotationValueDecision : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTAnnotationValueFailed : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTFailure *> *value;
@end
@interface TTAnnotationValueScore : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTAnnotationValueTags : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *value;
@end
@interface TTEstimatedInputDenialAdditionalRequest : TTEstimatedInputDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *limit;
@end
@interface TTEstimatedInputDenialInitialRequest : TTEstimatedInputDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *limit;
@end
@interface TTEstimatedInputDenialRetry : TTEstimatedInputDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastStatus;
@property(nonatomic, readonly) TTPresence<NSNumber *> *limit;
@end
@interface TTDeclarationObjectRootBuilder : TTDeclarationBuilder
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, TTPropertyTypeBuilder *> *> *properties;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *required;
@property(nonatomic, readonly) TTPresence<NSString *> *type;
@end
@interface TTDeclarationStringRootBuilder : TTDeclarationBuilder
@property(nonatomic, readonly) TTPresence<NSString *> *type;
@end
@interface TTPropertyTypeArrayBuilder : TTPropertyTypeBuilder
@property(nonatomic, readonly) TTPresence<TTStringRoot *> *items;
@property(nonatomic, readonly) TTPresence<NSString *> *type;
@end
@interface TTPropertyTypeBooleanBuilder : TTPropertyTypeBuilder
@property(nonatomic, readonly) TTPresence<NSString *> *type;
@end
@interface TTPropertyTypeNumberBuilder : TTPropertyTypeBuilder
@property(nonatomic, readonly) TTPresence<NSString *> *type;
@end
@interface TTPropertyTypeStringBuilder : TTPropertyTypeBuilder
@property(nonatomic, readonly) TTPresence<NSString *> *type;
@end
@interface TTObservationFailureId : TTObservation
@property(nonatomic, readonly) TTPresence<NSString *> *failureId;
@end
@interface TTObservationObservationId : TTObservation
@property(nonatomic, readonly) TTPresence<NSString *> *observationId;
@end
@interface TTRecognitionOddsBoundaryOdds : TTRecognitionOdds
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
@end
@interface TTRecognitionOddsRecognizeAnswer : TTRecognitionOdds
@property(nonatomic, readonly) TTPresence<NSArray<TTNameOdds *> *> *names;
@property(nonatomic, readonly) TTPresence<NSArray<TTPairOdds *> *> *pairs;
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognitionProposal *> *> *proposals;
@end
@interface TTRelationEndpointRelatedEntity : TTRelationEndpoint
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@end
@interface TTRelationEndpointSource : TTRelationEndpoint
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSNumber *> *ordinal;
@property(nonatomic, readonly) TTPresence<id> *record;
@end
@interface TTRelationMemberAnswerId : TTRelationMember
@property(nonatomic, readonly) TTPresence<NSString *> *direction;
@property(nonatomic, readonly) TTPresence<NSString *> *method;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSArray<TTQuestionSource *> *> *questionSources;
@property(nonatomic, readonly) TTPresence<NSString *> *reads;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<NSString *> *request;
@property(nonatomic, readonly) TTPresence<TTRelatedEntity *> *source;
@property(nonatomic, readonly) TTPresence<TTRelatedEntity *> *target;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<TTUsage *> *usage;
@property(nonatomic, readonly) TTPresence<NSNumber *> *accepted;
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *sourceName;
@end
@interface TTRelationMemberFailureId : TTRelationMember
@property(nonatomic, readonly) TTPresence<NSString *> *direction;
@property(nonatomic, readonly) TTPresence<NSString *> *method;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<TTQuestionDocument *> *question;
@property(nonatomic, readonly) TTPresence<NSArray<TTQuestionSource *> *> *questionSources;
@property(nonatomic, readonly) TTPresence<NSString *> *reads;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<NSString *> *request;
@property(nonatomic, readonly) TTPresence<TTRelatedEntity *> *source;
@property(nonatomic, readonly) TTPresence<TTRelatedEntity *> *target;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<TTUsage *> *usage;
@property(nonatomic, readonly) TTPresence<TTFailure *> *failure;
@property(nonatomic, readonly) TTPresence<NSString *> *failureId;
@property(nonatomic, readonly) TTPresence<NSString *> *sourceName;
@end
@interface TTSendBudgetDenialBeforeAdditionalSend : TTSendBudgetDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@end
@interface TTSendBudgetDenialBeforeFirstSend : TTSendBudgetDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@end
@interface TTSendBudgetDenialBeforeRetry : TTSendBudgetDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastStatus;
@end
@interface TTAnswerChoice : TTAnswer
@property(nonatomic, readonly) TTPresence<NSNumber *> *confidence;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *pick;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *probabilities;
@end
@interface TTAnswerScore : TTAnswer
@property(nonatomic, readonly) TTPresence<NSNumber *> *confidence;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *level;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *probabilities;
@end
@interface TTAnswerTag : TTAnswer
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *probabilities;
@end
@interface TTAnswerYesNo : TTAnswer
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@end
@interface TTQuestionDocumentChoose : TTQuestionDocument
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *options;
@property(nonatomic, readonly) TTPresence<id> *text;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
@end
@interface TTQuestionDocumentDecide : TTQuestionDocument
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@property(nonatomic, readonly) TTPresence<id> *falseValue;
@property(nonatomic, readonly) TTPresence<id> *text;
@property(nonatomic, readonly) TTPresence<id> *trueValue;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
@end
@interface TTQuestionDocumentScore : TTQuestionDocument
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *levels;
@property(nonatomic, readonly) TTPresence<id> *text;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
@end
@interface TTQuestionDocumentTag : TTQuestionDocument
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTDeclarationBuilder *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *labels;
@property(nonatomic, readonly) TTPresence<id> *text;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
@end
@interface TTRecognizeEntities : TTRecognize
@property(nonatomic, readonly) TTPresence<NSArray<TTEntity *> *> *entities;
@property(nonatomic, readonly) TTPresence<NSArray<TTEntityEdge *> *> *relations;
@end
@interface TTRecognizeModeProposals : TTRecognize
@property(nonatomic, readonly) TTPresence<NSString *> *mode;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
@end
@interface TTSessionJudgmentChoice : TTSessionJudgment
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *value;
@end
@interface TTSessionJudgmentDecision : TTSessionJudgment
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTSessionJudgmentScore : TTSessionJudgment
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTSessionJudgmentTags : TTSessionJudgment
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *value;
@end
@interface TTSessionObservationQuestion : TTSessionObservation
@property(nonatomic, readonly) TTPresence<TTSessionQuestionDetail *> *detail;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *member;
@property(nonatomic, readonly) TTPresence<NSNumber *> *position;
@property(nonatomic, readonly) TTPresence<NSString *> *stage;
@end
@interface TTSessionObservationRow : TTSessionObservation
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTSessionObservedRow *> *value;
@end
@interface TTSessionObservedRowAnnotated : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTSessionAnnotation *> *> *value;
@end
@interface TTSessionObservedRowFind : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTSessionObservedRowJudgment : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTSessionJudgment *> *value;
@end
@interface TTSessionObservedRowRecognized : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTSessionRecognition *> *value;
@end
@interface TTSessionObservedRowRelations : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTSessionRelationEdge *> *> *value;
@end
@interface TTSessionPacketAnnotateAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTAnnotation *> *> *value;
@end
@interface TTSessionPacketAnnotateRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTAnnotation *> *value;
@end
@interface TTSessionPacketChooseAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTChoiceRecord *> *> *value;
@end
@interface TTSessionPacketChooseRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTChoiceRecord *> *value;
@end
@interface TTSessionPacketDecideAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTDecisionRecord *> *> *value;
@end
@interface TTSessionPacketDecideRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTDecisionRecord *> *value;
@end
@interface TTSessionPacketFilterAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTFilteredRecord *> *> *value;
@end
@interface TTSessionPacketFilterRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTFilteredRecord *> *value;
@end
@interface TTSessionPacketFindAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTFind *> *value;
@end
@interface TTSessionPacketObservation : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTSessionObservation *> *value;
@end
@interface TTSessionPacketRankAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankedRecord *> *> *value;
@end
@interface TTSessionPacketRecognizeAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognition *> *> *value;
@end
@interface TTSessionPacketRelateAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTRelation *> *value;
@end
@interface TTSessionPacketScoreAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTScoreRecord *> *> *value;
@end
@interface TTSessionPacketScoreRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTScoreRecord *> *value;
@end
@interface TTSessionPacketTagAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTTagRecord *> *> *value;
@end
@interface TTSessionPacketTagRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTTagRecord *> *value;
@end
@interface TTSessionPacketTerminal : TTSessionPacket
@property(nonatomic, readonly) TTPresence<TTFacts *> *facts;
@property(nonatomic, readonly) TTPresence<TTCallError *> *failure;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@end
@interface TTSessionProbabilitiesNamed : TTSessionProbabilities
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTSessionNamedProbability *> *> *value;
@end
@interface TTSessionProbabilitiesYesNo : TTSessionProbabilities
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
@end
@interface TTOrderedEntry : NSObject
@property(nonatomic, copy, readonly) NSString *key;
@property(nonatomic, strong, readonly) id value;
- (instancetype)initWithKey:(NSString *)key value:(id)value;
@end
@interface TTOrderedMap : NSObject
@property(nonatomic, copy, readonly) NSArray<TTOrderedEntry *> *entries;
- (instancetype)initWithEntries:(NSArray<TTOrderedEntry *> *)entries;
@end
@interface TTRequestObject : NSObject
@end
@interface TTQuestion : TTRequestObject
@end
@interface TTInput : TTRequestObject
@end
typedef NSString *TTImageMediaBuilder NS_TYPED_ENUM;
FOUNDATION_EXPORT TTImageMediaBuilder const TTImageMediaImageJpegBuilder;
FOUNDATION_EXPORT TTImageMediaBuilder const TTImageMediaImagePngBuilder;
typedef NSString *TTReaderMediaBuilder NS_TYPED_ENUM;
FOUNDATION_EXPORT TTReaderMediaBuilder const TTReaderMediaTextBuilder;
FOUNDATION_EXPORT TTReaderMediaBuilder const TTReaderMediaImageBuilder;
typedef NSString *TTRecognitionModeBuilder NS_TYPED_ENUM;
FOUNDATION_EXPORT TTRecognitionModeBuilder const TTRecognitionModeWholeBuilder;
FOUNDATION_EXPORT TTRecognitionModeBuilder const TTRecognitionModeBoundaryOnlyBuilder;
typedef NSString *TTRequestFraming NS_TYPED_ENUM;
FOUNDATION_EXPORT TTRequestFraming const TTRequestFramingDocument;
FOUNDATION_EXPORT TTRequestFraming const TTRequestFramingLines;
FOUNDATION_EXPORT TTRequestFraming const TTRequestFramingJsonl;
FOUNDATION_EXPORT TTRequestFraming const TTRequestFramingCsv;
FOUNDATION_EXPORT TTRequestFraming const TTRequestFramingTsv;
typedef NSString *TTRequestFunction NS_TYPED_ENUM;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionDecide;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionChoose;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionTag;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionScore;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionFilter;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionRank;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionFind;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionAnnotate;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionRecognize;
FOUNDATION_EXPORT TTRequestFunction const TTRequestFunctionRelate;
typedef NSString *TTSourceUnitBuilder NS_TYPED_ENUM;
FOUNDATION_EXPORT TTSourceUnitBuilder const TTSourceUnitLineBuilder;
FOUNDATION_EXPORT TTSourceUnitBuilder const TTSourceUnitWindowBuilder;
FOUNDATION_EXPORT TTSourceUnitBuilder const TTSourceUnitFileBuilder;
@class TTAnnotateDefinitionBuilder, TTAnnotateQuestionChooseBuilder, TTAnnotateQuestionDecideBuilder,
TTAnnotateQuestionScoreBuilder, TTAnnotateQuestionTagBuilder, TTAuthoredChooseBuilder, TTAuthoredDecideBuilder,
TTAuthoredFindBuilder, TTAuthoredInputDeclarationObjectBuilder, TTAuthoredInputDeclarationStringBuilder,
TTAuthoredInputPropertyArrayBuilder, TTAuthoredInputPropertyBooleanBuilder, TTAuthoredInputPropertyNumberBuilder,
TTAuthoredInputPropertyStringBuilder, TTAuthoredRelateBuilder, TTAuthoredRelationBuilder, TTAuthoredScoreBuilder,
TTAuthoredTagBuilder, TTSettings, TTOptionSchemaBuilder, TTRecognitionExampleEntityBuilder,
TTRecognitionExampleTextBuilder, TTRecognitionSeedSpanBuilder, TTRecognitionStageContextBuilder,
TTRecognizeDefinitionBuilder, TTRecognizeQuestionBuilder, TTRelateFieldsBuilder, TTRelateQuestionBuilder,
TTRequest, TTRequestCall, TTRequestImageBytes, TTRequestImageFile, TTEntitiesInput, TTFeedInput,
TTJSONInput, TTRecordsInput, TTSourceInput, TTTextInput, TTUnitsInput, TTItem, TTControls,
TTRequestOriginalJson, TTRequestOriginalText, TTQuestionDefinition, TTQuestionFile, TTQuestionName,
TTQuestionReference, TTTextQuestion, TTRequestReader, TTRequestReaderFailureInvalidInput, TTRequestReaderFailureIo,
TTRequestReaderFailureProducer, TTRequestReaderFailureUtf8, TTRequestSessionDescriptor, TTFiles,
TTSessionSourceLocationBuilder;
@interface TTAnnotateDefinitionBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id batch;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) TTOrderedMap * questions;
@property(nonatomic, strong, nullable) id threshold;
@end
@interface TTAnnotateQuestionChooseBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id choose;
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) id options;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTAnnotateQuestionDecideBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id decide;
@property(nonatomic, strong, nullable) id falseValue;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) id trueValue;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTAnnotateQuestionScoreBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) id levels;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) id score;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTAnnotateQuestionTagBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) id labels;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) id tag;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTAuthoredChooseBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id batch;
@property(nonatomic, strong, nullable) id choose;
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) id options;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTAuthoredDecideBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id batch;
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id decide;
@property(nonatomic, strong, nullable) id falseValue;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) id trueValue;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTAuthoredFindBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id find;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTAuthoredInputDeclarationObjectBuilder : TTRequestObject
@property(nonatomic, strong, nullable) NSDictionary * properties;
@property(nonatomic, strong, nullable) NSArray * required;
@end
@interface TTAuthoredInputDeclarationStringBuilder : TTRequestObject
@end
@interface TTAuthoredInputPropertyArrayBuilder : TTRequestObject
@property(nonatomic, strong, nullable) NSDictionary * items;
@end
@interface TTAuthoredInputPropertyBooleanBuilder : TTRequestObject
@end
@interface TTAuthoredInputPropertyNumberBuilder : TTRequestObject
@end
@interface TTAuthoredInputPropertyStringBuilder : TTRequestObject
@end
@interface TTAuthoredRelateBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) TTRelateQuestionBuilder * relate;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTAuthoredRelationBuilder : TTRequestObject
@property(nonatomic, strong, nullable) NSNumber * either;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) NSString * reads;
@property(nonatomic, strong, nullable) NSNumber * single;
@property(nonatomic, strong, nullable) NSString * source;
@property(nonatomic, strong, nullable) NSString * target;
@end
@interface TTAuthoredScoreBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id batch;
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) id levels;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) id score;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTAuthoredTagBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id batch;
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) id labels;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) id tag;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTSettings : TTRequestObject
@property(nonatomic, strong, nullable) NSString * backend;
@property(nonatomic, strong, nullable) NSString * baseUrl;
@property(nonatomic, strong, nullable) id batch;
@property(nonatomic, strong, nullable) id cache;
@property(nonatomic, strong, nullable) id maxEstimatedInputTokensTotal;
@property(nonatomic, strong, nullable) NSNumber * maxRequestBytes;
@property(nonatomic, strong, nullable) id maxRequests;
@property(nonatomic, strong, nullable) id maxRequestsTotal;
@property(nonatomic, strong, nullable) NSNumber * maxRetries;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) id proxy;
@property(nonatomic, strong, nullable) NSString * record;
@property(nonatomic, strong, nullable) NSNumber * refreshCache;
@property(nonatomic, strong, nullable) NSString * replay;
@property(nonatomic, strong, nullable) NSNumber * throttle;
@property(nonatomic, strong, nullable) NSNumber * timeout;
@property(nonatomic, strong, nullable) NSString * usdPerMillionInput;
@property(nonatomic, strong, nullable) NSString * usdPerMillionOutput;
@end
@interface TTOptionSchemaBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id descriptionValue;
@property(nonatomic, strong, nullable) NSString * name;
@end
@interface TTRecognitionExampleEntityBuilder : TTRequestObject
@property(nonatomic, strong, nullable) NSNumber * end;
@property(nonatomic, strong, nullable) NSString * kind;
@property(nonatomic, strong, nullable) NSNumber * start;
@end
@interface TTRecognitionExampleTextBuilder : TTRequestObject
@property(nonatomic, strong, nullable) NSArray * entities;
@property(nonatomic, strong, nullable) NSArray * kinds;
@property(nonatomic, strong, nullable) NSString * text;
@end
@interface TTRecognitionSeedSpanBuilder : TTRequestObject
@property(nonatomic, strong, nullable) NSNumber * end;
@property(nonatomic, strong, nullable) NSString * kind;
@property(nonatomic, strong, nullable) NSNumber * start;
@end
@interface TTRecognitionStageContextBuilder : TTRequestObject
@property(nonatomic, strong, nullable) NSString * boundary;
@property(nonatomic, strong, nullable) NSString * kindEdge;
@property(nonatomic, strong, nullable) NSString * relation;
@end
@interface TTRecognizeDefinitionBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) TTRecognizeQuestionBuilder * recognize;
@property(nonatomic, strong, nullable) id relationThreshold;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTRecognizeQuestionBuilder : TTRequestObject
@property(nonatomic, strong, nullable) id entityDefinition;
@property(nonatomic, strong, nullable) id instructions;
@property(nonatomic, strong, nullable) TTOrderedMap * kinds;
@property(nonatomic, strong, nullable) TTRecognitionModeBuilder mode;
@property(nonatomic, strong, nullable) NSArray * relations;
@property(nonatomic, strong, nullable) NSNumber * snippetPieces;
@property(nonatomic, strong, nullable) TTRecognitionStageContextBuilder * stageContext;
@end
@interface TTRelateFieldsBuilder : TTRequestObject
@property(nonatomic, strong, nullable) NSString * kind;
@property(nonatomic, strong, nullable) NSString * name;
@end
@interface TTRelateQuestionBuilder : TTRequestObject
@property(nonatomic, strong, nullable) TTRelateFieldsBuilder * fields;
@property(nonatomic, strong, nullable) NSArray * relations;
@end
@interface TTRequest : TTRequestObject
@property(nonatomic, strong, nullable) TTRequestCall * call;
@end
@interface TTRequestCall : TTRequestObject
@property(nonatomic, strong, nullable) TTRequestFunction function;
@property(nonatomic, strong, nullable) TTInput * input;
@property(nonatomic, strong, nullable) TTControls * options;
@property(nonatomic, strong, nullable) TTQuestion * question;
@end
@interface TTRequestImageBytes : TTRequestObject
@property(nonatomic, strong, nullable) NSString * bytes;
@property(nonatomic, strong, nullable) TTImageMediaBuilder media;
@end
@interface TTRequestImageFile : TTRequestObject
@property(nonatomic, strong, nullable) TTImageMediaBuilder media;
@property(nonatomic, strong, nullable) NSString * path;
@end
@interface TTEntitiesInput : TTInput
@property(nonatomic, strong, nullable) NSArray * items;
@end
@interface TTFeedInput : TTInput
@property(nonatomic, strong, nullable) TTRequestFraming framing;
@property(nonatomic, strong, nullable) NSArray * images;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) TTRequestReader * reading;
@end
@interface TTJSONInput : TTInput
@property(nonatomic, strong, nullable) NSArray * images;
@property(nonatomic, strong, nullable) id value;
@end
@interface TTRecordsInput : TTInput
@property(nonatomic, strong, nullable) NSArray * items;
@end
@interface TTSourceInput : TTInput
@property(nonatomic, strong, nullable) TTFiles * source;
@end
@interface TTTextInput : TTInput
@property(nonatomic, strong, nullable) NSArray * images;
@property(nonatomic, strong, nullable) NSString * text;
@end
@interface TTUnitsInput : TTInput
@property(nonatomic, strong, nullable) NSArray * items;
@end
@interface TTItem : TTRequestObject
@property(nonatomic, strong, nullable) id context;
@property(nonatomic, strong, nullable) NSArray * examples;
@property(nonatomic, strong, nullable) NSArray * images;
@property(nonatomic, strong, nullable) NSArray * options;
@property(nonatomic, strong, nullable) id original;
@property(nonatomic, strong, nullable) NSArray * seedSpans;
@end
@interface TTControls : TTRequestObject
@property(nonatomic, strong, nullable) NSNumber * attempts;
@property(nonatomic, strong, nullable) id batch;
@property(nonatomic, strong, nullable) NSString * context;
@property(nonatomic, strong, nullable) NSString * contextField;
@property(nonatomic, strong, nullable) NSNumber * deadlineMs;
@property(nonatomic, strong, nullable) NSNumber * details;
@property(nonatomic, strong, nullable) NSArray * examples;
@property(nonatomic, strong, nullable) NSString * examplesField;
@property(nonatomic, strong, nullable) NSArray * field;
@property(nonatomic, strong, nullable) NSNumber * filesOnly;
@property(nonatomic, strong, nullable) NSNumber * maxRequestsTotal;
@property(nonatomic, strong, nullable) TTRecognitionModeBuilder mode;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSNumber * none;
@property(nonatomic, strong, nullable) NSString * optionsField;
@property(nonatomic, strong, nullable) id relationThreshold;
@property(nonatomic, strong, nullable) NSArray * seedSpans;
@property(nonatomic, strong, nullable) NSString * seedSpansField;
@property(nonatomic, strong, nullable) NSNumber * snippetPieces;
@property(nonatomic, strong, nullable) TTRecognitionStageContextBuilder * stageContext;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * top;
@end
@interface TTRequestOriginalJson : TTRequestObject
@property(nonatomic, strong, nullable) id value;
@end
@interface TTRequestOriginalText : TTRequestObject
@property(nonatomic, strong, nullable) NSString * text;
@end
@interface TTQuestionDefinition : TTQuestion
@property(nonatomic, strong, nullable) id value;
@end
@interface TTQuestionFile : TTQuestion
@property(nonatomic, strong, nullable) NSString * path;
@end
@interface TTQuestionName : TTQuestion
@property(nonatomic, strong, nullable) NSString * name;
@end
@interface TTQuestionReference : TTQuestion
@property(nonatomic, strong, nullable) NSString * reference;
@end
@interface TTTextQuestion : TTQuestion
@property(nonatomic, strong, nullable) NSString * text;
@end
@interface TTRequestReader : TTRequestObject
@property(nonatomic, strong, nullable) TTSourceUnitBuilder unit;
@property(nonatomic, strong, nullable) NSNumber * window;
@end
@interface TTRequestReaderFailureInvalidInput : TTRequestObject
@property(nonatomic, strong, nullable) TTSessionSourceLocationBuilder * location;
@end
@interface TTRequestReaderFailureIo : TTRequestObject
@property(nonatomic, strong, nullable) TTSessionSourceLocationBuilder * location;
@end
@interface TTRequestReaderFailureProducer : TTRequestObject
@end
@interface TTRequestReaderFailureUtf8 : TTRequestObject
@property(nonatomic, strong, nullable) TTSessionSourceLocationBuilder * location;
@end
@interface TTRequestSessionDescriptor : TTRequestObject
@property(nonatomic, strong, nullable) TTItem * item;
@property(nonatomic, strong, nullable) TTSessionSourceLocationBuilder * location;
@end
@interface TTFiles : TTRequestObject
@property(nonatomic, strong, nullable) TTRequestFraming framing;
@property(nonatomic, strong, nullable) TTReaderMediaBuilder media;
@property(nonatomic, strong, nullable) NSArray * paths;
@property(nonatomic, strong, nullable) TTRequestReader * reading;
@end
@interface TTSessionSourceLocationBuilder : TTRequestObject
@property(nonatomic, strong, nullable) NSString * file;
@property(nonatomic, strong, nullable) NSNumber * firstLine;
@property(nonatomic, strong, nullable) NSNumber * lastLine;
@end
@class TTCall, TTSession, TTTask, TTUsagePersistenceStatus;
typedef void (^TTCompletion)(TTCall * _Nullable call, NSError * _Nullable error);
typedef TTRequestSessionDescriptor * _Nullable (^TTFeed)(TTRequestObject * _Nullable * _Nonnull readerFailure);
@interface TTCall : NSObject
@property(nonatomic, strong, readonly) TTPresence<TTFacts *> *facts;
@end
@interface TTCall (Answers)
- (nullable TTPresence<NSNumber *> *)decideAnswer:(NSError * _Nullable * _Nullable)error;
- (nullable TTPresence<NSString *> *)chooseAnswer:(NSError * _Nullable * _Nullable)error;
- (nullable TTPresence<NSArray<NSString *> *> *)tagAnswer:(NSError * _Nullable * _Nullable)error;
- (nullable TTPresence<NSNumber *> *)scoreAnswer:(NSError * _Nullable * _Nullable)error;
- (nullable NSArray *)filterAnswer:(NSError * _Nullable * _Nullable)error;
- (nullable NSArray<NSNumber *> *)rankAnswer:(NSError * _Nullable * _Nullable)error;
- (nullable TTPresence<TTFind *> *)findAnswer:(NSError * _Nullable * _Nullable)error;
- (nullable TTPresence<NSArray<TTAnnotation *> *> *)annotateAnswer:(NSError * _Nullable * _Nullable)error;
- (nullable TTPresence<NSArray<TTRecognition *> *> *)recognizeAnswer:(NSError * _Nullable * _Nullable)error;
- (nullable TTPresence<TTRelation *> *)relateAnswer:(NSError * _Nullable * _Nullable)error;
@end
@interface TTFoundationClient : NSObject
- (nullable instancetype)initWithSettings:(nullable TTSettings *)settings error:(NSError * _Nullable * _Nullable)error NS_DESIGNATED_INITIALIZER;
- (instancetype)init NS_UNAVAILABLE;
+ (instancetype)new NS_UNAVAILABLE;
- (nullable TTPlan *)plan:(TTRequest *)request error:(NSError * _Nullable * _Nullable)error;
- (nullable TTSession *)startRequest:(TTRequest *)request error:(NSError * _Nullable * _Nullable)error;
- (nullable TTUsagePersistenceStatus *)usagePersistence:(NSError * _Nullable * _Nullable)error;
- (nullable TTUsagePersistenceStatus *)finishUsageStatus:(NSError * _Nullable * _Nullable)error;
@end
@interface TTFoundationClient (NamedCalls)
- (nullable TTTask *)decide:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)choose:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)tag:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)score:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)filter:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)rank:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)find:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)annotate:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)recognize:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
- (nullable TTTask *)relate:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error;
@end
FOUNDATION_EXPORT NSErrorDomain const TTErrorDomain;
typedef NS_ERROR_ENUM(TTErrorDomain, TTErrorCode) {
    TTErrorUsage = 1,
    TTErrorBackend = 2,
    TTErrorLocal = 4,
    TTErrorCancelled = 5,
    TTErrorDeadline = 3,
    TTErrorDefect = 6,
};
typedef NS_ENUM(uint32_t, TTUsagePersistenceState) {
    TTUsagePersistenceDisabled = 1,
    TTUsagePersistenceFailed = 4,
    TTUsagePersistencePending = 2,
    TTUsagePersistenceWritten = 3,
};
#define TT_GENERATED_USAGE_DECLARATIONS - (nullable TTUsagePersistenceStatus *)finishUsageStatus:(NSError * _Nullable * _Nullable)error; - (nullable TTUsagePersistenceStatus *)usagePersistence:(NSError * _Nullable * _Nullable)error;
NS_ASSUME_NONNULL_END
