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
        @"text": @"Does the customer ask for a refund?",
    };
    NSDictionary *broken = @{
        @"kind": @"text",
        @"text": @"Please refund my order. "
            @"It arrived broken.",
    };
    BOOL isRefund = NO;
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client decide:question
        input:broken
        options:nil
        feed:nil
        completion:^(TTCall *call, NSError *failure) {
            answered = call;
            dispatch_semaphore_signal(done);
        }
        error:&error];
    assert(task);
    dispatch_semaphore_wait(done, DISPATCH_TIME_FOREVER);

    Class wanted = TTSessionPacketDecideRow.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketDecideRow *found = packet;
        TTAtomicDecideValue *row = found.value.value;
        isRefund = [row.value.value isEqual:@YES];
    }
    assert(isRefund);
    return 0;
}
