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
        @"text": @"Which line gives the refund deadline?",
    };
    NSArray<NSString *> *policy = @[
        @"Returns need the original receipt.",
        @"Refunds are issued within 30 days of purchase.",
        @"Shipping is free on orders over $50.",
        @"Gift cards cannot be exchanged for cash.",
    ];
    NSMutableArray *records = [NSMutableArray array];
    for (NSString *one in policy) {
        [records addObject:@{
            @"original": @{@"kind": @"text", @"text": one},
        }];
    }
    NSDictionary *input = @{
        @"kind": @"records",
        @"items": records,
    };
    NSString *refundDeadline = nil;
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client find:question
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

    Class wanted = TTSessionPacketFindAggregate.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketFindAggregate *found = packet;
        refundDeadline = found.value.value.value.value;
    }
    assert([refundDeadline isEqualToString:policy[1]]);
    return 0;
}
