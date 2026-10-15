#import "ThinkThenFoundation.h"
#import <thinkthen.h>
#import "TTResultPrivate.h"
extern id TTNative_thinkthen_complete_session_packet_v1(const struct thinkthen_complete_session_packet_v1 *value);
extern id TTNative_thinkthen_complete_immediate_facts_v1(const struct thinkthen_complete_immediate_facts_v1 *value);
extern id TTNative_thinkthen_complete_json_v1(const struct thinkthen_complete_json_v1 *value);
NSErrorDomain const TTErrorDomain = @"io.github.botassembly.thinkthen";
NSString * const TTFailureDetailsKey = @"ThinkThenFailureDetails";
TTByteImage *TTImageBytes(NSData *bytes, NSString *media) {
    TTByteImage *image = [TTByteImage new];
    image.bytes = [bytes base64EncodedStringWithOptions:0]; image.media = media; return image;
}
@interface TTFailureError ()
@property(nonatomic) BOOL retryable;
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
static NSError *TTNativeFailure(const struct thinkthen_error_view_v1 *view, int code, NSString *fallback) {
    NSString *message = view ? [[NSString alloc] initWithBytes:view->message.data length:view->message.len encoding:NSUTF8StringEncoding] : fallback;
    TTFailureError *error = [[TTFailureError alloc] initWithDomain:TTErrorDomain code:code userInfo:@{NSLocalizedDescriptionKey:message ?: fallback}];
    error.retryable = view && view->retryable;
    if (view && view->facts) error.facts = [TTImmediateFacts read:TTNative_thinkthen_complete_immediate_facts_v1(view->facts)];
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
@interface TTSession () {
    struct thinkthen_session *_native;
    NSLock *_lock;
    NSUInteger _waiters;
    BOOL _closed;
}
- (instancetype)initWithNative:(struct thinkthen_session *)native;
- (BOOL)waitInterest:(uint32_t)interest error:(NSError **)error;
@end
@implementation TTSession
- (instancetype)initWithNative:(struct thinkthen_session *)native {
    if ((self = [super init])) { _native = native; _lock = [NSLock new]; }
    return self;
}
- (BOOL)live:(NSError **)error {
    if (_native && !_closed) return YES;
    if (error) *error = TTNativeFailure(NULL, TTErrorLocal, @"Session is closed");
    return NO;
}
- (BOOL)tryPush:(TTDescriptor *)descriptor accepted:(BOOL *)accepted closed:(BOOL *)closed error:(NSError **)error {
    NSData *data = TTEncode(descriptor, error); if (!data) return NO;
    [_lock lock];
    @try {
        if (![self live:error]) return NO;
        uint32_t status = 0;
        if (!TTCheck(thinkthen_session_try_push(_native, data.bytes, data.length, &status), error)) return NO;
        *accepted = status == THINKTHEN_SESSION_ACCEPTED_V1;
        *closed = status == THINKTHEN_SESSION_CLOSED_V1;
        return YES;
    } @finally { [_lock unlock]; }
}
- (BOOL)finish:(TTRequestObject *)readerFailure error:(NSError **)error {
    NSData *data = readerFailure ? TTEncode(readerFailure, error) : nil;
    if (readerFailure && !data) return NO;
    [_lock lock];
    @try { return [self live:error] && TTCheck(thinkthen_session_finish(_native, data.bytes, data.length), error); }
    @finally { [_lock unlock]; }
}
- (TTSessionPacket *)tryReadEnded:(BOOL *)ended error:(NSError **)error {
    [_lock lock];
    struct thinkthen_session_result *packet = NULL;
    @try {
        *ended = NO;
        if (![self live:error]) return nil;
        uint32_t status = 0;
        if (!TTCheck(thinkthen_session_try_read(_native, &status, &packet), error)) return nil;
        *ended = status == THINKTHEN_SESSION_END_V1;
        if (!packet) return nil;
        const struct thinkthen_complete_session_packet_v1 *view = NULL;
        if (!TTCheck(thinkthen_session_result_view(packet, &view), error)) return nil;
        return [TTSessionPacket read:TTNative_thinkthen_complete_session_packet_v1(view)];
    } @finally { thinkthen_session_result_free(packet); [_lock unlock]; }
}
- (void)cancel { [_lock lock]; thinkthen_session_cancel(_native); [_lock unlock]; }
- (BOOL)waitInput:(BOOL)input error:(NSError **)error {
    return [self waitInterest:input ? THINKTHEN_SESSION_WAIT_INPUT_V1 : THINKTHEN_SESSION_WAIT_SETTLEMENT_V1 error:error];
}
- (BOOL)waitInterest:(uint32_t)interest error:(NSError **)error {
    [_lock lock];
    if (![self live:error]) { [_lock unlock]; return NO; }
    struct thinkthen_session *retained = _native; _waiters++;
    [_lock unlock];
    uint32_t ready = 0;
    BOOL ok = TTCheck(thinkthen_session_wait(retained, interest, UINT32_MAX, &ready), error);
    [_lock lock]; _waiters--;
    if (_closed && !_waiters) { thinkthen_session_free(_native); _native = NULL; }
    [_lock unlock]; return ok;
}
- (void)close {
    [_lock lock]; _closed = YES; thinkthen_session_cancel(_native);
    if (!_waiters) { thinkthen_session_free(_native); _native = NULL; }
    [_lock unlock];
}
- (void)dealloc { thinkthen_session_free(_native); }
@end
@interface TTCall ()
@property(nonatomic, copy) NSArray<TTSessionPacket *> *packets;
@property(nonatomic, strong) TTSessionPacketTerminal *terminal;
@property(nonatomic, copy) NSString *function;
@property(nonatomic) BOOL singleInput;
- (instancetype)initWithPackets:(NSArray *)packets terminal:(TTSessionPacketTerminal *)terminal function:(NSString *)function singleInput:(BOOL)singleInput;
@end
@implementation TTCall
- (TTPresence<TTFacts *> *)facts { return self.terminal.facts; }
- (instancetype)initWithPackets:(NSArray *)packets terminal:(TTSessionPacketTerminal *)terminal function:(NSString *)function singleInput:(BOOL)singleInput {
    if ((self = [super init])) { _packets = [packets copy]; _terminal = terminal; _function = [function copy]; _singleInput = singleInput; }
    return self;
}
@end
@interface TTTask () {
    TTSession *_session;
    NSString *_function;
    BOOL _singleInput;
    TTFeed _feed;
    TTCompletion _completion;
    dispatch_queue_t _queue;
    TTDescriptor *_pending;
    NSMutableArray *_packets;
    TTSessionPacketTerminal *_terminal;
    BOOL _finished;
}
- (instancetype)initWithSession:(TTSession *)session function:(NSString *)function feed:(nullable TTFeed)feed completion:(TTCompletion)completion singleInput:(BOOL)singleInput asynchronous:(BOOL)asynchronous;
- (void)run;
@property(atomic) BOOL cancelled;
- (void)tick;
@end
@implementation TTTask
- (instancetype)initWithSession:(TTSession *)session function:(NSString *)function feed:(TTFeed)feed completion:(TTCompletion)completion singleInput:(BOOL)singleInput asynchronous:(BOOL)asynchronous {
    if ((self = [super init])) {
        _session = session; _function = [function copy]; _singleInput = singleInput; _feed = [feed copy]; _completion = [completion copy];
        _packets = [NSMutableArray array]; _queue = dispatch_queue_create("thinkthen.foundation.callback", DISPATCH_QUEUE_SERIAL);
        if (asynchronous) dispatch_async(_queue, ^{ [self run]; });
    }
    return self;
}
- (void)run {
    while (!_finished) {
        if (self.cancelled) { [self settle:TTNativeFailure(NULL, TTErrorCancelled, @"Cancelled") call:nil]; break; }
        [self tick];
        if (_finished) break;
        if (_feed && !_pending) continue;
        NSError *error = nil;
        if (![_session waitInterest:_pending ? THINKTHEN_SESSION_WAIT_INPUT_V1 : THINKTHEN_SESSION_WAIT_RESULT_V1 error:&error]) [self settle:error call:nil];
    }
}
- (void)settle:(NSError *)error call:(TTCall *)call {
    if (_finished) return;
    _finished = YES; [_session close];
    if (self.cancelled) { call = nil; error = TTNativeFailure(NULL, TTErrorCancelled, @"Cancelled"); }
    _feed = nil; _pending = nil;
    TTCompletion completion = _completion; _completion = nil;
    completion(call, error);
}
- (void)tick {
    if (_finished) return;
    NSError *error = nil;
    if (_feed) {
        TTRequestObject *readerFailure = nil;
        if (!_pending) _pending = _feed(&readerFailure);
        if (!_pending) {
            if (![_session finish:readerFailure error:&error]) { [self settle:error call:nil]; return; }
            _feed = nil;
        } else {
            BOOL accepted = NO, closed = NO;
            if (![_session tryPush:_pending accepted:&accepted closed:&closed error:&error]) { [self settle:error call:nil]; return; }
            if (accepted || closed) _pending = nil;
            if (closed) _feed = nil;
        }
    }
    // A bounded number per tick keeps other queued callbacks progressing.
    for (NSUInteger index = 0; index < 16; index++) {
        BOOL ended = NO;
        TTSessionPacket *packet = [_session tryReadEnded:&ended error:&error];
        if (error) { [self settle:error call:nil]; return; }
        if (packet) { [_packets addObject:packet]; if ([packet isKindOfClass:TTSessionPacketTerminal.class]) _terminal = (id)packet; }
        if (ended) {
            if (!_terminal) { [self settle:TTNativeFailure(NULL, TTErrorDefect, @"Native session ended without terminal") call:nil]; return; }
            TTCall *call = [[TTCall alloc] initWithPackets:_packets terminal:_terminal function:_function singleInput:_singleInput];
            if (_terminal.failure.state == TTPresenceValue) {
                TTCallError *failure = _terminal.failure.value;
                error = TTFailureForCall(failure, call);
            }
            [self settle:error call:call]; return;
        }
        if (!packet) break;
    }
}
- (void)cancel {
    // Signal now; dispatch only host completion. A held provider is never joined.
    self.cancelled = YES; [_session cancel]; [_session close];
    dispatch_async(_queue, ^{ [self settle:TTNativeFailure(NULL, TTErrorCancelled, @"Cancelled") call:nil]; });
}
@end
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
        _native = settings ? thinkthen_engine_new_with(text.UTF8String) : thinkthen_engine_new();
        if (!_native) {
            if (error) *error = TTNativeFailure(thinkthen_error_view(NULL), thinkthen_error_code(NULL), @"Engine construction failed");
            return nil;
        }
    }
    return self;
}
- (void)dealloc { thinkthen_engine_free(_native); }
- (TTUsagePersistenceStatus *)usageStatus:(BOOL)finish error:(NSError **)error {
    struct thinkthen_complete_usage_persistence_v1 state;
    struct thinkthen_complete_utf8_v1 advice;
    int code = finish ? thinkthen_engine_finish_usage_status_v1(_native, &state, &advice)
                      : thinkthen_engine_usage_persistence_v1(_native, &state, &advice);
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
        const struct thinkthen_complete_json_v1 *view = NULL;
        if (!TTCheck(thinkthen_value_view(owner, &view), error)) return nil;
        return [TTPlan read:TTNative_thinkthen_complete_json_v1(view)];
    } @finally { thinkthen_value_free(owner); }
}
- (TTSession *)startRequest:(TTRequest *)request error:(NSError **)error {
    NSData *data = TTEncode(request, error); if (!data) return nil;
    struct thinkthen_session *session = NULL;
    if (!TTCheck(thinkthen_session_new_with_surface(_native, data.bytes, data.length, "objective-c", 11, &session), error)) return nil;
    return [[TTSession alloc] initWithNative:session];
}
- (TTTask *)start:(NSString *)verb question:(TTQuestion *)question input:(TTInput *)input options:(TTControls *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    TTSession *session = [self namedSession:verb question:question input:input options:options feed:feed error:error];
    return session ? [[TTTask alloc] initWithSession:session function:verb feed:feed completion:completion singleInput:TTCallSingleInput(input) asynchronous:YES] : nil;
}
- (TTSession *)namedSession:(NSString *)verb question:(TTQuestion *)question input:(TTInput *)input options:(TTControls *)options feed:(TTFeed)feed error:(NSError **)error {
    NSData *questionBytes = TTEncode(question, error); if (!questionBytes) return nil;
    NSData *inputBytes = TTEncode(input, error); if (!inputBytes) return nil;
    NSData *optionBytes = TTEncode(options ?: @{}, error); if (!optionBytes) return nil;
    NSData *functionBytes = [verb dataUsingEncoding:NSUTF8StringEncoding];
    struct thinkthen_named_request_v1 request = {
        {functionBytes.bytes, functionBytes.length}, {questionBytes.bytes, questionBytes.length},
        {inputBytes.bytes, inputBytes.length}, {optionBytes.bytes, optionBytes.length}};
    struct thinkthen_session *native = NULL;
    if (!TTCheck(thinkthen_session_new_named_with_surface(_native, &request, (struct thinkthen_complete_utf8_v1){"objective-c", 11}, &native), error)) return nil;
    TTSession *session = [[TTSession alloc] initWithNative:native];
    if (!feed && ![session finish:nil error:error]) { [session close]; return nil; }
    return session;
}
- (TTCall *)complete:(NSString *)verb question:(TTQuestion *)question input:(TTInput *)input options:(TTControls *)options feed:(TTFeed)feed error:(NSError **)error {
    TTSession *session = [self namedSession:verb question:question input:input options:options feed:feed error:error];
    if (!session) return nil;
    __block TTCall *result = nil;
    __block NSError *failure = nil;
    TTTask *task = [[TTTask alloc] initWithSession:session function:verb feed:feed completion:^(TTCall *call, NSError *problem) { result = call; failure = problem; } singleInput:TTCallSingleInput(input) asynchronous:NO];
    [task run];
    if (failure && error) *error = failure;
    return result;
}
@end
