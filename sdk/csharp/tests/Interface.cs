using System.Text.Json;
using ThinkThen;


internal static partial class Program
{
    private static void InterfaceCases()
    {
        static void Check(bool value, string message)
        {
            if (!value) throw new Exception(message);
        }
        var source = new Source("fixture", Backend.Stub, "stub");
        var yes = ThinkThenModelFactory.Answered<bool, NoDetail>(Function.Decide, true, source, probability: 0.8);
        var duplicate = ThinkThenModelFactory.Answered<bool, NoDetail>(Function.Decide, true, source, probability: 0.8);
        bool answer = yes;
        Check(answer && yes == true && true == yes && yes != false, "answer conversion and symmetric comparison");
        Check(yes.Equals(duplicate) && yes.GetHashCode() == duplicate.GetHashCode(), "rows compare by value");
        Check(yes.ToString() == "True", "row prints its answer");
        Check(yes.TryGetAnswer(out var obtained) && obtained, "answer is available without a type pattern");
        Row<bool, NoDetail> unsure = ThinkThenModelFactory.Unsure<bool, NoDetail>(Function.Decide, 0.5, source);
        Check(unsure.IsUnsure && unsure != true && unsure != false && true != unsure, "unsure comparisons never guess or throw");
        Check(!unsure.TryGetAnswer(out _), "unsure has no answer");
        try { bool missing = unsure; throw new Exception($"unsure returned {missing}"); }
        catch (UnsureException error) { Check(error.Probability == 0.5, "unsure carries probability"); }
        Row<bool, NoDetail> failure = ThinkThenModelFactory.Failed<bool, NoDetail>(Function.Decide,
            new RowError(ErrorCode.RecordMissing, "missing"), source);
        Check(failure.IsFailed && failure != true && failure != false, "failed comparisons are safe");
        try { bool missing = failure; throw new Exception($"failure returned {missing}"); }
        catch (ThinkThenException error) { Check(error.ErrorCode == ErrorCode.RecordMissing, "failure preserves code"); }
        var first = ThinkThenModelFactory.Answered<ValueList<string>, LabelsDetail>(Function.Tag, ["billing", "refund"], source,
            detail: new(ValueMap<string, double>.Empty.Add("billing", 0.7).Add("refund", 0.6)));
        var second = ThinkThenModelFactory.Answered<ValueList<string>, LabelsDetail>(Function.Tag, ["billing", "refund"], source,
            detail: new(ValueMap<string, double>.Empty.Add("refund", 0.6).Add("billing", 0.7)));
        Check(first.Equals(second) && first.GetHashCode() == second.GetHashCode(), "list and map values compare structurally");
        Check(new HashSet<Row<ValueList<string>, LabelsDetail>> { first, second }.Count == 1, "hash collections agree");
        Check(first.Detail!.Equals(second.Detail), "details compare by value");
        Check(first.Answer.Equals(second.Answer), "public list answers compare their values");
        Check(first.Detail.Labels.Equals(second.Detail!.Labels), "public probability maps compare their values");
        using var serialized = JsonDocument.Parse(JsonSerializer.Serialize<Row<bool, NoDetail>>(yes));
        Check(serialized.RootElement.GetProperty("answer").GetBoolean(), "constructed rows serialize using the public row shape");
        Check(new DecideEntry(new EntryAnswered<bool, NoDetail>(true, 0.8, null)).Answer, "annotation answers take one step");
        Check(!typeof(DecideQuestion).GetMethods().Any(method => method.Name == "<Clone>$"), "question cannot clone its handle");


        // This method is compiled, but need not open a native client to prove null options are unambiguous.
        static Task<Row<bool, NoDetail>> NullOptions(ThinkThenClient client) => client.DecideAsync("Ready?", "alpha", null);
        _ = (Func<ThinkThenClient, Task<Row<bool, NoDetail>>>)NullOptions;
        try { DecideQuestion.Create(null!); throw new Exception("null question was accepted"); }
        catch (ArgumentNullException error) { Check(error.ParamName == "question", "null argument identifies its parameter"); }

        // Public-interface constraints run in the ordinary consumer, without a native engine.
        Check(typeof(UnsureException).BaseType == typeof(InvalidOperationException), "uncertainty follows .NET exception conventions");
        Check(typeof(ThinkThenClient).GetMethods().Where(method => method.Name.EndsWith("Async", StringComparison.Ordinal) && method.Name != "DisposeAsync")
            .All(method => method.GetParameters().Last().ParameterType == typeof(CancellationToken)), "async calls keep the final cancellation token");
        foreach (var method in typeof(ThinkThenClient).GetMethods().Where(method => method.DeclaringType == typeof(ThinkThenClient)))
        {
            Check(method.GetGenericArguments().All(type => type.Name is not ("A" or "D")), "generic names explain their role");
            if (method.GetParameters().Any(parameter => parameter.ParameterType.IsGenericType &&
                parameter.ParameterType.GetGenericTypeDefinition() == typeof(System.Text.Json.Serialization.Metadata.JsonTypeInfo<>)))
                Check(method.Name.Contains("WithMetadata", StringComparison.Ordinal), "metadata overloads have distinct names");
        }

        string department = ThinkThenModelFactory.Answered<string, LabelsDetail>(Function.Choose, "billing", source);
        double urgency = ThinkThenModelFactory.Answered<double, LevelsDetail>(Function.Score, 0.75, source);
        long rank = ThinkThenModelFactory.Answered<long, LevelsDetail>(Function.Rank, 2, source);
        TextSpan? found = ThinkThenModelFactory.Answered<TextSpan?, NoDetail>(Function.Find, null, source);
        Check(department == "billing" && urgency == 0.75 && rank == 2 && found is null, "scalar and nullable answers convert without defaults");
        var entity = new Entity(new Mention(new TextSpan("Ada", 0, 3), "person"), 0.9);
        var recognitionDetail = new RecognizeDetail(
            [new PieceDetail(entity.Mention.Span, ValueMap<BoundaryTag, double>.Empty.Add(BoundaryTag.Single, 0.9))],
            [new KindDetail("Ada", 0, 3, ValueMap<string, double>.Empty.Add("person", 0.9))]);
        var recognized = ThinkThenModelFactory.Answered<ValueList<Entity>, RecognizeDetail>(Function.Recognize, [entity], source, detail: recognitionDetail);
        var recognitionCopy = ThinkThenModelFactory.Answered<ValueList<Entity>, RecognizeDetail>(Function.Recognize,
            [new(new(new("Ada", 0, 3), "person"), 0.9)], source,
            detail: new([new(new("Ada", 0, 3), ValueMap<BoundaryTag, double>.Empty.Add(BoundaryTag.Single, 0.9))],
                [new("Ada", 0, 3, ValueMap<string, double>.Empty.Add("person", 0.9))]));
        Check(recognized == recognitionCopy && recognized.GetHashCode() == recognitionCopy.GetHashCode(), "recognition spans and probability maps compare by value");
        var relation = new Relation(entity.Mention, entity.Mention, "knows", 0.8, false);
        var relations = ThinkThenModelFactory.Answered<ValueList<Relation>, RelateDetail>(Function.Relate, [relation], source,
            detail: new([entity], [relation]));
        Check(relations.Equals(relations with { }), "relation row clones retain value equality");
        var annotation = ThinkThenModelFactory.Answered<ValueMap<string, AnnotationEntry>, NoDetail>(Function.Annotate,
            ValueMap<string, AnnotationEntry>.Empty.Add("ready", new DecideEntry(new EntryAnswered<bool, NoDetail>(true, 0.8, null))), source);
        var annotationCopy = ThinkThenModelFactory.Answered<ValueMap<string, AnnotationEntry>, NoDetail>(Function.Annotate,
            ValueMap<string, AnnotationEntry>.Empty.Add("ready", new DecideEntry(new EntryAnswered<bool, NoDetail>(true, 0.8, null))), source);
        Check(annotation.Equals(annotationCopy) && annotation.GetHashCode() == annotationCopy.GetHashCode(), "annotation outcomes compare by value");
        Check(annotation.Answer.Equals(annotationCopy.Answer), "public annotation maps compare their values");
        AnnotationEntry annotatedUnsure = new DecideEntry(new EntryUnsure<bool, NoDetail>(0.4, null));
        Check(annotatedUnsure != true && false != annotatedUnsure, "annotation uncertainty never equals an answer");
        try { bool missing = annotatedUnsure; throw new Exception($"annotation guessed {missing}"); }
        catch (UnsureException error) { Check(error.Probability == 0.4, "annotation preserves uncertainty probability"); }
        var caused = ThinkThenModelFactory.Failed<bool, NoDetail>(Function.Decide,
            new RowError(ErrorCode.RecordMissing, "missing") { Cause = new InvalidOperationException("host cause") }, source);
        Check(failure.Equals(caused) && failure.GetHashCode() == caused.GetHashCode(), "host cause identity is not row value data");
        Check(!yes.Equals(ThinkThenModelFactory.Answered<bool, NoDetail>(Function.Decide, false, source, probability: 0.8)), "different answers are unequal");
        Check(!yes.Equals(ThinkThenModelFactory.Answered<bool, NoDetail>(Function.Decide, true, source, position: 2, probability: 0.8)), "input positions participate in equality");
        Console.WriteLine("C# independent public interface checks passed.");
    }
}
