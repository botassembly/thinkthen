// Generated from the shared Rust result graph; do not edit.
#import "TTResults.g.h"
#import "TTResultPrivate.h"
NSString * const TTRequestVersion = @"thinkthen.request/1";
@implementation TTAnnotation
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSDictionary<NSString *, TTAnnotationMember *> *> *)answers {
    return [self presence:@"answers" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return [TTAnnotationMember read:item];
        });
    }];
}
- (TTPresence<NSString *> *)file {
    return [self presence:@"file" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)firstLine {
    return [self presence:@"first_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:YES convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSNumber *> *)lastLine {
    return [self presence:@"last_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTPosition *> *)position {
    return [self presence:@"position" required:NO convert:^id(id value) {
        return [TTPosition read:value];
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<NSDictionary<NSString *, TTAnnotatedField *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return [TTAnnotatedField read:item];
        });
    }];
}
@end
@implementation TTAnnotationMember
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if (object[@"answer_id"] != nil) return (id)[TTAnnotationMemberAnswerId read:value];
    if (object[@"failure_id"] != nil) return (id)[TTAnnotationMemberFailureId read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTAnnotationValue
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"kind"] isEqual:@"decision"]) return (id)[TTAnnotationValueDecision read:value];
    if ([object[@"kind"] isEqual:@"choice"]) return (id)[TTAnnotationValueChoice read:value];
    if ([object[@"kind"] isEqual:@"score"]) return (id)[TTAnnotationValueScore read:value];
    if ([object[@"kind"] isEqual:@"tags"]) return (id)[TTAnnotationValueTags read:value];
    if ([object[@"kind"] isEqual:@"failed"]) return (id)[TTAnnotationValueFailed read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTAnswers
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSArray<TTRelationMember *> *> *)questions {
    return [self presence:@"questions" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRelationMember read:item];
        });
    }];
}
@end
@implementation TTAtomicArrayOfString
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTImage *> *> *)images {
    return [self presence:@"images" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTImage read:item];
        });
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSArray<TTRankMember *> *> *)members {
    return [self presence:@"members" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRankMember read:item];
        });
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)questionName {
    return [self presence:@"question_name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<NSArray<NSString *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
@end
@implementation TTAtomicDecideValue
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTImage *> *> *)images {
    return [self presence:@"images" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTImage read:item];
        });
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSArray<TTRankMember *> *> *)members {
    return [self presence:@"members" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRankMember read:item];
        });
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)questionName {
    return [self presence:@"question_name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<id> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return value;
    }];
}
@end
@implementation TTAtomicNonZeroUsize
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTImage *> *> *)images {
    return [self presence:@"images" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTImage read:item];
        });
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSArray<TTRankMember *> *> *)members {
    return [self presence:@"members" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRankMember read:item];
        });
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)questionName {
    return [self presence:@"question_name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTAtomicNullableString
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTImage *> *> *)images {
    return [self presence:@"images" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTImage read:item];
        });
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSArray<TTRankMember *> *> *)members {
    return [self presence:@"members" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRankMember read:item];
        });
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)questionName {
    return [self presence:@"question_name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<NSString *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTAtomicBoolean
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTImage *> *> *)images {
    return [self presence:@"images" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTImage read:item];
        });
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSArray<TTRankMember *> *> *)members {
    return [self presence:@"members" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRankMember read:item];
        });
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)questionName {
    return [self presence:@"question_name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
@end
@implementation TTAtomicDouble
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTImage *> *> *)images {
    return [self presence:@"images" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTImage read:item];
        });
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSArray<TTRankMember *> *> *)members {
    return [self presence:@"members" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRankMember read:item];
        });
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)questionName {
    return [self presence:@"question_name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
@end
@implementation TTAttempt
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)ordinal {
    return [self presence:@"ordinal" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)outcome {
    return [self presence:@"outcome" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)requestId {
    return [self presence:@"request_id" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)requestSha256 {
    return [self presence:@"request_sha256" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)sdkRequestId {
    return [self presence:@"sdk_request_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)serverMs {
    return [self presence:@"server_ms" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)status {
    return [self presence:@"status" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)wallMs {
    return [self presence:@"wall_ms" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTBatch
+ (instancetype)read:(id)value {
    if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{
        @"value":TTInteger(value), @"kind":@"integer"
    }];
    if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{
        @"value":TTString(value), @"kind":@"string"
    }];
    TTInvalidResult();
    return nil;
}
- (id)value {
    return self.rawFields[@"value"];
}
- (NSString *)kind {
    return self.rawFields[@"kind"];
}
@end
@implementation TTBoundaryOdds
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSArray<TTPieceOdds *> *> *)pieces {
    return [self presence:@"pieces" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTPieceOdds read:item];
        });
    }];
}
- (TTPresence<NSArray<TTBoundaryProposal *> *> *)proposals {
    return [self presence:@"proposals" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTBoundaryProposal read:item];
        });
    }];
}
@end
@implementation TTBoundaryProposal
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)end {
    return [self presence:@"end" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)length {
    return [self presence:@"length" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSNumber *> *)start {
    return [self presence:@"start" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)text {
    return [self presence:@"text" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTCallError
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTError *> *)error {
    return [self presence:@"error" required:YES convert:^id(id value) {
        return [TTError read:value];
    }];
}
- (TTPresence<TTFacts *> *)facts {
    return [self presence:@"facts" required:NO convert:^id(id value) {
        return [TTFacts read:value];
    }];
}
@end
@implementation TTEntityDocument
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTError
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTEstimatedInputDenial *> *)estimatedInputDenial {
    return [self presence:@"estimated_input_denial" required:NO convert:^id(id value) {
        return [TTEstimatedInputDenial read:value];
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)message {
    return [self presence:@"message" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)retryable {
    return [self presence:@"retryable" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<TTSendBudgetDenial *> *)sendBudgetDenial {
    return [self presence:@"send_budget_denial" required:NO convert:^id(id value) {
        return [TTSendBudgetDenial read:value];
    }];
}
- (TTPresence<TTStopped *> *)stopped {
    return [self presence:@"stopped" required:YES convert:^id(id value) {
        return [TTStopped read:value];
    }];
}
@end
@implementation TTEstimatedInputDenial
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"kind"] isEqual:@"initial_request"]) return (id)[TTEstimatedInputDenialInitialRequest read:value];
    if ([object[@"kind"] isEqual:@"additional_request"]) return (id)[TTEstimatedInputDenialAdditionalRequest read:value];
    if ([object[@"kind"] isEqual:@"retry"]) return (id)[TTEstimatedInputDenialRetry read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTFacts
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSArray<TTAttempt *> *> *)attempts {
    return [self presence:@"attempts" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAttempt read:item];
        });
    }];
}
- (TTPresence<NSNumber *> *)cacheAnswers {
    return [self presence:@"cache_answers" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)callId {
    return [self presence:@"call_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)estimatedCostUsd {
    return [self presence:@"estimated_cost_usd" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)heldModelMismatch {
    return [self presence:@"held_model_mismatch" required:NO convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSNumber *> *)inputTokens {
    return [self presence:@"input_tokens" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)largestRequestBytes {
    return [self presence:@"largest_request_bytes" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)largestRequestEstimatedInputTokens {
    return [self presence:@"largest_request_estimated_input_tokens" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)outputTokens {
    return [self presence:@"output_tokens" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)records {
    return [self presence:@"records" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)requestsSent {
    return [self presence:@"requests_sent" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)seconds {
    return [self presence:@"seconds" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)tokenEstimateMethod {
    return [self presence:@"token_estimate_method" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPersistenceObservation *> *)usagePersistence {
    return [self presence:@"usage_persistence" required:NO convert:^id(id value) {
        return [TTPersistenceObservation read:value];
    }];
}
@end
@implementation TTFind
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTFindAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTFindAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTFindCandidate *> *> *)candidates {
    return [self presence:@"candidates" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTFindCandidate read:item];
        });
    }];
}
- (TTPresence<NSString *> *)file {
    return [self presence:@"file" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)firstLine {
    return [self presence:@"first_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)lastLine {
    return [self presence:@"last_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTPosition *> *)position {
    return [self presence:@"position" required:NO convert:^id(id value) {
        return [TTPosition read:value];
    }];
}
- (TTPresence<TTFindQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTFindQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<id> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<id> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return value;
    }];
}
@end
@implementation TTFindCandidate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:YES convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
@end
@implementation TTImage
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)base64 {
    return [self presence:@"base64" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)height {
    return [self presence:@"height" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)media {
    return [self presence:@"media" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)width {
    return [self presence:@"width" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTInputDeclaration
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"type"] isEqual:@"string"]) return (id)[TTInputDeclarationStringRoot read:value];
    if ([object[@"type"] isEqual:@"object"]) return (id)[TTInputDeclarationObjectRoot read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTInputPropertyType
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"type"] isEqual:@"string"]) return (id)[TTInputPropertyTypeString read:value];
    if ([object[@"type"] isEqual:@"number"]) return (id)[TTInputPropertyTypeNumber read:value];
    if ([object[@"type"] isEqual:@"boolean"]) return (id)[TTInputPropertyTypeBoolean read:value];
    if ([object[@"type"] isEqual:@"array"]) return (id)[TTInputPropertyTypeArray read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTLabel
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<id> *)descriptionValue {
    return [self presence:@"description" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTMeta
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)answeredBy {
    return [self presence:@"answered_by" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTAttempt *> *> *)attempts {
    return [self presence:@"attempts" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAttempt read:item];
        });
    }];
}
- (TTPresence<TTBatchSetting *> *)batchSetting {
    return [self presence:@"batch_setting" required:NO convert:^id(id value) {
        return [TTBatchSetting read:value];
    }];
}
- (TTPresence<TTBatchWarning *> *)batchWarning {
    return [self presence:@"batch_warning" required:NO convert:^id(id value) {
        return [TTBatchWarning read:value];
    }];
}
- (TTPresence<NSNumber *> *)cached {
    return [self presence:@"cached" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSString *> *)contextSha256 {
    return [self presence:@"context_sha256" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)failedQuestions {
    return [self presence:@"failed_questions" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTObservation *> *> *)observations {
    return [self presence:@"observations" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTObservation read:item];
        });
    }];
}
- (TTPresence<NSString *> *)origin {
    return [self presence:@"origin" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTProfileWarning *> *)profileWarning {
    return [self presence:@"profile_warning" required:NO convert:^id(id value) {
        return [TTProfileWarning read:value];
    }];
}
- (TTPresence<NSString *> *)questionSha256 {
    return [self presence:@"question_sha256" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources {
    return [self presence:@"question_sources" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTQuestionSource read:item];
        });
    }];
}
- (TTPresence<NSString *> *)questionsSha256 {
    return [self presence:@"questions_sha256" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)requests {
    return [self presence:@"requests" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSNumber *> *)requestsSent {
    return [self presence:@"requests_sent" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)tool {
    return [self presence:@"tool" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)url {
    return [self presence:@"url" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTUsage *> *)usage {
    return [self presence:@"usage" required:NO convert:^id(id value) {
        return [TTUsage read:value];
    }];
}
@end
@implementation TTObjectRoot
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSDictionary<NSString *, TTInputPropertyType *> *> *)properties {
    return [self presence:@"properties" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return [TTInputPropertyType read:item];
        });
    }];
}
- (TTPresence<NSArray<NSString *> *> *)required {
    return [self presence:@"required" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTObservation
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if (object[@"observation_id"] != nil) return (id)[TTObservationObservationId read:value];
    if (object[@"failure_id"] != nil) return (id)[TTObservationFailureId read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTPersistenceObservation
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)advice {
    return [self presence:@"advice" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)observedAt {
    return [self presence:@"observed_at" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)state {
    return [self presence:@"state" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTPhysicalSource
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)file {
    return [self presence:@"file" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)firstLine {
    return [self presence:@"first_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)lastLine {
    return [self presence:@"last_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTPosition
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)file {
    return [self presence:@"file" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)first {
    return [self presence:@"first" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)images {
    return [self presence:@"images" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSNumber *> *)last {
    return [self presence:@"last" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTQuestionSource
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)answeredBy {
    return [self presence:@"answered_by" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)batchSize {
    return [self presence:@"batch_size" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)origin {
    return [self presence:@"origin" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTRankMember
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTRankMemberResult *> *)result {
    return [self presence:@"result" required:YES convert:^id(id value) {
        return [TTRankMemberResult read:value];
    }];
}
@end
@implementation TTRankMemberResult
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTImage *> *> *)images {
    return [self presence:@"images" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTImage read:item];
        });
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<id> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTRecognition
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTRecognitionOdds *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTRecognitionOdds read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)file {
    return [self presence:@"file" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)firstLine {
    return [self presence:@"first_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSNumber *> *)lastLine {
    return [self presence:@"last_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTPosition *> *)position {
    return [self presence:@"position" required:NO convert:^id(id value) {
        return [TTPosition read:value];
    }];
}
- (TTPresence<TTRecognizeQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTRecognizeQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<TTRecognize *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTRecognize read:value];
    }];
}
@end
@implementation TTRecognitionEdgeDocument
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)either {
    return [self presence:@"either" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)relation {
    return [self presence:@"relation" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTEntity *> *)source {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return [TTEntity read:value];
    }];
}
- (TTPresence<TTEntity *> *)target {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return [TTEntity read:value];
    }];
}
@end
@implementation TTRecognitionOdds
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if (object[@"names"] != nil && object[@"pairs"] != nil && object[@"pieces"] != nil && object[@"proposals"] != nil) return (id)[TTRecognitionOddsRecognizeAnswer read:value];
    if (object[@"pieces"] != nil && object[@"proposals"] != nil && object[@"names"] == nil && object[@"pairs"] == nil) return (id)[TTRecognitionOddsBoundaryOdds read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTRecognitionProposal
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)end {
    return [self presence:@"end" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)kept {
    return [self presence:@"kept" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPlace *> *)selected {
    return [self presence:@"selected" required:NO convert:^id(id value) {
        return [TTPlace read:value];
    }];
}
- (TTPresence<NSNumber *> *)spanProbability {
    return [self presence:@"span_probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSNumber *> *)start {
    return [self presence:@"start" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)strength {
    return [self presence:@"strength" required:NO convert:^id(id value) {
        return TTNumber(value);
    }];
}
@end
@implementation TTRecognitionStageContext
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)boundary {
    return [self presence:@"boundary" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kindEdge {
    return [self presence:@"kind_edge" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)relation {
    return [self presence:@"relation" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTRelation
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTAnswers *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswers read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)file {
    return [self presence:@"file" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)firstLine {
    return [self presence:@"first_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSArray<TTSessionInputSource *> *> *)inputSources {
    return [self presence:@"input_sources" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTSessionInputSource read:item];
        });
    }];
}
- (TTPresence<NSNumber *> *)lastLine {
    return [self presence:@"last_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<TTMeta *> *)meta {
    return [self presence:@"meta" required:YES convert:^id(id value) {
        return [TTMeta read:value];
    }];
}
- (TTPresence<TTPosition *> *)position {
    return [self presence:@"position" required:NO convert:^id(id value) {
        return [TTPosition read:value];
    }];
}
- (TTPresence<TTRelateQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTRelateQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)schema {
    return [self presence:@"schema" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTRelatedEntityEdge *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRelatedEntityEdge read:item];
        });
    }];
}
@end
@implementation TTRelationEndpoint
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if (object[@"kind"] != nil && object[@"name"] != nil && object[@"file"] == nil && object[@"ordinal"] == nil && object[@"record"] == nil) return (id)[TTRelationEndpointRelatedEntity read:value];
    if (object[@"file"] != nil && object[@"kind"] != nil && object[@"name"] != nil && object[@"ordinal"] != nil && object[@"record"] != nil) return (id)[TTRelationEndpointSource read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTRelationMember
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if (object[@"answer_id"] != nil) return (id)[TTRelationMemberAnswerId read:value];
    if (object[@"failure_id"] != nil) return (id)[TTRelationMemberFailureId read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTSendBudgetDenial
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"kind"] isEqual:@"before_first_send"]) return (id)[TTSendBudgetDenialBeforeFirstSend read:value];
    if ([object[@"kind"] isEqual:@"before_additional_send"]) return (id)[TTSendBudgetDenialBeforeAdditionalSend read:value];
    if ([object[@"kind"] isEqual:@"before_retry"]) return (id)[TTSendBudgetDenialBeforeRetry read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTStopped
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)at {
    return [self presence:@"at" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)cause {
    return [self presence:@"cause" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)retryable {
    return [self presence:@"retryable" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSNumber *> *)status {
    return [self presence:@"status" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTStringRoot
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTUsage
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)inputTokens {
    return [self presence:@"input_tokens" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)outputTokens {
    return [self presence:@"output_tokens" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTAnnotatedField
+ (instancetype)read:(id)value {
    if (TTIsBoolean(value)) return [[self alloc] initWithFields:@{
        @"value":TTBoolean(value), @"kind":@"boolean"
    }];
    if (value == NSNull.null) return [[self alloc] initWithFields:@{
        @"value":value, @"kind":@"null"
    }];
    if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{
        @"value":TTString(value), @"kind":@"string"
    }];
    if ([value isKindOfClass:NSArray.class]) return [[self alloc] initWithFields:@{
        @"value":TTArray(value, ^id(id item) {
            return TTString(item);
        }), @"kind":@"array"
    }];
    if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{
        @"value":TTNumber(value), @"kind":@"number"
    }];
    if ([value isKindOfClass:NSDictionary.class]) return [[self alloc] initWithFields:@{
        @"value":[TTFailed read:value], @"kind":@"object"
    }];
    TTInvalidResult();
    return nil;
}
- (id)value {
    return self.rawFields[@"value"];
}
- (NSString *)kind {
    return self.rawFields[@"kind"];
}
@end
@implementation TTAnswer
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"kind"] isEqual:@"yes_no"]) return (id)[TTAnswerYesNo read:value];
    if ([object[@"kind"] isEqual:@"choice"]) return (id)[TTAnswerChoice read:value];
    if ([object[@"kind"] isEqual:@"tag"]) return (id)[TTAnswerTag read:value];
    if ([object[@"kind"] isEqual:@"score"]) return (id)[TTAnswerScore read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTBatchSetting
+ (instancetype)read:(id)value {
    if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{
        @"value":TTInteger(value), @"kind":@"integer"
    }];
    if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{
        @"value":TTString(value), @"kind":@"string"
    }];
    TTInvalidResult();
    return nil;
}
- (id)value {
    return self.rawFields[@"value"];
}
- (NSString *)kind {
    return self.rawFields[@"kind"];
}
@end
@implementation TTBatchWarning
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTBatchSetting *> *)running {
    return [self presence:@"running" required:YES convert:^id(id value) {
        return [TTBatchSetting read:value];
    }];
}
- (TTPresence<TTBatchSetting *> *)tunedFor {
    return [self presence:@"tuned_for" required:YES convert:^id(id value) {
        return [TTBatchSetting read:value];
    }];
}
@end
@implementation TTEntity
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)end {
    return [self presence:@"end" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)file {
    return [self presence:@"file" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)firstLine {
    return [self presence:@"first_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)lastLine {
    return [self presence:@"last_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)length {
    return [self presence:@"length" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)start {
    return [self presence:@"start" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)strength {
    return [self presence:@"strength" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)text {
    return [self presence:@"text" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTEntityEdge
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)either {
    return [self presence:@"either" required:NO convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)relation {
    return [self presence:@"relation" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTEntity *> *)source {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return [TTEntity read:value];
    }];
}
- (TTPresence<TTEntity *> *)target {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return [TTEntity read:value];
    }];
}
@end
@implementation TTFailed
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTFailure *> *)failed {
    return [self presence:@"failed" required:YES convert:^id(id value) {
        return [TTFailure read:value];
    }];
}
@end
@implementation TTFailure
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)cause {
    return [self presence:@"cause" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTFindAnswer
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)confidence {
    return [self presence:@"confidence" required:NO convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)pick {
    return [self presence:@"pick" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)probabilities {
    return [self presence:@"probabilities" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return TTNumber(item);
        });
    }];
}
@end
@implementation TTFindQuestionDocument
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTBatch *> *)batch {
    return [self presence:@"batch" required:NO convert:^id(id value) {
        return [TTBatch read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails {
    return [self presence:@"label_details" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTLabel read:item];
        });
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)none {
    return [self presence:@"none" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)on {
    return [self presence:@"on" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSString *> *)profile {
    return [self presence:@"profile" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<id> *)text {
    return [self presence:@"text" required:YES convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSString *> *)verb {
    return [self presence:@"verb" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)wordingVersion {
    return [self presence:@"wording_version" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTNameOdds
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)edges {
    return [self presence:@"edges" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return TTNumber(item);
        });
    }];
}
- (TTPresence<NSNumber *> *)end {
    return [self presence:@"end" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)kinds {
    return [self presence:@"kinds" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return TTNumber(item);
        });
    }];
}
- (TTPresence<NSNumber *> *)start {
    return [self presence:@"start" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTPairOdds
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)relation {
    return [self presence:@"relation" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTPlace *> *)source {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return [TTPlace read:value];
    }];
}
- (TTPresence<TTPlace *> *)target {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return [TTPlace read:value];
    }];
}
@end
@implementation TTPieceOdds
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)end {
    return [self presence:@"end" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)start {
    return [self presence:@"start" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)tags {
    return [self presence:@"tags" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return TTNumber(item);
        });
    }];
}
@end
@implementation TTPlace
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)end {
    return [self presence:@"end" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)start {
    return [self presence:@"start" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTPlan
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)estimatedBytes {
    return [self presence:@"estimated_bytes" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<TTTokenBand *> *)estimatedInputTokens {
    return [self presence:@"estimated_input_tokens" required:YES convert:^id(id value) {
        return [TTTokenBand read:value];
    }];
}
- (TTPresence<NSString *> *)firstBodyUtf8 {
    return [self presence:@"first_body_utf8" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)largestRequestBytes {
    return [self presence:@"largest_request_bytes" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)largestRequestEstimatedInputTokens {
    return [self presence:@"largest_request_estimated_input_tokens" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)records {
    return [self presence:@"records" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)requests {
    return [self presence:@"requests" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)tokenEstimateMethod {
    return [self presence:@"token_estimate_method" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)upperBound {
    return [self presence:@"upper_bound" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
@end
@implementation TTProfileWarning
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)running {
    return [self presence:@"running" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)tunedFor {
    return [self presence:@"tuned_for" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTQuestionDocument
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"verb"] isEqual:@"decide"]) return (id)[TTQuestionDocumentDecide read:value];
    if ([object[@"verb"] isEqual:@"choose"]) return (id)[TTQuestionDocumentChoose read:value];
    if ([object[@"verb"] isEqual:@"tag"]) return (id)[TTQuestionDocumentTag read:value];
    if ([object[@"verb"] isEqual:@"score"]) return (id)[TTQuestionDocumentScore read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTRecognize
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if (object[@"entities"] != nil && object[@"mode"] == nil && object[@"proposals"] == nil) return (id)[TTRecognizeEntities read:value];
    if (object[@"mode"] != nil && object[@"proposals"] != nil && object[@"entities"] == nil) return (id)[TTRecognizeModeProposals read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTRecognizeAnswer
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSArray<TTNameOdds *> *> *)names {
    return [self presence:@"names" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTNameOdds read:item];
        });
    }];
}
- (TTPresence<NSArray<TTPairOdds *> *> *)pairs {
    return [self presence:@"pairs" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTPairOdds read:item];
        });
    }];
}
- (TTPresence<NSArray<TTPieceOdds *> *> *)pieces {
    return [self presence:@"pieces" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTPieceOdds read:item];
        });
    }];
}
- (TTPresence<NSArray<TTRecognitionProposal *> *> *)proposals {
    return [self presence:@"proposals" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRecognitionProposal read:item];
        });
    }];
}
@end
@implementation TTRecognizeQuestionDocument
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTBatch *> *)batch {
    return [self presence:@"batch" required:NO convert:^id(id value) {
        return [TTBatch read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<id> *)entityDefinition {
    return [self presence:@"entity_definition" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<id> *)instructions {
    return [self presence:@"instructions" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<TTInputDeclaration *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<NSDictionary<NSString *, id> *> *)kinds {
    return [self presence:@"kinds" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return item;
        });
    }];
}
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails {
    return [self presence:@"label_details" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTLabel read:item];
        });
    }];
}
- (TTPresence<NSString *> *)mode {
    return [self presence:@"mode" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)on {
    return [self presence:@"on" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSString *> *)profile {
    return [self presence:@"profile" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTThreshold *> *)relationThreshold {
    return [self presence:@"relation_threshold" required:NO convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<NSArray<TTRelationRule *> *> *)relations {
    return [self presence:@"relations" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRelationRule read:item];
        });
    }];
}
- (TTPresence<NSNumber *> *)snippetPieces {
    return [self presence:@"snippet_pieces" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<TTRecognitionStageContext *> *)stageContext {
    return [self presence:@"stage_context" required:NO convert:^id(id value) {
        return [TTRecognitionStageContext read:value];
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<NSString *> *)verb {
    return [self presence:@"verb" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)wordingVersion {
    return [self presence:@"wording_version" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTRelateFields
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTRelateQuestionDocument
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTBatch *> *)batch {
    return [self presence:@"batch" required:NO convert:^id(id value) {
        return [TTBatch read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<TTRelateFields *> *)fields {
    return [self presence:@"fields" required:YES convert:^id(id value) {
        return [TTRelateFields read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails {
    return [self presence:@"label_details" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTLabel read:item];
        });
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)on {
    return [self presence:@"on" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSString *> *)profile {
    return [self presence:@"profile" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTRelationRule *> *> *)relations {
    return [self presence:@"relations" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRelationRule read:item];
        });
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<NSString *> *)verb {
    return [self presence:@"verb" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)wordingVersion {
    return [self presence:@"wording_version" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTRelatedEntity
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTRelatedEntityEdge
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)either {
    return [self presence:@"either" required:NO convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)relation {
    return [self presence:@"relation" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTRelationEndpoint *> *)source {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return [TTRelationEndpoint read:value];
    }];
}
- (TTPresence<TTRelationEndpoint *> *)target {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return [TTRelationEndpoint read:value];
    }];
}
- (TTPresence<NSString *> *)sourceName {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return TTString(TTObject(value)[@"name"]);
    }];
}
- (TTPresence<NSString *> *)targetName {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return TTString(TTObject(value)[@"name"]);
    }];
}
@end
@implementation TTRelationRule
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)either {
    return [self presence:@"either" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)reads {
    return [self presence:@"reads" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)single {
    return [self presence:@"single" required:NO convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSString *> *)source {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)target {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTSessionAnnotation
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTAnnotationValue *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTAnnotationValue read:value];
    }];
}
@end
@implementation TTSessionInputSource
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<TTPhysicalSource *> *)source {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
@end
@implementation TTSessionJudgment
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"kind"] isEqual:@"decision"]) return (id)[TTSessionJudgmentDecision read:value];
    if ([object[@"kind"] isEqual:@"choice"]) return (id)[TTSessionJudgmentChoice read:value];
    if ([object[@"kind"] isEqual:@"score"]) return (id)[TTSessionJudgmentScore read:value];
    if ([object[@"kind"] isEqual:@"tags"]) return (id)[TTSessionJudgmentTags read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTSessionNamedProbability
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
@end
@implementation TTSessionObservation
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"kind"] isEqual:@"question"]) return (id)[TTSessionObservationQuestion read:value];
    if ([object[@"kind"] isEqual:@"row"]) return (id)[TTSessionObservationRow read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTSessionObservedRow
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"kind"] isEqual:@"judgment"]) return (id)[TTSessionObservedRowJudgment read:value];
    if ([object[@"kind"] isEqual:@"annotated"]) return (id)[TTSessionObservedRowAnnotated read:value];
    if ([object[@"kind"] isEqual:@"recognized"]) return (id)[TTSessionObservedRowRecognized read:value];
    if ([object[@"kind"] isEqual:@"find"]) return (id)[TTSessionObservedRowFind read:value];
    if ([object[@"kind"] isEqual:@"relations"]) return (id)[TTSessionObservedRowRelations read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTSessionPacket
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
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
    TTInvalidResult();
    return nil;
}
@end
@implementation TTSessionProbabilities
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"kind"] isEqual:@"yes_no"]) return (id)[TTSessionProbabilitiesYesNo read:value];
    if ([object[@"kind"] isEqual:@"named"]) return (id)[TTSessionProbabilitiesNamed read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTSessionQuestionDetail
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)cached {
    return [self presence:@"cached" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSNumber *> *)confidence {
    return [self presence:@"confidence" required:NO convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSNumber *> *)failedQuestions {
    return [self presence:@"failed_questions" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<TTFailure *> *)failure {
    return [self presence:@"failure" required:NO convert:^id(id value) {
        return [TTFailure read:value];
    }];
}
- (TTPresence<NSString *> *)failureId {
    return [self presence:@"failure_id" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<id> *)input {
    return [self presence:@"input" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<TTPhysicalSource *> *)inputSource {
    return [self presence:@"input_source" required:NO convert:^id(id value) {
        return [TTPhysicalSource read:value];
    }];
}
- (TTPresence<NSArray<TTSessionInputSource *> *> *)inputSources {
    return [self presence:@"input_sources" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTSessionInputSource read:item];
        });
    }];
}
- (TTPresence<NSArray<id> *> *)inputs {
    return [self presence:@"inputs" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return item;
        });
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTObservation *> *> *)observations {
    return [self presence:@"observations" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTObservation read:item];
        });
    }];
}
- (TTPresence<TTSessionProbabilities *> *)probabilities {
    return [self presence:@"probabilities" required:NO convert:^id(id value) {
        return [TTSessionProbabilities read:value];
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSString *> *)questionSha256 {
    return [self presence:@"question_sha256" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources {
    return [self presence:@"question_sources" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTQuestionSource read:item];
        });
    }];
}
- (TTPresence<NSString *> *)rawPick {
    return [self presence:@"raw_pick" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTUsage *> *)reportedUsage {
    return [self presence:@"reported_usage" required:NO convert:^id(id value) {
        return [TTUsage read:value];
    }];
}
- (TTPresence<NSArray<NSString *> *> *)requests {
    return [self presence:@"requests" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSNumber *> *)requestsSent {
    return [self presence:@"requests_sent" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:NO convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<NSString *> *)url {
    return [self presence:@"url" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTTokenUsage *> *)usage {
    return [self presence:@"usage" required:NO convert:^id(id value) {
        return [TTTokenUsage read:value];
    }];
}
- (TTPresence<TTValue *> *)value {
    return [self presence:@"value" required:NO convert:^id(id value) {
        return [TTValue read:value];
    }];
}
@end
@implementation TTSessionRecognition
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSArray<TTEntity *> *> *)entities {
    return [self presence:@"entities" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTEntity read:item];
        });
    }];
}
- (TTPresence<NSString *> *)mode {
    return [self presence:@"mode" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTBoundaryProposal *> *> *)proposals {
    return [self presence:@"proposals" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTBoundaryProposal read:item];
        });
    }];
}
- (TTPresence<NSArray<TTRecognitionEdgeDocument *> *> *)relations {
    return [self presence:@"relations" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRecognitionEdgeDocument read:item];
        });
    }];
}
@end
@implementation TTSessionRelationEdge
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)either {
    return [self presence:@"either" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)relation {
    return [self presence:@"relation" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTEntityDocument *> *)source {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return [TTEntityDocument read:value];
    }];
}
- (TTPresence<TTEntityDocument *> *)target {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return [TTEntityDocument read:value];
    }];
}
- (TTPresence<NSString *> *)sourceName {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return TTString(TTObject(value)[@"name"]);
    }];
}
- (TTPresence<NSString *> *)targetName {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return TTString(TTObject(value)[@"name"]);
    }];
}
@end
@implementation TTSourceRelationEndpoint
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)file {
    return [self presence:@"file" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)firstLine {
    return [self presence:@"first_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)lastLine {
    return [self presence:@"last_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)ordinal {
    return [self presence:@"ordinal" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)record {
    return [self presence:@"record" required:YES convert:^id(id value) {
        return value;
    }];
}
@end
@implementation TTThreshold
+ (instancetype)read:(id)value {
    if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{
        @"value":TTNumber(value), @"kind":@"number"
    }];
    if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{
        @"value":TTString(value), @"kind":@"string"
    }];
    TTInvalidResult();
    return nil;
}
- (id)value {
    return self.rawFields[@"value"];
}
- (NSString *)kind {
    return self.rawFields[@"kind"];
}
@end
@implementation TTTokenBand
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)lower {
    return [self presence:@"lower" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)upper {
    return [self presence:@"upper" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTTokenUsage
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)inputTokens {
    return [self presence:@"input_tokens" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)outputTokens {
    return [self presence:@"output_tokens" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTValue
+ (instancetype)read:(id)value {
    if (TTIsBoolean(value)) return [[self alloc] initWithFields:@{
        @"value":TTBoolean(value), @"kind":@"boolean"
    }];
    if (value == NSNull.null) return [[self alloc] initWithFields:@{
        @"value":value, @"kind":@"null"
    }];
    if ([value isKindOfClass:NSString.class]) return [[self alloc] initWithFields:@{
        @"value":TTString(value), @"kind":@"string"
    }];
    if ([value isKindOfClass:NSArray.class]) return [[self alloc] initWithFields:@{
        @"value":TTArray(value, ^id(id item) {
            return TTString(item);
        }), @"kind":@"array"
    }];
    if ([value isKindOfClass:NSNumber.class] && !TTIsBoolean(value)) return [[self alloc] initWithFields:@{
        @"value":TTNumber(value), @"kind":@"number"
    }];
    TTInvalidResult();
    return nil;
}
- (id)value {
    return self.rawFields[@"value"];
}
- (NSString *)kind {
    return self.rawFields[@"kind"];
}
@end
@implementation TTAnnotationMemberAnswerId
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTObservation *> *> *)observations {
    return [self presence:@"observations" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTObservation read:item];
        });
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources {
    return [self presence:@"question_sources" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTQuestionSource read:item];
        });
    }];
}
- (TTPresence<NSString *> *)request {
    return [self presence:@"request" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<TTUsage *> *)usage {
    return [self presence:@"usage" required:NO convert:^id(id value) {
        return [TTUsage read:value];
    }];
}
- (TTPresence<TTValue *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTValue read:value];
    }];
}
@end
@implementation TTAnnotationMemberFailureId
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTFailure *> *)failure {
    return [self presence:@"failure" required:YES convert:^id(id value) {
        return [TTFailure read:value];
    }];
}
- (TTPresence<NSString *> *)failureId {
    return [self presence:@"failure_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTObservation *> *> *)observations {
    return [self presence:@"observations" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTObservation read:item];
        });
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources {
    return [self presence:@"question_sources" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTQuestionSource read:item];
        });
    }];
}
- (TTPresence<NSString *> *)request {
    return [self presence:@"request" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<TTUsage *> *)usage {
    return [self presence:@"usage" required:NO convert:^id(id value) {
        return [TTUsage read:value];
    }];
}
@end
@implementation TTAnnotationValueChoice
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTAnnotationValueDecision
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
@end
@implementation TTAnnotationValueFailed
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTFailure *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTFailure read:value];
    }];
}
@end
@implementation TTAnnotationValueScore
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
@end
@implementation TTAnnotationValueTags
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
@end
@implementation TTEstimatedInputDenialAdditionalRequest
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)limit {
    return [self presence:@"limit" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTEstimatedInputDenialInitialRequest
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)limit {
    return [self presence:@"limit" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTEstimatedInputDenialRetry
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)lastStatus {
    return [self presence:@"last_status" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSNumber *> *)limit {
    return [self presence:@"limit" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTInputDeclarationObjectRoot
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSDictionary<NSString *, TTInputPropertyType *> *> *)properties {
    return [self presence:@"properties" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return [TTInputPropertyType read:item];
        });
    }];
}
- (TTPresence<NSArray<NSString *> *> *)required {
    return [self presence:@"required" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTInputDeclarationStringRoot
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTInputPropertyTypeArray
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTStringRoot *> *)items {
    return [self presence:@"items" required:YES convert:^id(id value) {
        return [TTStringRoot read:value];
    }];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTInputPropertyTypeBoolean
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTInputPropertyTypeNumber
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTInputPropertyTypeString
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTObservationFailureId
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)failureId {
    return [self presence:@"failure_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTObservationObservationId
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)observationId {
    return [self presence:@"observation_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTRecognitionOddsBoundaryOdds
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSArray<TTPieceOdds *> *> *)pieces {
    return [self presence:@"pieces" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTPieceOdds read:item];
        });
    }];
}
- (TTPresence<NSArray<TTBoundaryProposal *> *> *)proposals {
    return [self presence:@"proposals" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTBoundaryProposal read:item];
        });
    }];
}
@end
@implementation TTRecognitionOddsRecognizeAnswer
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSArray<TTNameOdds *> *> *)names {
    return [self presence:@"names" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTNameOdds read:item];
        });
    }];
}
- (TTPresence<NSArray<TTPairOdds *> *> *)pairs {
    return [self presence:@"pairs" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTPairOdds read:item];
        });
    }];
}
- (TTPresence<NSArray<TTPieceOdds *> *> *)pieces {
    return [self presence:@"pieces" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTPieceOdds read:item];
        });
    }];
}
- (TTPresence<NSArray<TTRecognitionProposal *> *> *)proposals {
    return [self presence:@"proposals" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRecognitionProposal read:item];
        });
    }];
}
@end
@implementation TTRelationEndpointRelatedEntity
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTRelationEndpointSource
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)file {
    return [self presence:@"file" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)firstLine {
    return [self presence:@"first_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)lastLine {
    return [self presence:@"last_line" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)ordinal {
    return [self presence:@"ordinal" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)record {
    return [self presence:@"record" required:YES convert:^id(id value) {
        return value;
    }];
}
@end
@implementation TTRelationMemberAnswerId
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)direction {
    return [self presence:@"direction" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)method {
    return [self presence:@"method" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTObservation *> *> *)observations {
    return [self presence:@"observations" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTObservation read:item];
        });
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources {
    return [self presence:@"question_sources" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTQuestionSource read:item];
        });
    }];
}
- (TTPresence<NSString *> *)reads {
    return [self presence:@"reads" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)relation {
    return [self presence:@"relation" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)request {
    return [self presence:@"request" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTRelatedEntity *> *)source {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return [TTRelatedEntity read:value];
    }];
}
- (TTPresence<TTRelatedEntity *> *)target {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return [TTRelatedEntity read:value];
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<TTUsage *> *)usage {
    return [self presence:@"usage" required:NO convert:^id(id value) {
        return [TTUsage read:value];
    }];
}
- (TTPresence<NSNumber *> *)accepted {
    return [self presence:@"accepted" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
- (TTPresence<TTAnswer *> *)answer {
    return [self presence:@"answer" required:YES convert:^id(id value) {
        return [TTAnswer read:value];
    }];
}
- (TTPresence<NSString *> *)answerId {
    return [self presence:@"answer_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)sourceName {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return TTString(TTObject(value)[@"name"]);
    }];
}
@end
@implementation TTRelationMemberFailureId
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)direction {
    return [self presence:@"direction" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)method {
    return [self presence:@"method" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTObservation *> *> *)observations {
    return [self presence:@"observations" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTObservation read:item];
        });
    }];
}
- (TTPresence<TTQuestionDocument *> *)question {
    return [self presence:@"question" required:YES convert:^id(id value) {
        return [TTQuestionDocument read:value];
    }];
}
- (TTPresence<NSArray<TTQuestionSource *> *> *)questionSources {
    return [self presence:@"question_sources" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTQuestionSource read:item];
        });
    }];
}
- (TTPresence<NSString *> *)reads {
    return [self presence:@"reads" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)relation {
    return [self presence:@"relation" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)request {
    return [self presence:@"request" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTRelatedEntity *> *)source {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return [TTRelatedEntity read:value];
    }];
}
- (TTPresence<TTRelatedEntity *> *)target {
    return [self presence:@"target" required:YES convert:^id(id value) {
        return [TTRelatedEntity read:value];
    }];
}
- (TTPresence<TTThreshold *> *)threshold {
    return [self presence:@"threshold" required:YES convert:^id(id value) {
        return [TTThreshold read:value];
    }];
}
- (TTPresence<TTUsage *> *)usage {
    return [self presence:@"usage" required:NO convert:^id(id value) {
        return [TTUsage read:value];
    }];
}
- (TTPresence<TTFailure *> *)failure {
    return [self presence:@"failure" required:YES convert:^id(id value) {
        return [TTFailure read:value];
    }];
}
- (TTPresence<NSString *> *)failureId {
    return [self presence:@"failure_id" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)sourceName {
    return [self presence:@"source" required:YES convert:^id(id value) {
        return TTString(TTObject(value)[@"name"]);
    }];
}
@end
@implementation TTSendBudgetDenialBeforeAdditionalSend
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTSendBudgetDenialBeforeFirstSend
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTSendBudgetDenialBeforeRetry
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)lastStatus {
    return [self presence:@"last_status" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTAnswerChoice
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)confidence {
    return [self presence:@"confidence" required:NO convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)pick {
    return [self presence:@"pick" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)probabilities {
    return [self presence:@"probabilities" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return TTNumber(item);
        });
    }];
}
@end
@implementation TTAnswerScore
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)confidence {
    return [self presence:@"confidence" required:NO convert:^id(id value) {
        return TTNumber(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)level {
    return [self presence:@"level" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)probabilities {
    return [self presence:@"probabilities" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return TTNumber(item);
        });
    }];
}
@end
@implementation TTAnswerTag
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSDictionary<NSString *, NSNumber *> *> *)probabilities {
    return [self presence:@"probabilities" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return TTNumber(item);
        });
    }];
}
@end
@implementation TTAnswerYesNo
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)probability {
    return [self presence:@"probability" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
@end
@implementation TTQuestionDocumentChoose
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTBatch *> *)batch {
    return [self presence:@"batch" required:NO convert:^id(id value) {
        return [TTBatch read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails {
    return [self presence:@"label_details" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTLabel read:item];
        });
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)on {
    return [self presence:@"on" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSString *> *)profile {
    return [self presence:@"profile" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)wordingVersion {
    return [self presence:@"wording_version" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)options {
    return [self presence:@"options" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<id> *)text {
    return [self presence:@"text" required:YES convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSString *> *)verb {
    return [self presence:@"verb" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTQuestionDocumentDecide
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTBatch *> *)batch {
    return [self presence:@"batch" required:NO convert:^id(id value) {
        return [TTBatch read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails {
    return [self presence:@"label_details" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTLabel read:item];
        });
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)on {
    return [self presence:@"on" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSString *> *)profile {
    return [self presence:@"profile" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)wordingVersion {
    return [self presence:@"wording_version" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<id> *)falseValue {
    return [self presence:@"false" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<id> *)text {
    return [self presence:@"text" required:YES convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<id> *)trueValue {
    return [self presence:@"true" required:NO convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSString *> *)verb {
    return [self presence:@"verb" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTQuestionDocumentScore
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTBatch *> *)batch {
    return [self presence:@"batch" required:NO convert:^id(id value) {
        return [TTBatch read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails {
    return [self presence:@"label_details" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTLabel read:item];
        });
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)on {
    return [self presence:@"on" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSString *> *)profile {
    return [self presence:@"profile" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)wordingVersion {
    return [self presence:@"wording_version" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)levels {
    return [self presence:@"levels" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<id> *)text {
    return [self presence:@"text" required:YES convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSString *> *)verb {
    return [self presence:@"verb" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTQuestionDocumentTag
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTBatch *> *)batch {
    return [self presence:@"batch" required:NO convert:^id(id value) {
        return [TTBatch read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<TTInputDeclaration *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTInputDeclaration read:value];
    }];
}
- (TTPresence<NSArray<TTLabel *> *> *)labelDetails {
    return [self presence:@"label_details" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTLabel read:item];
        });
    }];
}
- (TTPresence<NSString *> *)model {
    return [self presence:@"model" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)name {
    return [self presence:@"name" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)on {
    return [self presence:@"on" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<NSString *> *)profile {
    return [self presence:@"profile" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)wordingVersion {
    return [self presence:@"wording_version" required:NO convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)labels {
    return [self presence:@"labels" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
- (TTPresence<id> *)text {
    return [self presence:@"text" required:YES convert:^id(id value) {
        return value;
    }];
}
- (TTPresence<NSString *> *)verb {
    return [self presence:@"verb" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTRecognizeEntities
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSArray<TTEntity *> *> *)entities {
    return [self presence:@"entities" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTEntity read:item];
        });
    }];
}
- (TTPresence<NSArray<TTEntityEdge *> *> *)relations {
    return [self presence:@"relations" required:NO convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTEntityEdge read:item];
        });
    }];
}
@end
@implementation TTRecognizeModeProposals
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)mode {
    return [self presence:@"mode" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTBoundaryProposal *> *> *)proposals {
    return [self presence:@"proposals" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTBoundaryProposal read:item];
        });
    }];
}
@end
@implementation TTSessionJudgmentChoice
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTSessionJudgmentDecision
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTBoolean(value);
    }];
}
@end
@implementation TTSessionJudgmentScore
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
@end
@implementation TTSessionJudgmentTags
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<NSString *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return TTString(item);
        });
    }];
}
@end
@implementation TTSessionObservationQuestion
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTSessionQuestionDetail *> *)detail {
    return [self presence:@"detail" required:YES convert:^id(id value) {
        return [TTSessionQuestionDetail read:value];
    }];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)member {
    return [self presence:@"member" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)position {
    return [self presence:@"position" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)stage {
    return [self presence:@"stage" required:NO convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTSessionObservationRow
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSNumber *> *)index {
    return [self presence:@"index" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTSessionObservedRow *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTSessionObservedRow read:value];
    }];
}
@end
@implementation TTSessionObservedRowAnnotated
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTSessionAnnotation *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTSessionAnnotation read:item];
        });
    }];
}
@end
@implementation TTSessionObservedRowFind
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTInteger(value);
    }];
}
@end
@implementation TTSessionObservedRowJudgment
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTSessionJudgment *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTSessionJudgment read:value];
    }];
}
@end
@implementation TTSessionObservedRowRecognized
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTSessionRecognition *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTSessionRecognition read:value];
    }];
}
@end
@implementation TTSessionObservedRowRelations
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTSessionRelationEdge *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTSessionRelationEdge read:item];
        });
    }];
}
@end
@implementation TTSessionPacketAnnotateAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTAnnotation *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAnnotation read:item];
        });
    }];
}
@end
@implementation TTSessionPacketAnnotateRow
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTAnnotation *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTAnnotation read:value];
    }];
}
@end
@implementation TTSessionPacketChooseAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTAtomicNullableString *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicNullableString read:item];
        });
    }];
}
@end
@implementation TTSessionPacketChooseRow
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTAtomicNullableString *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTAtomicNullableString read:value];
    }];
}
@end
@implementation TTSessionPacketDecideAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTAtomicDecideValue *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicDecideValue read:item];
        });
    }];
}
@end
@implementation TTSessionPacketDecideRow
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTAtomicDecideValue *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTAtomicDecideValue read:value];
    }];
}
@end
@implementation TTSessionPacketFilterAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTAtomicBoolean *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicBoolean read:item];
        });
    }];
}
@end
@implementation TTSessionPacketFilterRow
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTAtomicBoolean *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTAtomicBoolean read:value];
    }];
}
@end
@implementation TTSessionPacketFindAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTFind *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTFind read:value];
    }];
}
@end
@implementation TTSessionPacketObservation
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTSessionObservation *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTSessionObservation read:value];
    }];
}
@end
@implementation TTSessionPacketRankAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTAtomicNonZeroUsize *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicNonZeroUsize read:item];
        });
    }];
}
@end
@implementation TTSessionPacketRecognizeAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTRecognition *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRecognition read:item];
        });
    }];
}
@end
@implementation TTSessionPacketRelateAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTRelation *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTRelation read:value];
    }];
}
@end
@implementation TTSessionPacketScoreAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTAtomicDouble *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicDouble read:item];
        });
    }];
}
@end
@implementation TTSessionPacketScoreRow
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTAtomicDouble *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTAtomicDouble read:value];
    }];
}
@end
@implementation TTSessionPacketTagAggregate
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTAtomicArrayOfString *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicArrayOfString read:item];
        });
    }];
}
@end
@implementation TTSessionPacketTagRow
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)function {
    return [self presence:@"function" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<TTAtomicArrayOfString *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTAtomicArrayOfString read:value];
    }];
}
@end
@implementation TTSessionPacketTerminal
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<TTFacts *> *)facts {
    return [self presence:@"facts" required:NO convert:^id(id value) {
        return [TTFacts read:value];
    }];
}
- (TTPresence<TTCallError *> *)failure {
    return [self presence:@"failure" required:NO convert:^id(id value) {
        return [TTCallError read:value];
    }];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTSessionProbabilitiesNamed
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSArray<TTSessionNamedProbability *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTSessionNamedProbability read:item];
        });
    }];
}
@end
@implementation TTSessionProbabilitiesYesNo
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)kind {
    return [self presence:@"kind" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
- (TTPresence<NSNumber *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTNumber(value);
    }];
}
@end
NSInteger TTNativeErrorCode(NSString *kind) {
    if ([kind isEqual:@"usage"]) return 1;
    if ([kind isEqual:@"backend"]) return 2;
    if ([kind isEqual:@"local"]) return 4;
    if ([kind isEqual:@"cancelled"]) return 5;
    if ([kind isEqual:@"deadline"]) return 3;
    if ([kind isEqual:@"defect"]) return 6;
    TTInvalidResult();
    return 0;
}
@implementation TTSessionPacket (Answers)
- (TTPresence<NSArray<TTAtomicDecideValue *> *> *)decideAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"decide"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicDecideValue read:item];
        });
    }];
}
- (TTPresence<NSArray<TTAtomicNullableString *> *> *)chooseAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"choose"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicNullableString read:item];
        });
    }];
}
- (TTPresence<NSArray<TTAtomicArrayOfString *> *> *)tagAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"tag"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicArrayOfString read:item];
        });
    }];
}
- (TTPresence<NSArray<TTAtomicDouble *> *> *)scoreAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"score"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicDouble read:item];
        });
    }];
}
- (TTPresence<NSArray<TTAtomicBoolean *> *> *)filterAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"filter"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicBoolean read:item];
        });
    }];
}
- (TTPresence<NSArray<TTAtomicNonZeroUsize *> *> *)rankAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"rank"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAtomicNonZeroUsize read:item];
        });
    }];
}
- (TTPresence<TTFind *> *)findAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"find"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTFind read:value];
    }];
}
- (TTPresence<NSArray<TTAnnotation *> *> *)annotateAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"annotate"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTAnnotation read:item];
        });
    }];
}
- (TTPresence<NSArray<TTRecognition *> *> *)recognizeAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"recognize"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRecognition read:item];
        });
    }];
}
- (TTPresence<TTRelation *> *)relateAnswer {
    if (![self.rawFields[@"kind"] isEqual:@"aggregate"] || ![self.rawFields[@"function"] isEqual:@"relate"]) TTInvalidResult();
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTRelation read:value];
    }];
}
@end
@implementation TTOrderedEntry
- (instancetype)initWithKey:(NSString *)key value:(id)value {
    if ((self = [super init])) {
        _key = [key copy];
        _value = value;
    } return self;
}
@end
@implementation TTOrderedMap
- (instancetype)initWithEntries:(NSArray<TTOrderedEntry *> *)entries {
    if ((self = [super init])) {
        _entries = [entries copy];
    } return self;
}
@end
NSData *TTRequestJSONData(id value, NSError **error) {
    if ([value isKindOfClass:TTRequestObject.class]) value = ((TTRequestObject *)value).requestFields;
    NSArray<TTOrderedEntry *> *members = nil;
    if ([value isKindOfClass:TTOrderedMap.class]) members = ((TTOrderedMap *)value).entries;
    else if ([value isKindOfClass:NSDictionary.class]) {
        NSMutableArray *entries = [NSMutableArray array];
        for (NSString *key in value) [entries addObject:[[TTOrderedEntry alloc] initWithKey:key value:value[key]]];
        members = entries;
    }
    BOOL array = [value isKindOfClass:NSArray.class];
    if (!members && !array) return [NSJSONSerialization dataWithJSONObject:value options:NSJSONWritingFragmentsAllowed error:error];
    NSMutableData *data = [NSMutableData data];
    [data appendData:[(array ? @"[" : @"{") dataUsingEncoding:NSUTF8StringEncoding]];
    NSArray *values = array ? value : members;
    for (NSUInteger index = 0;
    index < values.count;
    index++) {
        if (index) [data appendData:[@"," dataUsingEncoding:NSUTF8StringEncoding]];
        id item = values[index];
        if (!array) {
            TTOrderedEntry *entry = item;
            NSData *key = TTRequestJSONData(entry.key, error);
            if (!key) return nil;
            [data appendData:key];
            [data appendData:[@":" dataUsingEncoding:NSUTF8StringEncoding]];
            item = entry.value;
        }
        NSData *encoded = TTRequestJSONData(item, error);
        if (!encoded) return nil;
        [data appendData:encoded];
    }
    [data appendData:[(array ? @"]" : @"}") dataUsingEncoding:NSUTF8StringEncoding]];
    return data;
}
@implementation TTRequestObject
- (instancetype)init {
    if ((self = [super init])) _requestFields = [NSMutableDictionary dictionary];
    return self;
}
@end
TTInputImageMedia const TTInputImageMediaImageJpeg = @"image/jpeg";
TTInputImageMedia const TTInputImageMediaImagePng = @"image/png";
TTInputReaderMedia const TTInputReaderMediaText = @"text";
TTInputReaderMedia const TTInputReaderMediaImage = @"image";
TTInputRecognitionMode const TTInputRecognitionModeWhole = @"whole";
TTInputRecognitionMode const TTInputRecognitionModeBoundaryOnly = @"boundary_only";
TTInputRequestFraming const TTInputRequestFramingDocument = @"document";
TTInputRequestFraming const TTInputRequestFramingLines = @"lines";
TTInputRequestFraming const TTInputRequestFramingJsonl = @"jsonl";
TTInputRequestFraming const TTInputRequestFramingCsv = @"csv";
TTInputRequestFraming const TTInputRequestFramingTsv = @"tsv";
TTInputRequestFunction const TTInputRequestFunctionDecide = @"decide";
TTInputRequestFunction const TTInputRequestFunctionChoose = @"choose";
TTInputRequestFunction const TTInputRequestFunctionTag = @"tag";
TTInputRequestFunction const TTInputRequestFunctionScore = @"score";
TTInputRequestFunction const TTInputRequestFunctionFilter = @"filter";
TTInputRequestFunction const TTInputRequestFunctionRank = @"rank";
TTInputRequestFunction const TTInputRequestFunctionFind = @"find";
TTInputRequestFunction const TTInputRequestFunctionAnnotate = @"annotate";
TTInputRequestFunction const TTInputRequestFunctionRecognize = @"recognize";
TTInputRequestFunction const TTInputRequestFunctionRelate = @"relate";
TTInputRequestVersion const TTInputRequestVersionThinkthenRequest1 = @"thinkthen.request/1";
TTInputSourceUnit const TTInputSourceUnitLine = @"line";
TTInputSourceUnit const TTInputSourceUnitWindow = @"window";
TTInputSourceUnit const TTInputSourceUnitFile = @"file";
@implementation TTInputAnnotateDefinition
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (TTOrderedMap *)questions {
    return self.requestFields[@"questions"];
}
- (void)setQuestions:(TTOrderedMap *)value {
    if (value) self.requestFields[@"questions"] = value;
    else [self.requestFields removeObjectForKey:@"questions"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"version"] = @(1);
    } return self;
}
@end
@implementation TTInputAnnotateQuestionChoose
- (id)choose {
    return self.requestFields[@"choose"];
}
- (void)setChoose:(id)value {
    if (value) self.requestFields[@"choose"] = value;
    else [self.requestFields removeObjectForKey:@"choose"];
}
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (id)options {
    return self.requestFields[@"options"];
}
- (void)setOptions:(id)value {
    if (value) self.requestFields[@"options"] = value;
    else [self.requestFields removeObjectForKey:@"options"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputAnnotateQuestionDecide
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)decide {
    return self.requestFields[@"decide"];
}
- (void)setDecide:(id)value {
    if (value) self.requestFields[@"decide"] = value;
    else [self.requestFields removeObjectForKey:@"decide"];
}
- (id)falseValue {
    return self.requestFields[@"false"];
}
- (void)setFalseValue:(id)value {
    if (value) self.requestFields[@"false"] = value;
    else [self.requestFields removeObjectForKey:@"false"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (id)trueValue {
    return self.requestFields[@"true"];
}
- (void)setTrueValue:(id)value {
    if (value) self.requestFields[@"true"] = value;
    else [self.requestFields removeObjectForKey:@"true"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputAnnotateQuestionScore
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (id)levels {
    return self.requestFields[@"levels"];
}
- (void)setLevels:(id)value {
    if (value) self.requestFields[@"levels"] = value;
    else [self.requestFields removeObjectForKey:@"levels"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (id)score {
    return self.requestFields[@"score"];
}
- (void)setScore:(id)value {
    if (value) self.requestFields[@"score"] = value;
    else [self.requestFields removeObjectForKey:@"score"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputAnnotateQuestionTag
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (id)labels {
    return self.requestFields[@"labels"];
}
- (void)setLabels:(id)value {
    if (value) self.requestFields[@"labels"] = value;
    else [self.requestFields removeObjectForKey:@"labels"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (id)tag {
    return self.requestFields[@"tag"];
}
- (void)setTag:(id)value {
    if (value) self.requestFields[@"tag"] = value;
    else [self.requestFields removeObjectForKey:@"tag"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputAuthoredChoose
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (id)choose {
    return self.requestFields[@"choose"];
}
- (void)setChoose:(id)value {
    if (value) self.requestFields[@"choose"] = value;
    else [self.requestFields removeObjectForKey:@"choose"];
}
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (id)options {
    return self.requestFields[@"options"];
}
- (void)setOptions:(id)value {
    if (value) self.requestFields[@"options"] = value;
    else [self.requestFields removeObjectForKey:@"options"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputAuthoredDecide
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)decide {
    return self.requestFields[@"decide"];
}
- (void)setDecide:(id)value {
    if (value) self.requestFields[@"decide"] = value;
    else [self.requestFields removeObjectForKey:@"decide"];
}
- (id)falseValue {
    return self.requestFields[@"false"];
}
- (void)setFalseValue:(id)value {
    if (value) self.requestFields[@"false"] = value;
    else [self.requestFields removeObjectForKey:@"false"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (id)trueValue {
    return self.requestFields[@"true"];
}
- (void)setTrueValue:(id)value {
    if (value) self.requestFields[@"true"] = value;
    else [self.requestFields removeObjectForKey:@"true"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputAuthoredFind
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)find {
    return self.requestFields[@"find"];
}
- (void)setFind:(id)value {
    if (value) self.requestFields[@"find"] = value;
    else [self.requestFields removeObjectForKey:@"find"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputAuthoredInputDeclarationObject
- (NSDictionary *)properties {
    return self.requestFields[@"properties"];
}
- (void)setProperties:(NSDictionary *)value {
    if (value) self.requestFields[@"properties"] = value;
    else [self.requestFields removeObjectForKey:@"properties"];
}
- (NSArray *)required {
    return self.requestFields[@"required"];
}
- (void)setRequired:(NSArray *)value {
    if (value) self.requestFields[@"required"] = value;
    else [self.requestFields removeObjectForKey:@"required"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"object";
    } return self;
}
@end
@implementation TTInputAuthoredInputDeclarationString
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"string";
    } return self;
}
@end
@implementation TTInputAuthoredInputPropertyArray
- (NSDictionary *)items {
    return self.requestFields[@"items"];
}
- (void)setItems:(NSDictionary *)value {
    if (value) self.requestFields[@"items"] = value;
    else [self.requestFields removeObjectForKey:@"items"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"array";
    } return self;
}
@end
@implementation TTInputAuthoredInputPropertyBoolean
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"boolean";
    } return self;
}
@end
@implementation TTInputAuthoredInputPropertyNumber
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"number";
    } return self;
}
@end
@implementation TTInputAuthoredInputPropertyString
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"string";
    } return self;
}
@end
@implementation TTInputAuthoredRelate
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (TTInputRelateQuestion *)relate {
    return self.requestFields[@"relate"];
}
- (void)setRelate:(TTInputRelateQuestion *)value {
    if (value) self.requestFields[@"relate"] = value;
    else [self.requestFields removeObjectForKey:@"relate"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"version"] = @(1);
    } return self;
}
@end
@implementation TTInputAuthoredRelation
- (NSNumber *)either {
    return self.requestFields[@"either"];
}
- (void)setEither:(NSNumber *)value {
    if (value) self.requestFields[@"either"] = value;
    else [self.requestFields removeObjectForKey:@"either"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (NSString *)reads {
    return self.requestFields[@"reads"];
}
- (void)setReads:(NSString *)value {
    if (value) self.requestFields[@"reads"] = value;
    else [self.requestFields removeObjectForKey:@"reads"];
}
- (NSNumber *)single {
    return self.requestFields[@"single"];
}
- (void)setSingle:(NSNumber *)value {
    if (value) self.requestFields[@"single"] = value;
    else [self.requestFields removeObjectForKey:@"single"];
}
- (NSString *)source {
    return self.requestFields[@"source"];
}
- (void)setSource:(NSString *)value {
    if (value) self.requestFields[@"source"] = value;
    else [self.requestFields removeObjectForKey:@"source"];
}
- (NSString *)target {
    return self.requestFields[@"target"];
}
- (void)setTarget:(NSString *)value {
    if (value) self.requestFields[@"target"] = value;
    else [self.requestFields removeObjectForKey:@"target"];
}
@end
@implementation TTInputAuthoredScore
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (id)levels {
    return self.requestFields[@"levels"];
}
- (void)setLevels:(id)value {
    if (value) self.requestFields[@"levels"] = value;
    else [self.requestFields removeObjectForKey:@"levels"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (id)score {
    return self.requestFields[@"score"];
}
- (void)setScore:(id)value {
    if (value) self.requestFields[@"score"] = value;
    else [self.requestFields removeObjectForKey:@"score"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputAuthoredTag
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (id)labels {
    return self.requestFields[@"labels"];
}
- (void)setLabels:(id)value {
    if (value) self.requestFields[@"labels"] = value;
    else [self.requestFields removeObjectForKey:@"labels"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (id)tag {
    return self.requestFields[@"tag"];
}
- (void)setTag:(id)value {
    if (value) self.requestFields[@"tag"] = value;
    else [self.requestFields removeObjectForKey:@"tag"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputEngineSettings
- (NSString *)backend {
    return self.requestFields[@"backend"];
}
- (void)setBackend:(NSString *)value {
    if (value) self.requestFields[@"backend"] = value;
    else [self.requestFields removeObjectForKey:@"backend"];
}
- (NSString *)baseUrl {
    return self.requestFields[@"base_url"];
}
- (void)setBaseUrl:(NSString *)value {
    if (value) self.requestFields[@"base_url"] = value;
    else [self.requestFields removeObjectForKey:@"base_url"];
}
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (id)cache {
    return self.requestFields[@"cache"];
}
- (void)setCache:(id)value {
    if (value) self.requestFields[@"cache"] = value;
    else [self.requestFields removeObjectForKey:@"cache"];
}
- (id)maxEstimatedInputTokensTotal {
    return self.requestFields[@"max_estimated_input_tokens_total"];
}
- (void)setMaxEstimatedInputTokensTotal:(id)value {
    if (value) self.requestFields[@"max_estimated_input_tokens_total"] = value;
    else [self.requestFields removeObjectForKey:@"max_estimated_input_tokens_total"];
}
- (NSNumber *)maxRequestBytes {
    return self.requestFields[@"max_request_bytes"];
}
- (void)setMaxRequestBytes:(NSNumber *)value {
    if (value) self.requestFields[@"max_request_bytes"] = value;
    else [self.requestFields removeObjectForKey:@"max_request_bytes"];
}
- (id)maxRequests {
    return self.requestFields[@"max_requests"];
}
- (void)setMaxRequests:(id)value {
    if (value) self.requestFields[@"max_requests"] = value;
    else [self.requestFields removeObjectForKey:@"max_requests"];
}
- (id)maxRequestsTotal {
    return self.requestFields[@"max_requests_total"];
}
- (void)setMaxRequestsTotal:(id)value {
    if (value) self.requestFields[@"max_requests_total"] = value;
    else [self.requestFields removeObjectForKey:@"max_requests_total"];
}
- (NSNumber *)maxRetries {
    return self.requestFields[@"max_retries"];
}
- (void)setMaxRetries:(NSNumber *)value {
    if (value) self.requestFields[@"max_retries"] = value;
    else [self.requestFields removeObjectForKey:@"max_retries"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (id)proxy {
    return self.requestFields[@"proxy"];
}
- (void)setProxy:(id)value {
    if (value) self.requestFields[@"proxy"] = value;
    else [self.requestFields removeObjectForKey:@"proxy"];
}
- (NSString *)record {
    return self.requestFields[@"record"];
}
- (void)setRecord:(NSString *)value {
    if (value) self.requestFields[@"record"] = value;
    else [self.requestFields removeObjectForKey:@"record"];
}
- (NSNumber *)refreshCache {
    return self.requestFields[@"refresh_cache"];
}
- (void)setRefreshCache:(NSNumber *)value {
    if (value) self.requestFields[@"refresh_cache"] = value;
    else [self.requestFields removeObjectForKey:@"refresh_cache"];
}
- (NSString *)replay {
    return self.requestFields[@"replay"];
}
- (void)setReplay:(NSString *)value {
    if (value) self.requestFields[@"replay"] = value;
    else [self.requestFields removeObjectForKey:@"replay"];
}
- (NSNumber *)throttle {
    return self.requestFields[@"throttle"];
}
- (void)setThrottle:(NSNumber *)value {
    if (value) self.requestFields[@"throttle"] = value;
    else [self.requestFields removeObjectForKey:@"throttle"];
}
- (NSNumber *)timeout {
    return self.requestFields[@"timeout"];
}
- (void)setTimeout:(NSNumber *)value {
    if (value) self.requestFields[@"timeout"] = value;
    else [self.requestFields removeObjectForKey:@"timeout"];
}
- (NSString *)usdPerMillionInput {
    return self.requestFields[@"usd_per_million_input"];
}
- (void)setUsdPerMillionInput:(NSString *)value {
    if (value) self.requestFields[@"usd_per_million_input"] = value;
    else [self.requestFields removeObjectForKey:@"usd_per_million_input"];
}
- (NSString *)usdPerMillionOutput {
    return self.requestFields[@"usd_per_million_output"];
}
- (void)setUsdPerMillionOutput:(NSString *)value {
    if (value) self.requestFields[@"usd_per_million_output"] = value;
    else [self.requestFields removeObjectForKey:@"usd_per_million_output"];
}
@end
@implementation TTInputOptionSchema
- (id)descriptionValue {
    return self.requestFields[@"description"];
}
- (void)setDescriptionValue:(id)value {
    if (value) self.requestFields[@"description"] = value;
    else [self.requestFields removeObjectForKey:@"description"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
@end
@implementation TTInputRecognitionExampleEntity
- (NSNumber *)end {
    return self.requestFields[@"end"];
}
- (void)setEnd:(NSNumber *)value {
    if (value) self.requestFields[@"end"] = value;
    else [self.requestFields removeObjectForKey:@"end"];
}
- (NSString *)kind {
    return self.requestFields[@"kind"];
}
- (void)setKind:(NSString *)value {
    if (value) self.requestFields[@"kind"] = value;
    else [self.requestFields removeObjectForKey:@"kind"];
}
- (NSNumber *)start {
    return self.requestFields[@"start"];
}
- (void)setStart:(NSNumber *)value {
    if (value) self.requestFields[@"start"] = value;
    else [self.requestFields removeObjectForKey:@"start"];
}
@end
@implementation TTInputRecognitionExampleText
- (NSArray *)entities {
    return self.requestFields[@"entities"];
}
- (void)setEntities:(NSArray *)value {
    if (value) self.requestFields[@"entities"] = value;
    else [self.requestFields removeObjectForKey:@"entities"];
}
- (NSArray *)kinds {
    return self.requestFields[@"kinds"];
}
- (void)setKinds:(NSArray *)value {
    if (value) self.requestFields[@"kinds"] = value;
    else [self.requestFields removeObjectForKey:@"kinds"];
}
- (NSString *)text {
    return self.requestFields[@"text"];
}
- (void)setText:(NSString *)value {
    if (value) self.requestFields[@"text"] = value;
    else [self.requestFields removeObjectForKey:@"text"];
}
@end
@implementation TTInputRecognitionSeedSpan
- (NSNumber *)end {
    return self.requestFields[@"end"];
}
- (void)setEnd:(NSNumber *)value {
    if (value) self.requestFields[@"end"] = value;
    else [self.requestFields removeObjectForKey:@"end"];
}
- (NSString *)kind {
    return self.requestFields[@"kind"];
}
- (void)setKind:(NSString *)value {
    if (value) self.requestFields[@"kind"] = value;
    else [self.requestFields removeObjectForKey:@"kind"];
}
- (NSNumber *)start {
    return self.requestFields[@"start"];
}
- (void)setStart:(NSNumber *)value {
    if (value) self.requestFields[@"start"] = value;
    else [self.requestFields removeObjectForKey:@"start"];
}
@end
@implementation TTInputRecognitionStageContext
- (NSString *)boundary {
    return self.requestFields[@"boundary"];
}
- (void)setBoundary:(NSString *)value {
    if (value) self.requestFields[@"boundary"] = value;
    else [self.requestFields removeObjectForKey:@"boundary"];
}
- (NSString *)kindEdge {
    return self.requestFields[@"kind_edge"];
}
- (void)setKindEdge:(NSString *)value {
    if (value) self.requestFields[@"kind_edge"] = value;
    else [self.requestFields removeObjectForKey:@"kind_edge"];
}
- (NSString *)relation {
    return self.requestFields[@"relation"];
}
- (void)setRelation:(NSString *)value {
    if (value) self.requestFields[@"relation"] = value;
    else [self.requestFields removeObjectForKey:@"relation"];
}
@end
@implementation TTInputRecognizeDefinition
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (TTInputRecognizeQuestion *)recognize {
    return self.requestFields[@"recognize"];
}
- (void)setRecognize:(TTInputRecognizeQuestion *)value {
    if (value) self.requestFields[@"recognize"] = value;
    else [self.requestFields removeObjectForKey:@"recognize"];
}
- (id)relationThreshold {
    return self.requestFields[@"relation_threshold"];
}
- (void)setRelationThreshold:(id)value {
    if (value) self.requestFields[@"relation_threshold"] = value;
    else [self.requestFields removeObjectForKey:@"relation_threshold"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"version"] = @(1);
    } return self;
}
@end
@implementation TTInputRecognizeQuestion
- (id)entityDefinition {
    return self.requestFields[@"entity_definition"];
}
- (void)setEntityDefinition:(id)value {
    if (value) self.requestFields[@"entity_definition"] = value;
    else [self.requestFields removeObjectForKey:@"entity_definition"];
}
- (id)instructions {
    return self.requestFields[@"instructions"];
}
- (void)setInstructions:(id)value {
    if (value) self.requestFields[@"instructions"] = value;
    else [self.requestFields removeObjectForKey:@"instructions"];
}
- (TTOrderedMap *)kinds {
    return self.requestFields[@"kinds"];
}
- (void)setKinds:(TTOrderedMap *)value {
    if (value) self.requestFields[@"kinds"] = value;
    else [self.requestFields removeObjectForKey:@"kinds"];
}
- (TTInputRecognitionMode)mode {
    return self.requestFields[@"mode"];
}
- (void)setMode:(TTInputRecognitionMode)value {
    if (value) self.requestFields[@"mode"] = value;
    else [self.requestFields removeObjectForKey:@"mode"];
}
- (NSArray *)relations {
    return self.requestFields[@"relations"];
}
- (void)setRelations:(NSArray *)value {
    if (value) self.requestFields[@"relations"] = value;
    else [self.requestFields removeObjectForKey:@"relations"];
}
- (NSNumber *)snippetPieces {
    return self.requestFields[@"snippet_pieces"];
}
- (void)setSnippetPieces:(NSNumber *)value {
    if (value) self.requestFields[@"snippet_pieces"] = value;
    else [self.requestFields removeObjectForKey:@"snippet_pieces"];
}
- (TTInputRecognitionStageContext *)stageContext {
    return self.requestFields[@"stage_context"];
}
- (void)setStageContext:(TTInputRecognitionStageContext *)value {
    if (value) self.requestFields[@"stage_context"] = value;
    else [self.requestFields removeObjectForKey:@"stage_context"];
}
@end
@implementation TTInputRelateFields
- (NSString *)kind {
    return self.requestFields[@"kind"];
}
- (void)setKind:(NSString *)value {
    if (value) self.requestFields[@"kind"] = value;
    else [self.requestFields removeObjectForKey:@"kind"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
@end
@implementation TTInputRelateQuestion
- (TTInputRelateFields *)fields {
    return self.requestFields[@"fields"];
}
- (void)setFields:(TTInputRelateFields *)value {
    if (value) self.requestFields[@"fields"] = value;
    else [self.requestFields removeObjectForKey:@"fields"];
}
- (NSArray *)relations {
    return self.requestFields[@"relations"];
}
- (void)setRelations:(NSArray *)value {
    if (value) self.requestFields[@"relations"] = value;
    else [self.requestFields removeObjectForKey:@"relations"];
}
@end
@implementation TTInputRequest
- (TTInputRequestCall *)call {
    return self.requestFields[@"call"];
}
- (void)setCall:(TTInputRequestCall *)value {
    if (value) self.requestFields[@"call"] = value;
    else [self.requestFields removeObjectForKey:@"call"];
}
- (TTInputRequestVersion)schema {
    return self.requestFields[@"schema"];
}
- (void)setSchema:(TTInputRequestVersion)value {
    if (value) self.requestFields[@"schema"] = value;
    else [self.requestFields removeObjectForKey:@"schema"];
}
@end
@implementation TTInputRequestCall
- (TTInputRequestFunction)function {
    return self.requestFields[@"function"];
}
- (void)setFunction:(TTInputRequestFunction)value {
    if (value) self.requestFields[@"function"] = value;
    else [self.requestFields removeObjectForKey:@"function"];
}
- (id)input {
    return self.requestFields[@"input"];
}
- (void)setInput:(id)value {
    if (value) self.requestFields[@"input"] = value;
    else [self.requestFields removeObjectForKey:@"input"];
}
- (TTInputRequestOptions *)options {
    return self.requestFields[@"options"];
}
- (void)setOptions:(TTInputRequestOptions *)value {
    if (value) self.requestFields[@"options"] = value;
    else [self.requestFields removeObjectForKey:@"options"];
}
- (id)question {
    return self.requestFields[@"question"];
}
- (void)setQuestion:(id)value {
    if (value) self.requestFields[@"question"] = value;
    else [self.requestFields removeObjectForKey:@"question"];
}
@end
@implementation TTInputRequestDefinitionAnnotateDefinition
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (TTOrderedMap *)questions {
    return self.requestFields[@"questions"];
}
- (void)setQuestions:(TTOrderedMap *)value {
    if (value) self.requestFields[@"questions"] = value;
    else [self.requestFields removeObjectForKey:@"questions"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"version"] = @(1);
    } return self;
}
@end
@implementation TTInputRequestDefinitionAuthoredChoose
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (id)choose {
    return self.requestFields[@"choose"];
}
- (void)setChoose:(id)value {
    if (value) self.requestFields[@"choose"] = value;
    else [self.requestFields removeObjectForKey:@"choose"];
}
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (id)options {
    return self.requestFields[@"options"];
}
- (void)setOptions:(id)value {
    if (value) self.requestFields[@"options"] = value;
    else [self.requestFields removeObjectForKey:@"options"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputRequestDefinitionAuthoredDecide
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)decide {
    return self.requestFields[@"decide"];
}
- (void)setDecide:(id)value {
    if (value) self.requestFields[@"decide"] = value;
    else [self.requestFields removeObjectForKey:@"decide"];
}
- (id)falseValue {
    return self.requestFields[@"false"];
}
- (void)setFalseValue:(id)value {
    if (value) self.requestFields[@"false"] = value;
    else [self.requestFields removeObjectForKey:@"false"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (id)trueValue {
    return self.requestFields[@"true"];
}
- (void)setTrueValue:(id)value {
    if (value) self.requestFields[@"true"] = value;
    else [self.requestFields removeObjectForKey:@"true"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputRequestDefinitionAuthoredFind
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)find {
    return self.requestFields[@"find"];
}
- (void)setFind:(id)value {
    if (value) self.requestFields[@"find"] = value;
    else [self.requestFields removeObjectForKey:@"find"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputRequestDefinitionAuthoredRelate
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (TTInputRelateQuestion *)relate {
    return self.requestFields[@"relate"];
}
- (void)setRelate:(TTInputRelateQuestion *)value {
    if (value) self.requestFields[@"relate"] = value;
    else [self.requestFields removeObjectForKey:@"relate"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"version"] = @(1);
    } return self;
}
@end
@implementation TTInputRequestDefinitionAuthoredScore
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (id)levels {
    return self.requestFields[@"levels"];
}
- (void)setLevels:(id)value {
    if (value) self.requestFields[@"levels"] = value;
    else [self.requestFields removeObjectForKey:@"levels"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (id)score {
    return self.requestFields[@"score"];
}
- (void)setScore:(id)value {
    if (value) self.requestFields[@"score"] = value;
    else [self.requestFields removeObjectForKey:@"score"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputRequestDefinitionAuthoredTag
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (id)labels {
    return self.requestFields[@"labels"];
}
- (void)setLabels:(id)value {
    if (value) self.requestFields[@"labels"] = value;
    else [self.requestFields removeObjectForKey:@"labels"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (id)tag {
    return self.requestFields[@"tag"];
}
- (void)setTag:(id)value {
    if (value) self.requestFields[@"tag"] = value;
    else [self.requestFields removeObjectForKey:@"tag"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
@end
@implementation TTInputRequestDefinitionRecognizeDefinition
- (id)contextSchema {
    return self.requestFields[@"context_schema"];
}
- (void)setContextSchema:(id)value {
    if (value) self.requestFields[@"context_schema"] = value;
    else [self.requestFields removeObjectForKey:@"context_schema"];
}
- (id)itemSchema {
    return self.requestFields[@"item_schema"];
}
- (void)setItemSchema:(id)value {
    if (value) self.requestFields[@"item_schema"] = value;
    else [self.requestFields removeObjectForKey:@"item_schema"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (id)on {
    return self.requestFields[@"on"];
}
- (void)setOn:(id)value {
    if (value) self.requestFields[@"on"] = value;
    else [self.requestFields removeObjectForKey:@"on"];
}
- (NSString *)profile {
    return self.requestFields[@"profile"];
}
- (void)setProfile:(NSString *)value {
    if (value) self.requestFields[@"profile"] = value;
    else [self.requestFields removeObjectForKey:@"profile"];
}
- (TTInputRecognizeQuestion *)recognize {
    return self.requestFields[@"recognize"];
}
- (void)setRecognize:(TTInputRecognizeQuestion *)value {
    if (value) self.requestFields[@"recognize"] = value;
    else [self.requestFields removeObjectForKey:@"recognize"];
}
- (id)relationThreshold {
    return self.requestFields[@"relation_threshold"];
}
- (void)setRelationThreshold:(id)value {
    if (value) self.requestFields[@"relation_threshold"] = value;
    else [self.requestFields removeObjectForKey:@"relation_threshold"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)wordingVersion {
    return self.requestFields[@"wording_version"];
}
- (void)setWordingVersion:(NSNumber *)value {
    if (value) self.requestFields[@"wording_version"] = value;
    else [self.requestFields removeObjectForKey:@"wording_version"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"version"] = @(1);
    } return self;
}
@end
@implementation TTInputRequestImageBytes
- (NSString *)bytes {
    return self.requestFields[@"bytes"];
}
- (void)setBytes:(NSString *)value {
    if (value) self.requestFields[@"bytes"] = value;
    else [self.requestFields removeObjectForKey:@"bytes"];
}
- (TTInputImageMedia)media {
    return self.requestFields[@"media"];
}
- (void)setMedia:(TTInputImageMedia)value {
    if (value) self.requestFields[@"media"] = value;
    else [self.requestFields removeObjectForKey:@"media"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"bytes";
    } return self;
}
@end
@implementation TTInputRequestImageFile
- (TTInputImageMedia)media {
    return self.requestFields[@"media"];
}
- (void)setMedia:(TTInputImageMedia)value {
    if (value) self.requestFields[@"media"] = value;
    else [self.requestFields removeObjectForKey:@"media"];
}
- (NSString *)path {
    return self.requestFields[@"path"];
}
- (void)setPath:(NSString *)value {
    if (value) self.requestFields[@"path"] = value;
    else [self.requestFields removeObjectForKey:@"path"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"file";
    } return self;
}
@end
@implementation TTInputRequestInputEntities
- (NSArray *)items {
    return self.requestFields[@"items"];
}
- (void)setItems:(NSArray *)value {
    if (value) self.requestFields[@"items"] = value;
    else [self.requestFields removeObjectForKey:@"items"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"entities";
    } return self;
}
@end
@implementation TTInputRequestInputFeed
- (TTInputRequestFraming)framing {
    return self.requestFields[@"framing"];
}
- (void)setFraming:(TTInputRequestFraming)value {
    if (value) self.requestFields[@"framing"] = value;
    else [self.requestFields removeObjectForKey:@"framing"];
}
- (NSArray *)images {
    return self.requestFields[@"images"];
}
- (void)setImages:(NSArray *)value {
    if (value) self.requestFields[@"images"] = value;
    else [self.requestFields removeObjectForKey:@"images"];
}
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (TTInputRequestReader *)reading {
    return self.requestFields[@"reading"];
}
- (void)setReading:(TTInputRequestReader *)value {
    if (value) self.requestFields[@"reading"] = value;
    else [self.requestFields removeObjectForKey:@"reading"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"feed";
    } return self;
}
@end
@implementation TTInputRequestInputJson
- (NSArray *)images {
    return self.requestFields[@"images"];
}
- (void)setImages:(NSArray *)value {
    if (value) self.requestFields[@"images"] = value;
    else [self.requestFields removeObjectForKey:@"images"];
}
- (id)value {
    return self.requestFields[@"value"];
}
- (void)setValue:(id)value {
    if (value) self.requestFields[@"value"] = value;
    else [self.requestFields removeObjectForKey:@"value"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"json";
    } return self;
}
@end
@implementation TTInputRequestInputRecords
- (NSArray *)items {
    return self.requestFields[@"items"];
}
- (void)setItems:(NSArray *)value {
    if (value) self.requestFields[@"items"] = value;
    else [self.requestFields removeObjectForKey:@"items"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"records";
    } return self;
}
@end
@implementation TTInputRequestInputSource
- (TTInputRequestSource *)source {
    return self.requestFields[@"source"];
}
- (void)setSource:(TTInputRequestSource *)value {
    if (value) self.requestFields[@"source"] = value;
    else [self.requestFields removeObjectForKey:@"source"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"source";
    } return self;
}
@end
@implementation TTInputRequestInputText
- (NSArray *)images {
    return self.requestFields[@"images"];
}
- (void)setImages:(NSArray *)value {
    if (value) self.requestFields[@"images"] = value;
    else [self.requestFields removeObjectForKey:@"images"];
}
- (NSString *)text {
    return self.requestFields[@"text"];
}
- (void)setText:(NSString *)value {
    if (value) self.requestFields[@"text"] = value;
    else [self.requestFields removeObjectForKey:@"text"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"text";
    } return self;
}
@end
@implementation TTInputRequestInputUnits
- (NSArray *)items {
    return self.requestFields[@"items"];
}
- (void)setItems:(NSArray *)value {
    if (value) self.requestFields[@"items"] = value;
    else [self.requestFields removeObjectForKey:@"items"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"units";
    } return self;
}
@end
@implementation TTInputRequestItem
- (id)context {
    return self.requestFields[@"context"];
}
- (void)setContext:(id)value {
    if (value) self.requestFields[@"context"] = value;
    else [self.requestFields removeObjectForKey:@"context"];
}
- (NSArray *)examples {
    return self.requestFields[@"examples"];
}
- (void)setExamples:(NSArray *)value {
    if (value) self.requestFields[@"examples"] = value;
    else [self.requestFields removeObjectForKey:@"examples"];
}
- (NSArray *)images {
    return self.requestFields[@"images"];
}
- (void)setImages:(NSArray *)value {
    if (value) self.requestFields[@"images"] = value;
    else [self.requestFields removeObjectForKey:@"images"];
}
- (NSArray *)options {
    return self.requestFields[@"options"];
}
- (void)setOptions:(NSArray *)value {
    if (value) self.requestFields[@"options"] = value;
    else [self.requestFields removeObjectForKey:@"options"];
}
- (id)original {
    return self.requestFields[@"original"];
}
- (void)setOriginal:(id)value {
    if (value) self.requestFields[@"original"] = value;
    else [self.requestFields removeObjectForKey:@"original"];
}
- (NSArray *)seedSpans {
    return self.requestFields[@"seed_spans"];
}
- (void)setSeedSpans:(NSArray *)value {
    if (value) self.requestFields[@"seed_spans"] = value;
    else [self.requestFields removeObjectForKey:@"seed_spans"];
}
@end
@implementation TTInputRequestOptions
- (NSNumber *)attempts {
    return self.requestFields[@"attempts"];
}
- (void)setAttempts:(NSNumber *)value {
    if (value) self.requestFields[@"attempts"] = value;
    else [self.requestFields removeObjectForKey:@"attempts"];
}
- (id)batch {
    return self.requestFields[@"batch"];
}
- (void)setBatch:(id)value {
    if (value) self.requestFields[@"batch"] = value;
    else [self.requestFields removeObjectForKey:@"batch"];
}
- (NSString *)context {
    return self.requestFields[@"context"];
}
- (void)setContext:(NSString *)value {
    if (value) self.requestFields[@"context"] = value;
    else [self.requestFields removeObjectForKey:@"context"];
}
- (NSString *)contextField {
    return self.requestFields[@"context_field"];
}
- (void)setContextField:(NSString *)value {
    if (value) self.requestFields[@"context_field"] = value;
    else [self.requestFields removeObjectForKey:@"context_field"];
}
- (NSNumber *)deadlineMs {
    return self.requestFields[@"deadline_ms"];
}
- (void)setDeadlineMs:(NSNumber *)value {
    if (value) self.requestFields[@"deadline_ms"] = value;
    else [self.requestFields removeObjectForKey:@"deadline_ms"];
}
- (NSNumber *)details {
    return self.requestFields[@"details"];
}
- (void)setDetails:(NSNumber *)value {
    if (value) self.requestFields[@"details"] = value;
    else [self.requestFields removeObjectForKey:@"details"];
}
- (NSArray *)examples {
    return self.requestFields[@"examples"];
}
- (void)setExamples:(NSArray *)value {
    if (value) self.requestFields[@"examples"] = value;
    else [self.requestFields removeObjectForKey:@"examples"];
}
- (NSString *)examplesField {
    return self.requestFields[@"examples_field"];
}
- (void)setExamplesField:(NSString *)value {
    if (value) self.requestFields[@"examples_field"] = value;
    else [self.requestFields removeObjectForKey:@"examples_field"];
}
- (NSArray *)field {
    return self.requestFields[@"field"];
}
- (void)setField:(NSArray *)value {
    if (value) self.requestFields[@"field"] = value;
    else [self.requestFields removeObjectForKey:@"field"];
}
- (NSNumber *)filesOnly {
    return self.requestFields[@"files_only"];
}
- (void)setFilesOnly:(NSNumber *)value {
    if (value) self.requestFields[@"files_only"] = value;
    else [self.requestFields removeObjectForKey:@"files_only"];
}
- (NSNumber *)maxRequestsTotal {
    return self.requestFields[@"max_requests_total"];
}
- (void)setMaxRequestsTotal:(NSNumber *)value {
    if (value) self.requestFields[@"max_requests_total"] = value;
    else [self.requestFields removeObjectForKey:@"max_requests_total"];
}
- (TTInputRecognitionMode)mode {
    return self.requestFields[@"mode"];
}
- (void)setMode:(TTInputRecognitionMode)value {
    if (value) self.requestFields[@"mode"] = value;
    else [self.requestFields removeObjectForKey:@"mode"];
}
- (NSString *)model {
    return self.requestFields[@"model"];
}
- (void)setModel:(NSString *)value {
    if (value) self.requestFields[@"model"] = value;
    else [self.requestFields removeObjectForKey:@"model"];
}
- (NSNumber *)none {
    return self.requestFields[@"none"];
}
- (void)setNone:(NSNumber *)value {
    if (value) self.requestFields[@"none"] = value;
    else [self.requestFields removeObjectForKey:@"none"];
}
- (NSString *)optionsField {
    return self.requestFields[@"options_field"];
}
- (void)setOptionsField:(NSString *)value {
    if (value) self.requestFields[@"options_field"] = value;
    else [self.requestFields removeObjectForKey:@"options_field"];
}
- (id)relationThreshold {
    return self.requestFields[@"relation_threshold"];
}
- (void)setRelationThreshold:(id)value {
    if (value) self.requestFields[@"relation_threshold"] = value;
    else [self.requestFields removeObjectForKey:@"relation_threshold"];
}
- (NSArray *)seedSpans {
    return self.requestFields[@"seed_spans"];
}
- (void)setSeedSpans:(NSArray *)value {
    if (value) self.requestFields[@"seed_spans"] = value;
    else [self.requestFields removeObjectForKey:@"seed_spans"];
}
- (NSString *)seedSpansField {
    return self.requestFields[@"seed_spans_field"];
}
- (void)setSeedSpansField:(NSString *)value {
    if (value) self.requestFields[@"seed_spans_field"] = value;
    else [self.requestFields removeObjectForKey:@"seed_spans_field"];
}
- (NSNumber *)snippetPieces {
    return self.requestFields[@"snippet_pieces"];
}
- (void)setSnippetPieces:(NSNumber *)value {
    if (value) self.requestFields[@"snippet_pieces"] = value;
    else [self.requestFields removeObjectForKey:@"snippet_pieces"];
}
- (TTInputRecognitionStageContext *)stageContext {
    return self.requestFields[@"stage_context"];
}
- (void)setStageContext:(TTInputRecognitionStageContext *)value {
    if (value) self.requestFields[@"stage_context"] = value;
    else [self.requestFields removeObjectForKey:@"stage_context"];
}
- (id)threshold {
    return self.requestFields[@"threshold"];
}
- (void)setThreshold:(id)value {
    if (value) self.requestFields[@"threshold"] = value;
    else [self.requestFields removeObjectForKey:@"threshold"];
}
- (NSNumber *)top {
    return self.requestFields[@"top"];
}
- (void)setTop:(NSNumber *)value {
    if (value) self.requestFields[@"top"] = value;
    else [self.requestFields removeObjectForKey:@"top"];
}
@end
@implementation TTInputRequestOriginalJson
- (id)value {
    return self.requestFields[@"value"];
}
- (void)setValue:(id)value {
    if (value) self.requestFields[@"value"] = value;
    else [self.requestFields removeObjectForKey:@"value"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"json";
    } return self;
}
@end
@implementation TTInputRequestOriginalText
- (NSString *)text {
    return self.requestFields[@"text"];
}
- (void)setText:(NSString *)value {
    if (value) self.requestFields[@"text"] = value;
    else [self.requestFields removeObjectForKey:@"text"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"text";
    } return self;
}
@end
@implementation TTInputRequestQuestionDefinition
- (id)value {
    return self.requestFields[@"value"];
}
- (void)setValue:(id)value {
    if (value) self.requestFields[@"value"] = value;
    else [self.requestFields removeObjectForKey:@"value"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"definition";
    } return self;
}
@end
@implementation TTInputRequestQuestionFile
- (NSString *)path {
    return self.requestFields[@"path"];
}
- (void)setPath:(NSString *)value {
    if (value) self.requestFields[@"path"] = value;
    else [self.requestFields removeObjectForKey:@"path"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"file";
    } return self;
}
@end
@implementation TTInputRequestQuestionName
- (NSString *)name {
    return self.requestFields[@"name"];
}
- (void)setName:(NSString *)value {
    if (value) self.requestFields[@"name"] = value;
    else [self.requestFields removeObjectForKey:@"name"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"name";
    } return self;
}
@end
@implementation TTInputRequestQuestionReference
- (NSString *)reference {
    return self.requestFields[@"reference"];
}
- (void)setReference:(NSString *)value {
    if (value) self.requestFields[@"reference"] = value;
    else [self.requestFields removeObjectForKey:@"reference"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"reference";
    } return self;
}
@end
@implementation TTInputRequestQuestionText
- (NSString *)text {
    return self.requestFields[@"text"];
}
- (void)setText:(NSString *)value {
    if (value) self.requestFields[@"text"] = value;
    else [self.requestFields removeObjectForKey:@"text"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"text";
    } return self;
}
@end
@implementation TTInputRequestReader
- (TTInputSourceUnit)unit {
    return self.requestFields[@"unit"];
}
- (void)setUnit:(TTInputSourceUnit)value {
    if (value) self.requestFields[@"unit"] = value;
    else [self.requestFields removeObjectForKey:@"unit"];
}
- (NSNumber *)window {
    return self.requestFields[@"window"];
}
- (void)setWindow:(NSNumber *)value {
    if (value) self.requestFields[@"window"] = value;
    else [self.requestFields removeObjectForKey:@"window"];
}
@end
@implementation TTInputRequestReaderFailureInvalidInput
- (TTInputSessionSourceLocation *)location {
    return self.requestFields[@"location"];
}
- (void)setLocation:(TTInputSessionSourceLocation *)value {
    if (value) self.requestFields[@"location"] = value;
    else [self.requestFields removeObjectForKey:@"location"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"invalid_input";
    } return self;
}
@end
@implementation TTInputRequestReaderFailureIo
- (TTInputSessionSourceLocation *)location {
    return self.requestFields[@"location"];
}
- (void)setLocation:(TTInputSessionSourceLocation *)value {
    if (value) self.requestFields[@"location"] = value;
    else [self.requestFields removeObjectForKey:@"location"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"io";
    } return self;
}
@end
@implementation TTInputRequestReaderFailureProducer
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"producer";
    } return self;
}
@end
@implementation TTInputRequestReaderFailureUtf8
- (TTInputSessionSourceLocation *)location {
    return self.requestFields[@"location"];
}
- (void)setLocation:(TTInputSessionSourceLocation *)value {
    if (value) self.requestFields[@"location"] = value;
    else [self.requestFields removeObjectForKey:@"location"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"utf8";
    } return self;
}
@end
@implementation TTInputRequestSessionDescriptor
- (TTInputRequestItem *)item {
    return self.requestFields[@"item"];
}
- (void)setItem:(TTInputRequestItem *)value {
    if (value) self.requestFields[@"item"] = value;
    else [self.requestFields removeObjectForKey:@"item"];
}
- (TTInputSessionSourceLocation *)location {
    return self.requestFields[@"location"];
}
- (void)setLocation:(TTInputSessionSourceLocation *)value {
    if (value) self.requestFields[@"location"] = value;
    else [self.requestFields removeObjectForKey:@"location"];
}
@end
@implementation TTInputRequestSource
- (TTInputRequestFraming)framing {
    return self.requestFields[@"framing"];
}
- (void)setFraming:(TTInputRequestFraming)value {
    if (value) self.requestFields[@"framing"] = value;
    else [self.requestFields removeObjectForKey:@"framing"];
}
- (TTInputReaderMedia)media {
    return self.requestFields[@"media"];
}
- (void)setMedia:(TTInputReaderMedia)value {
    if (value) self.requestFields[@"media"] = value;
    else [self.requestFields removeObjectForKey:@"media"];
}
- (NSArray *)paths {
    return self.requestFields[@"paths"];
}
- (void)setPaths:(NSArray *)value {
    if (value) self.requestFields[@"paths"] = value;
    else [self.requestFields removeObjectForKey:@"paths"];
}
- (TTInputRequestReader *)reading {
    return self.requestFields[@"reading"];
}
- (void)setReading:(TTInputRequestReader *)value {
    if (value) self.requestFields[@"reading"] = value;
    else [self.requestFields removeObjectForKey:@"reading"];
}
@end
@implementation TTInputSessionSourceLocation
- (NSString *)file {
    return self.requestFields[@"file"];
}
- (void)setFile:(NSString *)value {
    if (value) self.requestFields[@"file"] = value;
    else [self.requestFields removeObjectForKey:@"file"];
}
- (NSNumber *)firstLine {
    return self.requestFields[@"first_line"];
}
- (void)setFirstLine:(NSNumber *)value {
    if (value) self.requestFields[@"first_line"] = value;
    else [self.requestFields removeObjectForKey:@"first_line"];
}
- (NSNumber *)lastLine {
    return self.requestFields[@"last_line"];
}
- (void)setLastLine:(NSNumber *)value {
    if (value) self.requestFields[@"last_line"] = value;
    else [self.requestFields removeObjectForKey:@"last_line"];
}
@end
NSErrorDomain const TTAnswerErrorDomain = @"io.github.botassembly.thinkthen.answer";
extern NSErrorDomain const TTErrorDomain;
extern NSString * const TTFailureDetailsKey;
TTPresence *TTAnswerPresence(id payload, id (^convert)(id)) {
    TTResultNode *node = [[TTResultNode alloc] initWithFields:@{
        @"value":payload
    }];
    return [node presence:@"value" required:YES convert:convert];
}
id TTAnswerPayload(NSArray<TTSessionPacket *> *packets, TTSessionPacketTerminal *terminal, NSString *function,
NSString *judgment, id call, NSError **error) {
    if (terminal.failure.state == TTPresenceValue) {
        TTCallError *failure = terminal.failure.value;
        if (error) *error = [NSError errorWithDomain:TTErrorDomain code:TTNativeErrorCode(failure.error.value.kind.value) userInfo:@{
            NSLocalizedDescriptionKey:failure.error.value.message.value, TTFailureDetailsKey:failure, @"ThinkThenCall":call
        }];
        return nil;
    }
    NSMutableArray *values = [NSMutableArray array];
    if (terminal) for (TTSessionPacket *packet in packets) {
        NSDictionary *fields = packet.rawFields;
        if (![fields[@"function"] isEqual:function]) continue;
        id value;
        if (!judgment.length) {
            if (![fields[@"kind"] isEqual:@"aggregate"]) continue;
            value = fields[@"value"];
        } else {
            if (!([fields[@"kind"] isEqual:@"observation"] && [fields[@"value"][@"kind"] isEqual:@"row"] && [fields[@"value"][@"value"][@"kind"] isEqual:@"judgment"])) continue;
            NSDictionary *carrier = fields[@"value"][@"value"][@"value"];
            if (![carrier[@"kind"] isEqual:judgment]) continue;
            value = carrier[@"value"];
        } if (value) [values addObject:value];
    }
    if (values.count != 1) {
        if (error) *error = [NSError errorWithDomain:TTAnswerErrorDomain code:1 userInfo:@{
            NSLocalizedDescriptionKey:@"Expected exactly one successful answer", @"ThinkThenFunction":function,
            @"ThinkThenAnswerCount":@(values.count), @"ThinkThenCall":call
        }];
        return nil;
    }
    return values[0];
}
