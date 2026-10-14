# ThinkThen for Objective-C

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Install the matching archive: `tar -xzf thinkthen-objective-c-0.2.0-TARGET.tar.gz`. Replace `TARGET` with your platform.
Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```objc
#import "ThinkThenFoundation.h"
#include <stdio.h>
int main(void) { @autoreleasepool {
    NSError *error = nil; TTFoundationClient *client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
    if (!client) return 1;
    NSArray *refunds = @[@"Please refund my order. It arrived broken.", @"Thanks for the quick help yesterday!", @"I want to send this back."];
    NSArray *messages = @[@"Please refund the extra fee on my invoice.", @"My parcel went to the wrong address.", @"I cannot reset my password."];
    TTTextQuestion *refundQuestion = [TTTextQuestion new]; refundQuestion.text = @"Does the customer ask for a refund?";
    TTAuthoredChooseBuilder *teams = [TTAuthoredChooseBuilder new]; teams.choose = @"Which team owns this?";
    teams.options = [[TTOrderedMap alloc] initWithEntries:@[[[TTOrderedEntry alloc] initWithKey:@"billing" value:@"Invoices, fees, and refunds."], [[TTOrderedEntry alloc] initWithKey:@"shipping" value:@"Parcels and delivery."], [[TTOrderedEntry alloc] initWithKey:@"account" value:@"Logins and passwords."]]];
    TTQuestionDefinition *teamQuestion = [TTQuestionDefinition new]; teamQuestion.value = teams;
    for (NSString *text in refunds) {
        TTTextInput *input = [TTTextInput new]; input.text = text;
        TTCall *refund = [client decide:refundQuestion input:input options:nil feed:nil error:&error];
        if (!refund || error) return 1;
        puts(refund.value.scalar ? ([refund.value.scalar boolValue] ? "true" : "false") : "not sure");
    }
    for (NSString *text in messages) {
        TTTextInput *input = [TTTextInput new]; input.text = text;
        TTCall *team = [client choose:teamQuestion input:input options:nil feed:nil error:&error];
        if (!team || error) return 1;
        puts(team.value.scalar ? [team.value.scalar UTF8String] : "not sure");
    }
    return 0;
} }
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
