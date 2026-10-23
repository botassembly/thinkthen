# C# public API proposal

These declarations describe the API for review. They are not source files. All types use the `ThinkThen` namespace. Public DTOs are immutable; collection inputs are copied at admission and collection outputs cannot be mutated, including through a cast to an array or mutable collection. Every retained `JsonElement`, including records, questions, identifiers and answers, owns its backing data independently of a caller's `JsonDocument`. Lazy detail decoding retains its owned JSON data for the row's lifetime and is safe for concurrent readers.

```csharp
public sealed class ThinkThenClient : IDisposable
{
    public ThinkThenClient(ClientSettings? settings = null);

    public Task<ResultRow<bool?>> DecideAsync(Question question, Record record, CallOptions? options = null, CancellationToken cancellationToken = default);
    public Task<ResultRow<string>> ChooseAsync(Question question, Record record, IReadOnlyList<string>? options = null, CallOptions? settings = null, CancellationToken cancellationToken = default);
    public Task<ResultRow<IReadOnlyList<string>>> TagAsync(Question question, Record record, IReadOnlyList<string>? labels = null, CallOptions? options = null, CancellationToken cancellationToken = default);
    public Task<ResultRow<double?>> ScoreAsync(Question question, Record record, IReadOnlyList<string>? levels = null, CallOptions? options = null, CancellationToken cancellationToken = default);
    public IAsyncEnumerable<Record> FilterAsync(Question question, IAsyncEnumerable<Record> records, CallOptions? options = null, CancellationToken cancellationToken = default);
    public IAsyncEnumerable<ResultRow<long?>> RankAsync(Question question, IAsyncEnumerable<Record> records, RankOptions? options = null, CancellationToken cancellationToken = default);
    public Task<ResultRow<TextSpan>> FindAsync(Question question, Record record, CallOptions? options = null, CancellationToken cancellationToken = default);
    public Task<ResultRow<JsonElement?>> AnnotateAsync(JsonElement toolbox, Record record, CallOptions? options = null, CancellationToken cancellationToken = default);
    public Task<ResultRow<IReadOnlyList<RecognizedName>>> RecognizeAsync(Question question, Record record, CallOptions? options = null, CancellationToken cancellationToken = default);
    public Task<ResultRow<IReadOnlyList<RelationEdge>>> RelateAsync(Question question, Record record, CallOptions? options = null, CancellationToken cancellationToken = default);
}
```

Choose, tag and score omit the domain override when its argument is omitted or null. This lets a saved question supply its own domain. `RankOptions.Levels` passes inline rank levels to the engine.

The eight single-record methods also have `DecideStreamAsync`, `ChooseStreamAsync`, `TagStreamAsync`, `ScoreStreamAsync`, `FindStreamAsync`, `AnnotateStreamAsync`, `RecognizeStreamAsync` and `RelateStreamAsync` forms. Each replaces `Record record` with `IAsyncEnumerable<Record> records` and returns `IAsyncEnumerable<ResultRow<TAnswer>>` with its corresponding answer type. These methods forward into one internal call driver. No separate session or request builder is public. Cancellation tokens also follow `WithCancellation` at enumeration. When the method token and enumeration token differ, the driver links them for the operation and source; either can cancel, and the linked token source is disposed with the operation.

```csharp
public sealed record ClientSettings
{
    public string? Mode { get; init; }
    public string? Adapter { get; init; }
    public string? Url { get; init; }
    public string? KeySource { get; init; }
    public string? Model { get; init; }
    public double? Batch { get; init; }
}

public record CallOptions
{
    public Threshold? Threshold { get; init; }
    public string? On { get; init; }
}
public sealed record RankOptions : CallOptions
{
    public double? Top { get; init; }
    public IReadOnlyList<string>? Levels { get; init; }
}

public readonly record struct Threshold
{
    public static implicit operator Threshold(double value);
    public static implicit operator Threshold(string value);
}

public sealed class Record
{
    public static Record Text(string value);
    public static Record Json(JsonElement value);
    public static implicit operator Record(string value);
    public bool IsText { get; }
    public string? TextValue { get; }
    public JsonElement? JsonValue { get; }
}

public sealed class Question
{
    public static Question Inline(string text);
    public static Question Kinds(params string[] kinds);
    public static Question Relations(IReadOnlyList<RelationRule> rules);
    public static Question Toolbox(JsonElement value, string? name = null);
    public static Question Reference(string name);
    public static implicit operator Question(string text);
}
public sealed record RelationRule(string Name, string Source, string Target,
    string? Reads = null, bool? Either = null);

public enum RowStatus { Answered, Unsure, Failed }
public sealed class ResultRow<TAnswer>
{
    public JsonElement Id { get; }
    public RowStatus Status { get; }
    public TAnswer? Answer { get; }
    public double? P { get; }
    public ThinkThenError? Error { get; }
    public JsonElement? Detail { get; }
}
public sealed record ThinkThenError(string Code, string Message);
public sealed class ThinkThenException : Exception
{
    public string Code { get; }
}
public sealed record TextSpan(string Text, int Start, int End);
public sealed record RecognizedName(string Text, int Start, int End,
    string Kind, double P);
public sealed record RelationName(string Text, int Start, int End, string Kind);
public sealed record RelationEdge(RelationName Source, RelationName Target,
    string Relation, double P, bool? Either);
```

`Threshold` stores the supplied number or band text and serializes it without validation. The engine owns admissibility. Numeric settings retain the contract's numeric domain and fail there with its code. Question factories only choose the contract's tagged representation. Relation endpoints follow the row schema as immutable names. Annotate answers remain plain JSON because each toolbox defines its keys and answer types. For its value-type answer, `ResultRow<JsonElement?>` uses null for an unsure or failed answer, just like the other answer types. An answered annotate result holds its ordinary JSON object. The nullable value types in the method signatures are intentional: an unconstrained generic `TAnswer?` alone does not turn a value type into `Nullable<TAnswer>`. `Status` distinguishes an unsure row from a failed row.

`ThinkThenException` represents engine operation failures with their original dotted code and message. Failed stream rows contain `ThinkThenError`. Host failures such as missing native assets use the corresponding .NET loader exceptions. Cancellation associated with a caller's token uses `OperationCanceledException`; a native cancellation with no cancelled caller token retains the engine's coded exception. Calling a disposed client uses `ObjectDisposedException`.

## Driver and ownership

P/Invoke declarations follow the shipped C header. A SafeHandle owns each client and call. One asynchronous driver serializes each call: run blocking native pulls on a worker thread, send one record after `NeedsInput`, close input when exhausted, and drain output. Source enumeration uses `GetAsyncEnumerator` with the effective cancellation token; it never blocks a worker thread waiting synchronously on `MoveNextAsync`. Initial records are not duplicated into both the request and the source. The driver sends text as counted UTF-16 for shared strict decoding and supplies the UTF-16 offset unit. It uses native position and selection metadata to retain filter object identity without recomputing any selection or failure rule.

A cancellation registration may call native cancel while a pull blocks. It also wakes an asynchronous wait for a source record. If a source ignores its token, cancellation stops awaiting its pending `MoveNextAsync`, cancels and releases the native call, and returns to the caller without waiting for that source. The pending source task remains observed and its enumerator is disposed when that advance finishes; disposal never races an active advance. The library cannot force arbitrary source code to terminate. Normal completion, source failure and an early loop break dispose the source once; any asynchronous source disposal receives the same cancellation treatment. A source or disposal failure propagates unchanged when it is the first failure and never replaces an earlier failure. Cleanup unregisters cancellation, waits for the host driver to leave native operations, then frees the call. It does not wait for backend workers. Every native row/error buffer is copied and freed once in a finally block. Normal fields deserialize once; detail bytes wait until `Detail` is read. No engine thread calls managed code. The source advances only when the engine requests input. SafeHandle references keep the client alive across call admission and keep the call alive across native operations and cancellation callbacks. Client disposal cannot race admission into a freed native pointer. The native asset remains loaded for the process lifetime.
