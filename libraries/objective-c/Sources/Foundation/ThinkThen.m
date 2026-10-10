#import "ThinkThen.h"
#import <thinkthen.h>
NSErrorDomain const TTErrorDomain = @"io.github.botassembly.thinkthen";
NSString * const TTFailureDetailsKey = @"ThinkThenFailureDetails";
static NSError *TTImmediate(int code) {
    return [NSError errorWithDomain:TTErrorDomain code:code userInfo:@{NSLocalizedDescriptionKey: [NSString stringWithUTF8String:thinkthen_session_error_message()] ?: @"Native session failure"}];
}
static BOOL TTCheck(int code, NSError **error) { if (!code) return YES; if (error) *error = TTImmediate(code); return NO; }
static NSData *TTEncode(id object, NSError **error) {
    // Foundation owns representation checks; native Request owns admission.
    return [NSJSONSerialization dataWithJSONObject:object options:NSJSONWritingFragmentsAllowed error:error];
}
@interface TTSession () {
    struct thinkthen_session *_native;
    NSLock *_lock;
}
- (instancetype)initWithNative:(struct thinkthen_session *)native;
@end
@implementation TTSession
- (instancetype)initWithNative:(struct thinkthen_session *)native {
    if ((self = [super init])) { _native = native; _lock = [NSLock new]; }
    return self;
}
- (BOOL)live:(NSError **)error {
    if (_native) return YES;
    if (error) *error = [NSError errorWithDomain:TTErrorDomain code:THINKTHEN_ELOCAL userInfo:@{NSLocalizedDescriptionKey:@"Session is closed"}];
    return NO;
}
- (BOOL)tryPush:(NSDictionary *)descriptor accepted:(BOOL *)accepted closed:(BOOL *)closed error:(NSError **)error {
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
- (BOOL)finish:(NSDictionary *)readerFailure error:(NSError **)error {
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
        const char *bytes = NULL; size_t length = 0;
        if (!TTCheck(thinkthen_session_result_json(packet, &bytes, &length), error)) return nil;
        NSData *copy = [NSData dataWithBytes:bytes length:length];
        id value = [NSJSONSerialization JSONObjectWithData:copy options:NSJSONReadingFragmentsAllowed error:error];
        return value ? [TTSessionPacket read:value] : nil;
    } @finally { thinkthen_session_result_free(packet); [_lock unlock]; }
}
- (void)cancel { [_lock lock]; thinkthen_session_cancel(_native); [_lock unlock]; }
- (void)close { [_lock lock]; thinkthen_session_free(_native); _native = NULL; [_lock unlock]; }
- (void)dealloc { thinkthen_session_free(_native); }
@end
@interface TTCall ()
- (instancetype)initWithPackets:(NSArray *)packets terminal:(TTSessionPacketTerminal *)terminal;
@end
@implementation TTCall
- (instancetype)initWithPackets:(NSArray *)packets terminal:(TTSessionPacketTerminal *)terminal {
    if ((self = [super init])) { _packets = [packets copy]; _terminal = terminal; }
    return self;
}
@end
@interface TTTask () {
    TTSession *_session;
    TTFeed _feed;
    TTCompletion _completion;
    dispatch_source_t _timer;
    dispatch_queue_t _queue;
    NSDictionary *_pending;
    NSMutableArray *_packets;
    TTSessionPacketTerminal *_terminal;
    BOOL _finished;
}
- (instancetype)initWithSession:(TTSession *)session feed:(nullable TTFeed)feed completion:(TTCompletion)completion;
@property(atomic) BOOL cancelled;
- (void)tick;
@end
@implementation TTTask
- (instancetype)initWithSession:(TTSession *)session feed:(TTFeed)feed completion:(TTCompletion)completion {
    if ((self = [super init])) {
        _session = session; _feed = [feed copy]; _completion = [completion copy];
        _packets = [NSMutableArray array]; _queue = dispatch_queue_create("thinkthen.foundation.callback", DISPATCH_QUEUE_SERIAL);
        _timer = dispatch_source_create(DISPATCH_SOURCE_TYPE_TIMER, 0, 0, _queue);
        // The source retains this task until completion or cancellation breaks the cycle.
        dispatch_source_set_event_handler(_timer, ^{ [self tick]; });
        dispatch_source_set_timer(_timer, DISPATCH_TIME_NOW, 10 * NSEC_PER_MSEC, NSEC_PER_MSEC);
        dispatch_resume(_timer);
    }
    return self;
}
- (void)settle:(NSError *)error call:(TTCall *)call {
    if (_finished) return;
    _finished = YES; [_session close];
    if (self.cancelled) { call = nil; error = [NSError errorWithDomain:TTErrorDomain code:THINKTHEN_ECANCELLED userInfo:@{NSLocalizedDescriptionKey:@"Cancelled"}]; }
    dispatch_source_cancel(_timer); _timer = nil; _feed = nil; _pending = nil;
    TTCompletion completion = _completion; _completion = nil;
    completion(call, error);
}
- (void)tick {
    if (_finished) return;
    NSError *error = nil;
    if (_feed) {
        NSDictionary *readerFailure = nil;
        if (!_pending) _pending = [_feed(&readerFailure) copy];
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
            if (!_terminal) { [self settle:[NSError errorWithDomain:TTErrorDomain code:THINKTHEN_EDEFECT userInfo:@{NSLocalizedDescriptionKey:@"Native session ended without terminal"}] call:nil]; return; }
            TTCall *call = [[TTCall alloc] initWithPackets:_packets terminal:_terminal];
            if (_terminal.failure.state == TTPresenceValue) {
                TTCallError *failure = _terminal.failure.value;
                error = [NSError errorWithDomain:TTErrorDomain code:TTNativeErrorCode(failure.error.value.kind.value) userInfo:@{NSLocalizedDescriptionKey:failure.error.value.message.value, TTFailureDetailsKey:failure, @"ThinkThenCall":call}];
            }
            [self settle:error call:call]; return;
        }
        if (!packet) break;
    }
}
- (void)cancel {
    // Signal now; dispatch only host completion. A held provider is never joined.
    self.cancelled = YES; [_session cancel]; [_session close];
    dispatch_async(_queue, ^{ [self settle:[NSError errorWithDomain:TTErrorDomain code:THINKTHEN_ECANCELLED userInfo:@{NSLocalizedDescriptionKey:@"Cancelled"}] call:nil]; });
}
@end
@interface TTFoundationClient () { struct thinkthen_engine *_native; }
@end
@implementation TTFoundationClient
- (instancetype)initWithSettings:(NSDictionary *)settings error:(NSError **)error {
    if ((self = [super init])) {
        NSData *bytes = settings ? TTEncode(settings, error) : nil;
        if (settings && !bytes) return nil;
        NSString *text = bytes ? [[NSString alloc] initWithData:bytes encoding:NSUTF8StringEncoding] : nil;
        _native = settings ? thinkthen_engine_new_with(text.UTF8String) : thinkthen_engine_new();
        if (!_native) {
            if (error) *error = [NSError errorWithDomain:TTErrorDomain code:thinkthen_error_code(NULL) userInfo:@{NSLocalizedDescriptionKey:[NSString stringWithUTF8String:thinkthen_error_message(NULL)] ?: @"Engine construction failed"}];
            return nil;
        }
    }
    return self;
}
- (void)dealloc { thinkthen_engine_free(_native); }
- (TTSession *)startRequest:(NSDictionary *)request error:(NSError **)error {
    NSData *data = TTEncode(request, error); if (!data) return nil;
    struct thinkthen_session *session = NULL;
    if (!TTCheck(thinkthen_session_new_with_surface(_native, data.bytes, data.length, "objective-c", 11, &session), error)) return nil;
    return [[TTSession alloc] initWithNative:session];
}
- (TTTask *)start:(NSString *)verb question:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error {
    NSMutableDictionary *call = [@{@"function":verb, @"question":question, @"input":input} mutableCopy];
    if (options) call[@"options"] = options;
    TTSession *session = [self startRequest:@{@"schema":TTRequestVersion, @"call":call} error:error];
    if (!session) return nil;
    if (!feed && ![session finish:nil error:error]) { [session close]; return nil; }
    return [[TTTask alloc] initWithSession:session feed:feed completion:completion];
}
- (TTTask *)decide:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"decide" question:question input:input options:options feed:feed completion:completion error:error]; }
- (TTTask *)choose:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"choose" question:question input:input options:options feed:feed completion:completion error:error]; }
- (TTTask *)tag:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"tag" question:question input:input options:options feed:feed completion:completion error:error]; }
- (TTTask *)score:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"score" question:question input:input options:options feed:feed completion:completion error:error]; }
- (TTTask *)filter:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"filter" question:question input:input options:options feed:feed completion:completion error:error]; }
- (TTTask *)rank:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"rank" question:question input:input options:options feed:feed completion:completion error:error]; }
- (TTTask *)find:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"find" question:question input:input options:options feed:feed completion:completion error:error]; }
- (TTTask *)annotate:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"annotate" question:question input:input options:options feed:feed completion:completion error:error]; }
- (TTTask *)recognize:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"recognize" question:question input:input options:options feed:feed completion:completion error:error]; }
- (TTTask *)relate:(id)question input:(NSDictionary *)input options:(NSDictionary *)options feed:(TTFeed)feed completion:(TTCompletion)completion error:(NSError **)error { return [self start:@"relate" question:question input:input options:options feed:feed completion:completion error:error]; }
@end
