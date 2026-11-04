using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using System.Text.Json.Serialization.Metadata;

namespace ThinkThen;

public sealed partial class ThinkThenClient
{
    /// <summary>Yields kept input records, then throws with every failed row attached if any record failed. Not-sure records are omitted.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<TRecord> FilterAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records,
        FilterOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Keep(FilterStreamAsync(question, records, options, name, cancellationToken), cancellationToken);
    /// <summary>Yields kept input records for an admitted question, then throws with all failed rows attached if any record failed.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<TRecord> FilterAsync<TRecord>(FilterQuestion question, IAsyncEnumerable<TRecord> records,
        string? name = null, CancellationToken cancellationToken = default) =>
        Keep(FilterStreamAsync(question, records, name, cancellationToken), cancellationToken);
    /// <summary>Yields kept input records using explicit JSON metadata, then throws with every failed row attached if any record failed.</summary>
    public IAsyncEnumerable<TRecord> FilterWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records,
        JsonTypeInfo<TRecord> recordTypeInfo, FilterOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Keep(FilterStreamWithMetadataAsync(question, records, recordTypeInfo, options, name, cancellationToken), cancellationToken);
    /// <summary>Yields kept input records for an admitted question using explicit JSON metadata; failures are attached to the final exception.</summary>
    public IAsyncEnumerable<TRecord> FilterWithMetadataAsync<TRecord>(FilterQuestion question, IAsyncEnumerable<TRecord> records,
        JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Keep(FilterStreamWithMetadataAsync(question, records, recordTypeInfo, name, cancellationToken), cancellationToken);
    private static async IAsyncEnumerable<T> Keep<T>(IAsyncEnumerable<FilterItem<T>> items,
        [EnumeratorCancellation] CancellationToken cancellationToken)
    {
        var failures = new List<Row<bool, NoDetail>>();
        await foreach (var item in items.WithCancellation(cancellationToken).ConfigureAwait(false))
        {
            if (item.Kept) yield return item.Item;
            else if (item.Row.IsFailed) failures.Add(item.Row);
        }
        if (failures.Count > 0)
        {
            var error = ((Failed<bool, NoDetail>)failures[0]).Error;
            throw new ThinkThenException(error, error.Cause, new(failures));
        }
    }
}
