# ThinkThen for csharp

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. Failed calls remain separate from answers.

## Install

```sh
dotnet add package Botassembly.ThinkThen
```

## Example

Configure a model backend before running this example.

```csharp
using ThinkThen;
using ThinkThen.Inputs;

using var engine = Engine.Open();
var call = await engine.DecideAsync(
    new InputRequestQuestionText { Text = "Does this ask for a refund?" },
    new InputRequestInputText { Text = "Please refund my order." });
Console.WriteLine(call.Decision());
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
