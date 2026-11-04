using System.Collections;
using System.Collections.Immutable;
using System.Runtime.CompilerServices;

namespace ThinkThen;

/// <summary>An immutable ordered answer collection whose equality compares its elements.</summary>
/// <typeparam name="T">The element type.</typeparam>
[CollectionBuilder(typeof(ValueList), nameof(ValueList.Create))]
public sealed class ValueList<T> : IReadOnlyList<T>, IEquatable<ValueList<T>>
{
    private readonly ImmutableArray<T> items;
    /// <summary>Copies the supplied elements into an immutable ordered collection.</summary>
    public ValueList(IEnumerable<T> items)
    { ArgumentNullException.ThrowIfNull(items); this.items = items.ToImmutableArray(); }
    /// <summary>Gets an empty immutable list.</summary>
    public static ValueList<T> Empty { get; } = new(Array.Empty<T>());
    /// <summary>Gets the number of elements.</summary>
    public int Count => items.Length;
    /// <summary>Gets the element at the zero-based index.</summary>
    public T this[int index] => items[index];
    /// <summary>Returns a new immutable list with the value appended.</summary>
    public ValueList<T> Add(T value) => new(items.Add(value));
    /// <inheritdoc/>
    public bool Equals(ValueList<T>? other) => ReferenceEquals(this, other) || other is not null &&
        Count == other.Count && items.Zip(other.items).All(pair => ValueEquality.Equal(pair.First, pair.Second));
    /// <inheritdoc/>
    public override bool Equals(object? other) => other is ValueList<T> values && Equals(values);
    /// <inheritdoc/>
    public override int GetHashCode()
    {
        var hash = new HashCode();
        foreach (T value in items) hash.Add(ValueEquality.Hash(value));
        return hash.ToHashCode();
    }
    /// <summary>Compares two lists by element value and order.</summary>
    public static bool operator ==(ValueList<T>? left, ValueList<T>? right) => Equals(left, right);
    /// <summary>Reports whether two lists differ by element value or order.</summary>
    public static bool operator !=(ValueList<T>? left, ValueList<T>? right) => !Equals(left, right);
    /// <inheritdoc/>
    public override string ToString() => ValueEquality.Display(this);
    /// <summary>Enumerates the elements in order.</summary>
    public IEnumerator<T> GetEnumerator() => ((IEnumerable<T>)items).GetEnumerator();
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
}

/// <summary>Builds immutable value lists from C# collection expressions.</summary>
public static class ValueList
{
    /// <summary>Copies the elements of a collection expression into a value list.</summary>
    public static ValueList<T> Create<T>(ReadOnlySpan<T> values) => new(values.ToArray());
}
