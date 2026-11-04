using System.Text.Json;
using System.Text.Json.Serialization;
namespace ThinkThen;
/// <summary>Selects the service that judges field.</summary>
[JsonConverter(typeof(BackendConverter))]
public enum Backend
{
    /// <summary>SystemOne backend value.</summary>
    SystemOne = 1,
    /// <summary>OpenAI backend value.</summary>
    OpenAI = 2,
    /// <summary>Stub backend value.</summary>
    Stub = 3,
}
/// <summary>Identifies one of the ten question functions.</summary>
public enum Function
{
    /// <summary>Decide function value.</summary>
    Decide = 1,
    /// <summary>Choose function value.</summary>
    Choose,
    /// <summary>Tag function value.</summary>
    Tag,
    /// <summary>Score function value.</summary>
    Score,
    /// <summary>Filter function value.</summary>
    Filter,
    /// <summary>Rank function value.</summary>
    Rank,
    /// <summary>Find function value.</summary>
    Find,
    /// <summary>Recognize function value.</summary>
    Recognize,
    /// <summary>Relate function value.</summary>
    Relate,
    /// <summary>Annotate function value.</summary>
    Annotate,
}
/// <summary>Distinguishes an answer, uncertainty, and a failed record.</summary>
public enum RowStatus
{
    /// <summary>Answered rowstatus value.</summary>
    Answered = 1,
    /// <summary>Unsure rowstatus value.</summary>
    Unsure,
    /// <summary>Failed rowstatus value.</summary>
    Failed,
}
/// <summary>Groups shared engine errors by their cause.</summary>
public enum ErrorFamily
{
    /// <summary>Usage errorfamily value.</summary>
    Usage,
    /// <summary>Record errorfamily value.</summary>
    Record,
    /// <summary>Backend errorfamily value.</summary>
    Backend,
    /// <summary>Local errorfamily value.</summary>
    Local,
    /// <summary>Internal errorfamily value.</summary>
    Internal,
}
/// <summary>Identifies a token boundary within a recognized name.</summary>
public enum BoundaryTag
{
    /// <summary>Begin boundarytag value.</summary>
    Begin,
    /// <summary>End boundarytag value.</summary>
    End,
    /// <summary>Inside boundarytag value.</summary>
    Inside,
    /// <summary>Out boundarytag value.</summary>
    Out,
    /// <summary>Single boundarytag value.</summary>
    Single,
}
/// <summary>Holds a credential and redacts it from printed output.</summary>
[System.Diagnostics.DebuggerDisplay("[redacted]")]
public sealed class ApiKey
{
    [System.Diagnostics.DebuggerBrowsable(System.Diagnostics.DebuggerBrowsableState.Never)]
    internal string Value { get; }
    private ApiKey(string value) => Value = value;
    /// <summary>Creates a typed value from the supplied input.</summary>
    public static ApiKey From(string value)
    { ArgumentNullException.ThrowIfNull(value); return new(value); }
    /// <inheritdoc/>
    public override string ToString() => "[redacted]";
}
/// <summary>Configures a client and its record serializer.</summary>
[JsonConverter(typeof(ThinkThenOptionsConverter))]
public sealed class ThinkThenOptions
{
    /// <summary>Selects an explicit backend; null uses THINKTHEN_BACKEND or the engine default.</summary>
    public Backend? Backend { get; set; }
    /// <summary>Selects the backend model; null uses its default.</summary>
    public string? Model { get; set; }
    /// <summary>Sets the backend address; null uses its default.</summary>
    public Uri? Url { get; set; }
    /// <summary>Supplies a credential; null uses the selected backend's environment variable.</summary>
    [JsonIgnore]
    public ApiKey? ApiKey { get; set; }
    /// <summary>Sets the record batch size; null uses the engine default.</summary>
    public int? Batch { get; set; }
    /// <summary>Sets retry options; null uses engine defaults.</summary>
    public RetryOptions? Retry { get; set; }
    /// <summary>Gets the json value.</summary>
    [JsonIgnore]
    public JsonSerializerOptions Json { get; set; } = new();
    /// <inheritdoc/>
    public override string ToString() => nameof(ThinkThenOptions);
}
/// <summary>Configures retry count, backoff and attempt timeout.</summary>
public sealed record RetryOptions
{
    /// <summary>Sets the number of retries after the initial attempt.</summary>
    public int? MaxRetries { get; init; }
    /// <summary>Sets the first retry delay.</summary>
    public TimeSpan? InitialBackoff { get; init; }
    /// <summary>Sets the maximum duration of one backend attempt.</summary>
    public TimeSpan? AttemptTimeout { get; init; }
}
/// <summary>Selects the probability cutoff or uncertainty band for a question.</summary>
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
    /// <summary>Creates a single probability cutoff.</summary>
    public static Threshold Cut(double value) => global::ThinkThen.Cut.From(value);
    /// <summary>Creates an uncertainty band between the low and high probabilities.</summary>
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
/// <summary>A single probability cutoff.</summary>
public sealed record Cut : Threshold
{
    /// <summary>Gets the underlying value.</summary>
    public double Value { get; }
    private Cut(double value) => Value = value;
    /// <summary>Creates a typed value from the supplied input.</summary>
    public static Cut From(double value) => new(value);
    private protected override void Closed() { }
    internal override void Write(Utf8JsonWriter writer) => Number(writer, Value);
}
/// <summary>An field selector expressed as a JSON Pointer.</summary>
public sealed record Pointer
{
    /// <summary>Gets the underlying value.</summary>
    public string Value { get; }
    private Pointer(string value) => Value = value;
    /// <summary>Creates a typed value from the supplied input.</summary>
    public static Pointer From(string value) => new(Json.HostText(value));
}
/// <summary>Describes a relation between source and target kinds.</summary>
public sealed record RelationRule
{
    /// <summary>Names the recognized kind at the source end of this relation.</summary>
    public required string Source { get; init; }
    /// <summary>Names the recognized kind at the target end of this relation.</summary>
    public required string Target { get; init; }
    /// <summary>Gets or sets the relation.</summary>
    [JsonPropertyName("relation")]
    public required string Name { get; init; }
    /// <summary>Gets or sets the reads.</summary>
    public string? Reads { get; init; }
    /// <summary>Gets or sets the either.</summary>
    public bool Either { get; init; }
}
/// <summary>Matched text and its UTF-16 start and end offsets.</summary>
public sealed record TextSpan(string Text, int Start, int End);
/// <summary>A matched span and its recognized kind.</summary>
public sealed record Mention(TextSpan Span, string Kind);
/// <summary>A recognized mention and its probability.</summary>
public sealed record Entity(Mention Mention, double Probability);
/// <summary>A relation between two mentions and its probability.</summary>
public sealed record Relation(Mention Source, Mention Target, string Name, double Probability, bool Either);
/// <summary>Probabilities keyed by label.</summary>
public sealed record LabelsDetail(ValueMap<string, double> Labels)
{
    /// <inheritdoc/>
    public bool Equals(LabelsDetail? other) => ValueEquality.Equal(this, other);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(this);
}
/// <summary>Probabilities in the question's level order.</summary>
public sealed record LevelsDetail(ValueList<double> Levels)
{
    /// <inheritdoc/>
    public bool Equals(LevelsDetail? other) => ValueEquality.Equal(this, other);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(this);
}
/// <summary>A text span and its token boundary probabilities.</summary>
public sealed record PieceDetail(TextSpan Span, ValueMap<BoundaryTag, double> Tags)
{
    /// <inheritdoc/>
    public bool Equals(PieceDetail? other) => ValueEquality.Equal(this, other);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(this);
}
/// <summary>Kind probabilities for a recognized span.</summary>
public sealed record KindDetail(string Text, int Start, int End, ValueMap<string, double> Kinds)
{
    /// <inheritdoc/>
    public bool Equals(KindDetail? other) => ValueEquality.Equal(this, other);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(this);
}
/// <summary>Token boundary and kind probabilities for recognition.</summary>
public sealed record RecognizeDetail(ValueList<PieceDetail> Pieces, ValueList<KindDetail> Names)
{
    /// <inheritdoc/>
    public bool Equals(RecognizeDetail? other) => ValueEquality.Equal(this, other);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(this);
}
/// <summary>Recognized names and scored relations.</summary>
public sealed record RelateDetail(ValueList<Entity> Names, ValueList<Relation> Pairs)
{
    /// <inheritdoc/>
    public bool Equals(RelateDetail? other) => ValueEquality.Equal(this, other);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(this);
}
/// <summary>Marks functions that do not return additional detail.</summary>
public sealed class NoDetail { private NoDetail() { } }
/// <summary>Identifies the question, backend and model that produced a row.</summary>
public sealed record Source(string QuestionIdentity, Backend Backend, string Model, string? Name = null);
/// <summary>An engine error with its stable code, message and optional HTTP status.</summary>
public sealed record RowError(ErrorCode ErrorCode, string Message, int? HttpStatus = null)
{
    /// <summary>Gets the canonical string error code from the shared engine registry.</summary>
    [JsonIgnore]
    public string Code => Contract.CodeName((int)ErrorCode) ?? throw Json.Abi();
    /// <summary>Gets the host exception that caused conversion to fail.</summary>
    [JsonIgnore]
    public Exception? Cause { get; init; }
}
/// <summary>Signals a shared engine or record failure.</summary>
public sealed class ThinkThenException : Exception
{
    /// <summary>Gets the stable shared error code.</summary>
    public ErrorCode ErrorCode { get; }
    /// <summary>Gets the canonical string error code from the shared engine registry.</summary>
    public string Code => Contract.CodeName((int)ErrorCode) ?? throw Json.Abi();
    /// <summary>Gets the HTTP status when the backend supplied one.</summary>
    public int? HttpStatus { get; }
    /// <summary>Gets every failed filter row when kept-only filtering raises after its kept records; otherwise empty.</summary>
    public ValueList<Row<bool, NoDetail>> FailedRows { get; }
    internal ThinkThenException(RowError error, Exception? cause = null, ValueList<Row<bool, NoDetail>>? failedRows = null) : base(error.Message, cause)
    { ErrorCode = error.ErrorCode; HttpStatus = error.HttpStatus; FailedRows = failedRows ?? ValueList<Row<bool, NoDetail>>.Empty; }
}
/// <summary>An optional string or exact integer record identifier.</summary>
public abstract record RecordId
{
    internal RecordId() { }
    private protected abstract void Closed();
}
/// <summary>A string record identifier.</summary>
public sealed record StringId(string Value) : RecordId { private protected override void Closed() { } }
/// <summary>A signed integer record identifier.</summary>
public sealed record SignedId(long Value) : RecordId { private protected override void Closed() { } }
/// <summary>An unsigned integer record identifier.</summary>
public sealed record UnsignedId(ulong Value) : RecordId { private protected override void Closed() { } }
