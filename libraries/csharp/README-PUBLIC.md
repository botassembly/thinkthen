# ThinkThen for C#

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
dotnet add package Botassembly.ThinkThen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```csharp
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var teams = new Dictionary<string, InputAuthoredDescription>();
foreach (var pair in new[]{("billing", "Invoices, fees, and refunds."),
    ("shipping", "Parcels and delivery."), ("account", "Logins and passwords.")})
    teams[pair.Item1] = new InputAuthoredDescription { Value = new InputAuthoredDescriptionValueAlternative0 { Value = pair.Item2 } };
var teamQuestion = new InputRequestQuestionDefinition { Value = new InputRequestDefinitionAuthoredChoose {
    Choose = new InputAuthoredQuestionTextAlternative0 { Value = "Which team owns this?" },
    Options = new InputAuthoredOptionsAlternative1 { Value = teams }
} };
foreach (var text in new[]{"Please refund my order. It arrived broken.",
    "Thanks for the quick help yesterday!", "I want to send this back."}) {
    var refund = await engine.DecideValueAsync(new InputRequestQuestionText { Text = "Does the customer ask for a refund?" }, new InputRequestInputText { Text = text });
    Console.WriteLine(refund?.ToString().ToLowerInvariant() ?? "not sure");
}
foreach (var text in new[]{"Please refund the extra fee on my invoice.",
    "My parcel went to the wrong address.", "I cannot reset my password."}) {
    var team = await engine.ChooseValueAsync(teamQuestion, new InputRequestInputText { Text = text });
    Console.WriteLine(team ?? "not sure");
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
