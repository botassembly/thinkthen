// Generated from the shared Rust result graph; do not edit.
#import "TTResults.g.h"
#import "TTResultPrivate.h"
NSString * const TTRequestVersion = @"thinkthen.request/1";
@implementation TTAnnotation
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSDictionary<NSString *, TTAnnotationMember *> *> *)answers { return [self presence:@"answers" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return [TTAnnotationMember read:item]; }); }]; }
- (TTPresence<NSString *> *)file { return [self presence:@"file" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)firstLine { return [self presence:@"first_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:YES convert:^id(id value) { return value; }]; }
- (TTPresence<NSNumber *> *)lastLine { return [self presence:@"last_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTPosition *> *)position { return [self presence:@"position" required:NO convert:^id(id value) { return [TTPosition read:value]; }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<NSDictionary<NSString *, TTAnnotatedField *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return [TTAnnotatedField read:item]; }); }]; }
@end
@implementation TTAnnotationMember
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if (object[@"answer_id"] != nil) return (id)[TTAnnotationMemberAnswerId read:value];
if (object[@"failure_id"] != nil) return (id)[TTAnnotationMemberFailureId read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTAnnotationValue
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"kind"] isEqual:@"decision"]) return (id)[TTAnnotationValueDecision read:value];
if ([object[@"kind"] isEqual:@"choice"]) return (id)[TTAnnotationValueChoice read:value];
if ([object[@"kind"] isEqual:@"score"]) return (id)[TTAnnotationValueScore read:value];
if ([object[@"kind"] isEqual:@"tags"]) return (id)[TTAnnotationValueTags read:value];
if ([object[@"kind"] isEqual:@"failed"]) return (id)[TTAnnotationValueFailed read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTAnswers
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSArray<TTRelationMember *> *> *)questions { return [self presence:@"questions" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRelationMember read:item]; }); }]; }
@end
@implementation TTAtomicArrayOfString
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTImage *> *> *)images { return [self presence:@"images" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTImage read:item]; }); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSArray<TTRankMember *> *> *)members { return [self presence:@"members" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRankMember read:item]; }); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSString *> *)questionName { return [self presence:@"question_name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<NSArray<NSString *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
@end
@implementation TTAtomicDecideValue
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTImage *> *> *)images { return [self presence:@"images" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTImage read:item]; }); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSArray<TTRankMember *> *> *)members { return [self presence:@"members" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRankMember read:item]; }); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSString *> *)questionName { return [self presence:@"question_name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<id> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return value; }]; }
@end
@implementation TTAtomicNonZeroUsize
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTImage *> *> *)images { return [self presence:@"images" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTImage read:item]; }); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSArray<TTRankMember *> *> *)members { return [self presence:@"members" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRankMember read:item]; }); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSString *> *)questionName { return [self presence:@"question_name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTAtomicNullableString
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTImage *> *> *)images { return [self presence:@"images" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTImage read:item]; }); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSArray<TTRankMember *> *> *)members { return [self presence:@"members" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRankMember read:item]; }); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSString *> *)questionName { return [self presence:@"question_name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<NSString *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTAtomicBoolean
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTImage *> *> *)images { return [self presence:@"images" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTImage read:item]; }); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSArray<TTRankMember *> *> *)members { return [self presence:@"members" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRankMember read:item]; }); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSString *> *)questionName { return [self presence:@"question_name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
@end
@implementation TTAtomicDouble
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTImage *> *> *)images { return [self presence:@"images" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTImage read:item]; }); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSArray<TTRankMember *> *> *)members { return [self presence:@"members" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRankMember read:item]; }); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSString *> *)questionName { return [self presence:@"question_name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTNumber(value); }]; }
@end
@implementation TTAttempt
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)ordinal { return [self presence:@"ordinal" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)outcome { return [self presence:@"outcome" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)requestId { return [self presence:@"request_id" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)requestSha256 { return [self presence:@"request_sha256" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)sdkRequestId { return [self presence:@"sdk_request_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)serverMs { return [self presence:@"server_ms" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)status { return [self presence:@"status" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)wallMs { return [self presence:@"wall_ms" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTBatch
+ (instancetype)read:(id)value {
if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{@"value":TTInteger(value), @"kind":@"integer"}];
if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{@"value":TTString(value), @"kind":@"string"}];
TTInvalidResult(); return nil;
}
- (id)value { return self.rawFields[@"value"]; }
- (NSString *)kind { return self.rawFields[@"kind"]; }
@end
@implementation TTBoundaryOdds
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSArray<TTPieceOdds *> *> *)pieces { return [self presence:@"pieces" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTPieceOdds read:item]; }); }]; }
- (TTPresence<NSArray<TTBoundaryProposal *> *> *)proposals { return [self presence:@"proposals" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTBoundaryProposal read:item]; }); }]; }
@end
@implementation TTBoundaryProposal
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)end { return [self presence:@"end" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)length { return [self presence:@"length" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSNumber *> *)start { return [self presence:@"start" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)text { return [self presence:@"text" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTCallError
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTError *> *)error { return [self presence:@"error" required:YES convert:^id(id value) { return [TTError read:value]; }]; }
- (TTPresence<TTFacts *> *)facts { return [self presence:@"facts" required:NO convert:^id(id value) { return [TTFacts read:value]; }]; }
@end
@implementation TTEntityDocument
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTError
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTEstimatedInputDenial *> *)estimatedInputDenial { return [self presence:@"estimated_input_denial" required:NO convert:^id(id value) { return [TTEstimatedInputDenial read:value]; }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)message { return [self presence:@"message" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)retryable { return [self presence:@"retryable" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<TTSendBudgetDenial *> *)sendBudgetDenial { return [self presence:@"send_budget_denial" required:NO convert:^id(id value) { return [TTSendBudgetDenial read:value]; }]; }
- (TTPresence<TTStopped *> *)stopped { return [self presence:@"stopped" required:YES convert:^id(id value) { return [TTStopped read:value]; }]; }
@end
@implementation TTEstimatedInputDenial
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"kind"] isEqual:@"initial_request"]) return (id)[TTEstimatedInputDenialInitialRequest read:value];
if ([object[@"kind"] isEqual:@"additional_request"]) return (id)[TTEstimatedInputDenialAdditionalRequest read:value];
if ([object[@"kind"] isEqual:@"retry"]) return (id)[TTEstimatedInputDenialRetry read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTFacts
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSArray<TTAttempt *> *> *)attempts { return [self presence:@"attempts" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTAttempt read:item]; }); }]; }
- (TTPresence<NSNumber *> *)cacheAnswers { return [self presence:@"cache_answers" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)callId { return [self presence:@"call_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)estimatedCostUsd { return [self presence:@"estimated_cost_usd" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)heldModelMismatch { return [self presence:@"held_model_mismatch" required:NO convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSNumber *> *)inputTokens { return [self presence:@"input_tokens" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)largestRequestBytes { return [self presence:@"largest_request_bytes" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)largestRequestEstimatedInputTokens { return [self presence:@"largest_request_estimated_input_tokens" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)outputTokens { return [self presence:@"output_tokens" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)records { return [self presence:@"records" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)requestsSent { return [self presence:@"requests_sent" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)seconds { return [self presence:@"seconds" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)tokenEstimateMethod { return [self presence:@"token_estimate_method" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPersistenceObservation *> *)usagePersistence { return [self presence:@"usage_persistence" required:NO convert:^id(id value) { return [TTPersistenceObservation read:value]; }]; }
@end
@implementation TTFind
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTFindAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTFindAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTFindCandidate *> *> *)candidates { return [self presence:@"candidates" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTFindCandidate read:item]; }); }]; }
- (TTPresence<NSString *> *)file { return [self presence:@"file" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)firstLine { return [self presence:@"first_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)lastLine { return [self presence:@"last_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTPosition *> *)position { return [self presence:@"position" required:NO convert:^id(id value) { return [TTPosition read:value]; }]; }
- (TTPresence<TTReadableQuestion2 *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion2 read:value]; }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<id> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return value; }]; }
- (TTPresence<id> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return value; }]; }
@end
@implementation TTFindCandidate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:YES convert:^id(id value) { return value; }]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
@end
@implementation TTImage
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)base64 { return [self presence:@"base64" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)height { return [self presence:@"height" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)media { return [self presence:@"media" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)width { return [self presence:@"width" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTInputDeclaration
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"type"] isEqual:@"string"]) return (id)[TTInputDeclarationString read:value];
if ([object[@"type"] isEqual:@"object"]) return (id)[TTInputDeclarationObject read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTInputPropertyType
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"type"] isEqual:@"string"]) return (id)[TTInputPropertyTypeString read:value];
if ([object[@"type"] isEqual:@"number"]) return (id)[TTInputPropertyTypeNumber read:value];
if ([object[@"type"] isEqual:@"boolean"]) return (id)[TTInputPropertyTypeBoolean read:value];
if ([object[@"type"] isEqual:@"array"]) return (id)[TTInputPropertyTypeArray read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTLabel
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<id> *)descriptionValue { return [self presence:@"description" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTMeta
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)answeredBy { return [self presence:@"answered_by" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTAttempt *> *> *)attempts { return [self presence:@"attempts" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTAttempt read:item]; }); }]; }
- (TTPresence<TTBatchSetting *> *)batchSetting { return [self presence:@"batch_setting" required:NO convert:^id(id value) { return [TTBatchSetting read:value]; }]; }
- (TTPresence<TTBatchWarning *> *)batchWarning { return [self presence:@"batch_warning" required:NO convert:^id(id value) { return [TTBatchWarning read:value]; }]; }
- (TTPresence<NSNumber *> *)cached { return [self presence:@"cached" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSString *> *)contextSha256 { return [self presence:@"context_sha256" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)failedQuestions { return [self presence:@"failed_questions" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTObservation *> *> *)observations { return [self presence:@"observations" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTObservation read:item]; }); }]; }
- (TTPresence<NSString *> *)origin { return [self presence:@"origin" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTProfileWarning *> *)profileWarning { return [self presence:@"profile_warning" required:NO convert:^id(id value) { return [TTProfileWarning read:value]; }]; }
- (TTPresence<NSString *> *)questionSha256 { return [self presence:@"question_sha256" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources { return [self presence:@"question_sources" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTQuestionSource read:item]; }); }]; }
- (TTPresence<NSString *> *)questionsSha256 { return [self presence:@"questions_sha256" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)requests { return [self presence:@"requests" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSNumber *> *)requestsSent { return [self presence:@"requests_sent" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)tool { return [self presence:@"tool" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)url { return [self presence:@"url" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTUsage *> *)usage { return [self presence:@"usage" required:NO convert:^id(id value) { return [TTUsage read:value]; }]; }
@end
@implementation TTObjectRoot
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSDictionary<NSString *, TTInputPropertyType *> *> *)properties { return [self presence:@"properties" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return [TTInputPropertyType read:item]; }); }]; }
- (TTPresence<NSArray<NSString *> *> *)required { return [self presence:@"required" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSString *> *)type { return [self presence:@"type" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTObservation
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if (object[@"observation_id"] != nil) return (id)[TTObservationObservationId read:value];
if (object[@"failure_id"] != nil) return (id)[TTObservationFailureId read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTPersistenceObservation
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)advice { return [self presence:@"advice" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)observedAt { return [self presence:@"observed_at" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)state { return [self presence:@"state" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTPhysicalSource
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)file { return [self presence:@"file" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)firstLine { return [self presence:@"first_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)lastLine { return [self presence:@"last_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTPosition
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)file { return [self presence:@"file" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)first { return [self presence:@"first" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)images { return [self presence:@"images" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSNumber *> *)last { return [self presence:@"last" required:NO convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTQuestionSource
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)answeredBy { return [self presence:@"answered_by" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)batchSize { return [self presence:@"batch_size" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)origin { return [self presence:@"origin" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTRankMember
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTRankMemberResult *> *)result { return [self presence:@"result" required:YES convert:^id(id value) { return [TTRankMemberResult read:value]; }]; }
@end
@implementation TTRankMemberResult
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTImage *> *> *)images { return [self presence:@"images" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTImage read:item]; }); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<id> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return value; }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTReadableQuestion
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"verb"] isEqual:@"decide"]) return (id)[TTReadableQuestionDecide read:value];
if ([object[@"verb"] isEqual:@"choose"]) return (id)[TTReadableQuestionChoose read:value];
if ([object[@"verb"] isEqual:@"tag"]) return (id)[TTReadableQuestionTag read:value];
if ([object[@"verb"] isEqual:@"score"]) return (id)[TTReadableQuestionScore read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTReadableQuestion2
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTBatch *> *)batch { return [self presence:@"batch" required:NO convert:^id(id value) { return [TTBatch read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)contextSchema { return [self presence:@"context_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)itemSchema { return [self presence:@"item_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails { return [self presence:@"label_details" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTLabel read:item]; }); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)none { return [self presence:@"none" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)on { return [self presence:@"on" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSString *> *)profile { return [self presence:@"profile" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<id> *)text { return [self presence:@"text" required:YES convert:^id(id value) { return value; }]; }
- (TTPresence<NSString *> *)verb { return [self presence:@"verb" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)wordingVersion { return [self presence:@"wording_version" required:NO convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTReadableQuestion3
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTBatch *> *)batch { return [self presence:@"batch" required:NO convert:^id(id value) { return [TTBatch read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)contextSchema { return [self presence:@"context_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<id> *)entityDefinition { return [self presence:@"entity_definition" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<id> *)instructions { return [self presence:@"instructions" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<TTInputDeclaration *> *)itemSchema { return [self presence:@"item_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<NSDictionary<NSString *, id> *> *)kinds { return [self presence:@"kinds" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return item; }); }]; }
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails { return [self presence:@"label_details" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTLabel read:item]; }); }]; }
- (TTPresence<NSString *> *)mode { return [self presence:@"mode" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)on { return [self presence:@"on" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSString *> *)profile { return [self presence:@"profile" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTThreshold *> *)relationThreshold { return [self presence:@"relation_threshold" required:NO convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<NSArray<TTRelationRule *> *> *)relations { return [self presence:@"relations" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRelationRule read:item]; }); }]; }
- (TTPresence<NSNumber *> *)snippetPieces { return [self presence:@"snippet_pieces" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<TTRecognitionStageContext *> *)stageContext { return [self presence:@"stage_context" required:NO convert:^id(id value) { return [TTRecognitionStageContext read:value]; }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<NSString *> *)verb { return [self presence:@"verb" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)wordingVersion { return [self presence:@"wording_version" required:NO convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTReadableQuestion4
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTBatch *> *)batch { return [self presence:@"batch" required:NO convert:^id(id value) { return [TTBatch read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)contextSchema { return [self presence:@"context_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<TTRelateFields *> *)fields { return [self presence:@"fields" required:YES convert:^id(id value) { return [TTRelateFields read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)itemSchema { return [self presence:@"item_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails { return [self presence:@"label_details" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTLabel read:item]; }); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)on { return [self presence:@"on" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSString *> *)profile { return [self presence:@"profile" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTRelationRule *> *> *)relations { return [self presence:@"relations" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRelationRule read:item]; }); }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<NSString *> *)verb { return [self presence:@"verb" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)wordingVersion { return [self presence:@"wording_version" required:NO convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTRecognition
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTRecognitionOdds *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTRecognitionOdds read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)file { return [self presence:@"file" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)firstLine { return [self presence:@"first_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSNumber *> *)lastLine { return [self presence:@"last_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTPosition *> *)position { return [self presence:@"position" required:NO convert:^id(id value) { return [TTPosition read:value]; }]; }
- (TTPresence<TTReadableQuestion3 *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion3 read:value]; }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<TTRecognize *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTRecognize read:value]; }]; }
@end
@implementation TTRecognitionEdgeDocument
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)either { return [self presence:@"either" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)relation { return [self presence:@"relation" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTEntity *> *)source { return [self presence:@"source" required:YES convert:^id(id value) { return [TTEntity read:value]; }]; }
- (TTPresence<TTEntity *> *)target { return [self presence:@"target" required:YES convert:^id(id value) { return [TTEntity read:value]; }]; }
@end
@implementation TTRecognitionOdds
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if (object[@"names"] != nil && object[@"pairs"] != nil && object[@"pieces"] != nil && object[@"proposals"] != nil) return (id)[TTRecognitionOddsFieldsNamesPairsPiecesProposals read:value];
if (object[@"pieces"] != nil && object[@"proposals"] != nil && object[@"names"] == nil && object[@"pairs"] == nil) return (id)[TTRecognitionOddsFieldsPiecesProposals read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTRecognitionProposal
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)end { return [self presence:@"end" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)kept { return [self presence:@"kept" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPlace *> *)selected { return [self presence:@"selected" required:NO convert:^id(id value) { return [TTPlace read:value]; }]; }
- (TTPresence<NSNumber *> *)spanProbability { return [self presence:@"span_probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSNumber *> *)start { return [self presence:@"start" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)strength { return [self presence:@"strength" required:NO convert:^id(id value) { return TTNumber(value); }]; }
@end
@implementation TTRecognitionStageContext
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)boundary { return [self presence:@"boundary" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kindEdge { return [self presence:@"kind_edge" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)relation { return [self presence:@"relation" required:NO convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTRelation
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTAnswers *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswers read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)file { return [self presence:@"file" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)firstLine { return [self presence:@"first_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSArray<TTSessionInputSource *> *> *)inputSources { return [self presence:@"input_sources" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTSessionInputSource read:item]; }); }]; }
- (TTPresence<NSNumber *> *)lastLine { return [self presence:@"last_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<TTMeta *> *)meta { return [self presence:@"meta" required:YES convert:^id(id value) { return [TTMeta read:value]; }]; }
- (TTPresence<TTPosition *> *)position { return [self presence:@"position" required:NO convert:^id(id value) { return [TTPosition read:value]; }]; }
- (TTPresence<TTReadableQuestion4 *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion4 read:value]; }]; }
- (TTPresence<NSString *> *)schema { return [self presence:@"schema" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTRelatedEntityEdge *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRelatedEntityEdge read:item]; }); }]; }
@end
@implementation TTRelationMember
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if (object[@"answer_id"] != nil) return (id)[TTRelationMemberAnswerId read:value];
if (object[@"failure_id"] != nil) return (id)[TTRelationMemberFailureId read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTSendBudgetDenial
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"kind"] isEqual:@"before_first_send"]) return (id)[TTSendBudgetDenialBeforeFirstSend read:value];
if ([object[@"kind"] isEqual:@"before_additional_send"]) return (id)[TTSendBudgetDenialBeforeAdditionalSend read:value];
if ([object[@"kind"] isEqual:@"before_retry"]) return (id)[TTSendBudgetDenialBeforeRetry read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTStopped
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)at { return [self presence:@"at" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)cause { return [self presence:@"cause" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)retryable { return [self presence:@"retryable" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSNumber *> *)status { return [self presence:@"status" required:NO convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTStringRoot
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)type { return [self presence:@"type" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTUsage
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)inputTokens { return [self presence:@"input_tokens" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)outputTokens { return [self presence:@"output_tokens" required:NO convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTAnnotatedField
+ (instancetype)read:(id)value {
if (TTIsBoolean(value)) return [[self alloc] initWithFields:@{@"value":TTBoolean(value), @"kind":@"boolean"}];
if (value == NSNull.null) return [[self alloc] initWithFields:@{@"value":value, @"kind":@"null"}];
if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{@"value":TTString(value), @"kind":@"string"}];
if ([value isKindOfClass:NSArray.class]) return [[self alloc] initWithFields:@{@"value":TTArray(value, ^id(id item) { return TTString(item); }), @"kind":@"array"}];
if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{@"value":TTNumber(value), @"kind":@"number"}];
if ([value isKindOfClass:NSDictionary.class]) return [[self alloc] initWithFields:@{@"value":[TTFailed read:value], @"kind":@"object"}];
TTInvalidResult(); return nil;
}
- (id)value { return self.rawFields[@"value"]; }
- (NSString *)kind { return self.rawFields[@"kind"]; }
@end
@implementation TTAnswer
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"kind"] isEqual:@"yes_no"]) return (id)[TTAnswerYesNo read:value];
if ([object[@"kind"] isEqual:@"choice"]) return (id)[TTAnswerChoice read:value];
if ([object[@"kind"] isEqual:@"tag"]) return (id)[TTAnswerTag read:value];
if ([object[@"kind"] isEqual:@"score"]) return (id)[TTAnswerScore read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTBatchSetting
+ (instancetype)read:(id)value {
if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{@"value":TTInteger(value), @"kind":@"integer"}];
if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{@"value":TTString(value), @"kind":@"string"}];
TTInvalidResult(); return nil;
}
- (id)value { return self.rawFields[@"value"]; }
- (NSString *)kind { return self.rawFields[@"kind"]; }
@end
@implementation TTBatchWarning
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTBatchSetting *> *)running { return [self presence:@"running" required:YES convert:^id(id value) { return [TTBatchSetting read:value]; }]; }
- (TTPresence<TTBatchSetting *> *)tunedFor { return [self presence:@"tuned_for" required:YES convert:^id(id value) { return [TTBatchSetting read:value]; }]; }
@end
@implementation TTEntity
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)end { return [self presence:@"end" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)file { return [self presence:@"file" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)firstLine { return [self presence:@"first_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)lastLine { return [self presence:@"last_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)length { return [self presence:@"length" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)start { return [self presence:@"start" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)strength { return [self presence:@"strength" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)text { return [self presence:@"text" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTEntityEdge
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)either { return [self presence:@"either" required:NO convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)relation { return [self presence:@"relation" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTEntity *> *)source { return [self presence:@"source" required:YES convert:^id(id value) { return [TTEntity read:value]; }]; }
- (TTPresence<TTEntity *> *)target { return [self presence:@"target" required:YES convert:^id(id value) { return [TTEntity read:value]; }]; }
@end
@implementation TTFailed
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTFailure *> *)failed { return [self presence:@"failed" required:YES convert:^id(id value) { return [TTFailure read:value]; }]; }
@end
@implementation TTFailure
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)cause { return [self presence:@"cause" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTFindAnswer
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)confidence { return [self presence:@"confidence" required:NO convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)pick { return [self presence:@"pick" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)probabilities { return [self presence:@"probabilities" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return TTNumber(item); }); }]; }
@end
@implementation TTNameOdds
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)edges { return [self presence:@"edges" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return TTNumber(item); }); }]; }
- (TTPresence<NSNumber *> *)end { return [self presence:@"end" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)kinds { return [self presence:@"kinds" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return TTNumber(item); }); }]; }
- (TTPresence<NSNumber *> *)start { return [self presence:@"start" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTPairOdds
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)relation { return [self presence:@"relation" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTPlace *> *)source { return [self presence:@"source" required:YES convert:^id(id value) { return [TTPlace read:value]; }]; }
- (TTPresence<TTPlace *> *)target { return [self presence:@"target" required:YES convert:^id(id value) { return [TTPlace read:value]; }]; }
@end
@implementation TTPieceOdds
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)end { return [self presence:@"end" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)start { return [self presence:@"start" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)tags { return [self presence:@"tags" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return TTNumber(item); }); }]; }
@end
@implementation TTPlace
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)end { return [self presence:@"end" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)start { return [self presence:@"start" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTPlan
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)estimatedBytes { return [self presence:@"estimated_bytes" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<TTTokenBand *> *)estimatedInputTokens { return [self presence:@"estimated_input_tokens" required:YES convert:^id(id value) { return [TTTokenBand read:value]; }]; }
- (TTPresence<NSString *> *)firstBodyUtf8 { return [self presence:@"first_body_utf8" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)largestRequestBytes { return [self presence:@"largest_request_bytes" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)largestRequestEstimatedInputTokens { return [self presence:@"largest_request_estimated_input_tokens" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)records { return [self presence:@"records" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)requests { return [self presence:@"requests" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)tokenEstimateMethod { return [self presence:@"token_estimate_method" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)upperBound { return [self presence:@"upper_bound" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
@end
@implementation TTProfileWarning
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)running { return [self presence:@"running" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)tunedFor { return [self presence:@"tuned_for" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTRecognize
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if (object[@"entities"] != nil && object[@"mode"] == nil && object[@"proposals"] == nil) return (id)[TTRecognizeFieldsEntities read:value];
if (object[@"mode"] != nil && object[@"proposals"] != nil && object[@"entities"] == nil) return (id)[TTRecognizeFieldsModeProposals read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTRecognizeAnswer
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSArray<TTNameOdds *> *> *)names { return [self presence:@"names" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTNameOdds read:item]; }); }]; }
- (TTPresence<NSArray<TTPairOdds *> *> *)pairs { return [self presence:@"pairs" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTPairOdds read:item]; }); }]; }
- (TTPresence<NSArray<TTPieceOdds *> *> *)pieces { return [self presence:@"pieces" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTPieceOdds read:item]; }); }]; }
- (TTPresence<NSArray<TTRecognitionProposal *> *> *)proposals { return [self presence:@"proposals" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRecognitionProposal read:item]; }); }]; }
@end
@implementation TTRelateFields
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTRelatedEntity
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTRelatedEntityEdge
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)either { return [self presence:@"either" required:NO convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)relation { return [self presence:@"relation" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTRelatedEntityEdgePropertiesSource *> *)source { return [self presence:@"source" required:YES convert:^id(id value) { return [TTRelatedEntityEdgePropertiesSource read:value]; }]; }
- (TTPresence<TTRelatedEntityEdgePropertiesSource *> *)target { return [self presence:@"target" required:YES convert:^id(id value) { return [TTRelatedEntityEdgePropertiesSource read:value]; }]; }
@end
@implementation TTRelatedEntityEdgePropertiesSource
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if (object[@"kind"] != nil && object[@"name"] != nil && object[@"file"] == nil && object[@"ordinal"] == nil && object[@"record"] == nil) return (id)[TTRelatedEntityEdgePropertiesSourceFieldsKindName read:value];
if (object[@"file"] != nil && object[@"kind"] != nil && object[@"name"] != nil && object[@"ordinal"] != nil && object[@"record"] != nil) return (id)[TTRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTRelationRule
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)either { return [self presence:@"either" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)reads { return [self presence:@"reads" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)single { return [self presence:@"single" required:NO convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSString *> *)source { return [self presence:@"source" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)target { return [self presence:@"target" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTSessionAnnotation
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTAnnotationValue *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTAnnotationValue read:value]; }]; }
@end
@implementation TTSessionInputSource
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<TTPhysicalSource *> *)source { return [self presence:@"source" required:YES convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
@end
@implementation TTSessionJudgment
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"kind"] isEqual:@"decision"]) return (id)[TTSessionJudgmentDecision read:value];
if ([object[@"kind"] isEqual:@"choice"]) return (id)[TTSessionJudgmentChoice read:value];
if ([object[@"kind"] isEqual:@"score"]) return (id)[TTSessionJudgmentScore read:value];
if ([object[@"kind"] isEqual:@"tags"]) return (id)[TTSessionJudgmentTags read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTSessionNamedProbability
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
@end
@implementation TTSessionObservation
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"kind"] isEqual:@"question"]) return (id)[TTSessionObservationQuestion read:value];
if ([object[@"kind"] isEqual:@"row"]) return (id)[TTSessionObservationRow read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTSessionObservedRow
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"kind"] isEqual:@"judgment"]) return (id)[TTSessionObservedRowJudgment read:value];
if ([object[@"kind"] isEqual:@"annotated"]) return (id)[TTSessionObservedRowAnnotated read:value];
if ([object[@"kind"] isEqual:@"recognized"]) return (id)[TTSessionObservedRowRecognized read:value];
if ([object[@"kind"] isEqual:@"find"]) return (id)[TTSessionObservedRowFind read:value];
if ([object[@"kind"] isEqual:@"relations"]) return (id)[TTSessionObservedRowRelations read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTSessionPacket
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"function"] isEqual:@"decide"] && [object[@"kind"] isEqual:@"row"]) return (id)[TTSessionPacketDecideRow read:value];
if ([object[@"function"] isEqual:@"choose"] && [object[@"kind"] isEqual:@"row"]) return (id)[TTSessionPacketChooseRow read:value];
if ([object[@"function"] isEqual:@"tag"] && [object[@"kind"] isEqual:@"row"]) return (id)[TTSessionPacketTagRow read:value];
if ([object[@"function"] isEqual:@"score"] && [object[@"kind"] isEqual:@"row"]) return (id)[TTSessionPacketScoreRow read:value];
if ([object[@"function"] isEqual:@"filter"] && [object[@"kind"] isEqual:@"row"]) return (id)[TTSessionPacketFilterRow read:value];
if ([object[@"function"] isEqual:@"annotate"] && [object[@"kind"] isEqual:@"row"]) return (id)[TTSessionPacketAnnotateRow read:value];
if ([object[@"function"] isEqual:@"decide"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketDecideAggregate read:value];
if ([object[@"function"] isEqual:@"choose"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketChooseAggregate read:value];
if ([object[@"function"] isEqual:@"tag"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketTagAggregate read:value];
if ([object[@"function"] isEqual:@"score"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketScoreAggregate read:value];
if ([object[@"function"] isEqual:@"filter"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketFilterAggregate read:value];
if ([object[@"function"] isEqual:@"rank"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketRankAggregate read:value];
if ([object[@"function"] isEqual:@"find"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketFindAggregate read:value];
if ([object[@"function"] isEqual:@"annotate"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketAnnotateAggregate read:value];
if ([object[@"function"] isEqual:@"recognize"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketRecognizeAggregate read:value];
if ([object[@"function"] isEqual:@"relate"] && [object[@"kind"] isEqual:@"aggregate"]) return (id)[TTSessionPacketRelateAggregate read:value];
if ([object[@"kind"] isEqual:@"observation"]) return (id)[TTSessionPacketObservation read:value];
if ([object[@"kind"] isEqual:@"terminal"]) return (id)[TTSessionPacketTerminal read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTSessionProbabilities
+ (instancetype)read:(id)value { NSDictionary *object = TTObject(value);
if ([object[@"kind"] isEqual:@"yes_no"]) return (id)[TTSessionProbabilitiesYesNo read:value];
if ([object[@"kind"] isEqual:@"named"]) return (id)[TTSessionProbabilitiesNamed read:value];
TTInvalidResult(); return nil;
}
@end
@implementation TTSessionQuestionDetail
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)cached { return [self presence:@"cached" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSNumber *> *)confidence { return [self presence:@"confidence" required:NO convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSNumber *> *)failedQuestions { return [self presence:@"failed_questions" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<TTFailure *> *)failure { return [self presence:@"failure" required:NO convert:^id(id value) { return [TTFailure read:value]; }]; }
- (TTPresence<NSString *> *)failureId { return [self presence:@"failure_id" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<id> *)input { return [self presence:@"input" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<TTPhysicalSource *> *)inputSource { return [self presence:@"input_source" required:NO convert:^id(id value) { return [TTPhysicalSource read:value]; }]; }
- (TTPresence<NSArray<TTSessionInputSource *> *> *)inputSources { return [self presence:@"input_sources" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTSessionInputSource read:item]; }); }]; }
- (TTPresence<NSArray<id> *> *)inputs { return [self presence:@"inputs" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return item; }); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTObservation *> *> *)observations { return [self presence:@"observations" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTObservation read:item]; }); }]; }
- (TTPresence<TTSessionProbabilities *> *)probabilities { return [self presence:@"probabilities" required:NO convert:^id(id value) { return [TTSessionProbabilities read:value]; }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSString *> *)questionSha256 { return [self presence:@"question_sha256" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources { return [self presence:@"question_sources" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTQuestionSource read:item]; }); }]; }
- (TTPresence<NSString *> *)rawPick { return [self presence:@"raw_pick" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTUsage *> *)reportedUsage { return [self presence:@"reported_usage" required:NO convert:^id(id value) { return [TTUsage read:value]; }]; }
- (TTPresence<NSArray<NSString *> *> *)requests { return [self presence:@"requests" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSNumber *> *)requestsSent { return [self presence:@"requests_sent" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:NO convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<NSString *> *)url { return [self presence:@"url" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTTokenUsage *> *)usage { return [self presence:@"usage" required:NO convert:^id(id value) { return [TTTokenUsage read:value]; }]; }
- (TTPresence<TTValue *> *)value { return [self presence:@"value" required:NO convert:^id(id value) { return [TTValue read:value]; }]; }
@end
@implementation TTSessionRecognition
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSArray<TTEntity *> *> *)entities { return [self presence:@"entities" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTEntity read:item]; }); }]; }
- (TTPresence<NSString *> *)mode { return [self presence:@"mode" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTBoundaryProposal *> *> *)proposals { return [self presence:@"proposals" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTBoundaryProposal read:item]; }); }]; }
- (TTPresence<NSArray<TTRecognitionEdgeDocument *> *> *)relations { return [self presence:@"relations" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRecognitionEdgeDocument read:item]; }); }]; }
@end
@implementation TTSessionRelationEdge
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)either { return [self presence:@"either" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)relation { return [self presence:@"relation" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTEntityDocument *> *)source { return [self presence:@"source" required:YES convert:^id(id value) { return [TTEntityDocument read:value]; }]; }
- (TTPresence<TTEntityDocument *> *)target { return [self presence:@"target" required:YES convert:^id(id value) { return [TTEntityDocument read:value]; }]; }
@end
@implementation TTSourceRelationEndpoint
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)file { return [self presence:@"file" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)firstLine { return [self presence:@"first_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)lastLine { return [self presence:@"last_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)ordinal { return [self presence:@"ordinal" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)record { return [self presence:@"record" required:YES convert:^id(id value) { return value; }]; }
@end
@implementation TTThreshold
+ (instancetype)read:(id)value {
if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{@"value":TTNumber(value), @"kind":@"number"}];
if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{@"value":TTString(value), @"kind":@"string"}];
TTInvalidResult(); return nil;
}
- (id)value { return self.rawFields[@"value"]; }
- (NSString *)kind { return self.rawFields[@"kind"]; }
@end
@implementation TTTokenBand
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)lower { return [self presence:@"lower" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)upper { return [self presence:@"upper" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTTokenUsage
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)inputTokens { return [self presence:@"input_tokens" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)outputTokens { return [self presence:@"output_tokens" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTValue
+ (instancetype)read:(id)value {
if (TTIsBoolean(value)) return [[self alloc] initWithFields:@{@"value":TTBoolean(value), @"kind":@"boolean"}];
if (value == NSNull.null) return [[self alloc] initWithFields:@{@"value":value, @"kind":@"null"}];
if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{@"value":TTString(value), @"kind":@"string"}];
if ([value isKindOfClass:NSArray.class]) return [[self alloc] initWithFields:@{@"value":TTArray(value, ^id(id item) { return TTString(item); }), @"kind":@"array"}];
if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{@"value":TTNumber(value), @"kind":@"number"}];
TTInvalidResult(); return nil;
}
- (id)value { return self.rawFields[@"value"]; }
- (NSString *)kind { return self.rawFields[@"kind"]; }
@end
@implementation TTAnnotationMemberAnswerId
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTObservation *> *> *)observations { return [self presence:@"observations" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTObservation read:item]; }); }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources { return [self presence:@"question_sources" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTQuestionSource read:item]; }); }]; }
- (TTPresence<NSString *> *)request { return [self presence:@"request" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<TTUsage *> *)usage { return [self presence:@"usage" required:NO convert:^id(id value) { return [TTUsage read:value]; }]; }
- (TTPresence<TTValue *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTValue read:value]; }]; }
@end
@implementation TTAnnotationMemberFailureId
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTFailure *> *)failure { return [self presence:@"failure" required:YES convert:^id(id value) { return [TTFailure read:value]; }]; }
- (TTPresence<NSString *> *)failureId { return [self presence:@"failure_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTObservation *> *> *)observations { return [self presence:@"observations" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTObservation read:item]; }); }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources { return [self presence:@"question_sources" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTQuestionSource read:item]; }); }]; }
- (TTPresence<NSString *> *)request { return [self presence:@"request" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<TTUsage *> *)usage { return [self presence:@"usage" required:NO convert:^id(id value) { return [TTUsage read:value]; }]; }
@end
@implementation TTAnnotationValueChoice
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTAnnotationValueDecision
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
@end
@implementation TTAnnotationValueFailed
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTFailure *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTFailure read:value]; }]; }
@end
@implementation TTAnnotationValueScore
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTNumber(value); }]; }
@end
@implementation TTAnnotationValueTags
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
@end
@implementation TTEstimatedInputDenialAdditionalRequest
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)limit { return [self presence:@"limit" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTEstimatedInputDenialInitialRequest
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)limit { return [self presence:@"limit" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTEstimatedInputDenialRetry
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)lastStatus { return [self presence:@"last_status" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSNumber *> *)limit { return [self presence:@"limit" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTInputDeclarationObject
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSDictionary<NSString *, TTInputPropertyType *> *> *)properties { return [self presence:@"properties" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return [TTInputPropertyType read:item]; }); }]; }
- (TTPresence<NSArray<NSString *> *> *)required { return [self presence:@"required" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSString *> *)type { return [self presence:@"type" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTInputDeclarationString
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)type { return [self presence:@"type" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTInputPropertyTypeArray
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTStringRoot *> *)items { return [self presence:@"items" required:YES convert:^id(id value) { return [TTStringRoot read:value]; }]; }
- (TTPresence<NSString *> *)type { return [self presence:@"type" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTInputPropertyTypeBoolean
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)type { return [self presence:@"type" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTInputPropertyTypeNumber
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)type { return [self presence:@"type" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTInputPropertyTypeString
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)type { return [self presence:@"type" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTObservationFailureId
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)failureId { return [self presence:@"failure_id" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTObservationObservationId
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)observationId { return [self presence:@"observation_id" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTReadableQuestionChoose
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTBatch *> *)batch { return [self presence:@"batch" required:NO convert:^id(id value) { return [TTBatch read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)contextSchema { return [self presence:@"context_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)itemSchema { return [self presence:@"item_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails { return [self presence:@"label_details" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTLabel read:item]; }); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)on { return [self presence:@"on" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSString *> *)profile { return [self presence:@"profile" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)wordingVersion { return [self presence:@"wording_version" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)options { return [self presence:@"options" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<id> *)text { return [self presence:@"text" required:YES convert:^id(id value) { return value; }]; }
- (TTPresence<NSString *> *)verb { return [self presence:@"verb" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTReadableQuestionDecide
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTBatch *> *)batch { return [self presence:@"batch" required:NO convert:^id(id value) { return [TTBatch read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)contextSchema { return [self presence:@"context_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)itemSchema { return [self presence:@"item_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails { return [self presence:@"label_details" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTLabel read:item]; }); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)on { return [self presence:@"on" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSString *> *)profile { return [self presence:@"profile" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)wordingVersion { return [self presence:@"wording_version" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)falseValue { return [self presence:@"false" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<id> *)text { return [self presence:@"text" required:YES convert:^id(id value) { return value; }]; }
- (TTPresence<id> *)trueValue { return [self presence:@"true" required:NO convert:^id(id value) { return value; }]; }
- (TTPresence<NSString *> *)verb { return [self presence:@"verb" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTReadableQuestionScore
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTBatch *> *)batch { return [self presence:@"batch" required:NO convert:^id(id value) { return [TTBatch read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)contextSchema { return [self presence:@"context_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)itemSchema { return [self presence:@"item_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails { return [self presence:@"label_details" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTLabel read:item]; }); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)on { return [self presence:@"on" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSString *> *)profile { return [self presence:@"profile" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)wordingVersion { return [self presence:@"wording_version" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)levels { return [self presence:@"levels" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<id> *)text { return [self presence:@"text" required:YES convert:^id(id value) { return value; }]; }
- (TTPresence<NSString *> *)verb { return [self presence:@"verb" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTReadableQuestionTag
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTBatch *> *)batch { return [self presence:@"batch" required:NO convert:^id(id value) { return [TTBatch read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)contextSchema { return [self presence:@"context_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<TTInputDeclaration *> *)itemSchema { return [self presence:@"item_schema" required:NO convert:^id(id value) { return [TTInputDeclaration read:value]; }]; }
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails { return [self presence:@"label_details" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTLabel read:item]; }); }]; }
- (TTPresence<NSString *> *)model { return [self presence:@"model" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)on { return [self presence:@"on" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<NSString *> *)profile { return [self presence:@"profile" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)wordingVersion { return [self presence:@"wording_version" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)labels { return [self presence:@"labels" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
- (TTPresence<id> *)text { return [self presence:@"text" required:YES convert:^id(id value) { return value; }]; }
- (TTPresence<NSString *> *)verb { return [self presence:@"verb" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTRecognitionOddsFieldsNamesPairsPiecesProposals
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSArray<TTNameOdds *> *> *)names { return [self presence:@"names" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTNameOdds read:item]; }); }]; }
- (TTPresence<NSArray<TTPairOdds *> *> *)pairs { return [self presence:@"pairs" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTPairOdds read:item]; }); }]; }
- (TTPresence<NSArray<TTPieceOdds *> *> *)pieces { return [self presence:@"pieces" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTPieceOdds read:item]; }); }]; }
- (TTPresence<NSArray<TTRecognitionProposal *> *> *)proposals { return [self presence:@"proposals" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRecognitionProposal read:item]; }); }]; }
@end
@implementation TTRecognitionOddsFieldsPiecesProposals
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSArray<TTPieceOdds *> *> *)pieces { return [self presence:@"pieces" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTPieceOdds read:item]; }); }]; }
- (TTPresence<NSArray<TTBoundaryProposal *> *> *)proposals { return [self presence:@"proposals" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTBoundaryProposal read:item]; }); }]; }
@end
@implementation TTRelationMemberAnswerId
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)direction { return [self presence:@"direction" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)method { return [self presence:@"method" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTObservation *> *> *)observations { return [self presence:@"observations" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTObservation read:item]; }); }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources { return [self presence:@"question_sources" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTQuestionSource read:item]; }); }]; }
- (TTPresence<NSString *> *)reads { return [self presence:@"reads" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)relation { return [self presence:@"relation" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)request { return [self presence:@"request" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTRelatedEntity *> *)source { return [self presence:@"source" required:YES convert:^id(id value) { return [TTRelatedEntity read:value]; }]; }
- (TTPresence<TTRelatedEntity *> *)target { return [self presence:@"target" required:YES convert:^id(id value) { return [TTRelatedEntity read:value]; }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<TTUsage *> *)usage { return [self presence:@"usage" required:NO convert:^id(id value) { return [TTUsage read:value]; }]; }
- (TTPresence<NSNumber *> *)accepted { return [self presence:@"accepted" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
- (TTPresence<TTAnswer *> *)answer { return [self presence:@"answer" required:YES convert:^id(id value) { return [TTAnswer read:value]; }]; }
- (TTPresence<NSString *> *)answerId { return [self presence:@"answer_id" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
@end
@implementation TTRelationMemberFailureId
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)direction { return [self presence:@"direction" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)method { return [self presence:@"method" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTObservation *> *> *)observations { return [self presence:@"observations" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTObservation read:item]; }); }]; }
- (TTPresence<TTReadableQuestion *> *)question { return [self presence:@"question" required:YES convert:^id(id value) { return [TTReadableQuestion read:value]; }]; }
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources { return [self presence:@"question_sources" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTQuestionSource read:item]; }); }]; }
- (TTPresence<NSString *> *)reads { return [self presence:@"reads" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)relation { return [self presence:@"relation" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)request { return [self presence:@"request" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTRelatedEntity *> *)source { return [self presence:@"source" required:YES convert:^id(id value) { return [TTRelatedEntity read:value]; }]; }
- (TTPresence<TTRelatedEntity *> *)target { return [self presence:@"target" required:YES convert:^id(id value) { return [TTRelatedEntity read:value]; }]; }
- (TTPresence<TTThreshold *> *)threshold { return [self presence:@"threshold" required:YES convert:^id(id value) { return [TTThreshold read:value]; }]; }
- (TTPresence<TTUsage *> *)usage { return [self presence:@"usage" required:NO convert:^id(id value) { return [TTUsage read:value]; }]; }
- (TTPresence<TTFailure *> *)failure { return [self presence:@"failure" required:YES convert:^id(id value) { return [TTFailure read:value]; }]; }
- (TTPresence<NSString *> *)failureId { return [self presence:@"failure_id" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTSendBudgetDenialBeforeAdditionalSend
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTSendBudgetDenialBeforeFirstSend
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTSendBudgetDenialBeforeRetry
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)lastStatus { return [self presence:@"last_status" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTAnswerChoice
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)confidence { return [self presence:@"confidence" required:NO convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)pick { return [self presence:@"pick" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)probabilities { return [self presence:@"probabilities" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return TTNumber(item); }); }]; }
@end
@implementation TTAnswerScore
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)confidence { return [self presence:@"confidence" required:NO convert:^id(id value) { return TTNumber(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)level { return [self presence:@"level" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)probabilities { return [self presence:@"probabilities" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return TTNumber(item); }); }]; }
@end
@implementation TTAnswerTag
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)probabilities { return [self presence:@"probabilities" required:YES convert:^id(id value) { return TTMap(value, ^id(id item) { return TTNumber(item); }); }]; }
@end
@implementation TTAnswerYesNo
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)probability { return [self presence:@"probability" required:YES convert:^id(id value) { return TTNumber(value); }]; }
@end
@implementation TTRecognizeFieldsEntities
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSArray<TTEntity *> *> *)entities { return [self presence:@"entities" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTEntity read:item]; }); }]; }
- (TTPresence<NSArray<TTEntityEdge *> *> *)relations { return [self presence:@"relations" required:NO convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTEntityEdge read:item]; }); }]; }
@end
@implementation TTRecognizeFieldsModeProposals
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)mode { return [self presence:@"mode" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTBoundaryProposal *> *> *)proposals { return [self presence:@"proposals" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTBoundaryProposal read:item]; }); }]; }
@end
@implementation TTRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)file { return [self presence:@"file" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)firstLine { return [self presence:@"first_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)lastLine { return [self presence:@"last_line" required:NO convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)ordinal { return [self presence:@"ordinal" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<id> *)record { return [self presence:@"record" required:YES convert:^id(id value) { return value; }]; }
@end
@implementation TTRelatedEntityEdgePropertiesSourceFieldsKindName
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)name { return [self presence:@"name" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTSessionJudgmentChoice
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTSessionJudgmentDecision
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTBoolean(value); }]; }
@end
@implementation TTSessionJudgmentScore
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTNumber(value); }]; }
@end
@implementation TTSessionJudgmentTags
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<NSString *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }]; }
@end
@implementation TTSessionObservationQuestion
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTSessionQuestionDetail *> *)detail { return [self presence:@"detail" required:YES convert:^id(id value) { return [TTSessionQuestionDetail read:value]; }]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)member { return [self presence:@"member" required:NO convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)position { return [self presence:@"position" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)stage { return [self presence:@"stage" required:NO convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTSessionObservationRow
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSNumber *> *)index { return [self presence:@"index" required:YES convert:^id(id value) { return TTInteger(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTSessionObservedRow *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTSessionObservedRow read:value]; }]; }
@end
@implementation TTSessionObservedRowAnnotated
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTSessionAnnotation *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTSessionAnnotation read:item]; }); }]; }
@end
@implementation TTSessionObservedRowFind
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTInteger(value); }]; }
@end
@implementation TTSessionObservedRowJudgment
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTSessionJudgment *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTSessionJudgment read:value]; }]; }
@end
@implementation TTSessionObservedRowRecognized
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTSessionRecognition *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTSessionRecognition read:value]; }]; }
@end
@implementation TTSessionObservedRowRelations
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTSessionRelationEdge *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTSessionRelationEdge read:item]; }); }]; }
@end
@implementation TTSessionPacketAnnotateAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTAnnotation *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTAnnotation read:item]; }); }]; }
@end
@implementation TTSessionPacketAnnotateRow
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTAnnotation *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTAnnotation read:value]; }]; }
@end
@implementation TTSessionPacketChooseAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTAtomicNullableString *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTAtomicNullableString read:item]; }); }]; }
@end
@implementation TTSessionPacketChooseRow
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTAtomicNullableString *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTAtomicNullableString read:value]; }]; }
@end
@implementation TTSessionPacketDecideAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTAtomicDecideValue *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTAtomicDecideValue read:item]; }); }]; }
@end
@implementation TTSessionPacketDecideRow
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTAtomicDecideValue *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTAtomicDecideValue read:value]; }]; }
@end
@implementation TTSessionPacketFilterAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTAtomicBoolean *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTAtomicBoolean read:item]; }); }]; }
@end
@implementation TTSessionPacketFilterRow
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTAtomicBoolean *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTAtomicBoolean read:value]; }]; }
@end
@implementation TTSessionPacketFindAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTFind *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTFind read:value]; }]; }
@end
@implementation TTSessionPacketObservation
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTSessionObservation *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTSessionObservation read:value]; }]; }
@end
@implementation TTSessionPacketRankAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTAtomicNonZeroUsize *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTAtomicNonZeroUsize read:item]; }); }]; }
@end
@implementation TTSessionPacketRecognizeAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTRecognition *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTRecognition read:item]; }); }]; }
@end
@implementation TTSessionPacketRelateAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTRelation *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTRelation read:value]; }]; }
@end
@implementation TTSessionPacketScoreAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTAtomicDouble *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTAtomicDouble read:item]; }); }]; }
@end
@implementation TTSessionPacketScoreRow
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTAtomicDouble *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTAtomicDouble read:value]; }]; }
@end
@implementation TTSessionPacketTagAggregate
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTAtomicArrayOfString *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTAtomicArrayOfString read:item]; }); }]; }
@end
@implementation TTSessionPacketTagRow
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)function { return [self presence:@"function" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<TTAtomicArrayOfString *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return [TTAtomicArrayOfString read:value]; }]; }
@end
@implementation TTSessionPacketTerminal
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<TTFacts *> *)facts { return [self presence:@"facts" required:NO convert:^id(id value) { return [TTFacts read:value]; }]; }
- (TTPresence<TTCallError *> *)failure { return [self presence:@"failure" required:NO convert:^id(id value) { return [TTCallError read:value]; }]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
@end
@implementation TTSessionProbabilitiesNamed
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSArray<TTSessionNamedProbability *> *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTArray(value, ^id(id item) { return [TTSessionNamedProbability read:item]; }); }]; }
@end
@implementation TTSessionProbabilitiesYesNo
+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }
- (TTPresence<NSString *> *)kind { return [self presence:@"kind" required:YES convert:^id(id value) { return TTString(value); }]; }
- (TTPresence<NSNumber *> *)value { return [self presence:@"value" required:YES convert:^id(id value) { return TTNumber(value); }]; }
@end
NSInteger TTNativeErrorCode(NSString *kind) {
if ([kind isEqual:@"usage"]) return 1;
if ([kind isEqual:@"backend"]) return 2;
if ([kind isEqual:@"local"]) return 4;
if ([kind isEqual:@"cancelled"]) return 5;
if ([kind isEqual:@"deadline"]) return 3;
if ([kind isEqual:@"defect"]) return 6;
TTInvalidResult(); return 0;
}
