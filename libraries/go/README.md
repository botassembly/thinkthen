# ThinkThen for Go

This Go 1.22 cgo source module calls the separately installed ThinkThen C library. Rust retains question grammar, judgment, scheduling and transport. The package gate runs on Linux x86_64. Each GitHub release tags the module as `libraries/go/v0.1.2` and ships the matching C archive.

## Build and use a matching local copy

From this repository checkout, build the C door and install its header and libraries into one prefix. The build using `libraries/c/Cargo.toml` writes to `libraries/c/target`, not the root `target` folder.

```sh
cargo build --locked --offline --manifest-path libraries/c/Cargo.toml --lib
NATIVE="$HOME/.local/thinkthen-c"
mkdir -p "$NATIVE/include" "$NATIVE/lib/pkgconfig"
cp libraries/c/include/thinkthen.h "$NATIVE/include/"
cp libraries/c/target/debug/libthinkthen_c.so "$NATIVE/lib/libthinkthen.so"
sh libraries/c/localize.sh libraries/c/target/debug/libthinkthen_c.a "$NATIVE/lib/libthinkthen.a"
ln -sfn libthinkthen.so "$NATIVE/lib/libthinkthen.so.0"
printf 'prefix=%s\nName: thinkthen\nDescription: ThinkThen C ABI\nVersion: 0.2.0\nLibs: -L${prefix}/lib -lthinkthen\nCflags: -I${prefix}/include\n' "$NATIVE" > "$NATIVE/lib/pkgconfig/thinkthen.pc"
```

In a separate Go project, `go get github.com/botassembly/thinkthen/libraries/go@v0.1.2` adds the released module. To build against this checkout instead, add `require github.com/botassembly/thinkthen/libraries/go v0.1.2` and `replace github.com/botassembly/thinkthen/libraries/go => /absolute/path/to/this/checkout/libraries/go` to its `go.mod`. Set `PKG_CONFIG_PATH="$NATIVE/lib/pkgconfig"`, `CGO_ENABLED=1`, `GOPROXY=off`, `GOTOOLCHAIN=local`, and `LD_LIBRARY_PATH="$NATIVE/lib"` when building and running. The [example](examples/decide/main.go) imports the product module path. Match the native header and library to the same source revision. Shared-library relocation needs an explicit loader policy. Static-C mode links the C archive but still depends on Linux system libraries.

`sh libraries/go/check.sh 0` runs the offline product gate with a counted loopback backend. It checks the current header/library match, four copied module consumers, exact request bodies, the public result corpus, settings precedence, and planted failures. It does not call a paid backend.

For Linux x86-64, `sdlc/scripts/release-pack TARGET OUT c go cpp` produces versioned Go, C++ and C archives from one clean source commit. Run `sdlc/scripts/release-go-cpp-pair OUT` before using the files together. The Go archive carries source only; unpack it and the matching C archive into separate folders. Point `PKG_CONFIG_PATH` to the C archive's `lib/pkgconfig`, and set `LD_LIBRARY_PATH` to its `lib` when running the shared build. The Go archive's `THINKTHEN-PACKAGE-INPUTS` records the exact C archive digest. The release ships these archives, and an installed-consumer check covers them.

## API and ownership

`New()` reads the native environment. `NewWith(settingsJSON)` applies the accepted settings object; its `base_url` overrides a valid but unusable environment route. An invalid settings object fails before sending. `Call` returns a Go-owned JSON string whose successful asking result contains `{"value":VALUE,"facts":FACTS}`. The four direct typed methods `Decide`, `DecideMany`, `Recognize`, and `Relate` return `Result[T]` with the former value in `.Value` and the call's facts object in `.Facts` as `json.RawMessage`. After `Decide`, read `result.Value.Outcome`, one of `Yes`, `No` and `Unsure` (1, 0 and 2); `Recognize` and `Relate` return their result JSON in `result.Value`. `specification/result.schema.json` describes every JSON value; decode the members you need and ignore the rest. `ReadField` reads one annotate answer member as `Unresolved` (JSON null), `Answered` with its JSON value, or `Failed` with the failure's kind and cause. `Error` carries numeric `Code`, an `ErrorKind` (`KindUsage` through `KindDefect`, codes 1 to 6), `Retryable`, a copied `Message`, and copied final `Facts` when the failed native call started.

`Plan(verb, question, input, settings)` previews a `decide`, `choose`, `score` or `tag` call through `thinkthen_plan_json` and returns the result schema's `plan` object as `json.RawMessage`. The question is bare text, or one question object when it starts with `{`; settings is nil or a `thinkthen.settings/1` object. It needs no key, reads no cache and sends nothing. A context deadline reaches every sending method as `deadline_ms`, and `NewWith` passes engine settings such as `max_requests_total` to the C constructor unchanged. The package offers no probability option on score or tag, so it has no probability refusal to make.

Each failing cgo call and its borrowed error reads stay on one OS thread with `runtime.LockOSThread`; a goroutine can migrate between those calls otherwise. Returned answers, JSON and error data are Go-owned. `Engine.Close` waits for in-flight calls and then refuses new work. A caller must join its own users of the engine before closing it. The binding rejects interior NUL in C-string inputs, keeps evidence byte lengths, bounds native result lengths before `GoStringN`, and frees native result strings after copying.

Each call owns a one-shot cancellation token. Its watcher joins before token free. A context deadline becomes a bounded native millisecond budget; explicit cancellation fires the token. Deadline and cancellation return different native error kinds, including after a held reply drains. Fresh tokens can recover. The Go race checker covers Go code, not Rust allocations. The local synthetic backend checks calling mechanics, not model quality or final-release packaging.

`NewWith(settingsJSON)` accepts `{"backend":"local"}` to select the `local` entry in the read-only ThinkThen configuration. Use `{"base_url":"http://localhost:11434/v1"}` for a direct address instead. A named backend supplies its address, model, wire settings and key environment variable; explicit constructor settings take precedence. Omitting `backend` preserves ordinary environment/default selection. A missing or invalid name fails before sending.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.

`Engine` implements `CompleteEngine`: all ten `*Complete` methods accept a `QuestionInput`, an explicit `InputSource`, and `CallControls`, and return owned typed rows, details, identities, observations and final facts. `Asked` uses counted typed constructors. `SavedQuestion`, `NamedQuestion` and `QuestionReference` take an explicit native `QuestionRole`; the native grammar loads declarations and saved sets. `SourceFiles` supports line, window, file, image-file and JSONL units. Known engine fields have typed accessors; caller-owned JSON originals remain `Content.Json`.

`DecideBatch`, `ChooseBatch`, `TagBatch`, `ScoreBatch`, `FilterBatch` and `AnnotateBatch` use the native lazy scheduler. Call `Next` until it returns nil, then read `Facts`, and always `Close` the batch before closing its engine. A dedicated pinned goroutine keeps every batch operation on its creating native thread. Completed rows remain valid after either handle closes. Started failures expose typed `Error.Complete`; absent summary metadata and aggregate observation IDs remain absent. Compatibility methods retain their signatures. The existing family gate executes all 247 required shared cases through these typed methods, including all 24 image-admission recipes, with counted loopback sends and zero-send replay checks. Whole-family review and landing remain with [ticket 0427](../../sdlc/tickets/0427-go-csharp-jvm-typed-parity.md).

Rank-set rows retain every member in saved declaration order. Each member exposes its native positive rank position, probability, answer identity, author declarations and complete details. Details preserve independently reported token dimensions and source batch sizes. Parent and member metadata overlap; read final call facts for invocation usage.

### Native owned calls (0.2 development)

Use `NewClient(settings)` and its ten named methods for the native request contract. Each method takes a `context.Context`, a question string or ordinary question-definition map, ordinary Go inputs, and call options. Slices become ordered records; `Item` adds per-record fields and `FileRecords` selects the native reader. The engine validates inputs and owns cache keys and replay.

```go
client, err := thinkthen.NewClient(map[string]any{"cache": false})
if err != nil { return err }
defer client.Close()
call, err := client.Decide(ctx, "Is this urgent?", []string{"first", "second"}, nil)
if err != nil { return err }
facts := call.Terminal.Facts()
if facts.Present && !facts.Null && facts.Err == nil {
    // Inspect facts.Value through generated accessors.
}
```

For incremental input, pass a `Producer` implementing `Next(context.Context) (any, error)`. Return ordinary values or `Item` values in order, and return `io.EOF` to finish. `Next` must return promptly when its supplied context is cancelled. The client calls it serially, cancels it when intake closes, and joins the reader before returning from the call or `Client.Close`. Caller code must respect that context; Go cannot interrupt an arbitrary blocked callback.

The client retains one pending descriptor and requests another value only after native acceptance. Native `FULL` retries the same descriptor while the host continues draining output. The existing nonblocking native interface uses timer-paced waits. Input grammar, image admission and scheduling remain native. Producer read errors become the native reader failure without copying the caller's diagnostic; value serialization failures become invalid input. Completed packets and native terminal facts remain available on failure. Finite slices and `FileRecords` retain their existing behavior.

`OwnedCall` retains generated packets and settled terminal facts after client cleanup. Generated accessors return `Presence[T]`: inspect `Present`, `Null` and `Err` before reading `Value`. Unknown fields survive JSON round trips. JSON numbers retain their original numeric representation. `SessionError` carries the complete failed call and typed native error. Context cancellation returns the context error and promptly frees the session; it does not invent settled facts. A cancelled call retains its packet prefix and has a nil terminal while settlement is pending. Client cleanup also stops active host readers promptly.

The staged Linux amd64 module carries its static native library under `native/x86_64-unknown-linux-gnu`. A consumer needs Go and a C compiler, with no pkg-config, Rust compiler or library-path setup. Other target assets and final distribution assembly still require their owning package work. Development source checks stage their own native build separately.

Existing `Engine` calls remain compatibility APIs until installed migration parity permits retirement: `Engine.Decide` becomes `Client.Decide`, the generic `Call` becomes a named method, and `Files` becomes a named method with `FileRecords`. This additive slice does not retire the compatibility readers. Named sessions identify the Go surface through the native constructor.

## Live usage persistence

`Client.UsagePersistence()` and `Client.FinishUsageStatus()` return an immutable `UsageStatus` and an error. The compatibility `Engine` exposes the same methods. Read the generated state through `State()` and the copied optional advice through `Advice()`.

These methods observe the native engine directly and refuse calls after close. Failed persistence leaves successful answers and their earlier facts intact. Written covers this engine's current deltas, not future calls or other engines. Only usage-lock acquisition has a deadline; other filesystem work can take longer. Returned observations remain readable after close. See [the C engine contract](../c/DESIGN.md) for native observation semantics.
