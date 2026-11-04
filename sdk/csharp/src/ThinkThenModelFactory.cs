namespace ThinkThen;

/// <summary>Create answer rows for tests without opening a client or loading the native engine.</summary>
public static class ThinkThenModelFactory
{
    /// <summary>Create an answered row with the supplied answer and optional metadata.</summary>
    public static Answered<TAnswer, TDetail> Answered<TAnswer, TDetail>(Function function, TAnswer answer,
        Source source, ulong position = 1, RecordId? id = null, double? probability = null, TDetail? detail = default)
    {
        ArgumentNullException.ThrowIfNull(source);
        return new(function, position, id, source, answer, probability, detail);
    }
    /// <summary>Create a not-sure row with its probability and optional detail.</summary>
    public static Unsure<TAnswer, TDetail> Unsure<TAnswer, TDetail>(Function function, double probability,
        Source source, ulong position = 1, RecordId? id = null, TDetail? detail = default)
    {
        ArgumentNullException.ThrowIfNull(source);
        return new(function, position, id, source, probability, detail);
    }
    /// <summary>Create a failed row whose answer access throws the supplied error.</summary>
    public static Failed<TAnswer, TDetail> Failed<TAnswer, TDetail>(Function function, RowError error,
        Source source, ulong position = 1, RecordId? id = null)
    {
        ArgumentNullException.ThrowIfNull(source);
        ArgumentNullException.ThrowIfNull(error);
        return new(function, position, id, source, error);
    }
}
