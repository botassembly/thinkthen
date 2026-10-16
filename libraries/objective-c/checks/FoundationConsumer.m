#import "ThinkThenFoundation.h"
#import <dispatch/dispatch.h>
#include <stdio.h>
#define require(condition) do { if (!(condition)) { fprintf(stderr, "Foundation condition failed at line %d: %s\n", __LINE__, #condition); abort(); } } while (0)
int main(int argc, const char *argv[]) {
    @autoreleasepool {
        require(argc == 2);
        NSString *barrier = [NSString stringWithUTF8String:argv[1]];
        NSError *error = nil;
        TTFoundationClient *client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
        require(client != nil && error == nil);
        TTTextQuestion *question = [TTTextQuestion new]; question.text = @"Is it?";
        TTTextInput *input = [TTTextInput new]; input.text = @"consumer-objc";
        TTRequest *request = [TTRequest new];
        TTCallArguments *arguments = [TTCallArguments new];
        arguments.function = TTFunctionDecide; arguments.question = question; arguments.input = input;
        request.call = arguments;
        TTPlan *plan = [client plan:request error:&error];
        require(plan != nil && error == nil);
        require(plan.firstBodyUtf8.state == TTPresenceValue && plan.records.value.unsignedIntegerValue == 1);
        TTRecordsInput *empty = [TTRecordsInput new]; empty.items = @[];
        arguments.input = empty;
        TTPlan *emptyPlan = [client plan:request error:&error];
        require(emptyPlan != nil && error == nil && emptyPlan.firstBodyUtf8.state == TTPresenceNull);
        require(emptyPlan.requests.state == TTPresenceValue && emptyPlan.requests.value.unsignedIntegerValue == 0);
        arguments.input = input;
        TTTask *refused = [client decide:[TTTextQuestion new] input:input options:nil feed:nil completion:^(TTCall *call, NSError *failure) { (void)call; (void)failure; abort(); } error:&error];
        require(refused == nil && [error isKindOfClass:TTFailureError.class] && !((TTFailureError *)error).retryable);
        error = nil;
        TTCall *refusedCall = nil;
        require((refusedCall = [client decide:[TTTextQuestion new] input:input options:nil feed:nil error:&error]) == nil);
        require(refusedCall == nil && [error isKindOfClass:TTFailureError.class] && ((TTFailureError *)error).kind == TTErrorUsage);
        TTFailureError *savedRefusal = (id)error;
        error = nil;
        __block TTCall *retained = nil;
        dispatch_semaphore_t completed = dispatch_semaphore_create(0);
        TTTask *task = [client decide:question input:input options:nil feed:nil completion:^(TTCall *call, NSError *failure) {
            require(call != nil && failure == nil);
            retained = call;
            dispatch_semaphore_signal(completed);
        } error:&error];
        require(task != nil && error == nil);
        require(dispatch_semaphore_wait(completed, dispatch_time(DISPATCH_TIME_NOW, 4 * NSEC_PER_SEC)) == 0);
        TTUsagePersistenceStatus *usage = [client finishUsageStatus:&error];
        require(usage != nil && error == nil && usage.state == TTUsagePersistenceWritten && usage.advice == nil);
        TTUsagePersistenceStatus *observed = [client usagePersistence:&error];
        require(observed != nil && error == nil && observed.advice == nil);
        require(observed.state == TTUsagePersistenceWritten || observed.state == TTUsagePersistencePending);
        task = nil; client = nil;
        require(usage.state == TTUsagePersistenceWritten && usage.advice == nil);
        require(retained.facts.state == TTPresenceValue);
        require(savedRefusal.kind == TTErrorUsage && !savedRefusal.retryable);
        require([retained.value.scalar isEqual:@YES]);
        require(retained.value.items == nil && retained.results.count == 1);
        require(retained.probability.scalar.doubleValue > 0 && retained.probability.scalar.doubleValue <= 1);
        require(retained.facts.value.requestsSent.value.unsignedIntegerValue == 1);
        error = nil;
        // Authored finite numbers survive native copying beyond decimal's exponent range.
        client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
        for (NSNumber *number in @[@(1e308), @(1e-308)]) {
            TTJSONInput *numericInput = [TTJSONInput new]; numericInput.value = number;
            arguments.function = TTFunctionFilter; arguments.input = numericInput;
            TTPlan *numericPlan = [client plan:request error:&error];
            require(numericPlan != nil && error == nil && numericPlan.records.value.unsignedIntegerValue == 1);
            require(numericPlan.requests.value.unsignedIntegerValue == 1 && numericPlan.firstBodyUtf8.state == TTPresenceValue);
            TTCall *numericCall = nil;
            numericCall = [client filter:question input:numericInput options:nil feed:nil error:&error];
            NSArray *answer = numericCall.value.items;
            require(error == nil && answer.count == 1);
            if ([answer[0] doubleValue] != number.doubleValue) fprintf(stderr, "Filtered number %.17g became %.17g\n", number.doubleValue, [answer[0] doubleValue]);
            require([answer[0] doubleValue] == number.doubleValue);
            require(numericCall.facts.value.requestsSent.value.unsignedIntegerValue == 1);
        }
        TTTextQuestion *filterQuestion = [TTTextQuestion new]; filterQuestion.text = @"Select the originals?";
        NSArray *originals = @[NSNull.null, @NO, @(1e308), @(1e-308)];
        TTControls *filterOptions = [TTControls new]; filterOptions.batch = @1;
        TTRecordsInput *multiple = [TTRecordsInput new];
        NSMutableArray *records = [NSMutableArray array];
        for (id original in originals) { TTOriginalJSON *value = [TTOriginalJSON new]; value.value = original; TTItem *item = [TTItem new]; item.original = value; [records addObject:item]; }
        multiple.items = records;
        TTTextInput *unselected = [TTTextInput new]; unselected.text = @"no";
        for (TTInput *filterInput in @[unselected, multiple, empty]) {
            TTCall *filterCall = nil;
            filterCall = [client filter:filterQuestion input:filterInput options:filterInput == multiple ? filterOptions : nil feed:nil error:&error];
            NSArray *answer = filterCall.value.items;
            require(answer != nil && error == nil);
            require([answer isEqual:filterInput == multiple ? originals : @[]]);
            require(filterCall.probability.items.count == answer.count);
            for (TTCallProbability *odds in filterCall.probability.items) require(odds.scalar.doubleValue > 0 && odds.scalar.doubleValue <= 1);
            require(filterCall.facts.value.requestsSent.value.unsignedIntegerValue == (filterInput == multiple ? 4 : filterInput == empty ? 0 : 1));
        }
        TTTextInput *decisionInput = [TTTextInput new];
        TTControls *decisionControls = [TTControls new]; decisionControls.threshold = @"0.2:0.8";
        for (NSString *text in @[@"no", @"unsure"]) {
            decisionInput.text = text;
            TTCall *decision = [client decide:question input:decisionInput options:decisionControls feed:nil error:&error];
            require(decision && !error && decision.value.items == nil);
            if ([text isEqual:@"no"]) {
                require([decision.value.scalar isEqual:@NO] && decision.probability.scalar.doubleValue == .1);
            } else require(decision.value.scalar == nil && decision.probability == nil);
        }
        TTAuthoredDecideBuilder *invalidMeaning = [TTAuthoredDecideBuilder new];
        invalidMeaning.decide = @"Is it?"; invalidMeaning.trueValue = @NO;
        TTQuestionDefinition *invalidQuestion = [TTQuestionDefinition new]; invalidQuestion.value = invalidMeaning;
        decisionInput.text = @"authored-bool-objc";
        require([client decide:invalidQuestion input:decisionInput options:nil feed:nil error:&error] == nil);
        require([error isKindOfClass:TTFailureError.class] && ((TTFailureError *)error).kind == TTErrorUsage);
        error = nil;
        NSArray *authored = @[NSNull.null, @{@"value":NSNull.null, @"ok":@NO}, @[NSNull.null, @NO, @"leaf"]];
        NSMutableArray<TTCall *> *authoredCalls = [NSMutableArray array];
        for (NSUInteger index = 0; index < authored.count; index++) {
            TTAuthoredDecideBuilder *definition = [TTAuthoredDecideBuilder new];
            definition.decide = @"Is it?"; definition.trueValue = authored[index];
            TTQuestionDefinition *authoredQuestion = [TTQuestionDefinition new]; authoredQuestion.value = definition;
            decisionInput.text = [NSString stringWithFormat:@"authored-objc-%lu", (unsigned long)index];
            TTCall *decision = [client decide:authoredQuestion input:decisionInput options:nil feed:nil error:&error];
            require(decision && !error && decision.value.items == nil);
            [authoredCalls addObject:decision];
        }
        TTAuthoredDecideBuilder *schemaDefinition = [TTAuthoredDecideBuilder new];
        schemaDefinition.decide = @"Is it?";
        schemaDefinition.itemSchema = @{@"type":@"object", @"properties":@{
            @"enabled":@{@"type":@"boolean"}, @"name":@{@"type":@"string"},
            @"count":@{@"type":@"number"}, @"labels":@{@"type":@"array", @"items":@{@"type":@"string"}}}};
        TTQuestionDefinition *schemaQuestion = [TTQuestionDefinition new]; schemaQuestion.value = schemaDefinition;
        for (NSNumber *enabled in @[@NO, @YES]) {
            TTJSONInput *schemaInput = [TTJSONInput new];
            schemaInput.value = @{@"enabled":enabled, @"name":@"schema-objc", @"count":@2, @"labels":@[@"one"]};
            TTCall *schemaCall = [client decide:schemaQuestion input:schemaInput options:nil feed:nil error:&error];
            require(schemaCall && !error && [schemaCall.value.scalar isEqual:@YES]);
            TTDecisionRecord *row = (TTDecisionRecord *)schemaCall.results[0];
            require([row.input.value isEqual:schemaInput.value]);
            TTQuestionDocumentDecide *document = (TTQuestionDocumentDecide *)row.question.value;
            TTDeclarationObjectRootBuilder *declaration = (TTDeclarationObjectRootBuilder *)document.itemSchema.value;
            require([declaration.type.value isEqual:@"object"] && declaration.properties.value.count == 4);
            TTPropertyTypeBooleanBuilder *boolean = (TTPropertyTypeBooleanBuilder *)declaration.properties.value[@"enabled"];
            require([boolean isKindOfClass:TTPropertyTypeBooleanBuilder.class] && [boolean.type.value isEqual:@"boolean"]);
            require([((TTPropertyTypeStringBuilder *)declaration.properties.value[@"name"]).type.value isEqual:@"string"]);
            require([((TTPropertyTypeNumberBuilder *)declaration.properties.value[@"count"]).type.value isEqual:@"number"]);
            require([((TTPropertyTypeArrayBuilder *)declaration.properties.value[@"labels"]).type.value isEqual:@"array"]);
        }
        client = nil;
        for (NSUInteger index = 0; index < authoredCalls.count; index++) {
            require([authoredCalls[index].value.scalar isEqual:authored[index]]);
            require(authoredCalls[index].probability.scalar.doubleValue == .9);
            require(authoredCalls[index].results.count == 1);
        }
        // A new client and unrelated host work progress while a provider is held.
        client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
        dispatch_semaphore_t cancelled = dispatch_semaphore_create(0);
        input = [TTTextInput new]; input.text = @"hold-foundation";
        TTTask *held = [client decide:question input:input options:nil feed:nil completion:^(TTCall *call, NSError *failure) {
            require(call == nil && [failure isKindOfClass:TTFailureError.class] && ((TTFailureError *)failure).kind == TTErrorCancelled && ((TTFailureError *)failure).facts == nil);
            dispatch_semaphore_signal(cancelled);
        } error:&error];
        require(held != nil && error == nil);
        NSString *arrival = [barrier stringByAppendingPathComponent:@"arrived-hold-foundation"];
        NSDate *limit = [NSDate dateWithTimeIntervalSinceNow:4];
        while (![NSFileManager.defaultManager fileExistsAtPath:arrival]) {
            require(limit.timeIntervalSinceNow > 0); [NSThread sleepForTimeInterval:0.005];
        }
        dispatch_semaphore_t progressed = dispatch_semaphore_create(0);
        dispatch_async(dispatch_get_global_queue(QOS_CLASS_DEFAULT, 0), ^{ dispatch_semaphore_signal(progressed); });
        require(dispatch_semaphore_wait(progressed, dispatch_time(DISPATCH_TIME_NOW, NSEC_PER_SEC)) == 0);
        [held cancel];
        require(dispatch_semaphore_wait(cancelled, dispatch_time(DISPATCH_TIME_NOW, NSEC_PER_SEC)) == 0);
        require(![NSFileManager.defaultManager fileExistsAtPath:[barrier stringByAppendingPathComponent:@"release-hold-foundation"]]);
        held = nil; client = nil; retained = nil;
        puts("FOUNDATION_INSTALLED_PASS");
    }
    return 0;
}
