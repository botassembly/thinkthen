# ThinkThen C# binding

`Botassembly.ThinkThen` is the local .NET 8 wrapper package. It calls the separately installed ThinkThen C library. `Botassembly.ThinkThen.C` is reserved for the native package; this wrapper does not ship, download, or install that library. Neither package has been published or registered.

Run `sh libraries/csharp/check.sh` from a source checkout with .NET 8, Rust and an offline Cargo cache. The .NET SDK resolves from `dotnet` on `PATH`, or `THINKTHEN_DOTNET` can name its executable. The check builds the C library from the same checkout, compares its exports with the current C header, packs the wrapper into a local NuGet feed, and runs the exact backend matrix, two isolated installed NuGet consumers, and the J1 public-binding type corpus. Generated artifacts and receipts stay under `libraries/csharp/target/`.

For an application on Linux x86_64, build and install the matching native C library separately, then install the local wrapper package:

```sh
dotnet add path/to/App.csproj package Botassembly.ThinkThen --version 0.0.1 --source path/to/local/feed
LD_LIBRARY_PATH=path/to/native/lib dotnet run --project path/to/App.csproj
```

The native library must match the header and ABI used to build the wrapper. The local gate checks the current header-derived symbol set, currently 30 declarations. Set the supported backend URL and credentials as described in the repository documentation. A package ID and local nupkg are metadata and proof, not registry availability or a compatibility promise for another platform.

`Engine` owns its native handle. Dispose it after active calls finish. The wrapper locks its lifetime against close during a call, and cancellation tokens are one-shot. C-string arguments reject interior NUL; counted UTF-8 evidence preserves it. Native result and error data, including borrowed error facts, are copied on the native calling thread before freeing native memory. `Decide`, `DecideMany`, `Recognize`, and `Relate` return `TypedResult<T>`: `.Value` keeps the former typed value and `.Facts` owns that call's `Records`, `RequestsSent`, `CacheAnswers`, `Seconds`, optional `InputTokens`/`OutputTokens`, and optional `Model`. Unreported tokens remain `null`; an empty call has no model. `Engine.CallTyped` retains its separate JSON `CallResult` value and facts. `Answer.OutcomeKind` and `Failure.Kind` expose named outcomes and errors while the raw integer fields retain ABI layout. A failure's copied `FactsJson` is distinct from a successful result's facts. The public API covers the six named native error classes, including cancellation. See the local check for exact request and type-contract evidence.

Linux x86_64 is the installed-consumer proof host. The local release pilot accepts a checksummed `thinkthen-csharp-<version>-<target>.tar.gz` beside the matching `thinkthen-c-<version>-<target>.tar.gz`; the managed archive contains the nupkg and a fixed input manifest, while the C archive supplies the native library. The two-file installed check reads only those unpacked product bytes. Other hosts, final release packaging, registry installation, and publication remain separate work.
