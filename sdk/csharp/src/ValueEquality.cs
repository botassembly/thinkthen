using System.Collections;

namespace ThinkThen;

// Compare the public value graph without relying on immutable collection storage
// identities, cached wire JSON, reflection or exception stack traces.
internal static class ValueEquality
{
    internal static bool Equal(object? left, object? right)
    {
        if (ReferenceEquals(left, right)) return true;
        if (left is null || right is null || left.GetType() != right.GetType()) return false;
        var first = Parts(left);
        var second = Parts(right);
        if (first is null || second is null) return left.Equals(right);
        return first.Length == second.Length && first.Zip(second).All(pair => Equal(pair.First, pair.Second));
    }
    internal static int Hash(object? value)
    {
        if (value is null) return 0;
        var parts = Parts(value);
        if (parts is null) return value.GetHashCode();
        var hash = new HashCode();
        foreach (object? part in parts) hash.Add(Hash(part));
        return hash.ToHashCode();
    }
    private static object?[]? Parts(object value) => value switch
    {
        RowError error => [error.Code, error.Message, error.HttpStatus],
        LabelsDetail detail => [detail.Labels],
        LevelsDetail detail => [detail.Levels],
        PieceDetail detail => [detail.Span, detail.Tags],
        KindDetail detail => [detail.Text, detail.Start, detail.End, detail.Kinds],
        RecognizeDetail detail => [detail.Pieces, detail.Names],
        RelateDetail detail => [detail.Names, detail.Pairs],
        AnnotationEntry entry => [entry.Function, entry.Value],
        IOutcome outcome => outcome.Values,
        IReadOnlyDictionary<string, double> map => map.OrderBy(pair => pair.Key, StringComparer.Ordinal)
            .SelectMany(pair => new object?[] { pair.Key, pair.Value }).ToArray(),
        IReadOnlyDictionary<BoundaryTag, double> map => map.OrderBy(pair => pair.Key)
            .SelectMany(pair => new object?[] { pair.Key, pair.Value }).ToArray(),
        IReadOnlyDictionary<string, AnnotationEntry> map => map.OrderBy(pair => pair.Key, StringComparer.Ordinal)
            .SelectMany(pair => new object?[] { pair.Key, pair.Value }).ToArray(),
        string => null,
        IEnumerable sequence => sequence.Cast<object?>().ToArray(),
        _ => null
    };
    internal static string Display(object? value) => value switch
    {
        null => "null",
        string text => text,
        IEnumerable sequence => "[" + string.Join(", ", sequence.Cast<object?>().Select(Display)) + "]",
        _ => value.ToString() ?? "null"
    };
}
internal interface IOutcome
{
    object?[] Values { get; }
}
