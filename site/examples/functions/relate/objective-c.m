#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    TTQuestionFile *relations = [TTQuestionFile new];
    relations.path = @"makes.json";
    NSArray *names = @[
        @[@"Apple", @"company"],
        @[@"Amazon", @"company"],
        @[@"iPhone", @"product"],
        @[@"Kindle", @"product"],
    ];
    NSMutableArray *entities = [NSMutableArray array];
    for (NSArray *name in names) {
        TTOriginalJSON *original = [TTOriginalJSON new];
        original.value =
        @{@"name": name[0], @"kind": name[1]};
        TTItem *item = [TTItem new];
        item.original = original;
        [entities addObject:item];
    }
    TTEntitiesInput *input = [TTEntitiesInput new];
    input.items = entities;
    NSMutableArray *whoMakes = [NSMutableArray array];
    TTPresence<TTRelation *> *judgment =
        [client relateAnswer:relations
        input:input
        options:nil feed:nil call:NULL error:&error];
    assert(judgment && !error);
    NSArray *edges = judgment.value.value.value;
    for (TTRelatedEntityEdge *edge in edges) {
        TTRelationEndpointRelatedEntity *company =
            (id)edge.source.value;
        TTRelationEndpointRelatedEntity *product =
            (id)edge.target.value;
        [whoMakes addObject:[NSString stringWithFormat:
            @"%@ makes %@", company.name.value,
                product.name.value]];
    }
    NSArray *expected = @[
        @"Apple makes iPhone",
        @"Amazon makes Kindle",
    ];
    assert([whoMakes isEqualToArray:expected]);
    return 0;
} }
