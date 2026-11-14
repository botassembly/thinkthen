using System.Text.Json;
using System.Text.Json.Serialization;
namespace ThinkThen;

/// <summary>A non-generic view of a result's safe metadata.</summary>
[JsonConverter(typeof(RowConverters))]
public interface IRow
{
    /// <summary>Gets this result metadata value.</summary>
    Function Function { get; }
    /// <summary>Gets this result metadata value.</summary>
    Status Status { get; }
    /// <summary>Gets this result metadata value.</summary>
    ulong Position { get; }
    /// <summary>Gets this result metadata value.</summary>
    RecordId? Id { get; }
    /// <summary>Gets this result metadata value.</summary>
    Source Source { get; }
    /// <summary>Gets this result metadata value.</summary>
    object? Item { get; }
    /// <summary>Gets this result metadata value.</summary>
    double? Probability { get; }
    /// <summary>Gets this result metadata value.</summary>
    Detail? Detail { get; }
    /// <summary>Gets this result metadata value.</summary>
    bool IsAnswered { get; }
    /// <summary>Gets this result metadata value.</summary>
    bool IsUnsure { get; }
    /// <summary>Gets this result metadata value.</summary>
    bool IsFailed { get; }
}
/// <summary>A failed result with its error and original record.</summary>
[JsonConverter(typeof(RowConverters))]
public interface IFailed : IRow
{
    /// <summary>The stable error of this failed record.</summary>
    RowError Error { get; }
}
internal interface IRowValue : IRow { JsonElement Canonical { get; } object? OwnAnswer { get; } object?[] OwnValues { get; } }
/// <summary>No answer was available, through uncertainty or record failure.</summary>
public abstract class NoAnswerException : Exception { internal NoAnswerException(string message) : base(message) {} }
/// <summary>The result is not sure.</summary>
public sealed class UnsureException : NoAnswerException
{
    /// <summary>The original unsure result.</summary>
    public IRow Row { get; }
    /// <summary>The aggregate probability, when present.</summary>
    public double Probability => Row.Probability!.Value;
    internal UnsureException(IRow row) : base("The answer is not sure.") => Row=row;
}
/// <summary>One or more records failed.</summary>
public sealed class FailedException : NoAnswerException
{
    /// <summary>Provides Code.</summary>
    public string Code { get; }
    /// <summary>Provides HttpStatus.</summary>
    public int? HttpStatus { get; }
    /// <summary>Every affected failed result.</summary>
    public ValueList<IFailed> Rows { get; }
    internal FailedException(IEnumerable<IFailed> rows) : this(new ValueList<IFailed>(rows)) {}
    private FailedException(ValueList<IFailed> rows) : base(rows[0].Error.Message)
    { Rows=rows; Code=rows[0].Error.Code; HttpStatus=rows[0].Error.HttpStatus; }
}
/// <summary>An immutable result and the caller's original record.</summary>
[JsonConverter(typeof(RowConverters))]
public abstract record Row<TAnswer,TRecord> : IRowValue
{
    internal Row(Function function, ulong position, RecordId? id, Source source, TRecord item)
    { Function=function; Position=position; Id=id; Source=source; Item=item; }
    private protected abstract void Closed();
    internal JsonElement Canonical { get; init; }
    JsonElement IRowValue.Canonical => Canonical;
    /// <summary>Provides Function.</summary>
    public Function Function { get; }
    /// <summary>Provides Position.</summary>
    public ulong Position { get; }
    /// <summary>Provides Id.</summary>
    public RecordId? Id { get; }
    /// <summary>Provides Source.</summary>
    public Source Source { get; }
    /// <summary>The caller's original record.</summary>
    public TRecord Item { get; }
    object? IRow.Item => Item;
    /// <summary>The result state.</summary>
    public abstract Status Status { get; }
    /// <summary>Reads the answer, throwing when none is available.</summary>
    public abstract TAnswer Answer { get; }
    /// <summary>The aggregate probability, when present.</summary>
    public abstract double? Probability { get; }
    /// <summary>The function-specific immutable detail.</summary>
    public abstract Detail? Detail { get; }
    /// <summary>Provides IsAnswered.</summary>
    public bool IsAnswered => Status==Status.Answered;
    /// <summary>Provides IsUnsure.</summary>
    public bool IsUnsure => Status==Status.Unsure;
    /// <summary>Provides IsFailed.</summary>
    public bool IsFailed => Status==Status.Failed;
    object? IRowValue.OwnAnswer => IsAnswered ? Answer : null;
    object?[] IRowValue.OwnValues => [Function,Status,IsAnswered ? Answer : null,Probability,(this as IFailed)?.Error,Position,Id,Source,Detail];
    /// <summary>Provides ThrowIfFailed.</summary>
    public Row<TAnswer,TRecord> ThrowIfFailed()
    {
        if (this is IFailed failed) throw new FailedException([failed]);
        if (IsAnswered && Answer is IAnnotations form) form.ThrowIfFailed();
        return this;
    }
    /// <summary>Provides Equals.</summary>
    public virtual bool Equals(Row<TAnswer,TRecord>? other) => other is not null && GetType()==other.GetType() &&
        ValueEquality.Equal(((IRowValue)this).OwnValues,((IRowValue)other).OwnValues);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(((IRowValue)this).OwnValues);
    /// <inheritdoc/>
    public sealed override string ToString() => ResultText.Print(this);
}
/// <summary>A result with an answer.</summary>
[JsonConverter(typeof(RowConverters))]
public sealed record Answered<TAnswer,TRecord> : Row<TAnswer,TRecord>
{
    internal Answered(Function function,ulong position,RecordId? id,Source source,TRecord item,TAnswer answer,double? probability,Detail? detail)
        : base(function,position,id,source,item) { Answer=answer; Probability=probability; Detail=detail; }
    /// <inheritdoc/>
    private protected override void Closed(){}
    /// <inheritdoc/>
    public override Status Status => Status.Answered;
    /// <inheritdoc/>
    public override TAnswer Answer { get; }
    /// <inheritdoc/>
    public override double? Probability { get; }
    /// <inheritdoc/>
    public override Detail? Detail { get; }
    /// <inheritdoc/>
    public bool Equals(Answered<TAnswer,TRecord>? other) => base.Equals(other);
    /// <inheritdoc/>
    public override int GetHashCode() => base.GetHashCode();
}
/// <summary>A result without a sufficiently likely answer.</summary>
[JsonConverter(typeof(RowConverters))]
public sealed record Unsure<TAnswer,TRecord> : Row<TAnswer,TRecord>
{
    internal Unsure(Function function,ulong position,RecordId? id,Source source,TRecord item,double probability,Detail? detail)
        : base(function,position,id,source,item) { Probability=probability; Detail=detail; }
    /// <inheritdoc/>
    private protected override void Closed(){}
    /// <inheritdoc/>
    public override Status Status => Status.Unsure;
    /// <inheritdoc/>
    public override TAnswer Answer => throw new UnsureException(this);
    /// <inheritdoc/>
    public override double? Probability { get; }
    /// <inheritdoc/>
    public override Detail? Detail { get; }
    /// <inheritdoc/>
    public bool Equals(Unsure<TAnswer,TRecord>? other) => base.Equals(other);
    /// <inheritdoc/>
    public override int GetHashCode() => base.GetHashCode();
}
/// <summary>A record that could not be judged.</summary>
[JsonConverter(typeof(RowConverters))]
public sealed record Failed<TAnswer,TRecord> : Row<TAnswer,TRecord>, IFailed
{
    internal Failed(Function function,ulong position,RecordId? id,Source source,TRecord item,RowError error)
        : base(function,position,id,source,item) => Error=error;
    /// <inheritdoc/>
    private protected override void Closed(){}
    /// <inheritdoc/>
    public override Status Status => Status.Failed;
    /// <inheritdoc/>
    public override TAnswer Answer => throw new FailedException([this]);
    /// <inheritdoc/>
    public override double? Probability => null;
    /// <inheritdoc/>
    public override Detail? Detail => null;
    /// <summary>The stable record error.</summary>
    public RowError Error { get; }
    /// <inheritdoc/>
    public bool Equals(Failed<TAnswer,TRecord>? other) => base.Equals(other);
    /// <inheritdoc/>
    public override int GetHashCode() => base.GetHashCode();
}
/// <summary>Decide results support the language's conditional operations.</summary>
public static class DecideOperators
{
    extension<TRecord>(Row<bool,TRecord>)
    {
        /// <summary>Provides true.</summary>
        public static bool operator true(Row<bool,TRecord> row) => row.Answer;
        /// <summary>Provides false.</summary>
        public static bool operator false(Row<bool,TRecord> row) => !row.Answer;
        /// <summary>Reads the answer, throwing when none is available.</summary>
        public static bool operator !(Row<bool,TRecord> row) => !row.Answer;
    }
}
