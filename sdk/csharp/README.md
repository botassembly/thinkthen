# ThinkThen for C#

ThinkThen judges evidence by meaning. A client calls the ten functions and returns immutable result rows. The package targets .NET 10 and uses the `ThinkThen` namespace.

```sh
dotnet add package Botassembly.ThinkThen
```

Set `THINKTHEN_API_KEY` in your shell, then call a function:

```csharp
using ThinkThen;

using var client = new ThinkThenClient();
using var stop = new CancellationTokenSource();
var row = await client.DecideAsync(
    "Does the customer ask for money back?",
    "Please refund my order.",
    cancellationToken: stop.Token);

if (row.Status == RowStatus.Answered)
    Console.WriteLine(row.Answer);
else
    Console.WriteLine("Not sure");
```

Single-record failures throw `ThinkThenException`, whose `Code` and `Message` come from the engine. An unsure row has a null answer. Client settings are admitted once. Use `new ThinkThenClient(new ClientSettings { Adapter = "stub" })` for the built-in backend without a service or key. Backend names remain strings. The same client settings and function methods serve both modes; proxy transport is not yet available.

## Streams

```csharp
using ThinkThen;

using var client = new ThinkThenClient(new ClientSettings { Adapter = "stub" });
using var stop = new CancellationTokenSource();

static async IAsyncEnumerable<Record> Messages()
{
    yield return "Please refund my order.";
    await Task.Yield();
    yield return "Thank you for the delivery.";
}

await foreach (var row in client.DecideStreamAsync(
    "Is a refund requested?", Messages(), cancellationToken: stop.Token))
{
    Console.WriteLine($"{row.Id}: {row.Status}");
}
```

A string converts to a text `Record`. `Record.Json(value)` holds a structured `JsonElement`; arrays remain one record. Text travels as counted UTF-16. The shared engine rejects unpaired surrogates with `usage.record` before any backend request. Structured values travel as JSON. Records and JSON values remain valid after a source `JsonDocument` is disposed.

Streams consume an `IAsyncEnumerable<Record>` on demand and yield `IAsyncEnumerable<ResultRow<TAnswer>>`. They do not collect the source. Each enumeration opens one operation. The shared engine requests input and owns batching, ordering, ranking and failures. Failed stream records remain rows with `Error.Code` and `Error.Message`. `FilterAsync` yields the original `Record` instances selected by the engine; every completion error comes from the engine after its output. Ranking may retain records as described in the main README.

A `CancellationToken` cancels the native operation and wakes a blocked pull. Cancellation raises `OperationCanceledException` carrying that token. An already cancelled token sends no request. Breaking an `await foreach` loop disposes its enumerator, cancels the call and disposes the input enumerator. Cancellation also interrupts waiting for an asynchronous source record. A source that ignores cancellation may keep running; the library observes its eventual completion and disposes it without delaying release of the native call or racing an active source advance. Source exceptions propagate unchanged. Cleanup preserves the original exception. Delivered rows remain valid. Blocking native work runs on a worker thread, so awaiting does not block the caller's thread. Calls may run concurrently through one client. Disposing a client releases its owner; calls already opened retain their engine.

## Questions, rows and settings

Ordinary question strings stay literal, including strings ending in `.json`. Use `Question.Toolbox(toolbox, "refund")` to select a question from an in-memory JSON toolbox. `Question.Reference("refund")` carries a named reference for the future proxy. Recognition accepts `Question.Kinds(...)`; relations accept `Question.Relations(...)`. The engine admits the question against its function. Annotate accepts a toolbox JSON value. The library reads no question files.

Rows have typed answers, a typed status, a JSON identifier, optional probability and optional coded error. `Detail` is a nullable ordinary `JsonElement`, decoded on first access from owned UTF-8 bytes. Names and spans use UTF-16 code-unit offsets with an exclusive end, so `text[start..end]` extracts the span. The engine performs that conversion in answers and detail. Rows expose no mutable collections and retain no native allocation.

Client settings map to the shared [client contract](../../schemas/client.schema.json). Call options map to the [request contract](../../schemas/request.schema.json). Settings, thresholds, evidence selection, offsets and errors have no second implementation in C#. See [API.md](API.md) for the public surface and [PACKAGING.md](PACKAGING.md) for installation and native ownership.
