using System.Collections;
using System.Collections.Immutable;
using System.Diagnostics.CodeAnalysis;

namespace ThinkThen;

/// <summary>An immutable answer map whose equality compares keys and values regardless of insertion order.</summary>
/// <typeparam name="TKey">The key type.</typeparam>
/// <typeparam name="TValue">The value type.</typeparam>
public sealed class ValueMap<TKey, TValue> : IReadOnlyDictionary<TKey, TValue>, IEquatable<ValueMap<TKey, TValue>> where TKey : notnull
{
    private readonly ImmutableDictionary<TKey, TValue> items;
    /// <summary>Copies key-value pairs into an immutable map; duplicate keys are refused.</summary>
    public ValueMap(IEnumerable<KeyValuePair<TKey, TValue>> items)
    { ArgumentNullException.ThrowIfNull(items); this.items = items.ToImmutableDictionary(); }
    /// <summary>Gets an empty immutable map.</summary>
    public static ValueMap<TKey, TValue> Empty { get; } = new(Array.Empty<KeyValuePair<TKey, TValue>>());
    /// <summary>Gets the number of entries.</summary>
    public int Count => items.Count;
    /// <summary>Gets the map's keys.</summary>
    public IEnumerable<TKey> Keys => items.Keys;
    /// <summary>Gets the map's values.</summary>
    public IEnumerable<TValue> Values => items.Values;
    /// <summary>Gets the value associated with a key.</summary>
    public TValue this[TKey key] => items[key];
    /// <summary>Reports whether the map contains the key.</summary>
    public bool ContainsKey(TKey key) => items.ContainsKey(key);
    /// <summary>Tries to read the value for a key without throwing when the key is absent.</summary>
    public bool TryGetValue(TKey key, [MaybeNullWhen(false)] out TValue value) => items.TryGetValue(key, out value);
    /// <summary>Returns a new immutable map with an additional entry; a different value for an existing key is refused.</summary>
    public ValueMap<TKey, TValue> Add(TKey key, TValue value) => new(items.Add(key, value));
    /// <inheritdoc/>
    public bool Equals(ValueMap<TKey, TValue>? other) => ReferenceEquals(this, other) || other is not null && Count == other.Count &&
        items.All(pair => other.items.TryGetValue(pair.Key, out TValue? value) && ValueEquality.Equal(pair.Value, value));
    /// <inheritdoc/>
    public override bool Equals(object? other) => other is ValueMap<TKey, TValue> values && Equals(values);
    /// <inheritdoc/>
    public override int GetHashCode()
    {
        int hash = 0;
        foreach (var pair in items) hash ^= HashCode.Combine(pair.Key, ValueEquality.Hash(pair.Value));
        return hash;
    }
    /// <summary>Compares two maps by their keys and values.</summary>
    public static bool operator ==(ValueMap<TKey, TValue>? left, ValueMap<TKey, TValue>? right) => Equals(left, right);
    /// <summary>Reports whether two maps differ by key or value.</summary>
    public static bool operator !=(ValueMap<TKey, TValue>? left, ValueMap<TKey, TValue>? right) => !Equals(left, right);
    /// <summary>Enumerates the map's entries without imposing an order.</summary>
    public IEnumerator<KeyValuePair<TKey, TValue>> GetEnumerator() => items.GetEnumerator();
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
}

internal static class ValueCollections
{
    internal static ValueList<T> ToValueList<T>(this IEnumerable<T> values) => new(values);
    internal static ValueMap<TKey, TValue> ToValueMap<T, TKey, TValue>(this IEnumerable<T> values,
        Func<T, TKey> key, Func<T, TValue> value) where TKey : notnull => new(values.Select(item => KeyValuePair.Create(key(item), value(item))));
}
