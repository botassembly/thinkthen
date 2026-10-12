#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    NSDictionary *form = @{
        @"kind": @"file",
        @"path": @"form.json",
    };
    NSDictionary *report = @{
        @"kind": @"text",
        @"text": @"Steps: click Log in. Nobody gets in.",
    };
    NSDictionary<NSString *, TTAnnotatedField *> *triage;
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client annotate:form
        input:report
        options:nil
        feed:nil
        completion:^(TTCall *call, NSError *failure) {
            answered = call;
            dispatch_semaphore_signal(done);
        }
        error:&error];
    assert(task);
    dispatch_semaphore_wait(done, DISPATCH_TIME_FOREVER);

    Class wanted = TTSessionPacketAnnotateRow.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketAnnotateRow *found = packet;
        triage = found.value.value.value.value;
    }
    assert([triage[@"steps"].value isEqual:@YES]);
    assert([triage[@"area"].value isEqual:@"login"]);
    assert([triage[@"impact"].value isEqual:@1.98]);
    return 0;
}
