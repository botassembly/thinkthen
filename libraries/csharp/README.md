# ThinkThen C# binding

`Botassembly.ThinkThen` is the .NET 8 wrapper package on NuGet. It calls the separately installed ThinkThen C library. This wrapper does not ship, download, or install that library. Take the library from `thinkthen-c-0.1.2-x86_64-unknown-linux-gnu.tar.gz` on the same GitHub release. `Botassembly.ThinkThen.C` is reserved for a native package and is not published.

Run `sh libraries/csharp/check.sh` from a source checkout with .NET 8, Rust and an offline Cargo cache. The .NET SDK resolves from `dotnet` on `PATH`, or `THINKTHEN_DOTNET` can name its executable. The check builds the C library from the same checkout, compares its exports with the current C header, packs the wrapper into a local NuGet feed, and runs the exact backend matrix, two isolated installed NuGet consumers, and the J1 public-binding type corpus. Generated artifacts and receipts stay under `libraries/csharp/target/`.

For an application on Linux x86_64, install the matching native C library separately, then install the wrapper package:

```sh
dotnet add path/to/App.csproj package Botassembly.ThinkThen --version 0.1.2
LD_LIBRARY_PATH=path/to/native/lib dotnet run --project path/to/App.csproj
```

The native library must match the header and ABI used to build the wrapper. The local gate checks the current header-derived symbol set, currently 31 declarations. Set the supported backend URL and credentials as described in the repository documentation. The package makes no compatibility promise for another platform. To install without NuGet, unpack the release's `thinkthen-csharp-0.1.2-x86_64-unknown-linux-gnu.tar.gz` and pass its folder to `dotnet add` as `--source`.

`Engine` owns its native handle. Dispose it after active calls finish. The wrapper locks its lifetime against close during a call, and cancellation tokens are one-shot. C-string arguments reject interior NUL; counted UTF-8 evidence preserves it. Native result and error data, including borrowed error facts, are copied on the native calling thread before freeing native memory. `Decide`, `DecideMany`, `Recognize`, `Relate` and `CallTyped` return `TypedResult<T>`: `.Value` is the call's value and `.Facts` is that call's facts object as a `JsonElement`, as `specification/result.schema.json` describes it. `Recognize`, `Relate` and `CallTyped` values are `JsonElement` too. A reader ignores members it does not know. `AnnotatedField.Read` reads one member of an annotate row as `Unresolved` (JSON null), `Answered` with its value, or `Failed` with its kind and cause. `Engine.Plan(verb, question, input, settingsJson)` previews a decide, choose, score or tag call through `thinkthen_plan_json` and returns the plan object; it needs no key and sends nothing. Every call that sends takes an optional `TimeSpan` budget; `RelateWithOptions` carries it for `Relate`. `max_requests_total` and other engine settings pass through `Engine.Open(settingsJson)`. `Answer.OutcomeKind` and `Failure.Kind` expose named outcomes and errors while the raw integer fields retain ABI layout. A failure's copied `FactsJson` is distinct from a successful result's facts. The public API covers the six named native error classes, including cancellation. See the local check for exact request and type-contract evidence.

Linux x86_64 is the installed-consumer proof host. The release ships a checksummed `thinkthen-csharp-<version>-<target>.tar.gz` beside the matching `thinkthen-c-<version>-<target>.tar.gz`; the managed archive contains the nupkg and a fixed input manifest, while the C archive supplies the native library. The two-file installed check reads only those unpacked product bytes. Other hosts remain separate work.

`Engine.Open(settingsJson)` accepts `{"backend":"local"}` to select the `local` entry in the read-only ThinkThen configuration. Use `{"base_url":"http://localhost:11434/v1"}` for a direct address instead. A named backend supplies its address, model, wire settings and key environment variable; explicit constructor settings take precedence. Omitting `backend` preserves ordinary environment/default selection. A missing or invalid name fails before sending.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.

Prepared complete requests and owned typed carriers are available; complete native execution remains pending. [Ticket 0427](../../sdlc/tickets/0427-go-csharp-jvm-typed-parity.md#current-implementation-and-integration-requirements) describes the APIs, checks and exact integration requirements. Existing calls keep their current behavior.
