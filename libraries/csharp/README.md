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
    if (packet is SessionPacketDecideRow row) Console.WriteLine(row.Value.Value);
SessionPacketTerminal terminal = call.Terminal;
```

The ten named `*Async` methods admit generated Request inputs and return owned generated row, aggregate, observation and terminal packets. `SessionFailure` retains the native typed failure and completed packet prefix. Immediate admission errors use `Failure`. Host cancellation throws `OperationCanceledException` promptly and releases session ownership. The native provider can still be settling after that exception.

`Engine.StartSession` exposes `OwnedSession.ReadAsync`, `PushAsync`, `Finish`, `Cancel` and disposal. A read returns null only at native End. Push retries the same descriptor after Full; false means Closed and the producer must stop advancing its reader. One reader and one producer can run concurrently. `ExecuteAsync` runs an asynchronous feed and drain together, which prevents either bounded queue from blocking the other. `Finish` accepts an optional generated `InputRequestReaderFailure` for I/O, UTF-8 or input-grammar failures and an optional caller-owned source location. `ExecuteAsync` maps `IOException`, `DecoderFallbackException`, `JsonException` and `InvalidDataException` from its feed to these native failures without copying host diagnostics. Native admission validates settings and locations. An arbitrary asynchronous iterator must honor its cancellation token; the binding cannot interrupt foreign iterator code.

Read and push wait through cancellable timer delays after Pending or Full. Each session has at most one pending delay in each direction and no host worker thread. Disposal signals cancellation and closes ownership without joining the native worker. SafeHandle pins protect native operations and packet byte copies. Generated result values survive engine and session disposal and retain unknown nested members and missing versus explicit null fields.

An explicit `Cancel` retains the output receiver. Call `ReadAsync` with a fresh token to drain completed packets and eventual native terminal facts. Disposal relinquishes unread output; it does not promise that settlement has finished. A named convenience call disposes its session when cancelled.

`Engine.UsagePersistence()` observes live persistence without waiting for the writer or filesystem. `Engine.FinishUsageStatus()` finishes current usage deltas; only usage-lock acquisition has a deadline, and other filesystem work may take longer. Both return an immutable `UsagePersistenceStatus` with generated `UsagePersistenceState` and optional native safe advice, retained after disposal. Failed persistence leaves successful answers and their recorded call-facts snapshot intact. Written covers this engine's current deltas only, not future calls or other engines.

Use `Engine.Open(InputEngineSettings)` for native defaults and typed settings. Nullable budget alternatives preserve explicit null. `Engine.Plan(InputRequest)` returns the generated owned `Plan` with unsigned token bounds and explicit nullable body presence. It forwards native canonical Request preview for decide, choose, tag and score without key reads, cache reads or sends. Native admission refuses unsupported functions.

The migration maps earlier callers to these public doors:

| Earlier API | Typed API |
| --- | --- |
| `Engine.Open(settingsJson)` | `Engine.Open(InputEngineSettings)` |
| `Decide`, `DecideComplete` and the other named judgments | The corresponding named `*Async` method with generated question and input objects |
| Named batch readers | The named `*Async` overload accepting `IAsyncEnumerable<InputRequestSessionDescriptor>` |
| `Call`, `CallTyped` | A named function; use `ExecuteAsync(InputRequest)` or `StartSession(InputRequest)` for explicit session control |
| `Plan(verb, question, texts, settings)` | `Plan(InputRequest)` |
| Compatibility result readers and native layouts | Owned generated `ThinkThen.Results` classes and packet variants |

Question file, saved-name and reference selectors are generated input objects. Authored questions and ordered options also have generated types. `Engine.ParseQuestion(AuthoredQuestionKind, authoredJson)` uses the existing native authored-question parser and returns an owned generated definition. Its typed properties are immutable; serializing the definition preserves admitted unknown members and authored order. The JSON here is authored question data. Execution always receives generated Request objects. User records and original result inputs retain arbitrary JSON data; the caller does not build transport JSON. A streamed producer owns its descriptors and supplies optional source locations. The native engine owns question admission, selection, framing and judgment semantics.

To pack locally, supply an already-built native asset, portable RID and native filename:

```sh
dotnet pack libraries/csharp/ThinkThen.csproj -c Release -o path/to/feed \
  -p:ThinkThenNativeAsset=/absolute/path/to/libthinkthen_c.so \
  -p:ThinkThenNativeRid=linux-x64 -p:ThinkThenNativeName=libthinkthen.so
```

The existing package checker compares native bytes and reflected imports against the generated C header. `tests/isolated_consumer.py` restores the local package into an isolated application with no separate native archive or library-path override. Its `sessions` mode covers owned packets, held-provider cancellation, concurrent Task progress, Full and Closed, explicit drain, disposal races and presence semantics. `sh libraries/csharp/check.sh` also exercises the typed counted matrix, the shared J1 semantic cases and all current native shared fixtures through named Async calls. The 13 structurally invalid J1 legacy wire forms and the legacy `{usage:true}` control remain schema fixtures and frozen C compatibility tests; C# exposes no legacy JSON execution door. The SDK executable resolves from `dotnet` or `THINKTHEN_DOTNET`.
