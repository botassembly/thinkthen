using System.Collections.Immutable;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace ThinkThen;
[JsonConverter(typeof(BackendConverter))]
public enum Backend { SystemOne = 1, OpenAI = 2, Stub = 3 }
public enum Function { Decide = 1, Choose, Tag, Score, Filter, Rank, Find, Recognize, Relate, Annotate }
public enum RowStatus { Answered = 1, Unsure, Failed }
public enum ErrorFamily { Usage, Record, Backend, Local, Internal }
public enum BoundaryTag { Begin, End, Inside, Out, Single }
[System.Diagnostics.DebuggerDisplay("[redacted]")]
public sealed class ApiKey
{
    [System.Diagnostics.DebuggerBrowsable(System.Diagnostics.DebuggerBrowsableState.Never)]
    internal string Value { get; }
    private ApiKey(string value) => Value = value;
    public static ApiKey From(string value) => new(value);
    public override string ToString() => "[redacted]";
}
[JsonConverter(typeof(ThinkThenOptionsConverter))]
public sealed class ThinkThenOptions
{
    public Backend Backend { get; set; } = Backend.SystemOne;
    public string? Model { get; set; }
    public Uri? Url { get; set; }
    [JsonIgnore] public ApiKey? ApiKey { get; set; }
    public int? Batch { get; set; }
    public RetryOptions? Retry { get; set; }
    [JsonIgnore] public JsonSerializerOptions Json { get; set; } = new();
    public override string ToString() => nameof(ThinkThenOptions);
}
public sealed record RetryOptions
{
    public int? MaxRetries { get; init; }
    public TimeSpan? InitialBackoff { get; init; }
    public TimeSpan? AttemptTimeout { get; init; }
}
public abstract record Threshold
{
    internal Threshold() { }
    private protected abstract void Closed();
    internal abstract void Write(Utf8JsonWriter writer);
    internal static void Number(Utf8JsonWriter writer, double value)
    {
        if (double.IsFinite(value)) writer.WriteNumberValue(value);
        else writer.WriteStringValue(value.ToString(System.Globalization.CultureInfo.InvariantCulture));
    }
    public static Threshold Cut(double value) => global::ThinkThen.Cut.From(value);
    public static Threshold Band(double low, double high)
    {
        return new BandThreshold(low, high);
    }
    private sealed record BandThreshold(double Low, double High) : Threshold
    {
        private protected override void Closed() { }
        internal override void Write(Utf8JsonWriter writer)
        { writer.WriteStartObject(); writer.WritePropertyName("low"); Number(writer, Low); writer.WritePropertyName("high"); Number(writer, High); writer.WriteEndObject(); }
    }
}
public sealed record Cut : Threshold
{
    public double Value { get; }
    private Cut(double value) => Value = value;
    public static Cut From(double value) => new(value);
    private protected override void Closed() { }
    internal override void Write(Utf8JsonWriter writer) => Number(writer, Value);
}
public sealed record Pointer
{
    public string Value { get; }
    private Pointer(string value) => Value = value;
    public static Pointer From(string value) => new(Json.HostText(value));
}
public sealed record RelationRule
{
    public required string Source { get; init; }
    public required string Target { get; init; }
    public required string Relation { get; init; }
    public string? Reads { get; init; }
    public bool Either { get; init; }
}
public sealed record TextSpan(string Text, int Start, int End);
public sealed record Mention(TextSpan Span, string Kind);
public sealed record Entity(Mention Mention, double Probability);
public sealed record Relation(Mention Source, Mention Target, string Name, double Probability, bool Either);
public sealed record LabelsDetail(ImmutableDictionary<string, double> Labels);
public sealed record LevelsDetail(ImmutableArray<double> Levels);
public sealed record PieceDetail(TextSpan Span, ImmutableDictionary<BoundaryTag, double> Tags);
public sealed record KindDetail(int Start, int End, ImmutableDictionary<string, double> Kinds);
public sealed record RecognizeDetail(ImmutableArray<PieceDetail> Pieces, ImmutableArray<KindDetail> Names);
public sealed record RelateDetail(ImmutableArray<Entity> Names, ImmutableArray<Relation> Pairs);
public sealed class NoDetail { private NoDetail() { } }
public sealed record Source(string QuestionIdentity, Backend Backend, string Model, string? Name = null);
public sealed record RowError(ErrorCode Code, string Message, int? HttpStatus = null)
{
    [JsonIgnore] public Exception? Cause { get; init; }
}
public sealed class ThinkThenException : Exception
{
    public ErrorCode Code { get; }
    public int? HttpStatus { get; }
    internal ThinkThenException(RowError error, Exception? cause = null) : base(error.Message, cause)
    { Code = error.Code; HttpStatus = error.HttpStatus; }
}
public abstract record RecordId
{
    internal RecordId() { }
    private protected abstract void Closed();
}
public sealed record StringId(string Value) : RecordId { private protected override void Closed() { } }
public sealed record SignedId(long Value) : RecordId { private protected override void Closed() { } }
public sealed record UnsignedId(ulong Value) : RecordId { private protected override void Closed() { } }
[JsonConverter(typeof(RowConverters))]
public abstract record Row<A, D>
{
    internal Row(Function function, ulong position, RecordId? id, Source source)
    { Function = function; Position = position; Id = id; Source = source; }
    private protected abstract void Closed();
    internal JsonElement Canonical { get; init; }
    public Function Function { get; }
    public ulong Position { get; }
    public RecordId? Id { get; }
    public Source Source { get; }
    public abstract RowStatus Status { get; }
}
public sealed record Answered<A,D> : Row<A,D>
{
    internal Answered(Function function, ulong position, RecordId? id, Source source, A answer, double? probability, D? detail)
        : base(function, position, id, source) { Answer = answer; Probability = probability; Detail = detail; }
    private protected override void Closed() { }
    public override RowStatus Status => RowStatus.Answered;
    public A Answer { get; }
    public double? Probability { get; }
    public D? Detail { get; }
}
public sealed record Unsure<A,D> : Row<A,D>
{
    internal Unsure(Function function, ulong position, RecordId? id, Source source, double probability, D? detail)
        : base(function, position, id, source) { Probability = probability; Detail = detail; }
    private protected override void Closed() { }
    public override RowStatus Status => RowStatus.Unsure;
    public double Probability { get; }
    public D? Detail { get; }
}
public sealed record Failed<A,D> : Row<A,D>
{
    internal Failed(Function function, ulong position, RecordId? id, Source source, RowError error)
        : base(function, position, id, source) => Error = error;
    private protected override void Closed() { }
    public override RowStatus Status => RowStatus.Failed;
    public RowError Error { get; }
}
public sealed record FilterItem<T>(T Item, Row<bool, NoDetail> Row)
{ public bool Kept => Row is Answered<bool, NoDetail> { Answer: true }; }
public sealed record RankedItem<T>(T Item, Row<long, LevelsDetail> Row);
public abstract record EntryOutcome<A,D>
{
    internal EntryOutcome() { }
    private protected abstract void Closed();
}
public sealed record EntryAnswered<A,D>(A Answer, double? Probability, D? Detail) : EntryOutcome<A,D>
{ private protected override void Closed() { } }
public sealed record EntryUnsure<A,D>(double Probability, D? Detail) : EntryOutcome<A,D>
{ private protected override void Closed() { } }
public sealed record EntryFailed<A,D>(RowError Error) : EntryOutcome<A,D>
{ private protected override void Closed() { } }
public abstract record AnnotationEntry
{
    internal AnnotationEntry() { }
    private protected abstract void Closed();
    public abstract Function Function { get; }
}
public sealed record DecideEntry(EntryOutcome<bool,NoDetail> Outcome) : AnnotationEntry { private protected override void Closed() { } public override Function Function => Function.Decide; }
public sealed record ChooseEntry(EntryOutcome<string,LabelsDetail> Outcome) : AnnotationEntry { private protected override void Closed() { } public override Function Function => Function.Choose; }
public sealed record TagEntry(EntryOutcome<ImmutableArray<string>,LabelsDetail> Outcome) : AnnotationEntry { private protected override void Closed() { } public override Function Function => Function.Tag; }
public sealed record ScoreEntry(EntryOutcome<double,LevelsDetail> Outcome) : AnnotationEntry { private protected override void Closed() { } public override Function Function => Function.Score; }
public sealed record FilterEntry(EntryOutcome<bool,NoDetail> Outcome) : AnnotationEntry { private protected override void Closed() { } public override Function Function => Function.Filter; }
public sealed record FindEntry(EntryOutcome<TextSpan?,NoDetail> Outcome) : AnnotationEntry { private protected override void Closed() { } public override Function Function => Function.Find; }
public sealed record RecognizeEntry(EntryOutcome<ImmutableArray<Entity>,RecognizeDetail> Outcome) : AnnotationEntry { private protected override void Closed() { } public override Function Function => Function.Recognize; }
public sealed record RelateEntry(EntryOutcome<ImmutableArray<Relation>,RelateDetail> Outcome) : AnnotationEntry { private protected override void Closed() { } public override Function Function => Function.Relate; }
