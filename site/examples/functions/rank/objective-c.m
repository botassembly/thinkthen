#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    NSDictionary *question = @{
        @"kind": @"text",
        @"text": @"Is this urgent?",
    };
    NSArray<NSString *> *inbox = @[
        (@"Newsletter: our autumn catalog is here. "
        @"No reply needed."),
        (@"Our checkout page is down and "
        @"customers cannot pay"),
        @"Reminder: your invoice is due in 30 days",
        @"Please send the signed quote by 5 pm today",
    ];
    NSMutableArray *records = [NSMutableArray array];
    for (NSString *one in inbox) {
        [records addObject:@{
            @"original": @{@"kind": @"text", @"text": one},
        }];
    }
    NSDictionary *input = @{
        @"kind": @"records",
        @"items": records,
    };
    NSMutableArray *byUrgency =
        [@[@0, @0, @0, @0] mutableCopy];
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client rank:question
        input:input
        options:nil
        feed:nil
        completion:^(TTCall *call, NSError *failure) {
            answered = call;
            dispatch_semaphore_signal(done);
        }
        error:&error];
    assert(task);
    dispatch_semaphore_wait(done, DISPATCH_TIME_FOREVER);

    Class wanted = TTSessionPacketRankAggregate.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketRankAggregate *found = packet;
        NSArray *places = found.value.value;
        for (TTAtomicNonZeroUsize *one in places) {
            int place = one.value.value.intValue;
            byUrgency[place - 1] = one.index.value;
        }
    }
    NSArray *expected = @[@1, @3, @2, @0];
    assert([byUrgency isEqualToArray:expected]);
    return 0;
}
