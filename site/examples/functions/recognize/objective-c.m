#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    NSDictionary *kinds = @{
        @"kind": @"file",
        @"path": @"kinds.json",
    };
    NSDictionary *text = @{
        @"kind": @"text",
        @"text": @"Sam Patel says his Kindle "
            @"arrived broken in Denver.",
    };
    NSMutableArray *names = [NSMutableArray array];
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client recognize:kinds
        input:text
        options:nil
        feed:nil
        completion:^(TTCall *call, NSError *failure) {
            answered = call;
            dispatch_semaphore_signal(done);
        }
        error:&error];
    assert(task);
    dispatch_semaphore_wait(done, DISPATCH_TIME_FOREVER);

    Class wanted = TTSessionPacketRecognizeAggregate.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketRecognizeAggregate *found = packet;
        for (TTRecognition *row in found.value.value) {
            TTRecognize *value = row.value.value;
            TTRecognizeFieldsEntities *facts = (id)value;
            for (TTEntity *one in facts.entities.value) {
                NSString *name = [NSString stringWithFormat:
                    @"%@ (%@)",
                    one.text.value,
                    one.kind.value];
                [names addObject:name];
            }
        }
    }
    NSArray *expected = @[
        @"Sam Patel (person)",
        @"Kindle (product)",
        @"Denver (place)",
    ];
    assert([names isEqualToArray:expected]);
    return 0;
}
