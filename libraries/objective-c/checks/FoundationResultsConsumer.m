#import "TTResults.g.h"
static void require(BOOL condition) { if (!condition) abort(); }
int main(int argc, const char *argv[]) {
    @autoreleasepool {
        require(argc == 2);
        NSData *data = [NSData dataWithContentsOfFile:[NSString stringWithUTF8String:argv[1]]];
        NSDictionary *fixture = [NSJSONSerialization JSONObjectWithData:data options:0 error:NULL];
        require(fixture != nil);
        NSMutableDictionary *fields = [fixture[@"facts"] mutableCopy];
        NSMutableArray *future = [@[@NO, NSNull.null, @0] mutableCopy];
        fields[@"future"] = @{@"values":future};
        TTFacts *facts = [TTFacts read:fields];
        require([facts.rawFields[@"future"] isEqual:fields[@"future"]]);
        [future removeAllObjects];
        require([facts.rawFields[@"future"][@"values"] count] == 3);
        require(facts.model.state == TTPresenceMissing);
        fields[@"model"] = NSNull.null;
        require([TTFacts read:fields].model.state == TTPresenceNull);
        fields[@"model"] = @"future";
        require([[TTFacts read:fields].model.value isEqual:@"future"]);
        require(facts.records.state == TTPresenceValue);
        for (NSDictionary *row in fixture[@"results"]) {
            NSDictionary *result = row[@"result"];
            if ([row[@"type"] isEqual:@"DecideResult"]) {
                TTAtomicDecideValue *decision = [TTAtomicDecideValue read:result];
                require(decision.value.state == TTPresenceValue && [decision.value.value isEqual:@NO]);
            }
            if ([row[@"type"] isEqual:@"ChooseResult"]) {
                NSMutableDictionary *answer = [result[@"answer"] mutableCopy];
                require([(TTAnswerChoice *)[TTAnswer read:answer] confidence].state == TTPresenceValue);
                answer[@"confidence"] = NSNull.null;
                require([(TTAnswerChoice *)[TTAnswer read:answer] confidence].state == TTPresenceNull);
                [answer removeObjectForKey:@"confidence"];
                require([(TTAnswerChoice *)[TTAnswer read:answer] confidence].state == TTPresenceMissing);
            }
            if ([row[@"type"] isEqual:@"AnnotateResult"]) {
                TTAnnotation *annotation = [TTAnnotation read:result];
                require([annotation.value.value[@"ok"].kind isEqual:@"null"]);
                TTAnnotatedField *failed = annotation.value.value[@"bad"];
                require([failed.kind isEqual:@"object"] && [failed.value isKindOfClass:TTFailed.class]);
            }
        }
        TTError *failure = [TTError read:fixture[@"started_error"]];
        require(failure.retryable.state == TTPresenceValue);
        require(TTNativeErrorCode(failure.kind.value) > 0);
        data = nil; fixture = nil; fields = nil;
        require(facts.rawFields[@"future"] != nil);
        puts("FOUNDATION_RESULTS_PASS");
    }
    return 0;
}
