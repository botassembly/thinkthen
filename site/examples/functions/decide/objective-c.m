#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    TTTextQuestion *question = [TTTextQuestion new];
    question.text = @"Does the customer ask for a refund?";
    TTTextInput *broken = [TTTextInput new];
    broken.text = @"Please refund my order. "
            @"It arrived broken.";
    BOOL isRefund = NO;
    TTPresence<NSNumber *> *judgment =
        [client decideAnswer:question
        input:broken
        options:nil feed:nil call:NULL error:&error];
    assert(judgment && !error);
    isRefund = judgment.value.boolValue;
    assert(isRefund);
    return 0;
} }
