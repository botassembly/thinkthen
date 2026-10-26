# ThinkThen for C#

Use .NET 10 and run `dotnet add package Botassembly.ThinkThen`.

The example uses the stub backend and needs no key or service.

```csharp
using ThinkThen;

await using var client = new ThinkThenClient(new ThinkThenOptions { Backend = Backend.Stub });
using var stop = new CancellationTokenSource();
var row = await client.DecideAsync(
    "Does the customer ask for money back?",
    "Please refund my order.",
    cancellationToken: stop.Token);

if (row is Answered<bool, NoDetail> answered)
    Console.WriteLine(answered.Answer);
else if (row is Failed<bool, NoDetail> failed)
    Console.Error.WriteLine($"{failed.Error.Code}: {failed.Error.Message}");
else
    Console.WriteLine("Not sure");
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/csharp/API.md).
