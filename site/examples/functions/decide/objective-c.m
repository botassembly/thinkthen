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
    TTCall *judgment =
        [client decide:question
        input:broken
        options:nil feed:nil error:&error];
    assert(judgment && !error);
    isRefund = [[judgment.value scalar] boolValue];
    NSNumber *refundProbability =
        judgment.probability.scalar;
    assert(isRefund && refundProbability);
    return 0;
} }
