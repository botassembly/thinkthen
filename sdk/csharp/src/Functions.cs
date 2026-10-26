using System.Collections.Immutable;
using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using System.Text.Json;
using System.Text.Json.Serialization.Metadata;
namespace ThinkThen;
public sealed partial class ThinkThenClient
{
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<bool,NoDetail>> DecideAsync<TRecord>(string question, TRecord record, DecideOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(DecideQuestion.Create(question, options, cancellationToken), record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<bool,NoDetail>> DecideAsync<TRecord>(DecideQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<bool,NoDetail>> DecideStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, DecideOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(DecideQuestion.Create(question, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<bool,NoDetail>> DecideStreamAsync<TRecord>(DecideQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    public Task<Row<bool,NoDetail>> DecideAsync<TRecord>(string question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, DecideOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(DecideQuestion.Create(question, options, cancellationToken), record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public Task<Row<bool,NoDetail>> DecideAsync<TRecord>(DecideQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<bool,NoDetail>> DecideStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, DecideOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(DecideQuestion.Create(question, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<bool,NoDetail>> DecideStreamAsync<TRecord>(DecideQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<string,LabelsDetail>> ChooseAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> choices, ChooseOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(ChooseQuestion.Create(question, choices, options, cancellationToken), record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<string,LabelsDetail>> ChooseAsync<TRecord>(ChooseQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<string,LabelsDetail>> ChooseStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> choices, ChooseOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(ChooseQuestion.Create(question, choices, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<string,LabelsDetail>> ChooseStreamAsync<TRecord>(ChooseQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    public Task<Row<string,LabelsDetail>> ChooseAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(ChooseQuestion.Create(question, choices, options, cancellationToken), record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public Task<Row<string,LabelsDetail>> ChooseAsync<TRecord>(ChooseQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<string,LabelsDetail>> ChooseStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(ChooseQuestion.Create(question, choices, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<string,LabelsDetail>> ChooseStreamAsync<TRecord>(ChooseQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ImmutableArray<string>,LabelsDetail>> TagAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> labels, TagOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(TagQuestion.Create(question, labels, options, cancellationToken), record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ImmutableArray<string>,LabelsDetail>> TagAsync<TRecord>(TagQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ImmutableArray<string>,LabelsDetail>> TagStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> labels, TagOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(TagQuestion.Create(question, labels, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ImmutableArray<string>,LabelsDetail>> TagStreamAsync<TRecord>(TagQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    public Task<Row<ImmutableArray<string>,LabelsDetail>> TagAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(TagQuestion.Create(question, labels, options, cancellationToken), record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public Task<Row<ImmutableArray<string>,LabelsDetail>> TagAsync<TRecord>(TagQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<ImmutableArray<string>,LabelsDetail>> TagStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(TagQuestion.Create(question, labels, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<ImmutableArray<string>,LabelsDetail>> TagStreamAsync<TRecord>(TagQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<double,LevelsDetail>> ScoreAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> levels, ScoreOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(ScoreQuestion.Create(question, levels, options, cancellationToken), record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<double,LevelsDetail>> ScoreAsync<TRecord>(ScoreQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,LevelsDetail>> ScoreStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, ScoreOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(ScoreQuestion.Create(question, levels, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,LevelsDetail>> ScoreStreamAsync<TRecord>(ScoreQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    public Task<Row<double,LevelsDetail>> ScoreAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(ScoreQuestion.Create(question, levels, options, cancellationToken), record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public Task<Row<double,LevelsDetail>> ScoreAsync<TRecord>(ScoreQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<double,LevelsDetail>> ScoreStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(ScoreQuestion.Create(question, levels, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<double,LevelsDetail>> ScoreStreamAsync<TRecord>(ScoreQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<FilterItem<TRecord>> FilterAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, FilterOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        FilterItems(FilterQuestion.Create(question, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<FilterItem<TRecord>> FilterAsync<TRecord>(FilterQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        FilterItems(question, records, Serialize<TRecord>, name, cancellationToken);
    public IAsyncEnumerable<FilterItem<TRecord>> FilterAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, FilterOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        FilterItems(FilterQuestion.Create(question, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<FilterItem<TRecord>> FilterAsync<TRecord>(FilterQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        FilterItems(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<RankedItem<TRecord>> RankAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, RankOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        RankItems(RankQuestion.Create(question, levels, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<RankedItem<TRecord>> RankAsync<TRecord>(RankQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        RankItems(question, records, Serialize<TRecord>, name, cancellationToken);
    public IAsyncEnumerable<RankedItem<TRecord>> RankAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, RankOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        RankItems(RankQuestion.Create(question, levels, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<RankedItem<TRecord>> RankAsync<TRecord>(RankQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        RankItems(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<TextSpan?,NoDetail>> FindAsync<TRecord>(string question, TRecord record, FindOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(FindQuestion.Create(question, options, cancellationToken), record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<TextSpan?,NoDetail>> FindAsync<TRecord>(FindQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TextSpan?,NoDetail>> FindStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, FindOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(FindQuestion.Create(question, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TextSpan?,NoDetail>> FindStreamAsync<TRecord>(FindQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    public Task<Row<TextSpan?,NoDetail>> FindAsync<TRecord>(string question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, FindOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(FindQuestion.Create(question, options, cancellationToken), record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public Task<Row<TextSpan?,NoDetail>> FindAsync<TRecord>(FindQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<TextSpan?,NoDetail>> FindStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, FindOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(FindQuestion.Create(question, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<TextSpan?,NoDetail>> FindStreamAsync<TRecord>(FindQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ImmutableArray<Entity>,RecognizeDetail>> RecognizeAsync<TRecord>(TRecord record, IReadOnlyList<string> kinds, RecognizeOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(RecognizeQuestion.Create(kinds, options, cancellationToken), record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ImmutableArray<Entity>,RecognizeDetail>> RecognizeAsync<TRecord>(RecognizeQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ImmutableArray<Entity>,RecognizeDetail>> RecognizeStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<string> kinds, RecognizeOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(RecognizeQuestion.Create(kinds, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ImmutableArray<Entity>,RecognizeDetail>> RecognizeStreamAsync<TRecord>(RecognizeQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    public Task<Row<ImmutableArray<Entity>,RecognizeDetail>> RecognizeAsync<TRecord>(TRecord record, IReadOnlyList<string> kinds, JsonTypeInfo<TRecord> recordTypeInfo, RecognizeOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(RecognizeQuestion.Create(kinds, options, cancellationToken), record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public Task<Row<ImmutableArray<Entity>,RecognizeDetail>> RecognizeAsync<TRecord>(RecognizeQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<ImmutableArray<Entity>,RecognizeDetail>> RecognizeStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<string> kinds, JsonTypeInfo<TRecord> recordTypeInfo, RecognizeOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(RecognizeQuestion.Create(kinds, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<ImmutableArray<Entity>,RecognizeDetail>> RecognizeStreamAsync<TRecord>(RecognizeQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ImmutableArray<Relation>,RelateDetail>> RelateAsync<TRecord>(TRecord record, IReadOnlyList<RelationRule> rules, RelateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(RelateQuestion.Create(rules, options, cancellationToken), record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ImmutableArray<Relation>,RelateDetail>> RelateAsync<TRecord>(RelateQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ImmutableArray<Relation>,RelateDetail>> RelateStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<RelationRule> rules, RelateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(RelateQuestion.Create(rules, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ImmutableArray<Relation>,RelateDetail>> RelateStreamAsync<TRecord>(RelateQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    public Task<Row<ImmutableArray<Relation>,RelateDetail>> RelateAsync<TRecord>(TRecord record, IReadOnlyList<RelationRule> rules, JsonTypeInfo<TRecord> recordTypeInfo, RelateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(RelateQuestion.Create(rules, options, cancellationToken), record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public Task<Row<ImmutableArray<Relation>,RelateDetail>> RelateAsync<TRecord>(RelateQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<ImmutableArray<Relation>,RelateDetail>> RelateStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<RelationRule> rules, JsonTypeInfo<TRecord> recordTypeInfo, RelateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(RelateQuestion.Create(rules, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<ImmutableArray<Relation>,RelateDetail>> RelateStreamAsync<TRecord>(RelateQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ImmutableDictionary<string, AnnotationEntry>,NoDetail>> AnnotateAsync<TRecord>(AnnotationQuestions questions, TRecord record, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(AnnotateQuestion.Create(questions, options, cancellationToken), record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ImmutableDictionary<string, AnnotationEntry>,NoDetail>> AnnotateAsync<TRecord>(AnnotateQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ImmutableDictionary<string, AnnotationEntry>,NoDetail>> AnnotateStreamAsync<TRecord>(AnnotationQuestions questions, IAsyncEnumerable<TRecord> records, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(AnnotateQuestion.Create(questions, options), records, Serialize<TRecord>, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ImmutableDictionary<string, AnnotationEntry>,NoDetail>> AnnotateStreamAsync<TRecord>(AnnotateQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    public Task<Row<ImmutableDictionary<string, AnnotationEntry>,NoDetail>> AnnotateAsync<TRecord>(AnnotationQuestions questions, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Single(AnnotateQuestion.Create(questions, options, cancellationToken), record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public Task<Row<ImmutableDictionary<string, AnnotationEntry>,NoDetail>> AnnotateAsync<TRecord>(AnnotateQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<ImmutableDictionary<string, AnnotationEntry>,NoDetail>> AnnotateStreamAsync<TRecord>(AnnotationQuestions questions, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(AnnotateQuestion.Create(questions, options), records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    public IAsyncEnumerable<Row<ImmutableDictionary<string, AnnotationEntry>,NoDetail>> AnnotateStreamAsync<TRecord>(AnnotateQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, recordTypeInfo), name, cancellationToken);
    private async IAsyncEnumerable<FilterItem<T>> FilterItems<T>(FilterQuestion question, IAsyncEnumerable<T> records,
        Func<T,byte[]> encode, string? name, [EnumeratorCancellation] CancellationToken token)
    {
        await foreach (var output in Run(question, records, encode, true, name, token).ConfigureAwait(false))
            yield return new(output.Item, output.Row);
    }
    private async IAsyncEnumerable<RankedItem<T>> RankItems<T>(RankQuestion question, IAsyncEnumerable<T> records,
        Func<T,byte[]> encode, string? name, [EnumeratorCancellation] CancellationToken token)
    {
        await foreach (var output in Run(question, records, encode, true, name, token).ConfigureAwait(false))
            yield return new(output.Item, output.Row);
    }
    public ToolboxView Toolbox(AnnotationQuestions questions) => new(this, questions);
}
public sealed class ToolboxView
{
    private readonly ThinkThenClient client;
    private readonly AnnotationQuestions questions;
    internal ToolboxView(ThinkThenClient client, AnnotationQuestions questions) { this.client = client; this.questions = questions; }
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ImmutableDictionary<string, AnnotationEntry>, NoDetail>> AnnotateAsync<TRecord>(TRecord record, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        client.AnnotateAsync(questions, record, options, name, cancellationToken);
    public Task<Row<ImmutableDictionary<string, AnnotationEntry>, NoDetail>> AnnotateAsync<TRecord>(TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        client.AnnotateAsync(questions, record, recordTypeInfo, options, name, cancellationToken);
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ImmutableDictionary<string, AnnotationEntry>, NoDetail>> AnnotateStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        client.AnnotateStreamAsync(questions, records, options, name, cancellationToken);
    public IAsyncEnumerable<Row<ImmutableDictionary<string, AnnotationEntry>, NoDetail>> AnnotateStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        client.AnnotateStreamAsync(questions, records, recordTypeInfo, options, name, cancellationToken);
}
