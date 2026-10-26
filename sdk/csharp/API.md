# C# API

The `ThinkThen` namespace targets .NET 10. Create `ThinkThenClient` with `ThinkThenOptions`, or register a singleton with `services.AddThinkThen(options => ...)`. The client snapshots settings and serializer options. Dispose it with `await using`. An optional `ILogger<ThinkThenClient>` receives lifecycle events without request bodies, settings or credentials.

Settings use `Backend`, `Model`, `Url`, `ApiKey`, `Batch`, `Retry` and `Json`. Construct a direct key with `ApiKey.From(value)`. Diagnostics and serialized settings redact the key. Settings JSON admits keys through the engine. `RetryOptions` uses `MaxRetries`, `InitialBackoff` and `AttemptTimeout`. Durations must represent whole milliseconds.

Single calls return `Task<Row<TAnswer,TDetail>>`. Streams take `IAsyncEnumerable<TRecord>` and return `IAsyncEnumerable<Row<TAnswer,TDetail>>`. Each operation accepts an optional execution `name` and a final `CancellationToken`. `WithCancellation` also reaches the input enumerator and native operation.

`DecideAsync`, `ChooseAsync`, `TagAsync`, `ScoreAsync`, `FindAsync`, `RecognizeAsync`, `RelateAsync` and `AnnotateAsync` have matching `StreamAsync` methods. Choose requires choices; tag requires labels; score and rank require levels; recognize requires kinds; relate requires relation rules. Recognize and relate take their domains without an invented question prompt. Each function has its own options record. Evidence uses `Pointer.From(value)`. Decide, choose and filter accept `Threshold.Cut(value)` or `Threshold.Band(low,high)`; the numeric-only functions accept `Cut.From(value)`.

Each method accepts either convenience arguments or its admitted typed question. For example, `ChooseQuestion.Create(question, choices, options)` snapshots the choices and options. Typed question overloads accept the question and records without replacement options. Questions and result unions are closed to external derivation.

`Row<TAnswer,TDetail>` has three cases:

- `Answered<TAnswer,TDetail>` exposes `Answer`, optional `Probability` and typed `Detail`.
- `Unsure<TAnswer,TDetail>` exposes `Probability` and typed `Detail`.
- `Failed<TAnswer,TDetail>` exposes `RowError`.

Every row exposes `Function`, `Position`, optional `RecordId`, `Source` and `Status`. IDs preserve strings and signed or unsigned 64-bit integers. Source identifies the attempted question, backend, model and optional execution name. Supplying a name changes question identity while preserving the answer. Toolbox selection supplies no execution name. Rows own their data after a call or client closes. Serialization preserves the shared row shape, full probability precision and exact IDs.

Details use immutable `LabelsDetail`, `LevelsDetail`, `RecognizeDetail` or `RelateDetail`. Entity and relation endpoints use `Mention` and `TextSpan`. Span offsets count UTF-16 units with an exclusive end. Annotate returns an immutable map of typed `AnnotationEntry` cases. Their `EntryOutcome<TAnswer,TDetail>` contains answered, unsure or failed entry data without outer row metadata.

`FilterAsync` yields `FilterItem<TRecord>` for every input row. `Item` preserves the original managed item; `Kept` is true only for an answered true. `RankAsync` yields `RankedItem<TRecord>` in engine order. Its row's rank answer remains distinct from its original input position. `RankOptions.Top` restricts successful rows while retaining failures.

`Toolbox.Parse(json)` delegates the flat question map to engine admission. Typed selectors return admitted questions with their required lists. `Toolbox.Create` accepts named `ToolboxQuestion` values, including rank. `AnnotationQuestions` accepts the narrower `AnnotationQuestion` union. Use it with `AnnotateAsync`, `AnnotateStreamAsync` or `client.Toolbox(questions)`. A general toolbox and rank cannot enter annotation through the typed API.

Record strings remain raw text; structured values use the snapshotted `JsonSerializerOptions`. Every record-taking overload also accepts `JsonTypeInfo<TRecord>`. A trimmed or Native AOT application supplies generated metadata or a generated resolver. Reflection serialization overloads carry the corresponding trimming and dynamic-code annotations.

Record and backend failures return failed rows. Null records use the engine's missing-record operation. Usage, setup and internal failures throw `ThinkThenException` with typed `ErrorCode`, optional HTTP status and an original host cause when available. The `Family()` extension exposes each code's family. Cancellation throws `OperationCanceledException`. Disposing the client cancels active operations, waits for native work to finish and rejects new admission. A paused stream cannot retain a native allocation after disposal. An uncooperative input advance remains observed and disposes after it returns.
