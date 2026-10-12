#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    NSString *question = @"Which labels fit this message?";
    NSArray *labels = @[@"praise", @"bug", @"billing"];
    NSDictionary *whichLabels = @{
        @"kind": @"definition",
        @"value": @{@"tag": question, @"labels": labels},
    };
    NSDictionary *message = @{
        @"kind": @"text",
        @"text": @"Love the new dashboard, "
            @"but export crashes the app,\n"
            @"and I was charged twice.\n",
    };
    NSArray<NSString *> *fittingLabels = nil;
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client tag:whichLabels
        input:message
        options:nil
        feed:nil
        completion:^(TTCall *call, NSError *failure) {
            answered = call;
            dispatch_semaphore_signal(done);
        }
        error:&error];
    assert(task);
    dispatch_semaphore_wait(done, DISPATCH_TIME_FOREVER);

    Class wanted = TTSessionPacketTagRow.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketTagRow *found = packet;
        fittingLabels = found.value.value.value.value;
    }
    assert([fittingLabels isEqualToArray:labels]);
    return 0;
}
