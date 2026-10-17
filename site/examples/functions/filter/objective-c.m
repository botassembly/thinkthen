#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    TTTextQuestion *question = [TTTextQuestion new];
    question.text = @"Is this a complaint?";
    NSArray<NSString *> *reviews = @[
        @"Arrived a day early. Thank you!",
        @"The zipper broke the first time I used it.",
        @"Does this come in blue?",
        @"The strap snapped on day two.",
    ];
    NSMutableArray *records = [NSMutableArray array];
    for (NSString *one in reviews) {
        TTOriginalText *original = [TTOriginalText new];
        original.text = one;
        TTItem *item = [TTItem new];
        item.original = original;
        [records addObject:item];
    }
    TTRecordsInput *input = [TTRecordsInput new];
    input.items = records;
    NSMutableArray *complaints = [NSMutableArray array];
    TTCall *judgment = [client filter:question
        input:input
        options:nil feed:nil error:&error];
    assert(judgment && !error);
    [complaints addObjectsFromArray:judgment.value.items];
    NSArray *expected = @[reviews[1], reviews[3]];
    assert([complaints isEqualToArray:expected]);
    return 0;
} }
