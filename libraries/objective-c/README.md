# ThinkThen for Objective-C

Call a System One model from Objective-C and read its answer through a typed call. The ten functions are `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`.

```objc
#import "ThinkThenFoundation.h"
#include <assert.h>

int main(void) { @autoreleasepool {
    NSError *error = nil;
    TTFoundationClient *client = [[TTFoundationClient alloc]
        initWithSettings:nil
        error:&error];
    assert(client);
    TTTextQuestion *question = [TTTextQuestion new];
    question.text = @"Does the customer ask for a refund?";
    TTTextInput *broken = [TTTextInput new];
    broken.text = @"Please refund my order. "
            @"It arrived broken.";
    BOOL isRefund = NO;
    TTCall *judgment =
        [client decide:question
        input:broken
        options:nil feed:nil error:&error];
    assert(judgment && !error);
    isRefund = [[judgment.value scalar] boolValue];
    NSNumber *refundProbability =
        judgment.probability.scalar;
    assert(isRefund);
    assert(refundProbability.doubleValue == 0.99);
    return 0;
} }
```

`call.value` is a `TTCallValue`: `scalar` holds one answer and `items` holds ordered answers for record inputs. Scalars preserve authored Foundation values, including `NSNull`, booleans and precise integers. Read an authored leaf with message syntax, such as `[call.value scalar]`. A nil decision or choice means not sure. A failure returns an error instead of an answer.

`call.probability` is a `TTCallProbability`: `scalar` is an `NSNumber`, and `items` contains typed probability wrappers for records. A score's probability differs from its numerical value. `call.results` retains the complete typed result records; `call.facts` retains final usage, counts and model information. Generated `TTPresence` fields distinguish missing, explicit null and value. These immutable results survive native owner release.

`TTTextQuestion`, `TTQuestionFile`, `TTQuestionName` and `TTQuestionReference` select questions. `TTQuestionDefinition` accepts authored builders. `TTOrderedMap` preserves authored member order. `TTItem` builds records, `TTFiles` selects native file readers, and `TTImageBytes(data, media)` builds a byte attachment. The engine checks image admission. Arbitrary authored leaves accept ordinary Foundation values.

## Install

Unpack the matching [release archive](https://github.com/botassembly/thinkthen/releases). Add its SwiftPM package and use the `ThinkThenFoundation` product. The archive includes `CThinkThen.xcframework`; consumers need no Rust compiler or native library path. The library requires Apple Foundation, ARC and blocks on macOS 15 or later.

Set a hosted key with `THINKTHEN_API_KEY`. [Backend setup](https://thinkthen.dev/install/backends/) covers Jev, SystemOne, OpenAI and local models. The library is MIT licensed. Hosted models charge per token; local models use your computer.

## Advanced calls

`TTSettings` configures the client; `TTControls` selects call options. `plan:error:` builds a typed plan without sending. Blocking calls wait on the calling thread. Asynchronous calls take a completion block and return a cancellable task. Completion runs on a private serial queue; dispatch UI changes to the main queue.

A `TTFeed` supplies one descriptor at a time, EOF, or a typed reader failure. `TTSession` exposes push, finish, read, cancel and close for caller-owned scheduling. `TTFailureError` preserves the native error, retryability and available final facts. `usagePersistence:` observes usage storage; `finishUsageStatus:` finishes pending writes.

## Build and check

`python3 build-tools/results/generate.py --target objc --check` checks generated freshness. On macOS, `check.sh` installs an assembled package into an unrelated SwiftPM consumer and checks typed answers, ownership, cancellation and saved responses. `THINKTHEN_TEST_PROFILE=full` selects the full shared inventory.
