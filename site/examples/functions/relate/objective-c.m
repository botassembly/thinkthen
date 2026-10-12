#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    NSDictionary *relations = @{
        @"kind": @"file",
        @"path": @"sings.json",
    };
    NSArray *names = @[
        @[@"Paul McCartney", @"singer"],
        @[@"Ringo Starr", @"singer"],
        @[@"Yesterday", @"song"],
        @[@"Octopus's Garden", @"song"],
    ];
    NSMutableArray *entities = [NSMutableArray array];
    for (NSArray *name in names) {
        [entities addObject:@{@"original": @{
            @"kind": @"json",
            @"value": @{@"name": name[0], @"kind": name[1]},
        }}];
    }
    NSDictionary *input = @{
        @"kind": @"entities",
        @"items": entities,
    };
    NSMutableArray *whoSings = [NSMutableArray array];
    __block TTCall *answered = nil;
    dispatch_semaphore_t done =
        dispatch_semaphore_create(0);
    TTTask *task = [client relate:relations
        input:input
        options:nil
        feed:nil
        completion:^(TTCall *call, NSError *failure) {
            answered = call;
            dispatch_semaphore_signal(done);
        }
        error:&error];
    assert(task);
    dispatch_semaphore_wait(done, DISPATCH_TIME_FOREVER);

    Class wanted = TTSessionPacketRelateAggregate.class;
    for (id packet in answered.packets) {
        if (![packet isKindOfClass:wanted]) continue;
        TTSessionPacketRelateAggregate *found = packet;
        NSArray *edges = found.value.value.value.value;
        for (TTRelatedEntityEdge *edge in edges) {
            TTResultNode *singer = edge.source.value;
            TTResultNode *song = edge.target.value;
            NSString *sings = [NSString stringWithFormat:
                @"%@ sings %@",
                singer.rawFields[@"name"],
                song.rawFields[@"name"]];
            [whoSings addObject:sings];
        }
    }
    NSArray *expected = @[
        @"Paul McCartney sings Yesterday",
        @"Ringo Starr sings Octopus's Garden",
    ];
    assert([whoSings isEqualToArray:expected]);
    return 0;
}
