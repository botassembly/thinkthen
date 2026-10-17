#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    TTTextQuestion *question = [TTTextQuestion new];
    question.text = @"Is this urgent?";
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
        TTOriginalText *original = [TTOriginalText new];
        original.text = one;
        TTItem *item = [TTItem new];
        item.original = original;
        [records addObject:item];
    }
    TTRecordsInput *input = [TTRecordsInput new];
    input.items = records;
    NSMutableArray *byUrgency =
        [@[@0, @0, @0, @0] mutableCopy];
    TTCall *judgment =
        [client rank:question
        input:input
        options:nil feed:nil error:&error];
    assert(judgment && !error);
    byUrgency = [judgment.value.items mutableCopy];
    NSArray *expected = @[@1, @3, @2, @0];
    assert([byUrgency isEqualToArray:expected]);
    return 0;
} }
