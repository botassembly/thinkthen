using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using System.Text.Json;
using System.Text.Json.Serialization.Metadata;
namespace ThinkThen;
/// <summary>Runs typed questions over records and retains the caller's original objects.</summary>
public sealed partial class ThinkThenClient
{
    /// <summary>Runs decide with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<bool,TRecord>> DecideAsync<TRecord>(string question, TRecord record, DecideOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(DecideQuestion.Create(question,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs decide with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<bool,TRecord>> DecideAsync<TRecord>(DecideQuestion question, TRecord record, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs decide with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<bool,TRecord>> DecideStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, DecideOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(DecideQuestion.Create(question,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs decide with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<bool,TRecord>> DecideStreamAsync<TRecord>(DecideQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs decide with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<bool,TRecord>> DecideStreamAsync<TRecord>(string question, IEnumerable<TRecord> records, DecideOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(DecideQuestion.Create(question,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs decide with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<bool,TRecord>> DecideStreamAsync<TRecord>(DecideQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs decide with typed results and original records.</summary>
    public Task<Row<bool,TRecord>> DecideWithMetadataAsync<TRecord>(string question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, DecideOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(DecideQuestion.Create(question,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs decide with typed results and original records.</summary>
    public Task<Row<bool,TRecord>> DecideWithMetadataAsync<TRecord>(DecideQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs decide with typed results and original records.</summary>
    public IAsyncEnumerable<Row<bool,TRecord>> DecideStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, DecideOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(DecideQuestion.Create(question,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs decide with typed results and original records.</summary>
    public IAsyncEnumerable<Row<bool,TRecord>> DecideStreamWithMetadataAsync<TRecord>(DecideQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs decide with typed results and original records.</summary>
    public IAsyncEnumerable<Row<bool,TRecord>> DecideStreamWithMetadataAsync<TRecord>(string question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, DecideOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(DecideQuestion.Create(question,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs decide with typed results and original records.</summary>
    public IAsyncEnumerable<Row<bool,TRecord>> DecideStreamWithMetadataAsync<TRecord>(DecideQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<string,TRecord>> ChooseAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> choices, ChooseOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(ChooseQuestion.Create(question,choices,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<string,TRecord>> ChooseAsync<TRecord>(ChooseQuestion question, TRecord record, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<string,TRecord>> ChooseStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> choices, ChooseOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(ChooseQuestion.Create(question,choices,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<string,TRecord>> ChooseStreamAsync<TRecord>(ChooseQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<string,TRecord>> ChooseStreamAsync<TRecord>(string question, IEnumerable<TRecord> records, IReadOnlyList<string> choices, ChooseOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(ChooseQuestion.Create(question,choices,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<string,TRecord>> ChooseStreamAsync<TRecord>(ChooseQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    public Task<Row<string,TRecord>> ChooseWithMetadataAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(ChooseQuestion.Create(question,choices,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs choose with typed results and original records.</summary>
    public Task<Row<string,TRecord>> ChooseWithMetadataAsync<TRecord>(ChooseQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs choose with typed results and original records.</summary>
    public IAsyncEnumerable<Row<string,TRecord>> ChooseStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(ChooseQuestion.Create(question,choices,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    public IAsyncEnumerable<Row<string,TRecord>> ChooseStreamWithMetadataAsync<TRecord>(ChooseQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    public IAsyncEnumerable<Row<string,TRecord>> ChooseStreamWithMetadataAsync<TRecord>(string question, IEnumerable<TRecord> records, IReadOnlyList<string> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(ChooseQuestion.Create(question,choices,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    public IAsyncEnumerable<Row<string,TRecord>> ChooseStreamWithMetadataAsync<TRecord>(ChooseQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<string>,TRecord>> TagAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> labels, TagOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(TagQuestion.Create(question,labels,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<string>,TRecord>> TagAsync<TRecord>(TagQuestion question, TRecord record, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<string>,TRecord>> TagStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> labels, TagOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(TagQuestion.Create(question,labels,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<string>,TRecord>> TagStreamAsync<TRecord>(TagQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<string>,TRecord>> TagStreamAsync<TRecord>(string question, IEnumerable<TRecord> records, IReadOnlyList<string> labels, TagOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(TagQuestion.Create(question,labels,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<string>,TRecord>> TagStreamAsync<TRecord>(TagQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    public Task<Row<ValueList<string>,TRecord>> TagWithMetadataAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(TagQuestion.Create(question,labels,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs tag with typed results and original records.</summary>
    public Task<Row<ValueList<string>,TRecord>> TagWithMetadataAsync<TRecord>(TagQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs tag with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<string>,TRecord>> TagStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(TagQuestion.Create(question,labels,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<string>,TRecord>> TagStreamWithMetadataAsync<TRecord>(TagQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<string>,TRecord>> TagStreamWithMetadataAsync<TRecord>(string question, IEnumerable<TRecord> records, IReadOnlyList<string> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(TagQuestion.Create(question,labels,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<string>,TRecord>> TagStreamWithMetadataAsync<TRecord>(TagQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<double,TRecord>> ScoreAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> levels, ScoreOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(ScoreQuestion.Create(question,levels,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs score with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<double,TRecord>> ScoreAsync<TRecord>(ScoreQuestion question, TRecord record, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs score with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, ScoreOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(ScoreQuestion.Create(question,levels,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamAsync<TRecord>(ScoreQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamAsync<TRecord>(string question, IEnumerable<TRecord> records, IReadOnlyList<string> levels, ScoreOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(ScoreQuestion.Create(question,levels,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamAsync<TRecord>(ScoreQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    public Task<Row<double,TRecord>> ScoreWithMetadataAsync<TRecord>(string question, TRecord record, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(ScoreQuestion.Create(question,levels,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs score with typed results and original records.</summary>
    public Task<Row<double,TRecord>> ScoreWithMetadataAsync<TRecord>(ScoreQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs score with typed results and original records.</summary>
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(ScoreQuestion.Create(question,levels,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamWithMetadataAsync<TRecord>(ScoreQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamWithMetadataAsync<TRecord>(string question, IEnumerable<TRecord> records, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(ScoreQuestion.Create(question,levels,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamWithMetadataAsync<TRecord>(ScoreQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs find with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<TextSpan?,TRecord>> FindAsync<TRecord>(string question, TRecord record, FindOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(FindQuestion.Create(question,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs find with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<TextSpan?,TRecord>> FindAsync<TRecord>(FindQuestion question, TRecord record, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs find with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TextSpan?,TRecord>> FindStreamAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, FindOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(FindQuestion.Create(question,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs find with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TextSpan?,TRecord>> FindStreamAsync<TRecord>(FindQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs find with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TextSpan?,TRecord>> FindStreamAsync<TRecord>(string question, IEnumerable<TRecord> records, FindOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(FindQuestion.Create(question,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs find with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TextSpan?,TRecord>> FindStreamAsync<TRecord>(FindQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs find with typed results and original records.</summary>
    public Task<Row<TextSpan?,TRecord>> FindWithMetadataAsync<TRecord>(string question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, FindOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(FindQuestion.Create(question,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs find with typed results and original records.</summary>
    public Task<Row<TextSpan?,TRecord>> FindWithMetadataAsync<TRecord>(FindQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs find with typed results and original records.</summary>
    public IAsyncEnumerable<Row<TextSpan?,TRecord>> FindStreamWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, FindOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(FindQuestion.Create(question,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs find with typed results and original records.</summary>
    public IAsyncEnumerable<Row<TextSpan?,TRecord>> FindStreamWithMetadataAsync<TRecord>(FindQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs find with typed results and original records.</summary>
    public IAsyncEnumerable<Row<TextSpan?,TRecord>> FindStreamWithMetadataAsync<TRecord>(string question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, FindOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(FindQuestion.Create(question,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs find with typed results and original records.</summary>
    public IAsyncEnumerable<Row<TextSpan?,TRecord>> FindStreamWithMetadataAsync<TRecord>(FindQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs recognize with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<Entity>,TRecord>> RecognizeAsync<TRecord>(TRecord record, IReadOnlyList<string> kinds, RecognizeOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(RecognizeQuestion.Create(kinds,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs recognize with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<Entity>,TRecord>> RecognizeAsync<TRecord>(RecognizeQuestion question, TRecord record, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs recognize with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<Entity>,TRecord>> RecognizeStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<string> kinds, RecognizeOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RecognizeQuestion.Create(kinds,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs recognize with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<Entity>,TRecord>> RecognizeStreamAsync<TRecord>(RecognizeQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs recognize with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<Entity>,TRecord>> RecognizeStreamAsync<TRecord>(IEnumerable<TRecord> records, IReadOnlyList<string> kinds, RecognizeOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RecognizeQuestion.Create(kinds,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs recognize with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<Entity>,TRecord>> RecognizeStreamAsync<TRecord>(RecognizeQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs recognize with typed results and original records.</summary>
    public Task<Row<ValueList<Entity>,TRecord>> RecognizeWithMetadataAsync<TRecord>(TRecord record, IReadOnlyList<string> kinds, JsonTypeInfo<TRecord> recordTypeInfo, RecognizeOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(RecognizeQuestion.Create(kinds,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs recognize with typed results and original records.</summary>
    public Task<Row<ValueList<Entity>,TRecord>> RecognizeWithMetadataAsync<TRecord>(RecognizeQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs recognize with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<Entity>,TRecord>> RecognizeStreamWithMetadataAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<string> kinds, JsonTypeInfo<TRecord> recordTypeInfo, RecognizeOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RecognizeQuestion.Create(kinds,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs recognize with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<Entity>,TRecord>> RecognizeStreamWithMetadataAsync<TRecord>(RecognizeQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs recognize with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<Entity>,TRecord>> RecognizeStreamWithMetadataAsync<TRecord>(IEnumerable<TRecord> records, IReadOnlyList<string> kinds, JsonTypeInfo<TRecord> recordTypeInfo, RecognizeOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RecognizeQuestion.Create(kinds,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs recognize with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<Entity>,TRecord>> RecognizeStreamWithMetadataAsync<TRecord>(RecognizeQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs relate with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<FoundRelation>,TRecord>> RelateAsync<TRecord>(TRecord record, IReadOnlyList<RelationRule> rules, RelateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(RelateQuestion.Create(rules,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs relate with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<FoundRelation>,TRecord>> RelateAsync<TRecord>(RelateQuestion question, TRecord record, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs relate with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<FoundRelation>,TRecord>> RelateStreamAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<RelationRule> rules, RelateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RelateQuestion.Create(rules,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs relate with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<FoundRelation>,TRecord>> RelateStreamAsync<TRecord>(RelateQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs relate with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<FoundRelation>,TRecord>> RelateStreamAsync<TRecord>(IEnumerable<TRecord> records, IReadOnlyList<RelationRule> rules, RelateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RelateQuestion.Create(rules,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs relate with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<FoundRelation>,TRecord>> RelateStreamAsync<TRecord>(RelateQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs relate with typed results and original records.</summary>
    public Task<Row<ValueList<FoundRelation>,TRecord>> RelateWithMetadataAsync<TRecord>(TRecord record, IReadOnlyList<RelationRule> rules, JsonTypeInfo<TRecord> recordTypeInfo, RelateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(RelateQuestion.Create(rules,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs relate with typed results and original records.</summary>
    public Task<Row<ValueList<FoundRelation>,TRecord>> RelateWithMetadataAsync<TRecord>(RelateQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Single(question,record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs relate with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<FoundRelation>,TRecord>> RelateStreamWithMetadataAsync<TRecord>(IAsyncEnumerable<TRecord> records, IReadOnlyList<RelationRule> rules, JsonTypeInfo<TRecord> recordTypeInfo, RelateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RelateQuestion.Create(rules,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs relate with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<FoundRelation>,TRecord>> RelateStreamWithMetadataAsync<TRecord>(RelateQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs relate with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<FoundRelation>,TRecord>> RelateStreamWithMetadataAsync<TRecord>(IEnumerable<TRecord> records, IReadOnlyList<RelationRule> rules, JsonTypeInfo<TRecord> recordTypeInfo, RelateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RelateQuestion.Create(rules,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs relate with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<FoundRelation>,TRecord>> RelateStreamWithMetadataAsync<TRecord>(RelateQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<long,TRecord>> RankAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, RankOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RankQuestion.Create(question,levels,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<long,TRecord>> RankAsync<TRecord>(RankQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<long,TRecord>> RankAsync<TRecord>(string question, IEnumerable<TRecord> records, IReadOnlyList<string> levels, RankOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RankQuestion.Create(question,levels,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<long,TRecord>> RankAsync<TRecord>(RankQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    public IAsyncEnumerable<Row<long,TRecord>> RankWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, RankOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RankQuestion.Create(question,levels,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    public IAsyncEnumerable<Row<long,TRecord>> RankWithMetadataAsync<TRecord>(RankQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    public IAsyncEnumerable<Row<long,TRecord>> RankWithMetadataAsync<TRecord>(string question, IEnumerable<TRecord> records, IReadOnlyList<string> levels, JsonTypeInfo<TRecord> recordTypeInfo, RankOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(RankQuestion.Create(question,levels,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    public IAsyncEnumerable<Row<long,TRecord>> RankWithMetadataAsync<TRecord>(RankQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs filter with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<TRecord> FilterAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, FilterOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Keep(Run(FilterQuestion.Create(question,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken),cancellationToken);
    /// <summary>Runs filter with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<TRecord> FilterAsync<TRecord>(FilterQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Keep(Run(question,records,Serialize<TRecord>,name,cancellationToken),cancellationToken);
    /// <summary>Runs filter with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<TRecord> FilterAsync<TRecord>(string question, IEnumerable<TRecord> records, FilterOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Keep(Run(FilterQuestion.Create(question,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken),cancellationToken);
    /// <summary>Runs filter with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<TRecord> FilterAsync<TRecord>(FilterQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Keep(Run(question,Input(records),Serialize<TRecord>,name,cancellationToken),cancellationToken);
    /// <summary>Runs filter with typed results and original records.</summary>
    public IAsyncEnumerable<TRecord> FilterWithMetadataAsync<TRecord>(string question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, FilterOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Keep(Run(FilterQuestion.Create(question,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken),cancellationToken);
    /// <summary>Runs filter with typed results and original records.</summary>
    public IAsyncEnumerable<TRecord> FilterWithMetadataAsync<TRecord>(FilterQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Keep(Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken),cancellationToken);
    /// <summary>Runs filter with typed results and original records.</summary>
    public IAsyncEnumerable<TRecord> FilterWithMetadataAsync<TRecord>(string question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, FilterOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Keep(Run(FilterQuestion.Create(question,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken),cancellationToken);
    /// <summary>Runs filter with typed results and original records.</summary>
    public IAsyncEnumerable<TRecord> FilterWithMetadataAsync<TRecord>(FilterQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Keep(Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken),cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<TChoice,TRecord>> ChooseAsync<TRecord, TChoice>(string question, TRecord record, IReadOnlyList<TChoice> choices, ChooseOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(ChooseQuestion.Create(question,choices,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<TChoice,TRecord>> ChooseAsync<TRecord, TChoice>(ChooseQuestion<TChoice> question, TRecord record, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(question,record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TChoice,TRecord>> ChooseStreamAsync<TRecord, TChoice>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<TChoice> choices, ChooseOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(ChooseQuestion.Create(question,choices,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TChoice,TRecord>> ChooseStreamAsync<TRecord, TChoice>(ChooseQuestion<TChoice> question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TChoice,TRecord>> ChooseStreamAsync<TRecord, TChoice>(string question, IEnumerable<TRecord> records, IReadOnlyList<TChoice> choices, ChooseOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(ChooseQuestion.Create(question,choices,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<TChoice,TRecord>> ChooseStreamAsync<TRecord, TChoice>(ChooseQuestion<TChoice> question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    public Task<Row<TChoice,TRecord>> ChooseWithMetadataAsync<TRecord, TChoice>(string question, TRecord record, IReadOnlyList<TChoice> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(ChooseQuestion.Create(question,choices,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs choose with typed results and original records.</summary>
    public Task<Row<TChoice,TRecord>> ChooseWithMetadataAsync<TRecord, TChoice>(ChooseQuestion<TChoice> question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(question,record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs choose with typed results and original records.</summary>
    public IAsyncEnumerable<Row<TChoice,TRecord>> ChooseStreamWithMetadataAsync<TRecord, TChoice>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<TChoice> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(ChooseQuestion.Create(question,choices,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    public IAsyncEnumerable<Row<TChoice,TRecord>> ChooseStreamWithMetadataAsync<TRecord, TChoice>(ChooseQuestion<TChoice> question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    public IAsyncEnumerable<Row<TChoice,TRecord>> ChooseStreamWithMetadataAsync<TRecord, TChoice>(string question, IEnumerable<TRecord> records, IReadOnlyList<TChoice> choices, JsonTypeInfo<TRecord> recordTypeInfo, ChooseOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(ChooseQuestion.Create(question,choices,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs choose with typed results and original records.</summary>
    public IAsyncEnumerable<Row<TChoice,TRecord>> ChooseStreamWithMetadataAsync<TRecord, TChoice>(ChooseQuestion<TChoice> question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<TChoice>,TRecord>> TagAsync<TRecord, TChoice>(string question, TRecord record, IReadOnlyList<TChoice> labels, TagOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(TagQuestion.Create(question,labels,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<ValueList<TChoice>,TRecord>> TagAsync<TRecord, TChoice>(TagQuestion<TChoice> question, TRecord record, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(question,record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<TChoice>,TRecord>> TagStreamAsync<TRecord, TChoice>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<TChoice> labels, TagOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(TagQuestion.Create(question,labels,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<TChoice>,TRecord>> TagStreamAsync<TRecord, TChoice>(TagQuestion<TChoice> question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(question,records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<TChoice>,TRecord>> TagStreamAsync<TRecord, TChoice>(string question, IEnumerable<TRecord> records, IReadOnlyList<TChoice> labels, TagOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(TagQuestion.Create(question,labels,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<ValueList<TChoice>,TRecord>> TagStreamAsync<TRecord, TChoice>(TagQuestion<TChoice> question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(question,Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    public Task<Row<ValueList<TChoice>,TRecord>> TagWithMetadataAsync<TRecord, TChoice>(string question, TRecord record, IReadOnlyList<TChoice> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(TagQuestion.Create(question,labels,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs tag with typed results and original records.</summary>
    public Task<Row<ValueList<TChoice>,TRecord>> TagWithMetadataAsync<TRecord, TChoice>(TagQuestion<TChoice> question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(question,record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs tag with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<TChoice>,TRecord>> TagStreamWithMetadataAsync<TRecord, TChoice>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<TChoice> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(TagQuestion.Create(question,labels,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<TChoice>,TRecord>> TagStreamWithMetadataAsync<TRecord, TChoice>(TagQuestion<TChoice> question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(question,records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<TChoice>,TRecord>> TagStreamWithMetadataAsync<TRecord, TChoice>(string question, IEnumerable<TRecord> records, IReadOnlyList<TChoice> labels, JsonTypeInfo<TRecord> recordTypeInfo, TagOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(TagQuestion.Create(question,labels,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs tag with typed results and original records.</summary>
    public IAsyncEnumerable<Row<ValueList<TChoice>,TRecord>> TagStreamWithMetadataAsync<TRecord, TChoice>(TagQuestion<TChoice> question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(question,Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<double,TRecord>> ScoreAsync<TRecord, TChoice>(string question, TRecord record, IReadOnlyList<TChoice> levels, ScoreOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(ScoreQuestion.Create(question,levels,options,cancellationToken),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Runs score with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamAsync<TRecord, TChoice>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<TChoice> levels, ScoreOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(ScoreQuestion.Create(question,levels,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamAsync<TRecord, TChoice>(string question, IEnumerable<TRecord> records, IReadOnlyList<TChoice> levels, ScoreOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(ScoreQuestion.Create(question,levels,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    public Task<Row<double,TRecord>> ScoreWithMetadataAsync<TRecord, TChoice>(string question, TRecord record, IReadOnlyList<TChoice> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Single(ScoreQuestion.Create(question,levels,options,cancellationToken),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Runs score with typed results and original records.</summary>
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamWithMetadataAsync<TRecord, TChoice>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<TChoice> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(ScoreQuestion.Create(question,levels,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs score with typed results and original records.</summary>
    public IAsyncEnumerable<Row<double,TRecord>> ScoreStreamWithMetadataAsync<TRecord, TChoice>(string question, IEnumerable<TRecord> records, IReadOnlyList<TChoice> levels, JsonTypeInfo<TRecord> recordTypeInfo, ScoreOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(ScoreQuestion.Create(question,levels,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<long,TRecord>> RankAsync<TRecord, TChoice>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<TChoice> levels, RankOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(RankQuestion.Create(question,levels,options,cancellationToken),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<long,TRecord>> RankAsync<TRecord, TChoice>(string question, IEnumerable<TRecord> records, IReadOnlyList<TChoice> levels, RankOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(RankQuestion.Create(question,levels,options,cancellationToken),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    public IAsyncEnumerable<Row<long,TRecord>> RankWithMetadataAsync<TRecord, TChoice>(string question, IAsyncEnumerable<TRecord> records, IReadOnlyList<TChoice> levels, JsonTypeInfo<TRecord> recordTypeInfo, RankOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(RankQuestion.Create(question,levels,options,cancellationToken),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Runs rank with typed results and original records.</summary>
    public IAsyncEnumerable<Row<long,TRecord>> RankWithMetadataAsync<TRecord, TChoice>(string question, IEnumerable<TRecord> records, IReadOnlyList<TChoice> levels, JsonTypeInfo<TRecord> recordTypeInfo, RankOptions<TChoice>? options=null, string? name=null, CancellationToken cancellationToken=default) where TChoice:struct,Enum =>
        Run(RankQuestion.Create(question,levels,options,cancellationToken),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Answers named questions about each record.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<Annotations<TRecord>,TRecord>> AnnotateAsync<TRecord>(AnnotationQuestions questions, TRecord record, AnnotateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(new TypedAnnotateQuestion<TRecord>(Admission.Build(Function.Annotate,null,questions.Definition,"questions",options?.Field)),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Answers named questions about each record.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public Task<Row<Annotations<TRecord>,TRecord>> AnnotateAsync<TRecord>(AnnotateQuestion question, TRecord record, string? name=null, CancellationToken cancellationToken=default) =>
        Single(new TypedAnnotateQuestion<TRecord>(question.Definition),record,Serialize<TRecord>,name,cancellationToken,SingleCollection<TRecord>);
    /// <summary>Answers named questions about each record.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<Annotations<TRecord>,TRecord>> AnnotateStreamAsync<TRecord>(AnnotationQuestions questions, IAsyncEnumerable<TRecord> records, AnnotateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(new TypedAnnotateQuestion<TRecord>(Admission.Build(Function.Annotate,null,questions.Definition,"questions",options?.Field)),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Answers named questions about each record.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<Annotations<TRecord>,TRecord>> AnnotateStreamAsync<TRecord>(AnnotateQuestion question, IAsyncEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(new TypedAnnotateQuestion<TRecord>(question.Definition),records,Serialize<TRecord>,name,cancellationToken);
    /// <summary>Answers named questions about each record.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<Annotations<TRecord>,TRecord>> AnnotateStreamAsync<TRecord>(AnnotationQuestions questions, IEnumerable<TRecord> records, AnnotateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(new TypedAnnotateQuestion<TRecord>(Admission.Build(Function.Annotate,null,questions.Definition,"questions",options?.Field)),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Answers named questions about each record.</summary>
    [RequiresUnreferencedCode("Provide JsonTypeInfo for trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for AOT applications.")]
    public IAsyncEnumerable<Row<Annotations<TRecord>,TRecord>> AnnotateStreamAsync<TRecord>(AnnotateQuestion question, IEnumerable<TRecord> records, string? name=null, CancellationToken cancellationToken=default) =>
        Run(new TypedAnnotateQuestion<TRecord>(question.Definition),Input(records),Serialize<TRecord>,name,cancellationToken);
    /// <summary>Answers named questions about each record.</summary>
    public Task<Row<Annotations<TRecord>,TRecord>> AnnotateWithMetadataAsync<TRecord>(AnnotationQuestions questions, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Single(new TypedAnnotateQuestion<TRecord>(Admission.Build(Function.Annotate,null,questions.Definition,"questions",options?.Field)),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Answers named questions about each record.</summary>
    public Task<Row<Annotations<TRecord>,TRecord>> AnnotateWithMetadataAsync<TRecord>(AnnotateQuestion question, TRecord record, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Single(new TypedAnnotateQuestion<TRecord>(question.Definition),record,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken,_=>Admission.Required(recordTypeInfo).Kind==JsonTypeInfoKind.Enumerable);
    /// <summary>Answers named questions about each record.</summary>
    public IAsyncEnumerable<Row<Annotations<TRecord>,TRecord>> AnnotateStreamWithMetadataAsync<TRecord>(AnnotationQuestions questions, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(new TypedAnnotateQuestion<TRecord>(Admission.Build(Function.Annotate,null,questions.Definition,"questions",options?.Field)),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Answers named questions about each record.</summary>
    public IAsyncEnumerable<Row<Annotations<TRecord>,TRecord>> AnnotateStreamWithMetadataAsync<TRecord>(AnnotateQuestion question, IAsyncEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(new TypedAnnotateQuestion<TRecord>(question.Definition),records,record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Answers named questions about each record.</summary>
    public IAsyncEnumerable<Row<Annotations<TRecord>,TRecord>> AnnotateStreamWithMetadataAsync<TRecord>(AnnotationQuestions questions, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, AnnotateOptions? options=null, string? name=null, CancellationToken cancellationToken=default) =>
        Run(new TypedAnnotateQuestion<TRecord>(Admission.Build(Function.Annotate,null,questions.Definition,"questions",options?.Field)),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
    /// <summary>Answers named questions about each record.</summary>
    public IAsyncEnumerable<Row<Annotations<TRecord>,TRecord>> AnnotateStreamWithMetadataAsync<TRecord>(AnnotateQuestion question, IEnumerable<TRecord> records, JsonTypeInfo<TRecord> recordTypeInfo, string? name=null, CancellationToken cancellationToken=default) =>
        Run(new TypedAnnotateQuestion<TRecord>(question.Definition),Input(records),record=>JsonSerializer.SerializeToUtf8Bytes(record,Admission.Required(recordTypeInfo)),name,cancellationToken);
}
