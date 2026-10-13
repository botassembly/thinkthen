# ThinkThen for C#

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

```sh
dotnet add package Botassembly.ThinkThen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```csharp
using ThinkThen;
using ThinkThen.Inputs;

using var engine = Engine.Open();
var call = await engine.DecideAsync(
    new InputRequestQuestionText { Text = "Does this ask for a refund?" },
    new InputRequestInputText { Text = "Please refund my order." });
Console.WriteLine(call.Decision());
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
