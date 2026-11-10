# C# SDK

The C# SDK asks questions about text from your .NET code. Its first function, decide, answers a yes-or-no question with yes (`True`), no (`False`) or not sure.

Install it with .NET 10:

```sh
dotnet add package Botassembly.ThinkThen --version 0.2.0
```

It carries its own engine. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```csharp
using ThinkThen;

await using var client = new ThinkThenClient();
bool isRefund = await client.DecideAsync(
    "Does the customer request a refund?",
    "Please refund my order.");
Console.WriteLine(isRefund);
```

Output:

```text
True
```

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. Turning the result into a `bool` throws `UnsureException` when decide is not sure, and `ThinkThenException` when ThinkThen could not judge the message, so neither one passes for `false`. Send both to a person.

```csharp
using ThinkThen;

await using var client = new ThinkThenClient();
var bars = new DecideOptions
{
    Threshold = Threshold.Band(0.2, 0.8),
};
try
{
    bool isRefund = await client.DecideAsync(
        "Does the customer request a refund?",
        "I was charged twice for one invoice.", bars);
    Console.WriteLine(isRefund);
}
catch (UnsureException unsure)
{
    Console.WriteLine(
        $"Not sure ({unsure.Probability}): ask a person.");
}
catch (ThinkThenException failure)
{
    Console.WriteLine($"{failure.Code}: {failure.Message}");
}
```

Output:

```text
Not sure (0.7): ask a person.
```

## Next

The [C# SDK page](https://thinkthen.dev/sdks/csharp/) shows all ten functions. The [C# guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/csharp.md) and the [API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/csharp/API.md) cover every option.

### Inspect client usage

```csharp
using System;
using ThinkThen;

await using var client = new ThinkThenClient(new());
await client.DecideAsync("Ready?", "A message");
await client.DisposeAsync();
Usage snapshot = client.Usage;
Console.WriteLine(snapshot.Attempts);
```

`Usage` is an immutable cumulative snapshot for the explicit client and remains available after disposal. Input and output token totals preserve unknown usage; active attempts and calls remain separate. Use a separate client for isolated accounting. These counters are not a price or invoice total.

## Request allowance

```csharp
using ThinkThen;

await using var client = new ThinkThenClient(new() { MaxRequests = 100 });
await client.DecideAsync("Ready?", "A message");
```

The optional allowance belongs to this explicit client and counts actual remote send attempts, including retries. Unset is unlimited; zero forbids remote sends. Stub calls use no remote allowance. Exhaustion ends the call with `local.request-limit`; it does not promise a final bill or refund in-flight requests. This setting does not inherit the command's environment allowance.

### Local response storage

`ThinkThenOptions.ResponseStore = new ResponseStoreOptions(path, ResponseStoreMode.Record, namespaceName, modelGeneration)` explicitly enables local recording; `Cache` reuses eligible answers and `Replay` works without credentials. Optional `MaxBytes`, `MaxEntries`, `MaxAge` (seconds), and `BusyTimeoutMs` use engine defaults when null. `client.Usage.CacheHits` and `ReplayHits` report local reuse.

Storage is disabled unless configured. Local recordings retain question and record text; use a private local path and a nonsecret namespace/model-generation label. The shared engine owns limits, identity and concurrency policy. Replay misses fail locally rather than making a model call.
