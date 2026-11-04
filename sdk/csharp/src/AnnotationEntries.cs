using System.Text.Json;
using System.Text.Json.Serialization;
namespace ThinkThen;
/// <summary>The outcome of one annotation question.</summary>
public abstract record EntryOutcome<TAnswer,TDetail> : IOutcome
{
    internal EntryOutcome() { }
    private protected abstract void Closed();
    /// <summary>Gets the answer; uncertain answers throw UnsureException and failed answers throw ThinkThenException.</summary>
    public abstract TAnswer Answer { get; }
    /// <summary>Gets the outcome status.</summary>
    public abstract RowStatus Status { get; }
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public abstract double? Probability { get; }
    internal abstract object?[] Values { get; }
    object?[] IOutcome.Values => Values;
    /// <summary>Returns true and the answer only for an answered outcome; uncertainty and failure return false.</summary>
    public bool TryGetAnswer([System.Diagnostics.CodeAnalysis.MaybeNullWhen(false)] out TAnswer answer)
    {
        if (Status == RowStatus.Answered) { answer = Answer; return true; }
        answer = default; return false;
    }
}
/// <summary>An answered annotation question.</summary>
public sealed record EntryAnswered<TAnswer,TDetail> : EntryOutcome<TAnswer,TDetail>
{
    /// <summary>An answered annotation question.</summary>
    public EntryAnswered(TAnswer answer, double? probability, TDetail? detail)
    { Answer = answer; Probability = probability; Detail = detail; }
    /// <inheritdoc/>
    public bool Equals(EntryAnswered<TAnswer,TDetail>? other) => ValueEquality.Equal(this, other);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(this);
    private protected override void Closed() { }
    /// <summary>Gets the answer; uncertain answers throw UnsureException and failed answers throw ThinkThenException.</summary>
    public override TAnswer Answer { get; }
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public override double? Probability { get; }
    /// <summary>Gets the function-specific probability detail, if present.</summary>
    public TDetail? Detail { get; }
    /// <summary>Gets the outcome status.</summary>
    public override RowStatus Status => RowStatus.Answered;
    internal override object?[] Values => [Status, Answer, Probability, Detail];
}
/// <summary>An uncertain annotation question.</summary>
public sealed record EntryUnsure<TAnswer,TDetail> : EntryOutcome<TAnswer,TDetail>
{
    /// <summary>An uncertain annotation question.</summary>
    public EntryUnsure(double probability, TDetail? detail) { Probability = probability; Detail = detail; }
    /// <inheritdoc/>
    public bool Equals(EntryUnsure<TAnswer,TDetail>? other) => ValueEquality.Equal(this, other);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(this);
    private protected override void Closed() { }
    /// <inheritdoc/>
    public override TAnswer Answer => throw new UnsureException(Probability!.Value);
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public override double? Probability { get; }
    /// <summary>Gets the function-specific probability detail, if present.</summary>
    public TDetail? Detail { get; }
    /// <summary>Gets the outcome status.</summary>
    public override RowStatus Status => RowStatus.Unsure;
    internal override object?[] Values => [Status, Probability, Detail];
}
/// <summary>A failed annotation question.</summary>
public sealed record EntryFailed<TAnswer,TDetail>(RowError Error) : EntryOutcome<TAnswer,TDetail>
{
    /// <inheritdoc/>
    public bool Equals(EntryFailed<TAnswer,TDetail>? other) => ValueEquality.Equal(this, other);
    /// <inheritdoc/>
    public override int GetHashCode() => ValueEquality.Hash(this);
    private protected override void Closed() { }
    /// <inheritdoc/>
    public override TAnswer Answer => throw new ThinkThenException(Error, Error.Cause);
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public override double? Probability => null;
    /// <summary>Gets the outcome status.</summary>
    public override RowStatus Status => RowStatus.Failed;
    internal override object?[] Values => [Status, Error];
}
/// <summary>One annotation answer and its outcome metadata.</summary>
public abstract record AnnotationEntry
{
    internal AnnotationEntry() { }
    private protected abstract void Closed();
    internal abstract IOutcome Value { get; }
    internal abstract object? AnswerValue { get; }
    /// <summary>Gets the answer; uncertain answers throw UnsureException and failed answers throw ThinkThenException.</summary>
    public object? Answer => AnswerValue;
    /// <summary>Gets the question function.</summary>
    public abstract Function Function { get; }
    /// <summary>Gets the outcome status.</summary>
    public abstract RowStatus Status { get; }
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public abstract double? Probability { get; }
    /// <summary>Reports whether this outcome contains an answer.</summary>
    public bool IsAnswered => Status == RowStatus.Answered;
    /// <summary>Reports whether this outcome is uncertain.</summary>
    public bool IsUnsure => Status == RowStatus.Unsure;
    /// <summary>Reports whether this outcome failed.</summary>
    public bool IsFailed => Status == RowStatus.Failed;
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static implicit operator bool(AnnotationEntry entry) => (bool)entry.Answer!;
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static implicit operator string(AnnotationEntry entry) => (string)entry.Answer!;
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static implicit operator double(AnnotationEntry entry) => (double)entry.Answer!;
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static bool operator ==(AnnotationEntry? entry, object? answer) =>
        entry is not null && entry.IsAnswered && ValueEquality.Equal(entry.Answer, answer);
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static bool operator !=(AnnotationEntry? entry, object? answer) => !(entry == answer);
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static bool operator ==(object? answer, AnnotationEntry? entry) => entry == answer;
    /// <summary>Reads an answered value, or compares it without throwing for uncertainty or failure.</summary>
    public static bool operator !=(object? answer, AnnotationEntry? entry) => !(entry == answer);
    /// <inheritdoc/>
    public sealed override string ToString() => IsAnswered ? ValueEquality.Display(Answer) : IsUnsure ? "unsure" : "failed";
}
/// <summary>One annotation answer and its outcome metadata.</summary>
public abstract record AnnotationEntry<TAnswer,TDetail> : AnnotationEntry
{
    internal AnnotationEntry(EntryOutcome<TAnswer,TDetail> outcome)
    { ArgumentNullException.ThrowIfNull(outcome); Outcome = outcome; }
    /// <summary>Gets the annotation question's complete outcome.</summary>
    public EntryOutcome<TAnswer,TDetail> Outcome { get; }
    internal override IOutcome Value => Outcome;
    internal override object? AnswerValue => Outcome.Answer;
    /// <summary>Gets the answer; uncertain answers throw UnsureException and failed answers throw ThinkThenException.</summary>
    public new TAnswer Answer => Outcome.Answer;
    /// <summary>Gets the reported probability, or null when the outcome has no aggregate probability.</summary>
    public override double? Probability => Outcome.Probability;
    /// <summary>Gets the outcome status.</summary>
    public override RowStatus Status => Outcome.Status;
    /// <summary>Returns true and the answer only for an answered outcome; uncertainty and failure return false.</summary>
    public bool TryGetAnswer([System.Diagnostics.CodeAnalysis.MaybeNullWhen(false)] out TAnswer answer) => Outcome.TryGetAnswer(out answer);
}
/// <summary>An annotation decide answer with outcome metadata.</summary>
public sealed record DecideEntry : AnnotationEntry<bool,NoDetail>
{
    /// <summary>Wraps an annotation outcome with its function-specific answer type.</summary>
    public DecideEntry(EntryOutcome<bool,NoDetail> outcome) : base(outcome) { }
    private protected override void Closed() { }
    /// <summary>Gets the question function.</summary>
    public override Function Function => Function.Decide;
}
/// <summary>An annotation choose answer with outcome metadata.</summary>
public sealed record ChooseEntry : AnnotationEntry<string,LabelsDetail>
{
    /// <summary>Wraps an annotation outcome with its function-specific answer type.</summary>
    public ChooseEntry(EntryOutcome<string,LabelsDetail> outcome) : base(outcome) { }
    private protected override void Closed() { }
    /// <summary>Gets the question function.</summary>
    public override Function Function => Function.Choose;
}
/// <summary>An annotation tag answer with outcome metadata.</summary>
public sealed record TagEntry : AnnotationEntry<ValueList<string>,LabelsDetail>
{
    /// <summary>Wraps an annotation outcome with its function-specific answer type.</summary>
    public TagEntry(EntryOutcome<ValueList<string>,LabelsDetail> outcome) : base(outcome) { }
    private protected override void Closed() { }
    /// <summary>Gets the question function.</summary>
    public override Function Function => Function.Tag;
}
/// <summary>An annotation score answer with outcome metadata.</summary>
public sealed record ScoreEntry : AnnotationEntry<double,LevelsDetail>
{
    /// <summary>Wraps an annotation outcome with its function-specific answer type.</summary>
    public ScoreEntry(EntryOutcome<double,LevelsDetail> outcome) : base(outcome) { }
    private protected override void Closed() { }
    /// <summary>Gets the question function.</summary>
    public override Function Function => Function.Score;
}
/// <summary>An annotation filter answer with outcome metadata.</summary>
public sealed record FilterEntry : AnnotationEntry<bool,NoDetail>
{
    /// <summary>Wraps an annotation outcome with its function-specific answer type.</summary>
    public FilterEntry(EntryOutcome<bool,NoDetail> outcome) : base(outcome) { }
    private protected override void Closed() { }
    /// <summary>Gets the question function.</summary>
    public override Function Function => Function.Filter;
}
/// <summary>An annotation find answer with outcome metadata.</summary>
public sealed record FindEntry : AnnotationEntry<TextSpan?,NoDetail>
{
    /// <summary>Wraps an annotation outcome with its function-specific answer type.</summary>
    public FindEntry(EntryOutcome<TextSpan?,NoDetail> outcome) : base(outcome) { }
    private protected override void Closed() { }
    /// <summary>Gets the question function.</summary>
    public override Function Function => Function.Find;
}
/// <summary>An annotation recognize answer with outcome metadata.</summary>
public sealed record RecognizeEntry : AnnotationEntry<ValueList<Entity>,RecognizeDetail>
{
    /// <summary>Wraps an annotation outcome with its function-specific answer type.</summary>
    public RecognizeEntry(EntryOutcome<ValueList<Entity>,RecognizeDetail> outcome) : base(outcome) { }
    private protected override void Closed() { }
    /// <summary>Gets the question function.</summary>
    public override Function Function => Function.Recognize;
}
/// <summary>An annotation relate answer with outcome metadata.</summary>
public sealed record RelateEntry : AnnotationEntry<ValueList<Relation>,RelateDetail>
{
    /// <summary>Wraps an annotation outcome with its function-specific answer type.</summary>
    public RelateEntry(EntryOutcome<ValueList<Relation>,RelateDetail> outcome) : base(outcome) { }
    private protected override void Closed() { }
    /// <summary>Gets the question function.</summary>
    public override Function Function => Function.Relate;
}
