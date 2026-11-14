using System.Text.Json;
namespace ThinkThen;
internal static class Domain
{
    internal static string Word(object value) => value is Enum ? JsonNamingPolicy.SnakeCaseLower.ConvertName(value.ToString()!).Replace('_',' ') : (string)value;
    internal static string Member<T>(T value) where T:struct,Enum => Enum.IsDefined(value) ? Word(value) :
        throw new ArgumentException("Enum domains must contain named members.",nameof(value));
    internal static IReadOnlyDictionary<string,string>? Descriptions<T>(IReadOnlyDictionary<T,string>? values) where T:struct,Enum =>
        values?.ToDictionary(pair=>Member(pair.Key),pair=>pair.Value,StringComparer.Ordinal);
}
internal static class Domain<T> where T:struct,Enum
{
    internal static T Read(string word)
    {
        foreach (var member in Enum.GetValues<T>()) if (Domain.Word(member)==word) return member;
        throw new UsageException(new(ErrorCode.UsageArgument,$"Enum {typeof(T).Name} has no member for {word}"));
    }
}
internal static class ResultText
{
    internal static string Leaf(string value) => value.Replace('\r',' ').Replace('\n',' ').Replace('\t',' ');
    private static string Number(double value)
    {
        string text=value.ToString("R",System.Globalization.CultureInfo.InvariantCulture);
        int exponent=text.IndexOf('E');
        if(exponent<0) return text;
        string sign=text[0]=='-' ? "-" : "";
        string mantissa=text[sign.Length..exponent];
        int point=mantissa.IndexOf('.');
        int offset=(point<0 ? mantissa.Length : point)+int.Parse(text[(exponent+1)..],System.Globalization.CultureInfo.InvariantCulture);
        string digits=mantissa.Replace(".","");
        if(offset<=0) return sign+"0."+new string('0',-offset)+digits;
        if(offset>=digits.Length) return sign+digits+new string('0',offset-digits.Length);
        return sign+digits[..offset]+"."+digits[offset..];
    }
    internal static string Print(IRowValue row)
    {
        if (row is IFailed failed) return $"failed: {Leaf(failed.Error.Code)}: {Leaf(failed.Error.Message)}";
        if (row.IsUnsure) return "unsure";
        object? answer=row.OwnAnswer;
        string Join(System.Collections.IEnumerable values,Func<object,string> print) => string.Join("\t",values.Cast<object>().Select(print));
        return row.Function switch
        {
            Function.Decide or Function.Filter => answer is true ? "true" : "false",
            Function.Choose => Leaf(Domain.Word(answer!)),
            Function.Tag => Join((System.Collections.IEnumerable)answer!,value=>Leaf(Domain.Word(value))),
            Function.Score => Number((double)answer!),
            Function.Rank => Convert.ToString(answer,System.Globalization.CultureInfo.InvariantCulture)!,
            Function.Find => answer is TextSpan span ? Leaf(span.Text) : "",
            Function.Recognize => Join((System.Collections.IEnumerable)answer!,value=> { var entity=(Entity)value; return $"{Leaf(entity.Kind)}: {Leaf(entity.Text)} ({entity.Mention.Span.SavedStart}..{entity.Mention.Span.SavedEnd})"; }),
            Function.Relate => Join((System.Collections.IEnumerable)answer!,value=> { var relation=(FoundRelation)value; return $"{Leaf(relation.Source.Text)} --{Leaf(relation.Relation)}--> {Leaf(relation.Target.Text)}"; }),
            Function.Annotate => string.Join("\t",((IAnnotations)answer!).Entries.OrderBy(pair=>pair.Key,QuestionNameOrder.Instance).Select(pair=>$"{Leaf(pair.Key)}={Print((IRowValue)pair.Value)}")),
            _ => throw Json.Abi()
        };
    }
}

// Native question names are ordered by UTF-8 bytes, which differs from UTF-16 ordinal order.
internal static class QuestionNameOrder
{
    internal static IComparer<string> Instance { get; } = Comparer<string>.Create((left,right)=>
        System.Text.Encoding.UTF8.GetBytes(left).AsSpan().SequenceCompareTo(System.Text.Encoding.UTF8.GetBytes(right)));
}
