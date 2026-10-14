#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
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
    TTAuthoredScoreBuilder *definition =
        [TTAuthoredScoreBuilder new];
    definition.score = question;
    definition.levels = levels;
    TTQuestionDefinition *howUrgent =
        [TTQuestionDefinition new];
    howUrgent.value = definition;
    TTTextInput *outage = [TTTextInput new];
    outage.text = @"Our checkout page is down and "
            @"customers cannot pay.\n";
    double urgency = -1;
    TTCall *judgment =
        [client score:howUrgent
        input:outage
        options:nil feed:nil error:&error];
    assert(judgment && !error);
    urgency = judgment.value.scalar.doubleValue;
    assert(urgency == 2.0);
    return 0;
} }
