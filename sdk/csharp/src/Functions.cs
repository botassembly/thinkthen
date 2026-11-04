using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using System.Text.Json;
using System.Text.Json.Serialization.Metadata;
namespace ThinkThen;
/// <summary>Runs typed questions over records and owns active calls.</summary>
public sealed partial class ThinkThenClient
{
    /// <summary>Judges one record with a yes-or-no question and returns its answer and outcome metadata.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<bool,NoDetail>> DecideAsync<TRecord>(string question, TRecord record, DecideOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Own(() => DecideQuestion.Create(question, options, cancellationToken), question => Single(question, record, Serialize<TRecord>, name, cancellationToken, true, SingleCollection<TRecord>));
    /// <summary>Judges one record with a yes-or-no question and returns its answer and outcome metadata.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<bool,NoDetail>> DecideAsync<TRecord>(DecideQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken, false, SingleCollection<TRecord>);
    /// <summary>Returns decide outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<bool,NoDetail>> DecideStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, DecideOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        OwnStream(() => DecideQuestion.Create(question, options), question => Stream(question, records, Serialize<TRecord>, name, cancellationToken, true));
    /// <summary>Returns decide outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<bool,NoDetail>> DecideStreamAsync<TRecord>(DecideQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    /// <summary>Judges one record with a yes-or-no question and returns its answer and outcome metadata. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<bool,NoDetail>> DecideWithMetadataAsync<TRecord>(string question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, DecideOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Own(() => DecideQuestion.Create(question, options, cancellationToken), question => Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable));
    }
    /// <summary>Judges one record with a yes-or-no question and returns its answer and outcome metadata. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<bool,NoDetail>> DecideWithMetadataAsync<TRecord>(DecideQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, false, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable);
    }
    /// <summary>Returns decide outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<bool,NoDetail>> DecideStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, DecideOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return OwnStream(() => DecideQuestion.Create(question, options), question => Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true));
    }
    /// <summary>Returns decide outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<bool,NoDetail>> DecideStreamWithMetadataAsync<TRecord>(DecideQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken);
    }
    /// <summary>Chooses an option for one record and returns its answer and label probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<string,LabelsDetail>> ChooseAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> choices, ChooseOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Own(() => ChooseQuestion.Create(question, choices, options, cancellationToken), question => Single(question, record, Serialize<TRecord>, name, cancellationToken, true, SingleCollection<TRecord>));
    /// <summary>Chooses an option for one record and returns its answer and label probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<string,LabelsDetail>> ChooseAsync<TRecord>(ChooseQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken, false, SingleCollection<TRecord>);
    /// <summary>Returns choose outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<string,LabelsDetail>> ChooseStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> choices, ChooseOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        OwnStream(() => ChooseQuestion.Create(question, choices, options), question => Stream(question, records, Serialize<TRecord>, name, cancellationToken, true));
    /// <summary>Returns choose outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<string,LabelsDetail>> ChooseStreamAsync<TRecord>(ChooseQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    /// <summary>Chooses an option for one record and returns its answer and label probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<string,LabelsDetail>> ChooseWithMetadataAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Own(() => ChooseQuestion.Create(question, choices, options, cancellationToken), question => Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable));
    }
    /// <summary>Chooses an option for one record and returns its answer and label probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<string,LabelsDetail>> ChooseWithMetadataAsync<TRecord>(ChooseQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, false, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable);
    }
    /// <summary>Returns choose outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<string,LabelsDetail>> ChooseStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return OwnStream(() => ChooseQuestion.Create(question, choices, options), question => Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true));
    }
    /// <summary>Returns choose outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<string,LabelsDetail>> ChooseStreamWithMetadataAsync<TRecord>(ChooseQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken);
    }
    /// <summary>Assigns labels to one record and returns its answer and label probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<string>,LabelsDetail>> TagAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> labels, TagOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Own(() => TagQuestion.Create(question, labels, options, cancellationToken), question => Single(question, record, Serialize<TRecord>, name, cancellationToken, true, SingleCollection<TRecord>));
    /// <summary>Assigns labels to one record and returns its answer and label probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<string>,LabelsDetail>> TagAsync<TRecord>(TagQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken, false, SingleCollection<TRecord>);
    /// <summary>Returns tag outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<string>,LabelsDetail>> TagStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> labels, TagOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        OwnStream(() => TagQuestion.Create(question, labels, options), question => Stream(question, records, Serialize<TRecord>, name, cancellationToken, true));
    /// <summary>Returns tag outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<string>,LabelsDetail>> TagStreamAsync<TRecord>(TagQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    /// <summary>Assigns labels to one record and returns its answer and label probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<ValueList<string>,LabelsDetail>> TagWithMetadataAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Own(() => TagQuestion.Create(question, labels, options, cancellationToken), question => Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable));
    }
    /// <summary>Assigns labels to one record and returns its answer and label probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<ValueList<string>,LabelsDetail>> TagWithMetadataAsync<TRecord>(TagQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, false, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable);
    }
    /// <summary>Returns tag outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<ValueList<string>,LabelsDetail>> TagStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return OwnStream(() => TagQuestion.Create(question, labels, options), question => Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true));
    }
    /// <summary>Returns tag outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<ValueList<string>,LabelsDetail>> TagStreamWithMetadataAsync<TRecord>(TagQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken);
    }
    /// <summary>Scores one record against ordered levels and returns its answer and level probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<double,LevelsDetail>> ScoreAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> levels, ScoreOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Own(() => ScoreQuestion.Create(question, levels, options, cancellationToken), question => Single(question, record, Serialize<TRecord>, name, cancellationToken, true, SingleCollection<TRecord>));
    /// <summary>Scores one record against ordered levels and returns its answer and level probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<double,LevelsDetail>> ScoreAsync<TRecord>(ScoreQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken, false, SingleCollection<TRecord>);
    /// <summary>Returns score outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,LevelsDetail>> ScoreStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, ScoreOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        OwnStream(() => ScoreQuestion.Create(question, levels, options), question => Stream(question, records, Serialize<TRecord>, name, cancellationToken, true));
    /// <summary>Returns score outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,LevelsDetail>> ScoreStreamAsync<TRecord>(ScoreQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    /// <summary>Scores one record against ordered levels and returns its answer and level probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<double,LevelsDetail>> ScoreWithMetadataAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Own(() => ScoreQuestion.Create(question, levels, options, cancellationToken), question => Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable));
    }
    /// <summary>Scores one record against ordered levels and returns its answer and level probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<double,LevelsDetail>> ScoreWithMetadataAsync<TRecord>(ScoreQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, false, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable);
    }
    /// <summary>Returns score outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<double,LevelsDetail>> ScoreStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return OwnStream(() => ScoreQuestion.Create(question, levels, options), question => Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true));
    }
    /// <summary>Returns score outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<double,LevelsDetail>> ScoreStreamWithMetadataAsync<TRecord>(ScoreQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken);
    }
    /// <summary>Yields each input record with its filter outcome, including rejected, uncertain and failed records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<FilterItem<TRecord>> FilterStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, FilterOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        OwnStream(() => FilterQuestion.Create(question, options), question => FilterItems(question, records, Serialize<TRecord>, name, cancellationToken, true));
    /// <summary>Yields each input record with its filter outcome, including rejected, uncertain and failed records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<FilterItem<TRecord>> FilterStreamAsync<TRecord>(FilterQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        FilterItems(question, records, Serialize<TRecord>, name, cancellationToken);
    /// <summary>Yields each input record with its filter outcome, including rejected, uncertain and failed records. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<FilterItem<TRecord>> FilterStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, FilterOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return OwnStream(() => FilterQuestion.Create(question, options), question => FilterItems(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true));
    }
    /// <summary>Yields each input record with its filter outcome, including rejected, uncertain and failed records. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<FilterItem<TRecord>> FilterStreamWithMetadataAsync<TRecord>(FilterQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return FilterItems(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken);
    }
    /// <summary>Ranks input records and returns each emitted JSON snapshot as TRecord with its rank outcome. TRecord must support JSON reading as well as writing.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<RankedItem<TRecord>> RankAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, RankOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        OwnStream(() => RankQuestion.Create(question, levels, options), question => RankItems(question, records, Serialize<TRecord>, Deserialize<TRecord>, name, cancellationToken, true));
    /// <summary>Ranks input records and returns each emitted JSON snapshot as TRecord with its rank outcome. TRecord must support JSON reading as well as writing.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<RankedItem<TRecord>> RankAsync<TRecord>(RankQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        RankItems(question, records, Serialize<TRecord>, Deserialize<TRecord>, name, cancellationToken);
    /// <summary>Ranks input records and returns each emitted JSON snapshot as TRecord with its rank outcome. TRecord must support JSON reading as well as writing. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<RankedItem<TRecord>> RankWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, RankOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return OwnStream(() => RankQuestion.Create(question, levels, options), question => RankItems(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), data => JsonSerializer.Deserialize(data, recordTypeInfo)!, name, cancellationToken, true));
    }
    /// <summary>Ranks input records and returns each emitted JSON snapshot as TRecord with its rank outcome. TRecord must support JSON reading as well as writing. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<RankedItem<TRecord>> RankWithMetadataAsync<TRecord>(RankQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return RankItems(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), data => JsonSerializer.Deserialize(data, recordTypeInfo)!, name, cancellationToken);
    }
    /// <summary>Finds matching text in one record and returns its span in UTF-16 string offsets.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<TextSpan?,NoDetail>> FindAsync<TRecord>(string question, TRecord record, FindOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Own(() => FindQuestion.Create(question, options, cancellationToken), question => Single(question, record, Serialize<TRecord>, name, cancellationToken, true, SingleCollection<TRecord>));
    /// <summary>Finds matching text in one record and returns its span in UTF-16 string offsets.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<TextSpan?,NoDetail>> FindAsync<TRecord>(FindQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken, false, SingleCollection<TRecord>);
    /// <summary>Returns find outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TextSpan?,NoDetail>> FindStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, FindOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        OwnStream(() => FindQuestion.Create(question, options), question => Stream(question, records, Serialize<TRecord>, name, cancellationToken, true));
    /// <summary>Returns find outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TextSpan?,NoDetail>> FindStreamAsync<TRecord>(FindQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    /// <summary>Finds matching text in one record and returns its span in UTF-16 string offsets. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<TextSpan?,NoDetail>> FindWithMetadataAsync<TRecord>(string question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, FindOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Own(() => FindQuestion.Create(question, options, cancellationToken), question => Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable));
    }
    /// <summary>Finds matching text in one record and returns its span in UTF-16 string offsets. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<TextSpan?,NoDetail>> FindWithMetadataAsync<TRecord>(FindQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, false, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable);
    }
    /// <summary>Returns find outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<TextSpan?,NoDetail>> FindStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, FindOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return OwnStream(() => FindQuestion.Create(question, options), question => Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true));
    }
    /// <summary>Returns find outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<TextSpan?,NoDetail>> FindStreamWithMetadataAsync<TRecord>(FindQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken);
    }
    /// <summary>Recognizes named entities in one record and returns their spans and probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<Entity>,RecognizeDetail>> RecognizeAsync<TRecord>(TRecord record, IReadOnlyList<string> kinds, RecognizeOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Own(() => RecognizeQuestion.Create(kinds, options, cancellationToken), question => Single(question, record, Serialize<TRecord>, name, cancellationToken, true, SingleCollection<TRecord>));
    /// <summary>Recognizes named entities in one record and returns their spans and probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<Entity>,RecognizeDetail>> RecognizeAsync<TRecord>(RecognizeQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken, false, SingleCollection<TRecord>);
    /// <summary>Returns recognize outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<Entity>,RecognizeDetail>> RecognizeStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<string> kinds, RecognizeOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        OwnStream(() => RecognizeQuestion.Create(kinds, options), question => Stream(question, records, Serialize<TRecord>, name, cancellationToken, true));
    /// <summary>Returns recognize outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<Entity>,RecognizeDetail>> RecognizeStreamAsync<TRecord>(RecognizeQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    /// <summary>Recognizes named entities in one record and returns their spans and probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<ValueList<Entity>,RecognizeDetail>> RecognizeWithMetadataAsync<TRecord>(TRecord record, IReadOnlyList<string> kinds, JsonTypeInfo<TRecord> recordTypeInfo, RecognizeOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Own(() => RecognizeQuestion.Create(kinds, options, cancellationToken), question => Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable));
    }
    /// <summary>Recognizes named entities in one record and returns their spans and probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<ValueList<Entity>,RecognizeDetail>> RecognizeWithMetadataAsync<TRecord>(RecognizeQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, false, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable);
    }
    /// <summary>Returns recognize outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<ValueList<Entity>,RecognizeDetail>> RecognizeStreamWithMetadataAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<string> kinds, JsonTypeInfo<TRecord> recordTypeInfo, RecognizeOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return OwnStream(() => RecognizeQuestion.Create(kinds, options), question => Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true));
    }
    /// <summary>Returns recognize outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<ValueList<Entity>,RecognizeDetail>> RecognizeStreamWithMetadataAsync<TRecord>(RecognizeQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken);
    }
    /// <summary>Finds relations in one record and returns their mentions and probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<Relation>,RelateDetail>> RelateAsync<TRecord>(TRecord record, IReadOnlyList<RelationRule> rules, RelateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        Own(() => RelateQuestion.Create(rules, options, cancellationToken), question => Single(question, record, Serialize<TRecord>, name, cancellationToken, true, SingleCollection<TRecord>));
    /// <summary>Finds relations in one record and returns their mentions and probabilities.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<Relation>,RelateDetail>> RelateAsync<TRecord>(RelateQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken, false, SingleCollection<TRecord>);
    /// <summary>Returns relate outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<Relation>,RelateDetail>> RelateStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<RelationRule> rules, RelateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        OwnStream(() => RelateQuestion.Create(rules, options), question => Stream(question, records, Serialize<TRecord>, name, cancellationToken, true));
    /// <summary>Returns relate outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<Relation>,RelateDetail>> RelateStreamAsync<TRecord>(RelateQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    /// <summary>Finds relations in one record and returns their mentions and probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<ValueList<Relation>,RelateDetail>> RelateWithMetadataAsync<TRecord>(TRecord record, IReadOnlyList<RelationRule> rules, JsonTypeInfo<TRecord> recordTypeInfo, RelateOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Own(() => RelateQuestion.Create(rules, options, cancellationToken), question => Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable));
    }
    /// <summary>Finds relations in one record and returns their mentions and probabilities. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<ValueList<Relation>,RelateDetail>> RelateWithMetadataAsync<TRecord>(RelateQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, false, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable);
    }
    /// <summary>Returns relate outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<ValueList<Relation>,RelateDetail>> RelateStreamWithMetadataAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<RelationRule> rules, JsonTypeInfo<TRecord> recordTypeInfo, RelateOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return OwnStream(() => RelateQuestion.Create(rules, options), question => Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true));
    }
    /// <summary>Returns relate outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<ValueList<Relation>,RelateDetail>> RelateStreamWithMetadataAsync<TRecord>(RelateQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken);
    }
    /// <summary>Answers the named questions for one record and retains each child outcome.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueMap<string, AnnotationEntry>,NoDetail>> AnnotateAsync<TRecord>(AnnotationQuestions questions, TRecord record, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        options is null ? Single(Admission.Required(questions).Question, record, Serialize<TRecord>, name, cancellationToken, false, SingleCollection<TRecord>) :
        Own(() => AnnotateQuestion.Create(questions, options, cancellationToken), question => Single(question, record, Serialize<TRecord>, name, cancellationToken, true, SingleCollection<TRecord>));
    /// <summary>Answers the named questions for one record and retains each child outcome.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueMap<string, AnnotationEntry>,NoDetail>> AnnotateAsync<TRecord>(AnnotateQuestion question, TRecord record, string? name = null, CancellationToken cancellationToken = default) =>
        Single(question, record, Serialize<TRecord>, name, cancellationToken, false, SingleCollection<TRecord>);
    /// <summary>Returns annotate outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueMap<string, AnnotationEntry>,NoDetail>> AnnotateStreamAsync<TRecord>(AnnotationQuestions questions, IAsyncEnumerable<TRecord> records, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        options is null ? Stream(Admission.Required(questions).Question, records, Serialize<TRecord>, name, cancellationToken) :
        OwnStream(() => AnnotateQuestion.Create(questions, options), question => Stream(question, records, Serialize<TRecord>, name, cancellationToken, true));
    /// <summary>Returns annotate outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueMap<string, AnnotationEntry>,NoDetail>> AnnotateStreamAsync<TRecord>(AnnotateQuestion question, IAsyncEnumerable<TRecord> records, string? name = null, CancellationToken cancellationToken = default) =>
        Stream(question, records, Serialize<TRecord>, name, cancellationToken);
    /// <summary>Answers the named questions for one record and retains each child outcome. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<ValueMap<string, AnnotationEntry>,NoDetail>> AnnotateWithMetadataAsync<TRecord>(AnnotationQuestions questions, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return options is null ? Single(Admission.Required(questions).Question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, false, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable) :
            Own(() => AnnotateQuestion.Create(questions, options, cancellationToken), question => Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable));
    }
    /// <summary>Answers the named questions for one record and retains each child outcome. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<ValueMap<string, AnnotationEntry>,NoDetail>> AnnotateWithMetadataAsync<TRecord>(AnnotateQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Single(question, record, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, false, _ => recordTypeInfo.Kind == JsonTypeInfoKind.Enumerable);
    }
    /// <summary>Returns annotate outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<ValueMap<string, AnnotationEntry>,NoDetail>> AnnotateStreamWithMetadataAsync<TRecord>(AnnotationQuestions questions, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return options is null ? Stream(Admission.Required(questions).Question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken) :
            OwnStream(() => AnnotateQuestion.Create(questions, options), question => Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken, true));
    }
    /// <summary>Returns annotate outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<ValueMap<string, AnnotationEntry>,NoDetail>> AnnotateStreamWithMetadataAsync<TRecord>(AnnotateQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return Stream(question, records, record => JsonSerializer.SerializeToUtf8Bytes(record, Admission.Required(recordTypeInfo)), name, cancellationToken);
    }
    private async IAsyncEnumerable<FilterItem<T>> FilterItems<T>(FilterQuestion question, IAsyncEnumerable<T> records,
        Func<T,byte[]> encode, string? name, [EnumeratorCancellation] CancellationToken token, bool owned = false)
    {
        await foreach (var output in Run(question, records, encode, true, name, token, owned).ConfigureAwait(false))
            yield return new(output.Item, output.Row);
    }
    private async IAsyncEnumerable<RankedItem<T>> RankItems<T>(RankQuestion question, IAsyncEnumerable<T> records,
        Func<T,byte[]> encode, Func<byte[],T> decode, string? name, [EnumeratorCancellation] CancellationToken token, bool owned = false)
    {
        // Preflight the first record before it can enter a backend request.
        async IAsyncEnumerable<T> CheckedRecords([EnumeratorCancellation] CancellationToken cancellationToken = default)
        {
            bool first = true;
            await foreach (T item in records.WithCancellation(cancellationToken).ConfigureAwait(false))
            {
                if (first && !RankNeedsHostOriginal(item))
                {
                    byte[] bytes;
                    try { bytes = encode(item); }
                    catch (Exception cause) when (cause is JsonException or NotSupportedException)
                    { throw new JsonException($"Rank cannot write its record snapshot as {typeof(T)}: {cause.Message}", cause); }
                    _ = RankSnapshot(bytes, decode, encode);
                    first = false;
                }
                yield return item;
            }
        }
        ArgumentNullException.ThrowIfNull(records);
        await foreach (var output in Run(question, CheckedRecords(), encode, false, name, token, owned,
            data => RankSnapshot(data, decode, encode)).ConfigureAwait(false))
            yield return new(output.Item, output.Row);
    }
    /// <summary>Binds a named annotation form to this client.</summary>
    public ToolboxView Toolbox(AnnotationQuestions questions) => new(this, Admission.Required(questions));
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
    public Task<Row<ValueMap<string, AnnotationEntry>, NoDetail>> AnnotateAsync<TRecord>(TRecord record, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        client.AnnotateAsync(questions, record, options, name, cancellationToken);
    /// <summary>Answers the named questions for one record and retains each child outcome. Uses explicit JSON metadata for trimming and AOT.</summary>
    public Task<Row<ValueMap<string, AnnotationEntry>, NoDetail>> AnnotateWithMetadataAsync<TRecord>(TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return client.AnnotateWithMetadataAsync(questions, record, recordTypeInfo, options, name, cancellationToken);
    }
    /// <summary>Returns annotate outcomes for input records, preserving each position, uncertainty and record failure.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueMap<string, AnnotationEntry>, NoDetail>> AnnotateStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default) =>
        client.AnnotateStreamAsync(questions, records, options, name, cancellationToken);
    /// <summary>Returns annotate outcomes for input records, preserving each position, uncertainty and record failure. Uses explicit JSON metadata for trimming and AOT.</summary>
    public IAsyncEnumerable<Row<ValueMap<string, AnnotationEntry>, NoDetail>> AnnotateStreamWithMetadataAsync<TRecord>(IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options = null, string? name = null, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(recordTypeInfo);
        return client.AnnotateStreamWithMetadataAsync(questions, records, recordTypeInfo, options, name, cancellationToken);
    }
}
