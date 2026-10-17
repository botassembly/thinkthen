#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    TTTextQuestion *question = [TTTextQuestion new];
    question.text =
        @"Which line gives the refund deadline?";
    NSArray<NSString *> *policy = @[
        @"Returns need the original receipt.",
        @"Refunds are issued within 30 days of purchase.",
        @"Shipping is free on orders over $50.",
        @"Gift cards cannot be exchanged for cash.",
    ];
    NSMutableArray *records = [NSMutableArray array];
    for (NSString *one in policy) {
        TTOriginalText *original = [TTOriginalText new];
        original.text = one;
        TTItem *item = [TTItem new];
        item.original = original;
        [records addObject:item];
    }
    TTRecordsInput *input = [TTRecordsInput new];
    input.items = records;
    NSString *refundDeadline = nil;
    TTCall *judgment =
        [client find:question
        input:input
        options:nil feed:nil error:&error];
    assert(judgment && !error);
    refundDeadline = [judgment.value scalar];
    assert([refundDeadline isEqualToString:policy[1]]);
    return 0;
} }
