// Generated from the shared Rust result graph; do not edit.
#import "TTResults.g.h"
#import "TTResultPrivate.h"
#import "ThinkThenFoundation.h"
#define TT_GENERATED_NAMED_IMPLEMENTATIONS - (nullable TTTask *)decide:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"decide" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable TTPresence<NSNumber *> *)decideAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"decide" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed decideAnswer:error] : nil; } - (nullable TTTask *)choose:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"choose" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable TTPresence<NSString *> *)chooseAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"choose" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed chooseAnswer:error] : nil; } - (nullable TTTask *)tag:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"tag" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable TTPresence<NSArray<NSString *> *> *)tagAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"tag" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed tagAnswer:error] : nil; } - (nullable TTTask *)score:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"score" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable TTPresence<NSNumber *> *)scoreAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"score" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed scoreAnswer:error] : nil; } - (nullable TTTask *)filter:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"filter" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable NSArray *)filterAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"filter" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed filterAnswer:error] : nil; } - (nullable TTTask *)rank:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"rank" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable NSArray<NSNumber *> *)rankAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"rank" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed rankAnswer:error] : nil; } - (nullable TTTask *)find:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"find" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable TTPresence<TTFind *> *)findAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"find" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed findAnswer:error] : nil; } - (nullable TTTask *)annotate:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"annotate" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable TTPresence<NSArray<TTAnnotation *> *> *)annotateAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"annotate" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed annotateAnswer:error] : nil; } - (nullable TTTask *)recognize:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"recognize" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable TTPresence<NSArray<TTRecognition *> *> *)recognizeAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"recognize" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed recognizeAnswer:error] : nil; } - (nullable TTTask *)relate:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed completion:(TTCompletion)completion error:(NSError * _Nullable * _Nullable)error { return [self start:@"relate" question:question input:input options:options feed:feed completion:completion error:error]; } - (nullable TTPresence<TTRelation *> *)relateAnswer:(TTQuestion *)question input:(TTInput *)input options:(nullable TTControls *)options feed:(nullable TTFeed)feed call:(TTCall * _Nullable * _Nullable)call error:(NSError * _Nullable * _Nullable)error { TTCall *completed = [self complete:@"relate" question:question input:input options:options feed:feed error:error]; if (call) *call = completed; return completed ? [completed relateAnswer:error] : nil; }
#define TT_GENERATED_ANSWER_IMPLEMENTATIONS - (nullable TTPresence<NSNumber *> *)decideAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"decide", @"decision", self, error); if (!payload) return nil; return TTAnswerPresence(payload, ^id(id value) { return TTBoolean(value); }); } - (nullable TTPresence<NSString *> *)chooseAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"choose", @"choice", self, error); if (!payload) return nil; return TTAnswerPresence(payload, ^id(id value) { return TTString(value); }); } - (nullable TTPresence<NSArray<NSString *> *> *)tagAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"tag", @"tags", self, error); if (!payload) return nil; return TTAnswerPresence(payload, ^id(id value) { return TTArray(value, ^id(id item) { return TTString(item); }); }); } - (nullable TTPresence<NSNumber *> *)scoreAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"score", @"score", self, error); if (!payload) return nil; return TTAnswerPresence(payload, ^id(id value) { return TTNumber(value); }); } - (nullable NSArray *)filterAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"filter", @"", self, error); if (!payload) return nil; return TTArray(payload, ^id(id item) { id original = TTObject(item)[@"input"]; if (!original) TTInvalidResult(); return original; }); } - (nullable NSArray<NSNumber *> *)rankAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"rank", @"", self, error); if (!payload) return nil; return TTArray(payload, ^id(id item) { return TTInteger(TTObject(item)[@"index"]); }); } - (nullable TTPresence<TTFind *> *)findAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"find", @"", self, error); if (!payload) return nil; return TTAnswerPresence(payload, ^id(id value) { return [TTFind read:value]; }); } - (nullable TTPresence<NSArray<TTAnnotation *> *> *)annotateAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"annotate", @"", self, error); if (!payload) return nil; return TTAnswerPresence(payload, ^id(id value) { return TTArray(value, ^id(id item) { return [TTAnnotation read:item]; }); }); } - (nullable TTPresence<NSArray<TTRecognition *> *> *)recognizeAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"recognize", @"", self, error); if (!payload) return nil; return TTAnswerPresence(payload, ^id(id value) { return TTArray(value, ^id(id item) { return [TTRecognition read:item]; }); }); } - (nullable TTPresence<TTRelation *> *)relateAnswer:(NSError * _Nullable * _Nullable)error { id payload = TTAnswerPayload(self.packets, self.terminal, @"relate", @"", self, error); if (!payload) return nil; return TTAnswerPresence(payload, ^id(id value) { return [TTRelation read:value]; }); }
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
@implementation TTTagRecord
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
@implementation TTDecisionRecord
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
@implementation TTRankedRecord
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
@implementation TTChoiceRecord
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
@implementation TTFilteredRecord
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
@implementation TTScoreRecord
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
@implementation TTDeclarationBuilder
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"type"] isEqual:@"string"]) return (id)[TTDeclarationStringRootBuilder read:value];
    if ([object[@"type"] isEqual:@"object"]) return (id)[TTDeclarationObjectRootBuilder read:value];
    TTInvalidResult();
    return nil;
}
@end
@implementation TTPropertyTypeBuilder
+ (instancetype)read:(id)value {
    NSDictionary *object = TTObject(value);
    if ([object[@"type"] isEqual:@"string"]) return (id)[TTPropertyTypeStringBuilder read:value];
    if ([object[@"type"] isEqual:@"number"]) return (id)[TTPropertyTypeNumberBuilder read:value];
    if ([object[@"type"] isEqual:@"boolean"]) return (id)[TTPropertyTypeBooleanBuilder read:value];
    if ([object[@"type"] isEqual:@"array"]) return (id)[TTPropertyTypeArrayBuilder read:value];
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
- (TTPresence<NSDictionary<NSString *, TTPropertyTypeBuilder *> *> *)properties {
    return [self presence:@"properties" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return [TTPropertyTypeBuilder read:item];
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
- (TTPresence<TTDeclarationBuilder *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
    }];
}
- (TTPresence<TTDeclarationBuilder *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
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
- (TTPresence<TTDeclarationBuilder *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
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
- (TTPresence<TTDeclarationBuilder *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
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
- (TTPresence<TTDeclarationBuilder *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
    }];
}
- (TTPresence<TTRelateFields *> *)fields {
    return [self presence:@"fields" required:YES convert:^id(id value) {
        return [TTRelateFields read:value];
    }];
}
- (TTPresence<TTDeclarationBuilder *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
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
@implementation TTImmediateFacts
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
    return [self presence:@"call_id" required:NO convert:^id(id value) {
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
@implementation TTDeclarationObjectRootBuilder
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSDictionary<NSString *, TTPropertyTypeBuilder *> *> *)properties {
    return [self presence:@"properties" required:YES convert:^id(id value) {
        return TTMap(value, ^id(id item) {
            return [TTPropertyTypeBuilder read:item];
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
@implementation TTDeclarationStringRootBuilder
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTPropertyTypeArrayBuilder
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
@implementation TTPropertyTypeBooleanBuilder
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTPropertyTypeNumberBuilder
+ (instancetype)read:(id)value {
    return [[self alloc] initWithFields:TTObject(value)];
}
- (TTPresence<NSString *> *)type {
    return [self presence:@"type" required:YES convert:^id(id value) {
        return TTString(value);
    }];
}
@end
@implementation TTPropertyTypeStringBuilder
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
- (TTPresence<TTDeclarationBuilder *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
    }];
}
- (TTPresence<TTDeclarationBuilder *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
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
- (TTPresence<TTDeclarationBuilder *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
    }];
}
- (TTPresence<TTDeclarationBuilder *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
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
- (TTPresence<TTDeclarationBuilder *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
    }];
}
- (TTPresence<TTDeclarationBuilder *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
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
- (TTPresence<TTDeclarationBuilder *> *)contextSchema {
    return [self presence:@"context_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
    }];
}
- (TTPresence<TTDeclarationBuilder *> *)itemSchema {
    return [self presence:@"item_schema" required:NO convert:^id(id value) {
        return [TTDeclarationBuilder read:value];
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
- (TTPresence<NSArray<TTChoiceRecord *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTChoiceRecord read:item];
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
- (TTPresence<TTChoiceRecord *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTChoiceRecord read:value];
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
- (TTPresence<NSArray<TTDecisionRecord *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTDecisionRecord read:item];
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
- (TTPresence<TTDecisionRecord *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTDecisionRecord read:value];
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
- (TTPresence<NSArray<TTFilteredRecord *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTFilteredRecord read:item];
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
- (TTPresence<TTFilteredRecord *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTFilteredRecord read:value];
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
- (TTPresence<NSArray<TTRankedRecord *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTRankedRecord read:item];
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
- (TTPresence<NSArray<TTScoreRecord *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTScoreRecord read:item];
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
- (TTPresence<TTScoreRecord *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTScoreRecord read:value];
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
- (TTPresence<NSArray<TTTagRecord *> *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return TTArray(value, ^id(id item) {
            return [TTTagRecord read:item];
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
- (TTPresence<TTTagRecord *> *)value {
    return [self presence:@"value" required:YES convert:^id(id value) {
        return [TTTagRecord read:value];
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
@implementation TTQuestion
@end
@implementation TTInput
@end
@interface TTRequestObject ()
@property(nonatomic, strong) NSMutableDictionary<NSString *, id> *requestFields;
@end
@implementation TTRequestObject
- (instancetype)init {
    if ((self = [super init])) _requestFields = [NSMutableDictionary dictionary];
    return self;
}
- (instancetype)initWithFields:(NSDictionary *)fields {
    if ((self = [self init])) [_requestFields addEntriesFromDictionary:fields];
    return self;
}
@end
TTImageMedia const TTImageMediaImageJpeg = @"image/jpeg";
TTImageMedia const TTImageMediaImagePng = @"image/png";
TTReaderMedia const TTReaderMediaText = @"text";
TTReaderMedia const TTReaderMediaImage = @"image";
TTRecognitionMode const TTRecognitionModeWhole = @"whole";
TTRecognitionMode const TTRecognitionModeBoundaryOnly = @"boundary_only";
TTFraming const TTFramingDocument = @"document";
TTFraming const TTFramingLines = @"lines";
TTFraming const TTFramingJsonl = @"jsonl";
TTFraming const TTFramingCsv = @"csv";
TTFraming const TTFramingTsv = @"tsv";
TTFunction const TTFunctionDecide = @"decide";
TTFunction const TTFunctionChoose = @"choose";
TTFunction const TTFunctionTag = @"tag";
TTFunction const TTFunctionScore = @"score";
TTFunction const TTFunctionFilter = @"filter";
TTFunction const TTFunctionRank = @"rank";
TTFunction const TTFunctionFind = @"find";
TTFunction const TTFunctionAnnotate = @"annotate";
TTFunction const TTFunctionRecognize = @"recognize";
TTFunction const TTFunctionRelate = @"relate";
TTSourceUnit const TTSourceUnitLine = @"line";
TTSourceUnit const TTSourceUnitWindow = @"window";
TTSourceUnit const TTSourceUnitFile = @"file";
@implementation TTAnnotateDefinitionBuilder
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
@implementation TTAnnotateQuestionChooseBuilder
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
@implementation TTAnnotateQuestionDecideBuilder
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
@implementation TTAnnotateQuestionScoreBuilder
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
@implementation TTAnnotateQuestionTagBuilder
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
@implementation TTAuthoredChooseBuilder
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
@implementation TTAuthoredDecideBuilder
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
@implementation TTAuthoredFindBuilder
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
@implementation TTAuthoredInputDeclarationObjectBuilder
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
@implementation TTAuthoredInputDeclarationStringBuilder
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"string";
    } return self;
}
@end
@implementation TTAuthoredInputPropertyArrayBuilder
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
@implementation TTAuthoredInputPropertyBooleanBuilder
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"boolean";
    } return self;
}
@end
@implementation TTAuthoredInputPropertyNumberBuilder
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"number";
    } return self;
}
@end
@implementation TTAuthoredInputPropertyStringBuilder
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"type"] = @"string";
    } return self;
}
@end
@implementation TTAuthoredRelateBuilder
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
- (TTRelateQuestionBuilder *)relate {
    return self.requestFields[@"relate"];
}
- (void)setRelate:(TTRelateQuestionBuilder *)value {
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
@implementation TTAuthoredRelationBuilder
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
@implementation TTAuthoredScoreBuilder
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
@implementation TTAuthoredTagBuilder
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
@implementation TTSettings
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
@implementation TTOptionSchemaBuilder
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
@implementation TTRecognitionExampleEntityBuilder
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
@implementation TTRecognitionExampleTextBuilder
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
@implementation TTRecognitionSeedSpanBuilder
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
@implementation TTRecognitionStageContextBuilder
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
@implementation TTRecognizeDefinitionBuilder
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
- (TTRecognizeQuestionBuilder *)recognize {
    return self.requestFields[@"recognize"];
}
- (void)setRecognize:(TTRecognizeQuestionBuilder *)value {
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
@implementation TTRecognizeQuestionBuilder
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
- (TTRecognitionMode)mode {
    return self.requestFields[@"mode"];
}
- (void)setMode:(TTRecognitionMode)value {
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
- (TTRecognitionStageContextBuilder *)stageContext {
    return self.requestFields[@"stage_context"];
}
- (void)setStageContext:(TTRecognitionStageContextBuilder *)value {
    if (value) self.requestFields[@"stage_context"] = value;
    else [self.requestFields removeObjectForKey:@"stage_context"];
}
@end
@implementation TTRelateFieldsBuilder
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
@implementation TTRelateQuestionBuilder
- (TTRelateFieldsBuilder *)fields {
    return self.requestFields[@"fields"];
}
- (void)setFields:(TTRelateFieldsBuilder *)value {
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
@implementation TTRequest
- (TTCallArguments *)call {
    return self.requestFields[@"call"];
}
- (void)setCall:(TTCallArguments *)value {
    if (value) self.requestFields[@"call"] = value;
    else [self.requestFields removeObjectForKey:@"call"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"schema"] = @"thinkthen.request/1";
    } return self;
}
@end
@implementation TTCallArguments
- (TTFunction)function {
    return self.requestFields[@"function"];
}
- (void)setFunction:(TTFunction)value {
    if (value) self.requestFields[@"function"] = value;
    else [self.requestFields removeObjectForKey:@"function"];
}
- (TTInput *)input {
    return self.requestFields[@"input"];
}
- (void)setInput:(TTInput *)value {
    if (value) self.requestFields[@"input"] = value;
    else [self.requestFields removeObjectForKey:@"input"];
}
- (TTControls *)options {
    return self.requestFields[@"options"];
}
- (void)setOptions:(TTControls *)value {
    if (value) self.requestFields[@"options"] = value;
    else [self.requestFields removeObjectForKey:@"options"];
}
- (TTQuestion *)question {
    return self.requestFields[@"question"];
}
- (void)setQuestion:(TTQuestion *)value {
    if (value) self.requestFields[@"question"] = value;
    else [self.requestFields removeObjectForKey:@"question"];
}
@end
@implementation TTByteImage
- (NSString *)bytes {
    return self.requestFields[@"bytes"];
}
- (void)setBytes:(NSString *)value {
    if (value) self.requestFields[@"bytes"] = value;
    else [self.requestFields removeObjectForKey:@"bytes"];
}
- (TTImageMedia)media {
    return self.requestFields[@"media"];
}
- (void)setMedia:(TTImageMedia)value {
    if (value) self.requestFields[@"media"] = value;
    else [self.requestFields removeObjectForKey:@"media"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"bytes";
    } return self;
}
@end
@implementation TTFileImage
- (TTImageMedia)media {
    return self.requestFields[@"media"];
}
- (void)setMedia:(TTImageMedia)value {
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
@implementation TTEntitiesInput
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
@implementation TTFeedInput
- (TTFraming)framing {
    return self.requestFields[@"framing"];
}
- (void)setFraming:(TTFraming)value {
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
- (TTReading *)reading {
    return self.requestFields[@"reading"];
}
- (void)setReading:(TTReading *)value {
    if (value) self.requestFields[@"reading"] = value;
    else [self.requestFields removeObjectForKey:@"reading"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"feed";
    } return self;
}
@end
@implementation TTJSONInput
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
@implementation TTRecordsInput
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
@implementation TTSourceInput
- (TTFiles *)source {
    return self.requestFields[@"source"];
}
- (void)setSource:(TTFiles *)value {
    if (value) self.requestFields[@"source"] = value;
    else [self.requestFields removeObjectForKey:@"source"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"source";
    } return self;
}
@end
@implementation TTTextInput
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
@implementation TTUnitsInput
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
@implementation TTItem
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
@implementation TTControls
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
- (TTRecognitionMode)mode {
    return self.requestFields[@"mode"];
}
- (void)setMode:(TTRecognitionMode)value {
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
- (TTRecognitionStageContextBuilder *)stageContext {
    return self.requestFields[@"stage_context"];
}
- (void)setStageContext:(TTRecognitionStageContextBuilder *)value {
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
@implementation TTOriginalJSON
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
@implementation TTOriginalText
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
@implementation TTQuestionDefinition
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
@implementation TTQuestionFile
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
@implementation TTQuestionName
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
@implementation TTQuestionReference
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
@implementation TTTextQuestion
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
@implementation TTReading
- (TTSourceUnit)unit {
    return self.requestFields[@"unit"];
}
- (void)setUnit:(TTSourceUnit)value {
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
@implementation TTInvalidInputFailure
- (TTSessionSourceLocationBuilder *)location {
    return self.requestFields[@"location"];
}
- (void)setLocation:(TTSessionSourceLocationBuilder *)value {
    if (value) self.requestFields[@"location"] = value;
    else [self.requestFields removeObjectForKey:@"location"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"invalid_input";
    } return self;
}
@end
@implementation TTReaderIOFailure
- (TTSessionSourceLocationBuilder *)location {
    return self.requestFields[@"location"];
}
- (void)setLocation:(TTSessionSourceLocationBuilder *)value {
    if (value) self.requestFields[@"location"] = value;
    else [self.requestFields removeObjectForKey:@"location"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"io";
    } return self;
}
@end
@implementation TTProducerFailure
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"producer";
    } return self;
}
@end
@implementation TTReaderUTF8Failure
- (TTSessionSourceLocationBuilder *)location {
    return self.requestFields[@"location"];
}
- (void)setLocation:(TTSessionSourceLocationBuilder *)value {
    if (value) self.requestFields[@"location"] = value;
    else [self.requestFields removeObjectForKey:@"location"];
}
- (instancetype)init {
    if ((self = [super init])) {
        self.requestFields[@"kind"] = @"utf8";
    } return self;
}
@end
@implementation TTDescriptor
- (TTItem *)item {
    return self.requestFields[@"item"];
}
- (void)setItem:(TTItem *)value {
    if (value) self.requestFields[@"item"] = value;
    else [self.requestFields removeObjectForKey:@"item"];
}
- (TTSessionSourceLocationBuilder *)location {
    return self.requestFields[@"location"];
}
- (void)setLocation:(TTSessionSourceLocationBuilder *)value {
    if (value) self.requestFields[@"location"] = value;
    else [self.requestFields removeObjectForKey:@"location"];
}
@end
@implementation TTFiles
- (TTFraming)framing {
    return self.requestFields[@"framing"];
}
- (void)setFraming:(TTFraming)value {
    if (value) self.requestFields[@"framing"] = value;
    else [self.requestFields removeObjectForKey:@"framing"];
}
- (TTReaderMedia)media {
    return self.requestFields[@"media"];
}
- (void)setMedia:(TTReaderMedia)value {
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
- (TTReading *)reading {
    return self.requestFields[@"reading"];
}
- (void)setReading:(TTReading *)value {
    if (value) self.requestFields[@"reading"] = value;
    else [self.requestFields removeObjectForKey:@"reading"];
}
@end
@implementation TTSessionSourceLocationBuilder
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
        if (error) *error = TTFailureForCall(terminal.failure.value, call);
        return nil;
    }
    NSMutableArray *values = [NSMutableArray array];
    if (terminal) for (TTSessionPacket *packet in packets) {
        NSDictionary *fields = packet.rawFields;
        if (![fields[@"function"] isEqual:function]) continue;
        id value;
        BOOL rowAnswer = [function isEqual:@"filter"] || [function isEqual:@"annotate"];
        if (rowAnswer) {
            if ([fields[@"kind"] isEqual:@"row"]) [values addObject:fields[@"value"]];
            continue;
        }
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
    if (([function isEqual:@"filter"] || [function isEqual:@"annotate"]) && [((TTCall *)call).function isEqual:function]) return [values copy];
    if (values.count != 1) {
        if (error) *error = [NSError errorWithDomain:TTAnswerErrorDomain code:1 userInfo:@{
            NSLocalizedDescriptionKey:@"Expected exactly one successful answer", @"ThinkThenFunction":function,
            @"ThinkThenAnswerCount":@(values.count), @"ThinkThenCall":call
        }];
        return nil;
    }
    return values[0];
}
@implementation TTCall (Answers)
TT_GENERATED_ANSWER_IMPLEMENTATIONS
@end
@implementation TTFoundationClient (NamedCalls)
TT_GENERATED_NAMED_IMPLEMENTATIONS
@end
#if __has_include(<thinkthen.h>)
// Generated from the shared Rust result graph; do not edit.
#import "TTResultPrivate.h"
#import <thinkthen.h>
#include <math.h>
id TTNative_thinkthen_complete_attempt_list_v1(const struct thinkthen_complete_attempt_list_v1 *value);
id TTNative_thinkthen_complete_attempt_list_presence_v1(const struct thinkthen_complete_attempt_list_presence_v1 *value);
id TTNative_thinkthen_complete_call_id_presence_v1(const struct thinkthen_complete_call_id_presence_v1 *value);
id TTNative_thinkthen_complete_utf8_presence_v1(const struct thinkthen_complete_utf8_presence_v1 *value);
id TTNative_thinkthen_complete_u32_presence_v1(const struct thinkthen_complete_u32_presence_v1 *value);
id TTNative_thinkthen_complete_u64_presence_v1(const struct thinkthen_complete_u64_presence_v1 *value);
id TTNative_thinkthen_complete_persistence_observation_presence_v1(const struct thinkthen_complete_persistence_observation_presence_v1 *value);
id TTNative_thinkthen_complete_immediate_facts_v1(const struct thinkthen_complete_immediate_facts_v1 *value);
id TTNative_thinkthen_complete_persistence_observation_v1(const struct thinkthen_complete_persistence_observation_v1 *value);
id TTNative_thinkthen_complete_usage_persistence_v1(const struct thinkthen_complete_usage_persistence_v1 *value);
id TTNative_thinkthen_complete_call_id_v1(const struct thinkthen_complete_call_id_v1 *value);
id TTNative_thinkthen_complete_attempt_v1(const struct thinkthen_complete_attempt_v1 *value);
id TTNative_thinkthen_complete_sdk_request_id_v1(const struct thinkthen_complete_sdk_request_id_v1 *value);
id TTNative_thinkthen_complete_attempt_outcome_v1(const struct thinkthen_complete_attempt_outcome_v1 *value);
id TTNative_thinkthen_complete_session_packet_v1(const struct thinkthen_complete_session_packet_v1 *value);
id TTNative_thinkthen_complete_facts_presence_v1(const struct thinkthen_complete_facts_presence_v1 *value);
id TTNative_thinkthen_complete_call_error_presence_v1(const struct thinkthen_complete_call_error_presence_v1 *value);
id TTNative_thinkthen_complete_session_packet_terminal_v1(const struct thinkthen_complete_session_packet_terminal_v1 *value);
id TTNative_thinkthen_complete_call_error_v1(const struct thinkthen_complete_call_error_v1 *value);
id TTNative_thinkthen_complete_facts_v1(const struct thinkthen_complete_facts_v1 *value);
id TTNative_thinkthen_complete_input_denial_presence_v1(const struct thinkthen_complete_input_denial_presence_v1 *value);
id TTNative_thinkthen_complete_send_denial_presence_v1(const struct thinkthen_complete_send_denial_presence_v1 *value);
id TTNative_thinkthen_complete_error_v1(const struct thinkthen_complete_error_v1 *value);
id TTNative_thinkthen_complete_stopped_v1(const struct thinkthen_complete_stopped_v1 *value);
id TTNative_thinkthen_complete_stop_cause_v1(const struct thinkthen_complete_stop_cause_v1 *value);
id TTNative_thinkthen_complete_send_denial_v1(const struct thinkthen_complete_send_denial_v1 *value);
id TTNative_thinkthen_complete_send_denial_before_retry_v1(const struct thinkthen_complete_send_denial_before_retry_v1 *value);
id TTNative_thinkthen_complete_send_denial_before_additional_send_v1(const struct thinkthen_complete_send_denial_before_additional_send_v1 *value);
id TTNative_thinkthen_complete_send_denial_before_first_send_v1(const struct thinkthen_complete_send_denial_before_first_send_v1 *value);
id TTNative_thinkthen_complete_failure_kind_v1(const struct thinkthen_complete_failure_kind_v1 *value);
id TTNative_thinkthen_complete_input_denial_v1(const struct thinkthen_complete_input_denial_v1 *value);
id TTNative_thinkthen_complete_input_denial_retry_v1(const struct thinkthen_complete_input_denial_retry_v1 *value);
id TTNative_thinkthen_complete_input_denial_additional_request_v1(const struct thinkthen_complete_input_denial_additional_request_v1 *value);
id TTNative_thinkthen_complete_input_denial_initial_request_v1(const struct thinkthen_complete_input_denial_initial_request_v1 *value);
id TTNative_thinkthen_complete_session_packet_observation_v1(const struct thinkthen_complete_session_packet_observation_v1 *value);
id TTNative_thinkthen_complete_session_observation_v1(const struct thinkthen_complete_session_observation_v1 *value);
id TTNative_thinkthen_complete_session_observation_row_v1(const struct thinkthen_complete_session_observation_row_v1 *value);
id TTNative_thinkthen_complete_session_observed_row_v1(const struct thinkthen_complete_session_observed_row_v1 *value);
id TTNative_thinkthen_complete_session_relation_edge_list_v1(const struct thinkthen_complete_session_relation_edge_list_v1 *value);
id TTNative_thinkthen_complete_session_observed_row_relations_v1(const struct thinkthen_complete_session_observed_row_relations_v1 *value);
id TTNative_thinkthen_complete_session_relation_edge_v1(const struct thinkthen_complete_session_relation_edge_v1 *value);
id TTNative_thinkthen_complete_entity_document_v1(const struct thinkthen_complete_entity_document_v1 *value);
id TTNative_thinkthen_complete_session_observed_row_find_v1(const struct thinkthen_complete_session_observed_row_find_v1 *value);
id TTNative_thinkthen_complete_session_observed_row_recognized_v1(const struct thinkthen_complete_session_observed_row_recognized_v1 *value);
id TTNative_thinkthen_complete_entity_list_v1(const struct thinkthen_complete_entity_list_v1 *value);
id TTNative_thinkthen_complete_boundary_proposal_list_v1(const struct thinkthen_complete_boundary_proposal_list_v1 *value);
id TTNative_thinkthen_complete_boundary_proposal_list_presence_v1(const struct thinkthen_complete_boundary_proposal_list_presence_v1 *value);
id TTNative_thinkthen_complete_recognition_edge_list_v1(const struct thinkthen_complete_recognition_edge_list_v1 *value);
id TTNative_thinkthen_complete_recognition_edge_list_presence_v1(const struct thinkthen_complete_recognition_edge_list_presence_v1 *value);
id TTNative_thinkthen_complete_session_recognition_v1(const struct thinkthen_complete_session_recognition_v1 *value);
id TTNative_thinkthen_complete_recognition_edge_v1(const struct thinkthen_complete_recognition_edge_v1 *value);
id TTNative_thinkthen_complete_entity_v1(const struct thinkthen_complete_entity_v1 *value);
id TTNative_thinkthen_complete_boundary_proposal_v1(const struct thinkthen_complete_boundary_proposal_v1 *value);
id TTNative_thinkthen_complete_recognition_mode_v1(const struct thinkthen_complete_recognition_mode_v1 *value);
id TTNative_thinkthen_complete_session_annotation_list_v1(const struct thinkthen_complete_session_annotation_list_v1 *value);
id TTNative_thinkthen_complete_session_observed_row_annotated_v1(const struct thinkthen_complete_session_observed_row_annotated_v1 *value);
id TTNative_thinkthen_complete_session_annotation_v1(const struct thinkthen_complete_session_annotation_v1 *value);
id TTNative_thinkthen_complete_annotation_value_v1(const struct thinkthen_complete_annotation_value_v1 *value);
id TTNative_thinkthen_complete_annotation_value_failed_v1(const struct thinkthen_complete_annotation_value_failed_v1 *value);
id TTNative_thinkthen_complete_failure_field_kind_v1(const struct thinkthen_complete_failure_field_kind_v1 *value);
id TTNative_thinkthen_complete_failure_v1(const struct thinkthen_complete_failure_v1 *value);
id TTNative_thinkthen_complete_failure_cause_v1(const struct thinkthen_complete_failure_cause_v1 *value);
id TTNative_thinkthen_complete_string_list_v1(const struct thinkthen_complete_string_list_v1 *value);
id TTNative_thinkthen_complete_annotation_value_tags_v1(const struct thinkthen_complete_annotation_value_tags_v1 *value);
id TTNative_thinkthen_complete_annotation_value_score_v1(const struct thinkthen_complete_annotation_value_score_v1 *value);
id TTNative_thinkthen_complete_annotation_value_choice_v1(const struct thinkthen_complete_annotation_value_choice_v1 *value);
id TTNative_thinkthen_complete_annotation_value_decision_v1(const struct thinkthen_complete_annotation_value_decision_v1 *value);
id TTNative_thinkthen_complete_session_observed_row_judgment_v1(const struct thinkthen_complete_session_observed_row_judgment_v1 *value);
id TTNative_thinkthen_complete_session_judgment_v1(const struct thinkthen_complete_session_judgment_v1 *value);
id TTNative_thinkthen_complete_session_judgment_tags_v1(const struct thinkthen_complete_session_judgment_tags_v1 *value);
id TTNative_thinkthen_complete_session_judgment_score_v1(const struct thinkthen_complete_session_judgment_score_v1 *value);
id TTNative_thinkthen_complete_session_judgment_choice_v1(const struct thinkthen_complete_session_judgment_choice_v1 *value);
id TTNative_thinkthen_complete_session_judgment_decision_v1(const struct thinkthen_complete_session_judgment_decision_v1 *value);
id TTNative_thinkthen_complete_session_observation_question_v1(const struct thinkthen_complete_session_observation_question_v1 *value);
id TTNative_thinkthen_complete_answer_id_presence_v1(const struct thinkthen_complete_answer_id_presence_v1 *value);
id TTNative_thinkthen_complete_f64_presence_v1(const struct thinkthen_complete_f64_presence_v1 *value);
id TTNative_thinkthen_complete_failure_presence_v1(const struct thinkthen_complete_failure_presence_v1 *value);
id TTNative_thinkthen_complete_failure_id_presence_v1(const struct thinkthen_complete_failure_id_presence_v1 *value);
id TTNative_thinkthen_complete_json_presence_v1(const struct thinkthen_complete_json_presence_v1 *value);
id TTNative_thinkthen_complete_physical_source_presence_v1(const struct thinkthen_complete_physical_source_presence_v1 *value);
id TTNative_thinkthen_complete_session_input_source_list_v1(const struct thinkthen_complete_session_input_source_list_v1 *value);
id TTNative_thinkthen_complete_json_list_v1(const struct thinkthen_complete_json_list_v1 *value);
id TTNative_thinkthen_complete_observation_list_v1(const struct thinkthen_complete_observation_list_v1 *value);
id TTNative_thinkthen_complete_session_probabilities_presence_v1(const struct thinkthen_complete_session_probabilities_presence_v1 *value);
id TTNative_thinkthen_complete_question_source_list_v1(const struct thinkthen_complete_question_source_list_v1 *value);
id TTNative_thinkthen_complete_usage_presence_v1(const struct thinkthen_complete_usage_presence_v1 *value);
id TTNative_thinkthen_complete_threshold_presence_v1(const struct thinkthen_complete_threshold_presence_v1 *value);
id TTNative_thinkthen_complete_token_usage_presence_v1(const struct thinkthen_complete_token_usage_presence_v1 *value);
id TTNative_thinkthen_complete_value_presence_v1(const struct thinkthen_complete_value_presence_v1 *value);
id TTNative_thinkthen_complete_session_question_detail_v1(const struct thinkthen_complete_session_question_detail_v1 *value);
id TTNative_thinkthen_complete_value_v1(const struct thinkthen_complete_value_v1 *value);
id TTNative_thinkthen_complete_token_usage_v1(const struct thinkthen_complete_token_usage_v1 *value);
id TTNative_thinkthen_complete_threshold_v1(const struct thinkthen_complete_threshold_v1 *value);
id TTNative_thinkthen_complete_usage_v1(const struct thinkthen_complete_usage_v1 *value);
id TTNative_thinkthen_complete_question_source_v1(const struct thinkthen_complete_question_source_v1 *value);
id TTNative_thinkthen_complete_origin_v1(const struct thinkthen_complete_origin_v1 *value);
id TTNative_thinkthen_complete_question_document_v1(const struct thinkthen_complete_question_document_v1 *value);
id TTNative_thinkthen_complete_batch_presence_v1(const struct thinkthen_complete_batch_presence_v1 *value);
id TTNative_thinkthen_complete_input_declaration_presence_v1(const struct thinkthen_complete_input_declaration_presence_v1 *value);
id TTNative_thinkthen_complete_label_list_v1(const struct thinkthen_complete_label_list_v1 *value);
id TTNative_thinkthen_complete_label_list_presence_v1(const struct thinkthen_complete_label_list_presence_v1 *value);
id TTNative_thinkthen_complete_question_name_presence_v1(const struct thinkthen_complete_question_name_presence_v1 *value);
id TTNative_thinkthen_complete_string_list_presence_v1(const struct thinkthen_complete_string_list_presence_v1 *value);
id TTNative_thinkthen_complete_wording_version_presence_v1(const struct thinkthen_complete_wording_version_presence_v1 *value);
id TTNative_thinkthen_complete_question_document_score_v1(const struct thinkthen_complete_question_document_score_v1 *value);
id TTNative_thinkthen_complete_wording_version_v1(const struct thinkthen_complete_wording_version_v1 *value);
id TTNative_thinkthen_complete_question_name_v1(const struct thinkthen_complete_question_name_v1 *value);
id TTNative_thinkthen_complete_label_v1(const struct thinkthen_complete_label_v1 *value);
id TTNative_thinkthen_complete_input_declaration_v1(const struct thinkthen_complete_input_declaration_v1 *value);
id TTNative_thinkthen_complete_input_property_type_map_entry_v1(const struct thinkthen_complete_input_property_type_map_entry_v1 *value);
id TTNative_thinkthen_complete_input_property_type_map_v1(const struct thinkthen_complete_input_property_type_map_v1 *value);
id TTNative_thinkthen_complete_input_declaration_object_root_v1(const struct thinkthen_complete_input_declaration_object_root_v1 *value);
id TTNative_thinkthen_complete_object_type_v1(const struct thinkthen_complete_object_type_v1 *value);
id TTNative_thinkthen_complete_input_property_type_v1(const struct thinkthen_complete_input_property_type_v1 *value);
id TTNative_thinkthen_complete_input_property_type_array_v1(const struct thinkthen_complete_input_property_type_array_v1 *value);
id TTNative_thinkthen_complete_string_root_v1(const struct thinkthen_complete_string_root_v1 *value);
id TTNative_thinkthen_complete_string_type_v1(const struct thinkthen_complete_string_type_v1 *value);
id TTNative_thinkthen_complete_input_property_type_boolean_v1(const struct thinkthen_complete_input_property_type_boolean_v1 *value);
id TTNative_thinkthen_complete_input_property_type_number_v1(const struct thinkthen_complete_input_property_type_number_v1 *value);
id TTNative_thinkthen_complete_input_property_type_string_v1(const struct thinkthen_complete_input_property_type_string_v1 *value);
id TTNative_thinkthen_complete_input_declaration_string_root_v1(const struct thinkthen_complete_input_declaration_string_root_v1 *value);
id TTNative_thinkthen_complete_batch_string_v1(const struct thinkthen_complete_batch_string_v1 *value);
id TTNative_thinkthen_complete_batch_v1(const struct thinkthen_complete_batch_v1 *value);
id TTNative_thinkthen_complete_question_document_tag_v1(const struct thinkthen_complete_question_document_tag_v1 *value);
id TTNative_thinkthen_complete_question_document_choose_v1(const struct thinkthen_complete_question_document_choose_v1 *value);
id TTNative_thinkthen_complete_question_document_decide_v1(const struct thinkthen_complete_question_document_decide_v1 *value);
id TTNative_thinkthen_complete_session_probabilities_v1(const struct thinkthen_complete_session_probabilities_v1 *value);
id TTNative_thinkthen_complete_session_named_probability_list_v1(const struct thinkthen_complete_session_named_probability_list_v1 *value);
id TTNative_thinkthen_complete_session_probabilities_named_v1(const struct thinkthen_complete_session_probabilities_named_v1 *value);
id TTNative_thinkthen_complete_session_named_probability_v1(const struct thinkthen_complete_session_named_probability_v1 *value);
id TTNative_thinkthen_complete_session_probabilities_yes_no_v1(const struct thinkthen_complete_session_probabilities_yes_no_v1 *value);
id TTNative_thinkthen_complete_observation_v1(const struct thinkthen_complete_observation_v1 *value);
id TTNative_thinkthen_complete_observation_failure_id_v1(const struct thinkthen_complete_observation_failure_id_v1 *value);
id TTNative_thinkthen_complete_failure_id_v1(const struct thinkthen_complete_failure_id_v1 *value);
id TTNative_thinkthen_complete_observation_observation_id_v1(const struct thinkthen_complete_observation_observation_id_v1 *value);
id TTNative_thinkthen_complete_observation_id_v1(const struct thinkthen_complete_observation_id_v1 *value);
id TTNative_thinkthen_complete_session_input_source_v1(const struct thinkthen_complete_session_input_source_v1 *value);
id TTNative_thinkthen_complete_physical_source_v1(const struct thinkthen_complete_physical_source_v1 *value);
id TTNative_thinkthen_complete_answer_id_v1(const struct thinkthen_complete_answer_id_v1 *value);
id TTNative_thinkthen_complete_request_function_v1(const struct thinkthen_complete_request_function_v1 *value);
id TTNative_thinkthen_complete_session_packet_relate_aggregate_v1(const struct thinkthen_complete_session_packet_relate_aggregate_v1 *value);
id TTNative_thinkthen_complete_session_input_source_list_presence_v1(const struct thinkthen_complete_session_input_source_list_presence_v1 *value);
id TTNative_thinkthen_complete_position_presence_v1(const struct thinkthen_complete_position_presence_v1 *value);
id TTNative_thinkthen_complete_related_entity_edge_list_v1(const struct thinkthen_complete_related_entity_edge_list_v1 *value);
id TTNative_thinkthen_complete_relation_v1(const struct thinkthen_complete_relation_v1 *value);
id TTNative_thinkthen_complete_related_entity_edge_v1(const struct thinkthen_complete_related_entity_edge_v1 *value);
id TTNative_thinkthen_complete_relation_endpoint_v1(const struct thinkthen_complete_relation_endpoint_v1 *value);
id TTNative_thinkthen_complete_relation_endpoint_source_v1(const struct thinkthen_complete_relation_endpoint_source_v1 *value);
id TTNative_thinkthen_complete_relation_endpoint_related_entity_v1(const struct thinkthen_complete_relation_endpoint_related_entity_v1 *value);
id TTNative_thinkthen_complete_version_v1(const struct thinkthen_complete_version_v1 *value);
id TTNative_thinkthen_complete_relate_fields_presence_v1(const struct thinkthen_complete_relate_fields_presence_v1 *value);
id TTNative_thinkthen_complete_relation_rule_list_v1(const struct thinkthen_complete_relation_rule_list_v1 *value);
id TTNative_thinkthen_complete_relate_question_document_v1(const struct thinkthen_complete_relate_question_document_v1 *value);
id TTNative_thinkthen_complete_relation_rule_v1(const struct thinkthen_complete_relation_rule_v1 *value);
id TTNative_thinkthen_complete_relate_fields_v1(const struct thinkthen_complete_relate_fields_v1 *value);
id TTNative_thinkthen_complete_position_v1(const struct thinkthen_complete_position_v1 *value);
id TTNative_thinkthen_complete_batch_setting_presence_v1(const struct thinkthen_complete_batch_setting_presence_v1 *value);
id TTNative_thinkthen_complete_batch_warning_presence_v1(const struct thinkthen_complete_batch_warning_presence_v1 *value);
id TTNative_thinkthen_complete_origin_presence_v1(const struct thinkthen_complete_origin_presence_v1 *value);
id TTNative_thinkthen_complete_profile_warning_presence_v1(const struct thinkthen_complete_profile_warning_presence_v1 *value);
id TTNative_thinkthen_complete_meta_v1(const struct thinkthen_complete_meta_v1 *value);
id TTNative_thinkthen_complete_profile_warning_v1(const struct thinkthen_complete_profile_warning_v1 *value);
id TTNative_thinkthen_complete_batch_warning_v1(const struct thinkthen_complete_batch_warning_v1 *value);
id TTNative_thinkthen_complete_batch_setting_v1(const struct thinkthen_complete_batch_setting_v1 *value);
id TTNative_thinkthen_complete_relation_member_list_v1(const struct thinkthen_complete_relation_member_list_v1 *value);
id TTNative_thinkthen_complete_answers_v1(const struct thinkthen_complete_answers_v1 *value);
id TTNative_thinkthen_complete_relation_member_v1(const struct thinkthen_complete_relation_member_v1 *value);
id TTNative_thinkthen_complete_related_entity_presence_v1(const struct thinkthen_complete_related_entity_presence_v1 *value);
id TTNative_thinkthen_complete_relation_member_failure_id_v1(const struct thinkthen_complete_relation_member_failure_id_v1 *value);
id TTNative_thinkthen_complete_related_entity_v1(const struct thinkthen_complete_related_entity_v1 *value);
id TTNative_thinkthen_complete_relation_method_v1(const struct thinkthen_complete_relation_method_v1 *value);
id TTNative_thinkthen_complete_relation_direction_v1(const struct thinkthen_complete_relation_direction_v1 *value);
id TTNative_thinkthen_complete_relation_member_answer_id_v1(const struct thinkthen_complete_relation_member_answer_id_v1 *value);
id TTNative_thinkthen_complete_answer_v1(const struct thinkthen_complete_answer_v1 *value);
id TTNative_thinkthen_complete_number_map_entry_v1(const struct thinkthen_complete_number_map_entry_v1 *value);
id TTNative_thinkthen_complete_number_map_v1(const struct thinkthen_complete_number_map_v1 *value);
id TTNative_thinkthen_complete_answer_score_v1(const struct thinkthen_complete_answer_score_v1 *value);
id TTNative_thinkthen_complete_answer_tag_v1(const struct thinkthen_complete_answer_tag_v1 *value);
id TTNative_thinkthen_complete_answer_choice_v1(const struct thinkthen_complete_answer_choice_v1 *value);
id TTNative_thinkthen_complete_answer_yes_no_v1(const struct thinkthen_complete_answer_yes_no_v1 *value);
id TTNative_thinkthen_complete_recognition_list_v1(const struct thinkthen_complete_recognition_list_v1 *value);
id TTNative_thinkthen_complete_session_packet_recognize_aggregate_v1(const struct thinkthen_complete_session_packet_recognize_aggregate_v1 *value);
id TTNative_thinkthen_complete_recognition_v1(const struct thinkthen_complete_recognition_v1 *value);
id TTNative_thinkthen_complete_recognize_v1(const struct thinkthen_complete_recognize_v1 *value);
id TTNative_thinkthen_complete_recognize_mode_proposals_v1(const struct thinkthen_complete_recognize_mode_proposals_v1 *value);
id TTNative_thinkthen_complete_boundary_mode_v1(const struct thinkthen_complete_boundary_mode_v1 *value);
id TTNative_thinkthen_complete_entity_edge_list_v1(const struct thinkthen_complete_entity_edge_list_v1 *value);
id TTNative_thinkthen_complete_entity_edge_list_presence_v1(const struct thinkthen_complete_entity_edge_list_presence_v1 *value);
id TTNative_thinkthen_complete_recognize_entities_v1(const struct thinkthen_complete_recognize_entities_v1 *value);
id TTNative_thinkthen_complete_entity_edge_v1(const struct thinkthen_complete_entity_edge_v1 *value);
id TTNative_thinkthen_complete_json_map_entry_v1(const struct thinkthen_complete_json_map_entry_v1 *value);
id TTNative_thinkthen_complete_json_map_v1(const struct thinkthen_complete_json_map_v1 *value);
id TTNative_thinkthen_complete_recognition_mode_presence_v1(const struct thinkthen_complete_recognition_mode_presence_v1 *value);
id TTNative_thinkthen_complete_relation_rule_list_presence_v1(const struct thinkthen_complete_relation_rule_list_presence_v1 *value);
id TTNative_thinkthen_complete_recognition_stage_context_presence_v1(const struct thinkthen_complete_recognition_stage_context_presence_v1 *value);
id TTNative_thinkthen_complete_recognize_question_document_v1(const struct thinkthen_complete_recognize_question_document_v1 *value);
id TTNative_thinkthen_complete_verb_v1(const struct thinkthen_complete_verb_v1 *value);
id TTNative_thinkthen_complete_recognition_stage_context_v1(const struct thinkthen_complete_recognition_stage_context_v1 *value);
id TTNative_thinkthen_complete_recognition_odds_v1(const struct thinkthen_complete_recognition_odds_v1 *value);
id TTNative_thinkthen_complete_piece_odds_list_v1(const struct thinkthen_complete_piece_odds_list_v1 *value);
id TTNative_thinkthen_complete_recognition_odds_boundary_odds_v1(const struct thinkthen_complete_recognition_odds_boundary_odds_v1 *value);
id TTNative_thinkthen_complete_piece_odds_v1(const struct thinkthen_complete_piece_odds_v1 *value);
id TTNative_thinkthen_complete_name_odds_list_v1(const struct thinkthen_complete_name_odds_list_v1 *value);
id TTNative_thinkthen_complete_pair_odds_list_v1(const struct thinkthen_complete_pair_odds_list_v1 *value);
id TTNative_thinkthen_complete_recognition_proposal_list_v1(const struct thinkthen_complete_recognition_proposal_list_v1 *value);
id TTNative_thinkthen_complete_recognition_odds_recognize_answer_v1(const struct thinkthen_complete_recognition_odds_recognize_answer_v1 *value);
id TTNative_thinkthen_complete_place_presence_v1(const struct thinkthen_complete_place_presence_v1 *value);
id TTNative_thinkthen_complete_recognition_proposal_v1(const struct thinkthen_complete_recognition_proposal_v1 *value);
id TTNative_thinkthen_complete_place_v1(const struct thinkthen_complete_place_v1 *value);
id TTNative_thinkthen_complete_pair_odds_v1(const struct thinkthen_complete_pair_odds_v1 *value);
id TTNative_thinkthen_complete_number_map_presence_v1(const struct thinkthen_complete_number_map_presence_v1 *value);
id TTNative_thinkthen_complete_name_odds_v1(const struct thinkthen_complete_name_odds_v1 *value);
id TTNative_thinkthen_complete_annotation_list_v1(const struct thinkthen_complete_annotation_list_v1 *value);
id TTNative_thinkthen_complete_session_packet_annotate_aggregate_v1(const struct thinkthen_complete_session_packet_annotate_aggregate_v1 *value);
id TTNative_thinkthen_complete_annotation_member_map_entry_v1(const struct thinkthen_complete_annotation_member_map_entry_v1 *value);
id TTNative_thinkthen_complete_annotation_member_map_v1(const struct thinkthen_complete_annotation_member_map_v1 *value);
id TTNative_thinkthen_complete_annotation_v1(const struct thinkthen_complete_annotation_v1 *value);
id TTNative_thinkthen_complete_annotated_field_map_entry_v1(const struct thinkthen_complete_annotated_field_map_entry_v1 *value);
id TTNative_thinkthen_complete_annotated_field_map_v1(const struct thinkthen_complete_annotated_field_map_v1 *value);
id TTNative_thinkthen_complete_annotated_row_v1(const struct thinkthen_complete_annotated_row_v1 *value);
id TTNative_thinkthen_complete_annotated_field_v1(const struct thinkthen_complete_annotated_field_v1 *value);
id TTNative_thinkthen_complete_failed_v1(const struct thinkthen_complete_failed_v1 *value);
id TTNative_thinkthen_complete_annotation_member_v1(const struct thinkthen_complete_annotation_member_v1 *value);
id TTNative_thinkthen_complete_annotation_member_failure_id_v1(const struct thinkthen_complete_annotation_member_failure_id_v1 *value);
id TTNative_thinkthen_complete_annotation_member_answer_id_v1(const struct thinkthen_complete_annotation_member_answer_id_v1 *value);
id TTNative_thinkthen_complete_session_packet_find_aggregate_v1(const struct thinkthen_complete_session_packet_find_aggregate_v1 *value);
id TTNative_thinkthen_complete_find_candidate_list_v1(const struct thinkthen_complete_find_candidate_list_v1 *value);
id TTNative_thinkthen_complete_find_candidate_list_presence_v1(const struct thinkthen_complete_find_candidate_list_presence_v1 *value);
id TTNative_thinkthen_complete_find_v1(const struct thinkthen_complete_find_v1 *value);
id TTNative_thinkthen_complete_find_question_document_v1(const struct thinkthen_complete_find_question_document_v1 *value);
id TTNative_thinkthen_complete_find_candidate_v1(const struct thinkthen_complete_find_candidate_v1 *value);
id TTNative_thinkthen_complete_find_answer_v1(const struct thinkthen_complete_find_answer_v1 *value);
id TTNative_thinkthen_complete_atomic_non_zero_usize_list_v1(const struct thinkthen_complete_atomic_non_zero_usize_list_v1 *value);
id TTNative_thinkthen_complete_session_packet_rank_aggregate_v1(const struct thinkthen_complete_session_packet_rank_aggregate_v1 *value);
id TTNative_thinkthen_complete_image_list_v1(const struct thinkthen_complete_image_list_v1 *value);
id TTNative_thinkthen_complete_image_list_presence_v1(const struct thinkthen_complete_image_list_presence_v1 *value);
id TTNative_thinkthen_complete_rank_member_list_v1(const struct thinkthen_complete_rank_member_list_v1 *value);
id TTNative_thinkthen_complete_rank_member_list_presence_v1(const struct thinkthen_complete_rank_member_list_presence_v1 *value);
id TTNative_thinkthen_complete_atomic_non_zero_usize_v1(const struct thinkthen_complete_atomic_non_zero_usize_v1 *value);
id TTNative_thinkthen_complete_rank_member_v1(const struct thinkthen_complete_rank_member_v1 *value);
id TTNative_thinkthen_complete_rank_member_result_v1(const struct thinkthen_complete_rank_member_result_v1 *value);
id TTNative_thinkthen_complete_image_v1(const struct thinkthen_complete_image_v1 *value);
id TTNative_thinkthen_complete_image_media_v1(const struct thinkthen_complete_image_media_v1 *value);
id TTNative_thinkthen_complete_atomic_boolean_list_v1(const struct thinkthen_complete_atomic_boolean_list_v1 *value);
id TTNative_thinkthen_complete_session_packet_filter_aggregate_v1(const struct thinkthen_complete_session_packet_filter_aggregate_v1 *value);
id TTNative_thinkthen_complete_atomic_boolean_v1(const struct thinkthen_complete_atomic_boolean_v1 *value);
id TTNative_thinkthen_complete_atomic_double_list_v1(const struct thinkthen_complete_atomic_double_list_v1 *value);
id TTNative_thinkthen_complete_session_packet_score_aggregate_v1(const struct thinkthen_complete_session_packet_score_aggregate_v1 *value);
id TTNative_thinkthen_complete_atomic_double_v1(const struct thinkthen_complete_atomic_double_v1 *value);
id TTNative_thinkthen_complete_atomic_array_of_string_list_v1(const struct thinkthen_complete_atomic_array_of_string_list_v1 *value);
id TTNative_thinkthen_complete_session_packet_tag_aggregate_v1(const struct thinkthen_complete_session_packet_tag_aggregate_v1 *value);
id TTNative_thinkthen_complete_atomic_array_of_string_v1(const struct thinkthen_complete_atomic_array_of_string_v1 *value);
id TTNative_thinkthen_complete_atomic_nullable_string_list_v1(const struct thinkthen_complete_atomic_nullable_string_list_v1 *value);
id TTNative_thinkthen_complete_session_packet_choose_aggregate_v1(const struct thinkthen_complete_session_packet_choose_aggregate_v1 *value);
id TTNative_thinkthen_complete_atomic_nullable_string_v1(const struct thinkthen_complete_atomic_nullable_string_v1 *value);
id TTNative_thinkthen_complete_atomic_decide_value_list_v1(const struct thinkthen_complete_atomic_decide_value_list_v1 *value);
id TTNative_thinkthen_complete_session_packet_decide_aggregate_v1(const struct thinkthen_complete_session_packet_decide_aggregate_v1 *value);
id TTNative_thinkthen_complete_decide_value_presence_v1(const struct thinkthen_complete_decide_value_presence_v1 *value);
id TTNative_thinkthen_complete_atomic_decide_value_v1(const struct thinkthen_complete_atomic_decide_value_v1 *value);
id TTNative_thinkthen_complete_decide_value_v1(const struct thinkthen_complete_decide_value_v1 *value);
id TTNative_thinkthen_complete_session_packet_annotate_row_v1(const struct thinkthen_complete_session_packet_annotate_row_v1 *value);
id TTNative_thinkthen_complete_session_packet_filter_row_v1(const struct thinkthen_complete_session_packet_filter_row_v1 *value);
id TTNative_thinkthen_complete_session_packet_score_row_v1(const struct thinkthen_complete_session_packet_score_row_v1 *value);
id TTNative_thinkthen_complete_session_packet_tag_row_v1(const struct thinkthen_complete_session_packet_tag_row_v1 *value);
id TTNative_thinkthen_complete_session_packet_choose_row_v1(const struct thinkthen_complete_session_packet_choose_row_v1 *value);
id TTNative_thinkthen_complete_session_packet_decide_row_v1(const struct thinkthen_complete_session_packet_decide_row_v1 *value);
static NSString *TTNativeString(struct thinkthen_complete_utf8_v1 value) {
    NSString *text = [[NSString alloc] initWithBytes:value.data ?: "" length:value.len encoding:NSUTF8StringEncoding];
    if (!text) TTInvalidResult();
    return text;
}
id TTNative_thinkthen_complete_json_v1(const struct thinkthen_complete_json_v1 *value) {
    if (!value) TTInvalidResult();
    switch (value->kind) {
        case THINKTHEN_COMPLETE_JSON_NULL_V1: return NSNull.null;
        case THINKTHEN_COMPLETE_JSON_BOOLEAN_V1: return @((BOOL)value->data.boolean);
        case THINKTHEN_COMPLETE_JSON_STRING_V1: return TTNativeString(value->data.string);
        case THINKTHEN_COMPLETE_JSON_NUMBER_V1: {
            NSString *text = TTNativeString(value->data.number);
            NSDecimalNumber *number = [NSDecimalNumber decimalNumberWithString:text locale:@{
                NSLocaleDecimalSeparator:@"."
            }];
            double floating = text.doubleValue;
            if (![number isEqual:NSDecimalNumber.notANumber] &&
            !([number isEqual:NSDecimalNumber.zero] && floating != 0)) return number;
            if (!isfinite(floating)) TTInvalidResult();
            return @(floating);
        }
        case THINKTHEN_COMPLETE_JSON_ARRAY_V1: {
            NSMutableArray *items = [NSMutableArray array];
            for (size_t i = 0;
            i < value->data.array.len;
            i++) [items addObject:TTNative_thinkthen_complete_json_v1(value->data.array.data[i])];
            return [items copy];
        }
        case THINKTHEN_COMPLETE_JSON_OBJECT_V1: {
            NSMutableDictionary *items = [NSMutableDictionary dictionary];
            for (size_t i = 0;
            i < value->data.object.len;
            i++) {
                const struct thinkthen_complete_json_entry_v1 *item = &value->data.object.data[i];
                items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
            }
            return [items copy];
        }
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_attempt_list_v1(const struct thinkthen_complete_attempt_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_attempt_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_attempt_list_presence_v1(const struct thinkthen_complete_attempt_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_attempt_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_call_id_presence_v1(const struct thinkthen_complete_call_id_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_call_id_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_utf8_presence_v1(const struct thinkthen_complete_utf8_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNativeString(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_u32_presence_v1(const struct thinkthen_complete_u32_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? @(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_u64_presence_v1(const struct thinkthen_complete_u64_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? @(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_persistence_observation_presence_v1(const struct thinkthen_complete_persistence_observation_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_persistence_observation_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_immediate_facts_v1(const struct thinkthen_complete_immediate_facts_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->attempts.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"attempts"] = NSNull.null;
    else if (value->attempts.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"attempts"] = TTNative_thinkthen_complete_attempt_list_v1(&(value->attempts.value));
    items[@"cache_answers"] = @(value->cache_answers);
    if (value->call_id.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"call_id"] = NSNull.null;
    else if (value->call_id.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"call_id"] = TTNative_thinkthen_complete_call_id_v1(value->call_id.value);
    if (value->estimated_cost_usd.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"estimated_cost_usd"] = NSNull.null;
    else if (value->estimated_cost_usd.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"estimated_cost_usd"] = TTNativeString(value->estimated_cost_usd.value);
    if (value->held_model_mismatch.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"held_model_mismatch"] = NSNull.null;
    else if (value->held_model_mismatch.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"held_model_mismatch"] = @((BOOL)value->held_model_mismatch.value);
    if (value->input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input_tokens"] = NSNull.null;
    else if (value->input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input_tokens"] = @(value->input_tokens.value);
    items[@"largest_request_bytes"] = @(value->largest_request_bytes);
    if (value->largest_request_estimated_input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"largest_request_estimated_input_tokens"] = NSNull.null;
    else if (value->largest_request_estimated_input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"largest_request_estimated_input_tokens"] = @(value->largest_request_estimated_input_tokens.value);
    if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"model"] = NSNull.null;
    else if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"model"] = TTNativeString(value->model.value);
    if (value->output_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"output_tokens"] = NSNull.null;
    else if (value->output_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"output_tokens"] = @(value->output_tokens.value);
    items[@"records"] = @(value->records);
    items[@"requests_sent"] = @(value->requests_sent);
    items[@"seconds"] = @(value->seconds);
    items[@"token_estimate_method"] = TTNativeString(value->token_estimate_method);
    if (value->usage_persistence.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"usage_persistence"] = NSNull.null;
    else if (value->usage_persistence.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"usage_persistence"] = TTNative_thinkthen_complete_persistence_observation_v1(value->usage_persistence.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_persistence_observation_v1(const struct thinkthen_complete_persistence_observation_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->advice.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"advice"] = NSNull.null;
    else if (value->advice.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"advice"] = TTNativeString(value->advice.value);
    items[@"observed_at"] = TTNativeString(value->observed_at);
    items[@"state"] = TTNative_thinkthen_complete_usage_persistence_v1(value->state);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_usage_persistence_v1(const struct thinkthen_complete_usage_persistence_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1: return @"disabled";
        case THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1: return @"pending";
        case THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1: return @"written";
        case THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1: return @"failed";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_call_id_v1(const struct thinkthen_complete_call_id_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNativeString(value->value);
}
id TTNative_thinkthen_complete_attempt_v1(const struct thinkthen_complete_attempt_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"ordinal"] = @(value->ordinal);
    items[@"outcome"] = TTNative_thinkthen_complete_attempt_outcome_v1(value->outcome);
    if (value->request_id.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"request_id"] = NSNull.null;
    else if (value->request_id.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"request_id"] = TTNativeString(value->request_id.value);
    items[@"request_sha256"] = TTNativeString(value->request_sha256);
    items[@"sdk_request_id"] = TTNative_thinkthen_complete_sdk_request_id_v1(value->sdk_request_id);
    if (value->server_ms.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"server_ms"] = NSNull.null;
    else if (value->server_ms.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"server_ms"] = @(value->server_ms.value);
    if (value->status.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"status"] = NSNull.null;
    else if (value->status.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"status"] = @(value->status.value);
    items[@"wall_ms"] = @(value->wall_ms);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_sdk_request_id_v1(const struct thinkthen_complete_sdk_request_id_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNativeString(value->value);
}
id TTNative_thinkthen_complete_attempt_outcome_v1(const struct thinkthen_complete_attempt_outcome_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK_V1: return @"ok";
        case THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS_V1: return @"status";
        case THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT_V1: return @"transport";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_session_packet_v1(const struct thinkthen_complete_session_packet_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1: return TTNative_thinkthen_complete_session_packet_decide_row_v1(value->data.decide_row);
        case THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1: return TTNative_thinkthen_complete_session_packet_choose_row_v1(value->data.choose_row);
        case THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1: return TTNative_thinkthen_complete_session_packet_tag_row_v1(value->data.tag_row);
        case THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1: return TTNative_thinkthen_complete_session_packet_score_row_v1(value->data.score_row);
        case THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1: return TTNative_thinkthen_complete_session_packet_filter_row_v1(value->data.filter_row);
        case THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1: return TTNative_thinkthen_complete_session_packet_annotate_row_v1(value->data.annotate_row);
        case THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_decide_aggregate_v1(value->data.decide_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_choose_aggregate_v1(value->data.choose_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_tag_aggregate_v1(value->data.tag_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_score_aggregate_v1(value->data.score_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_filter_aggregate_v1(value->data.filter_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_rank_aggregate_v1(value->data.rank_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_find_aggregate_v1(value->data.find_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_annotate_aggregate_v1(value->data.annotate_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_recognize_aggregate_v1(value->data.recognize_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1: return TTNative_thinkthen_complete_session_packet_relate_aggregate_v1(value->data.relate_aggregate);
        case THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1: return TTNative_thinkthen_complete_session_packet_observation_v1(value->data.observation);
        case THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1: return TTNative_thinkthen_complete_session_packet_terminal_v1(value->data.terminal);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_facts_presence_v1(const struct thinkthen_complete_facts_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_facts_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_call_error_presence_v1(const struct thinkthen_complete_call_error_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_call_error_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_session_packet_terminal_v1(const struct thinkthen_complete_session_packet_terminal_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->facts.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"facts"] = NSNull.null;
    else if (value->facts.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"facts"] = TTNative_thinkthen_complete_facts_v1(value->facts.value);
    if (value->failure.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"failure"] = NSNull.null;
    else if (value->failure.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"failure"] = TTNative_thinkthen_complete_call_error_v1(value->failure.value);
    items[@"kind"] = TTNativeString(value->kind);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_call_error_v1(const struct thinkthen_complete_call_error_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"error"] = TTNative_thinkthen_complete_error_v1(value->error);
    if (value->facts.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"facts"] = NSNull.null;
    else if (value->facts.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"facts"] = TTNative_thinkthen_complete_facts_v1(value->facts.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_facts_v1(const struct thinkthen_complete_facts_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->attempts.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"attempts"] = NSNull.null;
    else if (value->attempts.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"attempts"] = TTNative_thinkthen_complete_attempt_list_v1(&(value->attempts.value));
    items[@"cache_answers"] = @(value->cache_answers);
    items[@"call_id"] = TTNative_thinkthen_complete_call_id_v1(value->call_id);
    if (value->estimated_cost_usd.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"estimated_cost_usd"] = NSNull.null;
    else if (value->estimated_cost_usd.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"estimated_cost_usd"] = TTNativeString(value->estimated_cost_usd.value);
    if (value->held_model_mismatch.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"held_model_mismatch"] = NSNull.null;
    else if (value->held_model_mismatch.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"held_model_mismatch"] = @((BOOL)value->held_model_mismatch.value);
    if (value->input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input_tokens"] = NSNull.null;
    else if (value->input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input_tokens"] = @(value->input_tokens.value);
    items[@"largest_request_bytes"] = @(value->largest_request_bytes);
    if (value->largest_request_estimated_input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"largest_request_estimated_input_tokens"] = NSNull.null;
    else if (value->largest_request_estimated_input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"largest_request_estimated_input_tokens"] = @(value->largest_request_estimated_input_tokens.value);
    if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"model"] = NSNull.null;
    else if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"model"] = TTNativeString(value->model.value);
    if (value->output_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"output_tokens"] = NSNull.null;
    else if (value->output_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"output_tokens"] = @(value->output_tokens.value);
    items[@"records"] = @(value->records);
    items[@"requests_sent"] = @(value->requests_sent);
    items[@"seconds"] = @(value->seconds);
    items[@"token_estimate_method"] = TTNativeString(value->token_estimate_method);
    if (value->usage_persistence.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"usage_persistence"] = NSNull.null;
    else if (value->usage_persistence.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"usage_persistence"] = TTNative_thinkthen_complete_persistence_observation_v1(value->usage_persistence.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_input_denial_presence_v1(const struct thinkthen_complete_input_denial_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_input_denial_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_send_denial_presence_v1(const struct thinkthen_complete_send_denial_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_send_denial_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_error_v1(const struct thinkthen_complete_error_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->estimated_input_denial.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"estimated_input_denial"] = NSNull.null;
    else if (value->estimated_input_denial.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"estimated_input_denial"] = TTNative_thinkthen_complete_input_denial_v1(value->estimated_input_denial.value);
    items[@"kind"] = TTNative_thinkthen_complete_failure_kind_v1(value->kind);
    items[@"message"] = TTNativeString(value->message);
    items[@"retryable"] = @((BOOL)value->retryable);
    if (value->send_budget_denial.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"send_budget_denial"] = NSNull.null;
    else if (value->send_budget_denial.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"send_budget_denial"] = TTNative_thinkthen_complete_send_denial_v1(value->send_budget_denial.value);
    items[@"stopped"] = TTNative_thinkthen_complete_stopped_v1(value->stopped);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_stopped_v1(const struct thinkthen_complete_stopped_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->at.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"at"] = NSNull.null;
    else if (value->at.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"at"] = @(value->at.value);
    items[@"cause"] = TTNative_thinkthen_complete_stop_cause_v1(value->cause);
    items[@"retryable"] = @((BOOL)value->retryable);
    if (value->status.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"status"] = NSNull.null;
    else if (value->status.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"status"] = @(value->status.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_stop_cause_v1(const struct thinkthen_complete_stop_cause_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_STOP_CAUSE_USAGE_V1: return @"usage";
        case THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL_V1: return @"local";
        case THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY_V1: return @"no_key";
        case THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT_V1: return @"transport";
        case THINKTHEN_COMPLETE_STOP_CAUSE_STATUS_V1: return @"status";
        case THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE_V1: return @"too_large";
        case THINKTHEN_COMPLETE_STOP_CAUSE_REPLY_V1: return @"reply";
        case THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND_V1: return @"backend";
        case THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED_V1: return @"cancelled";
        case THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE_V1: return @"deadline";
        case THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT_V1: return @"defect";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_send_denial_v1(const struct thinkthen_complete_send_denial_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND_V1: return TTNative_thinkthen_complete_send_denial_before_first_send_v1(value->data.before_first_send);
        case THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND_V1: return TTNative_thinkthen_complete_send_denial_before_additional_send_v1(value->data.before_additional_send);
        case THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY_V1: return TTNative_thinkthen_complete_send_denial_before_retry_v1(value->data.before_retry);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_send_denial_before_retry_v1(const struct thinkthen_complete_send_denial_before_retry_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"last_status"] = @(value->last_status);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_send_denial_before_additional_send_v1(const struct thinkthen_complete_send_denial_before_additional_send_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_send_denial_before_first_send_v1(const struct thinkthen_complete_send_denial_before_first_send_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_failure_kind_v1(const struct thinkthen_complete_failure_kind_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_FAILURE_KIND_USAGE_V1: return @"usage";
        case THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1: return @"backend";
        case THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1: return @"local";
        case THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED_V1: return @"cancelled";
        case THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE_V1: return @"deadline";
        case THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT_V1: return @"defect";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_input_denial_v1(const struct thinkthen_complete_input_denial_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST_V1: return TTNative_thinkthen_complete_input_denial_initial_request_v1(value->data.initial_request);
        case THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST_V1: return TTNative_thinkthen_complete_input_denial_additional_request_v1(value->data.additional_request);
        case THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY_V1: return TTNative_thinkthen_complete_input_denial_retry_v1(value->data.retry);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_input_denial_retry_v1(const struct thinkthen_complete_input_denial_retry_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"last_status"] = @(value->last_status);
    items[@"limit"] = @(value->limit);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_input_denial_additional_request_v1(const struct thinkthen_complete_input_denial_additional_request_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"limit"] = @(value->limit);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_input_denial_initial_request_v1(const struct thinkthen_complete_input_denial_initial_request_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"limit"] = @(value->limit);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_observation_v1(const struct thinkthen_complete_session_packet_observation_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNative_thinkthen_complete_request_function_v1(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_session_observation_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_observation_v1(const struct thinkthen_complete_session_observation_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION_V1: return TTNative_thinkthen_complete_session_observation_question_v1(value->data.question);
        case THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW_V1: return TTNative_thinkthen_complete_session_observation_row_v1(value->data.row);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_session_observation_row_v1(const struct thinkthen_complete_session_observation_row_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"index"] = @(value->index);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_session_observed_row_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_observed_row_v1(const struct thinkthen_complete_session_observed_row_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT_V1: return TTNative_thinkthen_complete_session_observed_row_judgment_v1(value->data.judgment);
        case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED_V1: return TTNative_thinkthen_complete_session_observed_row_annotated_v1(value->data.annotated);
        case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED_V1: return TTNative_thinkthen_complete_session_observed_row_recognized_v1(value->data.recognized);
        case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND_V1: return TTNative_thinkthen_complete_session_observed_row_find_v1(value->data.find);
        case THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS_V1: return TTNative_thinkthen_complete_session_observed_row_relations_v1(value->data.relations);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_session_relation_edge_list_v1(const struct thinkthen_complete_session_relation_edge_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_session_relation_edge_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_observed_row_relations_v1(const struct thinkthen_complete_session_observed_row_relations_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_session_relation_edge_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_relation_edge_v1(const struct thinkthen_complete_session_relation_edge_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"either"] = @((BOOL)value->either);
    items[@"probability"] = @(value->probability);
    items[@"relation"] = TTNativeString(value->relation);
    items[@"source"] = TTNative_thinkthen_complete_entity_document_v1(value->source);
    items[@"target"] = TTNative_thinkthen_complete_entity_document_v1(value->target);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_entity_document_v1(const struct thinkthen_complete_entity_document_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"name"] = TTNativeString(value->name);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_observed_row_find_v1(const struct thinkthen_complete_session_observed_row_find_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = @(value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_observed_row_recognized_v1(const struct thinkthen_complete_session_observed_row_recognized_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_session_recognition_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_entity_list_v1(const struct thinkthen_complete_entity_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_entity_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_boundary_proposal_list_v1(const struct thinkthen_complete_boundary_proposal_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_boundary_proposal_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_boundary_proposal_list_presence_v1(const struct thinkthen_complete_boundary_proposal_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_boundary_proposal_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_recognition_edge_list_v1(const struct thinkthen_complete_recognition_edge_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_recognition_edge_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_edge_list_presence_v1(const struct thinkthen_complete_recognition_edge_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_recognition_edge_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_session_recognition_v1(const struct thinkthen_complete_session_recognition_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"entities"] = TTNative_thinkthen_complete_entity_list_v1(&(value->entities));
    items[@"mode"] = TTNative_thinkthen_complete_recognition_mode_v1(value->mode);
    if (value->proposals.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"proposals"] = NSNull.null;
    else if (value->proposals.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"proposals"] = TTNative_thinkthen_complete_boundary_proposal_list_v1(&(value->proposals.value));
    if (value->relations.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"relations"] = NSNull.null;
    else if (value->relations.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"relations"] = TTNative_thinkthen_complete_recognition_edge_list_v1(&(value->relations.value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_edge_v1(const struct thinkthen_complete_recognition_edge_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"either"] = @((BOOL)value->either);
    items[@"probability"] = @(value->probability);
    items[@"relation"] = TTNativeString(value->relation);
    items[@"source"] = TTNative_thinkthen_complete_entity_v1(value->source);
    items[@"target"] = TTNative_thinkthen_complete_entity_v1(value->target);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_entity_v1(const struct thinkthen_complete_entity_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"end"] = @(value->end);
    if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"file"] = NSNull.null;
    else if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"file"] = TTNativeString(value->file.value);
    if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"first_line"] = NSNull.null;
    else if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"first_line"] = @(value->first_line.value);
    items[@"kind"] = TTNativeString(value->kind);
    if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"last_line"] = NSNull.null;
    else if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"last_line"] = @(value->last_line.value);
    items[@"length"] = @(value->length);
    items[@"start"] = @(value->start);
    items[@"strength"] = @(value->strength);
    items[@"text"] = TTNativeString(value->text);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_boundary_proposal_v1(const struct thinkthen_complete_boundary_proposal_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"end"] = @(value->end);
    items[@"length"] = @(value->length);
    items[@"probability"] = @(value->probability);
    items[@"start"] = @(value->start);
    items[@"text"] = TTNativeString(value->text);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_mode_v1(const struct thinkthen_complete_recognition_mode_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE_V1: return @"whole";
        case THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY_V1: return @"boundary_only";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_session_annotation_list_v1(const struct thinkthen_complete_session_annotation_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_session_annotation_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_observed_row_annotated_v1(const struct thinkthen_complete_session_observed_row_annotated_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_session_annotation_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_annotation_v1(const struct thinkthen_complete_session_annotation_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"name"] = TTNativeString(value->name);
    items[@"value"] = TTNative_thinkthen_complete_annotation_value_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_value_v1(const struct thinkthen_complete_annotation_value_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION_V1: return TTNative_thinkthen_complete_annotation_value_decision_v1(value->data.decision);
        case THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE_V1: return TTNative_thinkthen_complete_annotation_value_choice_v1(value->data.choice);
        case THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE_V1: return TTNative_thinkthen_complete_annotation_value_score_v1(value->data.score);
        case THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS_V1: return TTNative_thinkthen_complete_annotation_value_tags_v1(value->data.tags);
        case THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED_V1: return TTNative_thinkthen_complete_annotation_value_failed_v1(value->data.failed);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_annotation_value_failed_v1(const struct thinkthen_complete_annotation_value_failed_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_failure_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_failure_field_kind_v1(const struct thinkthen_complete_failure_field_kind_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND_V1: return @"backend";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_failure_v1(const struct thinkthen_complete_failure_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"cause"] = TTNative_thinkthen_complete_failure_cause_v1(value->cause);
    items[@"kind"] = TTNative_thinkthen_complete_failure_field_kind_v1(value->kind);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_failure_cause_v1(const struct thinkthen_complete_failure_cause_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER_V1: return @"missing_answer";
        case THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND_V1: return @"wrong_kind";
        case THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY_V1: return @"missing_probability";
        case THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY_V1: return @"invalid_probability";
        case THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION_V1: return @"invalid_distribution";
        case THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY_V1: return @"unexpected_probability";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_string_list_v1(const struct thinkthen_complete_string_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNativeString(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_value_tags_v1(const struct thinkthen_complete_annotation_value_tags_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_string_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_value_score_v1(const struct thinkthen_complete_annotation_value_score_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = @(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_value_choice_v1(const struct thinkthen_complete_annotation_value_choice_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = TTNativeString(value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_value_decision_v1(const struct thinkthen_complete_annotation_value_decision_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = @((BOOL)value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_observed_row_judgment_v1(const struct thinkthen_complete_session_observed_row_judgment_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_session_judgment_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_judgment_v1(const struct thinkthen_complete_session_judgment_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION_V1: return TTNative_thinkthen_complete_session_judgment_decision_v1(value->data.decision);
        case THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE_V1: return TTNative_thinkthen_complete_session_judgment_choice_v1(value->data.choice);
        case THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE_V1: return TTNative_thinkthen_complete_session_judgment_score_v1(value->data.score);
        case THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS_V1: return TTNative_thinkthen_complete_session_judgment_tags_v1(value->data.tags);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_session_judgment_tags_v1(const struct thinkthen_complete_session_judgment_tags_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_string_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_judgment_score_v1(const struct thinkthen_complete_session_judgment_score_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = @(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_judgment_choice_v1(const struct thinkthen_complete_session_judgment_choice_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = TTNativeString(value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_judgment_decision_v1(const struct thinkthen_complete_session_judgment_decision_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = @((BOOL)value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_observation_question_v1(const struct thinkthen_complete_session_observation_question_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"detail"] = TTNative_thinkthen_complete_session_question_detail_v1(value->detail);
    items[@"index"] = @(value->index);
    items[@"kind"] = TTNativeString(value->kind);
    if (value->member.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"member"] = NSNull.null;
    else if (value->member.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"member"] = TTNativeString(value->member.value);
    items[@"position"] = @(value->position);
    if (value->stage.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"stage"] = NSNull.null;
    else if (value->stage.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"stage"] = TTNativeString(value->stage.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_answer_id_presence_v1(const struct thinkthen_complete_answer_id_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_answer_id_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_f64_presence_v1(const struct thinkthen_complete_f64_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? @(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_failure_presence_v1(const struct thinkthen_complete_failure_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_failure_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_failure_id_presence_v1(const struct thinkthen_complete_failure_id_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_failure_id_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_json_presence_v1(const struct thinkthen_complete_json_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_json_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_physical_source_presence_v1(const struct thinkthen_complete_physical_source_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_physical_source_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_session_input_source_list_v1(const struct thinkthen_complete_session_input_source_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_session_input_source_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_json_list_v1(const struct thinkthen_complete_json_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_json_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_observation_list_v1(const struct thinkthen_complete_observation_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_observation_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_probabilities_presence_v1(const struct thinkthen_complete_session_probabilities_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_session_probabilities_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_question_source_list_v1(const struct thinkthen_complete_question_source_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_question_source_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_usage_presence_v1(const struct thinkthen_complete_usage_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_usage_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_threshold_presence_v1(const struct thinkthen_complete_threshold_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_threshold_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_token_usage_presence_v1(const struct thinkthen_complete_token_usage_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_token_usage_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_value_presence_v1(const struct thinkthen_complete_value_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_value_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_session_question_detail_v1(const struct thinkthen_complete_session_question_detail_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->answer_id.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"answer_id"] = NSNull.null;
    else if (value->answer_id.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id.value);
    items[@"cached"] = @((BOOL)value->cached);
    if (value->confidence.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"confidence"] = NSNull.null;
    else if (value->confidence.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"confidence"] = @(value->confidence.value);
    items[@"failed_questions"] = @(value->failed_questions);
    if (value->failure.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"failure"] = NSNull.null;
    else if (value->failure.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"failure"] = TTNative_thinkthen_complete_failure_v1(value->failure.value);
    if (value->failure_id.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"failure_id"] = NSNull.null;
    else if (value->failure_id.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"failure_id"] = TTNative_thinkthen_complete_failure_id_v1(value->failure_id.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->input_source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input_source"] = NSNull.null;
    else if (value->input_source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input_source"] = TTNative_thinkthen_complete_physical_source_v1(value->input_source.value);
    items[@"input_sources"] = TTNative_thinkthen_complete_session_input_source_list_v1(&(value->input_sources));
    items[@"inputs"] = TTNative_thinkthen_complete_json_list_v1(&(value->inputs));
    items[@"model"] = TTNativeString(value->model);
    items[@"observations"] = TTNative_thinkthen_complete_observation_list_v1(&(value->observations));
    if (value->probabilities.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"probabilities"] = NSNull.null;
    else if (value->probabilities.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"probabilities"] = TTNative_thinkthen_complete_session_probabilities_v1(value->probabilities.value);
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    items[@"question_sha256"] = TTNativeString(value->question_sha256);
    items[@"question_sources"] = TTNative_thinkthen_complete_question_source_list_v1(&(value->question_sources));
    if (value->raw_pick.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"raw_pick"] = NSNull.null;
    else if (value->raw_pick.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"raw_pick"] = TTNativeString(value->raw_pick.value);
    if (value->reported_usage.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"reported_usage"] = NSNull.null;
    else if (value->reported_usage.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"reported_usage"] = TTNative_thinkthen_complete_usage_v1(value->reported_usage.value);
    items[@"requests"] = TTNative_thinkthen_complete_string_list_v1(&(value->requests));
    items[@"requests_sent"] = @(value->requests_sent);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold.value);
    items[@"url"] = TTNativeString(value->url);
    if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"usage"] = NSNull.null;
    else if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"usage"] = TTNative_thinkthen_complete_token_usage_v1(value->usage.value);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = TTNative_thinkthen_complete_value_v1(value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_value_v1(const struct thinkthen_complete_value_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_VALUE_BOOLEAN_V1: return @((BOOL)value->data.boolean);
        case THINKTHEN_COMPLETE_VALUE_NULL_V1: return NSNull.null;
        case THINKTHEN_COMPLETE_VALUE_STRING_V1: return TTNativeString(value->data.string);
        case THINKTHEN_COMPLETE_VALUE_ARRAY_V1: return TTNative_thinkthen_complete_string_list_v1(&(value->data.array));
        case THINKTHEN_COMPLETE_VALUE_NUMBER_V1: return @(value->data.number);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_token_usage_v1(const struct thinkthen_complete_token_usage_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"input_tokens"] = @(value->input_tokens);
    items[@"output_tokens"] = @(value->output_tokens);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_threshold_v1(const struct thinkthen_complete_threshold_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_THRESHOLD_NUMBER_V1: return @(value->data.number);
        case THINKTHEN_COMPLETE_THRESHOLD_STRING_V1: return TTNativeString(value->data.string);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_usage_v1(const struct thinkthen_complete_usage_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input_tokens"] = NSNull.null;
    else if (value->input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input_tokens"] = @(value->input_tokens.value);
    if (value->output_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"output_tokens"] = NSNull.null;
    else if (value->output_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"output_tokens"] = @(value->output_tokens.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_question_source_v1(const struct thinkthen_complete_question_source_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answered_by"] = TTNativeString(value->answered_by);
    if (value->batch_size.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch_size"] = NSNull.null;
    else if (value->batch_size.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch_size"] = @(value->batch_size.value);
    items[@"origin"] = TTNative_thinkthen_complete_origin_v1(value->origin);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_origin_v1(const struct thinkthen_complete_origin_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_ORIGIN_LIVE_V1: return @"live";
        case THINKTHEN_COMPLETE_ORIGIN_CACHE_V1: return @"cache";
        case THINKTHEN_COMPLETE_ORIGIN_REPLAY_V1: return @"replay";
        case THINKTHEN_COMPLETE_ORIGIN_PROXY_V1: return @"proxy";
        case THINKTHEN_COMPLETE_ORIGIN_MEMORY_V1: return @"memory";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_question_document_v1(const struct thinkthen_complete_question_document_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE_V1: return TTNative_thinkthen_complete_question_document_decide_v1(value->data.decide);
        case THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE_V1: return TTNative_thinkthen_complete_question_document_choose_v1(value->data.choose);
        case THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG_V1: return TTNative_thinkthen_complete_question_document_tag_v1(value->data.tag);
        case THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE_V1: return TTNative_thinkthen_complete_question_document_score_v1(value->data.score);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_batch_presence_v1(const struct thinkthen_complete_batch_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_batch_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_input_declaration_presence_v1(const struct thinkthen_complete_input_declaration_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_input_declaration_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_label_list_v1(const struct thinkthen_complete_label_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_label_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_label_list_presence_v1(const struct thinkthen_complete_label_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_label_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_question_name_presence_v1(const struct thinkthen_complete_question_name_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_question_name_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_string_list_presence_v1(const struct thinkthen_complete_string_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_string_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_wording_version_presence_v1(const struct thinkthen_complete_wording_version_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_wording_version_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_question_document_score_v1(const struct thinkthen_complete_question_document_score_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch"] = NSNull.null;
    else if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch"] = TTNative_thinkthen_complete_batch_v1(value->batch.value);
    if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"context_schema"] = NSNull.null;
    else if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"context_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->context_schema.value);
    if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"item_schema"] = NSNull.null;
    else if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"item_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->item_schema.value);
    if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"label_details"] = NSNull.null;
    else if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"label_details"] = TTNative_thinkthen_complete_label_list_v1(&(value->label_details.value));
    if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"model"] = NSNull.null;
    else if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"model"] = TTNativeString(value->model.value);
    if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"name"] = NSNull.null;
    else if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"name"] = TTNative_thinkthen_complete_question_name_v1(value->name.value);
    if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"on"] = NSNull.null;
    else if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"on"] = TTNative_thinkthen_complete_string_list_v1(&(value->on.value));
    if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"profile"] = NSNull.null;
    else if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"profile"] = TTNativeString(value->profile.value);
    if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"wording_version"] = NSNull.null;
    else if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"wording_version"] = TTNative_thinkthen_complete_wording_version_v1(value->wording_version.value);
    items[@"levels"] = TTNative_thinkthen_complete_string_list_v1(&(value->levels));
    if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"text"] = NSNull.null;
    else if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"text"] = TTNative_thinkthen_complete_json_v1(value->text.value);
    items[@"verb"] = TTNativeString(value->verb);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_wording_version_v1(const struct thinkthen_complete_wording_version_v1 *value) {
    if (!value) TTInvalidResult();
    return @(value->value);
}
id TTNative_thinkthen_complete_question_name_v1(const struct thinkthen_complete_question_name_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNativeString(value->value);
}
id TTNative_thinkthen_complete_label_v1(const struct thinkthen_complete_label_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->description.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"description"] = NSNull.null;
    else if (value->description.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"description"] = TTNative_thinkthen_complete_json_v1(value->description.value);
    items[@"name"] = TTNativeString(value->name);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_input_declaration_v1(const struct thinkthen_complete_input_declaration_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT_V1: return TTNative_thinkthen_complete_input_declaration_string_root_v1(value->data.string_root);
        case THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT_V1: return TTNative_thinkthen_complete_input_declaration_object_root_v1(value->data.object_root);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_input_property_type_map_entry_v1(const struct thinkthen_complete_input_property_type_map_entry_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNative_thinkthen_complete_input_property_type_v1(value->value);
}
id TTNative_thinkthen_complete_input_property_type_map_v1(const struct thinkthen_complete_input_property_type_map_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    for (size_t i = 0;
    i < value->len;
    i++) {
        items[TTNativeString(value->data[i].name)] = TTNative_thinkthen_complete_input_property_type_v1(value->data[i].value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_input_declaration_object_root_v1(const struct thinkthen_complete_input_declaration_object_root_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"properties"] = TTNative_thinkthen_complete_input_property_type_map_v1(&(value->properties));
    if (value->required.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"required"] = NSNull.null;
    else if (value->required.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"required"] = TTNative_thinkthen_complete_string_list_v1(&(value->required.value));
    items[@"type"] = TTNative_thinkthen_complete_object_type_v1(value->type);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_object_type_v1(const struct thinkthen_complete_object_type_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT_V1: return @"object";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_input_property_type_v1(const struct thinkthen_complete_input_property_type_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING_V1: return TTNative_thinkthen_complete_input_property_type_string_v1(value->data.string);
        case THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER_V1: return TTNative_thinkthen_complete_input_property_type_number_v1(value->data.number);
        case THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN_V1: return @((BOOL)value->data.boolean);
        case THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY_V1: return TTNative_thinkthen_complete_input_property_type_array_v1(value->data.array);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_input_property_type_array_v1(const struct thinkthen_complete_input_property_type_array_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"items"] = TTNative_thinkthen_complete_string_root_v1(value->items);
    items[@"type"] = TTNativeString(value->type);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_string_root_v1(const struct thinkthen_complete_string_root_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"type"] = TTNative_thinkthen_complete_string_type_v1(value->type);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_string_type_v1(const struct thinkthen_complete_string_type_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_STRING_TYPE_STRING_V1: return @"string";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_input_property_type_boolean_v1(const struct thinkthen_complete_input_property_type_boolean_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"type"] = TTNativeString(value->type);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_input_property_type_number_v1(const struct thinkthen_complete_input_property_type_number_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"type"] = TTNativeString(value->type);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_input_property_type_string_v1(const struct thinkthen_complete_input_property_type_string_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"type"] = TTNativeString(value->type);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_input_declaration_string_root_v1(const struct thinkthen_complete_input_declaration_string_root_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"type"] = TTNative_thinkthen_complete_string_type_v1(value->type);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_batch_string_v1(const struct thinkthen_complete_batch_string_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_BATCH_STRING_MAX_V1: return @"max";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_batch_v1(const struct thinkthen_complete_batch_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_BATCH_INTEGER_V1: return @(value->data.integer);
        case THINKTHEN_COMPLETE_BATCH_STRING_V1: return TTNative_thinkthen_complete_batch_string_v1(value->data.string);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_question_document_tag_v1(const struct thinkthen_complete_question_document_tag_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch"] = NSNull.null;
    else if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch"] = TTNative_thinkthen_complete_batch_v1(value->batch.value);
    if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"context_schema"] = NSNull.null;
    else if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"context_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->context_schema.value);
    if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"item_schema"] = NSNull.null;
    else if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"item_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->item_schema.value);
    if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"label_details"] = NSNull.null;
    else if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"label_details"] = TTNative_thinkthen_complete_label_list_v1(&(value->label_details.value));
    if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"model"] = NSNull.null;
    else if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"model"] = TTNativeString(value->model.value);
    if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"name"] = NSNull.null;
    else if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"name"] = TTNative_thinkthen_complete_question_name_v1(value->name.value);
    if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"on"] = NSNull.null;
    else if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"on"] = TTNative_thinkthen_complete_string_list_v1(&(value->on.value));
    if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"profile"] = NSNull.null;
    else if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"profile"] = TTNativeString(value->profile.value);
    if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"wording_version"] = NSNull.null;
    else if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"wording_version"] = TTNative_thinkthen_complete_wording_version_v1(value->wording_version.value);
    items[@"labels"] = TTNative_thinkthen_complete_string_list_v1(&(value->labels));
    if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"text"] = NSNull.null;
    else if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"text"] = TTNative_thinkthen_complete_json_v1(value->text.value);
    items[@"verb"] = TTNativeString(value->verb);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_question_document_choose_v1(const struct thinkthen_complete_question_document_choose_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch"] = NSNull.null;
    else if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch"] = TTNative_thinkthen_complete_batch_v1(value->batch.value);
    if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"context_schema"] = NSNull.null;
    else if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"context_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->context_schema.value);
    if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"item_schema"] = NSNull.null;
    else if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"item_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->item_schema.value);
    if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"label_details"] = NSNull.null;
    else if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"label_details"] = TTNative_thinkthen_complete_label_list_v1(&(value->label_details.value));
    if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"model"] = NSNull.null;
    else if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"model"] = TTNativeString(value->model.value);
    if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"name"] = NSNull.null;
    else if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"name"] = TTNative_thinkthen_complete_question_name_v1(value->name.value);
    if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"on"] = NSNull.null;
    else if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"on"] = TTNative_thinkthen_complete_string_list_v1(&(value->on.value));
    if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"profile"] = NSNull.null;
    else if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"profile"] = TTNativeString(value->profile.value);
    if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"wording_version"] = NSNull.null;
    else if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"wording_version"] = TTNative_thinkthen_complete_wording_version_v1(value->wording_version.value);
    items[@"options"] = TTNative_thinkthen_complete_string_list_v1(&(value->options));
    if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"text"] = NSNull.null;
    else if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"text"] = TTNative_thinkthen_complete_json_v1(value->text.value);
    items[@"verb"] = TTNativeString(value->verb);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_question_document_decide_v1(const struct thinkthen_complete_question_document_decide_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch"] = NSNull.null;
    else if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch"] = TTNative_thinkthen_complete_batch_v1(value->batch.value);
    if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"context_schema"] = NSNull.null;
    else if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"context_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->context_schema.value);
    if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"item_schema"] = NSNull.null;
    else if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"item_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->item_schema.value);
    if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"label_details"] = NSNull.null;
    else if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"label_details"] = TTNative_thinkthen_complete_label_list_v1(&(value->label_details.value));
    if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"model"] = NSNull.null;
    else if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"model"] = TTNativeString(value->model.value);
    if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"name"] = NSNull.null;
    else if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"name"] = TTNative_thinkthen_complete_question_name_v1(value->name.value);
    if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"on"] = NSNull.null;
    else if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"on"] = TTNative_thinkthen_complete_string_list_v1(&(value->on.value));
    if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"profile"] = NSNull.null;
    else if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"profile"] = TTNativeString(value->profile.value);
    if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"wording_version"] = NSNull.null;
    else if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"wording_version"] = TTNative_thinkthen_complete_wording_version_v1(value->wording_version.value);
    if (value->false_.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"false"] = NSNull.null;
    else if (value->false_.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"false"] = TTNative_thinkthen_complete_json_v1(value->false_.value);
    if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"text"] = NSNull.null;
    else if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"text"] = TTNative_thinkthen_complete_json_v1(value->text.value);
    if (value->true_.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"true"] = NSNull.null;
    else if (value->true_.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"true"] = TTNative_thinkthen_complete_json_v1(value->true_.value);
    items[@"verb"] = TTNativeString(value->verb);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_probabilities_v1(const struct thinkthen_complete_session_probabilities_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO_V1: return TTNative_thinkthen_complete_session_probabilities_yes_no_v1(value->data.yes_no);
        case THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED_V1: return TTNative_thinkthen_complete_session_probabilities_named_v1(value->data.named);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_session_named_probability_list_v1(const struct thinkthen_complete_session_named_probability_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_session_named_probability_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_probabilities_named_v1(const struct thinkthen_complete_session_probabilities_named_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_session_named_probability_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_named_probability_v1(const struct thinkthen_complete_session_named_probability_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"name"] = TTNativeString(value->name);
    items[@"probability"] = @(value->probability);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_probabilities_yes_no_v1(const struct thinkthen_complete_session_probabilities_yes_no_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = @(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_observation_v1(const struct thinkthen_complete_observation_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID_V1: return TTNative_thinkthen_complete_observation_observation_id_v1(value->data.observation_id);
        case THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID_V1: return TTNative_thinkthen_complete_observation_failure_id_v1(value->data.failure_id);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_observation_failure_id_v1(const struct thinkthen_complete_observation_failure_id_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"failure_id"] = TTNative_thinkthen_complete_failure_id_v1(value->failure_id);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_failure_id_v1(const struct thinkthen_complete_failure_id_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNativeString(value->value);
}
id TTNative_thinkthen_complete_observation_observation_id_v1(const struct thinkthen_complete_observation_observation_id_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"observation_id"] = TTNative_thinkthen_complete_observation_id_v1(value->observation_id);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_observation_id_v1(const struct thinkthen_complete_observation_id_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNativeString(value->value);
}
id TTNative_thinkthen_complete_session_input_source_v1(const struct thinkthen_complete_session_input_source_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"index"] = @(value->index);
    items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_physical_source_v1(const struct thinkthen_complete_physical_source_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"file"] = TTNativeString(value->file);
    if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"first_line"] = NSNull.null;
    else if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"first_line"] = @(value->first_line.value);
    if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"last_line"] = NSNull.null;
    else if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"last_line"] = @(value->last_line.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_answer_id_v1(const struct thinkthen_complete_answer_id_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNativeString(value->value);
}
id TTNative_thinkthen_complete_request_function_v1(const struct thinkthen_complete_request_function_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1: return @"decide";
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1: return @"choose";
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1: return @"tag";
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1: return @"score";
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1: return @"filter";
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1: return @"rank";
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1: return @"find";
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1: return @"annotate";
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1: return @"recognize";
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1: return @"relate";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_session_packet_relate_aggregate_v1(const struct thinkthen_complete_session_packet_relate_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_relation_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_input_source_list_presence_v1(const struct thinkthen_complete_session_input_source_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_session_input_source_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_position_presence_v1(const struct thinkthen_complete_position_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_position_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_related_entity_edge_list_v1(const struct thinkthen_complete_related_entity_edge_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_related_entity_edge_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_relation_v1(const struct thinkthen_complete_relation_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_answers_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"file"] = NSNull.null;
    else if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"file"] = TTNativeString(value->file.value);
    if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"first_line"] = NSNull.null;
    else if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"first_line"] = @(value->first_line.value);
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->input_sources.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input_sources"] = NSNull.null;
    else if (value->input_sources.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input_sources"] = TTNative_thinkthen_complete_session_input_source_list_v1(&(value->input_sources.value));
    if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"last_line"] = NSNull.null;
    else if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"last_line"] = @(value->last_line.value);
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    if (value->position.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"position"] = NSNull.null;
    else if (value->position.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"position"] = TTNative_thinkthen_complete_position_v1(value->position.value);
    items[@"question"] = TTNative_thinkthen_complete_relate_question_document_v1(value->question);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    items[@"value"] = TTNative_thinkthen_complete_related_entity_edge_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_related_entity_edge_v1(const struct thinkthen_complete_related_entity_edge_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->either.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"either"] = NSNull.null;
    else if (value->either.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"either"] = @((BOOL)value->either.value);
    items[@"probability"] = @(value->probability);
    items[@"relation"] = TTNativeString(value->relation);
    items[@"source"] = TTNative_thinkthen_complete_relation_endpoint_v1(value->source);
    items[@"target"] = TTNative_thinkthen_complete_relation_endpoint_v1(value->target);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_relation_endpoint_v1(const struct thinkthen_complete_relation_endpoint_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY_V1: return TTNative_thinkthen_complete_relation_endpoint_related_entity_v1(value->data.related_entity);
        case THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE_V1: return TTNative_thinkthen_complete_relation_endpoint_source_v1(value->data.source);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_relation_endpoint_source_v1(const struct thinkthen_complete_relation_endpoint_source_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"file"] = NSNull.null;
    else if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"file"] = TTNativeString(value->file.value);
    if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"first_line"] = NSNull.null;
    else if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"first_line"] = @(value->first_line.value);
    items[@"kind"] = TTNativeString(value->kind);
    if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"last_line"] = NSNull.null;
    else if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"last_line"] = @(value->last_line.value);
    items[@"name"] = TTNativeString(value->name);
    items[@"ordinal"] = @(value->ordinal);
    if (value->record.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"record"] = NSNull.null;
    else if (value->record.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"record"] = TTNative_thinkthen_complete_json_v1(value->record.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_relation_endpoint_related_entity_v1(const struct thinkthen_complete_relation_endpoint_related_entity_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"name"] = TTNativeString(value->name);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_version_v1(const struct thinkthen_complete_version_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_VERSION_THINKTHEN_RESULT_2_V1: return @"thinkthen.result/2";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_relate_fields_presence_v1(const struct thinkthen_complete_relate_fields_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_relate_fields_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_relation_rule_list_v1(const struct thinkthen_complete_relation_rule_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_relation_rule_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_relate_question_document_v1(const struct thinkthen_complete_relate_question_document_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch"] = NSNull.null;
    else if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch"] = TTNative_thinkthen_complete_batch_v1(value->batch.value);
    if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"context_schema"] = NSNull.null;
    else if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"context_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->context_schema.value);
    if (value->fields.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"fields"] = NSNull.null;
    else if (value->fields.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"fields"] = TTNative_thinkthen_complete_relate_fields_v1(value->fields.value);
    if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"item_schema"] = NSNull.null;
    else if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"item_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->item_schema.value);
    if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"label_details"] = NSNull.null;
    else if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"label_details"] = TTNative_thinkthen_complete_label_list_v1(&(value->label_details.value));
    if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"model"] = NSNull.null;
    else if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"model"] = TTNativeString(value->model.value);
    if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"name"] = NSNull.null;
    else if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"name"] = TTNative_thinkthen_complete_question_name_v1(value->name.value);
    if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"on"] = NSNull.null;
    else if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"on"] = TTNative_thinkthen_complete_string_list_v1(&(value->on.value));
    if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"profile"] = NSNull.null;
    else if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"profile"] = TTNativeString(value->profile.value);
    items[@"relations"] = TTNative_thinkthen_complete_relation_rule_list_v1(&(value->relations));
    items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold);
    items[@"verb"] = TTNativeString(value->verb);
    if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"wording_version"] = NSNull.null;
    else if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"wording_version"] = TTNative_thinkthen_complete_wording_version_v1(value->wording_version.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_relation_rule_v1(const struct thinkthen_complete_relation_rule_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"either"] = @((BOOL)value->either);
    items[@"name"] = TTNativeString(value->name);
    items[@"reads"] = TTNativeString(value->reads);
    if (value->single.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"single"] = NSNull.null;
    else if (value->single.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"single"] = @((BOOL)value->single.value);
    items[@"source"] = TTNativeString(value->source);
    items[@"target"] = TTNativeString(value->target);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_relate_fields_v1(const struct thinkthen_complete_relate_fields_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"name"] = TTNativeString(value->name);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_position_v1(const struct thinkthen_complete_position_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"file"] = NSNull.null;
    else if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"file"] = TTNativeString(value->file.value);
    if (value->first.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"first"] = NSNull.null;
    else if (value->first.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"first"] = @(value->first.value);
    if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"images"] = NSNull.null;
    else if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"images"] = TTNative_thinkthen_complete_string_list_v1(&(value->images.value));
    if (value->last.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"last"] = NSNull.null;
    else if (value->last.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"last"] = @(value->last.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_batch_setting_presence_v1(const struct thinkthen_complete_batch_setting_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_batch_setting_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_batch_warning_presence_v1(const struct thinkthen_complete_batch_warning_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_batch_warning_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_origin_presence_v1(const struct thinkthen_complete_origin_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_origin_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_profile_warning_presence_v1(const struct thinkthen_complete_profile_warning_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_profile_warning_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_meta_v1(const struct thinkthen_complete_meta_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->answered_by.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"answered_by"] = NSNull.null;
    else if (value->answered_by.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"answered_by"] = TTNativeString(value->answered_by.value);
    if (value->attempts.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"attempts"] = NSNull.null;
    else if (value->attempts.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"attempts"] = TTNative_thinkthen_complete_attempt_list_v1(&(value->attempts.value));
    if (value->batch_setting.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch_setting"] = NSNull.null;
    else if (value->batch_setting.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch_setting"] = TTNative_thinkthen_complete_batch_setting_v1(value->batch_setting.value);
    if (value->batch_warning.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch_warning"] = NSNull.null;
    else if (value->batch_warning.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch_warning"] = TTNative_thinkthen_complete_batch_warning_v1(value->batch_warning.value);
    items[@"cached"] = @((BOOL)value->cached);
    if (value->context_sha256.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"context_sha256"] = NSNull.null;
    else if (value->context_sha256.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"context_sha256"] = TTNativeString(value->context_sha256.value);
    items[@"failed_questions"] = @(value->failed_questions);
    items[@"model"] = TTNativeString(value->model);
    items[@"observations"] = TTNative_thinkthen_complete_observation_list_v1(&(value->observations));
    if (value->origin.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"origin"] = NSNull.null;
    else if (value->origin.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"origin"] = TTNative_thinkthen_complete_origin_v1(value->origin.value);
    if (value->profile_warning.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"profile_warning"] = NSNull.null;
    else if (value->profile_warning.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"profile_warning"] = TTNative_thinkthen_complete_profile_warning_v1(value->profile_warning.value);
    if (value->question_sha256.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"question_sha256"] = NSNull.null;
    else if (value->question_sha256.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"question_sha256"] = TTNativeString(value->question_sha256.value);
    items[@"question_sources"] = TTNative_thinkthen_complete_question_source_list_v1(&(value->question_sources));
    if (value->questions_sha256.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"questions_sha256"] = NSNull.null;
    else if (value->questions_sha256.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"questions_sha256"] = TTNativeString(value->questions_sha256.value);
    items[@"requests"] = TTNative_thinkthen_complete_string_list_v1(&(value->requests));
    items[@"requests_sent"] = @(value->requests_sent);
    items[@"tool"] = TTNativeString(value->tool);
    items[@"url"] = TTNativeString(value->url);
    if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"usage"] = NSNull.null;
    else if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"usage"] = TTNative_thinkthen_complete_usage_v1(value->usage.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_profile_warning_v1(const struct thinkthen_complete_profile_warning_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"running"] = TTNativeString(value->running);
    items[@"tuned_for"] = TTNativeString(value->tuned_for);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_batch_warning_v1(const struct thinkthen_complete_batch_warning_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"running"] = TTNative_thinkthen_complete_batch_setting_v1(value->running);
    items[@"tuned_for"] = TTNative_thinkthen_complete_batch_setting_v1(value->tuned_for);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_batch_setting_v1(const struct thinkthen_complete_batch_setting_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER_V1: return @(value->data.integer);
        case THINKTHEN_COMPLETE_BATCH_SETTING_STRING_V1: return TTNativeString(value->data.string);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_relation_member_list_v1(const struct thinkthen_complete_relation_member_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_relation_member_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_answers_v1(const struct thinkthen_complete_answers_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"questions"] = TTNative_thinkthen_complete_relation_member_list_v1(&(value->questions));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_relation_member_v1(const struct thinkthen_complete_relation_member_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID_V1: return TTNative_thinkthen_complete_relation_member_answer_id_v1(value->data.answer_id);
        case THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID_V1: return TTNative_thinkthen_complete_relation_member_failure_id_v1(value->data.failure_id);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_related_entity_presence_v1(const struct thinkthen_complete_related_entity_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_related_entity_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_relation_member_failure_id_v1(const struct thinkthen_complete_relation_member_failure_id_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"direction"] = TTNative_thinkthen_complete_relation_direction_v1(value->direction);
    items[@"method"] = TTNative_thinkthen_complete_relation_method_v1(value->method);
    items[@"observations"] = TTNative_thinkthen_complete_observation_list_v1(&(value->observations));
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    items[@"question_sources"] = TTNative_thinkthen_complete_question_source_list_v1(&(value->question_sources));
    items[@"reads"] = TTNativeString(value->reads);
    items[@"relation"] = TTNativeString(value->relation);
    items[@"request"] = TTNativeString(value->request);
    items[@"source"] = TTNative_thinkthen_complete_related_entity_v1(value->source);
    if (value->target.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"target"] = NSNull.null;
    else if (value->target.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"target"] = TTNative_thinkthen_complete_related_entity_v1(value->target.value);
    items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold);
    if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"usage"] = NSNull.null;
    else if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"usage"] = TTNative_thinkthen_complete_usage_v1(value->usage.value);
    items[@"failure"] = TTNative_thinkthen_complete_failure_v1(value->failure);
    items[@"failure_id"] = TTNative_thinkthen_complete_failure_id_v1(value->failure_id);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_related_entity_v1(const struct thinkthen_complete_related_entity_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"name"] = TTNativeString(value->name);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_relation_method_v1(const struct thinkthen_complete_relation_method_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO_V1: return @"yes_no";
        case THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE_V1: return @"choice";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_relation_direction_v1(const struct thinkthen_complete_relation_direction_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET_V1: return @"source_to_target";
        case THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER_V1: return @"either";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_relation_member_answer_id_v1(const struct thinkthen_complete_relation_member_answer_id_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"direction"] = TTNative_thinkthen_complete_relation_direction_v1(value->direction);
    items[@"method"] = TTNative_thinkthen_complete_relation_method_v1(value->method);
    items[@"observations"] = TTNative_thinkthen_complete_observation_list_v1(&(value->observations));
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    items[@"question_sources"] = TTNative_thinkthen_complete_question_source_list_v1(&(value->question_sources));
    items[@"reads"] = TTNativeString(value->reads);
    items[@"relation"] = TTNativeString(value->relation);
    items[@"request"] = TTNativeString(value->request);
    items[@"source"] = TTNative_thinkthen_complete_related_entity_v1(value->source);
    if (value->target.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"target"] = NSNull.null;
    else if (value->target.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"target"] = TTNative_thinkthen_complete_related_entity_v1(value->target.value);
    items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold);
    if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"usage"] = NSNull.null;
    else if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"usage"] = TTNative_thinkthen_complete_usage_v1(value->usage.value);
    items[@"accepted"] = @((BOOL)value->accepted);
    items[@"answer"] = TTNative_thinkthen_complete_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    items[@"probability"] = @(value->probability);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_answer_v1(const struct thinkthen_complete_answer_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_ANSWER_YES_NO_V1: return TTNative_thinkthen_complete_answer_yes_no_v1(value->data.yes_no);
        case THINKTHEN_COMPLETE_ANSWER_CHOICE_V1: return TTNative_thinkthen_complete_answer_choice_v1(value->data.choice);
        case THINKTHEN_COMPLETE_ANSWER_TAG_V1: return TTNative_thinkthen_complete_answer_tag_v1(value->data.tag);
        case THINKTHEN_COMPLETE_ANSWER_SCORE_V1: return TTNative_thinkthen_complete_answer_score_v1(value->data.score);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_number_map_entry_v1(const struct thinkthen_complete_number_map_entry_v1 *value) {
    if (!value) TTInvalidResult();
    return @(value->value);
}
id TTNative_thinkthen_complete_number_map_v1(const struct thinkthen_complete_number_map_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    for (size_t i = 0;
    i < value->len;
    i++) {
        items[TTNativeString(value->data[i].name)] = @(value->data[i].value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_answer_score_v1(const struct thinkthen_complete_answer_score_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->confidence.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"confidence"] = NSNull.null;
    else if (value->confidence.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"confidence"] = @(value->confidence.value);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"level"] = TTNativeString(value->level);
    items[@"probabilities"] = TTNative_thinkthen_complete_number_map_v1(&(value->probabilities));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_answer_tag_v1(const struct thinkthen_complete_answer_tag_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"probabilities"] = TTNative_thinkthen_complete_number_map_v1(&(value->probabilities));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_answer_choice_v1(const struct thinkthen_complete_answer_choice_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->confidence.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"confidence"] = NSNull.null;
    else if (value->confidence.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"confidence"] = @(value->confidence.value);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"pick"] = TTNativeString(value->pick);
    items[@"probabilities"] = TTNative_thinkthen_complete_number_map_v1(&(value->probabilities));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_answer_yes_no_v1(const struct thinkthen_complete_answer_yes_no_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"kind"] = TTNativeString(value->kind);
    items[@"probability"] = @(value->probability);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_list_v1(const struct thinkthen_complete_recognition_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_recognition_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_recognize_aggregate_v1(const struct thinkthen_complete_session_packet_recognize_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_recognition_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_v1(const struct thinkthen_complete_recognition_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_recognition_odds_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"file"] = NSNull.null;
    else if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"file"] = TTNativeString(value->file.value);
    if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"first_line"] = NSNull.null;
    else if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"first_line"] = @(value->first_line.value);
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"last_line"] = NSNull.null;
    else if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"last_line"] = @(value->last_line.value);
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    if (value->position.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"position"] = NSNull.null;
    else if (value->position.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"position"] = TTNative_thinkthen_complete_position_v1(value->position.value);
    items[@"question"] = TTNative_thinkthen_complete_recognize_question_document_v1(value->question);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    items[@"value"] = TTNative_thinkthen_complete_recognize_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognize_v1(const struct thinkthen_complete_recognize_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1: return TTNative_thinkthen_complete_recognize_entities_v1(value->data.entities);
        case THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS_V1: return TTNative_thinkthen_complete_recognize_mode_proposals_v1(value->data.mode_proposals);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_recognize_mode_proposals_v1(const struct thinkthen_complete_recognize_mode_proposals_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"mode"] = TTNative_thinkthen_complete_boundary_mode_v1(value->mode);
    items[@"proposals"] = TTNative_thinkthen_complete_boundary_proposal_list_v1(&(value->proposals));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_boundary_mode_v1(const struct thinkthen_complete_boundary_mode_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY_V1: return @"boundary_only";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_entity_edge_list_v1(const struct thinkthen_complete_entity_edge_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_entity_edge_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_entity_edge_list_presence_v1(const struct thinkthen_complete_entity_edge_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_entity_edge_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_recognize_entities_v1(const struct thinkthen_complete_recognize_entities_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"entities"] = TTNative_thinkthen_complete_entity_list_v1(&(value->entities));
    if (value->relations.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"relations"] = NSNull.null;
    else if (value->relations.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"relations"] = TTNative_thinkthen_complete_entity_edge_list_v1(&(value->relations.value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_entity_edge_v1(const struct thinkthen_complete_entity_edge_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->either.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"either"] = NSNull.null;
    else if (value->either.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"either"] = @((BOOL)value->either.value);
    items[@"probability"] = @(value->probability);
    items[@"relation"] = TTNativeString(value->relation);
    items[@"source"] = TTNative_thinkthen_complete_entity_v1(value->source);
    items[@"target"] = TTNative_thinkthen_complete_entity_v1(value->target);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_json_map_entry_v1(const struct thinkthen_complete_json_map_entry_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNative_thinkthen_complete_json_v1(value->value);
}
id TTNative_thinkthen_complete_json_map_v1(const struct thinkthen_complete_json_map_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    for (size_t i = 0;
    i < value->len;
    i++) {
        items[TTNativeString(value->data[i].name)] = TTNative_thinkthen_complete_json_v1(value->data[i].value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_mode_presence_v1(const struct thinkthen_complete_recognition_mode_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_recognition_mode_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_relation_rule_list_presence_v1(const struct thinkthen_complete_relation_rule_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_relation_rule_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_recognition_stage_context_presence_v1(const struct thinkthen_complete_recognition_stage_context_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_recognition_stage_context_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_recognize_question_document_v1(const struct thinkthen_complete_recognize_question_document_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch"] = NSNull.null;
    else if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch"] = TTNative_thinkthen_complete_batch_v1(value->batch.value);
    if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"context_schema"] = NSNull.null;
    else if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"context_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->context_schema.value);
    if (value->entity_definition.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"entity_definition"] = NSNull.null;
    else if (value->entity_definition.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"entity_definition"] = TTNative_thinkthen_complete_json_v1(value->entity_definition.value);
    if (value->instructions.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"instructions"] = NSNull.null;
    else if (value->instructions.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"instructions"] = TTNative_thinkthen_complete_json_v1(value->instructions.value);
    if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"item_schema"] = NSNull.null;
    else if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"item_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->item_schema.value);
    items[@"kinds"] = TTNative_thinkthen_complete_json_map_v1(&(value->kinds));
    if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"label_details"] = NSNull.null;
    else if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"label_details"] = TTNative_thinkthen_complete_label_list_v1(&(value->label_details.value));
    if (value->mode.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"mode"] = NSNull.null;
    else if (value->mode.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"mode"] = TTNative_thinkthen_complete_recognition_mode_v1(value->mode.value);
    if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"model"] = NSNull.null;
    else if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"model"] = TTNativeString(value->model.value);
    if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"name"] = NSNull.null;
    else if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"name"] = TTNative_thinkthen_complete_question_name_v1(value->name.value);
    if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"on"] = NSNull.null;
    else if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"on"] = TTNative_thinkthen_complete_string_list_v1(&(value->on.value));
    if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"profile"] = NSNull.null;
    else if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"profile"] = TTNativeString(value->profile.value);
    if (value->relation_threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"relation_threshold"] = NSNull.null;
    else if (value->relation_threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"relation_threshold"] = TTNative_thinkthen_complete_threshold_v1(value->relation_threshold.value);
    if (value->relations.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"relations"] = NSNull.null;
    else if (value->relations.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"relations"] = TTNative_thinkthen_complete_relation_rule_list_v1(&(value->relations.value));
    if (value->snippet_pieces.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"snippet_pieces"] = NSNull.null;
    else if (value->snippet_pieces.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"snippet_pieces"] = @(value->snippet_pieces.value);
    if (value->stage_context.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"stage_context"] = NSNull.null;
    else if (value->stage_context.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"stage_context"] = TTNative_thinkthen_complete_recognition_stage_context_v1(value->stage_context.value);
    items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold);
    items[@"verb"] = TTNative_thinkthen_complete_verb_v1(value->verb);
    if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"wording_version"] = NSNull.null;
    else if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"wording_version"] = TTNative_thinkthen_complete_wording_version_v1(value->wording_version.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_verb_v1(const struct thinkthen_complete_verb_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_VERB_RECOGNIZE_V1: return @"recognize";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_recognition_stage_context_v1(const struct thinkthen_complete_recognition_stage_context_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->boundary.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"boundary"] = NSNull.null;
    else if (value->boundary.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"boundary"] = TTNativeString(value->boundary.value);
    if (value->kind_edge.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"kind_edge"] = NSNull.null;
    else if (value->kind_edge.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"kind_edge"] = TTNativeString(value->kind_edge.value);
    if (value->relation.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"relation"] = NSNull.null;
    else if (value->relation.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"relation"] = TTNativeString(value->relation.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_odds_v1(const struct thinkthen_complete_recognition_odds_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER_V1: return TTNative_thinkthen_complete_recognition_odds_recognize_answer_v1(value->data.recognize_answer);
        case THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS_V1: return TTNative_thinkthen_complete_recognition_odds_boundary_odds_v1(value->data.boundary_odds);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_piece_odds_list_v1(const struct thinkthen_complete_piece_odds_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_piece_odds_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_odds_boundary_odds_v1(const struct thinkthen_complete_recognition_odds_boundary_odds_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"pieces"] = TTNative_thinkthen_complete_piece_odds_list_v1(&(value->pieces));
    items[@"proposals"] = TTNative_thinkthen_complete_boundary_proposal_list_v1(&(value->proposals));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_piece_odds_v1(const struct thinkthen_complete_piece_odds_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"end"] = @(value->end);
    items[@"start"] = @(value->start);
    items[@"tags"] = TTNative_thinkthen_complete_number_map_v1(&(value->tags));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_name_odds_list_v1(const struct thinkthen_complete_name_odds_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_name_odds_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_pair_odds_list_v1(const struct thinkthen_complete_pair_odds_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_pair_odds_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_proposal_list_v1(const struct thinkthen_complete_recognition_proposal_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_recognition_proposal_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_recognition_odds_recognize_answer_v1(const struct thinkthen_complete_recognition_odds_recognize_answer_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"names"] = TTNative_thinkthen_complete_name_odds_list_v1(&(value->names));
    items[@"pairs"] = TTNative_thinkthen_complete_pair_odds_list_v1(&(value->pairs));
    items[@"pieces"] = TTNative_thinkthen_complete_piece_odds_list_v1(&(value->pieces));
    items[@"proposals"] = TTNative_thinkthen_complete_recognition_proposal_list_v1(&(value->proposals));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_place_presence_v1(const struct thinkthen_complete_place_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_place_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_recognition_proposal_v1(const struct thinkthen_complete_recognition_proposal_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"end"] = @(value->end);
    items[@"kept"] = @((BOOL)value->kept);
    if (value->kind.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"kind"] = NSNull.null;
    else if (value->kind.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"kind"] = TTNativeString(value->kind.value);
    if (value->selected.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"selected"] = NSNull.null;
    else if (value->selected.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"selected"] = TTNative_thinkthen_complete_place_v1(value->selected.value);
    items[@"span_probability"] = @(value->span_probability);
    items[@"start"] = @(value->start);
    if (value->strength.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"strength"] = NSNull.null;
    else if (value->strength.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"strength"] = @(value->strength.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_place_v1(const struct thinkthen_complete_place_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"end"] = @(value->end);
    items[@"start"] = @(value->start);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_pair_odds_v1(const struct thinkthen_complete_pair_odds_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"probability"] = @(value->probability);
    items[@"relation"] = TTNativeString(value->relation);
    items[@"source"] = TTNative_thinkthen_complete_place_v1(value->source);
    items[@"target"] = TTNative_thinkthen_complete_place_v1(value->target);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_number_map_presence_v1(const struct thinkthen_complete_number_map_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_number_map_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_name_odds_v1(const struct thinkthen_complete_name_odds_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->edges.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"edges"] = NSNull.null;
    else if (value->edges.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"edges"] = TTNative_thinkthen_complete_number_map_v1(&(value->edges.value));
    items[@"end"] = @(value->end);
    if (value->kinds.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"kinds"] = NSNull.null;
    else if (value->kinds.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"kinds"] = TTNative_thinkthen_complete_number_map_v1(&(value->kinds.value));
    items[@"start"] = @(value->start);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_list_v1(const struct thinkthen_complete_annotation_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_annotation_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_annotate_aggregate_v1(const struct thinkthen_complete_session_packet_annotate_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_annotation_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_member_map_entry_v1(const struct thinkthen_complete_annotation_member_map_entry_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNative_thinkthen_complete_annotation_member_v1(value->value);
}
id TTNative_thinkthen_complete_annotation_member_map_v1(const struct thinkthen_complete_annotation_member_map_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    for (size_t i = 0;
    i < value->len;
    i++) {
        items[TTNativeString(value->data[i].name)] = TTNative_thinkthen_complete_annotation_member_v1(value->data[i].value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_v1(const struct thinkthen_complete_annotation_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    items[@"answers"] = TTNative_thinkthen_complete_annotation_member_map_v1(&(value->answers));
    if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"file"] = NSNull.null;
    else if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"file"] = TTNativeString(value->file.value);
    if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"first_line"] = NSNull.null;
    else if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"first_line"] = @(value->first_line.value);
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"last_line"] = NSNull.null;
    else if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"last_line"] = @(value->last_line.value);
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    if (value->position.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"position"] = NSNull.null;
    else if (value->position.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"position"] = TTNative_thinkthen_complete_position_v1(value->position.value);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    items[@"value"] = TTNative_thinkthen_complete_annotated_row_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotated_field_map_entry_v1(const struct thinkthen_complete_annotated_field_map_entry_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNative_thinkthen_complete_annotated_field_v1(value->value);
}
id TTNative_thinkthen_complete_annotated_field_map_v1(const struct thinkthen_complete_annotated_field_map_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    for (size_t i = 0;
    i < value->len;
    i++) {
        items[TTNativeString(value->data[i].name)] = TTNative_thinkthen_complete_annotated_field_v1(value->data[i].value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotated_row_v1(const struct thinkthen_complete_annotated_row_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNative_thinkthen_complete_annotated_field_map_v1(&(value->value));
}
id TTNative_thinkthen_complete_annotated_field_v1(const struct thinkthen_complete_annotated_field_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN_V1: return @((BOOL)value->data.boolean);
        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1: return NSNull.null;
        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1: return TTNativeString(value->data.string);
        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1: return TTNative_thinkthen_complete_string_list_v1(&(value->data.array));
        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER_V1: return @(value->data.number);
        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT_V1: return TTNative_thinkthen_complete_failed_v1(value->data.object);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_failed_v1(const struct thinkthen_complete_failed_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"failed"] = TTNative_thinkthen_complete_failure_v1(value->failed);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_member_v1(const struct thinkthen_complete_annotation_member_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID_V1: return TTNative_thinkthen_complete_annotation_member_answer_id_v1(value->data.answer_id);
        case THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID_V1: return TTNative_thinkthen_complete_annotation_member_failure_id_v1(value->data.failure_id);
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_annotation_member_failure_id_v1(const struct thinkthen_complete_annotation_member_failure_id_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"failure"] = TTNative_thinkthen_complete_failure_v1(value->failure);
    items[@"failure_id"] = TTNative_thinkthen_complete_failure_id_v1(value->failure_id);
    items[@"observations"] = TTNative_thinkthen_complete_observation_list_v1(&(value->observations));
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    items[@"question_sources"] = TTNative_thinkthen_complete_question_source_list_v1(&(value->question_sources));
    items[@"request"] = TTNativeString(value->request);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold.value);
    if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"usage"] = NSNull.null;
    else if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"usage"] = TTNative_thinkthen_complete_usage_v1(value->usage.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_annotation_member_answer_id_v1(const struct thinkthen_complete_annotation_member_answer_id_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    items[@"observations"] = TTNative_thinkthen_complete_observation_list_v1(&(value->observations));
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    items[@"question_sources"] = TTNative_thinkthen_complete_question_source_list_v1(&(value->question_sources));
    items[@"request"] = TTNativeString(value->request);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold.value);
    if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"usage"] = NSNull.null;
    else if (value->usage.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"usage"] = TTNative_thinkthen_complete_usage_v1(value->usage.value);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = TTNative_thinkthen_complete_value_v1(value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_find_aggregate_v1(const struct thinkthen_complete_session_packet_find_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_find_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_find_candidate_list_v1(const struct thinkthen_complete_find_candidate_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_find_candidate_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_find_candidate_list_presence_v1(const struct thinkthen_complete_find_candidate_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_find_candidate_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_find_v1(const struct thinkthen_complete_find_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_find_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->candidates.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"candidates"] = NSNull.null;
    else if (value->candidates.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"candidates"] = TTNative_thinkthen_complete_find_candidate_list_v1(&(value->candidates.value));
    if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"file"] = NSNull.null;
    else if (value->file.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"file"] = TTNativeString(value->file.value);
    if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"first_line"] = NSNull.null;
    else if (value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"first_line"] = @(value->first_line.value);
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"last_line"] = NSNull.null;
    else if (value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"last_line"] = @(value->last_line.value);
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    if (value->position.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"position"] = NSNull.null;
    else if (value->position.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"position"] = TTNative_thinkthen_complete_position_v1(value->position.value);
    items[@"question"] = TTNative_thinkthen_complete_find_question_document_v1(value->question);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_json_v1(value->threshold.value);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = TTNative_thinkthen_complete_json_v1(value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_find_question_document_v1(const struct thinkthen_complete_find_question_document_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"batch"] = NSNull.null;
    else if (value->batch.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"batch"] = TTNative_thinkthen_complete_batch_v1(value->batch.value);
    if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"context_schema"] = NSNull.null;
    else if (value->context_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"context_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->context_schema.value);
    if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"item_schema"] = NSNull.null;
    else if (value->item_schema.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"item_schema"] = TTNative_thinkthen_complete_input_declaration_v1(value->item_schema.value);
    if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"label_details"] = NSNull.null;
    else if (value->label_details.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"label_details"] = TTNative_thinkthen_complete_label_list_v1(&(value->label_details.value));
    if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"model"] = NSNull.null;
    else if (value->model.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"model"] = TTNativeString(value->model.value);
    if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"name"] = NSNull.null;
    else if (value->name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"name"] = TTNative_thinkthen_complete_question_name_v1(value->name.value);
    items[@"none"] = @((BOOL)value->none);
    if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"on"] = NSNull.null;
    else if (value->on.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"on"] = TTNative_thinkthen_complete_string_list_v1(&(value->on.value));
    if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"profile"] = NSNull.null;
    else if (value->profile.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"profile"] = TTNativeString(value->profile.value);
    if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"text"] = NSNull.null;
    else if (value->text.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"text"] = TTNative_thinkthen_complete_json_v1(value->text.value);
    items[@"verb"] = TTNativeString(value->verb);
    if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"wording_version"] = NSNull.null;
    else if (value->wording_version.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"wording_version"] = TTNative_thinkthen_complete_wording_version_v1(value->wording_version.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_find_candidate_v1(const struct thinkthen_complete_find_candidate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    items[@"probability"] = @(value->probability);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_find_answer_v1(const struct thinkthen_complete_find_answer_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    if (value->confidence.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"confidence"] = NSNull.null;
    else if (value->confidence.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"confidence"] = @(value->confidence.value);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"pick"] = TTNativeString(value->pick);
    items[@"probabilities"] = TTNative_thinkthen_complete_number_map_v1(&(value->probabilities));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_atomic_non_zero_usize_list_v1(const struct thinkthen_complete_atomic_non_zero_usize_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_atomic_non_zero_usize_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_rank_aggregate_v1(const struct thinkthen_complete_session_packet_rank_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_non_zero_usize_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_image_list_v1(const struct thinkthen_complete_image_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_image_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_image_list_presence_v1(const struct thinkthen_complete_image_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_image_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_rank_member_list_v1(const struct thinkthen_complete_rank_member_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_rank_member_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_rank_member_list_presence_v1(const struct thinkthen_complete_rank_member_list_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_rank_member_list_v1(&(value->value)) : NSNull.null;
}
id TTNative_thinkthen_complete_atomic_non_zero_usize_v1(const struct thinkthen_complete_atomic_non_zero_usize_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"images"] = NSNull.null;
    else if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"images"] = TTNative_thinkthen_complete_image_list_v1(&(value->images.value));
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"members"] = NSNull.null;
    else if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"members"] = TTNative_thinkthen_complete_rank_member_list_v1(&(value->members.value));
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"question_name"] = NSNull.null;
    else if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"question_name"] = TTNativeString(value->question_name.value);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold.value);
    items[@"value"] = @(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_rank_member_v1(const struct thinkthen_complete_rank_member_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"name"] = TTNativeString(value->name);
    items[@"result"] = TTNative_thinkthen_complete_rank_member_result_v1(value->result);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_rank_member_result_v1(const struct thinkthen_complete_rank_member_result_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"images"] = NSNull.null;
    else if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"images"] = TTNative_thinkthen_complete_image_list_v1(&(value->images.value));
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_json_v1(value->threshold.value);
    items[@"value"] = @(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_image_v1(const struct thinkthen_complete_image_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"base64"] = TTNativeString(value->base64);
    items[@"height"] = @(value->height);
    items[@"media"] = TTNative_thinkthen_complete_image_media_v1(value->media);
    items[@"width"] = @(value->width);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_image_media_v1(const struct thinkthen_complete_image_media_v1 *value) {
    if (!value) TTInvalidResult();
    switch(value->kind) {
        case THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG_V1: return @"image/jpeg";
        case THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG_V1: return @"image/png";
        default: TTInvalidResult();
    }
}
id TTNative_thinkthen_complete_atomic_boolean_list_v1(const struct thinkthen_complete_atomic_boolean_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_atomic_boolean_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_filter_aggregate_v1(const struct thinkthen_complete_session_packet_filter_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_boolean_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_atomic_boolean_v1(const struct thinkthen_complete_atomic_boolean_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"images"] = NSNull.null;
    else if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"images"] = TTNative_thinkthen_complete_image_list_v1(&(value->images.value));
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"members"] = NSNull.null;
    else if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"members"] = TTNative_thinkthen_complete_rank_member_list_v1(&(value->members.value));
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"question_name"] = NSNull.null;
    else if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"question_name"] = TTNativeString(value->question_name.value);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold.value);
    items[@"value"] = @((BOOL)value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_atomic_double_list_v1(const struct thinkthen_complete_atomic_double_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_atomic_double_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_score_aggregate_v1(const struct thinkthen_complete_session_packet_score_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_double_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_atomic_double_v1(const struct thinkthen_complete_atomic_double_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"images"] = NSNull.null;
    else if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"images"] = TTNative_thinkthen_complete_image_list_v1(&(value->images.value));
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"members"] = NSNull.null;
    else if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"members"] = TTNative_thinkthen_complete_rank_member_list_v1(&(value->members.value));
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"question_name"] = NSNull.null;
    else if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"question_name"] = TTNativeString(value->question_name.value);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold.value);
    items[@"value"] = @(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_atomic_array_of_string_list_v1(const struct thinkthen_complete_atomic_array_of_string_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_atomic_array_of_string_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_tag_aggregate_v1(const struct thinkthen_complete_session_packet_tag_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_array_of_string_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_atomic_array_of_string_v1(const struct thinkthen_complete_atomic_array_of_string_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"images"] = NSNull.null;
    else if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"images"] = TTNative_thinkthen_complete_image_list_v1(&(value->images.value));
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"members"] = NSNull.null;
    else if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"members"] = TTNative_thinkthen_complete_rank_member_list_v1(&(value->members.value));
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"question_name"] = NSNull.null;
    else if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"question_name"] = TTNativeString(value->question_name.value);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold.value);
    items[@"value"] = TTNative_thinkthen_complete_string_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_atomic_nullable_string_list_v1(const struct thinkthen_complete_atomic_nullable_string_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_atomic_nullable_string_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_choose_aggregate_v1(const struct thinkthen_complete_session_packet_choose_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_nullable_string_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_atomic_nullable_string_v1(const struct thinkthen_complete_atomic_nullable_string_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"images"] = NSNull.null;
    else if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"images"] = TTNative_thinkthen_complete_image_list_v1(&(value->images.value));
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"members"] = NSNull.null;
    else if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"members"] = TTNative_thinkthen_complete_rank_member_list_v1(&(value->members.value));
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"question_name"] = NSNull.null;
    else if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"question_name"] = TTNativeString(value->question_name.value);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold.value);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = TTNativeString(value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_atomic_decide_value_list_v1(const struct thinkthen_complete_atomic_decide_value_list_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableArray *items = [NSMutableArray array];
    for (size_t i = 0;
    i < value->len;
    i++) {
        [items addObject:TTNative_thinkthen_complete_atomic_decide_value_v1(value->data[i])];
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_decide_aggregate_v1(const struct thinkthen_complete_session_packet_decide_aggregate_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_decide_value_list_v1(&(value->value));
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_decide_value_presence_v1(const struct thinkthen_complete_decide_value_presence_v1 *value) {
    if (!value) TTInvalidResult();
    return value->presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? TTNative_thinkthen_complete_decide_value_v1(value->value) : NSNull.null;
}
id TTNative_thinkthen_complete_atomic_decide_value_v1(const struct thinkthen_complete_atomic_decide_value_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"answer"] = TTNative_thinkthen_complete_answer_v1(value->answer);
    items[@"answer_id"] = TTNative_thinkthen_complete_answer_id_v1(value->answer_id);
    if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"images"] = NSNull.null;
    else if (value->images.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"images"] = TTNative_thinkthen_complete_image_list_v1(&(value->images.value));
    if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"index"] = NSNull.null;
    else if (value->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"index"] = @(value->index.value);
    if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"input"] = NSNull.null;
    else if (value->input.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"input"] = TTNative_thinkthen_complete_json_v1(value->input.value);
    if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"members"] = NSNull.null;
    else if (value->members.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"members"] = TTNative_thinkthen_complete_rank_member_list_v1(&(value->members.value));
    items[@"meta"] = TTNative_thinkthen_complete_meta_v1(value->meta);
    items[@"question"] = TTNative_thinkthen_complete_question_document_v1(value->question);
    if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"question_name"] = NSNull.null;
    else if (value->question_name.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"question_name"] = TTNativeString(value->question_name.value);
    items[@"schema"] = TTNative_thinkthen_complete_version_v1(value->schema);
    if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"source"] = NSNull.null;
    else if (value->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"source"] = TTNative_thinkthen_complete_physical_source_v1(value->source.value);
    if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"threshold"] = NSNull.null;
    else if (value->threshold.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"threshold"] = TTNative_thinkthen_complete_threshold_v1(value->threshold.value);
    if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items[@"value"] = NSNull.null;
    else if (value->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items[@"value"] = TTNative_thinkthen_complete_decide_value_v1(value->value.value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_decide_value_v1(const struct thinkthen_complete_decide_value_v1 *value) {
    if (!value) TTInvalidResult();
    return TTNative_thinkthen_complete_json_v1(value->value);
}
id TTNative_thinkthen_complete_session_packet_annotate_row_v1(const struct thinkthen_complete_session_packet_annotate_row_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_annotation_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_filter_row_v1(const struct thinkthen_complete_session_packet_filter_row_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_boolean_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_score_row_v1(const struct thinkthen_complete_session_packet_score_row_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_double_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_tag_row_v1(const struct thinkthen_complete_session_packet_tag_row_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_array_of_string_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_choose_row_v1(const struct thinkthen_complete_session_packet_choose_row_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_nullable_string_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
id TTNative_thinkthen_complete_session_packet_decide_row_v1(const struct thinkthen_complete_session_packet_decide_row_v1 *value) {
    if (!value) TTInvalidResult();
    NSMutableDictionary *items = [NSMutableDictionary dictionary];
    items[@"function"] = TTNativeString(value->function);
    items[@"kind"] = TTNativeString(value->kind);
    items[@"value"] = TTNative_thinkthen_complete_atomic_decide_value_v1(value->value);
    for (size_t i = 0;
    i < value->extensions.len;
    i++) {
        const struct thinkthen_complete_extension_v1 *item = &value->extensions.data[i];
        items[TTNativeString(item->name)] = TTNative_thinkthen_complete_json_v1(item->value);
    }
    return [items copy];
}
#endif
