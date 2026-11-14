using System.Diagnostics.CodeAnalysis;
using System.Text.Json.Serialization.Metadata;
namespace ThinkThen;
public sealed partial class ThinkThenClient
{
    /// <summary>Binds a named form to this client.</summary>
    public ToolboxView Toolbox(AnnotationQuestions questions)=>new(this,Admission.Required(questions));
}
/// <summary>Runs an annotation form through a client.</summary>
public sealed class ToolboxView
{
    private readonly ThinkThenClient client;
    private readonly AnnotationQuestions questions;
    internal ToolboxView(ThinkThenClient client, AnnotationQuestions questions) { this.client = client; this.questions = questions; }
    /// <summary>Answers the named questions for one record and retains each child outcome.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<Annotations<TRecord>, TRecord>> AnnotateAsync<TRecord>(TRecord record, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        client.AnnotateAsync(questions, record, options, name, cancellationToken);
    /// <summary>Answers the named questions for one record and retains each child outcome. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<Annotations<TRecord>, TRecord>> AnnotateWithMetadataAsync<TRecord>(TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return client.AnnotateWithMetadataAsync(questions, record, recordTypeInfo, options, name, cancellationToken);
    }
    /// <summary>Returns annotate outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<Annotations<TRecord>, TRecord>> AnnotateStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        client.AnnotateStreamAsync(questions, records, options, name, cancellationToken);
    /// <summary>Returns annotate outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<Annotations<TRecord>, TRecord>> AnnotateStreamWithMetadataAsync<TRecord>(IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return client.AnnotateStreamWithMetadataAsync(questions, records, recordTypeInfo, options, name, cancellationToken);
    }
}
