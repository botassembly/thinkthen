#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    TTQuestionFile *kinds = [TTQuestionFile new];
    kinds.path = @"kinds.json";
    TTTextInput *text = [TTTextInput new];
    text.text = @"Sam Patel says his Kindle "
            @"arrived broken in Denver.";
    NSMutableArray *names = [NSMutableArray array];
    TTPresence<NSArray<TTRecognition *> *> *judgment =
        [client recognizeAnswer:kinds
        input:text
        options:nil feed:nil call:NULL error:&error];
    assert(judgment && !error);
    for (TTRecognition *row in judgment.value) {
        TTRecognizeFieldsEntities *facts =
        (id)row.value.value;
        for (TTEntity *one in facts.entities.value) {
            [names addObject:[NSString stringWithFormat:
                @"%@ (%@)", one.text.value,
                one.kind.value]];
        }
    }
    NSArray *expected = @[
        @"Sam Patel (person)",
        @"Kindle (product)",
        @"Denver (place)",
    ];
    assert([names isEqualToArray:expected]);
    return 0;
} }
