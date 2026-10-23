using System.Runtime.CompilerServices;
using System.Text.Json;

namespace ThinkThen;

public sealed partial class ThinkThenClient
{
    public Task<ResultRow<bool?>> DecideAsync(Question question, Record record, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Single<bool?>("decide", question.Value, record, options, null, cancellationToken);
    public Task<ResultRow<string>> ChooseAsync(Question question, Record record, IReadOnlyList<string>? options = null, CallOptions? settings = null, CancellationToken cancellationToken = default) =>
        Single<string>("choose", question.Value, record, settings, options?.ToArray(), cancellationToken);
    public Task<ResultRow<IReadOnlyList<string>>> TagAsync(Question question, Record record, IReadOnlyList<string>? labels = null, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Single<IReadOnlyList<string>>("tag", question.Value, record, options, labels?.ToArray(), cancellationToken);
    public Task<ResultRow<double?>> ScoreAsync(Question question, Record record, IReadOnlyList<string>? levels = null, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Single<double?>("score", question.Value, record, options, levels?.ToArray(), cancellationToken);
    public Task<ResultRow<TextSpan>> FindAsync(Question question, Record record, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Single<TextSpan>("find", question.Value, record, options, null, cancellationToken);
    public Task<ResultRow<JsonElement?>> AnnotateAsync(JsonElement toolbox, Record record, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Single<JsonElement?>("annotate", new { type = "inline", value = toolbox.Clone() }, record, options, null, cancellationToken);
    public Task<ResultRow<IReadOnlyList<RecognizedName>>> RecognizeAsync(Question question, Record record, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Single<IReadOnlyList<RecognizedName>>("recognize", question.Value, record, options, null, cancellationToken);
    public Task<ResultRow<IReadOnlyList<RelationEdge>>> RelateAsync(Question question, Record record, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Single<IReadOnlyList<RelationEdge>>("relate", question.Value, record, options, null, cancellationToken);

    public IAsyncEnumerable<ResultRow<bool?>> DecideStreamAsync(Question question, IAsyncEnumerable<Record> records, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Stream<bool?>("decide", question.Value, records, options, null, cancellationToken);
    public IAsyncEnumerable<ResultRow<string>> ChooseStreamAsync(Question question, IAsyncEnumerable<Record> records, IReadOnlyList<string>? options = null, CallOptions? settings = null, CancellationToken cancellationToken = default) =>
        Stream<string>("choose", question.Value, records, settings, options?.ToArray(), cancellationToken);
    public IAsyncEnumerable<ResultRow<IReadOnlyList<string>>> TagStreamAsync(Question question, IAsyncEnumerable<Record> records, IReadOnlyList<string>? labels = null, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Stream<IReadOnlyList<string>>("tag", question.Value, records, options, labels?.ToArray(), cancellationToken);
    public IAsyncEnumerable<ResultRow<double?>> ScoreStreamAsync(Question question, IAsyncEnumerable<Record> records, IReadOnlyList<string>? levels = null, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Stream<double?>("score", question.Value, records, options, levels?.ToArray(), cancellationToken);
    public IAsyncEnumerable<ResultRow<TextSpan>> FindStreamAsync(Question question, IAsyncEnumerable<Record> records, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Stream<TextSpan>("find", question.Value, records, options, null, cancellationToken);
    public IAsyncEnumerable<ResultRow<JsonElement?>> AnnotateStreamAsync(JsonElement toolbox, IAsyncEnumerable<Record> records, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Stream<JsonElement?>("annotate", new { type = "inline", value = toolbox.Clone() }, records, options, null, cancellationToken);
    public IAsyncEnumerable<ResultRow<IReadOnlyList<RecognizedName>>> RecognizeStreamAsync(Question question, IAsyncEnumerable<Record> records, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Stream<IReadOnlyList<RecognizedName>>("recognize", question.Value, records, options, null, cancellationToken);
    public IAsyncEnumerable<ResultRow<IReadOnlyList<RelationEdge>>> RelateStreamAsync(Question question, IAsyncEnumerable<Record> records, CallOptions? options = null, CancellationToken cancellationToken = default) =>
        Stream<IReadOnlyList<RelationEdge>>("relate", question.Value, records, options, null, cancellationToken);
    public IAsyncEnumerable<ResultRow<long?>> RankAsync(Question question, IAsyncEnumerable<Record> records, RankOptions? options = null, CancellationToken cancellationToken = default) =>
        Stream<long?>("rank", question.Value, records, options, null, cancellationToken);
    public async IAsyncEnumerable<Record> FilterAsync(Question question, IAsyncEnumerable<Record> records, CallOptions? options = null,
        [EnumeratorCancellation] CancellationToken cancellationToken = default)
    {
        await foreach (var item in Run<bool?>("filter", question.Value, records, options, null, true, cancellationToken).ConfigureAwait(false))
            if (item.Kept) yield return item.Record!;
    }
}
