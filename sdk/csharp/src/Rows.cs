using System.Text.Json;
using System.Text.Json.Serialization;
namespace ThinkThen;
/// <summary>Signals that an answer falls inside the question's uncertainty band.</summary>
public sealed class UnsureException : InvalidOperationException
{
    /// <summary>The probability reported for the uncertain answer.</summary>
    public double Probability { get; }
    /// <summary>Create an uncertainty exception with the reported probability.</summary>
    public UnsureException(double probability) : base("The answer is not sure.") => Probability = probability;
}
/// <summary>A result that converts to its answer and retains source and outcome metadata.</summary>
[JsonConverter(typeof(RowConverters))]
public abstract record Row<TAnswer, TDetail>
{
    internal Row(Function function, ulong position, RecordId? id, Source source)
    { Function = function; Position = position; Id = id; Source = source; }
    private protected abstract void Closed();
    internal JsonElement Canonical { get; init; }
    /// <summary>Gets the question function.</summary>
    public Function Function { get; }
    /// <summary>Gets the one-based input position.</summary>
    public ulong Position { get; }
    /// <summary>Gets the original record identifier, if present.</summary>
    public RecordId? Id { get; }
    /// <summary>Gets the producing question, backend and model.</summary>
    public Source Source { get; }
    /// <summary>Gets the outcome status.</summary>
    public abstract RowStatus Status { get; }
    /// <summary>Gets the answer; uncertain answers throw UnsureException and failed answers throw ThinkThenException.</summary>
    public abstract TAnswer Answer { get; }
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public abstract double? Probability { get; }
    /// <summary>Reports whether this outcome contains an answer.</summary>
    public bool IsAnswered => Status == RowStatus.Answered;
    /// <summary>Reports whether this outcome is uncertain.</summary>
    public bool IsUnsure => Status == RowStatus.Unsure;
    /// <summary>Reports whether this outcome failed.</summary>
    public bool IsFailed => Status == RowStatus.Failed;
    /// <summary>Returns true and the answer only for an answered outcome; uncertainty and failure return false.</summary>
    public bool TryGetAnswer([System.Diagnostics.CodeAnalysis.MaybeNullWhen(false)] out TAnswer answer)
    {
        if (IsAnswered) { answer = Answer; return true; }
        answer = default; return false;
    }
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static implicit operator TAnswer(Row<TAnswer, TDetail> row)
    { ArgumentNullException.ThrowIfNull(row); return row.Answer; }
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static bool operator ==(Row<TAnswer, TDetail>? row, TAnswer answer) =>
        row is not null && row.IsAnswered && ValueEquality.Equal(row.Answer, answer);
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static bool operator !=(Row<TAnswer, TDetail>? row, TAnswer answer) => !(row == answer);
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static bool operator ==(TAnswer answer, Row<TAnswer, TDetail>? row) => row == answer;
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static bool operator !=(TAnswer answer, Row<TAnswer, TDetail>? row) => !(row == answer);
    /// <summary>Compares public values, including collection contents, without cached wire JSON or exception causes.</summary>
    public virtual bool Equals(Row<TAnswer, TDetail>? other) => other is not null &&
        GetType() == other.GetType() && Function == other.Function && Position == other.Position &&
        Equals(Id, other.Id) && Equals(Source, other.Source);
    /// <inheritdoc/>
    public override int GetHashCode() => HashCode.Combine(Function, Position, Id, Source, Status);
    /// <inheritdoc/>
    public sealed override string ToString() => this switch
    {
        Failed<TAnswer, TDetail> failed => $"failed: {failed.Error.Code}: {failed.Error.Message}",
        _ when IsUnsure => "unsure",
        _ => ValueEquality.Display(Answer)
    };
}
/// <summary>A row containing an answer.</summary>
public sealed record Answered<TAnswer,TDetail> : Row<TAnswer,TDetail>
{
    internal Answered(Function function, ulong position, RecordId? id, Source source, TAnswer answer, double? probability, TDetail? detail)
        : base(function, position, id, source) { Answer = answer; Probability = probability; Detail = detail; }
    private protected override void Closed() { }
    /// <summary>Gets the outcome status.</summary>
    public override RowStatus Status => RowStatus.Answered;
    /// <summary>Gets the answer; uncertain answers throw UnsureException and failed answers throw ThinkThenException.</summary>
    public override TAnswer Answer { get; }
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public override double? Probability { get; }
    /// <summary>Gets the function-specific probability detail, if present.</summary>
    public TDetail? Detail { get; }
    /// <summary>Compares public values, including collection contents, without cached wire JSON or exception causes.</summary>
    public bool Equals(Answered<TAnswer,TDetail>? other) => base.Equals(other) &&
        ValueEquality.Equal(Answer, other!.Answer) && Nullable.Equals(Probability, other.Probability) && ValueEquality.Equal(Detail, other.Detail);
    /// <inheritdoc/>
    public override int GetHashCode() => HashCode.Combine(base.GetHashCode(), ValueEquality.Hash(Answer), Probability, ValueEquality.Hash(Detail));
}
/// <summary>A row whose answer is uncertain; answer access throws UnsureException.</summary>
public sealed record Unsure<TAnswer,TDetail> : Row<TAnswer,TDetail>
{
    internal Unsure(Function function, ulong position, RecordId? id, Source source, double probability, TDetail? detail)
        : base(function, position, id, source) { Probability = probability; Detail = detail; }
    private protected override void Closed() { }
    /// <summary>Gets the outcome status.</summary>
    public override RowStatus Status => RowStatus.Unsure;
    /// <inheritdoc/>
    public override TAnswer Answer => throw new UnsureException(Probability!.Value);
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public override double? Probability { get; }
    /// <summary>Gets the function-specific probability detail, if present.</summary>
    public TDetail? Detail { get; }
    /// <summary>Compares public values, including collection contents, without cached wire JSON or exception causes.</summary>
    public bool Equals(Unsure<TAnswer,TDetail>? other) => base.Equals(other) &&
        Nullable.Equals(Probability, other!.Probability) && ValueEquality.Equal(Detail, other.Detail);
    /// <inheritdoc/>
    public override int GetHashCode() => HashCode.Combine(base.GetHashCode(), Probability, ValueEquality.Hash(Detail));
}
/// <summary>A failed row; answer access throws its ThinkThenException.</summary>
public sealed record Failed<TAnswer,TDetail> : Row<TAnswer,TDetail>
{
    internal Failed(Function function, ulong position, RecordId? id, Source source, RowError error)
        : base(function, position, id, source) => Error = error;
    private protected override void Closed() { }
    /// <summary>Gets the outcome status.</summary>
    public override RowStatus Status => RowStatus.Failed;
    /// <inheritdoc/>
    public override TAnswer Answer => throw new ThinkThenException(Error, Error.Cause);
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public override double? Probability => null;
    /// <summary>Gets the failed record's shared error.</summary>
    public RowError Error { get; }
    /// <summary>Compares public values, including collection contents, without cached wire JSON or exception causes.</summary>
    public bool Equals(Failed<TAnswer,TDetail>? other) => base.Equals(other) && ValueEquality.Equal(Error, other!.Error);
    /// <inheritdoc/>
    public override int GetHashCode() => HashCode.Combine(base.GetHashCode(), ValueEquality.Hash(Error));
}
/// <summary>An original input record and its filter outcome.</summary>
public sealed record FilterItem<T>(T Item, Row<bool, NoDetail> Row)
{
    /// <summary>Reports whether the filter answered yes.</summary>
    public bool Kept => Row is Answered<bool, NoDetail> { Answer: true }; }
/// <summary>An equal JSON snapshot of an input record and its rank outcome. The record type must support JSON reading and writing.</summary>
public sealed record RankedItem<T>(T Item, Row<long, LevelsDetail> Row);
