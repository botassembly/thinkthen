# ThinkThen for C#

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

```sh
dotnet add package Botassembly.ThinkThen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```csharp
using ThinkThen;
using ThinkThen.Inputs;

using var engine = Engine.Open();
var call = await engine.DecideAsync(
    new RequestQuestionText { Text = "Does this ask for a refund?" },
    new RequestInputText { Text = "Please refund my order." });
Console.WriteLine(call.Decision());
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
