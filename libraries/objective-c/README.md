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
    isRefund = [judgment.value.value boolValue];
    NSNumber *refundProbability =
        judgment.probability.value;
    assert(isRefund);
    assert(refundProbability.doubleValue == 0.99);
    return 0;
} }
```

`call.value`, `call.probability`, `call.pick`, `call.index` and `call.resolution` read the final answer fields returned by Rust. Each uses `TTPresence` to preserve missing, null and value separately. A false answer remains a value. Check `state` before reading `value`; explicit null represents `TTPresenceNull`.

`call.answer` retains the generated typed answer. `call.results` retains the complete native receipts, and `call.facts` retains final usage, counts and model information. These immutable results survive native owner release. A failed call returns `TTFailureError` instead of an answer.

`TTTextQuestion`, `TTQuestionFile`, `TTQuestionName` and `TTQuestionReference` select questions. `TTQuestionDefinition` accepts authored builders. `TTOrderedMap` preserves authored member order. `TTItem` builds records, `TTFiles` selects native file readers, and `TTImageBytes(data, media)` builds a byte attachment. The engine checks image admission. Arbitrary authored leaves accept ordinary Foundation values.

## Install

Unpack the matching [release archive](https://github.com/botassembly/thinkthen/releases). Add its SwiftPM package and use the `ThinkThenFoundation` product. The archive includes `CThinkThen.xcframework`; consumers need no Rust compiler or native library path. The library requires Apple Foundation, ARC and blocks on macOS 15 or later.

Set a hosted key with `THINKTHEN_API_KEY`. [Backend setup](https://thinkthen.dev/install/backends/) covers Jev, SystemOne, OpenAI and local models. The library is MIT licensed. Hosted models charge per token; local models use your computer.

## Advanced calls

`TTSettings` configures the client; `TTControls` selects call options. `plan:error:` builds a typed plan without sending. Blocking calls wait on the calling thread. Asynchronous calls take a completion block and return a cancellable task. Completion runs on the dispatch worker; dispatch UI changes to the main queue.

A `TTFeed` supplies one descriptor at a time, EOF, or a typed reader failure on the calling worker. Cancellation signals the native token and completion waits for native cleanup. `TTFailureError` preserves the native error, retryability and available final facts. `usagePersistence:` observes usage storage; `finishUsageStatus:` finishes pending writes.

## Build and check

`python3 build-tools/results/generate.py --target objc --check` checks generated freshness. On macOS, `check.sh` installs an assembled package into an unrelated SwiftPM consumer and checks typed answers, ownership, cancellation and saved responses. `THINKTHEN_TEST_PROFILE=full` selects the full shared inventory.
