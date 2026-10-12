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
        @"path": @"makes.json",
    };
    NSArray *names = @[
        @[@"Apple", @"company"],
        @[@"Amazon", @"company"],
        @[@"iPhone", @"product"],
        @[@"Kindle", @"product"],
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
    NSMutableArray *whoMakes = [NSMutableArray array];
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
            TTResultNode *company = edge.source.value;
            TTResultNode *product = edge.target.value;
            NSString *makes = [NSString stringWithFormat:
                @"%@ makes %@",
                company.rawFields[@"name"],
                product.rawFields[@"name"]];
            [whoMakes addObject:makes];
        }
    }
    NSArray *expected = @[
        @"Apple makes iPhone",
        @"Amazon makes Kindle",
    ];
    assert([whoMakes isEqualToArray:expected]);
    return 0;
}
