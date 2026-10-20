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
int main(int argc, const char *argv[]) {
    @autoreleasepool {
        require(argc == 3 || argc == 4);
        BOOL blocking = argc == 4;
        NSDictionary *fixture = readFixture([NSData dataWithContentsOfFile:[NSString stringWithUTF8String:argv[1]]]);
        NSDictionary *question = orderedFields(fixture[@"ordered"][@"question"]);
        NSDictionary *input = orderedFields(fixture[@"ordered"][@"input"]);
        if ([fixture[@"call_result_contract"] boolValue]) {
            require([input[@"kind"] isEqual:@"records"] && [input[@"items"] count] == 1);
            NSDictionary *original = fixture[@"input"][@"items"][0][@"original"];
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
        NSString *verb = fixture[@"verb"];
        {
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
            if ([fixture[@"held_cancel"] boolValue]) {
                (void)getchar(); [task cancel]; puts("cancel-fired"); fflush(stdout);
            } else if ([fixture[@"cancel"] boolValue]) [task cancel];
            require(dispatch_semaphore_wait(completed, dispatch_time(DISPATCH_TIME_NOW, 30 * NSEC_PER_SEC)) == 0);
        } else failure = error;
        task = nil; client = nil;
        NSDictionary *payload;
        if (retained) {
            require(!failure);
            require(retained.facts.state == TTPresenceValue);
            require(retained.answer.state == TTPresenceValue);
            require(retained.resolution.state == TTPresenceValue);
            payload = retained.rawFields;
        } else {
            require([failure isKindOfClass:TTFailureError.class]);
            TTFailureError *native = (id)failure;
            NSMutableDictionary *fields = [@{@"code":@(native.kind), @"message":native.localizedDescription,
                @"retryable":@(native.retryable)} mutableCopy];
            if (native.position) fields[@"position"] = native.position;
            if (native.facts) fields[@"facts"] = native.facts.rawFields;
            payload = @{@"admission":fields};
        }
        NSData *output = [NSJSONSerialization dataWithJSONObject:payload options:0 error:NULL];
        require(output != nil);
        puts([[NSString alloc] initWithData:output encoding:NSUTF8StringEncoding].UTF8String);
    }
    return 0;
}
