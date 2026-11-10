namespace ThinkThen;

/// <summary>Selects how the shared engine uses local responses.</summary>
public enum ResponseStoreMode
{
    /// <summary>Reuse eligible answers and store misses.</summary>
    Cache,
    /// <summary>Record immutable live exchanges.</summary>
    Record,
    /// <summary>Read recordings without credentials or network.</summary>
    Replay
}

/// <summary>Opt-in local response storage; null limits use shared engine defaults.</summary>
/// <param name="Path">Private local storage directory.</param>
/// <param name="Mode">Cache, record or strict replay.</param>
/// <param name="Namespace">Nonsecret caller-owned namespace.</param>
/// <param name="ModelGeneration">Explicit nonsecret model-generation label.</param>
public sealed record ResponseStoreOptions(string Path, ResponseStoreMode Mode, string Namespace, string ModelGeneration)
{
    /// <summary>Logical retained byte bound.</summary>
    public ulong? MaxBytes { get; init; }
    /// <summary>Distinct eligible ask bound.</summary>
    public ulong? MaxEntries { get; init; }
    /// <summary>Cache age in seconds; recordings do not expire.</summary>
    public ulong? MaxAge { get; init; }
    /// <summary>SQLite busy timeout in milliseconds.</summary>
    public ulong? BusyTimeoutMs { get; init; }
}
