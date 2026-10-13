#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    NSDictionary *whichTeam = @{
        @"kind": @"file",
        @"path": @"teams.json",
    };
    NSDictionary *parcel = @{
        @"kind": @"text",
        @"text": @"My parcel went to the wrong address.",
    };
    NSString *team = nil;
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client choose:whichTeam
        input:parcel
        options:nil
        feed:nil
        completion:^(TTCall *call, NSError *failure) {
            answered = call;
            dispatch_semaphore_signal(done);
        }
        error:&error];
    assert(task);
    dispatch_semaphore_wait(done, DISPATCH_TIME_FOREVER);

    Class wanted = TTSessionPacketChooseRow.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketChooseRow *found = packet;
        team = found.value.value.value.value;
    }
    assert([team isEqualToString:@"shipping"]);
    return 0;
}
