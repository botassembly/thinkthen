# ThinkThen for Go

The development Go 1.22 module provides one `Client` with ten named calls. Its Linux amd64 package carries the matching static native library. Consumers need Go and a C compiler. Rust owns admission, judgments, cache, replay, scheduling and transport. Other native targets and registry installation await distribution qualification.

The released 0.1.2 module keeps its released API. The examples here describe the development 0.2 package.

```go
client, err := thinkthen.NewClient(thinkthen.EngineSettings{})
if err != nil { return err }
defer client.Close()
call, err := client.Decide(ctx, thinkthen.TextQuestion("Is this urgent?"), []string{"first", "second"}, nil)
if err != nil { return err }
facts := call.Terminal.Facts()
if facts.Present && !facts.Null && facts.Err == nil {
    // Read facts.Value through its generated accessors.
}
```

`Decide`, `Choose`, `Tag`, `Score`, `Filter`, `Rank`, `Find`, `Annotate`, `Recognize` and `Relate` take a `context.Context`, a generated `RequestQuestion`, Go evidence and optional generated `RequestOptions`. `TextQuestion` treats all wording literally. `DefinedQuestion` accepts a generated authored definition or question set. `RequestQuestionFile`, `RequestQuestionName` and `RequestQuestionReference` explicitly select native saved questions. Generated declarations follow [the Rust-owned request schema](../../specification/request.schema.json).

Use generated `AuthoredChoose`, `AuthoredTag` and `AuthoredScore` values for ordered options, labels and levels. Authored values retain their permitted text, object, array and null alternatives. For example, `AuthoredDescriptionObject` keeps a structured choice description and `AuthoredCriterionNull{}` keeps an authored null decision. `Ptr` preserves an optional false, zero, empty collection or explicit null. Nil pointers mean omitted; interface alternatives carry their declared Go type. Native Rust validates every value and its applicability.

Strings are literal originals. Slices and arrays retain input order, duplicates and original JSON values. `TextItem` and `JSONItem` create a generated `RequestItem` for separate per-record context, options, recognition settings and attachments. Passing a generated `RequestInput` selects an explicit source form. Passing `RequestSource` selects native files and folders, with generated framing, media and physical-unit controls. `BytesImage` retains ordered image bytes and declared media; `RequestImageFile` selects a user-named attachment. Rust validates and admits images.

A `Producer` supplies `Next(context.Context) (any, error)` and returns `io.EOF` to finish. It may return originals or generated items. `Next` must return promptly when its context is cancelled. The client calls it serially, retains one pending descriptor, requests another only after native acceptance, and drains output while native intake reports full. Cancellation and `Close` cancel and join the host reader. Go cannot interrupt a callback that ignores its context.

`OwnedCall` contains generated packets and an optional terminal. Read each generated `Presence[T]` through `Present`, `Null`, `Err` and `Value`. A false answer remains present and false. Unknown output fields and original number spellings survive JSON round trips. All results, observations and facts remain Go-owned after `Close`.

`SessionError` retains the failed call and its generated native error, including final facts. Constructor and session-admission errors return typed `Error` values. Explicit cancellation returns `context.Canceled`; an expired context returns `context.DeadlineExceeded`. Cancellation retains the completed packet prefix and leaves `Terminal` nil while final settlement is pending. `Close` stops active sessions promptly and refuses further calls.

`UsagePersistence` and `FinishUsageStatus` return immutable `UsageStatus` values. Generated states and copied optional advice remain available after close. Failed persistence leaves earlier answers and facts intact. Only native usage-lock acquisition has a deadline; filesystem work can take longer.

`Client.Plan(Request)` previews a generated canonical request and returns a generated `OwnedPlan`. Rust owns question and input admission and returns typed refusals for unsupported requests. Preview reads no key or cache and sends nothing. The request serializer supplies its schema version. Plans retain explicit empty-input null bodies and remain readable after close.

## Upgrade from the old Go API

| Old call | Current call |
| --- | --- |
| `New`, `NewWith` | `NewClient(EngineSettings)` |
| `Engine.Decide`, `DecideMany`, `*Complete`, `*Batch` | The corresponding `Client` named call with originals, slices or a `Producer` |
| `Engine.Plan` | `Client.Plan(Request)` |
| `Engine.Call` | The corresponding `Client` named call with generated question and input values |
| `Engine.Files`, `SourceFiles`, `FileRecords` | A named call with `RequestSource` |
| `Item`, `ReadField`, handwritten native views | `RequestItem` and generated packet/result accessors |

The old `Engine`, generic JSON call and handwritten compatibility readers have been removed. Native 0.1 C symbols remain a separate frozen contract.

## Build and check

From a matching source checkout, run `sh libraries/go/check.sh`. It builds the native library offline, stages a separate module with that library, compiles external consumers without library-path overrides, and runs routine shared type cases, typed failures, presence, files, cancellation, bounded intake and usage persistence against counted loopback backends. It also checks formatting, generated source and the package source ceilings. The [decide example](examples/decide/main.go) and [replay smoke](examples/smoke/main.go) use the same public API.

A local staged artifact can be made with `python3 libraries/go/fixtures/package_owned.py libraries/go NATIVE_PREFIX OUTPUT`, where the matching prefix contains `include/thinkthen.h` and `lib/libthinkthen.a`. Point a consumer's Go module replacement at `OUTPUT`; no Rust compiler, pkg-config or native library path is needed to build that consumer. Final release assembly, full parity and additional platforms belong to the release suite.
