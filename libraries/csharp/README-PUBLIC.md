# ThinkThen for C#

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
dotnet add package Botassembly.ThinkThen
```

Use Jev, SystemOne, OpenAI or local models. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```csharp
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var teams = new AuthoredOptionsObject { Value = new KeyValuePair<string, AuthoredDescription>[] {
    new("billing", new() { Value = new AuthoredDescriptionValueString { Value = "Invoices, fees, and refunds." } }),
    new("shipping", new() { Value = new AuthoredDescriptionValueString { Value = "Parcels and delivery." } }),
    new("account", new() { Value = new AuthoredDescriptionValueString { Value = "Logins and passwords." } }) } };
var teamQuestion = new RequestQuestionDefinition { Value = new RequestDefinitionAuthoredChoose {
    Choose = new AuthoredQuestionTextString { Value = "Which team owns this?" }, Options = teams } };
foreach (var text in new[] { "Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back." }) {
    var refund = await engine.DecideAsync(new RequestQuestionText { Text = "Does the customer ask for a refund?" }, new RequestInputText { Text = text });
    Console.WriteLine(refund.Value.ValueKind == System.Text.Json.JsonValueKind.Null ? "not sure" : refund.Value.GetBoolean().ToString());
}
foreach (var text in new[] { "Please refund the extra fee on my invoice.", "My parcel went to the wrong address.", "I cannot reset my password." }) {
    var team = await engine.ChooseAsync(teamQuestion, new RequestInputText { Text = text });
    Console.WriteLine(team.Value ?? "not sure");
}
```

Read `Value` for the answer, `Probability` for its odds, `Results` for complete records and `Facts` for call totals. A null decision means not sure.

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
