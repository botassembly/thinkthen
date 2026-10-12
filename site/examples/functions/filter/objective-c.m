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
        @"text": @"Is this a complaint?",
    };
    NSArray<NSString *> *reviews = @[
        @"Arrived a day early. Thank you!",
        @"The zipper broke the first time I used it.",
        @"Does this come in blue?",
        @"The strap snapped on day two.",
    ];
    NSMutableArray *records = [NSMutableArray array];
    for (NSString *one in reviews) {
        [records addObject:@{
            @"original": @{@"kind": @"text", @"text": one},
        }];
    }
    NSDictionary *input = @{
        @"kind": @"records",
        @"items": records,
    };
    NSMutableArray *complaints = [NSMutableArray array];
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client filter:question
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

    Class wanted = TTSessionPacketFilterRow.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketFilterRow *found = packet;
        TTAtomicBoolean *row = found.value.value;
        if (row.value.value.boolValue) {
            int at = row.index.value.intValue;
            [complaints addObject:reviews[at]];
        }
    }
    NSArray *expected = @[reviews[1], reviews[3]];
    assert([complaints isEqualToArray:expected]);
    return 0;
}
