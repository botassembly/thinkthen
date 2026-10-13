// Generated from the shared Rust result graph; do not edit.
#import "TTResult.h"
NS_ASSUME_NONNULL_BEGIN
FOUNDATION_EXPORT NSString * const TTRequestVersion;
FOUNDATION_EXPORT NSInteger TTNativeErrorCode(NSString *kind);
@class TTAnnotation, TTAnnotationMember, TTAnnotationMemberAnswerId, TTAnnotationMemberFailureId,
TTAnnotationValue, TTAnnotationValueChoice, TTAnnotationValueDecision, TTAnnotationValueFailed,
TTAnnotationValueScore, TTAnnotationValueTags, TTAnswers, TTAtomicArrayOfString, TTAtomicDecideValue,
TTAtomicNonZeroUsize, TTAtomicNullableString, TTAtomicBoolean, TTAtomicDouble, TTAttempt, TTBatch,
TTBoundaryOdds, TTBoundaryProposal, TTCallError, TTEntityDocument, TTError, TTEstimatedInputDenial,
TTEstimatedInputDenialAdditionalRequest, TTEstimatedInputDenialInitialRequest, TTEstimatedInputDenialRetry,
TTFacts, TTFind, TTFindCandidate, TTImage, TTInputDeclaration, TTInputDeclarationObjectRoot,
TTInputDeclarationStringRoot, TTInputPropertyType, TTInputPropertyTypeArray, TTInputPropertyTypeBoolean,
TTInputPropertyTypeNumber, TTInputPropertyTypeString, TTLabel, TTMeta, TTObjectRoot, TTObservation,
TTObservationFailureId, TTObservationObservationId, TTPersistenceObservation, TTPhysicalSource,
TTPosition, TTQuestionSource, TTRankMember, TTRankMemberResult, TTReadableQuestion, TTReadableQuestion2,
TTReadableQuestion3, TTReadableQuestion4, TTReadableQuestionChoose, TTReadableQuestionDecide,
TTReadableQuestionScore, TTReadableQuestionTag, TTRecognition, TTRecognitionEdgeDocument, TTRecognitionOdds,
TTRecognitionOddsBoundaryOdds, TTRecognitionOddsRecognizeAnswer, TTRecognitionProposal, TTRecognitionStageContext,
TTRelation, TTRelationEndpoint, TTRelationEndpointRelatedEntity, TTRelationEndpointSource,
TTRelationMember, TTRelationMemberAnswerId, TTRelationMemberFailureId, TTSendBudgetDenial,
TTSendBudgetDenialBeforeAdditionalSend, TTSendBudgetDenialBeforeFirstSend, TTSendBudgetDenialBeforeRetry,
TTStopped, TTStringRoot, TTUsage, TTAnnotatedField, TTAnswer, TTAnswerChoice, TTAnswerScore,
TTAnswerTag, TTAnswerYesNo, TTBatchSetting, TTBatchWarning, TTEntity, TTEntityEdge, TTFailed,
TTFailure, TTFindAnswer, TTNameOdds, TTPairOdds, TTPieceOdds, TTPlace, TTPlan, TTProfileWarning,
TTRecognize, TTRecognizeAnswer, TTRecognizeEntities, TTRecognizeModeProposals, TTRelateFields,
TTRelatedEntity, TTRelatedEntityEdge, TTRelationRule, TTSessionAnnotation, TTSessionInputSource,
TTSessionJudgment, TTSessionJudgmentChoice, TTSessionJudgmentDecision, TTSessionJudgmentScore,
TTSessionJudgmentTags, TTSessionNamedProbability, TTSessionObservation, TTSessionObservationQuestion,
TTSessionObservationRow, TTSessionObservedRow, TTSessionObservedRowAnnotated, TTSessionObservedRowFind,
TTSessionObservedRowJudgment, TTSessionObservedRowRecognized, TTSessionObservedRowRelations,
TTSessionPacket, TTSessionPacketAnnotateAggregate, TTSessionPacketAnnotateRow, TTSessionPacketChooseAggregate,
TTSessionPacketChooseRow, TTSessionPacketDecideAggregate, TTSessionPacketDecideRow, TTSessionPacketFilterAggregate,
TTSessionPacketFilterRow, TTSessionPacketFindAggregate, TTSessionPacketObservation, TTSessionPacketRankAggregate,
TTSessionPacketRecognizeAggregate, TTSessionPacketRelateAggregate, TTSessionPacketScoreAggregate,
TTSessionPacketScoreRow, TTSessionPacketTagAggregate, TTSessionPacketTagRow, TTSessionPacketTerminal,
TTSessionProbabilities, TTSessionProbabilitiesNamed, TTSessionProbabilitiesYesNo, TTSessionQuestionDetail,
TTSessionRecognition, TTSessionRelationEdge, TTSourceRelationEndpoint, TTThreshold, TTTokenBand,
TTTokenUsage, TTValue;
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
+ (instancetype)read:(id)value;
@end
@interface TTAnnotationMember : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTAnnotationValue : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTAnswers : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTRelationMember *> *> *questions;
+ (instancetype)read:(id)value;
@end
@interface TTAtomicArrayOfString : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAtomicDecideValue : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<id> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAtomicNonZeroUsize : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAtomicNullableString : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSString *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAtomicBoolean : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAtomicDouble : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSArray<TTRankMember *> *> *members;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *questionName;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
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
+ (instancetype)read:(id)value;
@end
@interface TTBatch : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
+ (instancetype)read:(id)value;
@end
@interface TTBoundaryOdds : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
+ (instancetype)read:(id)value;
@end
@interface TTBoundaryProposal : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSNumber *> *length;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
@property(nonatomic, readonly) TTPresence<NSString *> *text;
+ (instancetype)read:(id)value;
@end
@interface TTCallError : TTResultNode
@property(nonatomic, readonly) TTPresence<TTError *> *error;
@property(nonatomic, readonly) TTPresence<TTFacts *> *facts;
+ (instancetype)read:(id)value;
@end
@interface TTEntityDocument : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
+ (instancetype)read:(id)value;
@end
@interface TTError : TTResultNode
@property(nonatomic, readonly) TTPresence<TTEstimatedInputDenial *> *estimatedInputDenial;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *message;
@property(nonatomic, readonly) TTPresence<NSNumber *> *retryable;
@property(nonatomic, readonly) TTPresence<TTSendBudgetDenial *> *sendBudgetDenial;
@property(nonatomic, readonly) TTPresence<TTStopped *> *stopped;
+ (instancetype)read:(id)value;
@end
@interface TTEstimatedInputDenial : TTResultNode
+ (instancetype)read:(id)value;
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
+ (instancetype)read:(id)value;
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
@property(nonatomic, readonly) TTPresence<TTReadableQuestion2 *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<id> *threshold;
@property(nonatomic, readonly) TTPresence<id> *value;
+ (instancetype)read:(id)value;
@end
@interface TTFindCandidate : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<id> *input;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
+ (instancetype)read:(id)value;
@end
@interface TTImage : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *base64;
@property(nonatomic, readonly) TTPresence<NSNumber *> *height;
@property(nonatomic, readonly) TTPresence<NSString *> *media;
@property(nonatomic, readonly) TTPresence<NSNumber *> *width;
+ (instancetype)read:(id)value;
@end
@interface TTInputDeclaration : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTInputPropertyType : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTLabel : TTResultNode
@property(nonatomic, readonly) TTPresence<id> *descriptionValue;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
+ (instancetype)read:(id)value;
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
+ (instancetype)read:(id)value;
@end
@interface TTObjectRoot : TTResultNode
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, TTInputPropertyType *> *> *properties;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *required;
@property(nonatomic, readonly) TTPresence<NSString *> *type;
+ (instancetype)read:(id)value;
@end
@interface TTObservation : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTPersistenceObservation : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *advice;
@property(nonatomic, readonly) TTPresence<NSString *> *observedAt;
@property(nonatomic, readonly) TTPresence<NSString *> *state;
+ (instancetype)read:(id)value;
@end
@interface TTPhysicalSource : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
+ (instancetype)read:(id)value;
@end
@interface TTPosition : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *first;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *images;
@property(nonatomic, readonly) TTPresence<NSNumber *> *last;
+ (instancetype)read:(id)value;
@end
@interface TTQuestionSource : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *answeredBy;
@property(nonatomic, readonly) TTPresence<NSNumber *> *batchSize;
@property(nonatomic, readonly) TTPresence<NSString *> *origin;
+ (instancetype)read:(id)value;
@end
@interface TTRankMember : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<TTRankMemberResult *> *result;
+ (instancetype)read:(id)value;
@end
@interface TTRankMemberResult : TTResultNode
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTImage *> *> *images;
@property(nonatomic, readonly) TTPresence<TTMeta *> *meta;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<id> *threshold;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTReadableQuestion : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTReadableQuestion2 : TTResultNode
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSNumber *> *none;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<id> *text;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
+ (instancetype)read:(id)value;
@end
@interface TTReadableQuestion3 : TTResultNode
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *contextSchema;
@property(nonatomic, readonly) TTPresence<id> *entityDefinition;
@property(nonatomic, readonly) TTPresence<id> *instructions;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *itemSchema;
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
+ (instancetype)read:(id)value;
@end
@interface TTReadableQuestion4 : TTResultNode
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTRelateFields *> *fields;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<NSArray<TTRelationRule *> *> *relations;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
+ (instancetype)read:(id)value;
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
@property(nonatomic, readonly) TTPresence<TTReadableQuestion3 *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
@property(nonatomic, readonly) TTPresence<TTRecognize *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTRecognitionEdgeDocument : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTEntity *> *source;
@property(nonatomic, readonly) TTPresence<TTEntity *> *target;
+ (instancetype)read:(id)value;
@end
@interface TTRecognitionOdds : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTRecognitionProposal : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSNumber *> *kept;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTPlace *> *selected;
@property(nonatomic, readonly) TTPresence<NSNumber *> *spanProbability;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
@property(nonatomic, readonly) TTPresence<NSNumber *> *strength;
+ (instancetype)read:(id)value;
@end
@interface TTRecognitionStageContext : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *boundary;
@property(nonatomic, readonly) TTPresence<NSString *> *kindEdge;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
+ (instancetype)read:(id)value;
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
@property(nonatomic, readonly) TTPresence<TTReadableQuestion4 *> *question;
@property(nonatomic, readonly) TTPresence<NSString *> *schema;
@property(nonatomic, readonly) TTPresence<NSArray<TTRelatedEntityEdge *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTRelationEndpoint : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTRelationMember : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTSendBudgetDenial : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTStopped : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *at;
@property(nonatomic, readonly) TTPresence<NSString *> *cause;
@property(nonatomic, readonly) TTPresence<NSNumber *> *retryable;
@property(nonatomic, readonly) TTPresence<NSNumber *> *status;
+ (instancetype)read:(id)value;
@end
@interface TTStringRoot : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *type;
+ (instancetype)read:(id)value;
@end
@interface TTUsage : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *inputTokens;
@property(nonatomic, readonly) TTPresence<NSNumber *> *outputTokens;
+ (instancetype)read:(id)value;
@end
@interface TTAnnotatedField : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
+ (instancetype)read:(id)value;
@end
@interface TTAnswer : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTBatchSetting : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
+ (instancetype)read:(id)value;
@end
@interface TTBatchWarning : TTResultNode
@property(nonatomic, readonly) TTPresence<TTBatchSetting *> *running;
@property(nonatomic, readonly) TTPresence<TTBatchSetting *> *tunedFor;
+ (instancetype)read:(id)value;
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
+ (instancetype)read:(id)value;
@end
@interface TTEntityEdge : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTEntity *> *source;
@property(nonatomic, readonly) TTPresence<TTEntity *> *target;
+ (instancetype)read:(id)value;
@end
@interface TTFailed : TTResultNode
@property(nonatomic, readonly) TTPresence<TTFailure *> *failed;
+ (instancetype)read:(id)value;
@end
@interface TTFailure : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *cause;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
+ (instancetype)read:(id)value;
@end
@interface TTFindAnswer : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *confidence;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *pick;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *probabilities;
+ (instancetype)read:(id)value;
@end
@interface TTNameOdds : TTResultNode
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *edges;
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *kinds;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
+ (instancetype)read:(id)value;
@end
@interface TTPairOdds : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTPlace *> *source;
@property(nonatomic, readonly) TTPresence<TTPlace *> *target;
+ (instancetype)read:(id)value;
@end
@interface TTPieceOdds : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *tags;
+ (instancetype)read:(id)value;
@end
@interface TTPlace : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *end;
@property(nonatomic, readonly) TTPresence<NSNumber *> *start;
+ (instancetype)read:(id)value;
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
+ (instancetype)read:(id)value;
@end
@interface TTProfileWarning : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *running;
@property(nonatomic, readonly) TTPresence<NSString *> *tunedFor;
+ (instancetype)read:(id)value;
@end
@interface TTRecognize : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTRecognizeAnswer : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTNameOdds *> *> *names;
@property(nonatomic, readonly) TTPresence<NSArray<TTPairOdds *> *> *pairs;
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognitionProposal *> *> *proposals;
+ (instancetype)read:(id)value;
@end
@interface TTRelateFields : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
+ (instancetype)read:(id)value;
@end
@interface TTRelatedEntity : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
+ (instancetype)read:(id)value;
@end
@interface TTRelatedEntityEdge : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTRelationEndpoint *> *source;
@property(nonatomic, readonly) TTPresence<TTRelationEndpoint *> *target;
@property(nonatomic, readonly) TTPresence<NSString *> *sourceName;
@property(nonatomic, readonly) TTPresence<NSString *> *targetName;
+ (instancetype)read:(id)value;
@end
@interface TTRelationRule : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSString *> *reads;
@property(nonatomic, readonly) TTPresence<NSNumber *> *single;
@property(nonatomic, readonly) TTPresence<NSString *> *source;
@property(nonatomic, readonly) TTPresence<NSString *> *target;
+ (instancetype)read:(id)value;
@end
@interface TTSessionAnnotation : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<TTAnnotationValue *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionInputSource : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<TTPhysicalSource *> *source;
+ (instancetype)read:(id)value;
@end
@interface TTSessionJudgment : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTSessionNamedProbability : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
+ (instancetype)read:(id)value;
@end
@interface TTSessionObservation : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTSessionObservedRow : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacket : TTResultNode
+ (instancetype)read:(id)value;
@end
@interface TTSessionProbabilities : TTResultNode
+ (instancetype)read:(id)value;
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
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
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
+ (instancetype)read:(id)value;
@end
@interface TTSessionRecognition : TTResultNode
@property(nonatomic, readonly) TTPresence<NSArray<TTEntity *> *> *entities;
@property(nonatomic, readonly) TTPresence<NSString *> *mode;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognitionEdgeDocument *> *> *relations;
+ (instancetype)read:(id)value;
@end
@interface TTSessionRelationEdge : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *either;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
@property(nonatomic, readonly) TTPresence<NSString *> *relation;
@property(nonatomic, readonly) TTPresence<TTEntityDocument *> *source;
@property(nonatomic, readonly) TTPresence<TTEntityDocument *> *target;
@property(nonatomic, readonly) TTPresence<NSString *> *sourceName;
@property(nonatomic, readonly) TTPresence<NSString *> *targetName;
+ (instancetype)read:(id)value;
@end
@interface TTSourceRelationEndpoint : TTResultNode
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSNumber *> *ordinal;
@property(nonatomic, readonly) TTPresence<id> *record;
+ (instancetype)read:(id)value;
@end
@interface TTThreshold : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
+ (instancetype)read:(id)value;
@end
@interface TTTokenBand : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *lower;
@property(nonatomic, readonly) TTPresence<NSNumber *> *upper;
+ (instancetype)read:(id)value;
@end
@interface TTTokenUsage : TTResultNode
@property(nonatomic, readonly) TTPresence<NSNumber *> *inputTokens;
@property(nonatomic, readonly) TTPresence<NSNumber *> *outputTokens;
+ (instancetype)read:(id)value;
@end
@interface TTValue : TTResultNode
@property(nonatomic, readonly) id value;
@property(nonatomic, readonly) NSString *kind;
+ (instancetype)read:(id)value;
@end
@interface TTAnnotationMemberAnswerId : TTAnnotationMember
@property(nonatomic, readonly) TTPresence<TTAnswer *> *answer;
@property(nonatomic, readonly) TTPresence<NSString *> *answerId;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
@property(nonatomic, readonly) TTPresence<NSArray<TTQuestionSource *> *> *questionSources;
@property(nonatomic, readonly) TTPresence<NSString *> *request;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<TTUsage *> *usage;
@property(nonatomic, readonly) TTPresence<TTValue *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAnnotationMemberFailureId : TTAnnotationMember
@property(nonatomic, readonly) TTPresence<TTFailure *> *failure;
@property(nonatomic, readonly) TTPresence<NSString *> *failureId;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
@property(nonatomic, readonly) TTPresence<NSArray<TTQuestionSource *> *> *questionSources;
@property(nonatomic, readonly) TTPresence<NSString *> *request;
@property(nonatomic, readonly) TTPresence<TTThreshold *> *threshold;
@property(nonatomic, readonly) TTPresence<TTUsage *> *usage;
+ (instancetype)read:(id)value;
@end
@interface TTAnnotationValueChoice : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAnnotationValueDecision : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAnnotationValueFailed : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTFailure *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAnnotationValueScore : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTAnnotationValueTags : TTAnnotationValue
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTEstimatedInputDenialAdditionalRequest : TTEstimatedInputDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *limit;
+ (instancetype)read:(id)value;
@end
@interface TTEstimatedInputDenialInitialRequest : TTEstimatedInputDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *limit;
+ (instancetype)read:(id)value;
@end
@interface TTEstimatedInputDenialRetry : TTEstimatedInputDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastStatus;
@property(nonatomic, readonly) TTPresence<NSNumber *> *limit;
+ (instancetype)read:(id)value;
@end
@interface TTInputDeclarationObjectRoot : TTInputDeclaration
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, TTInputPropertyType *> *> *properties;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *required;
@property(nonatomic, readonly) TTPresence<NSString *> *type;
+ (instancetype)read:(id)value;
@end
@interface TTInputDeclarationStringRoot : TTInputDeclaration
@property(nonatomic, readonly) TTPresence<NSString *> *type;
+ (instancetype)read:(id)value;
@end
@interface TTInputPropertyTypeArray : TTInputPropertyType
@property(nonatomic, readonly) TTPresence<TTStringRoot *> *items;
@property(nonatomic, readonly) TTPresence<NSString *> *type;
+ (instancetype)read:(id)value;
@end
@interface TTInputPropertyTypeBoolean : TTInputPropertyType
@property(nonatomic, readonly) TTPresence<NSString *> *type;
+ (instancetype)read:(id)value;
@end
@interface TTInputPropertyTypeNumber : TTInputPropertyType
@property(nonatomic, readonly) TTPresence<NSString *> *type;
+ (instancetype)read:(id)value;
@end
@interface TTInputPropertyTypeString : TTInputPropertyType
@property(nonatomic, readonly) TTPresence<NSString *> *type;
+ (instancetype)read:(id)value;
@end
@interface TTObservationFailureId : TTObservation
@property(nonatomic, readonly) TTPresence<NSString *> *failureId;
+ (instancetype)read:(id)value;
@end
@interface TTObservationObservationId : TTObservation
@property(nonatomic, readonly) TTPresence<NSString *> *observationId;
+ (instancetype)read:(id)value;
@end
@interface TTReadableQuestionChoose : TTReadableQuestion
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *options;
@property(nonatomic, readonly) TTPresence<id> *text;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
+ (instancetype)read:(id)value;
@end
@interface TTReadableQuestionDecide : TTReadableQuestion
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *itemSchema;
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
+ (instancetype)read:(id)value;
@end
@interface TTReadableQuestionScore : TTReadableQuestion
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *levels;
@property(nonatomic, readonly) TTPresence<id> *text;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
+ (instancetype)read:(id)value;
@end
@interface TTReadableQuestionTag : TTReadableQuestion
@property(nonatomic, readonly) TTPresence<TTBatch *> *batch;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *contextSchema;
@property(nonatomic, readonly) TTPresence<TTInputDeclaration *> *itemSchema;
@property(nonatomic, readonly) TTPresence<NSArray<TTLabel *> *> *labelDetails;
@property(nonatomic, readonly) TTPresence<NSString *> *model;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *on;
@property(nonatomic, readonly) TTPresence<NSString *> *profile;
@property(nonatomic, readonly) TTPresence<NSNumber *> *wordingVersion;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *labels;
@property(nonatomic, readonly) TTPresence<id> *text;
@property(nonatomic, readonly) TTPresence<NSString *> *verb;
+ (instancetype)read:(id)value;
@end
@interface TTRecognitionOddsBoundaryOdds : TTRecognitionOdds
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
+ (instancetype)read:(id)value;
@end
@interface TTRecognitionOddsRecognizeAnswer : TTRecognitionOdds
@property(nonatomic, readonly) TTPresence<NSArray<TTNameOdds *> *> *names;
@property(nonatomic, readonly) TTPresence<NSArray<TTPairOdds *> *> *pairs;
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognitionProposal *> *> *proposals;
+ (instancetype)read:(id)value;
@end
@interface TTRelationEndpointRelatedEntity : TTRelationEndpoint
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
+ (instancetype)read:(id)value;
@end
@interface TTRelationEndpointSource : TTRelationEndpoint
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSNumber *> *ordinal;
@property(nonatomic, readonly) TTPresence<id> *record;
+ (instancetype)read:(id)value;
@end
@interface TTRelationMemberAnswerId : TTRelationMember
@property(nonatomic, readonly) TTPresence<NSString *> *direction;
@property(nonatomic, readonly) TTPresence<NSString *> *method;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
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
+ (instancetype)read:(id)value;
@end
@interface TTRelationMemberFailureId : TTRelationMember
@property(nonatomic, readonly) TTPresence<NSString *> *direction;
@property(nonatomic, readonly) TTPresence<NSString *> *method;
@property(nonatomic, readonly) TTPresence<NSArray<TTObservation *> *> *observations;
@property(nonatomic, readonly) TTPresence<TTReadableQuestion *> *question;
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
+ (instancetype)read:(id)value;
@end
@interface TTSendBudgetDenialBeforeAdditionalSend : TTSendBudgetDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
+ (instancetype)read:(id)value;
@end
@interface TTSendBudgetDenialBeforeFirstSend : TTSendBudgetDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
+ (instancetype)read:(id)value;
@end
@interface TTSendBudgetDenialBeforeRetry : TTSendBudgetDenial
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastStatus;
+ (instancetype)read:(id)value;
@end
@interface TTAnswerChoice : TTAnswer
@property(nonatomic, readonly) TTPresence<NSNumber *> *confidence;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *pick;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *probabilities;
+ (instancetype)read:(id)value;
@end
@interface TTAnswerScore : TTAnswer
@property(nonatomic, readonly) TTPresence<NSNumber *> *confidence;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *level;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *probabilities;
+ (instancetype)read:(id)value;
@end
@interface TTAnswerTag : TTAnswer
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, NSNumber *> *> *probabilities;
+ (instancetype)read:(id)value;
@end
@interface TTAnswerYesNo : TTAnswer
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *probability;
+ (instancetype)read:(id)value;
@end
@interface TTRecognizeEntities : TTRecognize
@property(nonatomic, readonly) TTPresence<NSArray<TTEntity *> *> *entities;
@property(nonatomic, readonly) TTPresence<NSArray<TTEntityEdge *> *> *relations;
+ (instancetype)read:(id)value;
@end
@interface TTRecognizeModeProposals : TTRecognize
@property(nonatomic, readonly) TTPresence<NSString *> *mode;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
+ (instancetype)read:(id)value;
@end
@interface TTSessionJudgmentChoice : TTSessionJudgment
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionJudgmentDecision : TTSessionJudgment
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionJudgmentScore : TTSessionJudgment
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionJudgmentTags : TTSessionJudgment
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionObservationQuestion : TTSessionObservation
@property(nonatomic, readonly) TTPresence<TTSessionQuestionDetail *> *detail;
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *member;
@property(nonatomic, readonly) TTPresence<NSNumber *> *position;
@property(nonatomic, readonly) TTPresence<NSString *> *stage;
+ (instancetype)read:(id)value;
@end
@interface TTSessionObservationRow : TTSessionObservation
@property(nonatomic, readonly) TTPresence<NSNumber *> *index;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTSessionObservedRow *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionObservedRowAnnotated : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTSessionAnnotation *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionObservedRowFind : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionObservedRowJudgment : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTSessionJudgment *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionObservedRowRecognized : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTSessionRecognition *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionObservedRowRelations : TTSessionObservedRow
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTSessionRelationEdge *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketAnnotateAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTAnnotation *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketAnnotateRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTAnnotation *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketChooseAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicNullableString *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketChooseRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTAtomicNullableString *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketDecideAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicDecideValue *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketDecideRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTAtomicDecideValue *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketFilterAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicBoolean *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketFilterRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTAtomicBoolean *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketFindAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTFind *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketObservation : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTSessionObservation *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketRankAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicNonZeroUsize *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketRecognizeAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognition *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketRelateAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTRelation *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketScoreAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicDouble *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketScoreRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTAtomicDouble *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketTagAggregate : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicArrayOfString *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketTagRow : TTSessionPacket
@property(nonatomic, readonly) TTPresence<NSString *> *function;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<TTAtomicArrayOfString *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacketTerminal : TTSessionPacket
@property(nonatomic, readonly) TTPresence<TTFacts *> *facts;
@property(nonatomic, readonly) TTPresence<TTCallError *> *failure;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
+ (instancetype)read:(id)value;
@end
@interface TTSessionProbabilitiesNamed : TTSessionProbabilities
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSArray<TTSessionNamedProbability *> *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionProbabilitiesYesNo : TTSessionProbabilities
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *value;
+ (instancetype)read:(id)value;
@end
@interface TTSessionPacket (Answers)
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicDecideValue *> *> *decideAnswer;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicNullableString *> *> *chooseAnswer;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicArrayOfString *> *> *tagAnswer;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicDouble *> *> *scoreAnswer;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicBoolean *> *> *filterAnswer;
@property(nonatomic, readonly) TTPresence<NSArray<TTAtomicNonZeroUsize *> *> *rankAnswer;
@property(nonatomic, readonly) TTPresence<TTFind *> *findAnswer;
@property(nonatomic, readonly) TTPresence<NSArray<TTAnnotation *> *> *annotateAnswer;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognition *> *> *recognizeAnswer;
@property(nonatomic, readonly) TTPresence<TTRelation *> *relateAnswer;
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
FOUNDATION_EXPORT NSData * _Nullable TTRequestJSONData(id value, NSError **error);
@interface TTRequestObject : NSObject
@property(nonatomic, readonly) NSMutableDictionary<NSString *, id> *requestFields;
@end
typedef NSString *TTInputImageMedia NS_TYPED_ENUM;
FOUNDATION_EXPORT TTInputImageMedia const TTInputImageMediaImageJpeg;
FOUNDATION_EXPORT TTInputImageMedia const TTInputImageMediaImagePng;
typedef NSString *TTInputReaderMedia NS_TYPED_ENUM;
FOUNDATION_EXPORT TTInputReaderMedia const TTInputReaderMediaText;
FOUNDATION_EXPORT TTInputReaderMedia const TTInputReaderMediaImage;
typedef NSString *TTInputRecognitionMode NS_TYPED_ENUM;
FOUNDATION_EXPORT TTInputRecognitionMode const TTInputRecognitionModeWhole;
FOUNDATION_EXPORT TTInputRecognitionMode const TTInputRecognitionModeBoundaryOnly;
typedef NSString *TTInputRequestFraming NS_TYPED_ENUM;
FOUNDATION_EXPORT TTInputRequestFraming const TTInputRequestFramingDocument;
FOUNDATION_EXPORT TTInputRequestFraming const TTInputRequestFramingLines;
FOUNDATION_EXPORT TTInputRequestFraming const TTInputRequestFramingJsonl;
FOUNDATION_EXPORT TTInputRequestFraming const TTInputRequestFramingCsv;
FOUNDATION_EXPORT TTInputRequestFraming const TTInputRequestFramingTsv;
typedef NSString *TTInputRequestFunction NS_TYPED_ENUM;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionDecide;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionChoose;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionTag;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionScore;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionFilter;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionRank;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionFind;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionAnnotate;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionRecognize;
FOUNDATION_EXPORT TTInputRequestFunction const TTInputRequestFunctionRelate;
typedef NSString *TTInputRequestVersion NS_TYPED_ENUM;
FOUNDATION_EXPORT TTInputRequestVersion const TTInputRequestVersionThinkthenRequest1;
typedef NSString *TTInputSourceUnit NS_TYPED_ENUM;
FOUNDATION_EXPORT TTInputSourceUnit const TTInputSourceUnitLine;
FOUNDATION_EXPORT TTInputSourceUnit const TTInputSourceUnitWindow;
FOUNDATION_EXPORT TTInputSourceUnit const TTInputSourceUnitFile;
@class TTInputAnnotateDefinition, TTInputAnnotateQuestionChoose, TTInputAnnotateQuestionDecide,
TTInputAnnotateQuestionScore, TTInputAnnotateQuestionTag, TTInputAuthoredChoose, TTInputAuthoredDecide,
TTInputAuthoredFind, TTInputAuthoredInputDeclarationObject, TTInputAuthoredInputDeclarationString,
TTInputAuthoredInputPropertyArray, TTInputAuthoredInputPropertyBoolean, TTInputAuthoredInputPropertyNumber,
TTInputAuthoredInputPropertyString, TTInputAuthoredRelate, TTInputAuthoredRelation, TTInputAuthoredScore,
TTInputAuthoredTag, TTInputEngineSettings, TTInputOptionSchema, TTInputRecognitionExampleEntity,
TTInputRecognitionExampleText, TTInputRecognitionSeedSpan, TTInputRecognitionStageContext,
TTInputRecognizeDefinition, TTInputRecognizeQuestion, TTInputRelateFields, TTInputRelateQuestion,
TTInputRequest, TTInputRequestCall, TTInputRequestDefinitionAnnotateDefinition, TTInputRequestDefinitionAuthoredChoose,
TTInputRequestDefinitionAuthoredDecide, TTInputRequestDefinitionAuthoredFind, TTInputRequestDefinitionAuthoredRelate,
TTInputRequestDefinitionAuthoredScore, TTInputRequestDefinitionAuthoredTag, TTInputRequestDefinitionRecognizeDefinition,
TTInputRequestImageBytes, TTInputRequestImageFile, TTInputRequestInputEntities, TTInputRequestInputFeed,
TTInputRequestInputJson, TTInputRequestInputRecords, TTInputRequestInputSource, TTInputRequestInputText,
TTInputRequestInputUnits, TTInputRequestItem, TTInputRequestOptions, TTInputRequestOriginalJson,
TTInputRequestOriginalText, TTInputRequestQuestionDefinition, TTInputRequestQuestionFile, TTInputRequestQuestionName,
TTInputRequestQuestionReference, TTInputRequestQuestionText, TTInputRequestReader, TTInputRequestReaderFailureInvalidInput,
TTInputRequestReaderFailureIo, TTInputRequestReaderFailureProducer, TTInputRequestReaderFailureUtf8,
TTInputRequestSessionDescriptor, TTInputRequestSource, TTInputSessionSourceLocation;
@interface TTInputAnnotateDefinition : TTRequestObject
@property(nonatomic, strong, nullable) id batch;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) TTOrderedMap * questions;
@property(nonatomic, strong, nullable) id threshold;
@end
@interface TTInputAnnotateQuestionChoose : TTRequestObject
@property(nonatomic, strong, nullable) id choose;
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) id options;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTInputAnnotateQuestionDecide : TTRequestObject
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
@interface TTInputAnnotateQuestionScore : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) id levels;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) id score;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTInputAnnotateQuestionTag : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) id labels;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) id tag;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTInputAuthoredChoose : TTRequestObject
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
@interface TTInputAuthoredDecide : TTRequestObject
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
@interface TTInputAuthoredFind : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id find;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTInputAuthoredInputDeclarationObject : TTRequestObject
@property(nonatomic, strong, nullable) NSDictionary * properties;
@property(nonatomic, strong, nullable) NSArray * required;
@end
@interface TTInputAuthoredInputDeclarationString : TTRequestObject
@end
@interface TTInputAuthoredInputPropertyArray : TTRequestObject
@property(nonatomic, strong, nullable) NSDictionary * items;
@end
@interface TTInputAuthoredInputPropertyBoolean : TTRequestObject
@end
@interface TTInputAuthoredInputPropertyNumber : TTRequestObject
@end
@interface TTInputAuthoredInputPropertyString : TTRequestObject
@end
@interface TTInputAuthoredRelate : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) TTInputRelateQuestion * relate;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTInputAuthoredRelation : TTRequestObject
@property(nonatomic, strong, nullable) NSNumber * either;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) NSString * reads;
@property(nonatomic, strong, nullable) NSNumber * single;
@property(nonatomic, strong, nullable) NSString * source;
@property(nonatomic, strong, nullable) NSString * target;
@end
@interface TTInputAuthoredScore : TTRequestObject
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
@interface TTInputAuthoredTag : TTRequestObject
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
@interface TTInputEngineSettings : TTRequestObject
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
@interface TTInputOptionSchema : TTRequestObject
@property(nonatomic, strong, nullable) id descriptionValue;
@property(nonatomic, strong, nullable) NSString * name;
@end
@interface TTInputRecognitionExampleEntity : TTRequestObject
@property(nonatomic, strong, nullable) NSNumber * end;
@property(nonatomic, strong, nullable) NSString * kind;
@property(nonatomic, strong, nullable) NSNumber * start;
@end
@interface TTInputRecognitionExampleText : TTRequestObject
@property(nonatomic, strong, nullable) NSArray * entities;
@property(nonatomic, strong, nullable) NSArray * kinds;
@property(nonatomic, strong, nullable) NSString * text;
@end
@interface TTInputRecognitionSeedSpan : TTRequestObject
@property(nonatomic, strong, nullable) NSNumber * end;
@property(nonatomic, strong, nullable) NSString * kind;
@property(nonatomic, strong, nullable) NSNumber * start;
@end
@interface TTInputRecognitionStageContext : TTRequestObject
@property(nonatomic, strong, nullable) NSString * boundary;
@property(nonatomic, strong, nullable) NSString * kindEdge;
@property(nonatomic, strong, nullable) NSString * relation;
@end
@interface TTInputRecognizeDefinition : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) TTInputRecognizeQuestion * recognize;
@property(nonatomic, strong, nullable) id relationThreshold;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTInputRecognizeQuestion : TTRequestObject
@property(nonatomic, strong, nullable) id entityDefinition;
@property(nonatomic, strong, nullable) id instructions;
@property(nonatomic, strong, nullable) TTOrderedMap * kinds;
@property(nonatomic, strong, nullable) TTInputRecognitionMode mode;
@property(nonatomic, strong, nullable) NSArray * relations;
@property(nonatomic, strong, nullable) NSNumber * snippetPieces;
@property(nonatomic, strong, nullable) TTInputRecognitionStageContext * stageContext;
@end
@interface TTInputRelateFields : TTRequestObject
@property(nonatomic, strong, nullable) NSString * kind;
@property(nonatomic, strong, nullable) NSString * name;
@end
@interface TTInputRelateQuestion : TTRequestObject
@property(nonatomic, strong, nullable) TTInputRelateFields * fields;
@property(nonatomic, strong, nullable) NSArray * relations;
@end
@interface TTInputRequest : TTRequestObject
@property(nonatomic, strong, nullable) TTInputRequestCall * call;
@property(nonatomic, strong, nullable) TTInputRequestVersion schema;
@end
@interface TTInputRequestCall : TTRequestObject
@property(nonatomic, strong, nullable) TTInputRequestFunction function;
@property(nonatomic, strong, nullable) id input;
@property(nonatomic, strong, nullable) TTInputRequestOptions * options;
@property(nonatomic, strong, nullable) id question;
@end
@interface TTInputRequestDefinitionAnnotateDefinition : TTRequestObject
@property(nonatomic, strong, nullable) id batch;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) TTOrderedMap * questions;
@property(nonatomic, strong, nullable) id threshold;
@end
@interface TTInputRequestDefinitionAuthoredChoose : TTRequestObject
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
@interface TTInputRequestDefinitionAuthoredDecide : TTRequestObject
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
@interface TTInputRequestDefinitionAuthoredFind : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id find;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTInputRequestDefinitionAuthoredRelate : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) TTInputRelateQuestion * relate;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTInputRequestDefinitionAuthoredScore : TTRequestObject
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
@interface TTInputRequestDefinitionAuthoredTag : TTRequestObject
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
@interface TTInputRequestDefinitionRecognizeDefinition : TTRequestObject
@property(nonatomic, strong, nullable) id contextSchema;
@property(nonatomic, strong, nullable) id itemSchema;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) id on;
@property(nonatomic, strong, nullable) NSString * profile;
@property(nonatomic, strong, nullable) TTInputRecognizeQuestion * recognize;
@property(nonatomic, strong, nullable) id relationThreshold;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * wordingVersion;
@end
@interface TTInputRequestImageBytes : TTRequestObject
@property(nonatomic, strong, nullable) NSString * bytes;
@property(nonatomic, strong, nullable) TTInputImageMedia media;
@end
@interface TTInputRequestImageFile : TTRequestObject
@property(nonatomic, strong, nullable) TTInputImageMedia media;
@property(nonatomic, strong, nullable) NSString * path;
@end
@interface TTInputRequestInputEntities : TTRequestObject
@property(nonatomic, strong, nullable) NSArray * items;
@end
@interface TTInputRequestInputFeed : TTRequestObject
@property(nonatomic, strong, nullable) TTInputRequestFraming framing;
@property(nonatomic, strong, nullable) NSArray * images;
@property(nonatomic, strong, nullable) NSString * name;
@property(nonatomic, strong, nullable) TTInputRequestReader * reading;
@end
@interface TTInputRequestInputJson : TTRequestObject
@property(nonatomic, strong, nullable) NSArray * images;
@property(nonatomic, strong, nullable) id value;
@end
@interface TTInputRequestInputRecords : TTRequestObject
@property(nonatomic, strong, nullable) NSArray * items;
@end
@interface TTInputRequestInputSource : TTRequestObject
@property(nonatomic, strong, nullable) TTInputRequestSource * source;
@end
@interface TTInputRequestInputText : TTRequestObject
@property(nonatomic, strong, nullable) NSArray * images;
@property(nonatomic, strong, nullable) NSString * text;
@end
@interface TTInputRequestInputUnits : TTRequestObject
@property(nonatomic, strong, nullable) NSArray * items;
@end
@interface TTInputRequestItem : TTRequestObject
@property(nonatomic, strong, nullable) id context;
@property(nonatomic, strong, nullable) NSArray * examples;
@property(nonatomic, strong, nullable) NSArray * images;
@property(nonatomic, strong, nullable) NSArray * options;
@property(nonatomic, strong, nullable) id original;
@property(nonatomic, strong, nullable) NSArray * seedSpans;
@end
@interface TTInputRequestOptions : TTRequestObject
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
@property(nonatomic, strong, nullable) TTInputRecognitionMode mode;
@property(nonatomic, strong, nullable) NSString * model;
@property(nonatomic, strong, nullable) NSNumber * none;
@property(nonatomic, strong, nullable) NSString * optionsField;
@property(nonatomic, strong, nullable) id relationThreshold;
@property(nonatomic, strong, nullable) NSArray * seedSpans;
@property(nonatomic, strong, nullable) NSString * seedSpansField;
@property(nonatomic, strong, nullable) NSNumber * snippetPieces;
@property(nonatomic, strong, nullable) TTInputRecognitionStageContext * stageContext;
@property(nonatomic, strong, nullable) id threshold;
@property(nonatomic, strong, nullable) NSNumber * top;
@end
@interface TTInputRequestOriginalJson : TTRequestObject
@property(nonatomic, strong, nullable) id value;
@end
@interface TTInputRequestOriginalText : TTRequestObject
@property(nonatomic, strong, nullable) NSString * text;
@end
@interface TTInputRequestQuestionDefinition : TTRequestObject
@property(nonatomic, strong, nullable) id value;
@end
@interface TTInputRequestQuestionFile : TTRequestObject
@property(nonatomic, strong, nullable) NSString * path;
@end
@interface TTInputRequestQuestionName : TTRequestObject
@property(nonatomic, strong, nullable) NSString * name;
@end
@interface TTInputRequestQuestionReference : TTRequestObject
@property(nonatomic, strong, nullable) NSString * reference;
@end
@interface TTInputRequestQuestionText : TTRequestObject
@property(nonatomic, strong, nullable) NSString * text;
@end
@interface TTInputRequestReader : TTRequestObject
@property(nonatomic, strong, nullable) TTInputSourceUnit unit;
@property(nonatomic, strong, nullable) NSNumber * window;
@end
@interface TTInputRequestReaderFailureInvalidInput : TTRequestObject
@property(nonatomic, strong, nullable) TTInputSessionSourceLocation * location;
@end
@interface TTInputRequestReaderFailureIo : TTRequestObject
@property(nonatomic, strong, nullable) TTInputSessionSourceLocation * location;
@end
@interface TTInputRequestReaderFailureProducer : TTRequestObject
@end
@interface TTInputRequestReaderFailureUtf8 : TTRequestObject
@property(nonatomic, strong, nullable) TTInputSessionSourceLocation * location;
@end
@interface TTInputRequestSessionDescriptor : TTRequestObject
@property(nonatomic, strong, nullable) TTInputRequestItem * item;
@property(nonatomic, strong, nullable) TTInputSessionSourceLocation * location;
@end
@interface TTInputRequestSource : TTRequestObject
@property(nonatomic, strong, nullable) TTInputRequestFraming framing;
@property(nonatomic, strong, nullable) TTInputReaderMedia media;
@property(nonatomic, strong, nullable) NSArray * paths;
@property(nonatomic, strong, nullable) TTInputRequestReader * reading;
@end
@interface TTInputSessionSourceLocation : TTRequestObject
@property(nonatomic, strong, nullable) NSString * file;
@property(nonatomic, strong, nullable) NSNumber * firstLine;
@property(nonatomic, strong, nullable) NSNumber * lastLine;
@end
#define TT_GENERATED_NAMED_DECLARATIONS \
- (nullable TTTask *)decide:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
\
- (nullable TTTask *)choose:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
\
- (nullable TTTask *)tag:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
\
- (nullable TTTask *)score:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
\
- (nullable TTTask *)filter:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
\
- (nullable TTTask *)rank:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
\
- (nullable TTTask *)find:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
\
- (nullable TTTask *)annotate:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
\
- (nullable TTTask *)recognize:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
\
- (nullable TTTask *)relate:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error;
#define TT_GENERATED_NAMED_IMPLEMENTATIONS \
- (nullable TTTask *)decide:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"decide" question:question input:input options:options feed:feed completion:completion error:error];
} \
- (nullable TTTask *)choose:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"choose" question:question input:input options:options feed:feed completion:completion error:error];
} \
- (nullable TTTask *)tag:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"tag" question:question input:input options:options feed:feed completion:completion error:error];
} \
- (nullable TTTask *)score:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"score" question:question input:input options:options feed:feed completion:completion error:error];
} \
- (nullable TTTask *)filter:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"filter" question:question input:input options:options feed:feed completion:completion error:error];
} \
- (nullable TTTask *)rank:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"rank" question:question input:input options:options feed:feed completion:completion error:error];
} \
- (nullable TTTask *)find:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"find" question:question input:input options:options feed:feed completion:completion error:error];
} \
- (nullable TTTask *)annotate:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"annotate" question:question input:input options:options feed:feed completion:completion error:error];
} \
- (nullable TTTask *)recognize:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"recognize" question:question input:input options:options feed:feed completion:completion error:error];
} \
- (nullable TTTask *)relate:(NSDictionary<NSString *, id> *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    return [self start:@"relate" question:question input:input options:options feed:feed completion:completion error:error];
}
FOUNDATION_EXPORT NSErrorDomain const TTAnswerErrorDomain;
FOUNDATION_EXPORT id _Nullable TTAnswerPayload(NSArray<TTSessionPacket *> *packets, TTSessionPacketTerminal *terminal,
NSString *function, NSString *judgment, id call, NSError **error);
FOUNDATION_EXPORT TTPresence *TTAnswerPresence(id payload, id (^convert)(id));
#define TT_GENERATED_ANSWER_DECLARATIONS \
- (nullable TTPresence<NSNumber *> *)decideAnswer:(NSError **)error;
\
- (nullable TTPresence<NSString *> *)chooseAnswer:(NSError **)error;
\
- (nullable TTPresence<NSArray<NSString *> *> *)tagAnswer:(NSError **)error;
\
- (nullable TTPresence<NSNumber *> *)scoreAnswer:(NSError **)error;
\
- (nullable TTPresence<NSArray<TTAtomicBoolean *> *> *)filterAnswer:(NSError **)error;
\
- (nullable TTPresence<NSArray<TTAtomicNonZeroUsize *> *> *)rankAnswer:(NSError **)error;
\
- (nullable TTPresence<TTFind *> *)findAnswer:(NSError **)error;
\
- (nullable TTPresence<NSArray<TTAnnotation *> *> *)annotateAnswer:(NSError **)error;
\
- (nullable TTPresence<NSArray<TTRecognition *> *> *)recognizeAnswer:(NSError **)error;
\
- (nullable TTPresence<TTRelation *> *)relateAnswer:(NSError **)error;
#define TT_GENERATED_ANSWER_IMPLEMENTATIONS \
- (nullable TTPresence<NSNumber *> *)decideAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"decide", @"decision", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return TTBoolean(value);
    });
} \
- (nullable TTPresence<NSString *> *)chooseAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"choose", @"choice", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return TTString(value);
    });
} \
- (nullable TTPresence<NSArray<NSString *> *> *)tagAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"tag", @"tags", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    });
} \
- (nullable TTPresence<NSNumber *> *)scoreAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"score", @"score", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return TTNumber(value);
    });
} \
- (nullable TTPresence<NSArray<TTAtomicBoolean *> *> *)filterAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"filter", @"", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicBoolean read:item];
        });
    });
} \
- (nullable TTPresence<NSArray<TTAtomicNonZeroUsize *> *> *)rankAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"rank", @"", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicNonZeroUsize read:item];
        });
    });
} \
- (nullable TTPresence<TTFind *> *)findAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"find", @"", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return [TTFind read:value];
    });
} \
- (nullable TTPresence<NSArray<TTAnnotation *> *> *)annotateAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"annotate", @"", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAnnotation read:item];
        });
    });
} \
- (nullable TTPresence<NSArray<TTRecognition *> *> *)recognizeAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"recognize", @"", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRecognition read:item];
        });
    });
} \
- (nullable TTPresence<TTRelation *> *)relateAnswer:(NSError **)error {
    id payload = TTAnswerPayload(self.packets, self.terminal, @"relate", @"", self, error);
    if (!payload) return nil;
    return TTAnswerPresence(payload, ^id(id value) {
        return [TTRelation read:value];
    });
}
#define TT_GENERATED_BLOCKING_ANSWER_DECLARATIONS \
- (nullable TTPresence<NSNumber *> *)decideAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
\
- (nullable TTPresence<NSString *> *)chooseAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
\
- (nullable TTPresence<NSArray<NSString *> *> *)tagAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
\
- (nullable TTPresence<NSNumber *> *)scoreAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
\
- (nullable TTPresence<NSArray<TTAtomicBoolean *> *> *)filterAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
\
- (nullable TTPresence<NSArray<TTAtomicNonZeroUsize *> *> *)rankAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
\
- (nullable TTPresence<TTFind *> *)findAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
\
- (nullable TTPresence<NSArray<TTAnnotation *> *> *)annotateAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
\
- (nullable TTPresence<NSArray<TTRecognition *> *> *)recognizeAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
\
- (nullable TTPresence<TTRelation *> *)relateAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error;
#define TT_GENERATED_BLOCKING_ANSWER_IMPLEMENTATIONS \
- (nullable TTPresence<NSNumber *> *)decideAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self decide:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result decideAnswer:error];
} \
- (nullable TTPresence<NSString *> *)chooseAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self choose:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result chooseAnswer:error];
} \
- (nullable TTPresence<NSArray<NSString *> *> *)tagAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self tag:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result tagAnswer:error];
} \
- (nullable TTPresence<NSNumber *> *)scoreAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self score:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result scoreAnswer:error];
} \
- (nullable TTPresence<NSArray<TTAtomicBoolean *> *> *)filterAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self filter:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result filterAnswer:error];
} \
- (nullable TTPresence<NSArray<TTAtomicNonZeroUsize *> *> *)rankAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self rank:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result rankAnswer:error];
} \
- (nullable TTPresence<TTFind *> *)findAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self find:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result findAnswer:error];
} \
- (nullable TTPresence<NSArray<TTAnnotation *> *> *)annotateAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self annotate:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result annotateAnswer:error];
} \
- (nullable TTPresence<NSArray<TTRecognition *> *> *)recognizeAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self recognize:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result recognizeAnswer:error];
} \
- (nullable TTPresence<TTRelation *> *)relateAnswer:(NSDictionary *)question input:(NSDictionary *)input options:(nullable NSDictionary *)options error:(NSError **)error {
    __block TTCall *result;
    __block NSError *failure = nil;
    dispatch_semaphore_t settled = dispatch_semaphore_create(0);
    TTTask *task = [self relate:question input:input options:options feed:nil completion:^(TTCall *call,
    NSError *problem) {
        result = call;
        failure = problem;
        dispatch_semaphore_signal(settled);
    } error:&failure];
    if (!task) {
        if (error) *error = failure;
        return nil;
    } dispatch_semaphore_wait(settled, DISPATCH_TIME_FOREVER);
    if (failure) {
        if (error) *error = failure;
        return nil;
    } return [result relateAnswer:error];
}
typedef NS_ENUM(uint32_t, TTUsagePersistenceState) {
    TTUsagePersistenceDisabled = 1,
    TTUsagePersistenceFailed = 4,
    TTUsagePersistencePending = 2,
    TTUsagePersistenceWritten = 3,
};
#define TT_GENERATED_USAGE_DECLARATIONS \
- (nullable TTUsagePersistenceStatus *)finishUsageStatus:(NSError **)error;
\
- (nullable TTUsagePersistenceStatus *)usagePersistence:(NSError **)error;
#define TT_GENERATED_USAGE_IMPLEMENTATIONS \
- (nullable TTUsagePersistenceStatus *)finishUsageStatus:(NSError **)error {
    return [self usageStatus:YES error:error];
} \
- (nullable TTUsagePersistenceStatus *)usagePersistence:(NSError **)error {
    return [self usageStatus:NO error:error];
}
NS_ASSUME_NONNULL_END
