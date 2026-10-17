#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    TTQuestionFile *whichTeam = [TTQuestionFile new];
    whichTeam.path = @"teams.json";
    TTTextInput *parcel = [TTTextInput new];
    parcel.text = @"My parcel went to the wrong address.";
    NSString *team = nil;
    TTCall *judgment =
        [client choose:whichTeam
        input:parcel
        options:nil feed:nil error:&error];
    assert(judgment && !error);
    team = [judgment.value scalar];
    assert([team isEqualToString:@"shipping"]);
    return 0;
} }
