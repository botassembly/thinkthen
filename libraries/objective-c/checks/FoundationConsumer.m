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
        require([retained decideAnswer:&error].state == TTPresenceValue && error == nil);
        // A new client and unrelated host work progress while a provider is held.
        client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
        dispatch_semaphore_t cancelled = dispatch_semaphore_create(0);
        input = [TTTextInput new]; input.text = @"hold-foundation";
        TTTask *held = [client decide:question input:input options:nil feed:nil completion:^(TTCall *call, NSError *failure) {
            require(call == nil && failure.code == TTNativeErrorCode(@"cancelled"));
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
