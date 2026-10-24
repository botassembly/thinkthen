# ThinkThen for Swift


This package exposes the ten functions through the shipped native engine.

## Install and ask

The release tag contains the `ThinkThen` library product and its native resources for Linux x86-64 and arm64, macOS arm64, and Windows x86-64. Add the release tag as a dependency:

```swift
.package(url: "https://github.com/botassembly/thinkthen", exact: "0.2.0")
```

The consuming package must declare `platforms: [.macOS("15.0")]` in its `Package` manifest to support macOS.

Add `.product(name: "ThinkThen", package: "ThinkThen")` to the consuming target. SwiftPM copies the bundled native resources, and the loader selects the engine for the host platform.

```swift
import Foundation
import ThinkThen

let client = try Client(settings: .init(adapter: "stub"))
let row = try await client.decide("Does the customer ask for money back?",
                                  record: .text("Please refund this purchase."))
if row.answer == true {
    print("Refund requested")
}
```

The default client uses the engine's default backend. Settings capture `mode`, `adapter`, `url`, `model`, `keySource` and numeric `batch`; omitted values retain engine defaults. Their JSON names follow the [client schema](../../schemas/client.schema.json). The same settings and methods serve embedded and future proxy calls. Swift implements no proxy transport.

## The ten functions

These examples run in an asynchronous throwing context. Questions accept string literals. JSON records retain owned raw UTF-8 bytes. A text record supplies raw UTF-8; a JSON string record supplies a structured JSON string.

```swift
let record: Record = .text("Ada works at Acme. Please refund the damaged order.")
let records: [Record] = [record, .text("The parcel arrived in good condition.")]

let decision = try await client.decide("Is a refund requested?", record: record)
let choice = try await client.choose("What is the topic?", record: record,
                                     options: ["billing", "shipping", "other"])
let tags = try await client.tag("Which labels apply?", record: record,
                                labels: ["refund", "damage"])
let score = try await client.score("How urgent is this?", record: record,
                                   levels: ["low", "medium", "high"])

let filtered = try client.filter("Is a refund requested?", records: records)
try await filtered.withRows { rows in
    for try await row in rows { print(row.answer as Any) }
}
let ranked = try client.rank("Which record best describes a damaged order?",
                         records: records, options: .init(top: 1))
try await ranked.withRows { rows in
    for try await row in rows { print(row.position, row.answer as Any) }
}

let found = try await client.find("Who works at Acme?", record: record)
let toolbox = JSON(utf8: Data(#"{"questions":{"refund":{"decide":"Is a refund requested?"},"topic":{"choose":"What is the topic?","options":["billing","shipping"]}}}"#.utf8))
let annotation = try await client.annotate(toolbox, record: record)
let names = try await client.recognize(.inline(try JSON.encoding(["person", "company"])),
                                      record: record)
let relations = try await client.relate(.inline(try JSON.encoding([
    ["name": "works_for", "source": "person", "target": "company"]
])), record: record)
```

`Question` is an immutable tagged value with `.inline(JSON)`, `.toolbox(JSON, name: String? = nil)` and `.reference(String)`. Its string-literal conformance produces inline wording. Libraries never interpret a question as a filename. `annotate` accepts a toolbox value directly and wraps its ordinary tagged question. No factory validates engine grammar.

The eight single-record functions also have overloads with `records:` returning `Rows<Answer>`. Every `records:` overload accepts either `[Record]` or a `Sendable` asynchronous sequence of records. The driver exclusively owns its iterator, which need not conform to `Sendable`. It creates and advances that iterator in one isolated source task; only a record or failure crosses back to the driver. It never runs two source advances concurrently. The array overload supplies a finite sequence through the same driver. No separate request builder or session is public.

| Function | Single-record return | Stream element |
| --- | --- | --- |
| decide | `Row<Bool>` | `Row<Bool>` |
| choose | `Row<String>` | `Row<String>` |
| tag | `Row<[String]>` | `Row<[String]>` |
| score | `Row<Double>` | `Row<Double>` |
| filter | stream only | `Row<Bool>` |
| rank | stream only | `Row<Int>` |
| find | `Row<TextSpan>` | `Row<TextSpan>` |
| annotate | `Row<JSON>` | `Row<JSON>` |
| recognize | `Row<[RecognizedName]>` | `Row<[RecognizedName]>` |
| relate | `Row<[RelationEdge]>` | `Row<[RelationEdge]>` |

Single-record overloads are `async throws`; stream construction throws only host encoding errors and returns a lazy `Rows` value and its iterator's `next()` is `async throws`. Choose's labels use the `options:` argument, tag's use `labels:`, and score's use `levels:`. Omit these domains when a saved toolbox question supplies them; an omitted argument sends no override. Their separate `callOptions:` argument holds the applicable call settings. Other functions use `options:`. `CallOptions` carries `threshold` and `on`; score and annotate take `EvidenceOptions` with only `on`. Rank takes `RankOptions` with `on`, optional `levels` and numeric `top`. Threshold stores a number or band string without checking it. Settings and options serialize once; Rust admits them. The [request schema](../../schemas/request.schema.json) owns the allowed fields and their meanings.

## Values and errors

`Row<Answer: Sendable>` is a hand-written immutable `Sendable` struct. Its public `let` properties are `id: JSON?`, `status: RowStatus`, `answer: Answer?`, `p: Double?`, `error: ThinkThenError?`, `position: Int` and `kept: Bool`. `RowStatus` has `answered`, `unsure` and `failed` cases. `answer == nil` follows the contract's null answer; `status` distinguishes uncertainty from failure. A missing identifier is nil; a present JSON null is a `JSON` value containing `null`. Rows survive their call and client.

`detail: JSON?` retains owned raw JSON without decoding. The existing C ABI `thinkthen_row_columns` projects identifiers, answers and errors into raw JSON strings inside a fixed envelope. Swift decodes that fixed envelope once, recovers the original JSON bytes from those strings, and retains opaque identifiers and annotate answers without a dictionary conversion. Known answer shapes decode from their separate bytes using Foundation `Codable`; the driver supplies each named function's answer conversion. The raw detail buffer already has its own ABI slot. No custom JSON parser or new ABI is needed. The native row and error buffers are copied and released once before Swift returns them. There is no shared mutable decode cache.

`TextSpan` has immutable text, start and end. `RecognizedName` adds kind and probability. `RelationEdge` holds immutable source and target names, relation, probability and optional either, following the [row schema](../../schemas/row.schema.json). Offsets use Unicode scalar positions through the existing C ABI code-point mode. They can index `String.unicodeScalars`; they are neither grapheme indexes nor `String.Index` values.

`JSON` is an immutable `Sendable` struct containing owned UTF-8 `Data`. Its minimal public API is:

```swift
public struct JSON: Sendable {
    public init(utf8: Data)
    public var utf8: Data { get }
    public static func encoding<T: Encodable>(_ value: T) throws -> JSON
    public func decode<T: Decodable>(_ type: T.Type) throws -> T
}
public enum Record: Sendable {
    case text(String)
    case json(JSON)
}
```

`JSON(utf8:)` copies the supplied bytes and does not parse or validate them. `utf8` returns a value with independent mutation semantics. `JSON.encoding` and `decode` are Foundation convenience conversions, with Foundation's original errors. The raw route preserves distinct JSON keys whose Swift strings compare as canonically equivalent, arbitrary NUL data keys and unsigned 64-bit values. The convenience view cannot promise those distinctions: Swift dictionaries and keyed Foundation decoding can merge canonically equivalent keys, and a caller's chosen numeric type can narrow a number. Only the raw bytes are authoritative. There is no custom parser, alternate dictionary key type or host validation. Raw question values are inserted unchanged into the tagged request's JSON framing; Swift encodes only its fixed field names and ordinary string/settings values. Rust alone admits the resulting request.

```swift
// These keys are distinct in JSON even though Swift String equality equates them.
let payload = JSON(utf8: Data(#"{"body":"evidence","id":{"\u00e9":1,"e\u0301":2,"a\u0000b":18446744073709551615}}"#.utf8))
let row = try await client.decide("Ready?", record: .json(payload))
let preservedID: Data? = row.id?.utf8
let preservedDetail: Data? = row.detail?.utf8
// Decode only when the application's chosen Swift type is a sufficient view.
let ordinaryFields = try row.id?.decode([String: UInt64].self)
```

The raw API preserves the engine's emitted JSON, including both distinct keys and exact integer values. It does not promise that the engine returns the caller's original whitespace or escape spelling.

`ThinkThenError: Error, Sendable` carries immutable `code` and `message` copied unchanged from the engine. Operation failures throw it. Failed stream rows retain it in `row.error`; the first failure is reported after all rows arrive. The driver does not select rows, sort rankings or recompute failures. Task cancellation reports Swift `CancellationError`; a native `local.cancelled` error without a cancelled caller task keeps its engine error. Native loading failures use a separate `NativeLoadError` with the loader's diagnostic; they invent no engine code.

## Pulling, cancellation and ownership

One internal driver handles every named function. It opens the shared C ABI call, sends one record after `NEEDS_INPUT`, closes input on exhaustion and drains rows. Initial records are never supplied twice. Input advances only when Rust requests it, and each iterator permits one outstanding `next()`. There is no eager producer task or unbounded `AsyncThrowingStream` buffer.

Blocking native pulls run on a dedicated serial Dispatch queue per active call. Checked continuations suspend the Swift task. A blocking pull never occupies the main actor or Swift's cooperative executor. An actor alone would not make that blocking work safe. A small synchronized call owner keeps cancel and native free from racing: cancellation may call `thinkthen_cancel` while pulling, but free runs only after the host native operation has returned. Concurrent calls have separate drivers. A client retains its native owner across admission; calls then retain their admitted engine independently. The only unchecked sendability, if required by imported native pointers, is internal to these synchronized owners and reviewed there.

Task cancellation handlers cancel the native call immediately and stop awaiting input. A source that ignores cancellation may keep running; the library observes its pending task without waiting for it and never advances or disposes its iterator concurrently. Its error cannot overwrite an earlier call error. No engine thread calls Swift.

`Rows` implements `AsyncSequence`. Its iterator owns the active call; iterator release cancels and frees it after any active native operation returns. Swift's iteration protocol has no deterministic early-break disposal hook. `Rows.cancel()` explicitly cancels a live iteration, and the `withRows` scope used above always cancels and closes on return or throw, including an early break. Bare `for try await` remains available, but callers who retain sequence or iterator state use `withRows` for deterministic cleanup. The scope inherits the caller’s actor isolation and lets its closure use actor-owned state. It holds one iterator and exposes that same traversal to its closure. It does not open another call. `makeAsyncIterator()` cannot throw. A second iterator is an inert rejected traversal whose first `next()` throws the public `RowsIterationError.alreadyIterated` before native access. It opens no call and never cancels or otherwise changes the first traversal. `RowsIterationError: Error, Sendable` is a host lifecycle error, not an engine code.

A held backend request must be cancelled by the consuming task, without an explicit `Rows.cancel()`:

```swift
let task = Task {
    try await client.decide("Ready?", record: .text("evidence"))
}
// The application's event confirms that the request reached the backend.
task.cancel()
_ = try await task.value // throws CancellationError
```

The loader remains loaded for the process lifetime because engine workers may outlive a cancelled host call. Cleanup never joins a backend worker. The existing C ABI remains unchanged.

## Package contents and runtime

The package uses Swift tools 6.3 and Swift 6 language mode. The release package lists its bundled native targets. Operating-system and runtime floors follow the shared [runtime floor contract](../../scripts/runtime-floors.json). The consumer needs the Swift toolchain and host SDK, but no Rust installation. Linux applications must retain their matching Swift runtime and package resource bundle alongside the executable.

The SwiftPM package has a Swift `ThinkThen` target and a C import target for the shipped public header. Constant aliases derive from that header's macros. The package copies the authoritative header rather than maintaining separate ABI declarations. The loader uses the host's native library-loading API and retains the loaded handle for the process lifetime.

Native engines and their licenses ship under `Sources/ThinkThen/Native`. SwiftPM copies those resources with `.copy("Native")`. The loader finds the matching resource through `Bundle.module` and opens that exact path. Consumers supply no library search path or downloader. The manifest uses no `unsafeFlags`.

The package checks use installed consumers, shared stub construction cases and held loopback requests. They cover raw JSON fidelity, owned rows, errors, streaming and task cancellation. Tests make no live calls. Package resource copying follows [PackageDescription](https://docs.swift.org/package-manager/PackageDescription/PackageDescription.html). Desktop signing and notarization remain the consuming application's responsibility.

## Design boundaries

`withRows` scopes cleanup for early exit. The opaque `JSON` byte value preserves record, identifier, annotation and detail keys and integers. Foundation decoding remains a caller-selected view.
