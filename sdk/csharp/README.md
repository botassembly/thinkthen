# ThinkThen for C#

Use .NET 10 and C# 14. Install `Botassembly.ThinkThen`, set `THINKTHEN_API_KEY`, and dispose the client with `await using`. The package includes the engine.

```csharp
using ThinkThen;

await using var client = new ThinkThenClient();

if (await client.DecideAsync("Does the customer request a refund?", "Please refund my order."))
{
    Console.WriteLine("refund queue");
}
else
{
    Console.WriteLine("inbox");
}
```

Awaiting a one-record call throws `FailedException` if its parent row failed. A returned scalar result is answered or unsure; `Answer` can throw `UnsureException`. Many-record results and form entries retain failed states, so their answer reads can also throw `FailedException`; catch `NoAnswerException` for either missing-answer state. Scalar annotate returns answered parents even when entries failed; `ThrowIfFailed()` explicitly checks those entries. State checks, printing, formatting and storage are safe. Every result retains the original record as `Item` without including it in equality or hashing.

The [C# guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/csharp.md) covers all ten functions. The [API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/csharp/API.md) covers settings, metadata, ownership and Native AOT methods.
