# C# functions that answer questions about text

ThinkThen 0.2.0 for C# gives you ten functions. Each one reads a record, answers one question about it, and returns a typed value your code can branch on.

## Install and ask your first question

```sh
dotnet add package Botassembly.ThinkThen --version 0.2.0
```

It needs .NET 10 and includes the native engine.

```csharp
using ThinkThen;

await using var client = new ThinkThenClient();
bool isRefund = await client.DecideAsync(
    "Does the customer request a refund?",
    "Please refund my order.");
Console.WriteLine(isRefund);
```

<!-- output: capture -->

Set `THINKTHEN_API_KEY` in the environment first; the client reads it from there. [Choose a service and get a key](service.md#choose-a-service-and-get-a-key) explains the backends and how to get a key.

## Handle not sure and failures

A call returns a `Row`. Converting it to its answer, as `bool isRefund = ...` does above, is the one step from row to answer, and it never hands back a default:

- When ThinkThen is not sure, it throws `UnsureException`. That is an `InvalidOperationException`, as `Nullable<T>.Value` throws, and its `Probability` gives the row's probability.
- When the record failed, it throws `ThinkThenException`. Its `Code` is the engine's error code as a string, such as `"record.missing"`, and its `ErrorCode` gives the same code as an enum value, such as `ErrorCode.RecordMissing`.

`row.Answer` does the same. With decide's default cut, every answer is yes or no. A band, `Threshold.Band(low, high)`, makes the middle not sure. Check `IsUnsure` first and send those records to a person:

```csharp
string message = "Please refund my order.";
var refundCheck = await client.DecideAsync(
    "Does the customer request a refund?", message,
    new DecideOptions { Threshold = Threshold.Band(0.2, 0.8) });
if (refundCheck.IsUnsure)
    Console.WriteLine($"Ask a person: {message}");
else if (refundCheck)
    Console.WriteLine($"Send to refunds: {message}");
else
    Console.WriteLine($"Leave in the inbox: {message}");
```

A failed record still reaches `if (refundCheck)` and throws there. `IsAnswered` and `IsFailed` test without throwing, and `refundCheck is Failed<bool, NoDetail> failed` gives the error in `failed.Error`. Comparing with `==` never throws, so a not-sure row is neither `== true` nor `== false`; don't branch on it.

Mistakes in the call itself throw `ThinkThenException` at once: blank text throws `ErrorCode.RecordEmpty` with no request, and a list or stream passed to a one-record call throws `ErrorCode.UsageRecordKind`. A null argument throws `ArgumentNullException`.

## The ten functions

Each snippet runs after the client line above. The many-record functions read the same three support messages:

```csharp
string[] messages = [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice."];
```

### decide: answer a yes-or-no question

```csharp
bool isRefund = await client.DecideAsync(
    "Does the customer request a refund?",
    "Please refund my order.");
```

### choose: pick the one option that fits

```csharp
string topic = await client.ChooseAsync(
    "What is the message about?",
    "Please refund my order.",
    ["billing", "shipping", "other"]);
```

### tag: label every option that fits

```csharp
ValueList<string> topics = await client.TagAsync(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    ["billing", "shipping"]);
```

### score: place the text on a scale from 0 to 1

```csharp
double urgency = await client.ScoreAsync(
    "How urgent is the request?",
    "Please refund my order.",
    ["routine", "urgent"]);
```

Levels run from lowest to highest. A score orders a queue a person reads. Use decide for a yes-or-no gate.

### filter: keep the records that answer yes

```csharp
await foreach (string refund in client.FilterAsync(
    "Does the customer request a refund?", messages.ToAsyncEnumerable()))
    Console.WriteLine(refund);
```

`FilterAsync` yields your own records, only the ones that answer yes. Records that are not sure are left out. When records fail, it yields every kept record first, then throws one `ThinkThenException` whose `FailedRows` lists every failed row with its `Position`. To see every record's outcome, call `FilterStreamAsync`, which yields a `FilterItem<T>` for each record with its `Item`, `Row` and `Kept`.

### rank: order every record, highest first

```csharp
await foreach (var ranked in client.RankAsync(
    "How urgent is the request?", messages.ToAsyncEnumerable(),
    ["routine", "urgent"]))
{
    long urgencyRank = ranked.Row;
    Console.WriteLine($"{urgencyRank} {ranked.Item}");
}
```

Each `RankedItem<T>` holds a copy of your record in `Item` and its row in `Row`. rank returns equal copies rebuilt from JSON, so `T` must round-trip through System.Text.Json; if it can't, rank checks your first record before any request is sent and throws a `JsonException` naming the type. filter returns the exact objects you passed. The row's answer is the rank, starting at 1; `ranked.Row.Position` is the record's place in your input. `new RankOptions { Top = n }` keeps the first `n` ranks and still reports failed records.

### find: point to the line that answers

```csharp
TextSpan? action = await client.FindAsync(
    "Which line states the requested action?",
    "Morgan works for Example Group.\nPlease refund my order.");
Console.WriteLine(action?.Text ?? "no line answers");
```

`null` means no line answers the question. Find picks one whole line, so keep the line breaks. A `TextSpan` gives the matched `Text` and its `Start` and `End` as UTF-16 offsets, with the end excluded.

### annotate: fill in several questions for each record

```csharp
using var refundQuestion = DecideQuestion.Create("Does the customer request a refund?");
using var topicQuestion = ChooseQuestion.Create("What is the message about?", ["billing", "shipping", "other"]);
using var urgencyQuestion = ScoreQuestion.Create("How urgent is the request?", ["routine", "urgent"]);
using var questions = AnnotationQuestions.Create(new Dictionary<string, AnnotationQuestion>
{
    ["refund"] = new AnnotationQuestion.Decide(refundQuestion),
    ["topic"] = new AnnotationQuestion.Choose(topicQuestion),
    ["urgency"] = new AnnotationQuestion.Score(urgencyQuestion),
});
await foreach (var form in client.AnnotateStreamAsync(questions, messages.ToAsyncEnumerable()))
{
    ValueMap<string, AnnotationEntry> answers = form;
    bool isRefund = answers["refund"];
    string topic = answers["topic"];
    double urgency = answers["urgency"];
    Console.WriteLine($"{form.Position} {isRefund} {topic} {urgency}");
}
```

Each question has its own outcome. Converting an entry to its answer throws on not sure or failure, as a row does, and each entry has `IsUnsure`, `IsFailed` and `Probability`. For one record, `client.AnnotateAsync(questions, message)` returns the row directly. Rank cannot go in `AnnotationQuestions`, and the compiler refuses it.

### recognize: pull out each name and its kind

```csharp
ValueList<Entity> names = await client.RecognizeAsync(
    "Morgan works for Example Group.",
    ["person", "organization"]);
foreach (Entity name in names)
    Console.WriteLine($"{name.Mention.Kind}: {name.Mention.Span.Text}");
```

### relate: link names by the relations you allow

```csharp
var worksFor = new RelationRule { Source = "person", Target = "organization", Name = "works_for" };
ValueList<Relation> relations = await client.RelateAsync(
    "Morgan works for Example Group.\nPlease refund my order.",
    [worksFor]);
foreach (Relation relation in relations)
    Console.WriteLine($"{relation.Source.Span.Text} {relation.Name} {relation.Target.Span.Text}");
```

A rule's optional `Reads` words the relation, and `Either = true` makes it undirected. Relate finds its own names, so it needs no recognize call first. [Relation rule fields](service.md#relation-rule-fields) describes each field.

## Judge many records

Every one-record method has a `…StreamAsync` twin, such as `DecideStreamAsync`, that takes an `IAsyncEnumerable<T>` and yields rows in input order. `FilterAsync`, `FilterStreamAsync` and `RankAsync` take many records already. Pass an array or list with .NET 10's `ToAsyncEnumerable()`, and a file with `File.ReadLinesAsync`. ThinkThen packs the records into batches; `ThinkThenOptions.Batch` sets the size. Use one stream for the whole collection rather than one awaited call per record.

A failed or not-sure record is a row like any other, so the stream keeps every record without stopping. This program reads `records.jsonl`, one JSON record per line, and judges each record's `text` field:

```csharp
using System.Text.Json;
using ThinkThen;

await using var client = new ThinkThenClient();
var records = File.ReadLinesAsync("records.jsonl")
    .Where(line => !string.IsNullOrWhiteSpace(line))
    .Select(line => JsonSerializer.Deserialize<JsonElement>(line));
var options = new DecideOptions { Field = Pointer.From("/text") };
await foreach (var refundCheck in client.DecideStreamAsync(
    "Does the customer request a refund?", records, options))
{
    if (refundCheck is Failed<bool, NoDetail> failed)
        Console.WriteLine($"{failed.Position} failed {failed.Error.Code}: {failed.Error.Message}");
    else if (refundCheck.IsUnsure)
        Console.WriteLine($"{refundCheck.Position} unsure: ask a person");
    else
    {
        bool isRefund = refundCheck;
        Console.WriteLine($"{refundCheck.Position} answered {isRefund}");
    }
}
```

`Position` counts from 1 in input order. Each row also carries `Id`, the record's own `id` as a `StringId`, `SignedId` or `UnsignedId`, and `Source`, the question, backend and model that answered. Inside a stream, blank text gives a failed row with `ErrorCode.RecordEmpty`, and the stream continues.

## Say what yes, no and each option mean

Decide and filter take `YesMeans` and `NoMeans`. Choose, tag, score and rank take `Descriptions`, a map from each option or level to what it means. The answers keep their short names.

```csharp
var refundOptions = new DecideOptions
{
    YesMeans = "The customer asks for money back",
    NoMeans = "The customer asks for anything else",
};
var topicOptions = new ChooseOptions
{
    Descriptions = new Dictionary<string, string>
    {
        ["billing"] = "Invoices, charges and refunds",
        ["shipping"] = "Delivery and parcels",
        ["other"] = "Anything else",
    },
};
string topic = await client.ChooseAsync(
    "What is the message about?", "Please refund my order.",
    ["billing", "shipping", "other"], topicOptions);
```

A description key that is not one of the options throws `ThinkThenException`.

## Records and field

A `string` record stays text, even when it holds JSON. Any other record is serialized with System.Text.Json, using `ThinkThenOptions.Json`; a `JsonElement`, a dictionary or your own class all work. `Pointer.From("/text")` in an options record's `Field` tells ThinkThen to judge one field. In a pointer, write `~1` for a `/` in a key and `~0` for a `~`.

A null record, a record without the field the pointer selects, or a field that holds JSON null gives a failed row with `ErrorCode.RecordMissing` and sends no request.

For trimmed or Native AOT applications, every record-taking method has a `…WithMetadataAsync` form, such as `DecideWithMetadataAsync`, that takes a `JsonTypeInfo<T>` from your `JsonSerializerContext`.

## Read the probability and details

`Probability` is a `double?` on every row, and null on a failed row. An `Answered` or `Unsure` row also has a typed `Detail`:

- Choose and tag give `LabelsDetail`, with each label's probability in `Labels`.
- Score and rank give `LevelsDetail`, with each level's probability in `Levels`, in level order.
- Recognize gives `RecognizeDetail`, and relate gives `RelateDetail` with every name it found in `Names` and every scored pair in `Pairs`.

```csharp
var topicCheck = await client.ChooseAsync(
    "What is the message about?", "Please refund my order.",
    ["billing", "shipping", "other"]);
if (topicCheck is Answered<string, LabelsDetail> { Detail: { } detail })
    foreach (var (label, probability) in detail.Labels)
        Console.WriteLine($"{label} {probability}");
```

Rows, `ValueList` and `ValueMap` compare by their contents, so two rows with the same data are equal and hash alike. `JsonSerializer.Serialize(row)` writes the row in the shape every ThinkThen language shares.

## Save and load a toolbox

A toolbox is a file of named questions. Save this as `toolbox.json`:

```json
{"entries":{"refund":{"question":{"function":"filter","question":"Does the customer request a refund?"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"]}}}}
```

Load it, pick a question by name and type, and pass that question to its function:

```csharp
using var toolbox = Toolbox.FromJson(await File.ReadAllTextAsync("toolbox.json"));
using var topicQuestion = toolbox.Choose("topic");
await foreach (var topicCheck in client.ChooseStreamAsync(topicQuestion, messages.ToAsyncEnumerable()))
{
    string topic = topicCheck;
    Console.WriteLine($"{topicCheck.Position} {topic}");
}
```

`Toolbox.FromJson(json)` reads this saved format, and `ToJson()` writes it. `Toolbox.Parse(json)` reads bare definitions with no `entries` wrapper, such as `{"topic":{"function":"choose", ...}}`. `Toolbox.Create(questions)` makes one in code. A typed selector, such as `toolbox.Choose(name)`, throws `ThinkThenException` for a missing name or a question of another function. Toolboxes and questions hold native resources, so dispose them with `using`.

## Cancel, dispose and register the client

Every async method takes a final `CancellationToken`, and `WithCancellation` passes one to a stream. Cancelling throws `OperationCanceledException` and stops the native work. `await using` disposes the client, which cancels its running calls; a call after that throws `ObjectDisposedException`.

In an ASP.NET Core or generic host application, `services.AddThinkThen(options => options.Model = "...")` registers one client for the application. It logs to `ILogger<ThinkThenClient>` without record text, settings or keys.

## Reference

[Read the API reference](../../sdk/csharp/API.md). Every public type and member has XML documentation in the package, so your editor shows it on hover.
