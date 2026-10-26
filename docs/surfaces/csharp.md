# ThinkThen in C#

Single Async methods return Task rows; StreamAsync methods and filter/rank return asynchronous streams. Arrays remain valid single records in single methods. `await foreach` closes traversal; `await using` closes clients.

Check `README.md` inside the NuGet package for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Use .NET 10. Install the supplied NuGet package from its local directory and restore its Microsoft.Extensions 10.0 dependencies from NuGet.org. A prepared local feed containing those dependencies can replace the public source:

Run `dotnet new console -n Example`, then `cd Example`, then `dotnet add package Botassembly.ThinkThen --version 0.2.0 --no-restore`, then `dotnet restore --source /path/to/packages --source https://api.nuget.org/v3/index.json`.

The package includes the native engine. Keep its runtime assets intact. Set `THINKTHEN_API_KEY` in the process environment before creating a normal client. Keep keys out of source and reports. Default options select the default backend. `Backend.Stub` needs no key or network and returns synthetic installation-check answers.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```csharp
using System.Text.Json;
using ThinkThen;

static async IAsyncEnumerable<JsonElement> ReadRecords(string path) {
    using var source = File.OpenText(path);
    while (await source.ReadLineAsync() is { } line) {
        if (string.IsNullOrWhiteSpace(line)) continue;
        using var document = JsonDocument.Parse(line);
        yield return document.RootElement.Clone();
    }
}

await using var client = new ThinkThenClient(new ThinkThenOptions { Backend = Backend.Stub });
var array = await client.DecideAsync("Does the customer request a refund?", new[] { "first", "second" });
Console.WriteLine($"array {array.Status}");
await foreach (var row in client.DecideStreamAsync("Does the customer request a refund?",
    ReadRecords("records.jsonl"), new DecideOptions { Evidence = Pointer.From("/text") })) {
    var message = row is Failed<bool, NoDetail> failed ? failed.Error.Message : "";
    bool? answer = row is Answered<bool, NoDetail> answered ? answered.Answer : null;
    Console.WriteLine($"{row.Position} {row.Status} {answer} {message}");
}
```

## Records and the ten functions

Strings stay raw text. Structured values use the client's snapshotted JsonSerializerOptions. Null records return missing-record rows. Structured records can carry string or exact signed/unsigned 64-bit IDs. An evidence pointer uses `Pointer.From("/text")`; escape `/` as `~1` and `~` as `~0` in keys. A trimmed or Native AOT application must provide generated JsonTypeInfo or a generated resolver.

| Method | Required domain | Answer |
| --- | --- | --- |
| DecideAsync | Wording and record | bool |
| ChooseAsync | Wording, record, choices | string |
| TagAsync | Wording, record, labels | ImmutableArray<string> |
| ScoreAsync | Wording, record, levels | double from 0 to 1 |
| FilterAsync | Wording and record stream | FilterItem with original item and bool row |
| RankAsync | Wording, record stream, levels | RankedItem with original item and rank row |
| FindAsync | Wording and record | TextSpan?; null for no match |
| RecognizeAsync | Record and kinds | ImmutableArray<Entity> |
| RelateAsync | Record and rules | ImmutableArray<Relation> |
| AnnotateAsync | AnnotationQuestions and record | ImmutableDictionary<string, AnnotationEntry> |

Use wording such as `"Does the customer request a refund?"`. Recognize and relate take domains without prompts. Build rules with `new RelationRule { Source = "person", Target = "organization", Relation = "works_for" }`; optional Reads supplies wording and Either makes the relation undirected. Every function has its own options record. Decide, choose and filter accept `Threshold.Cut(value)` or `Threshold.Band(low, high)`; tag, find, recognize and relate accept `Cut.From(value)` only. A decide/filter band yields false below low, unsure inside and true at or above high. Choose needs its winner to reach the upper cut. Score and rank levels go lowest to highest. `RankOptions.Top` limits successful ranks and retains failures.

A rank row's answer is its rank; `Position` is its input order. Rank places highest scores first. Filter returns every input outcome; `Kept` is true only for answered true. Preserve IDs or input positions when joining results to input.

## Rows, streams and saved questions

`Row<TAnswer,TDetail>` has Answered, Unsure and Failed cases. Pattern match before using answers. Answered exposes Answer, optional Probability and typed Detail. Failed exposes RowError with code, message and optional HTTP status. All rows expose Function, Position, optional RecordId, Source and Status. Source identifies the question, backend and served model. Details include LabelsDetail, LevelsDetail, RecognizeDetail and RelateDetail. Find can be answered with no match. TextSpan uses UTF-16 offsets from zero with an exclusive end. Find selects a whole nonblank line, so keep line breaks.

Single methods return Task rows. Decide, choose, tag, score, find, recognize, relate and annotate have matching StreamAsync forms taking IAsyncEnumerable records. Filter and rank already stream. Use one stream over the collection so the engine can batch; do not await a single call per row. ThinkThenOptions.Batch controls packing; byte and backend limits also apply. One record may require several backend questions.

```csharp
await foreach (var result in client.DecideStreamAsync(
    "Does the customer request a refund?", records,
    cancellationToken: stop.Token)) {
    if (result is Failed<bool, NoDetail> failure)
        throw new Exception($"{failure.Error.Code}: {failure.Error.Message}");
    Console.WriteLine(result.Position);
}
```

Here `records` is the application's IAsyncEnumerable and `stop` is a CancellationTokenSource. `await foreach` disposes traversal on an early break or error.

Typed factories such as `ChooseQuestion.Create(question, choices, options)` snapshot domains. Typed method overloads take the admitted question without replacement options. `Toolbox.FromJson(json)` reads saved question `entries`; `ToJson()` saves them. `Toolbox.Create` stores typed ToolboxQuestion values; typed selectors return questions for matching methods. `AnnotationQuestions` accepts the narrower AnnotationQuestion union and excludes rank. An answered annotation can contain failed or unsure entries; inspect every EntryOutcome independently.

Rows support System.Text.Json serialization with the shared row shape, exact IDs and full probability precision. Use your application's JSON, CSV or spreadsheet library for files. There is no shell toolbox-run API or spreadsheet reader in the SDK. Load a saved question toolbox, run a collection, inspect failures and save results while retaining IDs.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```csharp
var box = Toolbox.FromJson(await File.ReadAllTextAsync("toolbox.json"));
await File.WriteAllTextAsync("toolbox-copy.json", box.ToJson());
var topic = box.Choose("topic");
await using var client = new ThinkThenClient(new ThinkThenOptions { Backend = Backend.Stub });
await foreach (var row in client.ChooseStreamAsync(topic, ReadRecords("records.jsonl"))) {
    if (row is Failed<string, LabelsDetail> failed)
        throw new Exception($"{failed.Error.Code}: {failed.Error.Message}");
    Console.WriteLine(row);
}
```

This code reuses the first example's `ReadRecords` function. `box.Filter("ready")` and the other typed selectors return the matching question or report a library usage error. A selected question owns its evidence and choices.

Call `RelateAsync` or `RelateStreamAsync` alone when you need names and relations from the same evidence. After checking that an answered row has details, `answered.Detail.Names` contains all admitted names, including names with no returned relation, and `answered.Detail.Pairs` contains scored relation pairs. Read accepted relations from `answered.Answer`. A preceding recognize call is unnecessary.

## Failures and cancellation

Record and backend failures return Failed, including single calls. They do not throw automatically. Inspect them before saving or chaining. Throw or stop iteration when a failure must stop the workload, and inspect nested annotation entries too.

Usage, setup and internal failures throw ThinkThenException with typed ErrorCode and optional HTTP status. Pass a CancellationToken to each call, or use WithCancellation on its stream. Cancelling throws OperationCanceledException. `await using` closes the client, cancels active work and waits for native work to finish.

Make questions unambiguous. Define the evidence criterion, distinguish absence from explicit negatives and choose distinct labels. The stub checks installation, not question quality.
