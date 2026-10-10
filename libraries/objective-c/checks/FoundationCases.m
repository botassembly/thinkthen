#import "ThinkThenFoundation.h"
#import <dispatch/dispatch.h>
#include <stdio.h>
static void require(BOOL condition) { if (!condition) abort(); }
static id read(NSData *data) { return [NSJSONSerialization JSONObjectWithData:data options:NSJSONReadingFragmentsAllowed error:NULL]; }
static void typed(NSArray<TTSessionPacket *> *packets, TTSessionPacketTerminal *terminal) {
    require(terminal.kind.state == TTPresenceValue);
    if (terminal.facts.state == TTPresenceValue) {
        require(terminal.facts.value.callId.state == TTPresenceValue);
        require(terminal.facts.value.requestsSent.state == TTPresenceValue);
    }
    for (TTSessionPacket *packet in packets) {
        if ([packet isKindOfClass:TTSessionPacketDecideRow.class]) {
            TTAtomicDecideValue *value = ((TTSessionPacketDecideRow *)packet).value.value;
            require(value.value.state == TTPresenceValue);
            require([value.value.value isEqual:value.rawFields[@"value"]]);
        }
        if ([packet isKindOfClass:TTSessionPacketRecognizeAggregate.class]) {
            NSArray<TTRecognition *> *values = ((TTSessionPacketRecognizeAggregate *)packet).value.value;
            require(values.count == [packet.rawFields[@"value"] count]);
            for (TTRecognition *value in values) {
                require(value.value.state == TTPresenceValue);
                if ([value.value.value isKindOfClass:TTRecognizeFieldsEntities.class]) {
                    TTRecognizeFieldsEntities *recognized = (id)value.value.value;
                    require(recognized.entities.state == TTPresenceValue);
                    require(recognized.entities.value.count == [recognized.rawFields[@"entities"] count]);
                }
            }
        }
    }
}
int main(int argc, const char *argv[]) {
    @autoreleasepool {
        require(argc == 3);
        NSDictionary *fixture = read([NSData dataWithContentsOfFile:[NSString stringWithUTF8String:argv[1]]]);
        NSDictionary *settings = read([[NSString stringWithUTF8String:argv[2]] dataUsingEncoding:NSUTF8StringEncoding]);
        NSError *error = nil;
        TTFoundationClient *client = [[TTFoundationClient alloc] initWithSettings:settings error:&error];
        __block TTCall *retained = nil;
        __block NSError *failure = nil;
        dispatch_semaphore_t completed = dispatch_semaphore_create(0);
        TTCompletion completion = ^(TTCall *call, NSError *problem) {
            retained = call; failure = problem; dispatch_semaphore_signal(completed);
        };
        TTTask *task = nil;
        NSArray<TTSessionPacket *> *explicitPackets = nil;
        TTSessionPacketTerminal *explicitTerminal = nil;
        NSString *verb = fixture[@"verb"];
        if ([fixture[@"cancel"] boolValue] || [fixture[@"held_cancel"] boolValue]) {
            // The explicit session exposes the final native terminal. TTTask
            // deliberately settles cancellation early with facts still pending.
            NSDictionary *request = @{@"schema":TTRequestVersion, @"call":@{
                @"function":verb, @"question":fixture[@"question"],
                @"input":fixture[@"input"], @"options":fixture[@"options"]}};
            TTSession *session = [client startRequest:request error:&error];
            if (session) {
                if ([fixture[@"cancel"] boolValue]) [session cancel];
                require([session finish:nil error:&error]);
                if ([fixture[@"held_cancel"] boolValue]) {
                    (void)getchar(); [session cancel]; puts("cancel-fired"); fflush(stdout);
                }
                NSMutableArray *packets = [NSMutableArray array];
                NSDate *limit = [NSDate dateWithTimeIntervalSinceNow:30];
                BOOL ended = NO;
                while (!ended) {
                    require(limit.timeIntervalSinceNow > 0);
                    TTSessionPacket *packet = [session tryReadEnded:&ended error:&error];
                    require(error == nil);
                    if (packet) {
                        [packets addObject:packet];
                        if ([packet isKindOfClass:TTSessionPacketTerminal.class]) explicitTerminal = (id)packet;
                    } else if (!ended) [NSThread sleepForTimeInterval:0.001];
                }
                [session close]; session = nil;
                explicitPackets = [packets copy];
            }
        } else {
#define NAMED(name) if ([verb isEqual:@#name]) task = [client name:fixture[@"question"] input:fixture[@"input"] options:fixture[@"options"] feed:nil completion:completion error:&error];
        NAMED(decide) NAMED(choose) NAMED(tag) NAMED(score) NAMED(filter)
        NAMED(rank) NAMED(find) NAMED(annotate) NAMED(recognize) NAMED(relate)
#undef NAMED
        }
        if (task) {
            require(dispatch_semaphore_wait(completed, dispatch_time(DISPATCH_TIME_NOW, 30 * NSEC_PER_SEC)) == 0);
        } else failure = error;
        // Retained public result objects must outlive both native owners.
        task = nil; client = nil;
        NSDictionary *payload;
        if (retained || explicitPackets) {
            NSArray<TTSessionPacket *> *values = retained ? retained.packets : explicitPackets;
            typed(values, retained ? retained.terminal : explicitTerminal);
            NSMutableArray *packets = [NSMutableArray array];
            for (TTSessionPacket *packet in values) [packets addObject:packet.rawFields];
            payload = @{@"packets":packets};
        } else {
            require(failure != nil);
            payload = @{@"admission":@{@"code":@(failure.code), @"message":failure.localizedDescription}};
        }
        NSData *output = [NSJSONSerialization dataWithJSONObject:payload options:0 error:NULL];
        require(output != nil);
        puts([[NSString alloc] initWithData:output encoding:NSUTF8StringEncoding].UTF8String);
    }
    return 0;
}
