# ThinkThen for Swift

This SwiftPM source package calls the ThinkThen C library. The native library is a separate install. The checked product source runs on Swift 6.4 and Ubuntu 24.04 x86_64. macOS and other targets have not passed this package gate.

From a ThinkThen checkout, build the matching native library and the Swift package:

```sh
cargo build --locked --offline --manifest-path libraries/c/Cargo.toml --lib
mkdir -p /tmp/thinkthen-native/lib
cp libraries/c/target/debug/libthinkthen_c.so /tmp/thinkthen-native/lib/libthinkthen.so
ln -s libthinkthen.so /tmp/thinkthen-native/lib/libthinkthen.so.0
cp libraries/c/include/thinkthen.h libraries/swift/Sources/CThinkThen/include/
swift build --package-path libraries/swift -Xlinker -L -Xlinker /tmp/thinkthen-native/lib -Xlinker -rpath -Xlinker /tmp/thinkthen-native/lib
```

The C header has one source, `libraries/c/include/thinkthen.h`. SwiftPM runs no copy step for a system library, so copy it in before building from a checkout; `check.sh` and the release archive do this. Use `.package(path: "/absolute/path/to/thinkthen/libraries/swift")` and `.product(name: "ThinkThen", package: "ThinkThen")` in a SwiftPM consumer. Give that consumer the same native library search and runtime path. `Examples/main.swift` shows `Engine.decide`, the generic JSON door, and a cancelled token. Configure `THINKTHEN_BASE_URL` and `THINKTHEN_API_KEY` only when intentionally running the example against a backend. The product gate uses a synthetic loopback backend and sends no provider request.

`Engine()` reads ordinary environment settings. `Engine(settingsJSON:)` passes a JSON settings object to the C constructor; explicit `base_url` and `cache` settings take precedence. Invalid settings fail before a request. `Engine.call` returns the C success JSON envelope with `value` and `facts`; the caller can decode it with Foundation. The four typed methods return `CallResult` with the former value in `.value` and the call's facts in `.facts`, both from the same native operation. Facts, and the `recognize` and `relate` values, are JSON text; decode them with Foundation and ignore members you do not know. For example, `engine.decide("Is it?", text).value.outcome` reads the former answer. `decide`, `decideMany`, `call`, `recognize` and `relate` each take `deadline:` in milliseconds and a cancel `token:`. `AnnotatedField.read` takes one annotate answer member decoded by Foundation and returns `.unresolved` for null, `.failed(kind:cause:)` for a `{"failed": ...}` member, or `.answered` with the value. `Engine.plan(verb, question, input, settings:)` previews a decide, choose, score or tag call through `thinkthen_plan_json` and returns the result schema's `plan` object as JSON text; it needs no key and sends nothing. Question text that starts with `{` is a question object, as `decide` reads it. `max_requests_total` in `Engine(settingsJSON:)` caps the process's live sends. A valid unresolved answer is distinct from a thrown `DoorFailure`. `DoorFailure.kind` names usage, backend, deadline, local, cancelled or defect, and `factsJSON` copies the borrowed native failure facts when a started call provides them. Swift strings for questions and JSON requests reject interior NUL; evidence remains byte-counted. `CancelToken()` throws if native allocation fails. Join all calls using an engine or token before releasing either owner, then call `Engine.close()` once.

`sh libraries/swift/check.sh` builds the current C library offline, compares its installed exports with the current header, runs the public J1 corpus and exact request matrix, then builds two isolated source/native archive consumers. From a clean committed Linux x86-64 checkout, `sdlc/scripts/release-pack x86_64-unknown-linux-gnu OUT c swift zig` also makes a local checksummed Swift source archive with a matching C archive. `sdlc/scripts/release-go-cpp-pair OUT swift-zig` validates the three-file set. Passing its Swift and C archive paths as `THINKTHEN_ARTIFACT` and `THINKTHEN_C_ARTIFACT` to `check.sh` builds the unpacked SwiftPM package and runs a three-request public bulk consumer from those archive bytes. These local artifacts are disposable. Each GitHub release ships the checksummed Swift source archive and the matching C archive for Linux x86-64. No package registry holds the Swift package, and SwiftPM neither embeds nor downloads a native archive.

`Engine(settingsJSON:)` accepts `{"backend":"local"}` to select the `local` entry in the read-only ThinkThen configuration. Use `{"base_url":"http://localhost:11434/v1"}` for a direct address instead. A named backend supplies its address, model, wire settings and key environment variable; explicit constructor settings take precedence. Omitting `backend` preserves ordinary environment/default selection. A missing or invalid name fails before sending.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.

Typed 0.2 host descriptors are available through `QuestionInput/Requests`. Ten named request builders prepare explicit questions/question files and record/file/image sources; known answer, metadata, fact, identity, location, span and edge fields have typed carriers. This is independent carrier preparation, not complete-call runtime parity. The host descriptor codec is not the native wire format. Existing engine calls remain the executable compatibility API. Adoption of the real 0426 constructors, complete calls, result views and failure snapshots is still required; no legacy result is promoted to result/2.


## Complete native calls (0428 integration)

The additive complete API uses C's native question grammar and readers for all ten named functions. Bare and generic JSON calls remain available. Known result fields are typed; arbitrary original JSON is retained as counted content.

Question input is explicit: construct a typed descriptor, parse saved JSON with a loader role, load an exact file, load a configured name, or load a reference. The eight roles are atomic, question set, per-record choose, recognize, record relate, rank, rank set and find. A string is never guessed to be a path. Native declarations, named/versioned authors, record context, options, rank members, observations, answer/request/call IDs, final facts and owned failure snapshots remain distinct.

Records may carry explicit text or JSON, per-record context and ordered candidate replacements. Images use explicit PNG/JPEG media and preserve original compressed bytes, order and duplicates. Decide, choose and score admit native image routes; the other seven functions refuse before sending. File sources explicitly select line, window, whole file, image file or JSONL. Physical filename/line ranges and absent image line coordinates come from the native reader.

Complete results copy all borrowed native views before `thinkthen_result_free`. They survive destruction of the engine, question, source and input buffers. Presence remains explicit, including absent aggregate IDs/meta, nullable selections and independently unknown reported token dimensions. Cost strings are copied without floating-point conversion. Failures retain the six native kinds, safe messages, available stop details and final facts/attempts.

Decide, choose, tag, score, filter and annotate also have owned lazy native batches. Start/next/facts/free belong to the creating thread; keep the engine live until the batch closes. Starting clones question/source/context/cancellation state. Returned rows own independent snapshots. Rank, find, recognize and relate retain aggregate complete calls. No host parser, cache, scheduler or model-routing policy is added.

This is an unpublished integration API. The family ticket and root review own final qualification; the shared consumer reports actual failures instead of counting generic JSON as typed parity.

`Engine` has typed `question`, `parseQuestion`, `loadQuestion`, `namedQuestion`, `referenceQuestion`, `records`, `fileSource` and `image` constructors. `NativeRole`, `NativeUnit`, `NativeInput`, `NativeRecord`, `NativeControls` and call-scoped `NativeBuffers` keep selections explicit. All ten overloads accept `NativeQuestion` plus `source: NativeSource` and return `NativeResult`. Its ten getters and metadata arrays contain Swift-owned values. `NativeFailure` owns its complete typed result snapshot and exposes `kind`. The six `FUNCTIONBatch` overloads return `NativeLazyBatch`; `next`, `facts` and `close` must stay on its creating thread. Active native batches retain engine ownership even if the caller closes the engine facade.

Caller-supplied counted option/question descriptors borrow their buffers through construction only; keep their `NativeBuffers` live until construction returns. Native record/image constructors clone those buffers and compressed bytes.
