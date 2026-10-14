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
    NSDictionary<NSString *, TTAnnotatedField *> *triage;
    TTPresence<NSArray<TTAnnotation *> *> *judgment =
        [client annotateAnswer:form
        input:report
        options:nil feed:nil call:NULL error:&error];
    assert(judgment && !error);
    triage = judgment.value[0].value.value;
    assert([triage[@"steps"].value isEqual:@YES]);
    assert([triage[@"area"].value isEqual:@"login"]);
    assert([triage[@"impact"].value isEqual:@1.98]);
    return 0;
} }
