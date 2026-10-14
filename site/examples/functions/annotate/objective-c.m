#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    TTQuestionFile *form = [TTQuestionFile new];
    form.path = @"form.json";
    TTTextInput *report = [TTTextInput new];
    report.text = @"Steps: click Log in. Nobody gets in.";
    NSDictionary *triage;
    TTCall *judgment =
        [client annotate:form
        input:report
        options:nil feed:nil error:&error];
    assert(judgment && !error);
    triage = [judgment.value scalar];
    assert([triage[@"steps"] isEqual:@YES]);
    assert([triage[@"area"] isEqual:@"login"]);
    assert([triage[@"impact"] isEqual:@1.98]);
    return 0;
} }
