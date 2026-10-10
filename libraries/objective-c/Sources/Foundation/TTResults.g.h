// Generated from the shared Rust result graph; do not edit.
#import "TTResult.h"
NS_ASSUME_NONNULL_BEGIN
FOUNDATION_EXPORT NSString * const TTRequestVersion;
FOUNDATION_EXPORT NSInteger TTNativeErrorCode(NSString *kind);
@class TTAnnotation, TTAnnotationMember, TTAnnotationMemberAnswerId, TTAnnotationMemberFailureId, TTAnnotationValue, TTAnnotationValueChoice, TTAnnotationValueDecision, TTAnnotationValueFailed, TTAnnotationValueScore, TTAnnotationValueTags, TTAnswers, TTAtomicArrayOfString, TTAtomicDecideValue, TTAtomicNonZeroUsize, TTAtomicNullableString, TTAtomicBoolean, TTAtomicDouble, TTAttempt, TTBatch, TTBoundaryOdds, TTBoundaryProposal, TTCallError, TTEntityDocument, TTError, TTEstimatedInputDenial, TTEstimatedInputDenialAdditionalRequest, TTEstimatedInputDenialInitialRequest, TTEstimatedInputDenialRetry, TTFacts, TTFind, TTFindCandidate, TTImage, TTInputDeclaration, TTInputDeclarationObject, TTInputDeclarationString, TTInputPropertyType, TTInputPropertyTypeArray, TTInputPropertyTypeBoolean, TTInputPropertyTypeNumber, TTInputPropertyTypeString, TTLabel, TTMeta, TTObjectRoot, TTObservation, TTObservationFailureId, TTObservationObservationId, TTPersistenceObservation, TTPhysicalSource, TTPosition, TTQuestionSource, TTRankMember, TTRankMemberResult, TTReadableQuestion, TTReadableQuestion2, TTReadableQuestion3, TTReadableQuestion4, TTReadableQuestionChoose, TTReadableQuestionDecide, TTReadableQuestionScore, TTReadableQuestionTag, TTRecognition, TTRecognitionEdgeDocument, TTRecognitionOdds, TTRecognitionOddsFieldsNamesPairsPiecesProposals, TTRecognitionOddsFieldsPiecesProposals, TTRecognitionProposal, TTRecognitionStageContext, TTRelation, TTRelationMember, TTRelationMemberAnswerId, TTRelationMemberFailureId, TTSendBudgetDenial, TTSendBudgetDenialBeforeAdditionalSend, TTSendBudgetDenialBeforeFirstSend, TTSendBudgetDenialBeforeRetry, TTStopped, TTStringRoot, TTUsage, TTAnnotatedField, TTAnswer, TTAnswerChoice, TTAnswerScore, TTAnswerTag, TTAnswerYesNo, TTBatchSetting, TTBatchWarning, TTEntity, TTEntityEdge, TTFailed, TTFailure, TTFindAnswer, TTNameOdds, TTPairOdds, TTPieceOdds, TTPlace, TTPlan, TTProfileWarning, TTRecognize, TTRecognizeAnswer, TTRecognizeFieldsEntities, TTRecognizeFieldsModeProposals, TTRelateFields, TTRelatedEntity, TTRelatedEntityEdge, TTRelatedEntityEdgePropertiesSource, TTRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord, TTRelatedEntityEdgePropertiesSourceFieldsKindName, TTRelationRule, TTSessionAnnotation, TTSessionInputSource, TTSessionJudgment, TTSessionJudgmentChoice, TTSessionJudgmentDecision, TTSessionJudgmentScore, TTSessionJudgmentTags, TTSessionNamedProbability, TTSessionObservation, TTSessionObservationQuestion, TTSessionObservationRow, TTSessionObservedRow, TTSessionObservedRowAnnotated, TTSessionObservedRowFind, TTSessionObservedRowJudgment, TTSessionObservedRowRecognized, TTSessionObservedRowRelations, TTSessionPacket, TTSessionPacketAnnotateAggregate, TTSessionPacketAnnotateRow, TTSessionPacketChooseAggregate, TTSessionPacketChooseRow, TTSessionPacketDecideAggregate, TTSessionPacketDecideRow, TTSessionPacketFilterAggregate, TTSessionPacketFilterRow, TTSessionPacketFindAggregate, TTSessionPacketObservation, TTSessionPacketRankAggregate, TTSessionPacketRecognizeAggregate, TTSessionPacketRelateAggregate, TTSessionPacketScoreAggregate, TTSessionPacketScoreRow, TTSessionPacketTagAggregate, TTSessionPacketTagRow, TTSessionPacketTerminal, TTSessionProbabilities, TTSessionProbabilitiesNamed, TTSessionProbabilitiesYesNo, TTSessionQuestionDetail, TTSessionRecognition, TTSessionRelationEdge, TTSourceRelationEndpoint, TTThreshold, TTTokenBand, TTTokenUsage, TTValue;
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
@property(nonatomic, readonly) TTPresence<TTRelatedEntityEdgePropertiesSource *> *source;
@property(nonatomic, readonly) TTPresence<TTRelatedEntityEdgePropertiesSource *> *target;
+ (instancetype)read:(id)value;
@end
@interface TTRelatedEntityEdgePropertiesSource : TTResultNode
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
@interface TTInputDeclarationObject : TTInputDeclaration
@property(nonatomic, readonly) TTPresence<NSDictionary<NSString *, TTInputPropertyType *> *> *properties;
@property(nonatomic, readonly) TTPresence<NSArray<NSString *> *> *required;
@property(nonatomic, readonly) TTPresence<NSString *> *type;
+ (instancetype)read:(id)value;
@end
@interface TTInputDeclarationString : TTInputDeclaration
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
@interface TTRecognitionOddsFieldsNamesPairsPiecesProposals : TTRecognitionOdds
@property(nonatomic, readonly) TTPresence<NSArray<TTNameOdds *> *> *names;
@property(nonatomic, readonly) TTPresence<NSArray<TTPairOdds *> *> *pairs;
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTRecognitionProposal *> *> *proposals;
+ (instancetype)read:(id)value;
@end
@interface TTRecognitionOddsFieldsPiecesProposals : TTRecognitionOdds
@property(nonatomic, readonly) TTPresence<NSArray<TTPieceOdds *> *> *pieces;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
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
@interface TTRecognizeFieldsEntities : TTRecognize
@property(nonatomic, readonly) TTPresence<NSArray<TTEntity *> *> *entities;
@property(nonatomic, readonly) TTPresence<NSArray<TTEntityEdge *> *> *relations;
+ (instancetype)read:(id)value;
@end
@interface TTRecognizeFieldsModeProposals : TTRecognize
@property(nonatomic, readonly) TTPresence<NSString *> *mode;
@property(nonatomic, readonly) TTPresence<NSArray<TTBoundaryProposal *> *> *proposals;
+ (instancetype)read:(id)value;
@end
@interface TTRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord : TTRelatedEntityEdgePropertiesSource
@property(nonatomic, readonly) TTPresence<NSString *> *file;
@property(nonatomic, readonly) TTPresence<NSNumber *> *firstLine;
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSNumber *> *lastLine;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
@property(nonatomic, readonly) TTPresence<NSNumber *> *ordinal;
@property(nonatomic, readonly) TTPresence<id> *record;
+ (instancetype)read:(id)value;
@end
@interface TTRelatedEntityEdgePropertiesSourceFieldsKindName : TTRelatedEntityEdgePropertiesSource
@property(nonatomic, readonly) TTPresence<NSString *> *kind;
@property(nonatomic, readonly) TTPresence<NSString *> *name;
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
NS_ASSUME_NONNULL_END
