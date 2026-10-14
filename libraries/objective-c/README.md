# ThinkThen Foundation

The Objective-C library targets Apple Foundation, ARC and blocks on macOS 15 or later. Its SwiftPM product is `ThinkThenFoundation`. Distribution assembles the matching `CThinkThen.xcframework` beside [Package.swift](Package.swift). Consumers import `ThinkThenFoundation.h` and need no Rust compiler or separate native library path.

```objc
NSError *error = nil;
TTFoundationClient *client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
TTTextQuestion *question = [TTTextQuestion new];
question.text = @"Does this mention a refund?";
TTTextInput *input = [TTTextInput new];
input.text = @"Please refund my order.";
TTCall *call = [client decide:question input:input options:nil feed:nil error:&error];
if (error) { NSLog(@"%@", error.localizedDescription); return; }
NSLog(@"%@", call.value.scalar);
```

`decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate` each offer a typed blocking `decideAnswer:input:options:feed:call:error:` helper and a named asynchronous call. The optional `call` output retains facts and packet readers. Blocking helpers run the bounded native wait on the calling thread and run feed blocks there. Each completed call has the matching `decideAnswer:` or other answer accessor and typed `facts`. Answer accessors refuse a failed call or the wrong function. Accessors other than filter require exactly one answer. Rank returns Foundation numbers for the native indices. Filter returns zero or more original Foundation values in source order, including `NSNull` and false. Recognize and relate answers use generated entity and relation classes. Relation endpoints expose `sourceName` and `targetName`.

`TTSettings` and `TTControls` expose the generated engine settings and request options. Question and input builders expose their native fields without transport dictionaries. `TTQuestionFile`, `TTQuestionName` and `TTQuestionReference` select saved questions. `TTQuestionDefinition` takes an authored definition, including `TTRecognizeDefinitionBuilder` and `TTAuthoredRelateBuilder`. `TTItem` builds records, `TTFiles` selects paths and reading options, and `TTImageBytes(data, media)` builds a typed byte attachment. Arbitrary authored JSON leaves accept ordinary Foundation values. `TTOrderedMap` preserves authored member order.

`plan:error:` takes a `TTRequest` containing a `TTCallArguments` and returns a typed `TTPlan`. Rust admits the request and builds the plan without sending. All nine plan fields remain available, including the required nullable `firstBodyUtf8`. Rust also admits named execution requests. Native failures return `TTFailureError`, which preserves the generated native error kind, retryability and optional typed `TTImmediateFacts`. Completed failures also expose the typed `TTCallError` through `failure` and under `TTFailureDetailsKey` in `NSError.userInfo`.

Results copy Rust's native typed graph into host-owned immutable Foundation values. The library never parses result JSON. `TTPresence` distinguishes missing, explicit null and value. Booleans remain distinct from numeric zero, and integer values retain their precision. Results and error facts survive native owner release.

Async callers use `decide:input:options:feed:completion:error:` and retain the returned task when they need cancellation. A `TTFeed` returns one `TTDescriptor` at a time, nil at EOF, or nil with a typed reader failure. Its callback runs on a private serial queue and must return promptly. A task retains one pending descriptor and drains bounded native output, then uses the native timed readiness wait. `TTSession` exposes typed push, finish, read, cancel and close for callers that own scheduling. Completion runs on the private callback queue; dispatch UI changes to the main queue. Cancellation settles the task before a held provider releases; final native facts may still be pending.

`usagePersistence:` observes usage persistence without waiting for its writer. `finishUsageStatus:` finishes current deltas. Each returns an immutable `TTUsagePersistenceStatus` with a state and optional advice. Failed persistence preserves successful answers and facts. Only usage-lock acquisition has a deadline.

Run `python3 sdlc/generators/results/generate.py --target objc --check` for generated freshness. On macOS, `check.sh` installs the supplied assembled Apple package into an unrelated SwiftPM consumer and checks result ownership, plan admission, refused requests, held-provider cancellation and the shared routine inventory through all ten named calls. `THINKTHEN_TEST_PROFILE=full` selects candidate-only execution coverage. The nonpublishing Objective-C workflow builds the matching native test dependency and runs these installed checks. Linux exits 77 because it cannot qualify Apple Foundation.
