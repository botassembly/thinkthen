#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    NSString *question = @"How urgent is this?";
    NSArray *levels = @[
        @"Routine.",
        @"Soon.",
        @"Immediate.",
    ];
    NSDictionary *howUrgent = @{
        @"kind": @"definition",
        @"value": @{@"score": question, @"levels": levels},
    };
    NSDictionary *outage = @{
        @"kind": @"text",
        @"text": @"Our checkout page is down and "
            @"customers cannot pay.\n",
    };
    double urgency = -1;
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client score:howUrgent
        input:outage
        options:nil
        feed:nil
        completion:^(TTCall *call, NSError *failure) {
            answered = call;
            dispatch_semaphore_signal(done);
        }
        error:&error];
    assert(task);
    dispatch_semaphore_wait(done, DISPATCH_TIME_FOREVER);

    Class wanted = TTSessionPacketScoreRow.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketScoreRow *found = packet;
        TTAtomicDouble *row = found.value.value;
        urgency = row.value.value.doubleValue;
    }
    assert(urgency == 2.0);
    return 0;
}
