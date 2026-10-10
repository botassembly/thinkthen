#import "ThinkThenFoundation.h"
#import <dispatch/dispatch.h>
static void require(BOOL condition) { if (!condition) abort(); }
int main(int argc, const char *argv[]) {
    @autoreleasepool {
        require(argc == 2);
        NSString *barrier = [NSString stringWithUTF8String:argv[1]];
        NSError *error = nil;
        TTFoundationClient *client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
        require(client != nil && error == nil);
        NSDictionary *question = @{@"kind":@"text", @"text":@"Is it?"};
        __block TTCall *retained = nil;
        dispatch_semaphore_t completed = dispatch_semaphore_create(0);
        TTTask *task = [client decide:question input:@{@"kind":@"text", @"text":@"consumer-objc"} options:nil feed:nil completion:^(TTCall *call, NSError *failure) {
            require(call != nil && failure == nil);
            retained = call;
            dispatch_semaphore_signal(completed);
        } error:&error];
        require(task != nil && error == nil);
        require(dispatch_semaphore_wait(completed, dispatch_time(DISPATCH_TIME_NOW, 4 * NSEC_PER_SEC)) == 0);
        TTUsagePersistenceStatus *usage = [client finishUsageStatus:&error];
        require(usage != nil && error == nil && usage.state == TTUsagePersistenceWritten && usage.advice == nil);
        require([client usagePersistence:&error].state == usage.state && error == nil);
        task = nil; client = nil;
        require(usage.state == TTUsagePersistenceWritten && usage.advice == nil);
        require(retained.terminal != nil && retained.packets.count > 0);
        require(retained.terminal.facts.state == TTPresenceValue);
        // A new client and unrelated host work progress while a provider is held.
        client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
        dispatch_semaphore_t cancelled = dispatch_semaphore_create(0);
        TTTask *held = [client decide:question input:@{@"kind":@"text", @"text":@"hold-foundation"} options:nil feed:nil completion:^(TTCall *call, NSError *failure) {
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
