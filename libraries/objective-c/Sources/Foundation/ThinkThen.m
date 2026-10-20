#import "ThinkThenFoundation.h"
#import <thinkthen.h>
#import "TTResultPrivate.h"
extern id TTNative_thinkthen_complete_session_packet(const struct thinkthen_complete_session_packet *value);
extern id TTNative_thinkthen_complete_immediate_facts(const struct thinkthen_complete_immediate_facts *value);
extern id TTNative_thinkthen_complete_json(const struct thinkthen_complete_json *value);
NSErrorDomain const TTErrorDomain = @"io.github.botassembly.thinkthen";
NSString * const TTFailureDetailsKey = @"ThinkThenFailureDetails";
TTByteImage *TTImageBytes(NSData *bytes, NSString *media) {
    TTByteImage *image = [TTByteImage new];
    image.bytes = [bytes base64EncodedStringWithOptions:0]; image.media = media; return image;
}
@interface TTFailureError ()
@property(nonatomic) BOOL retryable;
@property(nonatomic, strong, nullable) NSNumber *position;
@property(nonatomic, strong, nullable) TTCallError *failure;
@property(nonatomic, strong, nullable) TTImmediateFacts *facts;
@end
@implementation TTFailureError
- (TTErrorCode)kind { return (TTErrorCode)self.code; }
@end
TTFailureError *TTFailureForCall(TTCallError *failure, id call) {
    TTFailureError *error = [[TTFailureError alloc] initWithDomain:TTErrorDomain code:TTNativeErrorCode(failure.error.value.kind.value) userInfo:@{NSLocalizedDescriptionKey:failure.error.value.message.value, TTFailureDetailsKey:failure, @"ThinkThenCall":call}];
    error.retryable = failure.error.value.retryable.value.boolValue; error.failure = failure;
    if (failure.facts.state == TTPresenceValue) error.facts = [TTImmediateFacts read:failure.facts.value.rawFields];
    return error;
}
static NSError *TTNativeFailure(const struct thinkthen_error_details *view, int code, NSString *fallback) {
    NSString *message = view ? [[NSString alloc] initWithBytes:view->message.data length:view->message.len encoding:NSUTF8StringEncoding] : fallback;
    TTFailureError *error = [[TTFailureError alloc] initWithDomain:TTErrorDomain code:code userInfo:@{NSLocalizedDescriptionKey:message ?: fallback}];
    error.retryable = view && view->retryable;
    if (view && view->facts) error.facts = [TTImmediateFacts read:TTNative_thinkthen_complete_immediate_facts(view->facts)];
    return error;
}
static NSError *TTImmediate(int code) {
    return TTNativeFailure(thinkthen_session_error_view(), code, @"Native session failure");
}
static BOOL TTCheck(int code, NSError **error) { if (!code) return YES; if (error) *error = TTImmediate(code); return NO; }
static NSData *TTEncode(id object, NSError **error) {
    // Foundation owns representation checks; native Request owns admission.
    @try {
        NSError *representation = nil;
        NSData *data = TTRequestJSONData(object, &representation);
        if (!data && error) *error = TTNativeFailure(NULL, TTErrorUsage, @"Input has an unsupported Foundation representation");
        return data;
    }
    @catch (NSException *exception) {
        (void)exception;
        if (error) *error = TTNativeFailure(NULL, TTErrorUsage, @"Input has an unsupported Foundation representation");
        return nil;
    }
}
static id TTJSON(struct thinkthen_complete_utf8 bytes) {
    NSData *data = [NSData dataWithBytes:bytes.data length:bytes.len];
    return [NSJSONSerialization JSONObjectWithData:data options:NSJSONReadingFragmentsAllowed error:NULL];
}
static TTFailureError *TTOwnedFailure(struct thinkthen_call_error *owner) {
    struct thinkthen_complete_utf8 message = thinkthen_call_error_message(owner);
    NSString *text = [[NSString alloc] initWithBytes:message.data length:message.len encoding:NSUTF8StringEncoding];
    TTFailureError *failure = [[TTFailureError alloc] initWithDomain:TTErrorDomain code:thinkthen_call_error_code(owner) userInfo:@{NSLocalizedDescriptionKey:text}];
    failure.retryable = thinkthen_call_error_retryable(owner);
    const size_t *position = thinkthen_call_error_position(owner);
    if (position) failure.position = @(*position);
    struct thinkthen_complete_utf8 facts = thinkthen_call_error_facts_json(owner);
    if (facts.len) failure.facts = [TTImmediateFacts read:TTJSON(facts)];
    return failure;
}
@interface TTTask () {
    struct thinkthen_call_cancel *_cancel;
    NSLock *_lock;
}
- (const struct thinkthen_call_cancel *)token;
@end
@implementation TTTask
- (instancetype)init {
    if ((self = [super init])) { _cancel = thinkthen_call_cancel_new(); _lock = [NSLock new]; }
    return self;
}
- (const struct thinkthen_call_cancel *)token { return _cancel; }
- (void)cancel { [_lock lock]; thinkthen_call_cancel_fire(_cancel); [_lock unlock]; }
- (void)dealloc { thinkthen_call_cancel_free(_cancel); }
@end
@interface TTCallFeed : NSObject
@property(nonatomic, copy) TTFeed feed;
@property(nonatomic, strong) NSData *bytes;
@property(nonatomic, strong) TTRequestObject *failure;
@property(nonatomic, strong) NSException *exception;
@end
@implementation TTCallFeed
@end
static uint32_t TTPull(void *context, struct thinkthen_complete_utf8 *item) {
    TTCallFeed *state = (__bridge TTCallFeed *)context;
    @try {
        TTRequestObject *failure = nil;
        TTDescriptor *descriptor = state.feed(&failure);
        state.failure = failure;
        if (failure) return THINKTHEN_CALL_PRODUCER_FAILURE;
        if (!descriptor) return THINKTHEN_CALL_PRODUCER_END;
        state.bytes = TTEncode(descriptor.requestFields[@"item"], NULL);
        if (!state.bytes) return THINKTHEN_CALL_PRODUCER_FAILURE;
        *item = (struct thinkthen_complete_utf8){state.bytes.bytes, state.bytes.length};
        return THINKTHEN_CALL_PRODUCER_ITEM;
    } @catch (NSException *exception) { state.exception = exception; return THINKTHEN_CALL_PRODUCER_FAILURE; }
}
@interface TTUsagePersistenceStatus ()
- (instancetype)initWithState:(TTUsagePersistenceState)state advice:(nullable NSString *)advice;
@end
@implementation TTUsagePersistenceStatus
- (instancetype)initWithState:(TTUsagePersistenceState)state advice:(NSString *)advice {
    if ((self = [super init])) { _state = state; _advice = [advice copy]; }
    return self;
}
@end
@interface TTFoundationClient () { struct thinkthen_engine *_native; }
@end
@implementation TTFoundationClient
- (instancetype)initWithSettings:(TTSettings *)settings error:(NSError **)error {
    if ((self = [super init])) {
        NSData *bytes = settings ? TTEncode(settings, error) : nil;
        if (settings && !bytes) return nil;
        NSString *text = bytes ? [[NSString alloc] initWithData:bytes encoding:NSUTF8StringEncoding] : nil;
        struct thinkthen_call_error *construction = NULL;
        _native = settings ? thinkthen_engine_new_with(text.UTF8String, &construction) : thinkthen_engine_new(&construction);
        if (!_native) {
            @try { if (error) *error = TTOwnedFailure(construction); }
            @finally { thinkthen_call_error_free(construction); }
            return nil;
        }
    }
    return self;
}
- (void)dealloc { thinkthen_engine_free(_native); }
- (TTUsagePersistenceStatus *)usageStatus:(BOOL)finish error:(NSError **)error {
    struct thinkthen_complete_usage_persistence state;
    struct thinkthen_complete_utf8 advice;
    int code = finish ? thinkthen_engine_finish_usage_status(_native, &state, &advice)
                      : thinkthen_engine_usage_persistence(_native, &state, &advice);
    if (!TTCheck(code, error)) return nil;
    NSString *copy = advice.data ? [[NSString alloc] initWithBytes:advice.data length:advice.len encoding:NSUTF8StringEncoding] : nil;
    return [[TTUsagePersistenceStatus alloc] initWithState:(TTUsagePersistenceState)state.kind advice:copy];
}
- (TTUsagePersistenceStatus *)usagePersistence:(NSError **)error { return [self usageStatus:NO error:error]; }
- (TTUsagePersistenceStatus *)finishUsageStatus:(NSError **)error { return [self usageStatus:YES error:error]; }
- (TTPlan *)plan:(TTRequest *)request error:(NSError **)error {
    NSData *data = TTEncode(request, error); if (!data) return nil;
    struct thinkthen_value *owner = NULL;
    if (!TTCheck(thinkthen_request_plan_value(_native, data.bytes, data.length, &owner), error)) return nil;
    @try {
        const struct thinkthen_complete_json *view = NULL;
        if (!TTCheck(thinkthen_value_view(owner, &view), error)) return nil;
        return [TTPlan read:TTNative_thinkthen_complete_json(view)];
    } @finally { thinkthen_value_free(owner); }
}
- (TTCall *)run:(NSString *)verb questionBytes:(NSData *)questionBytes inputBytes:(NSData *)inputBytes optionBytes:(NSData *)optionBytes feed:(TTFeed)feed token:(const struct thinkthen_call_cancel *)token error:(NSError **)error {
    NSData *functionBytes = [verb dataUsingEncoding:NSUTF8StringEncoding];
    struct thinkthen_named_request request = {
        {functionBytes.bytes, functionBytes.length}, {questionBytes.bytes, questionBytes.length},
        {inputBytes.bytes, inputBytes.length}, {optionBytes.bytes, optionBytes.length}};
    TTCallFeed *state = [TTCallFeed new]; state.feed = feed;
    struct thinkthen_call_result *result = NULL;
    struct thinkthen_call_error *failure = NULL;
    int code = thinkthen_run_call(_native, &request, feed ? TTPull : NULL, (__bridge void *)state, token, &result, &failure);
    @try {
        if (code) {
            TTFailureError *converted = TTOwnedFailure(failure);
            if (state.failure || state.exception) {
                NSMutableDictionary *details = [converted.userInfo mutableCopy];
                details[TTFailureDetailsKey] = state.exception ?: state.failure;
                TTFailureError *host = [[TTFailureError alloc] initWithDomain:converted.domain code:converted.code userInfo:details];
                host.retryable = converted.retryable; host.facts = converted.facts; host.position = converted.position;
                converted = host;
            }
            if (error) *error = converted;
            return nil;
        }
        return [TTCall read:TTJSON(thinkthen_call_result_json(result))];
    } @finally { thinkthen_call_result_free(result); thinkthen_call_error_free(failure); }
}
- (TTTask *)start:(NSString *)verb question:(TTQuestion *)question input:(TTInput *)input options:(TTControls *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    // Representation conversion may fail before dispatch; Rust owns semantic admission.
    NSData *questionBytes = TTEncode(question, error); if (!questionBytes) return nil;
    NSData *inputBytes = TTEncode(input, error); if (!inputBytes) return nil;
    NSData *optionBytes = TTEncode(options ?: @{}, error); if (!optionBytes) return nil;
    TTTask *task = [[TTTask alloc] init];
    dispatch_async(dispatch_get_global_queue(QOS_CLASS_DEFAULT, 0), ^{
        NSError *failure = nil;
        TTCall *call = [self run:verb questionBytes:questionBytes inputBytes:inputBytes optionBytes:optionBytes feed:feed token:task.token error:&failure];
        completion(call, failure);
    });
    return task;
}
- (TTCall *)complete:(NSString *)verb question:(TTQuestion *)question input:(TTInput *)input options:(TTControls *)options feed:(TTFeed)feed error:(NSError **)error {
    NSData *questionBytes = TTEncode(question, error); if (!questionBytes) return nil;
    NSData *inputBytes = TTEncode(input, error); if (!inputBytes) return nil;
    NSData *optionBytes = TTEncode(options ?: @{}, error); if (!optionBytes) return nil;
    return [self run:verb questionBytes:questionBytes inputBytes:inputBytes optionBytes:optionBytes feed:feed token:NULL error:error];
}
@end
