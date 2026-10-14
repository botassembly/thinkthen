#import "ThinkThenFoundation.h"
#import "TTResultPrivate.h"
#import <dispatch/dispatch.h>
#include <stdio.h>
#define require(condition) do { if (!(condition)) { fprintf(stderr, "Foundation condition failed at line %d: %s\n", __LINE__, #condition); abort(); } } while (0)
static id orderedValue(NSDictionary *node) {
    if ([node[@"kind"] isEqual:@"scalar"]) return node[@"value"];
    if ([node[@"kind"] isEqual:@"array"]) {
        NSMutableArray *items = [NSMutableArray array];
        for (NSDictionary *item in node[@"items"]) [items addObject:orderedValue(item)];
        return [items copy];
    }
    NSMutableArray *entries = [NSMutableArray array];
    for (NSArray *pair in node[@"entries"]) [entries addObject:[[TTOrderedEntry alloc] initWithKey:pair[0] value:orderedValue(pair[1])]];
    return [[TTOrderedMap alloc] initWithEntries:entries];
}
static NSDictionary *orderedFields(NSDictionary *node) {
    TTOrderedMap *object = orderedValue(node);
    NSMutableDictionary *fields = [NSMutableDictionary dictionary];
    for (TTOrderedEntry *entry in object.entries) fields[entry.key] = entry.value;
    return fields;
}
static id readFixture(NSData *data) { return [NSJSONSerialization JSONObjectWithData:data options:NSJSONReadingFragmentsAllowed error:NULL]; }
static id plainValue(id value) {
    if ([value isKindOfClass:TTResultNode.class]) return ((TTResultNode *)value).rawFields;
    if ([value isKindOfClass:NSArray.class]) return TTArray(value, ^id(id item) { return plainValue(item); });
    if ([value isKindOfClass:NSDictionary.class]) return TTMap(value, ^id(id item) { return plainValue(item); });
    return value ?: NSNull.null;
}
static void completeValue(TTCall *call, NSString *verb, NSDictionary *input) {
    NSMutableArray *expected = [NSMutableArray array];
    for (TTResultNode *result in call.results) {
        NSDictionary *record = result.rawFields;
        [expected addObject:record[[verb isEqual:@"rank"] ? @"index" : [verb isEqual:@"filter"] ? @"input" : @"value"]];
    }
    if ([verb isEqual:@"find"]) {
        require(expected.count == 1);
        require([plainValue(call.value.scalar) isEqual:expected[0]]);
    } else if ([verb isEqual:@"relate"]) {
        require(expected.count == 1);
        require([plainValue(call.value.items) isEqual:expected[0]]);
    } else if (![@[@"rank", @"filter"] containsObject:verb] && [@[@"text", @"json"] containsObject:input[@"kind"]]) {
        require(expected.count == 1 && call.value.items == nil);
        require([plainValue(call.value.scalar) isEqual:expected[0]]);
    } else {
        require(call.value.scalar == nil && call.value.items != nil);
        require([plainValue(call.value.items) isEqual:expected]);
    }
}
static void typed(NSArray<TTSessionPacket *> *packets, TTSessionPacketTerminal *terminal) {
    require(terminal.kind.state == TTPresenceValue);
    if (terminal.facts.state == TTPresenceValue) {
        require(terminal.facts.value.callId.state == TTPresenceValue);
        require(terminal.facts.value.requestsSent.state == TTPresenceValue);
    }
    for (TTSessionPacket *packet in packets) {
        if ([packet isKindOfClass:TTSessionPacketDecideRow.class]) {
            TTDecisionRecord *value = ((TTSessionPacketDecideRow *)packet).value.value;
            id raw = value.rawFields[@"value"];
            require(value.value.state == (raw == NSNull.null ? TTPresenceNull : TTPresenceValue));
            if (value.value.state == TTPresenceValue) require([value.value.value isEqual:raw]);
        }
        if ([packet isKindOfClass:TTSessionPacketRecognizeAggregate.class]) {
            NSArray<TTRecognition *> *values = ((TTSessionPacketRecognizeAggregate *)packet).value.value;
            require(values.count == [packet.rawFields[@"value"] count]);
            for (TTRecognition *value in values) {
                require(value.value.state == TTPresenceValue);
                if ([value.value.value isKindOfClass:TTRecognizeEntities.class]) {
                    TTRecognizeEntities *recognized = (id)value.value.value;
                    require(recognized.entities.state == TTPresenceValue);
                    require(recognized.entities.value.count == [recognized.rawFields[@"entities"] count]);
                }
            }
        }
    }
}
int main(int argc, const char *argv[]) {
    @autoreleasepool {
        require(argc == 3 || argc == 4);
        BOOL blocking = argc == 4;
        NSDictionary *fixture = readFixture([NSData dataWithContentsOfFile:[NSString stringWithUTF8String:argv[1]]]);
        NSDictionary *question = orderedFields(fixture[@"ordered"][@"question"]);
        NSDictionary *input = orderedFields(fixture[@"ordered"][@"input"]);
        if ([fixture[@"call_result_contract"] boolValue]) {
            require([input[@"kind"] isEqual:@"records"] && [input[@"items"] count] == 1);
            NSDictionary *original = input[@"items"][0][@"original"];
            input = [original[@"kind"] isEqual:@"text"] ? @{@"kind":@"text", @"text":original[@"text"]} : @{@"kind":@"json", @"value":original[@"value"]};
        }
        NSDictionary *options = orderedFields(fixture[@"ordered"][@"options"]);
        NSDictionary *settings = readFixture([[NSString stringWithUTF8String:argv[2]] dataUsingEncoding:NSUTF8StringEncoding]);
        NSError *error = nil;
        TTFoundationClient *client = [[TTFoundationClient alloc] initWithSettings:[[TTSettings alloc] initWithFields:settings] error:&error];
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
                @"function":verb, @"question":question,
                @"input":input, @"options":options}};
            TTSession *session = [client startRequest:[[TTRequest alloc] initWithFields:request] error:&error];
            if (session) {
                if ([fixture[@"cancel"] boolValue]) [session cancel];
                require([session finish:nil error:&error]);
                if ([fixture[@"held_cancel"] boolValue]) {
                    (void)getchar(); [session cancel]; puts("cancel-fired"); fflush(stdout);
                }
                if (retained && !failure) {
                require(retained.value != nil && retained.results != nil);
                completeValue(retained, verb, input);
                require(retained.facts.state == TTPresenceValue);
                if ([verb isEqual:@"score"] || [verb isEqual:@"tag"] || [verb isEqual:@"annotate"]) require(retained.probability == nil);
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
                    } else if (!ended) require([session waitInput:NO error:&error]);
                }
                [session close]; session = nil;
                explicitPackets = [packets copy];
            }
        } else {
            TTFeed feed = nil;
            NSArray *items = fixture[@"ordered"][@"feed_items"] ? orderedValue(fixture[@"ordered"][@"feed_items"]) : nil;
            if (items) {
                __block NSUInteger index = 0;
                feed = ^TTDescriptor *(TTRequestObject **readerFailure) {
                    *readerFailure = nil;
                    if (index == items.count) return nil;
                    return [[TTDescriptor alloc] initWithFields:@{@"item":items[index++]}];
                };
            }
#define NAMED(name) if ([verb isEqual:@#name]) { if (blocking) { retained = [client name:[[TTQuestion alloc] initWithFields:question] input:[[TTInput alloc] initWithFields:input] options:[[TTControls alloc] initWithFields:options] feed:feed error:&error]; require(retained != nil || error != nil); } else task = [client name:[[TTQuestion alloc] initWithFields:question] input:[[TTInput alloc] initWithFields:input] options:[[TTControls alloc] initWithFields:options] feed:feed completion:completion error:&error]; }
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
            if (blocking && retained && [verb isEqual:@"annotate"] && retained.terminal.failure.state != TTPresenceValue) {
                require(failure == nil && error == nil);
                NSArray<TTAnnotation *> *answer = (id)retained.results;
                require(answer != nil && error == nil);
                NSMutableArray *rows = [NSMutableArray array];
                for (TTSessionPacket *packet in values) if ([packet isKindOfClass:TTSessionPacketAnnotateRow.class]) [rows addObject:((TTSessionPacketAnnotateRow *)packet).value.value];
                require(answer.count == rows.count);
                for (NSUInteger index = 0; index < rows.count; index++) {
                    TTAnnotation *row = rows[index], *projected = answer[index];
                    require(projected.input.state == row.input.state);
                    require([projected.rawFields[@"input"] isEqual:row.rawFields[@"input"]]);
                    require(projected.value.value.count == row.value.value.count);
                    for (NSString *key in row.value.value) {
                        TTAnnotatedField *field = row.value.value[key], *copy = projected.value.value[key];
                        require([copy.kind isEqual:field.kind]);
                        if ([field.value isKindOfClass:TTFailed.class]) {
                            TTFailure *failed = ((TTFailed *)field.value).failed.value;
                            TTFailure *copied = ((TTFailed *)copy.value).failed.value;
                            require([copied.kind.value isEqual:failed.kind.value]);
                            require(copied.cause.state == failed.cause.state);
                            if (failed.cause.state == TTPresenceValue) require([copied.cause.value isEqual:failed.cause.value]);
                        } else require([copy.value isEqual:field.value]);
                    }
                }
            }
            if (retained && [verb isEqual:@"rank"] && !failure) {
                NSArray<NSNumber *> *answer = retained.value.items;
                require(answer != nil && error == nil);
                NSMutableArray *indices = [NSMutableArray array];
                for (TTSessionPacket *packet in values) if ([packet isKindOfClass:TTSessionPacketRankAggregate.class]) {
                    for (TTRankedRecord *row in ((TTSessionPacketRankAggregate *)packet).value.value) [indices addObject:row.index.value];
                }
                require([answer isEqual:indices]);
            }
            if (retained && !failure) {
                require(retained.value != nil && retained.results != nil);
                completeValue(retained, verb, input);
                require(retained.facts.state == TTPresenceValue);
                if ([verb isEqual:@"score"] || [verb isEqual:@"tag"] || [verb isEqual:@"annotate"]) require(retained.probability == nil);
            }
            NSMutableArray *packets = [NSMutableArray array];
            for (TTSessionPacket *packet in values) [packets addObject:packet.rawFields];
            payload = @{@"packets":packets};
            if ([fixture[@"call_result_contract"] boolValue]) {
                require(retained && !failure && retained.results.count == 1);
                NSMutableArray *records = [NSMutableArray array];
                for (TTResultNode *record in retained.results) [records addObject:record.rawFields];
                payload = @{@"packets":packets, @"call_result":@{
                    @"value":retained.value.scalar ?: NSNull.null,
                    @"probability":retained.probability.scalar ?: NSNull.null,
                    @"results":records, @"facts":retained.facts.value.rawFields}};
            }
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
