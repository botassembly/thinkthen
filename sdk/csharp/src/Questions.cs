using System.Collections.Immutable;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
namespace ThinkThen;
public abstract record Question<A,D>
{
    internal Question(JsonObject definition, QuestionHandle? handle = null)
    { Definition = definition; Handle = handle ?? Admission.Admit(definition); }
    internal QuestionHandle Handle { get; }
    internal JsonObject Definition { get; }
    public Function Function => Json.Function(Definition["function"]!.GetValue<string>());
    private protected abstract void Closed();
}
internal static class Admission
{
    internal static T Required<T>(T? value) where T : class =>
        value ?? throw Json.Conversion(new ArgumentNullException(nameof(value)));
    internal static JsonObject Named<T>(IReadOnlyDictionary<string, T> questions) where T : ToolboxQuestion =>
        new(Required(questions).Select(pair => KeyValuePair.Create<string, JsonNode?>(
            Json.HostText(pair.Key), Required(pair.Value).Definition.DeepClone())));
    internal static byte[] Bytes(JsonNode value) => Encoding.UTF8.GetBytes(value.ToJsonString());
    internal static QuestionHandle Admit(JsonObject value)
    {
        byte[] bytes = Bytes(value);
        NativeError error = NativeError.Empty;
        int result = Native.QuestionFromJson(bytes, (nuint)bytes.Length, out nint pointer, ref error);
        try { Json.Check(result, error); return new(pointer); }
        catch { if (pointer != 0) Native.QuestionFree(pointer); throw; }
    }
    internal static void Validate(JsonObject value) { using var handle = Admit(value); }
    internal static JsonObject Build(Function function, string? text, JsonNode? domain, string? field,
        Pointer? evidence = null, Threshold? threshold = null, int? top = null)
    {
        JsonObject value = new() { ["function"] = Contract.FunctionName((uint)function) };
        if (text is not null) value["question"] = Json.HostText(text);
        if (field is not null) value[field] = domain;
        if (evidence is not null) value["evidence"] = evidence.Value;
        if (threshold is not null)
        {
            using var stream = new MemoryStream();
            using (var writer = new Utf8JsonWriter(stream)) threshold.Write(writer);
            value["threshold"] = JsonNode.Parse(stream.ToArray());
        }
        if (top is not null) value["top"] = top.Value;
        return value;
    }
    internal static JsonArray Strings(IReadOnlyList<string> values) => new(Required(values).Select(value => (JsonNode?)JsonValue.Create(Json.HostText(value))).ToArray());
    internal static JsonArray Rules(IReadOnlyList<RelationRule> values) => new(Required(values).Select(rule => Rule(Required(rule))).ToArray());
    private static JsonNode Rule(RelationRule rule) => new JsonObject
    { ["source"] = Json.HostText(rule.Source), ["target"] = Json.HostText(rule.Target), ["relation"] = Json.HostText(rule.Relation),
      ["reads"] = rule.Reads is null ? null : Json.HostText(rule.Reads), ["either"] = rule.Either };
    internal static void Threshold(double low, double? high)
    {
        NativeError error = NativeError.Empty;
        Json.Check(Native.QuestionNew((uint)Function.Decide, Encoding.UTF8.GetBytes("validate"), 8, out nint question, ref error), error);
        try
        {
            error = NativeError.Empty;
            int result = high is double upper ? Native.SetBand(question, low, upper, ref error) : Native.SetCut(question, low, ref error);
            Json.Check(result, error);
        }
        finally { Native.QuestionFree(question); }
    }
    internal static void Pointer(string value)
    {
        byte[] bytes = Encoding.UTF8.GetBytes(Json.HostText(value));
        NativeError error = NativeError.Empty;
        Json.Check(Native.QuestionNew((uint)Function.Decide, Encoding.UTF8.GetBytes("validate"), 8, out nint question, ref error), error);
        try { error = NativeError.Empty; Json.Check(Native.SetEvidence(question, bytes, (nuint)bytes.Length, ref error), error); }
        finally { Native.QuestionFree(question); }
    }
}
public sealed record DecideQuestion : Question<bool,NoDetail>
{
    private DecideQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static DecideQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static DecideQuestion Create(string question, DecideOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Decide, question, null, null, options?.Evidence, options?.Threshold, null));
    }
    public string Text => Definition["question"]!.GetValue<string>();
}
public sealed record ChooseQuestion : Question<string,LabelsDetail>
{
    private ChooseQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static ChooseQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static ChooseQuestion Create(string question, IReadOnlyList<string> choices, ChooseOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Choose, question, Admission.Strings(choices), "choices", options?.Evidence, options?.Threshold, null));
    }
    public string Text => Definition["question"]!.GetValue<string>();
    public ImmutableArray<string> Choices => Definition["choices"]!.AsArray().Select(value => value!.GetValue<string>()).ToImmutableArray();
}
public sealed record TagQuestion : Question<ImmutableArray<string>,LabelsDetail>
{
    private TagQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static TagQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static TagQuestion Create(string question, IReadOnlyList<string> labels, TagOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Tag, question, Admission.Strings(labels), "labels", options?.Evidence, options?.Threshold, null));
    }
    public string Text => Definition["question"]!.GetValue<string>();
    public ImmutableArray<string> Labels => Definition["labels"]!.AsArray().Select(value => value!.GetValue<string>()).ToImmutableArray();
}
public sealed record ScoreQuestion : Question<double,LevelsDetail>
{
    private ScoreQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static ScoreQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static ScoreQuestion Create(string question, IReadOnlyList<string> levels, ScoreOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Score, question, Admission.Strings(levels), "levels", options?.Evidence, null, null));
    }
    public string Text => Definition["question"]!.GetValue<string>();
    public ImmutableArray<string> Levels => Definition["levels"]!.AsArray().Select(value => value!.GetValue<string>()).ToImmutableArray();
}
public sealed record FilterQuestion : Question<bool,NoDetail>
{
    private FilterQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static FilterQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static FilterQuestion Create(string question, FilterOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Filter, question, null, null, options?.Evidence, options?.Threshold, null));
    }
    public string Text => Definition["question"]!.GetValue<string>();
}
public sealed record RankQuestion : Question<long,LevelsDetail>
{
    private RankQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static RankQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static RankQuestion Create(string question, IReadOnlyList<string> levels, RankOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Rank, question, Admission.Strings(levels), "levels", options?.Evidence, null, options?.Top));
    }
    public string Text => Definition["question"]!.GetValue<string>();
    public ImmutableArray<string> Levels => Definition["levels"]!.AsArray().Select(value => value!.GetValue<string>()).ToImmutableArray();
}
public sealed record FindQuestion : Question<TextSpan?,NoDetail>
{
    private FindQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static FindQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static FindQuestion Create(string question, FindOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Find, question, null, null, options?.Evidence, options?.Threshold, null));
    }
    public string Text => Definition["question"]!.GetValue<string>();
}
public sealed record RecognizeQuestion : Question<ImmutableArray<Entity>,RecognizeDetail>
{
    private RecognizeQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static RecognizeQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static RecognizeQuestion Create(IReadOnlyList<string> kinds, RecognizeOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Recognize, null, Admission.Strings(kinds), "kinds", options?.Evidence, options?.Threshold, null));
    }
    public ImmutableArray<string> Kinds => Definition["kinds"]!.AsArray().Select(value => value!.GetValue<string>()).ToImmutableArray();
}
public sealed record RelateQuestion : Question<ImmutableArray<Relation>,RelateDetail>
{
    private RelateQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static RelateQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static RelateQuestion Create(IReadOnlyList<RelationRule> rules, RelateOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Relate, null, Admission.Rules(rules), "rules", options?.Evidence, options?.Threshold, null));
    }
}
public sealed record AnnotateQuestion : Question<ImmutableDictionary<string, AnnotationEntry>,NoDetail>
{
    private AnnotateQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static AnnotateQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    public static AnnotateQuestion Create(AnnotationQuestions questions, AnnotateOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Annotate, null, Admission.Required(questions).Definition.DeepClone(), "questions", options?.Evidence, null, null));
    }
}
public sealed class Toolbox
{
    internal JsonObject Definition { get; }
    private readonly ToolboxHandle handle;
    private Toolbox(JsonObject definition, ToolboxHandle handle) { Definition = definition; this.handle = handle; }
    public static Toolbox Parse(string json, CancellationToken cancellationToken = default) => Admit(json, false, cancellationToken);
    public static Toolbox FromJson(string json, CancellationToken cancellationToken = default) => Admit(json, true, cancellationToken);
    private static Toolbox Admit(string json, bool saved, CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();
        byte[] bytes = Encoding.UTF8.GetBytes(Json.HostText(json));
        NativeError error = NativeError.Empty;
        nint pointer;
        int result = saved
            ? Native.ToolboxFromJson(bytes, (nuint)bytes.Length, out pointer, ref error)
            : Native.ToolboxFromDefinitionsJson(bytes, (nuint)bytes.Length, out pointer, ref error);
        ToolboxHandle owner = new(pointer);
        try {
            Json.Check(result, error);
            error = NativeError.Empty;
            result = Native.ToolboxDefinitionsJson(owner, out Buffer output, ref error);
            try { Json.Check(result, error); return new(JsonNode.Parse(Native.Copy(output))!.AsObject(), owner); }
            finally { Native.BufferFree(ref output); }
        }
        catch { owner.Dispose(); throw; }
    }
    public string ToJson()
    {
        NativeError error = NativeError.Empty;
        int result = Native.ToolboxToJson(handle, out Buffer output, ref error);
        try { Json.Check(result, error); return Encoding.UTF8.GetString(Native.Copy(output)); }
        finally { Native.BufferFree(ref output); }
    }
    internal (JsonObject Definition, QuestionHandle Handle) Select(string name, Function function)
    {
        byte[] key = Encoding.UTF8.GetBytes(Json.HostText(name));
        NativeError error = NativeError.Empty;
        int result = Native.ToolboxSelectAs(handle, key, (nuint)key.Length, (uint)function, out nint pointer, ref error);
        var question = new QuestionHandle(pointer);
        try { Json.Check(result, error); return ((JsonObject)Definition[name]!.DeepClone(), question); }
        catch { question.Dispose(); throw; }
    }
    public DecideQuestion Decide(string name) { var selected = Select(name, Function.Decide); return DecideQuestion.Admit(selected.Definition, selected.Handle); }
    public ChooseQuestion Choose(string name) { var selected = Select(name, Function.Choose); return ChooseQuestion.Admit(selected.Definition, selected.Handle); }
    public TagQuestion Tag(string name) { var selected = Select(name, Function.Tag); return TagQuestion.Admit(selected.Definition, selected.Handle); }
    public ScoreQuestion Score(string name) { var selected = Select(name, Function.Score); return ScoreQuestion.Admit(selected.Definition, selected.Handle); }
    public FilterQuestion Filter(string name) { var selected = Select(name, Function.Filter); return FilterQuestion.Admit(selected.Definition, selected.Handle); }
    public RankQuestion Rank(string name) { var selected = Select(name, Function.Rank); return RankQuestion.Admit(selected.Definition, selected.Handle); }
    public FindQuestion Find(string name) { var selected = Select(name, Function.Find); return FindQuestion.Admit(selected.Definition, selected.Handle); }
    public RecognizeQuestion Recognize(string name) { var selected = Select(name, Function.Recognize); return RecognizeQuestion.Admit(selected.Definition, selected.Handle); }
    public RelateQuestion Relate(string name) { var selected = Select(name, Function.Relate); return RelateQuestion.Admit(selected.Definition, selected.Handle); }
    public static Toolbox Create(IReadOnlyDictionary<string, ToolboxQuestion> questions) =>
        Parse(Admission.Named(questions).ToJsonString());
}
public abstract record ToolboxQuestion
{
    internal ToolboxQuestion(JsonObject definition) => Definition = definition;
    internal JsonObject Definition { get; }
    private protected abstract void Closed();
    public sealed record Rank : ToolboxQuestion
    {
        public Rank(RankQuestion question) : base(Admission.Required(question).Definition) { }
        private protected override void Closed() { }
    }
}
public abstract record AnnotationQuestion : ToolboxQuestion
{
    internal AnnotationQuestion(JsonObject definition) : base(definition) { }
    public sealed record Decide : AnnotationQuestion { public Decide(DecideQuestion question) : base(Admission.Required(question).Definition) { } private protected override void Closed() { } }
    public sealed record Choose : AnnotationQuestion { public Choose(ChooseQuestion question) : base(Admission.Required(question).Definition) { } private protected override void Closed() { } }
    public sealed record Tag : AnnotationQuestion { public Tag(TagQuestion question) : base(Admission.Required(question).Definition) { } private protected override void Closed() { } }
    public sealed record Score : AnnotationQuestion { public Score(ScoreQuestion question) : base(Admission.Required(question).Definition) { } private protected override void Closed() { } }
    public sealed record Filter : AnnotationQuestion { public Filter(FilterQuestion question) : base(Admission.Required(question).Definition) { } private protected override void Closed() { } }
    public sealed record Find : AnnotationQuestion { public Find(FindQuestion question) : base(Admission.Required(question).Definition) { } private protected override void Closed() { } }
    public sealed record Recognize : AnnotationQuestion { public Recognize(RecognizeQuestion question) : base(Admission.Required(question).Definition) { } private protected override void Closed() { } }
    public sealed record Relate : AnnotationQuestion { public Relate(RelateQuestion question) : base(Admission.Required(question).Definition) { } private protected override void Closed() { } }
}
public sealed class AnnotationQuestions
{
    internal JsonObject Definition { get; }
    private AnnotationQuestions(JsonObject definition) => Definition = definition;
    public static AnnotationQuestions Create(IReadOnlyDictionary<string, AnnotationQuestion> questions)
    {
        JsonObject value = Admission.Named(questions);
        Admission.Validate(Admission.Build(Function.Annotate, null, value.DeepClone(), "questions"));
        return new(value);
    }
    public static AnnotationQuestions Parse(string json, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        JsonObject value = Toolbox.Parse(json, cancellationToken).Definition;
        Admission.Validate(Admission.Build(Function.Annotate, null, value.DeepClone(), "questions"));
        return new(value);
    }
}
