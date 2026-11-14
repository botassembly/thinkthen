namespace ThinkThen;

/// <summary>Create answer rows for tests without opening a client or loading the native engine.</summary>
public static class ThinkThenModelFactory
{
    /// <summary>Create an answered row with the supplied answer and optional metadata.</summary>
    public static Answered<TAnswer, TRecord> Answered<TAnswer, TRecord>(Function function, TAnswer answer,
        Source source, ulong position = 1, RecordId? id = null, double? probability = null, Detail? detail = null, TRecord item = default!)
    {
        ArgumentNullException.ThrowIfNull(source);
        return new(function, position, id, source, item, answer, probability, detail);
    }
    /// <summary>Create a not-sure row with its probability and optional detail.</summary>
    public static Unsure<TAnswer, TRecord> Unsure<TAnswer, TRecord>(Function function, double probability,
        Source source, ulong position = 1, RecordId? id = null, Detail? detail = null, TRecord item = default!)
    {
        ArgumentNullException.ThrowIfNull(source);
        return new(function, position, id, source, item, probability, detail);
    }
    /// <summary>Create a failed row whose answer access throws the supplied error.</summary>
    public static Failed<TAnswer, TRecord> Failed<TAnswer, TRecord>(Function function, RowError error,
        Source source, ulong position = 1, RecordId? id = null, TRecord item = default!)
    {
        ArgumentNullException.ThrowIfNull(source);
        ArgumentNullException.ThrowIfNull(error);
        return new(function, position, id, source, item, error);
    }
}
