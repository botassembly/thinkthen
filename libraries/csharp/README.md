# ThinkThen C# binding

`Botassembly.ThinkThen` provides a .NET 8 API and carries the matching native asset. The loader accepts the installed assembly's native asset by absolute path. A missing asset fails without consulting library-path variables or downloading code. Local package checks qualify Linux x64. Other operating systems require their own native build and qualification.

Install the package from a local feed and use native typed inputs:

```sh
dotnet add path/to/App.csproj package Botassembly.ThinkThen --version 0.2.0 --source path/to/feed
dotnet run --project path/to/App.csproj
```

```csharp
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open(new InputEngineSettings { MaxRetries = 0 });
OwnedCall call = await engine.DecideAsync(
    new InputRequestQuestionText { Text = "Does this answer the question?" },
    new InputRequestInputText { Text = "The supplied evidence." },
    cancellation: cancellationToken);
foreach (SessionPacket packet in call.Packets)
    if (packet is SessionPacketDecideRow row) Console.WriteLine(row.ToJsonString());
SessionPacketTerminal terminal = call.Terminal;
```

The ten named `*Async` methods admit generated Request inputs and return owned generated row, aggregate, observation and terminal packets. `SessionFailure` retains the native typed failure and completed packet prefix. Immediate admission errors use `Failure`. Host cancellation throws `OperationCanceledException` promptly and releases session ownership. The native provider can still be settling after that exception.

`Engine.StartSession` exposes `OwnedSession.ReadAsync`, `PushAsync`, `Finish`, `Cancel` and disposal. A read returns null only at native End. Push retries the same descriptor after Full; false means Closed and the producer must stop advancing its reader. One reader and one producer can run concurrently. `ExecuteAsync` runs an asynchronous feed and drain together, which prevents either bounded queue from blocking the other. `Finish` accepts an optional generated `InputRequestReaderFailure` for I/O, UTF-8 or input-grammar failures and an optional caller-owned source location. `ExecuteAsync` maps `IOException`, `DecoderFallbackException`, `JsonException` and `InvalidDataException` from its feed to these native failures without copying host diagnostics. Native admission validates settings and locations. An arbitrary asynchronous iterator must honor its cancellation token; the binding cannot interrupt foreign iterator code.

Read and push wait through cancellable timer delays after Pending or Full. Each session has at most one pending delay in each direction and no host worker thread. Disposal signals cancellation and closes ownership without joining the native worker. SafeHandle pins protect native operations and packet byte copies. Generated result values survive engine and session disposal and retain unknown nested members and missing versus explicit null fields.

An explicit `Cancel` retains the output receiver. Call `ReadAsync` with a fresh token to drain completed packets and eventual native terminal facts. Disposal relinquishes unread output; it does not promise that settlement has finished. A named convenience call disposes its session when cancelled.

The existing synchronous, complete and batch APIs remain available while consumers migrate. `Decide` maps to `DecideAsync`, the other named judgments map to their `*Async` counterparts, and `Call` maps to typed `ExecuteAsync` or `StartSession`. `Engine.Open(InputEngineSettings)` writes the native settings document through generated types. Missing settings retain native defaults; nullable budget alternatives preserve explicit null. The string settings overload and `Engine.Plan` remain compatibility APIs while canonical Request preview is completed. Their behavior remains part of the migration contract.

To pack locally, supply an already-built native asset, portable RID and native filename:

```sh
dotnet pack libraries/csharp/ThinkThen.csproj -c Release -o path/to/feed \
  -p:ThinkThenNativeAsset=/absolute/path/to/libthinkthen_c.so \
  -p:ThinkThenNativeRid=linux-x64 -p:ThinkThenNativeName=libthinkthen.so
```

The existing package checker compares native bytes and reflected imports against the generated C header. `tests/isolated_consumer.py` restores the local package into an isolated application with no separate native archive or library-path override. Its `sessions` mode covers owned packets, held-provider cancellation, concurrent Task progress, Full and Closed, explicit drain, disposal races and presence semantics. `sh libraries/csharp/check.sh` also exercises the compatibility matrix. The SDK executable resolves from `dotnet` or `THINKTHEN_DOTNET`.
