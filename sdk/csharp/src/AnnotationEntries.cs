namespace ThinkThen;
internal interface IAnnotations { void ThrowIfFailed(); IReadOnlyDictionary<string,IRow> Entries { get; } }
/// <summary>Immutable named results, reached through a form's answer.</summary>
public sealed class Annotations<TRecord> : IAnnotations, IEquatable<Annotations<TRecord>>
{
    private readonly ValueMap<string,IRow> entries;
    private readonly IReadOnlyDictionary<string,IReadOnlyList<string>> domains;
    internal Annotations(IEnumerable<KeyValuePair<string,IRow>> entries,IReadOnlyDictionary<string,IReadOnlyList<string>> domains)
    { this.entries=new(entries); this.domains=domains; }
    /// <summary>Provides Entries.</summary>
    public IReadOnlyDictionary<string,IRow> Entries => entries;
    private Row<A,TRecord> Get<A>(string name,Function function)
    {
        if (!entries.TryGetValue(name,out var row) || row.Function!=function)
            throw new UsageException(new(ErrorCode.UsageArgument,$"No {Contract.FunctionName((uint)function)} annotation named {name}; available names: {string.Join(", ",entries.Keys)}"));
        return (Row<A,TRecord>)row;
    }
    /// <summary>Provides Decide.</summary>
    public Row<bool,TRecord> Decide(string name) => Get<bool>(name,Function.Decide);
    /// <summary>Provides Choose.</summary>
    public Row<string,TRecord> Choose(string name) => Get<string>(name,Function.Choose);
    /// <summary>Provides Choose.</summary>
    public Row<T,TRecord> Choose<T>(string name) where T : struct,Enum
    { var row=Choose(name); Validate<T>(name); return Map(row,Domain<T>.Read); }
    /// <summary>Provides Tag.</summary>
    public Row<ValueList<string>,TRecord> Tag(string name) => Get<ValueList<string>>(name,Function.Tag);
    /// <summary>Provides Tag.</summary>
    public Row<ValueList<T>,TRecord> Tag<T>(string name) where T : struct,Enum
    { var row=Tag(name); Validate<T>(name); return Map(row,items=>new ValueList<T>(items.Select(Domain<T>.Read))); }
    private void Validate<T>(string name) where T:struct,Enum
    { foreach(var word in domains[name]) _=Domain<T>.Read(word); }
    /// <summary>Provides Score.</summary>
    public Row<double,TRecord> Score(string name) => Get<double>(name,Function.Score);
    /// <summary>Provides Find.</summary>
    public Row<TextSpan?,TRecord> Find(string name) => Get<TextSpan?>(name,Function.Find);
    /// <summary>Provides Recognize.</summary>
    public Row<ValueList<Entity>,TRecord> Recognize(string name) => Get<ValueList<Entity>>(name,Function.Recognize);
    /// <summary>Provides Relate.</summary>
    public Row<ValueList<FoundRelation>,TRecord> Relate(string name) => Get<ValueList<FoundRelation>>(name,Function.Relate);
    void IAnnotations.ThrowIfFailed()
    {
        var failed=entries.OrderBy(pair=>pair.Key,QuestionNameOrder.Instance).Select(pair=>pair.Value).OfType<IFailed>().FirstOrDefault();
        if (failed is not null) throw new FailedException([failed]);
    }
    internal static Row<B,TRecord> Map<A,B>(Row<A,TRecord> row,Func<A,B> convert) => row switch
    {
        Answered<A,TRecord> value => new Answered<B,TRecord>(value.Function,value.Position,value.Id,value.Source,value.Item,convert(value.Answer),value.Probability,value.Detail) { Canonical=value.Canonical },
        Unsure<A,TRecord> value => new Unsure<B,TRecord>(value.Function,value.Position,value.Id,value.Source,value.Item,value.Probability!.Value,value.Detail) { Canonical=value.Canonical },
        Failed<A,TRecord> value => new Failed<B,TRecord>(value.Function,value.Position,value.Id,value.Source,value.Item,value.Error) { Canonical=value.Canonical },
        _ => throw Json.Abi()
    };
    /// <inheritdoc/>
    public bool Equals(Annotations<TRecord>? other) => other is not null && entries.Equals(other.entries);
    /// <inheritdoc/>
    public override bool Equals(object? other) => other is Annotations<TRecord> value && Equals(value);
    /// <inheritdoc/>
    public override int GetHashCode() => entries.GetHashCode();
}
