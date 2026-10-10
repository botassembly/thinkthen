# ThinkThen Foundation

The 0.2 Objective-C API targets Apple Foundation, ARC and blocks. The package definition is [Package.swift](Package.swift). Distribution assembles the matching prebuilt `CThinkThen.xcframework` beside that manifest under the [Apple package design](../../sdlc/decisions/2026-10-09-native-package-design.md). A consuming SwiftPM project depends on `ThinkThenFoundation` and imports `ThinkThenFoundation.h`. It needs no separate C archive, Rust compiler or native library path.

The Foundation implementation lives in `Sources/Foundation`. Its generated result classes come from Rust's shared result graph. `TTPresence` distinguishes a missing member, explicit null and a value. Generated classes retain unknown members in `rawFields`. Failures carry the native typed `TTCallError` under `TTFailureDetailsKey` in `NSError.userInfo`, including retryability and final facts. Caller cancellation completes with a cancellation error while native final facts remain pending.

```objc
NSError *error = nil;
TTFoundationClient *client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
TTTask *task = [client decide:@{@"kind":@"text", @"text":@"Does this mention a refund?"}
    input:@{@"kind":@"text", @"text":@"Please refund my order."}
    options:nil feed:nil completion:^(TTCall *call, NSError *failure) {
        if (failure) { NSLog(@"%@", failure.localizedDescription); return; }
        NSLog(@"%@", call.terminal.facts.value.callId.value);
    } error:&error];
// Keep task when the caller needs cancellation.
[task cancel];
```

`decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate` share this named call family. Question selectors, input selectors and options use Foundation dictionaries, arrays, strings, numbers and `NSNull`. Rust owns their admission. `TTImageBytes(data, media)` accepts an `NSData` attachment and performs the internal base64 conversion. Authored JSON values remain ordinary Foundation values.

A feed supplies one descriptor at a time through `TTFeed`; its input selector names a native feed. The callback returns nil at EOF or nil with a native reader-failure dictionary. It runs on the task's private callback queue and must return promptly. A task keeps one descriptor while native intake is full and drains native output on the same timer. `TTSession` also exposes explicit nonblocking push, finish, read, cancel and close for callers that own their scheduling. ARC releases native owners. Completion runs on the private callback queue; dispatch UI changes to the main queue.

| Legacy call | Foundation call |
| --- | --- |
| `TTClient create`, `createWithSettings:length:failure:` | `TTFoundationClient initWithSettings:error:` |
| `decide:...answer:facts:failure:`, `many:...` | `decide:input:options:feed:completion:error:` |
| `recognize:...`, `relate:...` | Named `recognize` and `relate` with generated results |
| `json:...`, `files:...` | Named function with a Foundation input selector |
| `TTToken fire`, manual `dealloc`, `free`, `tt_json_free` | `TTTask cancel`, ARC |

GNU Objective-C support has ended. The legacy facade, handwritten JSON parser, copied native views and GNU consumers are removed. Only the Foundation API ships; Linux execution returns an explicit unsupported-platform result.

Run `python3 sdlc/generators/results/generate.py --target objc --check` for generated freshness. On macOS, `check.sh` installs the supplied assembled Apple package into an unrelated SwiftPM consumer and exercises packet ownership, ARC cleanup and held-provider cancellation. It refuses a missing XCFramework instead of starting a native build. The earlier Foundation implementation has accepted Apple SDK compilation and standalone result-ownership evidence. This retirement does not change that implementation. Current native linkage, installed session execution, cancellation and shared execution parity remain candidate qualification under [the migration ruling](../../sdlc/decisions/2026-10-10-drive-0-2-to-done.md); Linux checks cannot prove them. The installed consumers cover ownership and held-provider cancellation, then run the Objective-C shared inventory through all ten named calls. Routine selects the shared routine cases; `THINKTHEN_TEST_PROFILE=full` runs the full required inventory at a candidate. Those execution routes require macOS and have not run on this Linux host. Cancellation fixtures use the public explicit session to read final native facts after cancellation; the separate task consumer proves early callback settlement while those facts are pending. Shared assertions project the generated packet fields; direct generated accessors additionally check facts, decision values and recognition entities.

The Foundation client exposes `usagePersistence:` for a nonblocking observation and `finishUsageStatus:` to finish its current usage deltas. Each returns an immutable `TTUsagePersistenceStatus` with a native-header state and optional copied advice, or nil with a session diagnostic in `NSError`. Failed persistence is a returned state and preserves successful answers and call facts. Written covers current deltas only. Only usage-lock acquisition has a deadline; other filesystem work may take longer. Snapshots survive client release. Matching native Apple package execution remains required.
