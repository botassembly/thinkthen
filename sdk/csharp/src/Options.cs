namespace ThinkThen;
/// <summary>Optional settings for decide questions.</summary>
public sealed record DecideOptions
{
    /// <summary>Describes what a yes answer means to the backend.</summary>
    public string? YesMeans { get; init; }
    /// <summary>Describes what a no answer means to the backend.</summary>
    public string? NoMeans { get; init; }
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
    /// <summary>Sets the question's probability cutoff or band; null uses the engine default.</summary>
    public Threshold? Threshold { get; init; }
}
/// <summary>Optional settings for choose questions.</summary>
public sealed record ChooseOptions
{
    /// <summary>Maps each choice, label or level to its description; keys must belong to the question's domain.</summary>
    public IReadOnlyDictionary<string, string>? Descriptions { get; init; }
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
    /// <summary>Sets the question's probability cutoff or band; null uses the engine default.</summary>
    public Threshold? Threshold { get; init; }
}
/// <summary>Optional settings for tag questions.</summary>
public sealed record TagOptions
{
    /// <summary>Maps each choice, label or level to its description; keys must belong to the question's domain.</summary>
    public IReadOnlyDictionary<string, string>? Descriptions { get; init; }
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
    /// <summary>Sets the question's probability cutoff or band; null uses the engine default.</summary>
    public Cut? Threshold { get; init; }
}
/// <summary>Optional settings for score questions.</summary>
public sealed record ScoreOptions
{
    /// <summary>Maps each choice, label or level to its description; keys must belong to the question's domain.</summary>
    public IReadOnlyDictionary<string, string>? Descriptions { get; init; }
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
}
/// <summary>Optional settings for filter questions.</summary>
public sealed record FilterOptions
{
    /// <summary>Describes what a yes answer means to the backend.</summary>
    public string? YesMeans { get; init; }
    /// <summary>Describes what a no answer means to the backend.</summary>
    public string? NoMeans { get; init; }
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
    /// <summary>Sets the question's probability cutoff or band; null uses the engine default.</summary>
    public Threshold? Threshold { get; init; }
}
/// <summary>Optional settings for rank questions.</summary>
public sealed record RankOptions
{
    /// <summary>Maps each choice, label or level to its description; keys must belong to the question's domain.</summary>
    public IReadOnlyDictionary<string, string>? Descriptions { get; init; }
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
    /// <summary>Limits ranked output to the best records; null returns every rank.</summary>
    public int? Top { get; init; }
}
/// <summary>Optional settings for find questions.</summary>
public sealed record FindOptions
{
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
    /// <summary>Sets the question's probability cutoff or band; null uses the engine default.</summary>
    public Cut? Threshold { get; init; }
}
/// <summary>Optional settings for recognize questions.</summary>
public sealed record RecognizeOptions
{
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
    /// <summary>Sets the question's probability cutoff or band; null uses the engine default.</summary>
    public Cut? Threshold { get; init; }
}
/// <summary>Optional settings for relate questions.</summary>
public sealed record RelateOptions
{
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
    /// <summary>Sets the question's probability cutoff or band; null uses the engine default.</summary>
    public Cut? Threshold { get; init; }
}
/// <summary>Optional settings for annotate questions.</summary>
public sealed record AnnotateOptions
{
    /// <summary>Selects field using a JSON Pointer; null uses the whole record.</summary>
    public Pointer? Field { get; init; }
}
