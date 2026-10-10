# ThinkThen for Swift

The development API uses one typed `Client` family for `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`. Each call is `async throws` and returns an owned `OwnedCall` with generated packet types and the actual native terminal. Rust owns admission, files, image routes, cache, replay, execution and failure facts.

```swift
import ThinkThen

let client = try Client()
defer { client.close() }
let call = try await client.decide(.text("Does the writer request a refund?"),
                                   input: .text("Please refund my order."))
for packet in call.packets {
    if case .decideRow(let row) = packet {
        print(row.value.value)
    }
}
```

Use ordinary generated Swift inputs. `InputEngineSettings` configures the client. `InputRequestQuestion` selects literal text, a typed definition, an exact file, a configured name or an explicit reference. Strings supplied through `.text` remain literal text. `client.parseQuestion(.atomic, authored:)` admits an authored `JSONValue` through the native parser and returns its generated definition; parsing preserves authored field order and numeric spelling. `client.plan` accepts the same generated `InputRequest` and returns `OwnedPlan` without sending a request.

`InputRequestInput` selects text, JSON, records, units, entities, native sources or a bounded feed. `InputRequestInput.files` selects explicit paths and optional generated reading settings. Native source inputs perform file reads and retain provenance in Rust. Images use `.bytes(data, media:)` or `.file(path, media:)`; Swift only encodes supplied bytes for transport. Ordered arrays retain duplicates. Rust decides which functions and routes admit images and returns its typed failure for refusals.

Generated result fields include every known answer, question, observation, location, identifier, usage dimension and failure fact. `Presence<T>` distinguishes `.absent`, `.null` and `.value`; `.value` is an optional convenience and `.isPresent` remains explicit. Unknown nested output members survive `.json.data()` round trips. `JSONValue` retains arbitrary JSON number tokens without floating-point conversion, and parsed objects retain encounter order and duplicates. Generated values are immutable and own their parsed source independently of native buffers.

A native terminal failure throws `SessionFailure`, retaining its generated `OwnedCallError`, retryability, actual final facts and completed packets. Immediate admission or ownership failures throw `SessionBoundaryFailure` with the native code and copied safe diagnostic. Unresolved answers are successful result values. Caller task cancellation throws `CancellationError`; it does not fabricate a terminal or final facts.

`Session` is the bounded advanced view of this same family. `client.start(request)` admits a generated request. One producer uses `await session.push(descriptor)` and explicit `finish`; one reader uses `await session.read()`. A full queue retries the same descriptor before advancing intake. `client.execute(request, feed:)` runs an `AsyncThrowingStream` producer concurrently with output. Throw `FeedReadFailure` with a generated IO, UTF-8 or invalid-input failure to preserve the admitted prefix and report native reader failure. Consumer cancellation cancels the producer and detaches native ownership; caller stream termination code remains the caller's responsibility.

`Session.cancel()` signals cancellation and permits a later explicit drain from a fresh task. `Session.close()` detaches the native owner without waiting for a blocked provider. Short synchronized leases keep each native operation alive through its immediate copy; no borrowed pointer crosses an `await`. `Client.close()` releases its facade while already admitted sessions retain their native engine. Use `defer` for deterministic release. `usagePersistence()` and `finishUsageStatus()` return an owned native persistence observation, separate from historical call facts. Finalization can perform filesystem work and should run on a caller-selected background executor when needed.

## Package

The [approved package design](../../sdlc/decisions/2026-10-09-native-package-design.md) supplies the runtime floors and target pairs. A trusted versioned distribution contains this SwiftPM package and its matching native asset. Add its versioned source-control dependency and `.product(name: "ThinkThen", package: "ThinkThen")` in a consumer. Installation requires no native path variable or consumer Rust build.

On Apple, `Package.swift` selects the C-only static `CThinkThen.xcframework` beside the manifest and sets macOS 15. `build-xcframework.sh` assembles its universal macOS slice from matching prebuilt x86-64 and arm64 C package directories using Apple tools. It builds no Rust code. The Swift facade remains source. Actual Apple assembly, native dependency inspection and installed consumer qualification require an Apple machine.

On Linux, the Swift target carries `Native/<rust-target>/libthinkthen.so` as a copied SwiftPM resource. Generated C forwarders preserve compiler-derived signatures and remain hidden from native symbol lookup. The loader opens only the absolute bundle asset path and resolves the entire table once before any constructor calls native code. Engines, sessions and results free normally; the successful library handle remains until process exit because workers can outlive their Swift owners. Missing or unloadable assets throw a fixed local failure. Deploy the SwiftPM resource bundle with the executable: SwiftPM's generated `Bundle.module` accessor traps if the whole bundle is absent. The source checkout has no bundled binary; use an assembled trusted package. Linux support covers the declared glibc targets. Routine versioned Linux SwiftPM consumption is checked locally; final distribution contents and Apple linkage are qualified at the candidate.

## Compatibility and checks

The old `Engine`, JSON-string calls and handwritten `Native*` complete readers have been removed. New callers use `Client` and the generated `Input*` and `Owned*` values. Old `Engine.decide(question, text)` maps to `await client.decide(.text(question), input: .text(text))`; old `Engine.call` maps to a named typed function; old `NativeQuestion` and `NativeSource` map to `InputRequestQuestion` and `InputRequestInput`; old failure snapshots map to `SessionFailure.call`. Frozen C compatibility remains separate.

`generate.py --target swift --check`, repeated with `--inputs` and `--bridge`, compares generated results, inputs and compiler-derived ABI enums. `Tests/fixtures/owned_consumer.swift` checks typed success and failure, presence, unknown numeric fields, independent task progress and cancellation/cleanup while a provider remains held. The routine checker builds a consumer with an exact versioned SwiftPM dependency, loads the bundled Linux asset and runs selected shared cases through the installed typed API. `THINKTHEN_TEST_PROFILE=full` runs every required shared case at a candidate. Apple linkage, alternate targets, large-input behavior and release qualification remain candidate checks. The ordinary published installation remains the released version until publication is authorized.
