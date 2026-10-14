#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    NSString *question = @"Which labels fit this message?";
    NSArray *labels = @[@"praise", @"bug", @"billing"];
    TTAuthoredTagBuilder *definition =
        [TTAuthoredTagBuilder new];
    definition.tag = question;
    definition.labels = labels;
    TTQuestionDefinition *whichLabels =
        [TTQuestionDefinition new];
    whichLabels.value = definition;
    TTTextInput *message = [TTTextInput new];
    message.text = @"Love the new dashboard, "
            @"but export crashes the app,\n"
            @"and I was charged twice.\n";
    NSArray<NSString *> *fittingLabels = nil;
    TTCall *judgment =
        [client tag:whichLabels
        input:message
        options:nil feed:nil error:&error];
    assert(judgment && !error);
    fittingLabels = [judgment.value scalar];
    assert([fittingLabels isEqualToArray:labels]);
    return 0;
} }
